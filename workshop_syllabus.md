# Workshop syllabus

Each day also has its own top folder, such as `Day 1 - Remus1 plays legal
chess`, with a page for the day and a page for each period.

## The week at a glance

The students already know some coding and know how to play chess, including notation,
so every period goes to the engine. Each day has four 90-minute periods: P1 8:30 to
10:00, P2 10:30 to 12:00, P3 1:00 to 2:30, and P4 3:00 to 4:30.

| Day | What the student pairs build | Code lines typed | Ends with |
| --- | --- | ---: | --- |
| 1 | Remus1: the board, moves, attacks, move generation, a random mover, and the UCI loop | 193 | Remus1 plays legal chess in Lucas Chess |
| 2 | Remus2: the evaluation and the search | 161 | Remus2 plays Remus1 and the student pair in Lucas Chess |
| 3 | Remus3: pawn structure, the table, killers, the null move, the book, the clock | 193 | Remus3 plays Remus2 and the student pair |
| 4 | Remus4: draw sense, pruning, exchanges, the full evaluation | 159 | Remus4 plays Remus3 and Remus4-Rust |
| 5 | The science project: where to measure, and which crystals | 57 | Science results and the take-home copy |

Student pairs type only the code: no comments and no docstrings. The code lines
count what they type, the same count as the engine READMEs, and no day asks for more
than about 200. The instructor's engines in `engines_instructor/` keep every comment and
docstring, so a student pair reads them there during the week and takes them home on
Day 5.

## How a student pair works

- **One folder for each version.** The top `engines/` folder holds four
  folders, `Remus1` to `Remus4`, for the student pair's own code. Each starts with
  only `remus.bat` and `verify_remus.py`. On Day 1 the student
  pair creates every file of Remus1 in `engines/Remus1`. At the start of Day 2 the
  student pair copies every file of `engines/Remus1` into `engines/Remus2` in File
  Explorer, then edits those copies into Remus2. Day 3 starts the same way, from
  `engines/Remus2` into `engines/Remus3`, and Day 4 from `engines/Remus3` into
  `engines/Remus4`. Each version stays playable in its own folder.
- **Copy from the instructor's version.** The finished engines in
  `engines_instructor/Remus1` to `engines_instructor/Remus4` are the pages the
  student pair copies from.
  Each engine README in `docs/engines/Remus2` to `docs/engines/Remus4` lists the files and their
  line counts, and its `call_tree.svg` shows which function calls which.
- **Every edit on the period page.** Each period's page in its day folder, such as
  `Day_3_Period_2_Syllabus.md`, opens with the big picture of what the student pair
  types or edits and why, then a table of every file that is new or changes that
  period, then the edits file by file, with the line numbers of the student pair's
  own file. Those files hold no comments, so their line numbers are smaller than
  the instructor's. The student pair still types every line itself: from Day 2 on,
  the only copying in File Explorer is the student pair's own files from the day
  before, plus `opening_book.txt`, which is data, on Day 3.
- **Verify each finished version.** Every `engines/RemusN` folder starts with
  `verify_remus.py`, which the student pair runs but never types. Run with the
  Run Python File button once the version is typed, it counts the legal moves from
  six published positions and compares them with the published counts, tests the
  evaluation and a four-move search against the instructor's engine (Remus2 to
  Remus4), and talks to `uci.py` the way Lucas Chess does. At the first difference
  it stops and names the files to look at. On Day 4 the student pair also runs it
  at the end of Periods 1 to 3: it reads which period Remus4 has reached and
  compares the engine with the instructor's Remus4 at the end of that period.
- **Driver and navigator.** The driver types and the navigator reads the instructor's
  file aloud and checks each line. They swap at the start of every period.
- **Run after every file.** Open the file just typed and press Run Python File. An
  engine module prints nothing, so a clean run shows no output at all. A typo shows a
  red error that names the file and the line. Because each module imports the ones
  before it, running the newest file also checks every file it imports.
- **Test your own engine, play the instructor's.** `verify_remus.py` compares the
  student pair's engine with the instructor's numbers. Lucas Chess holds the
  instructor's engines from `engines_instructor/`, which step 6 of the Day 1 setup
  adds, so nobody adds an engine by hand.

