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
    move_text,
    position_key,
)
from evaluation import PIECE_VALUES, evaluate
from exchange import exchange_score
from make_move import make_move
from move_generation import generate_moves

FUTILITY_MARGIN = 200
DELTA_MARGIN = 200
ASPIRATION_WINDOW = 50
CAPTURE = 10_000


class SearchTimeout(Exception):
    pass


nodes = 0
deadline = 0.0
best_move = None
history = []
killers = []
for _ in range(128):
    killers.append([None, None])


def ordered_moves(position, first, killer_pair):
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
    scored.sort(reverse=True)
    return scored


def count_node():
    global nodes
    nodes += 1
    if nodes % 1024 == 0 and best_move is not None and time.perf_counter() > deadline:
        raise SearchTimeout


def quiescence(position, alpha, beta):
    count_node()
    board, white = position[0], position[1]
    stand_pat = evaluate(position)
    if stand_pat >= beta:
        return stand_pat
    alpha = max(alpha, stand_pat)
    for priority, move in ordered_moves(position, None, ()):
        if priority < CAPTURE:
            break
        if not move[2]:
            victim = PIECE_VALUES[board[move[1]] & 7]
            if stand_pat + victim + DELTA_MARGIN < alpha:
                continue
            if exchange_score(board, move) < 0:
                continue
        child = make_move(position, move)
        if in_check(child[0], white):
            continue
        score = -quiescence(child, -beta, -alpha)
        if score >= beta:
            return score
        alpha = max(alpha, score)
    return alpha


def is_draw(position, key):
    halfmove = position[4]
    return halfmove >= 100 or key in history[len(history) - halfmove :]


def has_pieces(board, white):
    count = 0
    side = WHITE if white else 0
    for kind in (KNIGHT, BISHOP, ROOK, QUEEN):
        count += board.count(kind + side)
    return count > 0


def alpha_beta(position, depth, ply, alpha, beta, can_null=True):
    global best_move
    count_node()
    board, white = position[0], position[1]
    key = position_key(position)
    if ply > 0 and is_draw(position, key):
        return 0
    check = in_check(board, white)
    if check:
        depth += 1
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
    for _, move in ordered_moves(position, first, killers[ply]):
        child = make_move(position, move)
        if in_check(child[0], white):
            continue
        legal_count += 1
        quiet = board[move[1]] == EMPTY and not move[2]
        calm = quiet and not check and not in_check(child[0], not white)
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
            reduce = (
                depth >= 3 and legal_count > 3 and calm and move not in killers[ply]
            )
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
        return -(MATE_SCORE - ply) if check else 0
    bound = tt.EXACT
    if best_score >= beta:
        bound = tt.LOWER
    elif best_score <= original_alpha:
        bound = tt.UPPER
    tt.store(key, depth, best_score, bound, best, ply)
    return best_score


def principal_variation(position, depth):
    line = ""
    for _ in range(depth):
        stored = tt.probe(position_key(position), 0)
        if stored is None or stored[3] is None:
            break
        line += " " + move_text(stored[3])
        position = make_move(position, stored[3])
    return line


def think(position, game_history, seconds, longest):
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
                if score <= alpha:
                    alpha = -MATE_SCORE - 1
                elif score >= beta:
                    beta = MATE_SCORE + 1
                else:
                    break
        except SearchTimeout:
            break
        if best_move is None:
            break
        line = principal_variation(position, depth)
        print(f"info depth {depth} score cp {score} nodes {nodes} pv{line}", flush=True)
        if previous is not None and best_move != previous:
            seconds = min(seconds * 1.5, longest)
        if abs(score) > MATE_SCORE - 100 or time.perf_counter() > start + seconds / 2:
            break
    return best_move
