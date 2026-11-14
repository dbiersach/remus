# Day 3, Period 2: The table, killer moves, and the null move

10:30 to 12:00. Part of [Day 3: Remus3](Day_3_Syllabus.md).

## Goal

Remus3 remembers positions it has searched, tries killer moves first, and prunes with the null move.

## The big picture

The student pair keeps working in `Day 3 - Remus3 adds the biggest gains/Remus3`, on the files copied and edited in Period 1. This period makes the search faster in three ways, adds one new file, and edits `search.py`.

Alpha-beta skips the most moves when it tries the best move first, so a faster search is mostly a better guess of the best move. The transposition table, the new file `transposition_table.py`, remembers every position the search has scored: its key, the depth searched, the score, and the best move. The same position comes up again and again by different move orders, and when it does the search reads the answer instead of searching it again, or at least tries the remembered best move first. Killer moves are quiet moves, moves that capture nothing, that caused a cutoff at the same depth in another line; they often cause one again, so they go right after the captures.

The null move is a test. If the side to move could pass, letting the opponent move twice, and still stand above beta, a real move will do even better, so the search cuts off without trying any. The test is skipped in check, and when the side to move has only a king and pawns, where passing can be the best move and the test would be wrong.

## The period step by step

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | Why move order matters to alpha-beta, and why the same position appears by many move orders: e4 e5 Nf3 and Nf3 e5 e4 reach one position. The null move: let the opponent move twice; if the side to move still stands above beta, cut off. |
| 10:45 to 11:00 | Type `transposition_table.py` (22 code lines): `store()` and `probe()`. |
| 11:00 to 11:55 | In `search.py`, add the killer moves and the table probe and store, then `has_pieces()` and null-move pruning in `alpha_beta()` (about 52 code lines). Press Run Python File in `uci.py` to check for syntax and import errors. No text is expected while it waits for UCI commands; starting it does not test the engine's chess functions. Close that run's terminal before continuing. |
| 11:55 to 12:00 | Swap driver and navigator. |

## What changes this period

| File | New or edited | What the student pair does | Lines in your file |
| --- | --- | --- | --- |
| `transposition_table.py` | New | Type the whole file: `store()` and `probe()` | 1 to 28 |
| `search.py` | Edited (begun in Period 1) | Import the table, add the killer moves, `has_pieces()`, the table probe and store, and the null move | 3, 20, 30 to 32, 35 to 50, 69 to 70, 82 to 146 |
| All other files | Unchanged since Period 1 | No edits this period | Unchanged |

Type the code shown in the screenshots. The line numbers refer to your own file at this point in the lesson. Work from the top down and preserve the displayed spacing.

## The edits, file by file

### `transposition_table.py`

![transposition_table.py, lines 1 to 28](../images/Day3/period2_transposition_table_lines_1_to_28.png)

Create `transposition_table.py` in `Day 3 - Remus3 adds the biggest gains/Remus3`. It has 28 lines when it is done.

`EXACT, LOWER, UPPER = 0, 1, 2` (line 3) names the three kinds of score the table holds: an exact score, a score that is at least this much (the search cut off above beta), and a score that is at most this much (no move reached alpha). `table = {}` (line 6) is the dictionary of positions, from key to entry.

`def store(key, depth, score, bound, move, ply):` (line 9) saves an entry, and clears the whole table when it holds more than `MAX_ENTRIES`, two million positions, so the table never fills the laptop's memory. `def probe(key, ply):` (line 19) reads an entry back, or returns `None` for a position never seen. Both functions adjust a mate score by `ply` (lines 10 to 13 and 24 to 27). A mate is stored as "mate so many moves from this position" rather than "from the root", so a position reached at another depth reads the right distance to mate.

### `search.py`

![search.py, line 3](../images/Day3/period2_search_line_3.png)

![search.py, line 20](../images/Day3/period2_search_line_20.png)

