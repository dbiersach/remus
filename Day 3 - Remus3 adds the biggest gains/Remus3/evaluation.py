import json
from pathlib import Path

from board import BISHOP, EMPTY, PAWN, WHITE

with Path(__file__).with_name("piece_square_tables.json").open() as table_file:
    TABLE_DATA = json.load(table_file)

PAWN_TABLE = tuple(TABLE_DATA["PAWN_TABLE"])
KNIGHT_TABLE = tuple(TABLE_DATA["KNIGHT_TABLE"])
BISHOP_TABLE = tuple(TABLE_DATA["BISHOP_TABLE"])
KING_TABLE = tuple(TABLE_DATA["KING_TABLE"])
ROOK_TABLE = tuple(TABLE_DATA["ROOK_TABLE"])

PIECE_VALUES = (0, 100, 300, 300, 500, 900, 0)

ZERO_TABLE = (0,) * 64

TABLES = (
    ZERO_TABLE, PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE,
    ROOK_TABLE, ZERO_TABLE, KING_TABLE,
)  # fmt: skip

PASSED_BONUS = (0, 120, 80, 50, 30, 15, 10)
DOUBLED_PAWN = 15
ISOLATED_PAWN = 15
BISHOP_PAIR = 30


def pawn_terms(pawns):
    score = 0
    counts = ([0] * 10, [0] * 10)
    for side in (0, 1):
        for square in pawns[side]:
            counts[side][square % 8 + 1] += 1
    for side in (0, 1):
        sign = 1 if side == 0 else -1
        enemy = 1 - side
        for count in counts[side]:
            score -= sign * DOUBLED_PAWN * max(count - 1, 0)
        for square in pawns[side]:
            row = square // 8
            col = square % 8
            if counts[side][col] == 0 and counts[side][col + 2] == 0:
                score -= sign * ISOLATED_PAWN
            passed = True
            for other in pawns[enemy]:
                ahead = other // 8 < row if side == 0 else other // 8 > row
                if ahead and abs(other % 8 - col) <= 1:
                    passed = False
            if passed:
                steps = row if side == 0 else 7 - row
                score += sign * PASSED_BONUS[steps]
    return score


def evaluate(position):
    board, white = position[0], position[1]
    score = 0
    pawns = ([], [])
    for square in range(64):
        piece = board[square]
        if piece == EMPTY:
            continue
        kind = piece & 7
        if kind == PAWN:
            pawns[0 if piece & WHITE else 1].append(square)
        if piece & WHITE:
            score += PIECE_VALUES[kind] + TABLES[kind][square]
        else:
            score -= (
                PIECE_VALUES[kind] + TABLES[kind][(7 - square // 8) * 8 + square % 8]
            )
    if board.count(BISHOP + WHITE) >= 2:
        score += BISHOP_PAIR
    if board.count(BISHOP) >= 2:
        score -= BISHOP_PAIR
    score += pawn_terms(pawns)
    return score if white else -score
