# Day 4: Remus4

**Goal for the day:** the student pair types the 159 new lines of Remus4 and runs the verifier at the end of each period.

## The day at a glance

| Period | Time | Title |
| --- | --- | --- |
| P1 | 8:30 to 10:00 | [Draw sense and searching less](period1/Day_4_Period_1_Syllabus.md) |
| | 10:00 to 10:30 | Break |
| P2 | 10:30 to 12:00 | [Static exchange evaluation](period2/Day_4_Period_2_Syllabus.md) |
| | 12:00 to 1:00 | Lunch |
| P3 | 1:00 to 2:30 | [The endgame evaluation](period3/Day_4_Period_3_Syllabus.md) |
| | 2:30 to 3:00 | Break |
| P4 | 3:00 to 4:30 | [The pawn terms, then test Remus4](period4/Day_4_Period_4_Syllabus.md) |

## Period 1, 8:30 to 10:00: Draw sense and searching less

| Time | Activity |
| --- | --- |
| 8:30 to 8:35 | In File Explorer, copy every file of `engines/Remus3` into `engines/Remus4`. |
| 8:35 to 8:45 | Reminder of threefold repetition and the fifty-move rule. Late move reductions, principal variation search, and futility pruning: three ways to spend less time on moves that are probably bad. |
| 8:45 to 9:55 | The halfmove clock in `board.py` and `make_move.py`, the game history in `uci.py`, then `is_draw()`, the history list, futility pruning, late move reductions, and PVS in `search.py` (about 36 code lines). Press Run Python File in `engines/Remus4/verify_remus.py` until the terminal prints `Remus4 after Period 1 passes every test`. |
| 9:55 to 10:00 | Swap driver and navigator. |

[`Day_4_Period_1_Syllabus.md`](period1/Day_4_Period_1_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 2, 10:30 to 12:00: Static exchange evaluation

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | A capture is good only if the whole exchange on its square wins material: play it out on paper, cheapest attacker first. |
| 10:45 to 11:50 | Type `exchange.py` (36 code lines): `cheapest_attacker()` and `exchange_score()`. In `quiescence()`, add delta pruning and skip the captures that lose material (about 8 code lines). |
| 11:50 to 12:00 | Press Run Python File in `engines/Remus4/verify_remus.py` until the terminal prints `Remus4 after Period 2 passes every test`. |

[`Day_4_Period_2_Syllabus.md`](period2/Day_4_Period_2_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 3, 1:00 to 2:30: The endgame evaluation

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | Reminder of insufficient material. In the endgame the king comes to the center, and the side that is ahead drives the lone king to the edge. |
| 1:15 to 2:25 | In `evaluation.py`, add the endgame king table and the taper that blends the two king tables, mobility, `mating_bonus()`, and insufficient material (about 45 code lines). Press Run Python File in `engines/Remus4/verify_remus.py` until the terminal prints `Remus4 after Period 3 passes every test`. |
| 2:25 to 2:30 | Swap driver and navigator. |

[`Day_4_Period_3_Syllabus.md`](period3/Day_4_Period_3_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## Period 4, 3:00 to 4:30: The pawn terms, then test Remus4

| Time | Activity |
| --- | --- |
| 3:00 to 3:30 | Reminder of the rule of the square. In `pawn_terms()`, add the pawn race, rooks on open files, and `king_safety()` (about 35 code lines). Change the name line to `id name Remus4`. |
| 3:30 to 4:00 | Press Run Python File in `engines/Remus4/verify_remus.py` until the terminal prints `Remus4 passes every test`. Then play one round of the instructor's Remus4 against the instructor's Remus3, two 10-minute games played at once in two Lucas workers. |
| 4:00 to 4:30 | One round of the instructor's Remus4 against Remus4-Rust, two 10-minute games played at once in two Lucas workers. Remus4-Rust is the instructor's Remus4 written in Rust, with the same evaluation and search and a larger table, so compare the reported search depths and game results. This round does not isolate speed from table size or prove a playing-strength difference. |
| If time remains | One round of the student pair against the instructor's Remus4, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |

Play stops at 4:30, the end of the period, even in the middle of a game.

[`Day_4_Period_4_Syllabus.md`](period4/Day_4_Period_4_Syllabus.md) has the period's goal, the big picture of what the student pair types or edits and why, every file that changes with its line numbers, and how the student pair knows it is done.

## By the end of the day

**Checkpoint:** Remus4 is complete, 694 code lines, and it passes every test in `verify_remus.py`.
