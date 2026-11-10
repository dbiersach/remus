# Day 2, Period 4: Remus2 against Remus1 and the student pair

3:00 to 4:30. Part of [Day 2: Remus2 searches ahead](../Day_2_Syllabus.md).

## Goal

Remus2 plays Remus1 and the student pair, and the class sees what the search buys over Remus1.

## The big picture

The student pair types no code this period. Remus2 is finished in `engines/Remus2`, and it passed its tests in Period 3. The period puts it to work: their Remus2 plays their Remus1, so the class sees what looking ahead buys over a random choice, then each student plays it, and then, if time remains, it plays Stockfish 1500. Tomorrow morning the student pair copies every file of `engines/Remus2` into `engines/Remus3` and starts Remus3 from those copies, so a student pair that did not finish Remus2 finishes it first.

## The period step by step

| Time | Activity |
| --- | --- |
| 3:00 to 3:25 | Start one round of their Remus2 against their Remus1, two 10-minute games played at once in two Lucas workers (Part 4 of the guide), to see what the search buys over a random choice. Beside each engine's score, Lucas Chess shows the depth of its search: Remus2's climbs one move at a time, and Remus1, which picks a move at random, shows none. Each student pair reads its results tab. |
| 3:25 to 4:05 | One round of the student pair against their Remus2, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |
| 4:05 to 4:30, if time remains | One round of their Remus2 against Stockfish 1500, two 10-minute games played at once in two Lucas workers. |

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| None | | No edits this period | |

## Done when

The student pair's Remus2 passes every test in `verify_remus.py`, it has played one round against its Remus1, and each student has played it. A round against Stockfish 1500 follows if time remains.
