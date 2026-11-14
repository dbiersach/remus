"""evaluation.py

Score a position in centipawns (a pawn is 100): the material each
side has, plus a bonus or penalty for where each piece stands. The
tables show White's view with rank 8 on top; a Black piece reads the
table upside down.
"""

import json
from pathlib import Path

from board import EMPTY, WHITE, Position

# Supplied tables hold all 64 squares: row 0 is rank 8, column 0 is file a.
with Path(__file__).with_name("piece_square_tables.json").open() as table_file:
    TABLE_DATA = json.load(table_file)

PAWN_TABLE = tuple(TABLE_DATA["PAWN_TABLE"])
KNIGHT_TABLE = tuple(TABLE_DATA["KNIGHT_TABLE"])
BISHOP_TABLE = tuple(TABLE_DATA["BISHOP_TABLE"])
KING_TABLE = tuple(TABLE_DATA["KING_TABLE"])

# What each kind of piece is worth, from empty (0) to king (6). The king is never
# captured, so it counts 0.
PIECE_VALUES = (0, 100, 300, 300, 500, 900, 0)

# Rooks and queens score the same on every square
ZERO_TABLE = (0,) * 64

# The table of each kind of piece, from empty (0) to king (6)
TABLES = (
    ZERO_TABLE, PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE,
    ZERO_TABLE, ZERO_TABLE, KING_TABLE,
)  # fmt: skip


def evaluate(position: Position) -> int:
    """Return the score for the side to move: positive when it is ahead."""
    board, white = position[0], position[1]
    score = 0  # From White's side
    for square in range(64):
        piece = board[square]
        if piece == EMPTY:
            continue
        kind = piece & 7
        if piece & WHITE:
            score += PIECE_VALUES[kind] + TABLES[kind][square]
        else:
            # Flip the row for Black: Black's a8 is White's a1
            score -= (
                PIECE_VALUES[kind] + TABLES[kind][(7 - square // 8) * 8 + square % 8]
            )
    return score if white else -score
