import time

from attacks import in_check
from board import EMPTY, QUEEN
from evaluation import PIECE_VALUES, evaluate
from make_move import make_move
from move_generation import generate_moves

MATE_SCORE = 100_000


class SearchTimeout(Exception):
    pass


nodes = 0
deadline = 0.0
best_move = None


def ordered_moves(position, first):
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
    scored.sort(reverse=True)
    return scored


def count_node():
    global nodes
    nodes += 1
    if nodes % 1024 == 0 and best_move is not None and time.perf_counter() > deadline:
        raise SearchTimeout


def quiescence(position, alpha, beta):
    count_node()
    stand_pat = evaluate(position)
    if stand_pat >= beta:
        return stand_pat
    alpha = max(alpha, stand_pat)
    for priority, move in ordered_moves(position, None):
        if priority == 0:
            break
        child = make_move(position, move)
        if in_check(child[0], position[1]):
            continue
        score = -quiescence(child, -beta, -alpha)
        if score >= beta:
            return score
        alpha = max(alpha, score)
    return alpha


def alpha_beta(position, depth, ply, alpha, beta):
    global best_move
    count_node()
    check = in_check(position[0], position[1])
    if check:
        depth += 1
    if depth <= 0:
        return quiescence(position, alpha, beta)
    has_legal_move = False
    for _, move in ordered_moves(position, best_move if ply == 0 else None):
        child = make_move(position, move)
        if in_check(child[0], position[1]):
            continue
        has_legal_move = True
        score = -alpha_beta(child, depth - 1, ply + 1, -beta, -alpha)
        if score >= beta:
            return score
        if score > alpha:
            alpha = score
            if ply == 0:
                best_move = move
    if not has_legal_move:
        return -(MATE_SCORE - ply) if check else 0
    return alpha


def think(position, seconds):
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
            break
        print(f"info depth {depth} score cp {score} nodes {nodes}", flush=True)
        if (
            abs(score) > MATE_SCORE - 100
            or time.perf_counter() > deadline - seconds / 2
        ):
            break
    return best_move
