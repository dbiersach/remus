# Day 5, Period 1: Where to measure next

8:30 to 10:00. Part of [Day 5: The science project](../Day_5_Syllabus.md).

## Goal

The student pair's science AI chooses where to measure next and compares its measurement count with random guessing on the supplied benchmark.

## The big picture

No engine file changes today: `engines/Remus1` to `engines/Remus4` stay as they were at the end of Day 4. The student pair writes `candidate_squares()`, `score_candidate()`, and `choose_next_measurement()` in `hidden_peak.py`, a science program that reuses the engine's ideas, not its code: the squares of the sample not yet measured are the legal moves, and the score of a square is its evaluation. `hidden_peak.py` holds every other line already, so the student pair types only the bodies of these functions, from their docstrings. The student pair edits `science/hidden_peak.py`, and `science_instructor/hidden_peak.py` holds the finished version.

## The period step by step

| Time | Activity |
| --- | --- |
| 8:30 to 8:45 | A 10 by 10 sample hides a hot spot, and every measurement costs beam time. Choosing the next square is choosing a move: list the candidates, score each one, keep the best. |
| 8:45 to 9:30 | Write `candidate_squares()`, `score_candidate()`, and `choose_next_measurement()` in `hidden_peak.py` (19 code lines). Press Run Python File in `science/hidden_peak.py`. With `EXPLORE_WEIGHT = 8`, compare the printed averages: random 54.3; greedy weights 0, 8, and 40: 17.5, 9.4, and 19.4. Close each of the two graph windows to finish. |
| 9:30 to 10:00 | After matching the default results, change `EXPLORE_WEIGHT`, save, and rerun `science/hidden_peak.py`. The program compares weights 0, the chosen value, and 40 over 100 generated samples. It also prints expectimax's 24.7 average over only 10 samples: a separate lookahead demonstration, not the same 100-sample comparison. Record the chosen weight and measured averages; the printed expectation text is a fixed reference. |

## The files

| File | The instructor's version |
| --- | --- |
| `hidden_peak.py` | [`science_instructor/hidden_peak.py`](../../science_instructor/hidden_peak.py) |

### `hidden_peak.py`

These images show the function bodies from [`science_instructor/hidden_peak.py`](../../science_instructor/hidden_peak.py). In `science/hidden_peak.py`, replace the comment and the `raise` line under each docstring with the body. Work from the top of the file down, and the line numbers match when you reach each function.

![candidate_squares(), lines 100 to 105](images/hidden_peak_lines_100_to_105.png)

![score_candidate(), lines 131 to 134](images/hidden_peak_lines_131_to_134.png)

![choose_next_measurement(), lines 159 to 167](images/hidden_peak_lines_159_to_167.png)

### The graphs

When `hidden_peak.py` finishes printing its results, matplotlib opens these two graphs, one window at a time. Close each window and the program continues. The student pair types none of the graph code.

![One hidden sample, measured by random guessing and by the AI](images/hidden_peak_search.png)

![Average measurements for each strategy](images/hidden_peak_benchmark.png)

## Done when

With `EXPLORE_WEIGHT = 8`, the student pair has compared the printed 100-sample averages with 54.3 for random guessing and 17.5, 9.4, and 19.4 for greedy weights 0, 8, and 40. The separate 10-sample expectimax result is 24.7. Both graph windows have been closed and the program has exited. These benchmark results do not prove correctness on every possible sample.
