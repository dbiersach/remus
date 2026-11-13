"""uci.py

Talk to a chess GUI such as Lucas Chess through the Universal Chess
Interface (UCI). The GUI sends one command per line, and Remus answers
each one. Moves are written from square to square, such as e2e4, with a
fifth letter for a promotion, such as e7e8q. The promotion letter's
place in ".pnbrqk" is the kind of piece.
"""

import time

import tablebase
import transposition_table as tt
from attacks import in_check
from board import START_POSITION, Move, Position, move_text, parse_square, position_key
from make_move import make_move
from move_generation import generate_moves
from opening_book import book_moves, load_book
from search import think
from search_control import SearchControl

HIT_GAP = 2.0  # Maximum seconds between a stopped ponder and its matching go


def read_time(words: list[str], name: str) -> int:
    """Return the milliseconds after a word of "go", such as wtime, or 0."""
    if name in words:
        return int(words[words.index(name) + 1])
    return 0


def legal_book_move(position: Position, played: list[str]) -> Move | None:
    """Return a legal move from the opening book, or None."""
    for text in book_moves(played, position[1]):
        for move in generate_moves(position):
            child = make_move(position, move)
            if move_text(move) == text and not in_check(child[0], position[1]):
                return move
    return None


def legal_choice(position: Position, preferred: Move | None = None) -> Move | None:
    """Return a legal preferred move, or the first legal move as a fallback."""
    first = None
    for move in generate_moves(position):
        if not in_check(make_move(position, move)[0], position[1]):
            if move == preferred:
                return move
            if first is None:
                first = move
    return first


def ponder_reply(position: Position, move: Move, played: list[str]) -> Move | None:
    """Prefer a cached reply, then a book reply, then a legal prediction."""
    child = make_move(position, move)
    stored = tt.probe(position_key(child), 0)
    if stored is not None and stored[3] is not None:
        reply = legal_choice(child, stored[3])
        if reply == stored[3]:
            return reply
    reply = legal_book_move(child, played + [move_text(move)])
    return legal_choice(child, reply)


def choose_move(
    position: Position,
    played: list[str],
    game_history: list[int],
    seconds: float,
    longest: float,
    use_tablebase: bool,
    control: SearchControl,
) -> Move | None:
    """Use root advice on our turn, or search natively while pondering."""
    started = time.perf_counter()
    move = None
    if control.mode == "normal" and not control.was_ponder:
        if use_tablebase:
            move = tablebase.lookup(position, game_history, longest)
        if move is not None:
            print("info string Lichess tablebase", flush=True)
            control.source = "tablebase"
    if move is None and control.mode != "infinite":
        move = legal_book_move(position, played)
        if move is not None:
            control.source = "book"
    control.log("root_choice", source=control.source)
    if move is None:
        elapsed = time.perf_counter() - started
        move = think(
            position,
            game_history,
            max(0.0, seconds - elapsed),
            max(0.0, longest - elapsed),
            control,
        )
    return legal_choice(position, move)


def main():
    load_book()
    control = SearchControl()
    use_tablebase = True
    use_ponder = False
    pondered = None
    position_words = ["position", "startpos"]
    position_text = "position startpos"
    position = START_POSITION
    played = []  # The moves of the game, as UCI text
    game_history = []  # The keys of the positions before this one
    while True:
        command = control.next_command().strip()
        words = command.split()
        if not words:
            continue
        control.log("command", command=command)
        if words[0] == "uci":
            print("id name Remus5")
            print("option name LichessTablebase type check default true")
            print("option name Ponder type check default false")
            print("option name PonderDiagnostics type check default false")
            print("uciok", flush=True)
        elif len(words) == 5 and words[:2] == ["setoption", "name"]:
            pondered = None
            enabled = words[4].lower() == "true"
            if words[2].lower() == "lichesstablebase":
                use_tablebase = enabled
            elif words[2].lower() == "ponder":
                use_ponder = enabled
            elif words[2].lower() == "ponderdiagnostics":
                control.diagnostics = enabled
                control.log("enabled", engine="Remus5")
        elif words[0] == "isready":
            print("readyok", flush=True)
        elif words[0] == "ucinewgame":
            pondered = None
            position_words = ["position", "startpos"]
            position_text = "position startpos"
            position = START_POSITION
            played = []
            game_history = []
        elif words[0] == "position":
            position_text = command
            position_words = words.copy()
            position = START_POSITION
            played = words[3:]
            game_history = []
            for text in played:
                game_history.append(position_key(position))
                promotion = ".pnbrqk".index(text[4]) if len(text) > 4 else 0
                move = (parse_square(text[0:2]), parse_square(text[2:4]), promotion)
                position = make_move(position, move)
        elif words[0] == "go":
            white = position[1]
            clock_left = read_time(words, "wtime" if white else "btime")
            increment = read_time(words, "winc" if white else "binc")
            # Plan a thirtieth of the clock plus half the increment.
            seconds = longest = 2.0
            if clock_left > 0:
                seconds = (clock_left / 30 + increment / 2) / 1000
                longest = min(3 * seconds, clock_left / 5000)
            if "movetime" in words:
                seconds = longest = read_time(words, "movetime") / 1000
            mode = "normal"
            if "ponder" in words:
                mode = "ponder"
            elif "infinite" in words:
                mode = "infinite"
            control.begin(mode, longest)
            control.can_reinvest = clock_left >= 60_000 and "movetime" not in words
            control.log(
                "search_start",
                mode=mode,
                command=command,
                position=position_text,
                plan_ms=seconds * 1000,
                hard_budget_ms=longest * 1000,
            )
            if pondered is not None:
                saved_words, credit, ended, source_search = pondered
                if (
                    mode == "normal"
                    and saved_words == position_words
                    and time.perf_counter() - ended < HIT_GAP
                ):
                    control.credit_ponder(*credit)
                    control.log(
                        "hit",
                        kind="stop_go",
                        source_search=source_search,
                        credited_ms=credit[0] * 1000,
                        depth=credit[1],
                    )
            pondered = None
            move = choose_move(
                position, played, game_history, seconds, longest, use_tablebase, control
            )
            control.wait()
            credit = control.ponder_credit()
            if credit is not None:
                pondered = (
                    position_words.copy(),
                    credit,
                    time.perf_counter(),
                    control.search_id,
                )
            if control.diagnostics:
                control.report()
            text = "0000" if move is None else move_text(move)
            if use_ponder and move is not None:
                reply = ponder_reply(position, move, played)
                if reply is not None:
                    text += " ponder " + move_text(reply)
            print("bestmove " + text, flush=True)
            control.log("bestmove", command="bestmove " + text)
        elif words[0] == "quit":
            break


if __name__ == "__main__":
    main()
