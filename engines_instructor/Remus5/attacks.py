"""attacks.py

Find out whether a square is attacked. Remus looks out from the
square itself: if a knight's jump away there stands an enemy
knight, the knight attacks the square, and the same idea works for
every piece.
"""

from board import BISHOP, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE

# The eight knight jumps, as (row change, column change)
KNIGHT_JUMPS = (
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    (1, -2), (1, 2), (2, -1), (2, 1),
)  # fmt: skip

# The eight directions: rooks use the first four, bishops the last
# four, and queens and kings all eight
DIRECTIONS = (
    (-1, 0), (1, 0), (0, -1), (0, 1),
    (-1, -1), (-1, 1), (1, -1), (1, 1),
)  # fmt: skip
STRAIGHT = DIRECTIONS[0:4]  # The rook's four directions
DIAGONAL = DIRECTIONS[4:8]  # The bishop's four directions


def is_attacked(board: list[int], square: int, by_white: bool) -> bool:
    """Return True if a piece of the given color attacks the square."""
    row = square // 8
    col = square % 8
    # Add 8 to each kind when White attacks
    side = WHITE if by_white else 0
    # A White pawn attacks diagonally forward, so it stands one row
    # below the square (row + 1)
    pawn_steps = ((1, -1), (1, 1)) if by_white else ((-1, -1), (-1, 1))
    # Each kind of attacker: the steps it takes from the square, whether
    # it slides on past empty squares, and the pieces that attack that way
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
            # The first piece on the line attacks if it is the right kind
            if 0 <= r < 8 and 0 <= c < 8 and board[r * 8 + c] in attackers:
                return True
    return False


def in_check(board: list[int], white: bool) -> bool:
    """Return True if the king of the given color is attacked."""
    return is_attacked(board, board.index(KING + WHITE if white else KING), not white)
