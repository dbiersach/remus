"""search.py

Choose a move for Remus1: a legal move picked at random. Remus1 looks no
moves ahead, so it plays legal chess, but badly.
"""

import random

from attacks import in_check
from board import Move, Position
from make_move import make_move
from move_generation import generate_moves


def think(position: Position, seconds: float) -> Move | None:
    """
    Return a random legal move, or None when there is no legal move.

    seconds is the time the GUI allows for the move. Remus1 answers at
    once, so it never needs it.
    """
    legal = []
    for move in generate_moves(position):
        child = make_move(position, move)
        # A move is legal only if the mover's king is not attacked after it
        if not in_check(child[0], position[1]):
            legal.append(move)
    # With no legal move (checkmate or stalemate), return None
    return random.choice(legal) if legal else None
