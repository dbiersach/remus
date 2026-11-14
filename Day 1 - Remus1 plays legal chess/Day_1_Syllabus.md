# Day 1: Remus1, legal chess at random

**Goal for the day:** `Day 1 - Remus1 plays legal chess/Remus1` holds an engine that generates legal moves and chooses one at random. The student pair tests its code with `verify_remus1.py`, then each student plays the student pair's Remus1 in Lucas Chess.

## The day at a glance

| Period | Time | Title |
| --- | --- | --- |
| P1 | 8:30 to 10:00 | [Set up, and the board](Day_1_Period_1_Syllabus.md) |
| | 10:00 to 10:30 | Break |
| P2 | 10:30 to 12:00 | [Moves and attacks](Day_1_Period_2_Syllabus.md) |
| | 12:00 to 1:00 | Lunch |
| P3 | 1:00 to 2:30 | [Move generation](Day_1_Period_3_Syllabus.md) |
| | 2:30 to 3:00 | Break |
| P4 | 3:00 to 4:30 | [The UCI loop, and Remus1 plays](Day_1_Period_4_Syllabus.md) |

## Period 1, 8:30 to 10:00: Set up, and the board

| Time | Activity |
| --- | --- |
| 8:30 to 9:15 | Follow the setup steps in the top `README.md`: install the tools, clone the repository, run `setup_workshop.ps1 -StudentCopy`, and open `Remus.code-workspace`. Initialize and close Lucas Chess, then run `docs/install_lucas_engines.py` in VS Code. Expect registration messages and Remus1 through Remus8 in External engines. This records paths; it does not test play. Python engines become playable after their modules are typed and verified. |
| 9:15 to 9:30 | On the teacher's laptop, the teacher demonstrates a completed Remus2 against Stockfish 1500 in Lucas Chess (Part 4 of the [Lucas Chess guide](../docs/lucas_chess_guide.md)), so the class sees the engine it will build by tomorrow afternoon. |
| 9:30 to 9:40 | Each student pair opens its `Day 1 - Remus1 plays legal chess/Remus1` folder in VS Code. The folder holds `remus1.bat`, the one line that starts the engine, and `verify_remus1.py`, which tests the finished engine; the student pair types the engine modules from the screenshots. |
| 9:40 to 10:00 | Type `board.py` (15 code lines): the piece numbers, the starting position, and square names. The reminder before typing: row 0 is rank 8, column 0 is file a, so a8 is square 0 and h1 is square 63. |

[`Day_1_Period_1_Syllabus.md`](Day_1_Period_1_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 2, 10:30 to 12:00: Moves and attacks

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | The position tuple and copy-make: `make_move()` copies the board and returns a new position, so no move is ever taken back. Reminder of castling rights, en passant, and promotion. |
| 10:45 to 11:15 | Type `make_move.py` (23 code lines). Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |
| 11:15 to 12:00 | Type `attacks.py` (38 code lines): the knight jumps, shared piece directions, `is_attacked()`, and `in_check()`. Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |

[`Day_1_Period_2_Syllabus.md`](Day_1_Period_2_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 3, 1:00 to 2:30: Move generation

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | One function moves every piece but the pawn: a knight or king takes one step, a bishop, rook, or queen slides. Reminder of the castling conditions. |
| 1:15 to 2:20 | Type `move_generation.py` (57 code lines): `add_pawn_move()`, `pawn_moves()`, `piece_moves()`, `castling_moves()`, and `generate_moves()`. Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |
| 2:20 to 2:30 | Catch-up. |

[`Day_1_Period_3_Syllabus.md`](Day_1_Period_3_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 4, 3:00 to 4:30: The UCI loop, and Remus1 plays

| Time | Activity |
| --- | --- |
| 3:00 to 3:10 | How Lucas Chess and Remus talk: read [`uci.md`](../docs/uci.md) together. A move is legal only if the mover's own king is not attacked after it. |
| 3:10 to 3:50 | Type `search.py` of `Day 1 - Remus1 plays legal chess/Remus1` (12 code lines), whose `think()` plays a random legal move, then `uci.py` (44 code lines). |
| 3:50 to 4:05 | Press Run Python File in `Day 1 - Remus1 plays legal chess/Remus1/verify_remus1.py` and fix any file it names until the terminal prints `Remus1 passes every test`. |
| 4:05 to 4:30 | One round of the student pair against the student pair's Remus1, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). Its moves are legal but chosen at random; watch how it misses threats. |

[`Day_1_Period_4_Syllabus.md`](Day_1_Period_4_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## By the end of the day

**Checkpoint:** Remus1 is complete, 189 code lines, and it passes every test in `verify_remus1.py`.
