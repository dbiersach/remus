# Day 5: The science project

**Goal for the day:** the student pairs aim the engine's ideas at two problems from a synchrotron beamline, report what their science AI found, and take their code home.

The project uses two programs, `hidden_peak.py` and `phase_hunt.py`. The student pair
works in the top `science/` folder, where each program holds every line but the
functions the student pair writes; the top `science_instructor/` folder holds the
finished programs. Both use common Python modules. Neither imports the engine:
they reuse the engine's ideas, not its code.

| Chess engine | Hidden peak | Phase hunt |
| --- | --- | --- |
| Legal moves | Squares not yet measured | Library phases not yet in the set |
| `make_move()` | Add a reading | Add a phase to the set |
| Evaluation | Predicted value plus distance to the nearest reading | Peaks explained, minus peaks left over, missing, and phases used |
| Search | Choose the best-scoring square | Score every set of up to three phases |
| Several moves with the best score | | Several sets tie: the pattern cannot decide |
| `verify_remus.py` | The average measurements over 100 hidden samples | The verdicts on 200 known powders |

## The two programs

Each program aims an idea from the Remus engines at a problem from a synchrotron beamline, such as NSLS-II at Brookhaven.

| Program | The question | The chess idea | Day 5 periods |
| --- | --- | --- | --- |
| `hidden_peak.py` | A 10 by 10 sample hides a hot spot. Where should the next measurement go? | The squares not yet measured are the legal moves, a score for each square is the evaluation, and the best-scoring square is the move. | Period 1 |
| `phase_hunt.py` | Which crystal phases are in a powder, judged from its X-ray peaks? | A set of phases is a position, adding a phase is a move, and a search scores every set of up to three phases. | Periods 2 and 3 |

The student pair writes the bodies of `candidate_squares()`, `score_candidate()`, and `choose_next_measurement()` in [`science/hidden_peak.py`](../science/hidden_peak.py), then `score()`, `greedy_phases()`, and `search()` in [`science/phase_hunt.py`](../science/phase_hunt.py). Each empty function keeps its docstring, which says what the function must do, and a comment names its period. A student pair that is stuck compares its function with the one in [`science_instructor/`](../science_instructor/).

## Running a program

Open the file in VS Code and click Run Python File. `main()` prints each result next to the result it expects, and matplotlib opens each graph in its own window. Close the window and the program continues. In `science/`, a function not written yet stops the run with `NotImplementedError` and its name, such as "Write score()".

Expected results of the finished programs:

- `hidden_peak.py`: random guessing needs 54.3 measurements on average over 100 hidden samples, and the program's AI with explore weight 8 needs 9.4.
- `phase_hunt.py`: on 200 known powders the greedy answer gets 166 right and 34 wrong, and the search gets 139 right, 52 "can't tell", and 9 wrong.

## The day at a glance

| Period | Time | Title |
| --- | --- | --- |
| P1 | 8:30 to 10:00 | [Where to measure next](period1/Day_5_Period_1_Syllabus.md) |
| | 10:00 to 10:30 | Break |
| P2 | 10:30 to 12:00 | [Name the crystals, the greedy way](period2/Day_5_Period_2_Syllabus.md) |
| | 12:00 to 1:00 | Lunch |
| P3 | 1:00 to 2:30 | [Search every set, and verify](period3/Day_5_Period_3_Syllabus.md) |
| | 2:30 to 3:00 | Break |
| P4 | 3:00 to 4:30 | [Science results and take-home](period4/Day_5_Period_4_Syllabus.md) |

## Period 1, 8:30 to 10:00: Where to measure next

| Time | Activity |
| --- | --- |
| 8:30 to 8:45 | A 10 by 10 sample hides a hot spot, and every measurement costs beam time. Choosing the next square is choosing a move: list the candidates, score each one, keep the best. |
| 8:45 to 9:30 | Write `candidate_squares()`, `score_candidate()`, and `choose_next_measurement()` in `hidden_peak.py` (19 code lines). Run it. |
| 9:30 to 10:00 | Tune `EXPLORE_WEIGHT` and compare. Over 100 hidden samples, random guessing needs 54.3 measurements on average, the AI with weight 0 needs 17.5, weight 8 needs 9.4, and weight 40 needs 19.4. The teacher shows the expectimax lookahead (24.7): it looks two measurements ahead, but its crude model makes it worse than the one-step AI. |

[`Day_5_Period_1_Syllabus.md`](period1/Day_5_Period_1_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## Period 2, 10:30 to 12:00: Name the crystals, the greedy way

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | The PDF beamline at Brookhaven's NSLS-II and its daily question: which crystal phases are in this powder? Each phase makes its own fingerprint of X-ray peaks. |
| 10:45 to 11:15 | Write `score()` in `phase_hunt.py` (11 code lines): a point for each explained peak, and a point lost for each left-over peak, missing peak, and phase used. |
| 11:15 to 12:00 | Write `greedy_phases()` (15 code lines), which adds the best phase again and again. On 200 known powders greedy names 166 right and 34 wrong, and on powder 12 it mistakes silver for gold. |

[`Day_5_Period_2_Syllabus.md`](period2/Day_5_Period_2_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## Period 3, 1:00 to 2:30: Search every set, and verify

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | The tree of phase sets beside Remus's game tree: a set is a position and adding a phase is a move. Quartz then rutile is the same set as rutile then quartz, the same idea as the transposition table. |
| 1:15 to 1:50 | Write `search()` (12 code lines), which scores every set of up to three phases and keeps every set that ties for the best score. |
| 1:50 to 2:15 | Verify on the 200 known powders: 139 right, 52 "can't tell", and 9 wrong, against greedy's 34 wrong. The search scores 176 sets per powder in library order, against 821 in every order. Change `PHASE_COST` and `MISSING` and watch the verdicts move. |
| 2:15 to 2:30 | The programs that search the same tree on real synchrotron data, and the agent that steers measurements at a beamline. |

[`Day_5_Period_3_Syllabus.md`](period3/Day_5_Period_3_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## Period 4, 3:00 to 4:30: Science results and take-home

| Time | Activity |
| --- | --- |
| 3:00 to 3:10 | Each student copies the Remus folder to a USB drive in File Explorer, leaving out `.venv`, and checks that the copy opens in VS Code. |
| 3:10 to 3:30 | Each student pair runs both programs once more and fills in its results: measurements against random guessing, and right, wrong, and can't-tell counts for greedy and for search, with the `PHASE_COST` and `MISSING` the student pair picked in Period 3. The class collects every student pair's numbers and its two values on the board. |
| 3:30 to 3:45 | Each student pair compares two powders. The search names powder 0 right: silicon and zincite. Powder 12 holds anatase and silver, and the search answers "can't tell": anatase with gold, or anatase with silver. Every peak of gold lands less than 0.3 degrees from a peak of silver, inside `TOLERANCE`, and 50 of the search's 52 "can't tell" answers are this same tie. No X-ray peak in the library settles powder 12, so the student pair works out what other test would. |
| 3:45 to 4:20 | Each student pair presents its results, its two values, and its answer for powder 12 in three minutes. |
| 4:20 to 4:30 | What comes next: AI agents that steer real experiments at light sources such as NSLS-II. |

[`Day_5_Period_4_Syllabus.md`](period4/Day_5_Period_4_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## By the end of the day

**Checkpoint:** every student pair's `choose_next_measurement()` beats random guessing, every student pair's `search()` makes fewer wrong calls than greedy, and every student leaves with the code on a USB drive.
