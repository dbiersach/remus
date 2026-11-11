"""exchange.py

Static exchange evaluation (SEE): play out every capture on one square,
cheapest attacker first, and return what the first capture wins or
loses. Quiescence skips a capture that loses material, such as a queen
taking a pawn that a pawn defends.
"""

from board import EMPTY, KING, PAWN, WHITE, Move
from evaluation import PIECE_VALUES
from move_generation import generate_moves

KING_VALUE = 2000  # The king captures last: losing it ends the game


def cheapest_attacker(board: list[int], square: int, by_white: bool) -> int:
    """Return the square of the cheapest piece that attacks the square, or -1."""
    best = -1
    best_value = KING_VALUE + 1
    # A piece attacks the square when one of its moves captures there.
    # The position (board, by_white, 0, -1, 0) gives the attacking side
    # the move, with no castling and no en passant.
    for start, target, _ in generate_moves((board, by_white, 0, -1, 0)):
        kind = board[start] & 7
        value = KING_VALUE if kind == KING else PIECE_VALUES[kind]
        if target == square and value < best_value:
            best = start
            best_value = value
    return best


def exchange_score(board: list[int], move: Move) -> int:
    """Return the material the capture wins when both sides keep recapturing."""
    start, target = move[0], move[1]
    board = board[:]
    # gains[i] is what the side making capture i has won after it
    gains = [PIECE_VALUES[board[target] & 7]]
    white = board[start] > WHITE
    if board[start] & 7 == PAWN and board[target] == EMPTY:
        # En passant removes the pawn behind the empty destination.
        gains[0] = PIECE_VALUES[PAWN]
        board[target + (8 if white else -8)] = EMPTY
    board[target] = board[start]
    board[start] = EMPTY
    attacker = cheapest_attacker(board, target, not white)
    while attacker != -1:
        # The king may not capture onto a defended square
        if board[target] & 7 == KING:
            if len(gains) == 1:
                return -KING_VALUE
            gains.pop()
            break
        gains.append(PIECE_VALUES[board[target] & 7] - gains[-1])
        board[target] = board[attacker]
        board[attacker] = EMPTY
        white = not white
        attacker = cheapest_attacker(board, target, not white)
    # Work back from the last capture: each side may stop capturing
    for i in range(len(gains) - 1, 0, -1):
        gains[i - 1] = -max(-gains[i - 1], gains[i])
    return gains[0]
