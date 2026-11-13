# Day 4, Period 3: The endgame evaluation

1:00 to 2:30. Part of [Day 4: Remus4](../Day_4_Syllabus.md).

## Goal

Remus4 plays endgames: the king comes forward, and the side ahead drives the lone king to the edge.

## The big picture

The student pair keeps working in `engines/Remus4`. This period edits only `evaluation.py`, and adds no file.

In the middlegame a king hides behind its pawns, but in the endgame, with few pieces left to attack it, the king is a strong piece that belongs in the center. Remus4 keeps two king tables, the middlegame table of Remus2 and a new endgame table, and blends them by the game phase: a number from 24 with every knight, bishop, rook, and queen on the board, down to 0 with none. With half the pieces gone, the king reads each table half and half. The same pass over the board counts each piece's mobility, the squares it can move to, worth 2 centipawns each.

Two endgame rules follow. A side that is ahead by 400 centipawns or more in a late endgame earns a bonus for driving the losing king toward the edge and bringing its own king close, the way every mate against a lone king is won. And a position with no pawns and at most one minor piece, such as king and bishop against king, scores 0, because neither side can mate: this is insufficient material.

## The period step by step

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | Reminder of insufficient material. In the endgame the king comes to the center, and the side that is ahead drives the lone king to the edge. |
| 1:15 to 2:25 | In `evaluation.py`, add the endgame king table and the taper that blends the two king tables, mobility, `mating_bonus()`, and insufficient material (about 45 code lines). Press Run Python File in `engines/Remus4/verify_remus.py` until the terminal prints `Remus4 after Period 3 passes every test`. |
| 2:25 to 2:30 | Swap driver and navigator. |

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| `evaluation.py` | Copied from `engines/Remus3`, edited | Add the endgame king table and the taper, mobility, `mating_bonus()`, and insufficient material | 1 to 3, 47, 69 to 78, 86, 89 to 97, 129 to 138, 144 to 146, 153 to 176, 181 to 182 |
| All other files | Unchanged since Period 2 | No edits this period | Unchanged |

The instructor's version is [`engines_instructor/Remus4/evaluation.py`](../../engines_instructor/Remus4/evaluation.py). Copy only the code: no comments, no docstrings, and no type hints. The line numbers on this page count the lines of the student pair's own `evaluation.py` in `engines/Remus4`, as it stands at the end of this period. Work from the top of the file down, and each number is right when you reach it.

## The edits, file by file

### `evaluation.py`

![evaluation.py, lines 1 to 3](images/evaluation_lines_1_to_3.png)

![evaluation.py, line 47](images/evaluation_line_47.png)

![evaluation.py, lines 69 to 78](images/evaluation_lines_69_to_78.png)

![evaluation.py, line 86](images/evaluation_line_86.png)

![evaluation.py, lines 89 to 97](images/evaluation_lines_89_to_97.png)

![evaluation.py, lines 129 to 138](images/evaluation_lines_129_to_138.png)

![evaluation.py, lines 144 to 146](images/evaluation_lines_144_to_146.png)

![evaluation.py, lines 153 to 176](images/evaluation_lines_153_to_176.png)

![evaluation.py, lines 181 to 182](images/evaluation_lines_181_to_182.png)

`evaluation.py` grows from 131 lines to 183. The edits, from the top down:

1. Replace the one import line with three (lines 1 to 3): `from attacks import DIAGONAL, DIRECTIONS, KNIGHT_JUMPS, STRAIGHT`, `from board import BISHOP, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE`, and `from move_generation import piece_moves`. Mobility borrows Day 1's `piece_moves()` to list each piece's moves.
2. Rename the king table: `KING_MIDDLE_TABLE = mirrored_table((` (line 47). Its numbers stay.
3. Below `ROOK_TABLE`, type `KING_END_TABLE = mirrored_table((` (lines 69 to 78), with `# fmt: skip` at the end of line 78. Its best squares are the four in the center, worth 40.
4. In `TABLES`, put `ZERO_TABLE` in the king's place, so line 86 reads `ROOK_TABLE, ZERO_TABLE, ZERO_TABLE,`. The king's score now comes from the blend of its two tables, added after the loop.
5. Below `TABLES`, type `MOBILITY_STEPS = {` (lines 89 to 92), a dictionary from each piece kind to its steps and whether it slides, with `# fmt: skip` at the end of line 92. After a blank line, type `PHASE_VALUES = (0, 0, 1, 1, 2, 4, 0)` (line 94): a knight or bishop counts 1 toward the phase, a rook 2, and a queen 4, so the full set of pieces counts 24.
6. Under `PASSED_BONUS`, type `MOBILITY_BONUS = 2` and `BISHOP_PAIR = 30` (lines 96 and 97), and delete the `BISHOP_PAIR = 30` line further down, under `ISOLATED_PAWN = 15`. The constant moves up so it sits beside the other evaluation terms; its value stays.
7. Between `pawn_terms()` and `evaluate()`, type `def mating_bonus(material, kings, phase):` (lines 129 to 138), with two blank lines after it. `edge` counts how far the losing king stands from the center in rows and columns, and `distance` counts the king steps between the two kings.
8. In `evaluate()`, type the three lines from `material = 0` to `kings = [0, 0]` (lines 144 to 146) under `score = 0`.
9. In the loop, under `kind = piece & 7`, replace the four lines from `if kind == PAWN:` to `score += PIECE_VALUES[kind] + TABLES[kind][square]` with the nine lines from `side = 0 if piece & WHITE else 1` to `pawns[side].append(square)` (lines 153 to 161). `view` is the square as the piece's own side sees the board, so one line reads the table for both colors. Material goes into its own total, so `mating_bonus()` can use it.
10. Keep the `else:` line under them (line 162). It now belongs to `if kind == KING:`, so its branch runs for knights, bishops, rooks, and queens. Replace the three lines under it, from `score -= (` to `)`, with the five lines from `phase += PHASE_VALUES[kind]` to `score += sign * MOBILITY_BONUS * len(moves)` (lines 163 to 167).
11. Under the loop, type the nine lines from `if phase <= 1 and not pawns[0] and not pawns[1]:` to `score += sign * taper // 24` (lines 168 to 176): insufficient material, then the king tables blended by the phase. `//` divides and rounds down to a whole number.
12. Near the end, change `score += pawn_terms(pawns)` to `score += material + pawn_terms(pawns)` (line 181), and type `score += mating_bonus(material, kings, phase)` (line 182) under it.

`pawn_terms()` does not change this period; Period 4 gives it the king and the rooks.

The terminal prints actual results beside `(expected ...)` references. A `PROBLEM:` message or a traceback means the run has not passed; fix the reported problem and rerun. Continue only after the final success line named in the schedule. These checks cover the listed reference cases, not every position or code branch.

## Done when

`verify_remus.py` in `engines/Remus4` reports that Remus4 after Period 3 passes every test. The verifier selects reference results from the modules and functions present. Its fixed-depth test calls the native search directly, without using the opening book. Passing means the listed reference cases match; it does not establish that every line or branch was exercised.