## Day 1: Remus1, legal chess at random

**Goal for the day:** `engines/Remus1` holds Remus1, which knows every rule of chess and
plays a random legal move, and each student plays it in Lucas Chess.

### P1, 8:30 to 10:00: Set up, and the board

| Time | Activity |
| --- | --- |
| 8:30 to 9:15 | Setup, steps 1 to 6 of the top `README.md`: install Git, VS Code, and `uv`, clone the repository, run `uv sync`, install the extensions, open `Remus.code-workspace`, and run `install_lucas_engines.py` to add the engines to Lucas Chess. This is the only time anyone types a git or `uv` command. |
| 9:15 to 9:30 | The teacher starts the instructor's Remus2 against Stockfish 1500 in Lucas Chess (Part 4 of the [Lucas Chess guide](docs/lucas_chess/lucas_chess_guide.md)), so the class sees the engine it will build by tomorrow afternoon. |
| 9:30 to 9:40 | Each student pair opens its `engines/Remus1` folder in VS Code. The folder holds `remus.bat`, the one line that starts the engine, and `verify_remus.py`, which tests the finished engine; the student pair types every other file. |
| 9:40 to 10:00 | Type `board.py` (15 code lines): the piece numbers, the starting position, and square names. The reminder before typing: row 0 is rank 8, column 0 is file a, so a8 is square 0 and h1 is square 63. |

### P2, 10:30 to 12:00: Moves and attacks

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | The position tuple and copy-make: `make_move()` copies the board and returns a new position, so no move is ever taken back. Reminder of castling rights, en passant, and promotion. |
| 10:45 to 11:15 | Type `make_move.py` (23 code lines). Run it. |
| 11:15 to 12:00 | Type `attacks.py` (34 code lines): the knight jumps, the eight directions, `is_attacked()`, and `in_check()`. Run it. |

### P3, 1:00 to 2:30: Move generation

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | One function moves every piece but the pawn: a knight or king takes one step, a bishop, rook, or queen slides. Reminder of the castling conditions. |
| 1:15 to 2:20 | Type `move_generation.py` (65 code lines): `add_pawn_move()`, `pawn_moves()`, `piece_moves()`, `castling_moves()`, and `generate_moves()`. Run it. |
| 2:20 to 2:30 | Catch-up. |

### P4, 3:00 to 4:30: The UCI loop, and Remus1 plays

| Time | Activity |
| --- | --- |
| 3:00 to 3:10 | How Lucas Chess and Remus talk: read [`uci.md`](docs/chess_tutor/uci.md) together. A move is legal only if the mover's own king is not attacked after it. |
| 3:10 to 3:50 | Type `search.py` of `engines_instructor/Remus1` (12 code lines), whose `think()` plays a random legal move, then `uci.py` (44 code lines). Run `uci.py` and talk to it by hand, as in the "Try it" part of `uci.md`. |
| 3:50 to 4:05 | Run `verify_remus.py` in `engines/Remus1` and fix any file it names until it reports that Remus1 passes every test. |
| 4:05 to 4:30 | One round of the student pair against their Remus1, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). Every move it makes is legal, and nearly every move is bad. |

**Checkpoint:** Remus1 is complete, 193 code lines, and it passes every test in `verify_remus.py`.

## Day 2: Remus2 searches ahead

**Goal for the day:** the student pair turns Remus1 into Remus2, and Remus2 plays
Remus1 and the student pair in Lucas Chess.

### P1, 8:30 to 10:00: The evaluation

| Time | Activity |
| --- | --- |
| 8:30 to 8:35 | In File Explorer, copy every file of `engines/Remus1` into `engines/Remus2`. The rest of Day 2 edits those copies. |
| 8:35 to 8:45 | A score in centipawns: material, plus a bonus or penalty for where each piece stands. A Black piece reads each table upside down. |
| 8:45 to 9:55 | Type `evaluation.py` (68 code lines): `mirrored_table()`, the four piece-square tables (files a to d only), the piece values, and `evaluate()`. Run it. |
| 9:55 to 10:00 | Swap driver and navigator. |

### P2, 10:30 to 12:00: Move ordering and quiescence

