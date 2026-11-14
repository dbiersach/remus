from board import EMPTY, KING, PAWN, WHITE

RIGHTS_OF_SQUARE = {60: 3, 63: 1, 56: 2, 4: 12, 7: 4, 0: 8}


def make_move(position, move):
    board, white, castling, en_passant = position
    start, target, promotion = move
    board = board[:]
    piece = board[start]
    is_pawn = piece & 7 == PAWN
    distance = abs(target - start)
    if is_pawn and target == en_passant:
        board[start // 8 * 8 + target % 8] = EMPTY
    if piece & 7 == KING and distance == 2:
        rook = start + 3 if target > start else start - 4
        board[(start + target) // 2] = board[rook]
        board[rook] = EMPTY
    en_passant = (start + target) // 2 if is_pawn and distance == 16 else -1
    if promotion:
        piece = promotion + (piece & WHITE)
    board[target] = piece
    board[start] = EMPTY
    for square in (start, target):
        castling &= ~RIGHTS_OF_SQUARE.get(square, 0)
    return board, not white, castling, en_passant
