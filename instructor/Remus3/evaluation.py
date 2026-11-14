"""evaluation.py

Score a position in centipawns (a pawn is 100): the material each
side has, a bonus or penalty for where each piece stands, and the
pawn structure (doubled, isolated, and passed pawns), and the bishop
pair. The tables show White's view with rank 8 on top; a Black piece
reads the table upside down.
"""

import json
from pathlib import Path

from board import BISHOP, EMPTY, PAWN, WHITE, Position

# Supplied tables hold all 64 squares: row 0 is rank 8, column 0 is file a.
with Path(__file__).with_name("piece_square_tables.json").open() as table_file:
    TABLE_DATA = json.load(table_file)

PAWN_TABLE = tuple(TABLE_DATA["PAWN_TABLE"])
KNIGHT_TABLE = tuple(TABLE_DATA["KNIGHT_TABLE"])
BISHOP_TABLE = tuple(TABLE_DATA["BISHOP_TABLE"])
KING_TABLE = tuple(TABLE_DATA["KING_TABLE"])
ROOK_TABLE = tuple(TABLE_DATA["ROOK_TABLE"])

# What each kind of piece is worth, from empty (0) to king (6). The king is never
# captured, so it counts 0.
PIECE_VALUES = (0, 100, 300, 300, 500, 900, 0)

# Queens score the same on every square
ZERO_TABLE = (0,) * 64

# The table of each kind of piece, from empty (0) to king (6)
TABLES = (
    ZERO_TABLE, PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE,
    ROOK_TABLE, ZERO_TABLE, KING_TABLE,
)  # fmt: skip

# The bonus of a passed pawn by the moves it needs to promote
PASSED_BONUS = (0, 120, 80, 50, 30, 15, 10)
DOUBLED_PAWN = 15  # For each extra pawn on a file
ISOLATED_PAWN = 15  # No friendly pawn on a file beside it
BISHOP_PAIR = 30


def pawn_terms(pawns) -> int:
    """
    Score the doubled, isolated, and passed pawns, from White's side.

    pawns holds the squares of White's pawns, then Black's.
    """
    score = 0
    # The pawns on each file, for each side. Entry col + 1 counts file col,
    # so the a and h files have an empty file on each side.
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
            # An enemy pawn ahead on this file or a file beside it can
            # stop the pawn
            passed = True
            for other in pawns[enemy]:
                ahead = other // 8 < row if side == 0 else other // 8 > row
                if ahead and abs(other % 8 - col) <= 1:
                    passed = False
            if passed:
                steps = row if side == 0 else 7 - row  # Moves to promotion
                score += sign * PASSED_BONUS[steps]
    return score


def evaluate(position: Position) -> int:
    """Return the score for the side to move: positive when it is ahead."""
    board, white = position[0], position[1]
    score = 0  # From White's side
    pawns = ([], [])  # The squares of White's pawns, then Black's
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
            # Flip the row for Black: Black's a8 is White's a1
            score -= (
                PIECE_VALUES[kind] + TABLES[kind][(7 - square // 8) * 8 + square % 8]
            )
    if board.count(BISHOP + WHITE) >= 2:
        score += BISHOP_PAIR
    if board.count(BISHOP) >= 2:
        score -= BISHOP_PAIR
    score += pawn_terms(pawns)
    return score if white else -score