![search.py, lines 30 to 32](../images/Day3/period2_search_lines_30_to_32.png)

![search.py, lines 35 to 50](../images/Day3/period2_search_lines_35_to_50.png)

![search.py, lines 69 to 70](../images/Day3/period2_search_lines_69_to_70.png)

![search.py, lines 82 to 146](../images/Day3/period2_search_lines_82_to_146.png)

`search.py` grows from 118 lines to 167. The edits, from the top down:

Type `import transposition_table as tt` (line 3) under `import time`. The `as tt` lets the code write `tt.probe()` instead of `transposition_table.probe()`.

Type `CAPTURE = 10_000` (line 20) between the imports and `class SearchTimeout`, with one blank line above it and two below. It lifts every capture's priority above the killer moves' 5000 and 4000.

Under `best_move = None`, type the killer list (lines 30 to 32), from `killers = []` to `killers.append([None, None])`: two killer moves for each ply, up to 128 plies.

`ordered_moves()` gets a third input and the killers. Change its first line to `def ordered_moves(position, first, killer_pair):` (line 35). Add `CAPTURE` to the capture priority, `priority = CAPTURE + 10 * PIECE_VALUES[victim] - PIECE_VALUES[attacker]` (line 44), and to the promotion, `priority += CAPTURE + 9000` (line 46). Type the two killer lines under the promotion, `if move in killer_pair:` and `priority = 5000 if move == killer_pair[0] else 4000` (lines 47 and 48). Raise the first move's priority to `priority = 1_000_000` (line 50), above every capture.

`quiescence()` calls the new `ordered_moves()`. Change its loop to `for priority, move in ordered_moves(position, None, ()):` (line 69), with an empty tuple for no killers, and the test under it to `if priority < CAPTURE:` (line 70): a killer now has a priority above 0 but is still not a capture.

Type `def has_pieces(board, white):` (lines 82 to 87) above `alpha_beta()`. It counts the side's knights, bishops, rooks, and queens, for the null move test.

`alpha_beta()` gets most of the period's lines:

1. Change its first line to `def alpha_beta(position, depth, ply, alpha, beta, can_null=True):` (line 90). `can_null=True` gives the input a default value, so every call that leaves it out gets `True`; the null move search passes `False`, so two null moves never come in a row.
2. Replace `check = in_check(position[0], position[1])` with two lines, `board, white = position[0], position[1]` and `check = in_check(board, white)` (lines 93 and 94).
3. Replace `has_legal_move = False` and the `for` line under it with lines 99 to 121: the table probe (lines 99 to 110), which returns a stored score when the stored search went at least as deep, the null move (lines 111 to 117), the three values that track the best move (lines 118 to 120), and the new loop line, `for _, move in ordered_moves(position, first, killers[ply]):` (line 121). `first` is the table's best move for this position.
4. In the loop, change the legality test to `if in_check(child[0], white):` (line 123), and delete `has_legal_move = True`.
5. Replace the two lines `if score >= beta:` and `return score` under the recursive call with lines 126 to 128, which remember the best score and move, from `if score > best_score:` to `best = move`.
6. Under the `if ply == 0:` lines, type the cutoff, from `if score >= beta:` to `break` (lines 133 to 137). A quiet move that causes a cutoff becomes this ply's first killer, and the old first killer moves to second place.
7. Change `if not has_legal_move:` to `if best is None:` (line 138); the mate and stalemate line under it stays.
8. Replace the last line, `return alpha`, with lines 140 to 146, from `bound = tt.EXACT` to `return best_score`: the search stores its result in the table before it returns.

`think()` does not change this period.

## Done when

Press Run Python File in `uci.py`. It loads without an error and waits silently for UCI commands. Close that run's terminal before continuing. This checks startup, including syntax and imports; it does not test evaluation or search, and it does not show that the engine plays correctly. The finished Remus3 is tested with `verify_remus3.py` in Period 4.
