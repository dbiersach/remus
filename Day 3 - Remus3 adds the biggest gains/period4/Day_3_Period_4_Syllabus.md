# Day 3, Period 4: The principal variation, then test Remus3

3:00 to 4:30. Part of [Day 3: Remus3](../Day_3_Syllabus.md).

## Goal

The student pair adds the principal variation and tests its Remus3 with `verify_remus.py`, then observes the instructor's Remus3 playing the instructor's Remus2.

## The big picture

The student pair finishes Remus3 in `engines/Remus3` with a few lines in `search.py`, then tests it. No file is added.

The principal variation is the line of moves the engine expects both sides to play, such as `g8f6 g1f3 b8c6 b1c3` after 1. a3. The transposition table of Period 2 already holds it: the best move stored for the position on the board, then the best move stored for the position after that move, and so on. `principal_variation()` follows those stored moves, and `think()` prints them at the end of each `info` line after the word `pv`, which Lucas Chess shows beside the engine's score.

Lucas Chess uses the registered instructor engines in `engines_instructor/`; these games do not test the student pair's files in `engines/`. Record wins, draws, losses, or unfinished games. A short round illustrates play but does not establish a reliable strength rating or a controlled speed comparison.

## The period step by step

| Time | Activity |
| --- | --- |
| 3:00 to 3:20 | Add `principal_variation()` and the `pv` of the `info` line (about 13 code lines). |
| 3:20 to 4:30 | Press Run Python File in `engines/Remus3/verify_remus.py` until the terminal prints `Remus3 passes every test`. Then play one round of the instructor's Remus3 against the instructor's Remus2, two 10-minute games played at once in two Lucas workers. Once Remus3 leaves the book, compare the search depths both engines report. Remus3 also reports its predicted line beside its score; Remus2 does not report a principal variation. Different game positions make this an observation, not a controlled speed test. Each student pair compares an opening sequence with `engines_instructor/Remus3/opening_book.txt`. Matching a listed move alone does not prove it came from the book. |
| If time remains | One round of the student pair against the instructor's Remus3, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |
| If more time remains | One round of the instructor's Remus3 against Stockfish 1500, two 10-minute games played at once in two Lucas workers. A student pair that also played Stockfish 1500 on Day 2 compares the results. |

Play stops at 4:30, the end of the period, even in the middle of a game.

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| `search.py` | Edited (begun in Period 1) | Add `principal_variation()` and the `pv` of the `info` line | 150 to 158, 186 to 187 |
| All other files | Unchanged since Period 3 | No edits this period | Unchanged |

The instructor's version is [`engines_instructor/Remus3/search.py`](../../engines_instructor/Remus3/search.py). Copy only the code: no comments, no docstrings, and no type hints. The line numbers on this page count the lines of the student pair's own `search.py` in `engines/Remus3`, as it stands at the end of this period.

## The edits, file by file

### `search.py`

![search.py, lines 150 to 158](images/search_lines_150_to_158.png)

![search.py, lines 186 to 187](images/search_lines_186_to_187.png)

`search.py` grows from 180 lines to 192, the finished Remus3 file.

Type `def principal_variation(position, depth):` (lines 150 to 158) between `alpha_beta()` and `think()`, with two blank lines after it. It starts with an empty string and, at most `depth` times, probes the table for the current position, adds the stored move's UCI text, and plays the move with `make_move()`. It stops early at a position the table does not hold, or one with no stored move.

In `think()`, replace the `print` line, `print(f"info depth {depth} score cp {score} nodes {nodes}", flush=True)`, with two lines: `line = principal_variation(position, depth)` (line 186) and `print(f"info depth {depth} score cp {score} nodes {nodes} pv{line}", flush=True)` (line 187). The `f` before the string makes it an f-string, so each name in braces is replaced by its value; `line` starts with a space, so the text reads `pv g8f6 g1f3`.

Remus3 is complete. Every file in `engines/Remus3` now matches the instructor's Remus3, comments, docstrings, and type hints aside.

The terminal prints actual results beside `(expected ...)` references. A `PROBLEM:` message or a traceback means the run has not passed; fix the reported problem and rerun. Continue only after the final success line named in the schedule. These checks cover the listed reference cases, not every position or code branch.

## Done when

The terminal reports `Remus3 passes every test` for the student pair's code. The student pair has observed the instructor's Remus3 against the instructor's Remus2 in Lucas Chess and recorded the game results. Games against the student pair and Stockfish 1500 follow if time remains.
