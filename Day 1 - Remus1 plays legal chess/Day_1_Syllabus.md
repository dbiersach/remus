# Day 1: Remus1, legal chess at random

**Goal for the day:** `engines/Remus1` holds an engine that generates legal moves and chooses one at random. The student pair tests its code with `verify_remus.py`, then each student plays the instructor's Remus1 in Lucas Chess.

## The day at a glance

| Period | Time | Title |
| --- | --- | --- |
| P1 | 8:30 to 10:00 | [Set up, and the board](period1/Day_1_Period_1_Syllabus.md) |
| | 10:00 to 10:30 | Break |
| P2 | 10:30 to 12:00 | [Moves and attacks](period2/Day_1_Period_2_Syllabus.md) |
| | 12:00 to 1:00 | Lunch |
| P3 | 1:00 to 2:30 | [Move generation](period3/Day_1_Period_3_Syllabus.md) |
| | 2:30 to 3:00 | Break |
| P4 | 3:00 to 4:30 | [The UCI loop, and Remus1 plays](period4/Day_1_Period_4_Syllabus.md) |

## Period 1, 8:30 to 10:00: Set up, and the board

| Time | Activity |
| --- | --- |
| 8:30 to 9:15 | Setup, steps 1 to 6 of the top `README.md`: install Git, VS Code, and `uv`, clone the repository, run `uv sync`, install the extensions, open `Remus.code-workspace`, then open Lucas Chess once to reach its main window, close it, and press Run Python File in `install_lucas_engines.py`. Expect registration messages in the terminal; reopen Lucas Chess and confirm Remus1 to Remus4 and Remus4-Rust appear under External engines. This registers the instructor engines; it does not test their play. This is the only time anyone types a git or `uv` command. |
| 9:15 to 9:30 | The teacher starts the instructor's Remus2 against Stockfish 1500 in Lucas Chess (Part 4 of the [Lucas Chess guide](../docs/lucas_chess/lucas_chess_guide.md)), so the class sees the engine it will build by tomorrow afternoon. |
| 9:30 to 9:40 | Each student pair opens its `engines/Remus1` folder in VS Code. The folder holds `remus.bat`, the one line that starts the engine, and `verify_remus.py`, which tests the finished engine; the student pair types every other file. |
| 9:40 to 10:00 | Type `board.py` (15 code lines): the piece numbers, the starting position, and square names. The reminder before typing: row 0 is rank 8, column 0 is file a, so a8 is square 0 and h1 is square 63. |

[`Day_1_Period_1_Syllabus.md`](period1/Day_1_Period_1_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 2, 10:30 to 12:00: Moves and attacks

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | The position tuple and copy-make: `make_move()` copies the board and returns a new position, so no move is ever taken back. Reminder of castling rights, en passant, and promotion. |
| 10:45 to 11:15 | Type `make_move.py` (23 code lines). Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |
| 11:15 to 12:00 | Type `attacks.py` (34 code lines): the knight jumps, the eight directions, `is_attacked()`, and `in_check()`. Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |

[`Day_1_Period_2_Syllabus.md`](period2/Day_1_Period_2_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 3, 1:00 to 2:30: Move generation

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | One function moves every piece but the pawn: a knight or king takes one step, a bishop, rook, or queen slides. Reminder of the castling conditions. |
| 1:15 to 2:20 | Type `move_generation.py` (65 code lines): `add_pawn_move()`, `pawn_moves()`, `piece_moves()`, `castling_moves()`, and `generate_moves()`. Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |
| 2:20 to 2:30 | Catch-up. |

[`Day_1_Period_3_Syllabus.md`](period3/Day_1_Period_3_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 4, 3:00 to 4:30: The UCI loop, and Remus1 plays

| Time | Activity |
| --- | --- |
| 3:00 to 3:10 | How Lucas Chess and Remus talk: read [`uci.md`](../docs/chess_tutor/uci.md) together. A move is legal only if the mover's own king is not attacked after it. |
| 3:10 to 3:50 | Type `search.py` of `engines_instructor/Remus1` (12 code lines), whose `think()` plays a random legal move, then `uci.py` (44 code lines). |
| 3:50 to 4:05 | Press Run Python File in `engines/Remus1/verify_remus.py` and fix any file it names until the terminal prints `Remus1 passes every test`. |
| 4:05 to 4:30 | One round of the student pair against the instructor's Remus1, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). Its moves are legal but chosen at random; watch how it misses threats. |

[`Day_1_Period_4_Syllabus.md`](period4/Day_1_Period_4_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## By the end of the day

**Checkpoint:** Remus1 is complete, 193 code lines, and it passes every test in `verify_remus.py`.
