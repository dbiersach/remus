"""board.py

The board, the starting position, and the names of the squares.

The board is a list of 64 numbers, one per square. Row 0 is rank 8
and column 0 is file a, and square number row * 8 + col names a
square: a8 is 0, h8 is 7, a1 is 56, and h1 is 63. A piece is its
kind, from 1 for a pawn to 6 for a king, plus 8 if it is White, so
a White knight is 10 and a Black knight is 2. piece & 7 gives the
kind, and 0 is an empty square.

A position is the tuple (board, white, castling, en_passant): the
board, True when White is to move, the castling rights as four bits,
and the square a pawn just skipped over, or -1.
"""

EMPTY, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING = range(7)  # Piece kinds
WHITE = 8  # Added to the kind of a White piece

Position = tuple[list[int], bool, int, int]
Move = tuple[int, int, int]  # (from square, to square, promotion or 0)

# The starting position. Every engine can share this code
# Row 0 is rank 8 and column 0 is file a. Both sides use the same back rank.
BACK_RANK = [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK]
# [PAWN] * 8 is a list of eight pawns while [EMPTY] * 32 is a list of 32 empty squares.
# The starting board is the Black back rank, the Black pawns,
# 32 empty squares, the White pawns, and the White back rank.
START_BOARD = BACK_RANK + [PAWN] * 8 + [EMPTY] * 32 + [WHITE + PAWN] * 8
# The White pieces are the same as the Black pieces, but with WHITE added to the kind.
for kind in BACK_RANK:
    START_BOARD.append(WHITE + kind)
# True = White moves first
# 15 =  All four castling rights: 1 + 2 for White, 4 + 8 for Black
# -1 = No pawn has just moved two squares
START_POSITION = (START_BOARD, True, 15, -1)


def is_own(piece: int, white: bool) -> bool:
    """Return True if the piece belongs to White (white is True) or Black."""
    return piece > WHITE if white else EMPTY < piece < WHITE


def parse_square(text: str) -> int:
    """Return the square that text such as "e2" names."""
    # text[0] is the file letter and text[1] is the rank digit.
    # Row 0 is rank 8, so the ranks are listed from 8 down to 1: the position
    # of "2" in "87654321" is 6, so rank 2 is row 6.
    # The position of "e" in "abcdefgh" is 4, so file e is column 4.
    # Each row holds 8 squares, so e2 is square 6 * 8 + 4 = 52.
    return "87654321".index(text[1]) * 8 + "abcdefgh".index(text[0])


def square_name(square: int) -> str:
    """Return the name of a square, such as "e2"."""
    # This undoes parse_square(). For square 52, the remainder 52 % 8 = 4 is the
    # column, and letter 4 of "abcdefgh" is "e". The whole-number division
    # 52 // 8 = 6 is the row, and digit 6 of "87654321" is "2".
    return "abcdefgh"[square % 8] + "87654321"[square // 8]
