"""make_move.py

Play a move. Remus never takes a move back: make_move() copies the
board, plays the move on the copy, and returns the new position.
The search throws the copy away when it is done with it.
"""

from board import EMPTY, KING, PAWN, WHITE, Move, Position

# The castling rights are four on-off switches added into one number:
# 1 = White may castle kingside, 2 = White queenside, 4 = Black kingside,
# 8 = Black queenside. All four on is 1 + 2 + 4 + 8 = 15.
# The dictionary below looks up a square and gives the switches that a
# move from or to that square turns off for good. The White king leaving
# e1 (square 60) turns off 1 + 2 = 3, both of White's rights. A rook
# leaving h1 (63), or a capture there, turns off 1.
# The squares: e1 60, h1 63, a1 56, e8 4, h8 7, a8 0
RIGHTS_OF_SQUARE = {60: 3, 63: 1, 56: 2, 4: 12, 7: 4, 0: 8}


def make_move(position: Position, move: Move) -> Position:
    """Return the position after the move, including the special moves."""
    board, white, castling, en_passant, halfmove = position
    start, target, promotion = move
    board = board[:]  # The copy: the caller's board stays as it was
    piece = board[start]
    is_pawn = piece & 7 == PAWN
    distance = abs(target - start)
    # A capture or pawn move restarts the fifty-move count
    halfmove = 0 if is_pawn or board[target] != EMPTY else halfmove + 1
    # En passant: the captured pawn stands beside the pawn that
    # moves, not on the square it lands on
    if is_pawn and target == en_passant:
        board[start // 8 * 8 + target % 8] = EMPTY
    # Castling: the king moves two squares, and the rook jumps to the
    # square the king crossed
    if piece & 7 == KING and distance == 2:
        rook = start + 3 if target > start else start - 4
        board[(start + target) // 2] = board[rook]
        board[rook] = EMPTY
    # A two-square pawn move lets an enemy pawn capture it en passant
    # on the next move only
    en_passant = (start + target) // 2 if is_pawn and distance == 16 else -1
    if promotion:
        piece = promotion + (piece & WHITE)
    board[target] = piece
    board[start] = EMPTY
    # A move from or to e1, h1, a1, e8, h8, or a8 turns off the rights of that
    # square. Turning off a right that is already off changes nothing.
    for square in (start, target):
        castling &= ~RIGHTS_OF_SQUARE.get(square, 0)
    return board, not white, castling, en_passant, halfmove
