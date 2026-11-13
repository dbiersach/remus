# Day 1, Period 3: Move generation

1:00 to 2:30. Part of [Day 1: Remus1, legal chess at random](../Day_1_Syllabus.md).

## Goal

Remus1 can list every move of a position, castling, en passant, and promotion included.

## The big picture

The student pair keeps building Remus1 from nothing in `engines/Remus1`, and this period's one file is new there. `move_generation.py` lists every move the side to move can make. It is the longest file of Remus1 and the heart of the rules of chess: once it is right, the engine can never play an illegal move, and every later version uses it unchanged.

The file lists the moves a piece could make before it checks whether the move leaves its own king in check. That check comes in Period 4, where `search.py` plays each move with `make_move()` and throws away the moves after which `in_check()` says the king is attacked. Listing first and checking after catches pins, discovered checks, and en passant captures that expose the king, with no extra code.

## The period step by step

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | One function moves every piece but the pawn: a knight or king takes one step, a bishop, rook, or queen slides. Reminder of the castling conditions. |
| 1:15 to 2:20 | Type `move_generation.py` (65 code lines): `add_pawn_move()`, `pawn_moves()`, `piece_moves()`, `castling_moves()`, and `generate_moves()`. Press Run Python File to check for syntax and import errors. No output is expected; this does not test the functions' behavior. |
| 2:20 to 2:30 | Catch-up. |

## What the student pair types this period

| File | New or edited | What it holds | Lines in your file |
| --- | --- | --- | --- |
| `move_generation.py` | New | `add_pawn_move()`, `pawn_moves()`, `piece_moves()`, `castling_moves()`, and `generate_moves()` | 1 to 75 |

The instructor's version is [`engines_instructor/Remus1/move_generation.py`](../../engines_instructor/Remus1/move_generation.py). Copy only the code: no comments, no docstrings, and no type hints. The line numbers on this page count the lines of the student pair's own file.

## The file, line by line

### `move_generation.py`

![move_generation.py, lines 1 to 75](images/move_generation_lines_1_to_75.png)

Create `move_generation.py` in `engines/Remus1`. It has 75 lines when it is done. Lines 1 and 2 import the knight jumps and the directions from `attacks.py` and the piece numbers from `board.py`.

`def add_pawn_move(moves, start, target):` (line 5) adds a pawn move to the list. A pawn that reaches the last rank, a square below 8 or from 56 up, adds four moves, one for each piece it can become; any other pawn move adds one move with promotion 0.

`def pawn_moves(board, white, en_passant, start, moves):` (line 11) lists one pawn's moves. A White pawn moves toward row 0, so `forward` is -8 for White and 8 for Black (line 12). Lines 14 to 18 push the pawn one square, and two squares from its first row. Lines 19 to 24 try the two diagonal captures, an enemy piece or the en passant square, and skip a side that would fall off the board.

`def piece_moves(board, white, start, steps, slides, moves):` (line 27) moves every other piece. A knight or king takes one step in each direction it has, and a bishop, rook, or queen slides on while the squares are empty. The `while` loop on lines 31 to 34 is the slide, and lines 35 and 36 add the last square if it is empty or holds an enemy piece.

`def castling_moves(board, white, castling, moves):` (line 39) adds castling when the right is still there, the squares between king and rook are empty, and the king does not start on, cross, or land on an attacked square. Line 45 stops early when neither side is open or the king is in check, and lines 47 and 49 test the square the king crosses. The landing square is tested later with every other move, when `search.py` checks the king after the move.

`def generate_moves(position):` (line 53) walks over the 64 squares, and for each piece of the side to move calls the function that moves that kind of piece (lines 61 to 72). Lines 73 and 74 add castling, and line 75 returns the list.

## Done when

Press Run Python File in `move_generation.py`. The file loads without an error and prints nothing. This checks syntax and imports, not whether the move-generation functions produce the right moves. The finished Remus1 is tested with `verify_remus.py` in Period 4.
