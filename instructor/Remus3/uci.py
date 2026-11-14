"""uci.py

Talk to a chess GUI such as Lucas Chess through the Universal Chess
Interface (UCI). The GUI sends one command per line, and Remus answers
each one. Moves are written from square to square, such as e2e4, with a
fifth letter for a promotion, such as e7e8q. The promotion letter's
place in ".pnbrqk" is the kind of piece.
"""

import sys

from attacks import in_check
from board import START_POSITION, move_text, parse_square
from make_move import make_move
from move_generation import generate_moves
from opening_book import book_moves, load_book
from search import think


def read_time(words: list[str], name: str) -> int:
    """Return the milliseconds after a word of "go", such as wtime, or 0."""
    if name in words:
        return int(words[words.index(name) + 1])
    return 0


def legal_book_move(position, played: list[str]):
    """Return a legal move from the opening book, or None."""
    for text in book_moves(played, position[1]):
        for move in generate_moves(position):
            child = make_move(position, move)
            if move_text(move) == text and not in_check(child[0], position[1]):
                return move
    return None


def main():
    load_book()
    position = START_POSITION
    played = []  # The moves of the game, as UCI text
    for line in sys.stdin:
        words = line.split()
        if not words:
            continue
        if words[0] == "uci":
            print("id name Remus3")
            print("uciok", flush=True)
        elif words[0] == "isready":
            print("readyok", flush=True)
        elif words[0] == "position":
            # "position startpos moves e2e4 e7e5": the starting position,
            # then the moves played since. words[3:] is every word from
            # the fourth on (words[0] is "position"), so just the moves
            position = START_POSITION
            played = words[3:]
            for text in played:
                # A move such as "e7e8q" has a fifth letter only for a
                # promotion. Its place in ".pnbrqk" is the piece kind:
                # "q" is at 5, QUEEN. With no fifth letter, promotion is 0
                promotion = ".pnbrqk".index(text[4]) if len(text) > 4 else 0
                # text[0:2] is the first two letters, the from square
                # ("e7"), and text[2:4] the next two, the to square ("e8")
                move = (parse_square(text[0:2]), parse_square(text[2:4]), promotion)
                position = make_move(position, move)
        elif words[0] == "go":
            white = position[1]
            clock_left = read_time(words, "wtime" if white else "btime")
            increment = read_time(words, "winc" if white else "binc")
            # Plan a thirtieth of the clock plus half the increment, and
            # never more than three times that or a fifth of the clock
            seconds = longest = 2.0
            if clock_left > 0:
                seconds = (clock_left / 30 + increment / 2) / 1000
                longest = min(3 * seconds, clock_left / 5000)
            if "movetime" in words:
                seconds = longest = read_time(words, "movetime") / 1000
            move = legal_book_move(position, played)
            if move is None:
                move = think(position, seconds, longest)
            # think() returns None when the side to move has no legal
            # move (checkmate or stalemate), and UCI then says "0000"
            text = "0000"
            if move is not None:
                text = move_text(move)
            print("bestmove " + text, flush=True)
        elif words[0] == "quit":
            break


if __name__ == "__main__":
    main()
