# Day 2, Period 2: Move ordering and quiescence

10:30 to 12:00. Part of [Day 2: Remus2 searches ahead](Day_2_Syllabus.md).

## Goal

The top of Remus2's search orders the moves and plays out the captures.

## The big picture

The student pair keeps working in `Day 2 - Remus2 searches ahead/Remus2`, on the copies made this morning. This period rewrites `search.py`, the file copied from Remus1 whose `think()` picks a random move. Remus2's search looks ahead instead, so almost all of the old file goes, and the period types the first half of the new one. No other file changes, and no new file is added.

The first half of the new `search.py` holds three helpers that the search of Period 3 calls. `ordered_moves()` puts the most promising moves first, because alpha-beta search, which comes in Period 3, skips more moves when it tries the best one early. It scores a capture by MVV-LVA, "most valuable victim, least valuable attacker": pawn takes queen comes before queen takes pawn. `count_node()` counts the positions searched and stops the search when its time is up. `quiescence()` keeps playing captures at the end of the search until the position is quiet, so the engine never stops looking in the middle of an exchange, with a queen just taken and the recapture not yet seen.

Until Period 3 finishes `search.py`, the file has no `think()`, so `uci.py` cannot run this period. Press Run Python File in `search.py` to check for syntax and import errors instead. No output is expected; this does not call the search functions or test their behavior.

## The period step by step

| Time | Activity |
| --- | --- |
| 10:30 to 10:50 | Alpha-beta on paper: look ahead, assume both sides play their best, and skip the moves that cannot change the result. MVV-LVA: try pawn takes queen before queen takes pawn. |
| 10:50 to 11:55 | In `Day 2 - Remus2 searches ahead/Remus2`, keep the three import lines of Remus1's `search.py`, change `import random` to `import time`, delete Remus1's random-move `think()`, and type the rest of the top of Remus2's `search.py` (about 50 code lines): the imports and constants, `ordered_moves()`, `count_node()`, and `quiescence()`. |
| 11:55 to 12:00 | Swap driver and navigator. |

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| `search.py` | Copied from `Day 1 - Remus1 plays legal chess/Remus1`, edited | Keep the three import lines, change the first line, delete `think()`, and type the first half of Remus2's search | 1 to 63 |

Type the code shown in the screenshots. The line numbers refer to your own file at this point in the lesson. Work from the top down and preserve the displayed spacing.

## The edits, file by file

### `search.py`

![search.py, lines 1 to 63](../images/Day2/period2_search_lines_1_to_63.png)

Open `search.py` in `Day 2 - Remus2 searches ahead/Remus2`. It is Remus1's 14-line file, and it becomes 63 lines this period.

Start with the imports. Remus1's three imports of `in_check()`, `make_move()`, and `generate_moves()` stay, because Remus2's search needs all three. Change the first line, `import random`, to `import time` (line 1): Remus2 needs a clock, not dice. Then type two new imports between the `attacks` and `make_move` imports: `from board import EMPTY, QUEEN` (line 4) and `from evaluation import PIECE_VALUES, evaluate` (line 5), the file typed in Period 1. The imports now fill lines 1 to 7.

Delete everything below the imports: the whole of Remus1's `think()`, from `def think(position, seconds):` to `return random.choice(legal) if legal else None`. Period 3 types Remus2's own `think()`.

Below the imports, type the constants and the search's memory. `MATE_SCORE = 100_000` (line 9) is the score of a checkmate, larger than any material count; Python lets an underscore stand in for a comma in a number. `class SearchTimeout(Exception):` (line 12) defines an exception, as `class SearchTimeout extends Exception` does in Java, and `count_node()` throws it when the time is up. Lines 16 to 18 hold three values every search function shares: the number of positions searched, the time to stop, and the best move found so far.

`def ordered_moves(position, first):` (line 21) gives each move a priority (lines 26 to 34) and returns the list sorted highest first. A capture scores ten times the victim's value minus the attacker's value, a queen promotion adds 9000, and the move named `first`, the best move of the last search, goes to the front with 100,000.

`def count_node():` (line 40) adds one to the count and, every 1024 positions, looks at the clock. `global nodes` (line 41) lets the function change the shared count instead of making a new local one.

`def quiescence(position, alpha, beta):` (line 47) scores the position as it stands (`stand_pat`, line 49), then tries only the captures: line 54 stops at the first move with priority 0, which is the first move that captures nothing. `score = -quiescence(child, -beta, -alpha)` (line 59) is a recursive call: the score of a move for one side is the opposite of the score for the other side.

## Done when

Press Run Python File in `search.py`. The file loads without an error and prints nothing. This checks syntax and imports, not whether the search functions work. The finished Remus2 is tested with `verify_remus2.py` in Period 3.
