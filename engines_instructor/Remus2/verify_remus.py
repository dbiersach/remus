"""verify_remus.py

Verify the Remus in this folder without playing a game. Run this file with the Run
Python File button once the student pair has typed every file of this version, and
on Day 4 at the end of every period.

The test runs in steps and stops at the first step that fails, naming the file
to look at:

1. Every file of this version is in the folder.
2. Move generation: the number of legal move sequences (perft) from six
   published positions matches the published counts.
3. Evaluation (Remus2 to Remus4): the score of five positions matches the
   instructor's engine.
4. Search (Remus2 to Remus4): a search four moves deep finds the same move, the
   same score, and the same number of positions as the instructor's engine.
5. UCI: uci.py answers the commands Lucas Chess sends.

The same file sits in engines/Remus1 to engines/Remus4. It reads the name of its
own folder to know which version it tests. Remus4 grows over the four periods of
Day 4, so in engines/Remus4 the file also reads which period the student pair has
reached and compares the engine with the instructor's Remus4 at the end of that
period.
"""

import importlib
import subprocess
import sys
import time
from pathlib import Path

FOLDER = Path(__file__).parent  # The engine folder this file sits in
VERSION = FOLDER.name  # "Remus1", "Remus2", "Remus3", or "Remus4"

# The files each version holds, beside remus.bat and this file
FILES = {
    "Remus1": [
        "board.py",
        "attacks.py",
        "make_move.py",
        "move_generation.py",
        "search.py",
        "uci.py",
    ],
}
FILES["Remus2"] = FILES["Remus1"] + ["evaluation.py"]
FILES["Remus3"] = FILES["Remus2"] + [
    "transposition_table.py",
    "opening_book.py",
    "opening_book.txt",
]
FILES["Remus4"] = FILES["Remus3"] + ["exchange.py"]
# Remus4 as it stands at the end of Day 4, Periods 1 to 3. exchange.py arrives in
# Period 2.
FILES["Remus4 after Period 1"] = FILES["Remus3"]
FILES["Remus4 after Period 2"] = FILES["Remus4"]
FILES["Remus4 after Period 3"] = FILES["Remus4"]

