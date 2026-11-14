"""search.py

Choose a move by alpha-beta search: look a few moves ahead, assume
both sides play their best, and skip the moves that cannot change
the result. Quiescence search plays out the captures at the end of
each line, and iterative deepening searches one move deeper at a
time until the time is up.
"""

import time

from attacks import in_check
from board import EMPTY, QUEEN, Move, Position
from evaluation import PIECE_VALUES, evaluate
from make_move import make_move
from move_generation import generate_moves

MATE_SCORE = 100_000  # Larger than any material count


class SearchTimeout(Exception):
    """Raised when the time for a move is up."""


# What the search keeps between calls: the positions searched, the
# clock time to stop, and the best root move so far
nodes = 0
deadline = 0.0
best_move: Move | None = None


def ordered_moves(position: Position, first: Move | None):
    """
    Return the moves, the likely best first, each as a pair
    (priority, move).

    A big capture by a small piece comes first (pawn takes queen
    before queen takes pawn), then quiet moves, whose priority is 0.
    The move first, the best move of the last search, goes before all
    of them.
    """
    board = position[0]
    scored = []
    for move in generate_moves(position):
        start, target, promotion = move
        priority = 0
        if board[target] != EMPTY:
            victim = board[target] & 7
            attacker = board[start] & 7
            priority = 10 * PIECE_VALUES[victim] - PIECE_VALUES[attacker]
        if promotion == QUEEN:
            priority += 9000
        if move == first:
            priority = 100_000
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
    for priority, move in ordered_moves(position, None):
        if priority == 0:
            break  # Only quiet moves remain
        child = make_move(position, move)
        if in_check(child[0], position[1]):
            continue  # Illegal: it leaves the king attacked
        score = -quiescence(child, -beta, -alpha)  # Recursive call to itself
        if score >= beta:
            return score
        alpha = max(alpha, score)
    return alpha


def alpha_beta(position: Position, depth: int, ply: int, alpha: int, beta: int):
    """
    Return the score for the side to move, searching depth moves ahead.

    ply counts the moves from the root. alpha is the score the side to
    move is already sure of, and beta the score the opponent will not
    allow.
    """
    global best_move
    count_node()
    check = in_check(position[0], position[1])
    if check:
        depth += 1  # Never stop the search in check
    if depth <= 0:
        return quiescence(position, alpha, beta)
    has_legal_move = False
    # The underscore _ takes the priority, which only quiescence() needs
    for _, move in ordered_moves(position, best_move if ply == 0 else None):
        child = make_move(position, move)
        if in_check(child[0], position[1]):
            continue  # Illegal: it leaves the king attacked
        has_legal_move = True
        # Recursive call to itself
        score = -alpha_beta(child, depth - 1, ply + 1, -beta, -alpha)
        if score >= beta:
            return score  # The opponent avoids this line
        if score > alpha:
            alpha = score
            if ply == 0:
                best_move = move
    if not has_legal_move:
        # Checkmate, worse the sooner it comes, or stalemate
        return -(MATE_SCORE - ply) if check else 0
    return alpha


def think(position: Position, seconds: float) -> Move | None:
    """
    Search one move deeper at a time until the time is up, and return
    the best move, or None when there is no legal move.
    """
    global nodes, deadline, best_move
    deadline = time.perf_counter() + seconds
    nodes = 0
    best_move = None
    for depth in range(1, 65):
        try:
            score = alpha_beta(position, depth, 0, -MATE_SCORE - 1, MATE_SCORE + 1)
        except SearchTimeout:
            break
        if best_move is None:
            break  # No legal move
        print(f"info depth {depth} score cp {score} nodes {nodes}", flush=True)
        # Stop at a forced mate, or when the next depth would not
        # finish in the half of the time that is left
        if (
            abs(score) > MATE_SCORE - 100
            or time.perf_counter() > deadline - seconds / 2
        ):
            break
    return best_move
