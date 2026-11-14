# Day 5: The science project

**Goal for the day:** the student pairs aim the engine's ideas at two problems from a synchrotron beamline, report what their science AI found, and take their code home.

The project uses two programs, `hidden_peak.py` and `phase_hunt.py`. The student pair
works in the Day 5 folder. Each program supplies the graphing and measurement code; student pairs type the function bodies shown in the period screenshots. Both use common Python modules. Neither imports the engine:
they reuse the engine's ideas, not its code.

| Chess engine | Hidden peak | Phase hunt |
| --- | --- | --- |
| Legal moves | Squares not yet measured | Library phases not yet in the set |
| `make_move()` | Add a reading | Add a phase to the set |
| Evaluation | Predicted value plus distance to the nearest reading | Peaks explained, minus peaks left over, missing, and phases used |
| Search | Choose the best-scoring square | Score every set of up to three phases |
| Several moves with the best score | | Several sets tie: the pattern cannot decide |
| Reference checks with `verify_remusN.py` (with your version number in place of N) | Manually compare the averages over 100 generated samples | Manually compare the verdict counts on 200 generated powders |

## The two programs

Each program aims an idea from the Remus engines at a problem from a synchrotron beamline, such as NSLS-II at Brookhaven.

| Program | The question | The chess idea | Day 5 periods |
| --- | --- | --- | --- |
| `hidden_peak.py` | A 10 by 10 sample hides a hot spot. Where should the next measurement go? | The squares not yet measured are the legal moves, a score for each square is the evaluation, and the best-scoring square is the move. | Period 1 |
| `phase_hunt.py` | Which crystal phases are in a powder, judged from its X-ray peaks? | A set of phases is a position, adding a phase is a move, and a search scores every set of up to three phases. | Periods 2 and 3 |

The student pair writes the bodies of `candidate_squares()`, `score_candidate()`, and `choose_next_measurement()` in [`Day 5 - Solve a science problem with chess AI/hidden_peak.py`](hidden_peak.py), then `score()`, `greedy_phases()`, and `search()` in [`Day 5 - Solve a science problem with chess AI/phase_hunt.py`](phase_hunt.py). Each empty function keeps its docstring, which says what the function must do, and a comment names its period. A student pair that is stuck compares its function with the period screenshot and asks the teacher for help.

## Running a program

Open the program in the Day 5 folder and press Run Python File. Its `main()` calls the student pair's functions, prints measured benchmark results, and opens graph windows. Close each window to continue; the program has not finished while it is waiting for a graph to close. Compare the actual numbers with the references below. The printed "expected" messages are fixed text, not assertions: a mismatch does not automatically stop the program or produce a failure message. The references use the supplied settings; changing a parameter can legitimately change the results. A remaining unfinished function stops the run with `NotImplementedError` and its name. At the end of Period 2, `Write search()` is expected after the greedy results and first graph; another error or an earlier stop is not that planned stopping point.

Reference results of the finished programs with the supplied settings (`EXPLORE_WEIGHT = 8`, `PHASE_COST = 1`, and `MISSING = 1`, with the other constants unchanged):

- `hidden_peak.py`: over 100 generated samples, random guessing averages 54.3 measurements; greedy weights 0, 8, and 40 average 17.5, 9.4, and 19.4. Expectimax averages 24.7 over a separate 10-sample run.
- `phase_hunt.py`: on 200 generated benchmark powders the greedy answer gets 166 right and 34 wrong, and the search gets 139 right, 52 "can't tell", and 9 wrong.

## The day at a glance

| Period | Time | Title |
| --- | --- | --- |
| P1 | 8:30 to 10:00 | [Where to measure next](Day_5_Period_1_Syllabus.md) |
| | 10:00 to 10:30 | Break |
| P2 | 10:30 to 12:00 | [Name the crystals, the greedy way](Day_5_Period_2_Syllabus.md) |
| | 12:00 to 1:00 | Lunch |
| P3 | 1:00 to 2:30 | [Search every set, and verify](Day_5_Period_3_Syllabus.md) |
| | 2:30 to 3:00 | Break |
| P4 | 3:00 to 4:30 | [Science results and take-home](Day_5_Period_4_Syllabus.md) |

## Period 1, 8:30 to 10:00: Where to measure next

