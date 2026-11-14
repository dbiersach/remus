from board import BISHOP, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE

KNIGHT_JUMPS = (
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    (1, -2), (1, 2), (2, -1), (2, 1),
)  # fmt: skip

DIRECTIONS = (
    (-1, 0), (1, 0), (0, -1), (0, 1),
    (-1, -1), (-1, 1), (1, -1), (1, 1),
)  # fmt: skip
STRAIGHT = DIRECTIONS[0:4]
DIAGONAL = DIRECTIONS[4:8]

PIECE_STEPS = (
    ((), False), ((), False), (KNIGHT_JUMPS, False), (DIAGONAL, True),
    (STRAIGHT, True), (DIRECTIONS, True), (DIRECTIONS, False),
)  # fmt: skip


def is_attacked(board, square, by_white):
    row = square // 8
    col = square % 8
    side = WHITE if by_white else 0
    pawn_steps = ((1, -1), (1, 1)) if by_white else ((-1, -1), (-1, 1))
    for steps, slides, attackers in (
        (pawn_steps, False, (PAWN + side,)),
        (KNIGHT_JUMPS, False, (KNIGHT + side,)),
        (DIRECTIONS, False, (KING + side,)),
        (STRAIGHT, True, (ROOK + side, QUEEN + side)),
        (DIAGONAL, True, (BISHOP + side, QUEEN + side)),
    ):
        for row_change, col_change in steps:
            r = row + row_change
            c = col + col_change
            while slides and 0 <= r < 8 and 0 <= c < 8 and board[r * 8 + c] == EMPTY:
                r += row_change
                c += col_change
            if 0 <= r < 8 and 0 <= c < 8 and board[r * 8 + c] in attackers:
                return True
    return False


def in_check(board, white):
    return is_attacked(board, board.index(KING + WHITE if white else KING), not white)
