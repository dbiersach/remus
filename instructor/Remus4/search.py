"""search.py

Choose a move by alpha-beta search with every enhancement of the plan:
a transposition table, killer moves, principal variation search,
null-move pruning, late move reductions, futility pruning, draw sense,
and a quiescence search with delta pruning and static exchange
evaluation. Iterative deepening searches one move deeper at a time
inside an aspiration window, and stops by the clock.
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
from exchange import exchange_score
from make_move import make_move
from move_generation import generate_moves

FUTILITY_MARGIN = 200  # A quiet move seldom gains more than this
DELTA_MARGIN = 200  # A capture seldom gains more than its victim plus this
ASPIRATION_WINDOW = 50  # The first window around the last score
CAPTURE = 10_000  # Every capture and queen promotion scores above this


class SearchTimeout(Exception):
    """Raised when the time for a move is up."""


nodes = 0  # Positions searched
deadline = 0.0  # The clock time to stop
best_move: Move | None = None  # The best root move so far
history: list[int] = []  # Keys of the positions before this one
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
    if nodes % 1024 == 0 and best_move is not None and time.perf_counter() > deadline:
        raise SearchTimeout


def quiescence(position: Position, alpha: int, beta: int) -> int:
    """
    Search only winning or even captures and queen promotions until the
    position is quiet. The side to move may also "stand pat".
    """
    count_node()
    board, white = position[0], position[1]
    stand_pat = evaluate(position)
    if stand_pat >= beta:
        return stand_pat
    alpha = max(alpha, stand_pat)
    for priority, move in ordered_moves(position, None, ()):
        if priority < CAPTURE:
            break  # Only quiet moves remain
        if not move[2]:
            # Delta pruning: even winning the victim cannot reach alpha
            victim = PIECE_VALUES[board[move[1]] & 7]
            if stand_pat + victim + DELTA_MARGIN < alpha:
                continue
            if exchange_score(board, move) < 0:
                continue  # The capture loses material
        child = make_move(position, move)
        if in_check(child[0], white):
            continue  # Illegal: it leaves the king attacked
        # Recursive call to itself
        score = -quiescence(child, -beta, -alpha)
        if score >= beta:
            return score
        alpha = max(alpha, score)
    return alpha


def is_draw(position: Position, key: int) -> bool:
    """Return True for the fifty-move rule or a repeated position."""
    halfmove = position[4]
    return halfmove >= 100 or key in history[len(history) - halfmove :]


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
    key = position_key(position)
    if ply > 0 and is_draw(position, key):
        return 0
    check = in_check(board, white)
    if check:
        depth += 1  # Never stop the search in check
    if depth <= 0:
        return quiescence(position, alpha, beta)
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
        null = (board, not white, position[2], -1, position[4] + 1)
        if has_pieces(board, white):
            history.append(key)
            score = -alpha_beta(null, depth - 3, ply + 1, -beta, -beta + 1, False)
            history.pop()
            if score >= beta:
                return beta
    history.append(key)
    original_alpha = alpha
    best_score = -MATE_SCORE - 1
    best = None
    legal_count = 0
    # The underscore _ takes the priority, which only quiescence() needs
    for _, move in ordered_moves(position, first, killers[ply]):
        child = make_move(position, move)
        if in_check(child[0], white):
            continue  # Illegal: it leaves the king attacked
        legal_count += 1
        quiet = board[move[1]] == EMPTY and not move[2]
        calm = quiet and not check and not in_check(child[0], not white)
        # Futility: one move from the leaves, a quiet move cannot lift a
        # position far below alpha
        if (
            depth == 1
            and calm
            and legal_count > 1
            and static + FUTILITY_MARGIN <= alpha
        ):
            continue
        if legal_count == 1:
            score = -alpha_beta(child, depth - 1, ply + 1, -beta, -alpha)
        else:
            # Late move reduction: a late quiet move gets one ply less
            reduce = (
                depth >= 3 and legal_count > 3 and calm and move not in killers[ply]
            )
            # Principal variation search: prove the move is no better
            # than alpha with a null window, and search again if it is
            score = -alpha_beta(child, depth - 1 - reduce, ply + 1, -alpha - 1, -alpha)
            if score > alpha and (reduce or score < beta):
                score = -alpha_beta(child, depth - 1, ply + 1, -beta, -alpha)
        if score > best_score:
            best_score = score
            best = move
        if score > alpha:
            alpha = score
            if ply == 0:
                best_move = move
        if score >= beta:
            if quiet and killers[ply][0] != move:
                killers[ply] = [move, killers[ply][0]]
            break
    history.pop()
    if legal_count == 0:
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


def think(position, game_history, seconds, longest) -> Move | None:
    """
    Search one move deeper at a time and return the best move, or None
    when there is no legal move. The search plans to use seconds, takes
    longer when the best move keeps changing, and never runs past longest.
    """
    global nodes, deadline, best_move, history
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
                history = game_history[:]
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
