import random

from attacks import in_check
from make_move import make_move
from move_generation import generate_moves


def think(position, seconds):
    legal = []
    for move in generate_moves(position):
        child = make_move(position, move)
        if not in_check(child[0], position[1]):
            legal.append(move)
    return random.choice(legal) if legal else None
