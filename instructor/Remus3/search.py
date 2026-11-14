"""search.py

Choose a move by alpha-beta search with a transposition table, killer
moves, and null-move pruning. Quiescence search plays out the captures
at the end of each line, and iterative deepening searches one move
deeper at a time inside an aspiration window, and stops by the clock.
"""

import time

import transposition_table as tt
from attacks import in_check
from board import (
    BISHOP,
    EMPTY,
    KNIGHT,
    MATE_SCORE,
    QUEEN,
    ROOK,
    WHITE,
    Move,
    Position,
    move_text,
    position_key,
)
from evaluation import PIECE_VALUES, evaluate
from make_move import make_move
from move_generation import generate_moves

ASPIRATION_WINDOW = 50  # The first window around the last score
CAPTURE = 10_000  # Every capture and queen promotion scores above this


class SearchTimeout(Exception):
    """Raised when the time for a move is up."""


nodes = 0  # Positions searched
deadline = 0.0  # The clock time to stop
best_move: Move | None = None  # The best root move so far
killers: list[list[Move | None]] = []  # Two quiet cutoff moves per ply
for _ in range(128):
    killers.append([None, None])


def ordered_moves(position: Position, first: Move | None, killer_pair):
    """
    Return the moves, the likely best first, each as a pair (priority,
    move): the move first (from the table), then captures by MVV-LVA
    (pawn takes queen before queen takes pawn), then the killer moves,
    then quiet moves. Every capture and queen promotion has a priority
    from CAPTURE up, and only the move first scores higher.
    """
    board = position[0]
    scored = []
    for move in generate_moves(position):
        start, target, promotion = move
        priority = 0
        if board[target] != EMPTY:
            victim = board[target] & 7
            attacker = board[start] & 7
            priority = CAPTURE + 10 * PIECE_VALUES[victim] - PIECE_VALUES[attacker]
        if promotion == QUEEN:
            priority += CAPTURE + 9000
        if move in killer_pair:
            priority = 5000 if move == killer_pair[0] else 4000
        if move == first:
            priority = 1_000_000
        scored.append((priority, move))
    # Python sorts pairs by their first item, and by the move when two
    # priorities are equal
    scored.sort(reverse=True)
    return scored


def count_node():
    """Count one more position, and look at the clock every 1,024."""
    global nodes
    nodes += 1
    # Remus never stops before it has a move to play
    if nodes % 1024 == 0 and best_move is not None and time.perf_counter() > deadline:
        raise SearchTimeout


def quiescence(position: Position, alpha: int, beta: int) -> int:
    """
    Search only captures and queen promotions until the position is
    quiet, so Remus never stops in the middle of an exchange. The side
    to move may also "stand pat": keep the score it has.
    """
    count_node()
    stand_pat = evaluate(position)
    if stand_pat >= beta:
        return stand_pat
    alpha = max(alpha, stand_pat)
    for priority, move in ordered_moves(position, None, ()):
        if priority < CAPTURE:
            break  # Only quiet moves remain
        child = make_move(position, move)
        if in_check(child[0], position[1]):
            continue  # Illegal: it leaves the king attacked
        score = -quiescence(child, -beta, -alpha)  # Recursive call to itself
        if score >= beta:
            return score
        alpha = max(alpha, score)
    return alpha


def has_pieces(board: list[int], white: bool) -> bool:
    """Return True if the side has a piece other than its king and pawns."""
    count = 0
    side = WHITE if white else 0
    for kind in (KNIGHT, BISHOP, ROOK, QUEEN):
        count += board.count(kind + side)
    return count > 0