| Time | Activity |
| --- | --- |
| 10:30 to 10:50 | Alpha-beta on paper: look ahead, assume both sides play their best, and skip the moves that cannot change the result. MVV-LVA: try pawn takes queen before queen takes pawn. |
| 10:50 to 11:55 | In `engines/Remus2`, keep the three import lines of Remus1's `search.py`, change `import random` to `import time`, delete Remus1's random-move `think()`, and type the rest of the top of Remus2's `search.py` (about 50 code lines): the imports and constants, `ordered_moves()`, `count_node()`, and `quiescence()`. |
| 11:55 to 12:00 | Swap driver and navigator. |

### P3, 1:00 to 2:30: Alpha-beta and iterative deepening

| Time | Activity |
| --- | --- |
| 1:00 to 1:10 | Reminder of checkmate and stalemate: a side with no legal move has lost if in check, and draws if not. |
| 1:10 to 2:10 | Type `alpha_beta()` and `think()` (about 43 code lines), which finish `search.py`. Change the name line in `uci.py` to `id name Remus2`. |
| 2:10 to 2:30 | Run `uci.py` and send `position startpos` and `go movetime 2000`: the `info` lines show Remus2 searching one move deeper at a time. Then run `verify_remus.py` in `engines/Remus2` and fix any file it names until Remus2 passes every test. |

### P4, 3:00 to 4:30: Remus2 against Remus1 and the student pair

| Time | Activity |
| --- | --- |
| 3:00 to 3:25 | Start one round of their Remus2 against their Remus1, two 10-minute games played at once in two Lucas workers (Part 4 of the guide), to see what the search buys over a random choice. Each student pair reads its results tab. |
| 3:25 to 4:05 | One round of the student pair against their Remus2, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |
| 4:05 to 4:30, if time remains | One round of their Remus2 against Stockfish 1500, two 10-minute games played at once in two Lucas workers. |

**Checkpoint:** Remus2 is complete, 342 code lines, and it passes every test in `verify_remus.py`.

## Day 3: Remus3

**Goal for the day:** the student pair turns its Remus2 into Remus3 by typing the 193
new lines, then tests it in P4. Each period ends with an engine that plays, because
the lines of one period never call a function of a later period.

### P1, 8:30 to 10:00: Pawn structure, the bishop pair, and the rook table

| Time | Activity |
| --- | --- |
| 8:30 to 8:35 | In File Explorer, copy every file of `engines/Remus2` into `engines/Remus3`. |
| 8:35 to 8:45 | Doubled, isolated, and passed pawns, on a position the teacher shows in Lucas Chess. Why two bishops together are worth more than a bishop and a knight. |
| 8:45 to 9:00 | Move `MATE_SCORE` from `search.py` to `board.py`, and add `position_key()` and `move_text()` (8 code lines in `board.py`). |
| 9:00 to 9:55 | In `evaluation.py`, add the rook table, `pawn_terms()`, the bishop pair, and the lines of `evaluate()` that collect the pawns (47 code lines). Run `uci.py`. |
| 9:55 to 10:00 | Swap driver and navigator. |

### P2, 10:30 to 12:00: The table, killer moves, and the null move

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | Why move order matters to alpha-beta, and why the same position appears by many move orders: e4 e5 Nf3 and Nf3 e5 e4 reach one position. The null move: let the opponent move twice; if the side to move still stands above beta, cut off. |
| 10:45 to 11:00 | Type `transposition_table.py` (22 code lines): `store()` and `probe()`. |
| 11:00 to 11:55 | In `search.py`, add the killer moves and the table probe and store, then `has_pieces()` and null-move pruning in `alpha_beta()` (about 52 code lines). Run `uci.py`. |
| 11:55 to 12:00 | Swap driver and navigator. |

### P3, 1:00 to 2:30: The opening book and the clock

| Time | Activity |
| --- | --- |
| 1:00 to 1:10 | An opening book is a list of good first moves in UCI text. Copy `opening_book.txt` from `engines_instructor/Remus3` into `engines/Remus3` in File Explorer: it is data, not code, so nobody types it. |
| 1:10 to 1:35 | Type `opening_book.py` (21 code lines): `load_book()` and `book_moves()`. |
| 1:35 to 2:25 | In `think()`, add the aspiration window and the time that grows when the best move changes; in `uci.py`, add `legal_book_move()`, the game's moves, and the hard time limit (about 30 code lines). |
| 2:25 to 2:30 | Change the name line in `uci.py` to `id name Remus3`. |

