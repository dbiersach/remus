import json
from pathlib import Path

from attacks import PIECE_STEPS
from board import BISHOP, EMPTY, KING, PAWN, ROOK, WHITE
from move_generation import piece_moves

with Path(__file__).with_name("piece_square_tables.json").open() as table_file:
    TABLE_DATA = json.load(table_file)

PAWN_TABLE = tuple(TABLE_DATA["PAWN_TABLE"])
KNIGHT_TABLE = tuple(TABLE_DATA["KNIGHT_TABLE"])
BISHOP_TABLE = tuple(TABLE_DATA["BISHOP_TABLE"])
KING_MIDDLE_TABLE = tuple(TABLE_DATA["KING_TABLE"])
ROOK_TABLE = tuple(TABLE_DATA["ROOK_TABLE"])
KING_END_TABLE = tuple(TABLE_DATA["KING_END_TABLE"])

PIECE_VALUES = (0, 100, 300, 300, 500, 900, 0)

ZERO_TABLE = (0,) * 64

TABLES = (
    ZERO_TABLE, PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE,
    ROOK_TABLE, ZERO_TABLE, ZERO_TABLE,
)  # fmt: skip

PHASE_VALUES = (0, 0, 1, 1, 2, 4, 0)
PASSED_BONUS = (0, 120, 80, 50, 30, 15, 10)
MOBILITY_BONUS = 2
BISHOP_PAIR = 30
ROOK_OPEN_FILE = 20
ROOK_HALF_OPEN_FILE = 10
DOUBLED_PAWN = 15
ISOLATED_PAWN = 15
UNSTOPPABLE_PAWN = 500
SHIELD_PAWN = 10
OPEN_KING_FILE = 15


def king_safety(pawns, counts, king, side, phase):
    col = king % 8
    if king // 8 != (7 if side == 0 else 0):
        return 0
    forward = -8 if side == 0 else 8
    score = 0
    for c in range(max(col - 1, 0), min(col + 2, 8)):
        if king + forward + c - col in pawns[side]:
            score += SHIELD_PAWN
        if counts[side][c + 1] == 0:
            score -= OPEN_KING_FILE
    return score * phase // 24


def pawn_terms(pawns, rooks, kings, phase, white_to_move):
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
            if not passed:
                continue
            steps = row if side == 0 else 7 - row
            score += sign * PASSED_BONUS[steps]
            king = kings[enemy]
            king_steps = max(abs(king // 8 - 7 * side), abs(king % 8 - col))
            if white_to_move != (side == 0):
                king_steps -= 1
            if phase == 0 and king_steps > min(steps, 5):
                score += sign * UNSTOPPABLE_PAWN
        for square in rooks[side]:
            if counts[side][square % 8 + 1] == 0:
                open_file = counts[enemy][square % 8 + 1] == 0
                score += sign * (ROOK_OPEN_FILE if open_file else ROOK_HALF_OPEN_FILE)
        score += sign * king_safety(pawns, counts, kings[side], side, phase)
    return score


def mating_bonus(material, kings, phase):
    if phase > 8 or abs(material) < 400:
        return 0
    winner = 0 if material > 0 else 1
    loser_row = kings[1 - winner] // 8
    loser_col = kings[1 - winner] % 8
    edge = max(3 - loser_row, loser_row - 4) + max(3 - loser_col, loser_col - 4)
    distance = max(abs(kings[0] // 8 - kings[1] // 8), abs(kings[0] % 8 - kings[1] % 8))
    bonus = 10 * edge + 5 * (7 - distance)
    return bonus if winner == 0 else -bonus


def evaluate(position):
    board, white = position[0], position[1]
    score = 0
    material = 0
    phase = 0
    kings = [0, 0]
    pawns = ([], [])
    rooks = ([], [])
    for square in range(64):
        piece = board[square]
        if piece == EMPTY:
            continue
        kind = piece & 7
        side = 0 if piece & WHITE else 1
        sign = 1 if side == 0 else -1
        view = square if side == 0 else (7 - square // 8) * 8 + square % 8
        material += sign * PIECE_VALUES[kind]
        score += sign * TABLES[kind][view]
        if kind == KING:
            kings[side] = square
        elif kind == PAWN:
            pawns[side].append(square)
        else:
            phase += PHASE_VALUES[kind]
            moves = []
            steps, slides = PIECE_STEPS[kind]
            piece_moves(board, side == 0, square, steps, slides, moves)
            score += sign * MOBILITY_BONUS * len(moves)
            if kind == ROOK:
                rooks[side].append(square)
    if phase <= 1 and not pawns[0] and not pawns[1]:
        return 0
    phase = min(phase, 24)
    for side in (0, 1):
        sign = 1 if side == 0 else -1
        king = kings[side]
        view = king if side == 0 else (7 - king // 8) * 8 + king % 8
        taper = KING_MIDDLE_TABLE[view] * phase + KING_END_TABLE[view] * (24 - phase)
        score += sign * taper // 24
    if board.count(BISHOP + WHITE) >= 2:
        score += BISHOP_PAIR
    if board.count(BISHOP) >= 2:
        score -= BISHOP_PAIR
    score += material + pawn_terms(pawns, rooks, kings, phase, white)
    score += mating_bonus(material, kings, phase)
    return score if white else -score
