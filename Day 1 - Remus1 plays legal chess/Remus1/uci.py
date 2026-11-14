import sys

from board import START_POSITION, parse_square, square_name
from make_move import make_move
from search import think


def read_time(words, name):
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
            position = START_POSITION
            for text in words[3:]:
                promotion = ".pnbrqk".index(text[4]) if len(text) > 4 else 0
                move = (parse_square(text[0:2]), parse_square(text[2:4]), promotion)
                position = make_move(position, move)
        elif words[0] == "go":
            clock_left = read_time(words, "wtime" if position[1] else "btime")
            increment = read_time(words, "winc" if position[1] else "binc")
            seconds = 2.0
            if clock_left > 0:
                seconds = (clock_left / 30 + increment / 2) / 1000
            if "movetime" in words:
                seconds = read_time(words, "movetime") / 1000
            move = think(position, seconds)
            text = "0000"
            if move is not None:
                text = square_name(move[0]) + square_name(move[1])
                if move[2]:
                    text += ".pnbrqk"[move[2]]
            print("bestmove " + text, flush=True)
        elif words[0] == "quit":
            break


if __name__ == "__main__":
    main()
