# Day 3, Period 3: The opening book and the clock

1:00 to 2:30. Part of [Day 3: Remus3](Day_3_Syllabus.md).

## Goal

Remus3 plays from an opening book and manages its clock.

## The big picture

The student pair keeps working in `Day 3 - Remus3 adds the biggest gains/Remus3`. This period adds two files, `opening_book.txt` and `opening_book.py`, and edits `search.py` and `uci.py`.

An opening book is a list of good opening lines, written as UCI moves such as `e2e4 e7e5 g1f3`. While a game follows a line of the book, Remus3 plays the line's next move at once instead of searching, so it reaches a sound middlegame and saves its clock for later. `opening_book.txt` is data, not code, so the student pair copies it in File Explorer and types none of it. `opening_book.py` reads it, and `uci.py` checks every book move with the engine's own move generator before playing it, so a wrong line in the book can never make Remus3 play an illegal move.

The clock gets smarter in two ways. `uci.py` now gives `think()` two times: the time it aims to use, and the longest it may take, a hard limit that is never more than a fifth of the time left on the clock. `think()` spends more of that time when the best move changes from one depth to the next, a sign that the position is hard. The search also uses an aspiration window: from depth 4 on, it first searches only a 50-centipawn window around the last depth's score, which skips more moves, and searches again with the full window when the score falls outside.

## The period step by step

| Time | Activity |
| --- | --- |
| 1:00 to 1:10 | An opening book is a list of good first moves in UCI text. Copy `opening_book.txt` from `Day 3 - Remus3 adds the biggest gains/Remus3` into `Day 3 - Remus3 adds the biggest gains/Remus3` in File Explorer: it is data, not code, so nobody types it. |
| 1:10 to 1:35 | Type `opening_book.py` (21 code lines): `load_book()` and `book_moves()`. |
| 1:35 to 2:25 | In `think()`, add the aspiration window and the time that grows when the best move changes; in `uci.py`, add `legal_book_move()`, the game's moves, and the hard time limit (about 30 code lines). |
| 2:25 to 2:30 | Change the name line in `uci.py` to `id name Remus3`. |

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| `opening_book.txt` | New, copied | Copy from `Day 3 - Remus3 adds the biggest gains/Remus3` into `Day 3 - Remus3 adds the biggest gains/Remus3` in File Explorer; type nothing | Data, not code |
| `opening_book.py` | New | Type the whole file: `load_book()` and `book_moves()` | 1 to 26 |
| `search.py` | Edited (begun in Period 1) | Add the aspiration window and the time that grows when the best move changes, in `think()` | 20, 150 to 178 |
| `uci.py` | Copied from `Day 2 - Remus2 searches ahead/Remus2`, edited | Add `legal_book_move()`, the game's moves, the hard time limit, and the name Remus3 | 3 to 7, 17 to 23, 27, 29, 35, 41 to 42, 47 to 61 |
| All other files | Unchanged since Period 1 | No edits this period | Unchanged |

Type the code shown in the screenshots. The line numbers refer to your own file at this point in the lesson. Work from the top down and preserve the displayed spacing.

## The edits, file by file

### `opening_book.txt`

In File Explorer, copy `Day 3 - Remus3 adds the biggest gains/Remus3/opening_book.txt` into `Day 3 - Remus3 adds the biggest gains/Remus3`. Each line is one opening, such as `e2e4 e7e5 g1f3 b8c6 f1b5`. A line that starts with `White:` or `Black:` is played only by that side: a trap defense, or an opening that leaves the other side too low a score at its end. A line that starts with `#` is a comment.

### `opening_book.py`

![opening_book.py, lines 1 to 26](../images/Day3/period3_opening_book_lines_1_to_26.png)

Create `opening_book.py` in `Day 3 - Remus3 adds the biggest gains/Remus3`. It has 26 lines when it is done.

`book = []` (line 4) holds the book's lines once they are read. `def load_book():` (line 7) finds `opening_book.txt` in the engine's own folder with `Path(__file__).parent` (line 8), so the file is found wherever Lucas Chess starts the engine. Line 9 returns quietly when the file is missing: an engine with no book still plays. The loop on lines 11 to 16 splits each line into its side, if any, and its list of moves.

`def book_moves(played, white):` (line 19) returns the next move of every book line that starts with the moves played so far, in random order (line 25), so Remus3 does not play the same opening every game.

### `search.py`

![search.py, line 20](../images/Day3/period3_search_line_20.png)

