EMPTY, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING = range(7)
WHITE = 8


BACK_RANK = [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK]
START_BOARD = BACK_RANK + [PAWN] * 8 + [EMPTY] * 32 + [WHITE + PAWN] * 8
for kind in BACK_RANK:
    START_BOARD.append(WHITE + kind)
START_POSITION = (START_BOARD, True, 15, -1)


def is_own(piece, white):
    return piece > WHITE if white else EMPTY < piece < WHITE


def parse_square(text):
    return "87654321".index(text[1]) * 8 + "abcdefgh".index(text[0])


def square_name(square):
    return "abcdefgh"[square % 8] + "87654321"[square // 8]