# (name, position in FEN, depth, published count). FEN writes a position as
# text: the board from rank 8 down to rank 1, the side to move, the castling
# rights, and the en passant square. These positions hold castling, en passant,
# promotions, pins, and checks, so each rule gets counted.
PERFT_POSITIONS = (
    ("start", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -", 1, 20),
    ("start", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -", 2, 400),
    ("start", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -", 3, 8_902),
    ("start", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -", 4, 197_281),
    (
        "kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
        3,
        97_862,
    ),
    ("position 3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -", 4, 43_238),
    (
        "position 4",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -",
        3,
        9_467,
    ),
    (
        "position 5",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -",
        3,
        62_379,
    ),
    (
        "position 6",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - -",
        3,
        89_890,
    ),
)

# Positions for the evaluation and search tests
TEST_POSITIONS = (
    ("start", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"),
    ("italian", "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq -"),
    (
        "kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
    ),
    ("rook ending", "8/8/4k3/8/2R5/4K3/4P3/8 w - -"),
    ("position 5", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -"),
)

# The instructor's scores of TEST_POSITIONS, in the same order
EXPECTED_SCORES = {
    "Remus2": [0, 0, 100, 580, 65],
    "Remus3": [0, 0, 100, 575, 185],
    "Remus4 after Period 1": [0, 0, 100, 575, 185],
    "Remus4 after Period 2": [0, 0, 100, 575, 185],
    "Remus4 after Period 3": [0, -6, 110, 632, 195],
    "Remus4": [0, -6, 100, 652, 160],
}

# The instructor's search of each of TEST_POSITIONS, four moves deep:
# (best move, score, positions searched)
SEARCH_DEPTH = 4
EXPECTED_SEARCHES = {
    "Remus2": [
        ("g1f3", 0, 17_360),
        ("b1c3", 0, 124_924),
        ("e2a6", 40, 83_574),
        ("e3f2", 590, 16_451),
        ("d7c8q", 460, 21_872),
    ],
    "Remus3": [
        ("g1f3", 0, 2_944),
        ("b1c3", 0, 14_139),
        ("e2a6", 70, 51_527),
        ("e3f2", 605, 5_196),
        ("d7c8q", 490, 14_257),
    ],
    "Remus4 after Period 1": [
        ("g1f3", 0, 2_233),
        ("b1c3", 0, 7_723),
        ("e2a6", 70, 22_130),
        ("e3f2", 605, 2_673),
        ("d7c8q", 490, 2_743),
    ],
    "Remus4 after Period 2": [
        ("g1f3", 0, 2_189),
        ("b1c3", 0, 5_635),
        ("e2a6", 70, 7_920),
        ("e3f2", 605, 2_673),
        ("d7c8q", 490, 2_151),
    ],
    "Remus4 after Period 3": [
        ("e2e4", 0, 3_516),
        ("b1c3", 0, 6_517),
        ("e2a6", 86, 8_022),
        ("e3d4", 695, 2_424),
        ("d7c8q", 519, 2_166),
    ],
    "Remus4": [
        ("g1f3", 0, 3_005),
        ("b1c3", 0, 6_619),
        ("e2a6", 71, 6_486),
        ("e3d4", 715, 2_424),
        ("d7c8q", 515, 3_585),
    ],
}


def fail(message: str):
    """Print what went wrong and stop the test."""
    print()
    print("PROBLEM: " + message)
    sys.exit(1)


def find_stage() -> str:
    """
    Return the name of the engine this folder holds: its version, or in
    engines/Remus4 the period of Day 4 the student pair has reached.

    Each period of Day 4 adds something the period before lacks: exchange.py in
    Period 2, mating_bonus() in Period 3, and king_safety() in Period 4.
    """
    if VERSION != "Remus4":
        return VERSION
    if not (FOLDER / "exchange.py").exists():
        return "Remus4 after Period 1"
    evaluation_text = (FOLDER / "evaluation.py").read_text(encoding="utf-8")
    if "def mating_bonus(" not in evaluation_text:
        return "Remus4 after Period 2"
    if "def king_safety(" not in evaluation_text:
        return "Remus4 after Period 3"
    return "Remus4"


def from_fen(fen: str, board_module):
    """Turn a FEN text into a Remus position."""
    parts = fen.split()
    squares = []
    # The board runs from a8 to h1, the same order as Remus's squares 0 to 63.
    # A digit counts empty squares, and "/" ends a rank.
    for letter in parts[0]:
        if letter.isdigit():
            for _ in range(int(letter)):
                squares.append(0)
        elif letter != "/":
            kind = ".pnbrqk".index(letter.lower())
            if letter.isupper():
                kind += board_module.WHITE
            squares.append(kind)
    castling = 0
    for letter, bit in (("K", 1), ("Q", 2), ("k", 4), ("q", 8)):
        if letter in parts[2]:
            castling += bit
    en_passant = -1
    if parts[3] != "-":
        en_passant = board_module.parse_square(parts[3])
    position = (squares, parts[1] == "w", castling, en_passant)
    # A Remus4 position also holds the halfmove clock
    if len(board_module.START_POSITION) == 5:
        position = position + (0,)
    return position


def move_name(move, board_module) -> str:
    """Write a move the way UCI does, such as "e2e4" or "e7e8q"."""
    text = board_module.square_name(move[0]) + board_module.square_name(move[1])
    if move[2]:
        text += ".pnbrqk"[move[2]]
    return text


def perft(position, depth: int, engine) -> int:
    """Count the legal move sequences of the given depth from a position."""
    if depth == 0:
        return 1
    count = 0
    for move in engine["move_generation"].generate_moves(position):
        child = engine["make_move"].make_move(position, move)
        if not engine["attacks"].in_check(child[0], position[1]):
            count += perft(child, depth - 1, engine)  # Recursive call to itself
    return count


class StoppedClock:
    """
    A clock for the search test. It reads 0 until stop is True, then reads a
    time far in the future, so think() ends after the depth the test asks for.
    """

    stop = False

    def perf_counter(self) -> float:
        if self.stop:
            return 1e9
        return 0.0


def search_to_depth(position, engine) -> tuple[str, int, int]:
    """
    Search a position SEARCH_DEPTH moves deep and return the best move, the
    score, and the number of positions searched.

    think() prints one "info depth" line per depth and stops when its clock runs
    out. This test hands think() a clock that runs out right after the "info
    depth 4" line, and reads that line instead of printing it.
    """
    search = engine["search"]
    clock = StoppedClock()
    last_line = []

    def read_info(text, flush=False):
        last_line.append(text)
        if text.startswith(f"info depth {SEARCH_DEPTH} "):
            clock.stop = True

    search.time = clock
    search.print = read_info
    if VERSION == "Remus2":
        move = search.think(position, 1e6)
    elif VERSION == "Remus3":
        move = search.think(position, 1e6, 1e6)
    else:
        move = search.think(position, [], 1e6, 1e6)
    words = last_line[-1].split()
    score = int(words[words.index("cp") + 1])
    nodes = int(words[words.index("nodes") + 1])
    return move_name(move, engine["board"]), score, nodes


def verify_files(stage: str):
    """Step 1: every file of this version is in the folder."""
    if stage not in FILES:
        fail(f"this file must sit in engines/Remus1 to Remus4, not in {VERSION}")
    missing = []
    for name in FILES[stage]:
        if not (FOLDER / name).exists():
            missing.append(name)
    if missing:
        fail(f"{VERSION} has no {missing} in {FOLDER}")
    print(f"1. Files: all {len(FILES[stage])} files of {stage} are here")


def load_engine(stage: str) -> dict:
    """Import every module of the engine in this folder, by name."""
    sys.path.insert(0, str(FOLDER))
    engine = {}
    for name in FILES[stage]:
        if name.endswith(".py"):
            module_name = name[:-3]
            # A typo that stops a file from loading shows up here, with the
            # file name and line number of the problem
            engine[module_name] = importlib.import_module(module_name)
    return engine


def verify_perft(engine):
    """Step 2: count the legal move sequences and compare with the published."""
    print("2. Move generation (perft):")
    for name, fen, depth, expected in PERFT_POSITIONS:
        started = time.perf_counter()
        position = from_fen(fen, engine["board"])
        count = perft(position, depth, engine)
        seconds = time.perf_counter() - started
        print(
            f"   {name:<10} depth {depth}: {count:>7,} "
            f"(expected {expected:,}), {seconds:.1f} s"
        )
        if count != expected:
            fail(
                "the move counts differ. Look at move_generation.py, make_move.py, "
                "attacks.py, and board.py"
            )


def verify_evaluation(engine, stage: str):
    """Step 3: score five positions and compare with the instructor's engine."""
    print("3. Evaluation:")
    for index in range(len(TEST_POSITIONS)):
        name, fen = TEST_POSITIONS[index]
        score = engine["evaluation"].evaluate(from_fen(fen, engine["board"]))
        expected = EXPECTED_SCORES[stage][index]
        print(f"   {name:<11} score {score:>5} (expected {expected})")
        if score != expected:
            fail("the scores differ. Look at evaluation.py and its tables")


def verify_search(engine, stage: str):
    """Step 4: search five positions and compare with the instructor's engine."""
    print(f"4. Search, {SEARCH_DEPTH} moves deep:")
    for index in range(len(TEST_POSITIONS)):
        name, fen = TEST_POSITIONS[index]
        found = search_to_depth(from_fen(fen, engine["board"]), engine)
        expected = EXPECTED_SEARCHES[stage][index]
        print(
            f"   {name:<11} {found[0]} score {found[1]} nodes {found[2]:,} "
            f"(expected {expected[0]} score {expected[1]} nodes {expected[2]:,})"
        )
        if found != expected:
            fail("the search differs. Look at search.py and the files it imports")


def verify_uci(stage: str):
    """Step 5: send uci.py the commands Lucas Chess sends."""
    # Remus4 takes its name in Day 4, Period 4, so until then uci.py says Remus3
    name = VERSION
    if stage != VERSION:
        name = "Remus3"
    commands = "uci\nisready\nposition startpos moves e2e4\ngo movetime 500\nquit\n"
    answer = subprocess.run(
        [sys.executable, "uci.py"],
        cwd=FOLDER,
        input=commands,
        capture_output=True,
        text=True,
        timeout=60,
        check=False,
    )
    lines = answer.stdout.split("\n")
    if f"id name {name}" not in lines:
        fail(f"uci.py does not answer 'id name {name}'. {answer.stderr}")
    if "uciok" not in lines or "readyok" not in lines:
        fail("uci.py does not answer uciok and readyok")
    last = answer.stdout.strip().split("\n")[-1]
    if not last.startswith("bestmove "):
        fail(f"uci.py ends with {last!r} instead of a bestmove")
    print(f"5. UCI: uci.py names itself {name} and answers e2e4 with {last}")


def main():
    started = time.perf_counter()
    stage = find_stage()
    print(f"Verifying {stage} in {FOLDER}")
    verify_files(stage)
    engine = load_engine(stage)
    verify_perft(engine)
    if VERSION != "Remus1":
        verify_evaluation(engine, stage)
        verify_search(engine, stage)
    verify_uci(stage)
    print()
    print(f"{stage} passes every test")
    print(f"Total run time (sec): {time.perf_counter() - started:.3f}")


if __name__ == "__main__":
    main()
