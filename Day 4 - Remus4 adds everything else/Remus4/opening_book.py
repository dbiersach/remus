import random
from pathlib import Path

book = []


def load_book():
    file_path = Path(__file__).parent / "opening_book.txt"
    if not file_path.exists():
        return
    for line in file_path.read_text().splitlines():
        side = ""
        if line.startswith(("White:", "Black:")):
            side, line = line.split(":")
        if line.strip() and not line.startswith("#"):
            book.append((side, line.split()))


def book_moves(played, white):
    moves = []
    for side, line in book:
        defends = side == "" or (side == "White") == white
        if defends and len(line) > len(played) and line[: len(played)] == played:
            moves.append(line[len(played)])
    random.shuffle(moves)
    return moves
