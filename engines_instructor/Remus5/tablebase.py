"""tablebase.py

Ask Lichess about a root position, with a bounded wait and native move validation.
FEN and JSON belong to this boundary; the search uses native positions and moves.
"""

import json
import time
from email.utils import parsedate_to_datetime
from http.client import HTTPException
from queue import Empty, Queue
from threading import Thread
from urllib.error import HTTPError
from urllib.parse import urlencode
from urllib.request import Request, urlopen

from attacks import in_check
from board import WHITE, Move, Position, move_text, position_key, square_name
from make_move import make_move
from move_generation import generate_moves

ENDPOINT = "https://tablebase.lichess.org/standard"  # Public Lichess tablebase API
MAX_BYTES = 65_536  # Bound the JSON response held in memory
CACHE_SIZE = 256  # Positions retained in this engine process
MAX_WAIT = 1.0  # At most one second, and at most a quarter of the move budget
MIN_WAIT = 0.02  # Very short moves use native search without a network request
OUTCOME = {"win": 1, "loss": -1, "draw": 0, "cursed-win": 0, "blessed-loss": 0}

Response = tuple[dict | None, float]  # JSON and the earliest next request time
cache: dict[str, dict] = {}
pending: tuple[str, Queue[Response]] | None = None
next_request = 0.0  # A monotonic timestamp, shared by requests in this process


def position_fen(position: Position) -> str:
    """Serialize the native position, including its actual halfmove counter."""
    board, white, castling, en_passant, halfmove = position
    text = ""
    # Row 0 is rank 8 and column 0 is file a, exactly the order FEN uses.
    for row in range(8):
        if row:
            text += "/"
        gap = 0
        for col in range(8):
            piece = board[row * 8 + col]
            if piece == 0:
                gap += 1
                continue
            if gap:
                text += str(gap)
                gap = 0
            letter = ".pnbrqk"[piece & 7]
            text += letter.upper() if piece & WHITE else letter
        if gap:
            text += str(gap)
    rights = ""
    for letter, bit in (("K", 1), ("Q", 2), ("k", 4), ("q", 8)):
        if castling & bit:
            rights += letter
    target = "-" if en_passant < 0 else square_name(en_passant)
    # Fullmove number does not affect a probe. The engine does not store it.
    return f"{text} {'w' if white else 'b'} {rights or '-'} {target} {halfmove} 1"


def eligible(position: Position, history: list[int]) -> bool:
    """Probe only small positions without castling rights or repeated history."""
    if position[2] or position[4] >= 100:
        return False
    count = 0
    for piece in position[0]:
        if piece:
            count += 1
    if count > 7:
        return False
    seen = {position_key(position)}
    start = max(0, len(history) - position[4])
    for key in history[start:]:
        if key in seen:
            return False
        seen.add(key)
    return True


def response_move(position: Position, history: list[int], data: dict) -> Move | None:
    """Accept the first ranked move only when its result and native legality agree."""
    category = data.get("category")
    moves = data.get("moves")
    if not isinstance(category, str) or category not in OUTCOME:
        return None
    if not isinstance(moves, list) or not moves or not isinstance(moves[0], dict):
        return None
    first = moves[0]
    child_category = first.get("category")
    if not isinstance(child_category, str) or child_category not in OUTCOME:
        return None
    # A move's category describes the opponent's position after that move.
    if OUTCOME[child_category] != -OUTCOME[category]:
        return None
    for move in generate_moves(position):
        if move_text(move) != first.get("uci"):
            continue
        child = make_move(position, move)
        if in_check(child[0], position[1]):
            return None
        if child[4]:
            recent = history[max(0, len(history) - position[4]) :]
            if position_key(child) in recent:
                return None
        return move
    return None


def retry_delay(value: str | None) -> float:
    """Respect Retry-After seconds or an HTTP date, with at least a minute off."""
    try:
        if value is not None and value.isdigit():
            return max(60.0, float(value))
        if value is not None:
            return max(60.0, parsedate_to_datetime(value).timestamp() - time.time())
    except ValueError, OverflowError, TypeError:
        pass
    return 60.0


def fetch(fen: str, seconds: float, results: Queue[Response]) -> None:
    """Send one request on a daemon thread and return JSON or a failure cooldown."""
    data = None
    delay = 5.0
    try:
        request = Request(
            ENDPOINT + "?" + urlencode({"fen": fen}),
            headers={"User-Agent": "Remus5/1.0", "Accept": "application/json"},
        )
        with urlopen(request, timeout=seconds) as response:
            body = response.read(MAX_BYTES + 1)
            if len(body) <= MAX_BYTES:
                parsed = json.loads(body)
                if isinstance(parsed, dict):
                    data = parsed
                    delay = 1.0
    except HTTPError as error:
        if error.code == 429:
            delay = retry_delay(error.headers.get("Retry-After"))
        error.close()
    except OSError, ValueError, RecursionError, HTTPException:
        pass
    finally:
        results.put((data, time.monotonic() + delay))


def collect(seconds: float) -> None:
    """Collect a pending response without exceeding the caller's wait budget."""
    global pending, next_request
    if pending is None:
        return
    fen, results = pending
    try:
        data, next_request = results.get(timeout=max(0.0, seconds))
    except Empty:
        return
    pending = None
    if data is not None:
        if len(cache) >= CACHE_SIZE:
            cache.clear()
        cache[fen] = data


def lookup(position: Position, history: list[int], longest: float) -> Move | None:
    """
    Return a validated tablebase move, or None for native-search fallback.

    Parameters
    ----------
    position : Position
        The current root position, never a node inside the search.
    history : list[int]
        Position keys before the root, used to avoid repetition-sensitive probes.
    longest : float
        Remaining move budget in seconds. At least three quarters stays for search.

    Returns
    -------
    Move or None
        The server's first ranked legal move, or no usable tablebase result.
    """
    global pending
    collect(0.0)
    if not eligible(position, history):
        return None
    fen = position_fen(position)
    if fen in cache:
        return response_move(position, history, cache[fen])
    seconds = min(MAX_WAIT, max(0.0, longest) / 4)
    if pending is not None or time.monotonic() < next_request or seconds < MIN_WAIT:
        return None
    results: Queue[Response] = Queue(maxsize=1)
    pending = (fen, results)
    worker = Thread(target=fetch, args=(fen, seconds, results), daemon=True)
    try:
        worker.start()
    except RuntimeError:
        pending = None
        return None
    collect(seconds)
    if fen in cache:
        return response_move(position, history, cache[fen])
    return None
