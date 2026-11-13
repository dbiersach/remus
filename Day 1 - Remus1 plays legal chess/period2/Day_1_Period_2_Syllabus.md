# Day 1, Period 2: Moves and attacks

10:30 to 12:00. Part of [Day 1: Remus1, legal chess at random](../Day_1_Syllabus.md).

## Goal

The student pair can make a move on a copy of the board and ask whether a square is attacked.

## The big picture

The student pair keeps building Remus1 from nothing in `engines/Remus1`, so both files of this period are new files there. They give the engine its two basic actions.

`make_move.py` plays one move. It never changes the board it is given: it copies the board, moves the piece on the copy, and returns a new position. This is copy-make, and it means the engine never has to take a move back. The file also handles the three moves that change more than two squares or more than the piece: castling moves a rook too, en passant removes a pawn from a square the capturing pawn never touches, and promotion turns a pawn into another piece.

`attacks.py` answers "is this square attacked?" by looking out from the square itself: if a knight's jump away stands an enemy knight, a knight attacks the square, and the same idea works for every piece. One question built on it, "is my king in check?", is what makes a move legal or not.

## The period step by step

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | The position tuple and copy-make: `make_move()` copies the board and returns a new position, so no move is ever taken back. Reminder of castling rights, en passant, and promotion. |
| 10:45 to 11:15 | Type `make_move.py` (23 code lines). Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |
| 11:15 to 12:00 | Type `attacks.py` (34 code lines): the knight jumps, the eight directions, `is_attacked()`, and `in_check()`. Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |

## What the student pair types this period

| File | New or edited | What it holds | Lines in your file |
| --- | --- | --- | --- |
| `make_move.py` | New | `make_move()`: play a move on a copy of the board | 1 to 26 |
| `attacks.py` | New | The knight jumps, the eight directions, `is_attacked()`, and `in_check()` | 1 to 40 |

The instructor's versions are [`engines_instructor/Remus1/make_move.py`](../../engines_instructor/Remus1/make_move.py) and [`engines_instructor/Remus1/attacks.py`](../../engines_instructor/Remus1/attacks.py). Copy only the code: no comments, no docstrings, and no type hints. The line numbers on this page count the lines of the student pair's own file.

## The files, line by line

### `make_move.py`

![make_move.py, lines 1 to 26](images/make_move_lines_1_to_26.png)

Create `make_move.py` in `engines/Remus1`. It has 26 lines when it is done.

`from board import EMPTY, KING, PAWN, WHITE` (line 1) borrows four names from `board.py`, much as an `import` in Java borrows a class. `RIGHTS_OF_SQUARE = {60: 3, 63: 1, 56: 2, 4: 12, 7: 4, 0: 8}` (line 3) is a dictionary, Python's version of a `HashMap`: it names, for each king and rook square, the castling rights lost when a piece moves from or to that square. A move from e1 (square 60) loses both of White's rights, 1 and 2, so it loses 3.

`def make_move(position, move):` (line 6) starts the function. Line 7 unpacks the four parts of the position into four names in one step, and line 8 does the same for the three parts of the move. `board = board[:]` (line 9) copies the list, so the caller's board stays as it was. Lines 13 and 14 remove the pawn taken en passant, lines 15 to 18 move the rook of a castling king, line 19 records the square a pawn skips with a two-square move, and lines 20 and 21 promote a pawn. Lines 22 and 23 move the piece, and the loop on lines 24 and 25 removes the castling rights of the two squares the move touched. `return board, not white, castling, en_passant` (line 26) hands back the new position with the other side to move.

### `attacks.py`

![attacks.py, lines 1 to 40](images/attacks_lines_1_to_40.png)

Create `attacks.py` in `engines/Remus1`. It has 40 lines when it is done.

Lines 3 to 6 list the eight knight jumps as (row change, column change) pairs, and lines 8 to 11 list the eight directions a queen or king can go. Type the `# fmt: skip` at the end of lines 6 and 11: it is the only comment a student pair types, and it stops VS Code from spreading the table over eight lines when it saves the file. `STRAIGHT = DIRECTIONS[0:4]` (line 12) takes the rook's four directions out of the list, and `DIAGONAL = DIRECTIONS[4:8]` (line 13) the bishop's.

`def is_attacked(board, square, by_white):` (line 16) checks each kind of attacker in turn. The loop header on lines 21 to 27 walks through five rows of a small table: the steps an attacker takes, whether it slides past empty squares, and which pieces attack that way. The `while` loop on lines 31 to 33 slides a rook, bishop, or queen along its line until it reaches a piece or the edge of the board.

`def in_check(board, white):` (line 39) finds the king of the side named by `white` with `board.index()`, and asks whether the other side attacks its square.

## Done when

Press Run Python File in `make_move.py` and then in `attacks.py`. Both files load without an error and print nothing. This checks syntax and imports; it does not call `make_move()`, `is_attacked()`, or `in_check()` to test their behavior. The finished Remus1 is tested with `verify_remus.py` in Period 4.
