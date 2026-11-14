from board import EMPTY, KING, WHITE
from evaluation import PIECE_VALUES
from move_generation import generate_moves

KING_VALUE = 2000


def cheapest_attacker(board, square, by_white):
    best = -1
    best_value = KING_VALUE + 1
    for start, target, _ in generate_moves((board, by_white, 0, -1, 0)):
        kind = board[start] & 7
        value = KING_VALUE if kind == KING else PIECE_VALUES[kind]
        if target == square and value < best_value:
            best = start
            best_value = value
    return best


def exchange_score(board, move):
    start, target = move[0], move[1]
    board = board[:]
    gains = [PIECE_VALUES[board[target] & 7]]
    white = board[start] > WHITE
    board[target] = board[start]
    board[start] = EMPTY
    attacker = cheapest_attacker(board, target, not white)
    while attacker != -1:
        if board[target] & 7 == KING:
            if len(gains) == 1:
                return -KING_VALUE
            gains.pop()
            break
        gains.append(PIECE_VALUES[board[target] & 7] - gains[-1])
        board[target] = board[attacker]
        board[attacker] = EMPTY
        white = not white
        attacker = cheapest_attacker(board, target, not white)
    for i in range(len(gains) - 1, 0, -1):
        gains[i - 1] = -max(-gains[i - 1], gains[i])
    return gains[0]
