EMPTY, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING = range(7)
WHITE = 8
MATE_SCORE = 100_000


BACK_RANK = [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK]
START_BOARD = BACK_RANK + [PAWN] * 8 + [EMPTY] * 32 + [WHITE + PAWN] * 8
for kind in BACK_RANK:
    START_BOARD.append(WHITE + kind)
START_POSITION = (START_BOARD, True, 15, -1)


def is_own(piece, white):
    return piece > WHITE if white else EMPTY < piece < WHITE


def position_key(position):
    return hash((tuple(position[0]), position[1], position[2], position[3]))


def parse_square(text):
    return "87654321".index(text[1]) * 8 + "abcdefgh".index(text[0])


def square_name(square):
    return "abcdefgh"[square % 8] + "87654321"[square // 8]


def move_text(move):
    text = square_name(move[0]) + square_name(move[1])
    if move[2]:
        text += ".pnbrqk"[move[2]]
    return text
