import sys

from attacks import in_check
from board import START_POSITION, move_text, parse_square
from make_move import make_move
from move_generation import generate_moves
from opening_book import book_moves, load_book
from search import think


def read_time(words, name):
    if name in words:
        return int(words[words.index(name) + 1])
    return 0


def legal_book_move(position, played):
    for text in book_moves(played, position[1]):
        for move in generate_moves(position):
            child = make_move(position, move)
            if move_text(move) == text and not in_check(child[0], position[1]):
                return move
    return None


def main():
    load_book()
    position = START_POSITION
    played = []
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
            position = START_POSITION
            played = words[3:]
            for text in played:
                promotion = ".pnbrqk".index(text[4]) if len(text) > 4 else 0
                move = (parse_square(text[0:2]), parse_square(text[2:4]), promotion)
                position = make_move(position, move)
        elif words[0] == "go":
            white = position[1]
            clock_left = read_time(words, "wtime" if white else "btime")
            increment = read_time(words, "winc" if white else "binc")
            seconds = longest = 2.0
            if clock_left > 0:
                seconds = (clock_left / 30 + increment / 2) / 1000
                longest = min(3 * seconds, clock_left / 5000)
            if "movetime" in words:
                seconds = longest = read_time(words, "movetime") / 1000
            move = legal_book_move(position, played)
            if move is None:
                move = think(position, seconds, longest)
            text = "0000"
            if move is not None:
                text = move_text(move)
            print("bestmove " + text, flush=True)
        elif words[0] == "quit":
            break


if __name__ == "__main__":
    main()