![search.py, lines 150 to 178](../images/Day3/period3_search_lines_150_to_178.png)

`search.py` grows from 167 lines to 180, all in `think()` except one constant.

Type `ASPIRATION_WINDOW = 50` (line 20) above `CAPTURE = 10_000`.

In `think()`, from the top down:

1. Change its first line to `def think(position, seconds, longest):` (line 150), with `longest`, the hard limit, as a third input.
2. Replace `deadline = time.perf_counter() + seconds` with `start = time.perf_counter()` and `deadline = start + longest` (lines 152 and 153). The search stops for good at the hard limit.
3. Type `score = 0` (line 156) under `best_move = None`, so depth 4 has a score to center its window on.
4. At the top of the `for depth` loop, type the four lines from `previous = best_move` to `alpha, beta = score - ASPIRATION_WINDOW, score + ASPIRATION_WINDOW` (lines 158 to 161).
5. Inside `try:`, replace the one line `score = alpha_beta(position, depth, 0, -MATE_SCORE - 1, MATE_SCORE + 1)` with the eight lines from `while True:` to `break` (lines 163 to 170). A score at or below alpha, or at or above beta, fell outside the window, so the loop opens that side of the window to the full range and searches again.
6. Below the `print` line, replace the five lines of the stop test, from `if (` to `):`, with three lines (176 to 178): `if previous is not None and best_move != previous:`, `seconds = min(seconds * 1.5, longest)`, and `if abs(score) > MATE_SCORE - 100 or time.perf_counter() > start + seconds / 2:`. The `break` under them stays.

### `uci.py`

![uci.py, lines 3 to 7](../images/Day3/period3_uci_lines_3_to_7.png)

![uci.py, lines 17 to 23](../images/Day3/period3_uci_lines_17_to_23.png)

![uci.py, line 27](../images/Day3/period3_uci_line_27.png)

![uci.py, line 29](../images/Day3/period3_uci_line_29.png)

![uci.py, line 35](../images/Day3/period3_uci_line_35.png)

![uci.py, lines 41 to 42](../images/Day3/period3_uci_lines_41_to_42.png)

![uci.py, lines 47 to 61](../images/Day3/period3_uci_lines_47_to_61.png)

`uci.py` grows from 51 lines to 68. The edits, from the top down:

1. Replace the import `from board import START_POSITION, parse_square, square_name` with two lines, `from attacks import in_check` and `from board import START_POSITION, move_text, parse_square` (lines 3 and 4).
2. Type `from move_generation import generate_moves` and `from opening_book import book_moves, load_book` (lines 6 and 7) under the `make_move` import.
3. Type `def legal_book_move(position, played):` (lines 17 to 23) between `read_time()` and `main()`. It returns the first book move that the move generator also lists and that leaves the king safe, or `None`.
4. At the top of `main()`, type `load_book()` (line 27), and under `position = START_POSITION` type `played = []` (line 29).
5. Change the name line to `print("id name Remus3")` (line 35).
6. In the `position` branch, replace `for text in words[3:]:` with `played = words[3:]` and `for text in played:` (lines 41 and 42), so the engine remembers the game's moves for the book.
7. At the top of the `go` branch, replace the three lines from `clock_left = read_time(words, "wtime" if position[1] else "btime")` to `seconds = 2.0` with the four lines from `white = position[1]` to `seconds = longest = 2.0` (lines 47 to 50).
8. Under `seconds = (clock_left / 30 + increment / 2) / 1000`, type `longest = min(3 * seconds, clock_left / 5000)` (line 53): at most three times the planned time, and never more than a fifth of the clock.
9. Change the `movetime` line to `seconds = longest = read_time(words, "movetime") / 1000` (line 55), and replace `move = think(position, seconds)` with the three lines from `move = legal_book_move(position, played)` to `move = think(position, seconds, longest)` (lines 56 to 58).
10. Replace the three lines that build the move's text, from `text = square_name(move[0]) + square_name(move[1])` to `text += ".pnbrqk"[move[2]]`, with the one line `text = move_text(move)` (line 61).

## Done when

The opening-book and clock edits are complete, and the name line in `uci.py` reads `id name Remus3`. Press Run Python File in `uci.py` to check for syntax and import errors. It waits silently for UCI commands; starting it does not test book moves or time management. Close that run's terminal before continuing. The finished Remus3 is tested with `verify_remus3.py` in Period 4.
