"""move_generation.py

List the moves of the side to move. A move is a tuple
(from_square, to_square, promotion), such as (52, 36, 0) for e2e4
or (12, 4, 5) for e7e8q, where 5 is a queen. The list holds pseudo-legal moves: some
may leave the mover's own king in check, and the search skips those
after it makes them.
"""

from attacks import PIECE_STEPS, is_attacked
from board import BISHOP, EMPTY, KNIGHT, PAWN, QUEEN, ROOK, Move, Position, is_own


def add_pawn_move(moves: list[Move], start: int, target: int):
    """Add a pawn move, or its four promotions on the last rank."""
    promotions = (QUEEN, ROOK, BISHOP, KNIGHT) if target < 8 or target >= 56 else (0,)
    for promotion in promotions:
        moves.append((start, target, promotion))


def pawn_moves(board, white, en_passant, start, moves):
    """Add the pushes and captures of the pawn on start."""
    # White pawns move toward row 0, one row is 8 squares
    forward = -8 if white else 8
    ahead = start + forward
    if board[ahead] == EMPTY:
        add_pawn_move(moves, start, ahead)
        # From its first row a pawn may move two squares
        first_row = 6 if white else 1
        if start // 8 == first_row and board[ahead + forward] == EMPTY:
            moves.append((start, ahead + forward, 0))
    for side in (-1, 1):
        if not 0 <= start % 8 + side < 8:
            continue  # The edge of the board
        target = ahead + side
        if is_own(board[target], not white) or target == en_passant:
            add_pawn_move(moves, start, target)


def piece_moves(board, white, start, steps, slides, moves):
    """
    Add the moves of a knight, bishop, rook, queen, or king. Each step
    goes one square in steps; a piece that slides keeps going until a
    piece or the edge of the board stops it.
    """
    for row_change, col_change in steps:
        r = start // 8 + row_change
        c = start % 8 + col_change
        while slides and 0 <= r < 8 and 0 <= c < 8 and board[r * 8 + c] == EMPTY:
            moves.append((start, r * 8 + c, 0))
            r += row_change
            c += col_change
        # The last square: empty, or an enemy piece to capture
        if 0 <= r < 8 and 0 <= c < 8 and not is_own(board[r * 8 + c], white):
            moves.append((start, r * 8 + c, 0))


def castling_moves(board, white, castling, moves):
    """
    Add the castling moves.

    A right that remains means the king and rook have not moved. The
    squares between them must be empty, and the king may not start in
    check or cross an attacked square. The search checks the square
    where the king lands.
    """
    king = 60 if white else 4  # e1 or e8
    kingside = 1 if white else 4
    queenside = kingside * 2
    kingside_open = castling & kingside and board[king + 1] == board[king + 2] == EMPTY
    queenside_open = castling & queenside and board[king - 3 : king] == [EMPTY] * 3
    if not (kingside_open or queenside_open) or is_attacked(board, king, not white):
        return
    if kingside_open and not is_attacked(board, king + 1, not white):
        moves.append((king, king + 2, 0))
    if queenside_open and not is_attacked(board, king - 1, not white):
        moves.append((king, king - 2, 0))


def generate_moves(position: Position) -> list[Move]:
    """Return the pseudo-legal moves of the side to move."""
    board, white, castling, en_passant = position
    moves = []
    for start, piece in enumerate(board):
        if not is_own(piece, white):
            continue
        kind = piece & 7
        if kind == PAWN:
            pawn_moves(board, white, en_passant, start, moves)
        else:
            steps, slides = PIECE_STEPS[kind]
            piece_moves(board, white, start, steps, slides, moves)
    if castling:
        castling_moves(board, white, castling, moves)
    return moves