### P4, 3:00 to 4:30: The principal variation, then test Remus3

| Time | Activity |
| --- | --- |
| 3:00 to 3:20 | Add `principal_variation()` and the `pv` of the `info` line (about 13 code lines). |
| 3:20 to 4:30 | Run `verify_remus.py` in `engines/Remus3` until Remus3 passes every test. Then play one round of their Remus3 against their Remus2, two 10-minute games played at once in two Lucas workers. Once a game leaves the book, Lucas Chess shows each engine's depth and the line it expects beside its score: Remus3 searches deeper than Remus2 in the same time. Each student pair finds one game where Remus3 played a book move. |
| If time remains | One round of the student pair against their Remus3, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |
| If more time remains | One round of their Remus3 against Stockfish 1500, two 10-minute games played at once in two Lucas workers. A student pair that also played Stockfish 1500 on Day 2 compares the results. |

Play stops at 4:30, the end of the period, even in the middle of a game.

**Checkpoint:** Remus3 is complete, 535 code lines, and it passes every test in `verify_remus.py`.

## Day 4: Remus4

**Goal for the day:** the student pair types the 159 new lines of Remus4, then tests
it in P4.

### P1, 8:30 to 10:00: Draw sense and searching less

| Time | Activity |
| --- | --- |
| 8:30 to 8:35 | In File Explorer, copy every file of `engines/Remus3` into `engines/Remus4`. |
| 8:35 to 8:45 | Reminder of threefold repetition and the fifty-move rule. Late move reductions, principal variation search, and futility pruning: three ways to spend less time on moves that are probably bad. |
| 8:45 to 9:55 | The halfmove clock in `board.py` and `make_move.py`, the game history in `uci.py`, then `is_draw()`, the history list, futility pruning, late move reductions, and PVS in `search.py` (about 36 code lines). Run `verify_remus.py` in `engines/Remus4` until Remus4 after Period 1 passes every test. |
| 9:55 to 10:00 | Swap driver and navigator. |

### P2, 10:30 to 12:00: Static exchange evaluation

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | A capture is good only if the whole exchange on its square wins material: play it out on paper, cheapest attacker first. |
| 10:45 to 11:50 | Type `exchange.py` (36 code lines): `cheapest_attacker()` and `exchange_score()`. In `quiescence()`, add delta pruning and skip the captures that lose material (about 8 code lines). |
| 11:50 to 12:00 | Run `verify_remus.py` in `engines/Remus4` until Remus4 after Period 2 passes every test. |

### P3, 1:00 to 2:30: The endgame evaluation

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | Reminder of insufficient material. In the endgame the king comes to the center, and the side that is ahead drives the lone king to the edge. |
| 1:15 to 2:25 | In `evaluation.py`, add the endgame king table and the taper that blends the two king tables, mobility, `mating_bonus()`, and insufficient material (about 45 code lines). Run `verify_remus.py` in `engines/Remus4` until Remus4 after Period 3 passes every test. |
| 2:25 to 2:30 | Swap driver and navigator. |

### P4, 3:00 to 4:30: The pawn terms, then test Remus4

| Time | Activity |
| --- | --- |
| 3:00 to 3:30 | Reminder of the rule of the square. In `pawn_terms()`, add the pawn race, rooks on open files, and `king_safety()` (about 35 code lines). Change the name line to `id name Remus4`. |
| 3:30 to 4:00 | Run `verify_remus.py` in `engines/Remus4` until Remus4 passes every test. Then play one round of their Remus4 against their Remus3, two 10-minute games played at once in two Lucas workers. |
| 4:00 to 4:30 | One round of their Remus4 against Remus4-Rust, two 10-minute games played at once in two Lucas workers. Remus4-Rust is the instructor's Remus4 written in Rust, with the same evaluation and search and a larger table, so speed and memory alone separate the two engines. |
| If time remains | One round of the student pair against their Remus4, two 10-minute games: each student plays one game, one with White and one with Black (Part 3 of the guide). |