| Time | Activity |
| --- | --- |
| 8:30 to 8:45 | A 10 by 10 sample hides a hot spot, and every measurement costs beam time. Choosing the next square is choosing a move: list the candidates, score each one, keep the best. |
| 8:45 to 9:30 | Write `candidate_squares()`, `score_candidate()`, and `choose_next_measurement()` in `hidden_peak.py` (19 code lines). Press Run Python File in `Day 5 - Solve a science problem with chess AI/hidden_peak.py`. With `EXPLORE_WEIGHT = 8`, compare the printed averages: random 54.3; greedy weights 0, 8, and 40: 17.5, 9.4, and 19.4. Close each of the two graph windows to finish. |
| 9:30 to 10:00 | After matching the default results, change `EXPLORE_WEIGHT`, save, and rerun `Day 5 - Solve a science problem with chess AI/hidden_peak.py`. The program compares weights 0, the chosen value, and 40 over 100 generated samples. It also prints expectimax's 24.7 average over only 10 samples: a separate lookahead demonstration, not the same 100-sample comparison. Record the chosen weight and measured averages; the printed expectation text is a fixed reference. |

[`Day_5_Period_1_Syllabus.md`](Day_5_Period_1_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## Period 2, 10:30 to 12:00: Name the crystals, the greedy way

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | The PDF beamline at Brookhaven's NSLS-II and its daily question: which crystal phases are in this powder? Each phase makes its own fingerprint of X-ray peaks. |
| 10:45 to 11:15 | Write `score()` in `phase_hunt.py` (11 code lines): a point for each explained peak, and a point lost for each left-over peak, missing peak, and phase used. |
| 11:15 to 12:00 | Write `greedy_phases()` (15 code lines), then press Run Python File in `Day 5 - Solve a science problem with chess AI/phase_hunt.py`. With the supplied settings, compare the printed counts: 166 right, 0 "can't tell", and 34 wrong over 200 generated powders. Powder 12 is reported as anatase and gold instead of anatase and silver. Close its graph; `NotImplementedError: Write search()` is expected until Period 3. |

[`Day_5_Period_2_Syllabus.md`](Day_5_Period_2_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## Period 3, 1:00 to 2:30: Search every set, and verify

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | The tree of phase sets beside Remus's game tree: a set is a position and adding a phase is a move. Quartz then rutile is the same set as rutile then quartz, the same idea as the transposition table. |
| 1:15 to 1:50 | Write `search()` (12 code lines), which scores every set of up to three phases and keeps every set that ties for the best score. |
| 1:50 to 2:15 | Press Run Python File in `Day 5 - Solve a science problem with chess AI/phase_hunt.py` with the supplied settings (`PHASE_COST = 1`, `MISSING = 1`). Close the first graph to reach the search results. Manually compare 139 right, 52 "can't tell", and 9 wrong with greedy's 166 right and 34 wrong; expect 176 sets in library order and 821 in every order. Close the second graph to finish. Then change `PHASE_COST` and `MISSING`, save, and rerun; record the new counts rather than expecting the default counts. |
| 2:15 to 2:30 | The programs that search the same tree on real synchrotron data, and the agent that steers measurements at a beamline. |

[`Day_5_Period_3_Syllabus.md`](Day_5_Period_3_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## Period 4, 3:00 to 4:30: Science results and take-home

| Time | Activity |
| --- | --- |
| 3:00 to 3:10 | Each student copies the Remus folder to a USB drive in File Explorer, leaving out `.venv`, then opens the copied folder in VS Code and opens an engine file and both science files. This checks that those files are present and readable; it does not test execution on another computer. The destination computer needs the setup in `README.md`. |
| 3:10 to 3:30 | Reopen `Remus.code-workspace` in the original folder on the class laptop, where `.venv` is installed. Each student pair uses Run Python File on `Day 5 - Solve a science problem with chess AI/hidden_peak.py` and `Day 5 - Solve a science problem with chess AI/phase_hunt.py`, closing each graph window to continue. Record the measured averages and right, wrong, and "can't tell" counts, together with `EXPLORE_WEIGHT`, `PHASE_COST`, and `MISSING`. Changed settings can produce different results from the printed references. The class collects the settings and results on the board. |
| 3:30 to 3:45 | Compare these supplied examples for the default settings: powder 0 is correctly identified as silicon and zincite; powder 12 ties between anatase with gold and anatase with silver. The normal program run displays powder 12, not powder 0. At the default settings, 50 of the 52 "can't tell" answers involve this gold/silver tie. Their library peaks differ by less than `TOLERANCE = 0.3`; discuss what additional measurement could distinguish them. Results with tuned settings can differ. |
| 3:45 to 4:20 | Each student pair presents its results, its parameter values, and its answer for powder 12 in three minutes. |
| 4:20 to 4:30 | What comes next: AI agents that steer real experiments at light sources such as NSLS-II. |

[`Day_5_Period_4_Syllabus.md`](Day_5_Period_4_Syllabus.md) has the period's goal, the big picture, what the student pair writes, and how the student pair knows it is done.

## By the end of the day

**Checkpoint:** every student pair has compared its default benchmark results with the references, recorded the effects of its chosen parameters, and presented its findings. Every student leaves with a readable copy of the code on a USB drive.
