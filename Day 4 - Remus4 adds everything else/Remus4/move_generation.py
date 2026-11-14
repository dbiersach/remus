from attacks import PIECE_STEPS, is_attacked
from board import BISHOP, EMPTY, KNIGHT, PAWN, QUEEN, ROOK, is_own


def add_pawn_move(moves, start, target):
    promotions = (QUEEN, ROOK, BISHOP, KNIGHT) if target < 8 or target >= 56 else (0,)
    for promotion in promotions:
        moves.append((start, target, promotion))


def pawn_moves(board, white, en_passant, start, moves):
    forward = -8 if white else 8
    ahead = start + forward
    if board[ahead] == EMPTY:
        add_pawn_move(moves, start, ahead)
        first_row = 6 if white else 1
        if start // 8 == first_row and board[ahead + forward] == EMPTY:
            moves.append((start, ahead + forward, 0))
    for side in (-1, 1):
        if not 0 <= start % 8 + side < 8:
            continue
        target = ahead + side
        if is_own(board[target], not white) or target == en_passant:
            add_pawn_move(moves, start, target)


def piece_moves(board, white, start, steps, slides, moves):
    for row_change, col_change in steps:
        r = start // 8 + row_change
        c = start % 8 + col_change
        while slides and 0 <= r < 8 and 0 <= c < 8 and board[r * 8 + c] == EMPTY:
            moves.append((start, r * 8 + c, 0))
            r += row_change
            c += col_change
        if 0 <= r < 8 and 0 <= c < 8 and not is_own(board[r * 8 + c], white):
            moves.append((start, r * 8 + c, 0))


def castling_moves(board, white, castling, moves):
    king = 60 if white else 4
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


def generate_moves(position):
    board, white, castling, en_passant = position[0:4]
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