Play stops at 4:30, the end of the period, even in the middle of a game.

**Checkpoint:** Remus4 is complete, 694 code lines, and it passes every test in `verify_remus.py`.

## Day 5: The science project

**Goal for the day:** the student pairs aim the engine's ideas at two problems from a
synchrotron beamline, report what their science AI found, and take their code home.

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

### P1, 8:30 to 10:00: Where to measure next

| Time | Activity |
| --- | --- |
| 8:30 to 8:45 | A 10 by 10 sample hides a hot spot, and every measurement costs beam time. Choosing the next square is choosing a move: list the candidates, score each one, keep the best. |
| 8:45 to 9:30 | Write `candidate_squares()`, `score_candidate()`, and `choose_next_measurement()` in `hidden_peak.py` (19 code lines). Run it. |
| 9:30 to 10:00 | Tune `EXPLORE_WEIGHT` and compare. Over 100 hidden samples, random guessing needs 54.3 measurements on average, the AI with weight 0 needs 17.5, weight 8 needs 9.4, and weight 40 needs 19.4. The teacher shows the expectimax lookahead (24.7): it looks two measurements ahead, but its crude model makes it worse than the one-step AI. |

### P2, 10:30 to 12:00: Name the crystals, the greedy way

| Time | Activity |
| --- | --- |
| 10:30 to 10:45 | The PDF beamline at Brookhaven's NSLS-II and its daily question: which crystal phases are in this powder? Each phase makes its own fingerprint of X-ray peaks. |
| 10:45 to 11:15 | Write `score()` in `phase_hunt.py` (11 code lines): a point for each explained peak, and a point lost for each left-over peak, missing peak, and phase used. |
| 11:15 to 12:00 | Write `greedy_phases()` (15 code lines), which adds the best phase again and again. On 200 known powders greedy names 166 right and 34 wrong, and on powder 12 it mistakes silver for gold. |

### P3, 1:00 to 2:30: Search every set, and verify

| Time | Activity |
| --- | --- |
| 1:00 to 1:15 | The tree of phase sets beside Remus's game tree: a set is a position and adding a phase is a move. Quartz then rutile is the same set as rutile then quartz, the same idea as the transposition table. |
| 1:15 to 1:50 | Write `search()` (12 code lines), which scores every set of up to three phases and keeps every set that ties for the best score. |
| 1:50 to 2:15 | Verify on the 200 known powders: 139 right, 52 "can't tell", and 9 wrong, against greedy's 34 wrong. The search scores 176 sets per powder in library order, against 821 in every order. Change `PHASE_COST` and `MISSING` and watch the verdicts move. |
| 2:15 to 2:30 | The programs that search the same tree on real synchrotron data, and the agent that steers measurements at a beamline. |

### P4, 3:00 to 4:30: Science results and take-home

| Time | Activity |
| --- | --- |
| 3:00 to 3:10 | Each student copies the Remus folder to a USB drive in File Explorer, leaving out `.venv`, and checks that the copy opens in VS Code. |
| 3:10 to 3:30 | Each student pair runs both programs once more and fills in its results: measurements against random guessing, and right, wrong, and can't-tell counts for greedy and for search, with the `PHASE_COST` and `MISSING` the student pair picked in Period 3. The class collects every student pair's numbers and its two values on the board. |
| 3:30 to 3:45 | Each student pair compares two powders. The search names powder 0 right: silicon and zincite. Powder 12 holds anatase and silver, and the search answers "can't tell": anatase with gold, or anatase with silver. Every peak of gold lands less than 0.3 degrees from a peak of silver, inside `TOLERANCE`, and 50 of the search's 52 "can't tell" answers are this same tie. No X-ray peak in the library settles powder 12, so the student pair works out what other test would. |
| 3:45 to 4:20 | Each student pair presents its results, its two values, and its answer for powder 12 in three minutes. |
| 4:20 to 4:30 | What comes next: AI agents that steer real experiments at light sources such as NSLS-II. |

**Checkpoint:** every student pair's `choose_next_measurement()` beats random guessing,
every student pair's `search()` makes fewer wrong calls than greedy, and every student
leaves with the code on a USB drive.
