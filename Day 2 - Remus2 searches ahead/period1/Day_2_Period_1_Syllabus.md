# Day 2, Period 1: The evaluation

8:30 to 10:00. Part of [Day 2: Remus2 searches ahead](../Day_2_Syllabus.md).

## Goal

Remus2 can score a position in centipawns.

## The big picture

Today the student pair turns Remus1 into Remus2, and all of Day 2's typing goes into the folder `engines/Remus2`. The period opens in File Explorer: copy every file of `engines/Remus1` into `engines/Remus2`, and say yes when Windows asks to replace `remus.bat` and `verify_remus.py`. From then on, edit only the files in `engines/Remus2`. `engines/Remus1` stays as it is.

Remus2 needs a way to tell a good position from a bad one before it can look ahead, so this period adds one new file, `evaluation.py`. It scores a position in centipawns, hundredths of a pawn: a pawn is worth 100, a knight or bishop 300, a rook 500, and a queen 900. On top of the material, each piece gets a bonus or a penalty for the square it stands on, read from a piece-square table: a White knight on e4 gets 20, the same knight on its home square g1 gets -40, and on the corner square a1 it gets -50. The score is from the side to move's point of view, so a position good for White scores above 0 when White is to move and below 0 when Black is to move.

No copied file changes this period. The copied files of Remus1 keep working, and `evaluation.py` waits until Period 2 for `search.py` to use it.

## The period step by step

| Time | Activity |
| --- | --- |
| 8:30 to 8:35 | In File Explorer, copy every file of `engines/Remus1` into `engines/Remus2`. The rest of Day 2 edits those copies. |
| 8:35 to 8:45 | A score in centipawns: material, plus a bonus or penalty for where each piece stands. A Black piece reads each table upside down. |
| 8:45 to 9:55 | Type `evaluation.py` (68 code lines): `mirrored_table()`, the four piece-square tables (files a to d only), the piece values, and `evaluate()`. Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |
| 9:55 to 10:00 | Swap driver and navigator. |

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| `board.py`, `make_move.py`, `attacks.py`, `move_generation.py`, `search.py`, `uci.py`, `remus.bat` | Copied from `engines/Remus1` | Copy into `engines/Remus2`; no edits this period | Unchanged |
| `evaluation.py` | New | Type the whole file | 1 to 80 |

The instructor's version is [`engines_instructor/Remus2/evaluation.py`](../../engines_instructor/Remus2/evaluation.py). Copy only the code: no comments, no docstrings, and no type hints. The line numbers on this page count the lines of the student pair's own file in `engines/Remus2`.

## The file, line by line

### `evaluation.py`

![evaluation.py, lines 1 to 80](images/evaluation_lines_1_to_80.png)

Create `evaluation.py` in `engines/Remus2`. It has 80 lines when it is done.

`def mirrored_table(left_half):` (line 4) saves typing. A piece-square table needs 64 numbers, but the tables are the same on the left and right halves of the board, so each one is typed as 32 numbers, files a to d of each rank. The loop on lines 6 to 8 builds all 64 by reading each row's four numbers forward and then backward: file e copies file d, f copies c, g copies b, and h copies a.

The four tables follow: `PAWN_TABLE = mirrored_table((` (line 12), `KNIGHT_TABLE = mirrored_table((` (line 23), `BISHOP_TABLE = mirrored_table((` (line 34), and `KING_TABLE = mirrored_table((` (line 45). Each holds eight rows of four numbers, the first row for rank 8 and the last for rank 1, as White sees the board. Type the `# fmt: skip` at the end of lines 21, 32, 43, and 54, so VS Code keeps the numbers in rows when it saves.

`PIECE_VALUES = (0, 100, 300, 300, 500, 900, 0)` (line 56) gives each piece number its value, from 0 for an empty square to 0 for the king, which is never captured. Rooks and queens get no table in Remus2, so `ZERO_TABLE` (line 58), 64 zeros, stands in for them, and `TABLES` (lines 60 to 63) puts the tables in piece-number order, so `TABLES[KNIGHT]` is the knight table.

`def evaluate(position):` (line 66) adds up every piece on the board. A White piece reads its table at its own square (line 75). A Black piece reads the table upside down, because rank 8 is Black's home rank: line 78 turns the row number `square // 8` into `7 - square // 8`. `return score if white else -score` (line 80) turns the score to the side to move's point of view.

## Done when

Press Run Python File in `evaluation.py`. The file loads without an error and prints nothing. This checks syntax and imports, not whether `evaluate()` returns correct scores. The finished Remus2 is tested with `verify_remus.py` in Period 3.
