# Day 5, Period 4: Science results and take-home

3:00 to 4:30. Part of [Day 5: The science project](Day_5_Syllabus.md).

## Goal

Every student takes the code home, and every student pair presents its science results.

## The big picture

The student pair writes no new functions this period and uses the parameter values chosen earlier. Each student copies the whole Remus folder to a USB drive, leaving out `.venv`, so the four engines in `Day 1 - Remus1 plays legal chess/Remus1` to `Day 4 - Remus4 adds everything else/Remus4`, every line of them typed by the student pair, go home.

## The period step by step

| Time | Activity |
| --- | --- |
| 3:00 to 3:10 | Each student copies the Remus folder to a USB drive in File Explorer, leaving out `.venv`, then opens the copied folder in VS Code and opens an engine file and both science files. This checks that those files are present and readable; it does not test execution on another computer. The destination computer needs the setup in `README.md`. |
| 3:10 to 3:30 | Reopen `Remus.code-workspace` in the original folder on the class laptop, where `.venv` is installed. Each student pair uses Run Python File on `Day 5 - Solve a science problem with chess AI/hidden_peak.py` and `Day 5 - Solve a science problem with chess AI/phase_hunt.py`, closing each graph window to continue. Record the measured averages and right, wrong, and "can't tell" counts, together with `EXPLORE_WEIGHT`, `PHASE_COST`, and `MISSING`. Changed settings can produce different results from the printed references. The class collects the settings and results on the board. |
| 3:30 to 3:45 | Compare these supplied examples for the default settings: powder 0 is correctly identified as silicon and zincite; powder 12 ties between anatase with gold and anatase with silver. The normal program run displays powder 12, not powder 0. At the default settings, 50 of the 52 "can't tell" answers involve this gold/silver tie. Their library peaks differ by less than `TOLERANCE = 0.3`; discuss what additional measurement could distinguish them. Results with tuned settings can differ. |
| 3:45 to 4:20 | Each student pair presents its results, its parameter values, and its answer for powder 12 in three minutes. |
| 4:20 to 4:30 | What comes next: AI agents that steer real experiments at light sources such as NSLS-II. |

## The files

The student pair types no code this period.

## Done when

Every student has opened an engine file and both science files from the USB copy, and every student pair has presented its measured results and parameter values. Reading the copy is not an execution test on a different computer.
