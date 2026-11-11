"""opening_book.py

Play a stored opening line while the game follows one. Each line of
opening_book.txt is one opening in UCI moves from the starting position.
A line that starts with "White:" or "Black:" shows how that side defends
against a trap, and Remus plays its moves only for that side.
"""

import random
from pathlib import Path

book: list[tuple[str, list[str]]] = []  # (side, moves) for each line


def load_book():
    """Read opening_book.txt beside this file. A missing book is no error."""
    file_path = Path(__file__).parent / "opening_book.txt"
    if not file_path.exists():
        return
    for line in file_path.read_text().splitlines():
        side = ""
        if line.startswith(("White:", "Black:")):
            side, line = line.split(":")
        if line.strip() and not line.startswith("#"):
            book.append((side, line.split()))


def book_moves(played: list[str], white: bool) -> list[str]:
    """Return the book's next moves after the moves played, in random order."""
    moves = []
    for side, line in book:
        defends = side == "" or (side == "White") == white
        if defends and len(line) > len(played) and line[: len(played)] == played:
            moves.append(line[len(played)])
    random.shuffle(moves)
    return moves