def alpha_beta(position, depth, ply, alpha, beta, can_null=True) -> int:
    """
    Return the score for the side to move, searching depth moves ahead.

    ply counts the moves from the root. alpha is the score the side to
    move is already sure of, and beta the score the opponent will not
    allow. can_null is False right after a null move.
    """
    global best_move
    count_node()
    board, white = position[0], position[1]
    check = in_check(board, white)
    if check:
        depth += 1  # Never stop the search in check
    if depth <= 0:
        return quiescence(position, alpha, beta)
    key = position_key(position)
    stored = tt.probe(key, ply)
    first = None
    if stored is not None:
        stored_depth, score, bound, first = stored
        if ply > 0 and stored_depth >= depth:
            if bound == tt.EXACT:
                return score
            if (bound == tt.LOWER and score >= beta) or (
                bound == tt.UPPER and score <= alpha
            ):
                return score
    static = evaluate(position)
    # Null move: let the opponent move twice. If the side to move still
    # stands above beta with a shallower search, its real moves will too.
    if can_null and not check and ply > 0 and depth >= 3 and static >= beta:
        null = (board, not white, position[2], -1)
        if has_pieces(board, white):
            score = -alpha_beta(null, depth - 3, ply + 1, -beta, -beta + 1, False)
            if score >= beta:
                return beta
    original_alpha = alpha
    best_score = -MATE_SCORE - 1
    best = None
    # The underscore _ takes the priority, which only quiescence() needs
    for _, move in ordered_moves(position, first, killers[ply]):
        child = make_move(position, move)
        if in_check(child[0], white):
            continue  # Illegal: it leaves the king attacked
        # Recursive call to itself
        score = -alpha_beta(child, depth - 1, ply + 1, -beta, -alpha)
        if score > best_score:
            best_score = score
            best = move
        if score > alpha:
            alpha = score
            if ply == 0:
                best_move = move
        if score >= beta:
            # A quiet move that cuts off here may cut off in the
            # positions beside this one too
            quiet = board[move[1]] == EMPTY and not move[2]
            if quiet and killers[ply][0] != move:
                killers[ply] = [move, killers[ply][0]]
            break
    if best is None:
        # Checkmate, worse the sooner it comes, or stalemate
        return -(MATE_SCORE - ply) if check else 0
    bound = tt.EXACT
    if best_score >= beta:
        bound = tt.LOWER
    elif best_score <= original_alpha:
        bound = tt.UPPER
    tt.store(key, depth, best_score, bound, best, ply)
    return best_score


def principal_variation(position: Position, depth: int) -> str:
    """Return the line of best play that the table holds, as UCI text."""
    line = ""
    for _ in range(depth):
        stored = tt.probe(position_key(position), 0)
        if stored is None or stored[3] is None:
            break
        line += " " + move_text(stored[3])
        position = make_move(position, stored[3])
    return line


def think(position: Position, seconds: float, longest: float) -> Move | None:
    """
    Search one move deeper at a time and return the best move, or None
    when there is no legal move. The search plans to use seconds, takes
    longer when the best move keeps changing, and never runs past longest.
    """
    global nodes, deadline, best_move
    start = time.perf_counter()
    deadline = start + longest
    nodes = 0
    best_move = None
    score = 0
    for depth in range(1, 65):
        previous = best_move
        alpha, beta = -MATE_SCORE - 1, MATE_SCORE + 1
        if depth >= 4:
            alpha, beta = score - ASPIRATION_WINDOW, score + ASPIRATION_WINDOW
        try:
            while True:
                score = alpha_beta(position, depth, 0, alpha, beta)
                # Outside the window: search again with that side open
                if score <= alpha:
                    alpha = -MATE_SCORE - 1
                elif score >= beta:
                    beta = MATE_SCORE + 1
                else:
                    break
        except SearchTimeout:
            break
        if best_move is None:
            break  # No legal move
        line = principal_variation(position, depth)
        print(f"info depth {depth} score cp {score} nodes {nodes} pv{line}", flush=True)
        # A best move that changes needs more time to settle
        if previous is not None and best_move != previous:
            seconds = min(seconds * 1.5, longest)
        if abs(score) > MATE_SCORE - 100 or time.perf_counter() > start + seconds / 2:
            break
    return best_move
