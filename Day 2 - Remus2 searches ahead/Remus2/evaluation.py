import json
from pathlib import Path

from board import EMPTY, WHITE

with Path(__file__).with_name("piece_square_tables.json").open() as table_file:
    TABLE_DATA = json.load(table_file)

PAWN_TABLE = tuple(TABLE_DATA["PAWN_TABLE"])
KNIGHT_TABLE = tuple(TABLE_DATA["KNIGHT_TABLE"])
BISHOP_TABLE = tuple(TABLE_DATA["BISHOP_TABLE"])
KING_TABLE = tuple(TABLE_DATA["KING_TABLE"])

PIECE_VALUES = (0, 100, 300, 300, 500, 900, 0)

ZERO_TABLE = (0,) * 64

TABLES = (
    ZERO_TABLE, PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE,
    ZERO_TABLE, ZERO_TABLE, KING_TABLE,
)  # fmt: skip


def evaluate(position):
    board, white = position[0], position[1]
    score = 0
    for square in range(64):
        piece = board[square]
        if piece == EMPTY:
            continue
        kind = piece & 7
        if piece & WHITE:
            score += PIECE_VALUES[kind] + TABLES[kind][square]
        else:
            score -= (
                PIECE_VALUES[kind] + TABLES[kind][(7 - square // 8) * 8 + square % 8]
            )
    return score if white else -score
