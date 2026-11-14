# Day 3: Remus3

**Goal for the day:** the student pair turns its Remus2 into Remus3 by typing the 184 new lines, then tests it in P4. The planned period states contain the functions they need, but successful startup does not establish that the student pair's implementation plays correctly.

## The day at a glance

| Period | Time | Title |
| --- | --- | --- |
| P1 | 8:30 to 10:00 | [Pawn structure, the bishop pair, and the rook table](Day_3_Period_1_Syllabus.md) |
| | 10:00 to 10:30 | Break |
| P2 | 10:30 to 12:00 | [The table, killer moves, and the null move](Day_3_Period_2_Syllabus.md) |
| | 12:00 to 1:00 | Lunch |
| P3 | 1:00 to 2:30 | [The opening book and the clock](Day_3_Period_3_Syllabus.md) |
| | 2:30 to 3:00 | Break |
| P4 | 3:00 to 4:30 | [The principal variation, then test Remus3](Day_3_Period_4_Syllabus.md) |

## Period 1, 8:30 to 10:00: Pawn structure, the bishop pair, and the rook table

| Time | Activity |
| --- | --- |
| 8:30 to 8:35 | In File Explorer, copy only the engine `.py` modules from `Day 2 - Remus2 searches ahead/Remus2` into `Day 3 - Remus3 adds the biggest gains/Remus3`, excluding the verifier. Keep the destination guide, verifier, launcher, and supplied data. |
| 8:35 to 8:45 | Doubled, isolated, and passed pawns, on a position the teacher shows in Lucas Chess. Why two bishops together are worth more than a bishop and a knight. |
| 8:45 to 9:00 | Move `MATE_SCORE` from `search.py` to `board.py`, and add `position_key()` and `move_text()` (8 code lines in `board.py`). |
| 9:00 to 9:55 | In `evaluation.py`, load the supplied rook table, add `pawn_terms()`, the bishop pair, and the lines of `evaluate()` that collect the pawns (38 code lines). Press Run Python File in `uci.py` to check for syntax and import errors. No text is expected while it waits for UCI commands; starting it does not test the engine's chess functions. Close that run's terminal before continuing. |
| 9:55 to 10:00 | Swap driver and navigator. |

[`Day_3_Period_1_Syllabus.md`](Day_3_Period_1_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 2, 10:30 to 12:00: The table, killer moves, and the null move

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | Why move order matters to alpha-beta, and why the same position appears by many move orders: e4 e5 Nf3 and Nf3 e5 e4 reach one position. The null move: let the opponent move twice; if the side to move still stands above beta, cut off. |
| 10:45 to 11:00 | Type `transposition_table.py` (22 code lines): `store()` and `probe()`. |
| 11:00 to 11:55 | In `search.py`, add the killer moves and the table probe and store, then `has_pieces()` and null-move pruning in `alpha_beta()` (about 52 code lines). Press Run Python File in `uci.py` to check for syntax and import errors. No text is expected while it waits for UCI commands; starting it does not test the engine's chess functions. Close that run's terminal before continuing. |
| 11:55 to 12:00 | Swap driver and navigator. |

[`Day_3_Period_2_Syllabus.md`](Day_3_Period_2_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 3, 1:00 to 2:30: The opening book and the clock

| Time | Activity |
| --- | --- |
| 1:00 to 1:10 | An opening book is a list of good first moves in UCI text. Copy `opening_book.txt` from `Day 3 - Remus3 adds the biggest gains/Remus3` into `Day 3 - Remus3 adds the biggest gains/Remus3` in File Explorer: it is data, not code, so nobody types it. |
| 1:10 to 1:35 | Type `opening_book.py` (21 code lines): `load_book()` and `book_moves()`. |
| 1:35 to 2:25 | In `think()`, add the aspiration window and the time that grows when the best move changes; in `uci.py`, add `legal_book_move()`, the game's moves, and the hard time limit (about 30 code lines). |
| 2:25 to 2:30 | Change the name line in `uci.py` to `id name Remus3`. |

[`Day_3_Period_3_Syllabus.md`](Day_3_Period_3_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 4, 3:00 to 4:30: The principal variation, then test Remus3

| Time | Activity |
| --- | --- |
| 3:00 to 3:20 | Add `principal_variation()` and the `pv` of the `info` line (about 13 code lines). |
| 3:20 to 4:30 | Press Run Python File in `Day 3 - Remus3 adds the biggest gains/Remus3/verify_remus3.py` until the terminal prints `Remus3 passes every test`. Then play one round of the student pair's Remus3 against the student pair's Remus2, two 10-minute games played at once in two Lucas workers. Once Remus3 leaves the book, compare the search depths both engines report. Remus3 also reports its predicted line beside its score; Remus2 does not report a principal variation. Different game positions make this an observation, not a controlled speed test. Each student pair compares an opening sequence with `Day 3 - Remus3 adds the biggest gains/Remus3/opening_book.txt`. Matching a listed move alone does not prove it came from the book. |
| If time remains | One round of the student pair against the student pair's Remus3, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |
| If more time remains | One round of the student pair's Remus3 against Stockfish 1500, two 10-minute games played at once in two Lucas workers. A student pair that also played Stockfish 1500 on Day 2 compares the results. |

Play stops at 4:30, the end of the period, even in the middle of a game.

[`Day_3_Period_4_Syllabus.md`](Day_3_Period_4_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## By the end of the day

**Checkpoint:** Remus3 is complete, 484 code lines, and it passes every test in `verify_remus3.py`.
