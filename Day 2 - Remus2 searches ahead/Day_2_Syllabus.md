# Day 2: Remus2 searches ahead

**Goal for the day:** the student pair builds and verifies its Remus2, then observes and plays its own engines in Lucas Chess.

## The day at a glance

| Period | Time | Title |
| --- | --- | --- |
| P1 | 8:30 to 10:00 | [The evaluation](Day_2_Period_1_Syllabus.md) |
| | 10:00 to 10:30 | Break |
| P2 | 10:30 to 12:00 | [Move ordering and quiescence](Day_2_Period_2_Syllabus.md) |
| | 12:00 to 1:00 | Lunch |
| P3 | 1:00 to 2:30 | [Alpha-beta and iterative deepening](Day_2_Period_3_Syllabus.md) |
| | 2:30 to 3:00 | Break |
| P4 | 3:00 to 4:30 | [Remus2 against Remus1 and the student pair](Day_2_Period_4_Syllabus.md) |

## Period 1, 8:30 to 10:00: The evaluation

| Time | Activity |
| --- | --- |
| 8:30 to 8:35 | In File Explorer, copy only the engine `.py` modules from `Day 1 - Remus1 plays legal chess/Remus1` into `Day 2 - Remus2 searches ahead/Remus2`, excluding the verifier. Keep the destination guide, verifier, launcher, and supplied data. The rest of Day 2 edits those copies. |
| 8:35 to 8:45 | A score in centipawns: material, plus a bonus or penalty for where each piece stands. A Black piece reads each table upside down. |
| 8:45 to 9:55 | Type `evaluation.py` (30 code lines): load the supplied JSON tables, set the piece values, and define `evaluate()`. The supplied `piece_square_tables.json` stays beside the module; students type none of its values. Press Run Python File to check syntax, imports, and loading the data file. No output is expected; this does not test evaluation scores. |
| 9:55 to 10:00 | Swap driver and navigator. |

[`Day_2_Period_1_Syllabus.md`](Day_2_Period_1_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 2, 10:30 to 12:00: Move ordering and quiescence

| Time | Activity |
| --- | --- |
| 10:30 to 10:50 | Alpha-beta on paper: look ahead, assume both sides play their best, and skip the moves that cannot change the result. MVV-LVA: try pawn takes queen before queen takes pawn. |
| 10:50 to 11:55 | In `Day 2 - Remus2 searches ahead/Remus2`, keep the three import lines of Remus1's `search.py`, change `import random` to `import time`, delete Remus1's random-move `think()`, and type the rest of the top of Remus2's `search.py` (about 50 code lines): the imports and constants, `ordered_moves()`, `count_node()`, and `quiescence()`. |
| 11:55 to 12:00 | Swap driver and navigator. |

[`Day_2_Period_2_Syllabus.md`](Day_2_Period_2_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 3, 1:00 to 2:30: Alpha-beta and iterative deepening

| Time | Activity |
| --- | --- |
| 1:00 to 1:10 | Reminder of checkmate and stalemate: a side with no legal move has lost if in check, and draws if not. |
| 1:10 to 2:10 | Type `alpha_beta()` and `think()` (about 43 code lines), which finish `search.py`. Change the name line in `uci.py` to `id name Remus2`. |
| 2:10 to 2:30 | Press Run Python File in `Day 2 - Remus2 searches ahead/Remus2/verify_remus2.py` and fix any file it names until the terminal prints `Remus2 passes every test`. |

[`Day_2_Period_3_Syllabus.md`](Day_2_Period_3_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 4, 3:00 to 4:30: Remus2 against Remus1 and the student pair

| Time | Activity |
| --- | --- |
| 3:00 to 3:25 | Start one round of the student pair's Remus2 against the student pair's Remus1, two 10-minute games played at once in two Lucas workers (Part 4 of the guide), to see what the search buys over a random choice. Beside each engine's score, Lucas Chess shows the depth of its search: Remus2 reports completed iterations in half-moves, and Remus1, which picks a move at random, shows none. Each student pair reads its results tab. |
| 3:25 to 4:05 | One round of the student pair against the student pair's Remus2, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |
| 4:05 to 4:30, if time remains | One round of the student pair's Remus2 against Stockfish 1500, two 10-minute games played at once in two Lucas workers. |

[`Day_2_Period_4_Syllabus.md`](Day_2_Period_4_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## By the end of the day

**Checkpoint:** Remus2 is complete, 300 code lines, and it passes every test in `verify_remus2.py`.
