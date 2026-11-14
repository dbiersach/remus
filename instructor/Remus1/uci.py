"""uci.py

Talk to a chess GUI such as Lucas Chess through the Universal Chess
Interface (UCI). The GUI sends one command per line, and Remus
answers each one. Moves are written from square to square, such as
e2e4, with a fifth letter for a promotion, such as e7e8q. The
promotion letter's place in ".pnbrqk" is the kind of piece.
"""

import sys

from board import START_POSITION, parse_square, square_name
from make_move import make_move
from search import think


def read_time(words: list[str], name: str) -> int:
    """Return the milliseconds after a word of "go", such as wtime, or 0."""
    if name in words:
        return int(words[words.index(name) + 1])
    return 0


def main():
    position = START_POSITION
    for line in sys.stdin:
        words = line.split()
        if not words:
            continue
        if words[0] == "uci":
            print("id name Remus1")
            print("uciok", flush=True)
        elif words[0] == "isready":
            print("readyok", flush=True)
        elif words[0] == "position":
            # "position startpos moves e2e4 e7e5": the starting position,
            # then the moves played since. words[3:] is every word from
            # the fourth on (words[0] is "position"), so just the moves
            position = START_POSITION
            for text in words[3:]:
                # A move such as "e7e8q" has a fifth letter only for a
                # promotion. Its place in ".pnbrqk" is the piece kind:
                # "q" is at 5, QUEEN. With no fifth letter, promotion is 0
                promotion = ".pnbrqk".index(text[4]) if len(text) > 4 else 0
                # text[0:2] is the first two letters, the from square
                # ("e7"), and text[2:4] the next two, the to square ("e8")
                move = (parse_square(text[0:2]), parse_square(text[2:4]), promotion)
                position = make_move(position, move)
        elif words[0] == "go":
            clock_left = read_time(words, "wtime" if position[1] else "btime")
            increment = read_time(words, "winc" if position[1] else "binc")
            # Spend a thirtieth of the clock plus half the increment,
            # or 2 seconds when there is no clock
            seconds = 2.0
            if clock_left > 0:
                seconds = (clock_left / 30 + increment / 2) / 1000
            if "movetime" in words:
                seconds = read_time(words, "movetime") / 1000
            move = think(position, seconds)
            # think() returns None when the side to move has no legal
            # move (checkmate or stalemate), and UCI then says "0000"
            text = "0000"
            if move is not None:
                # A move is a tuple of three slots: move[0] is the from
                # square, move[1] the to square, move[2] the promotion
                text = square_name(move[0]) + square_name(move[1])
                if move[2]:
                    # move[2] is 0 with no promotion, else a piece kind
                    # (2 knight to 5 queen), which picks its letter:
                    # ".pnbrqk"[5] is "q". The "." fills slot 0, EMPTY
                    text += ".pnbrqk"[move[2]]
            print("bestmove " + text, flush=True)
        elif words[0] == "quit":
            break


if __name__ == "__main__":
    main()
