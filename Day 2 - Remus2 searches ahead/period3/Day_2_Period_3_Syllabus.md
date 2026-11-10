# Day 2, Period 3: Alpha-beta and iterative deepening

1:00 to 2:30. Part of [Day 2: Remus2 searches ahead](../Day_2_Syllabus.md).

## Goal

Remus2 looks ahead with alpha-beta, one move deeper at a time.

## The big picture

The student pair keeps working in `engines/Remus2`. This period finishes `search.py` and gives the engine its new name in `uci.py`. No file is added.

`alpha_beta()` is the search itself. It tries every legal move, then every answer to each move, and so on to a fixed depth, assuming each side picks the move best for itself. Alpha and beta are the scores each side is already sure of; once a move proves worse for one side than a move already found, the rest of its answers cannot change the result, and alpha-beta skips them. At the end of the depth it calls `quiescence()` from Period 2 to play out the captures. A side with no legal move has lost if it is in check, and the game is drawn if it is not.

`think()` replaces the random `think()` deleted in Period 2. It searches to depth 1, then depth 2, then depth 3, and so on, a method called iterative deepening, and stops when time runs short. Each finished depth prints an `info` line, and each new depth tries the last depth's best move first.

## The period step by step

| Time | Activity |
| --- | --- |
| 1:00 to 1:10 | Reminder of checkmate and stalemate: a side with no legal move has lost if in check, and draws if not. |
| 1:10 to 2:10 | Type `alpha_beta()` and `think()` (about 43 code lines), which finish `search.py`. Change the name line in `uci.py` to `id name Remus2`. |
| 2:10 to 2:30 | Run `verify_remus.py` in `engines/Remus2` and fix any file it names until Remus2 passes every test. |

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| `search.py` | Edited (begun in Period 2) | Type `alpha_beta()` and `think()` at the end of the file | 64 to 110 |
| `uci.py` | Copied from `engines/Remus1`, edited | Change the engine's name to Remus2 | 21 |

The instructor's versions are [`engines_instructor/Remus2/search.py`](../../engines_instructor/Remus2/search.py) and [`engines_instructor/Remus2/uci.py`](../../engines_instructor/Remus2/uci.py). Copy only the code: no comments, no docstrings, and no type hints. The line numbers on this page count the lines of the student pair's own files in `engines/Remus2`.

## The edits, file by file

### `search.py`

![search.py, lines 64 to 110](images/search_lines_64_to_110.png)

Open `search.py` in `engines/Remus2`. It ends at line 63, `return alpha`, the last line of `quiescence()`. Leave two blank lines and type the rest; the file has 110 lines when it is done.

`def alpha_beta(position, depth, ply, alpha, beta):` (line 66) is the search. `ply` counts the moves from the root, the position on the board, so `ply == 0` means the root. Lines 69 to 71 search one move deeper when the side to move is in check, so the engine never stops in the middle of a check. `return quiescence(position, alpha, beta)` (line 73) hands over to the captures when the depth runs out. The loop on lines 75 to 86 plays each move, skips it when it leaves the mover's king in check (lines 77 and 78), and searches the answers with `score = -alpha_beta(child, depth - 1, ply + 1, -beta, -alpha)` (line 80), a recursive call. A score of beta or more is a cutoff: the opponent will never allow this position, so line 82 returns at once. At the root, a new best score also records the move in `best_move` (lines 85 and 86). `return -(MATE_SCORE - ply) if check else 0` (line 88) scores checkmate and stalemate; subtracting `ply` makes a quicker mate score higher.

`def think(position, seconds):` (line 92) replaces Remus1's `think()` and keeps its name and its two inputs, so `uci.py` calls it the same way. It sets the deadline and clears the count (lines 94 to 96). The loop `for depth in range(1, 65):` (line 97) is iterative deepening. The `try` and `except SearchTimeout:` lines (98 to 101) catch the exception that `count_node()` throws when time runs out, just as `try` and `catch` do in Java. Line 104 prints the `info` line, and lines 105 to 109 stop when the engine has found a mate or has used half its time, since the next depth would take longer than all the depths before it.

### `uci.py`

![uci.py, line 21](images/uci_line_21.png)

The copied `uci.py` works for Remus2 unchanged except for one line, the name Lucas Chess shows. Change `print("id name Remus1")` to `print("id name Remus2")` (line 21).

## Done when

The student pair's Remus2 passes every test in `verify_remus.py`.
