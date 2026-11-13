# Remus Project Rules and Python Style Guide

Remus is a chess engine written in pure, native Python as a teaching project for the
**"Turning a Chess Engine into Science AI"** workshop. The workshop has four engines in
the top `engines_instructor/` folder: Remus1, a random legal mover that ends Day 1, and Remus2,
Remus3, and Remus4;
[`docs/engines/remus_versions.md`](docs/engines/remus_versions.md) lists what each
one adds, and [`workshop_syllabus.md`](workshop_syllabus.md)
holds the plan of the five days. Remus5 and Remus5-Rust are single-threaded
experimental engines with positional evaluation, optional Lichess tablebase advice,
and UCI pondering. Remus6-Rust (Lazy SMP) and Remus7-Rust (YBWC) are Rust-only
parallel extensions outside the five-day workshop. Both default to four search threads.

This file holds every project and style rule, so that every AI tool used with this
repository builds Remus the same way. `CLAUDE.md` imports it and adds only the rules
that are specific to Claude Code. The Python style follows David's course repositories
(qis101, scicomp101, cse101), adapted for a chess engine and for students meeting its
ideas for the first time.

## Part 1: The project

### Why the name "Remus"

- **Sargon (1978)** was the landmark microcomputer chess program by Dan and Kathe
  Spracklen. Written in Z80 assembly for the Wavemate Jupiter II, it won the first
  microcomputer chess tournament (West Coast Computer Faire, 1978) with a perfect 5-0.
  Its complete source was published as a book, *Sargon: A Computer Chess Program*
  (Hayden, 1978), so people could read and learn from a real engine. It spawned Sargon II,
  2.5, III (ported to the first Macintosh), and later commercial versions.
- **Sargon of Akkad** (c. 2335 to 2279 BC, middle chronology) founded the Akkadian Empire,
  the first known great empire. He was succeeded by his sons **Rimush** (c. 2279 to 2270
  BC), who put down widespread revolts and was possibly assassinated by courtiers, and then
  **Manishtushu** (c. 2270 to 2255 BC), who campaigned south along the Tigris and into the
  Persian Gulf. Manishtushu's son Naram-Sin brought the empire to its height.
- **Remus** echoes **Rimush**, the first son and heir of Sargon. The engine is meant to be
  a "son of Sargon": an heir to that tradition of small, readable chess programs whose
  internals are laid open for learners, rebuilt in modern Python.
- The name leaves room for the family line: a later, stronger engine could take the name
  of the next heir (Manishtushu) or grandson (Naram-Sin).

### Project goals

- Build a legal chess engine that plays well, in code a student can read line by line.
- A working game comes first; stronger play comes later.
- Success means a learner can explain a move, say why a search result is legal and
  preferred, and say what each enhancement of Remus3 and Remus4 buys.

### Audience and workshop format

- Remus is an instructional engine for a **one-week, five-day workshop** taught to
  **students who know some coding and know how to play chess, including notation**
  (David, 2026-11-04): four 90-minute periods a day (8:30 to 10:00, 10:30 to 12:00,
  1:00 to 2:30, 3:00 to 4:30). Many students have learned Java, close enough to type
  Python and follow the ideas. Each engine carries the number of the day that
  builds it (David, 2026-11-03): Day 1 types in Remus1, Day 2 Remus2, Day 3 Remus3,
  Day 4 Remus4, and Day 5 is the science project.
- **The workshop is open to students of all ages** (David, 2026-11-04). Every
  document, comment, and brief names the audience with the plain word "students":
  no age, grade, school level, or age group.
- 20 students work in **10 student pairs** (driver and navigator, swapping every
  period). Each student pair gets an identical, preconfigured **Windows 11** class
  laptop with VS Code and Lucas Chess. Remus targets Windows only (David, 2026-10-28):
  the repository holds no Mac or Linux launchers or setup scripts.
- **Students type the engines in** (David, 2026-11-03). On Days 1 and 2 each
  student pair types in Remus1 and then Remus2, copying the instructor's version
  systematically, file by file. Later the student pairs type the new lines of Remus3
  and then of Remus4 the same way. The workshop has no Python skills labs and no
  unplugged (non-coding) activities.
- **Always minimize the code students type** (David, 2026-11-03). The engines hold
  no test, check, perft, or print code beyond what UCI needs: they "assume it works",
  and the workshop designer's `admin/verify_engines.py` checks them from
  outside. Prefer the shorter of two equally clear forms.
- Students know how the pieces move. Lessons remind them of the rarer rules right
  before the code that needs each one: castling conditions, en passant, promotion,
  stalemate, threefold repetition, the fifty-move rule, and insufficient material.
- The teacher writes no code during the workshop (David, 2026-10-31): every line a
  student pair does not type is already in the repository.
- Write code, comments, and lesson text for a student reading the idea for the first
  time: short functions, concrete names, and one small example position per idea.
- The class never builds a GUI. Every laptop uses **Lucas Chess** (David,
  2026-10-27), which runs Remus as a UCI engine through `remus.bat`.
  [`docs/lucas_chess/lucas_chess_guide.md`](docs/lucas_chess/lucas_chess_guide.md) shows
  students, step by step with screenshots, how to find the engines, play a game against
  Remus1, and run one-round tournaments between the versions.
  `docs/lucas_chess/install_lucas_engines.py` adds the instructor's Remus1 to Remus4
  and Remus4-Rust to the list of external engines, from `engines_instructor/`
  (David, 2026-11-04), without touching any other entry or adding a duplicate, so no
  student or teacher adds an engine by hand. Lucas Chess plays only the instructor's
  engines, and `verify_remus.py` tests what a student pair types. The guide's pictures
  live in `docs/lucas_chess/jpg/`.
- **No GitHub for students.** Students create no online accounts: cloning the public
  repository needs none.
- **git and `uv` only at setup** (David, 2026-10-29). On Day 1, period 1, the teacher
  walks each student through installing git and `uv`, cloning the repository, and
  running `uv sync` once to build `.venv`. After that setup no student or teacher uses
  git or `uv` at any point in the workshop: no fetch, branch, commit, restore, or
  clone, and no `uv run`. Code never moves between laptops through git: on Day 5
  each student copies the Remus folder, without `.venv`, to a USB drive in File
  Explorer to take home. Never plan a step that needs a student account, pushes
  student code online, or asks a student or teacher to type a git or `uv` command
  after setup.
- **Three people use the repository** (David, 2026-10-29), and every document names
  the right one:
  - **Students**, the student pairs, who type in Remus1, Remus2, Remus3, and Remus4.
  - **Teachers**, who run the class. A teacher does everything in VS Code: open a
    file and press the Run Python File button, or open a Markdown page. No step of the
    five-day workshop asks a student or a teacher to type a command.
  - **The workshop designer** (David), who builds the workshop. The tools in
    `admin/` are designer tools, and only they may need `uv run` in a
    terminal.
- Documents describe the workshop as it stands, with no dates of past events ("the
  league finished on 2026-10-28", "a survey from October 2026"), the same rule as
  for code comments. The David dates in this file record when a rule was set and
  stay here only.

### Hard rules for the engine code

1. **Clarity over speed.** Plain lists, tuples, dicts, and functions. Slow is fine,
   but the engine copies a position instead of undoing a move because the copy is both
   shorter and faster.
2. **No third-party libraries inside the Python engine.** Standard library only.
   No bitboards, magic numbers, compiled extensions, or framework machinery.
   Remus5-Rust, Remus6-Rust, and Remus7-Rust may use `reqwest`, `serde_json`, and `httpdate` only for HTTPS, JSON,
   and HTTP retry dates at the tablebase boundary. Their native chess code uses no
   external chess library.
3. **Engine is the single source of truth** for board, rules, legal moves, search,
   evaluation, and legal move choice. A GUI only sends moves and displays results.
   Remus5, Remus5-Rust, Remus6-Rust, and Remus7-Rust may use Lichess advice to choose a root move, after
   validating that move with the native generator and king-safety check. Their
   fallback search and evaluation remain those of Remus5 and Remus5-Rust.
4. **`python-chess` and Lucas Chess are boundary tools only.** Lucas Chess is the
   desktop GUI, which runs Remus over UCI. `python-chess` serves as an external
   cross-check in `verify_engines.py`, never as the engine's hidden implementation.
5. **Copy-make.** `make_move()` copies the board and returns a new position; no code
   changes a board in place, so the search never takes a move back and the live
   position stays unchanged even when the search stops early.
6. **Each version is the one before plus its enhancements.** Remus3 is Remus2 plus the
   steps that `remus_versions.md` gives Remus3, and Remus4 is Remus3 plus every other
   step. Keep the shared code identical from version to version, so a student pair
   types only the new lines. A change to a shared line goes into every version that
   holds it. Remus1 is one exception: Remus2 replaces its 14-line `search.py`.
   Remus5 and the Rust engines 5 through 7 add positional evaluation for backward pawns, central holes,
   knight outposts, and nearby king cover; keep these additions identical across these
   implementations. Remus1 through Remus4 retain their
   workshop evaluation. Another explicit exception is table capacity: Remus4-Rust, Remus5, and
   Remus5-Rust use fixed 512 MiB tables. Remus6-Rust and Remus7-Rust
   expose Hash with a 512 MiB default and 1024/2048 MiB experimental sizes. Remus3
   and Remus4 use two-million-entry Python dictionaries. Remus1 and Remus2 have no transposition table.

7. **Rules before search.** Move generation must match the published perft counts
   before any search change is trusted.
8. The opening book is a boundary feature. Validate every book move with the native
   generator; an absent book must never stop play. Remus opening choices come only from
   that engine's supplied `opening_book.txt`: Python reads it and Rust embeds it
   at build time. Disable Lucas Chess external books for every Remus entry,
   including Remus5, Remus5-Rust, Remus6-Rust, and Remus7-Rust. Tournament engine entries use
   explicit `<None>`, not `<By default>`, so they cannot inherit a GUI book.
   When no legal book continuation exists, use native move selection. Remus1
   and Remus2 have no opening book and keep their native behavior.
9. Remus5 and Rust engines 5 through 7 tablebase requests belong only at the root, never inside native search.
   Query at most seven pieces, including kings, with no castling rights. Preserve
   the actual halfmove clock and en passant square in the API FEN. Keep repetition
   checks native: FEN does not carry game history. Unknown or uncertain results,
   invalid moves, malformed replies, failures, and timeouts select native search.
   Make at most one request at a time per engine process, cache by complete API FEN,
   and wait at least 60 seconds after HTTP 429, honoring a longer Retry-After.
   Bound the lookup wait to one second and one quarter of the move budget, subtract
   elapsed lookup time from the search budget, and offer the `LichessTablebase` UCI
   option (default true). Fixed-depth Rust checks bypass the service.

10. Remus5 uses a fixed 512 MiB allocation for search caches, in
    both Python and Rust. Four 16-byte entries
    occupy each 64-byte bucket. Multiply-shift indexing selects the bucket;
    a full bucket replaces the entry with the lowest depth minus age penalty.
    Python uses standard-library `mmap` and `struct` only inside the table module;
    moves remain tuples everywhere else. Rust uses Windows virtual-memory
    allocation and release. Both allocate zero-filled memory at startup and
    proceed without caching if allocation fails. There is no UCI Hash option,
    RAM query, dynamic resizing, or smaller-capacity retry. Each process owns
    its own allocation. Remus4-Rust also has a fixed 512 MiB table without Hash; it retains its
    native allocator, low-bit bucket indexing, and depth/age replacement.
    Remus6-Rust and Remus7-Rust share one configurable table across their workers.

11. Remus5, Remus5-Rust, Remus6-Rust, and Remus7-Rust support standard UCI pondering with a `Ponder` check
    option, default false. Return a legal predicted reply with `bestmove` when
    enabled, but search on the opponent's time only after `go ponder`. Keep
    reading commands during search. `ponderhit` starts the move clock without
    restarting the search; completed ponder work counts toward the soft search
    target. A hit reserves a quarter of the adjusted plan on the engine clock,
    capped at a quarter of the hard budget; hard deadlines and cancellation take
    precedence. A matching explicit stop-and-go within two seconds earns credit
    after recovering the prior completed depth. Match complete position-command
    tokens and history; new-game and option commands clear pending credit. The hard
    deadline starts at the hit or matching go. `stop` returns one legal result. Finished ponder
    searches wait for hit or stop, including mate positions. Pondering uses native
    search and makes no tablebase requests. Lucas Chess R6 controls pondering;
    its external-engine and tournament entries must enable `Ponder`.

    All four engines preserve legal book continuations during both hit forms. A book
    ponder request waits for hit or stop without native search and earns no search
    credit. With at least 60 seconds remaining, an uncertain native hit reserves
    half the adjusted plan and tries for one further completed depth, bounded by
    one adjusted plan and half the hard budget. Uncertainty means hit depth below
    8, a score drop of at least 35 centipawns between mature iterations (depth 4+),
    or a changed best move in the latest mature iteration. Low clocks and explicit
    movetime retain the ordinary hit floor. Preserve these confidence signals
    across Lucas stop/go restarts and record the allowance in ponder diagnostics.

Diagnostic output is opt-in. `SearchDiagnostics` and `PonderDiagnostics` default
to false wherever offered, in both advertised UCI options and runtime state. A GUI
can explicitly enable them for a tournament; saved GUI values override defaults.
Normal UCI iteration and move output is independent of these diagnostic options.
Rust diagnostic files live in `logs/` beside the executable, with names
`search_<pid>_<session>.txt` and, where pondering is supported,
`ponder_<pid>_<session>.txt`. Both streams share one per-process session timestamp;
concurrent processes never append to a shared diagnostic file. Preserve old logs.

### Representation conventions

- Board: a list of 64 numbers, one per square. A piece is its kind, from 1 for a pawn
  to 6 for a king, plus 8 if it is White (`WHITE = 8`), so `piece & 7` gives the kind
  and 0 is an empty square.
- **Coordinates: row 0 is rank 8, column 0 is file `a`,** and square number
  `row * 8 + col` names a square: a8 is 0, h8 is 7, a1 is 56, and h1 is 63. Repeat
  this in comments.
- Position: the tuple `(board, white, castling, en_passant)`, plus `halfmove` in
  Remus4. `white` is True when White is to move, the castling rights are four bits
  (1 and 2 for White's kingside and queenside, 4 and 8 for Black's), and
  `en_passant` is the square a pawn just skipped over, or -1.
- Moves: the tuple `(from_square, to_square, promotion)`, with promotion 0 or a piece
  kind.
- The engines read and write only UCI moves (`e2e4`, `e7e8q`), and only at the UCI
  boundary. They hold no SAN. Workshop Python engines hold no FEN: Lucas Chess
  sends `position startpos moves ...` for every game. Remus5 permits outbound FEN
  serialization only in `tablebase.py`, at the HTTP API boundary. Native board,
  move generation, evaluation, and search use the existing position tuple. Rust
  retains its `position fen` UCI input for designer checks; `tablebase.rs` serializes
  outbound API FEN. The API fullmove number is 1 because it does not affect a probe.

### Architecture boundaries

| Part | Responsibility |
| --- | --- |
| Native Python engine | Board, rules, moves, search, evaluation, move choice |
| UCI adapter, `uci.py` | `uci`, `isready`, `position startpos moves ...`, `go`, `quit`; only protocol lines on stdout; `bestmove 0000` only with no legal move |
| Tablebase adapter, Remus5 and Rust 5 through 7 | Root advice from Lichess over HTTPS; FEN/JSON boundary, cache, cooldown, bounded wait, native legality validation, and search fallback |
| GUI | Lucas Chess, which starts `remus.bat` and hosts Remus as an external UCI engine |
| Checks | `admin/verify_engines.py`, outside the engines |

The engine must run without Lucas Chess or `python-chess` installed. Tablebase-enabled engines must
also play without internet access or a successful tablebase response.

### Verification

- Write expected results before running.
- `uv run admin/verify_engines.py` must pass after any engine change: perft
  from six published positions, ten mate puzzles, fixed-depth searches, and a short
  UCI conversation. Compare the fixed-depth node counts and moves before and after a
  change; a change that should not alter the search must leave them identical.
- For Remus5, also run `uv run admin/verify_tablebase.py` for shared Python/Rust
  HTTP fixtures, `uv run admin/verify_engines.py Remus5`, and
  `uv run admin/verify_ponder_control.py`, `verify_pondering.py`, and
  `verify_lucas_pondering.py` for standard and Lucas pondering. After a change to
  the ponder clock or the book rule, run `uv run admin/verify_ponder_reinvestment.py`
  for Python and `uv run admin/verify_rust_ponder_reinvestment.py` for the Rust
  engines 5 through 7. `--live` on the
  tablebase check adds sequential live HTTPS probes. Build with
  `uv run admin/build_rust.py --engine 5` and run `uv run admin/verify_rust.py --engine 5`.
- After a Remus5 table change, run `uv run admin/verify_transposition_tables.py`,
  rebuild Remus5-Rust, and run `uv run admin/verify_cache_uci.py`.
  Compare fixed-depth nodes, scores, and moves with the baseline; investigate
  differences because bucket indexing and replacement can affect collisions.
  Run large-allocation checks and Rust builds sequentially.
- Remus6-Rust and Remus7-Rust default to four search threads and 512 MiB shared tables.
  Use one search thread and a preserved parallel executable as the search baseline;
   their cache replacement can differ from Remus5. Compare native rules and
   evaluation with Remus5. Test parallel metrics,
  cancellation, pondering, and legal results separately; parallel node counts can
  vary with scheduling. Build separate candidates and verify before installing.
- Start no engine games or matches (any time control) without David's approval
  (David, 2026-11-03). Verify with perft and fixed-depth comparisons instead.

### Project setup

- Remus is a `uv` project on Python 3.14 (`.python-version`, `pyproject.toml`).
  Run `uv sync` once, then `uv run ruff check .`.
- The one runtime dependency is `matplotlib`, for the graphs of the Day 5 science
  programs (David, 2026-11-04); no engine imports it. The `dev` dependency group holds
  `python-chess` (imported as `chess`) and `ruff`. Engine modules never import
  `chess`.
- Lucas Chess is a desktop application, not a Python dependency. It hosts Remus as an
  external UCI engine and comes preinstalled on the class laptops.
- The top `engines_instructor/` folder holds one folder per finished engine,
  `engines_instructor/Remus1` to `engines_instructor/Remus4`, the versions the
  student pairs copy from. Each holds only the files the engine runs
  (David, 2026-11-03): its Python files and `remus.bat` (the one line `uv run uci.py`;
  Lucas Chess starts it in its own folder). Remus3 and Remus4 also hold
  `opening_book.txt`. Each also holds a copy of `verify_remus.py`, so the workshop
  designer can run the student pairs' test on the instructor's engine;
  `make_mermaid_call_trees.py` skips it and the tool that strips the engines copies it unchanged. Each engine's `README.md` and Mermaid call diagrams live in
  `docs/engines/Remus1` to `docs/engines/Remus4`.
  Each engine names itself in `uci.py` (`id name Remus2`). Modules import each other
  by name (`from board import START_POSITION`), so any module runs from inside its
  folder with the Run button, and nothing in an engine folder imports from another
  folder.
- `engines_instructor/Remus4-Rust` holds Remus4 rewritten in Rust (David, 2026-11-04),
  so the class sees that speed matters, not just the algorithms. Students neither type
  nor read it, and it lives only in this folder, with no copy in `engines/`. It has a
  real `remus4_rust.exe`, which git keeps, so a laptop needs no Rust
  installed. Each `.rs` file matches the Remus4 Python file of the same name, and
  `main.rs` matches `uci.py`. Its one difference in play is a larger table (David,
  2026-11-04): 512 MB of four-entry buckets that replace their least useful entry
  instead of starting over. Until a bucket fills, it searches the same tree as Remus4.
  A change to Remus4's rules, evaluation, search, or book needs the same change here,
  then `uv run admin/build_rust.py` and a clean `uv run admin/verify_rust.py`. The rules of Part 2 and
  the bans on bitboards and extra print code apply to the Python engines only: this
  engine uses bitboards, follows `cargo fmt` and `cargo clippy`, and takes the extra
  commands `position fen`, `go depth`, `go perft`, and `eval` for that check.
- Experimental `engines_instructor/Remus5` contains the Python engine, launcher,
  and opening book, with documentation in `docs/engines/Remus5`.
  `Remus5-Rust` implements the same single-threaded native search, evaluation,
  fixed table, tablebase boundary, and pondering. Boundary changes must preserve
  Python/Rust behavior and native fixed-depth results.
  `Remus6-Rust` adds Lazy SMP and `Remus7-Rust` adds YBWC with restricted worker
  helping. They are Rust-only and the only engines with parallel chess search.
  Each Rust folder contains source, locked dependencies, an embedded book, and
  its tracked `remusN_rust.exe`. Documentation lives in the matching folder under
  `docs/engines/`. These experimental engines have no copies in `engines/`.
  The workshop installer registers only Remus1 through Remus4 and Remus4-Rust;
  experimental READMEs give their external UCI paths.
- The top `engines/` folder holds the student pair's own engines (David,
  2026-11-03): four folders, `engines/Remus1` to `engines/Remus4`, each holding only
  `remus.bat` and `verify_remus.py` in the repository. `verify_remus.py` is the
  same file in all four: run with the Run button, it tests the student pair's
  finished version (files, perft, evaluation, a depth-4 search, UCI) against the
  instructor's numbers, which it holds. In `engines/Remus4` it also tests Remus4 as
  it stands at the end of Day 4, Periods 1 to 3, told apart by `exchange.py`,
  `mating_bonus()`, and `king_safety()`, with numbers measured on the period states
  that `admin/make_code_images.py` rebuilds. Update those numbers after any engine
  change that moves them. On Day 1 the student pair creates every
  file of Remus1 in `engines/Remus1`. At the start of Days 2, 3, and 4 it copies every
  file of the previous version's folder into the next one in File Explorer and edits
  those copies. Never put instructor code in `engines/`.
- The code a student pair types is `engines_instructor/` stripped by a tool that
  `admin/README.md` lists: no shebang, docstrings, comments, or type hints, only the
  `# fmt: skip` comments that keep the board tables in shape when VS Code formats on
  save. Never edit the stripped code by hand. It stays on David's computer only, so
  the student pairs cannot copy it from the public repository.
- Each engine README holds a table of its files with their code lines (blank lines,
  comments, and docstrings left out), the call tree picture, and what the version
  adds. `uv run admin/make_mermaid_call_trees.py --render` draws Mermaid SVGs
  from the code and updates the call-tree section in each engine README; run it after any change to which functions exist or which
  call which. Update the line counts in the READMEs and `remus_versions.md` after any
  code change. Each engine has one README in its documentation folder; its
  `mermaid/` subfolder holds diagram sources, SVGs, and a manifest only.
- A module keeps one file name in every version. When a later version makes a file
  smarter, the new code goes into the same file. Never add version numbers to file
  names.
- The top folder holds everything that is not an engine folder (David, 2026-11-04):
  - The plan of the workshop: `workshop_syllabus.md`, the whole week on one page,
    and one top folder per day, such as `Day 1 - Remus1 plays legal chess`, with
    `Day_1_Syllabus.md` and a folder per period (`period1` to `period4`) that
    holds `Day_1_Period_1_Syllabus.md`. The day and period pages follow
    `workshop_syllabus.md`; a change to one goes into the others. Each period
    folder with typed code also holds `images/` (David, 2026-11-04): pictures of
    that code from VS Code, so a student pair cannot paste it, written by
    `admin/make_code_images.py`, and the Day 5 graphs from
    `admin/make_science_figures.py`. Run both after a change to the code they show.
  - `science/`, the two Day 5 programs, `hidden_peak.py` and `phase_hunt.py`, with
    the functions the student pairs write left empty, and `science_instructor/`,
    the finished versions (David, 2026-11-04). This mirrors `engines/` and
    `engines_instructor/`. They follow the engine style (no shebang, copy-make), import
    no engine, and show their graphs with matplotlib: the student pair closes each
    window and the program continues. The student pair types none of the graph code.
  - `docs/`, the pages people read:
    - `docs/engines/`: `Remus1` to `Remus4`, each with its `README.md` and
      Mermaid call diagrams, and `remus_versions.md`.
    - `docs/lucas_chess/`: `lucas_chess_guide.md` and its pictures.
    - `docs/chess_tutor/`: `learn_chess.md` (the rarer rules and chess
      notation, with its board pictures in `images/`), `uci.md`, the minimax,
      negamax, and alpha-beta videos page, and `fide_titles_and_elo_ratings.md`.
    - `docs/briefs/`: the one-page management brief and the one-page student
      and parent brief, as PDFs.
  - `admin/`: the workshop designer's scripts, with a `README.md`. Every workshop
    designer tool lives here, finds the top Remus folder from its own path, and
    runs from the top folder with `uv run admin/<name>.py`. List every script in
    `admin/README.md` with what it does (David, 2026-10-28). Git ignores
    `admin/` (David, 2026-11-04): it stays on David's computer only, and no
    public page links to it.
- David keeps a snapshot of the workshop before the Remus engines in
  `.archive/` at the top of his Remus folder (David, 2026-11-03). Git ignores it.
  Never edit or delete it.
- **Say "student pairs"** (David, 2026-10-29). The students work in pairs, and every
  document and comment names them "student pairs" or "the student pair", never a bare
  "pairs". A pair of games or a `(row, col)` pair keeps its plain word.

## Part 2: Python style

### General principles

- Code should be **clear, explicit, and readable**.
- Prefer **teaching-oriented explanations** over compact or clever code.
- Write as if the reader is a **student learning the concept for the first time**.
- Avoid unnecessary abstraction unless it improves understanding.

### Script and module structure

Every engine file starts with a docstring whose first line is the filename. A longer
docstring adds a blank line and a short description of what the file teaches or does.
Engine files carry no shebang (David, 2026-11-03): students and teachers run them with
VS Code's Run button, which uses the project's `.venv` that the workspace file selects.

```python
"""search.py

Pick a move by looking ahead with alpha-beta search.
"""
```

A workshop designer script in `admin/` starts with the shebang line
`#!/usr/bin/env -S uv run` above its docstring, so on Linux and macOS a script marked
executable runs with the project's environment and no manual activation.

The parts of a file come in this order:

1. Filename docstring (a designer script puts its shebang above it).
2. Imports, standard library first, then the project's own modules, each group sorted
   (ruff's `I` rule enforces the order). Engine modules import only the standard
   library and each other.
3. Module constants in `UPPER_CASE`, each with a comment that says what it means, such
   as `MATE_SCORE = 100_000  # Larger than any material count`.
4. Functions, simplest first, so a reader meets each helper before the code that calls it.
5. In `uci.py` and in each designer script, a `main()` function and the guard that
   calls it:

```python
if __name__ == "__main__":
    main()
```

The engine modules other than `uci.py` define functions only and print nothing, so
students type no self-check code. Engine modules import each other, and `uci.py`
imports nearly all of them; a module that printed at import time would send stray
text to Lucas Chess and break the protocol. A designer script prints what it checked
next to the expected answer, so it never runs silently.

### Python code style

#### Type hints

- Use type hints for every reusable function and class, in modern Python 3.14 syntax:
  `list[str]`, `tuple[int, int]`, `str | None`, `dict[str, int]`.
- Give complex types a name once and reuse it, such as `Square = tuple[int, int]`, so a
  student reads `Square` instead of a nested bracket expression.
- `main()` and short designer scripts may leave out hints.

#### Docstrings

Use **NumPy-style docstrings** for reusable functions:

```python
def square_name(row: int, col: int) -> str:
    """
    Convert board coordinates to an algebraic square name.

    Parameters
    ----------
    row : int
        Board row, where row 0 is rank 8.
    col : int
        Board column, where column 0 is file a.

    Returns
    -------
    str
        The square name, such as "e4".
    """
```

Short helper functions may use a one-line docstring:
`"""Return True if the piece is White."""`

#### Names

- Functions and variables use `snake_case` with whole words: `legal_moves`,
  `captured_piece`, `halfmove_clock`. Abbreviate only where chess itself does (`fen`,
  `san`, `uci`, `pv`).
- Booleans read as yes-or-no questions: `is_sorted`, `in_check`, `swap_needed`.
- Loop variables may be short where the meaning is plain: `row`, `col`, `i`. Never use
  `l`, `O`, or `I` as a name.
- Do not shadow built-in names such as `sum`, `list`, `max`, or `input`.

#### Idioms used throughout

- f-strings for all output. Self-documenting `f"{moves=}"` is fine in quick
  checks.
- Thousands separators make counts readable: `197_281` in code and `f"{nodes:,}"` in
  output.
- `time.perf_counter()` for timing, reported as `f"Total run time (sec): {elapsed:.3f}"`.
- Seed the random generator (`random.seed(2016)`) wherever a check must give the
  same result on every run.
- Plain loops over clever one-liners, everywhere in the repository (David,
  2026-10-27). Simple code for student readers matters more than the most
  Pythonic or the fastest form, and every file follows the same rule:
  - Build text with `+` or `+=` in a loop, never with `str.join()`.
  - Find the best item with a loop that remembers the best so far, never with
    `max(..., key=lambda ...)` or `sorted(..., key=...)`. Do not use `lambda` at all.
  - Fill a list with a loop and `append`, never with a list, set, or dict
    comprehension. Check every item with a loop and a yes-or-no variable, never
    with `any()` or `all()` over a generator expression.
  - `min()` and `max()` of two numbers, and `sum()` of a list, are fine.
  - The workshop designer scripts in `admin/` are tools no student reads, so
    they may keep the compact forms.

### Comments

Comments are **functional and explanatory**. They say what a step means in chess or why
the code is written this way, not what the syntax already shows.

- Explain purpose and meaning: `# Row 0 is rank 8, so White pawns move toward row 0`.
- Address the student where a step needs a nudge: `# Try a rook on d4: it has 14 moves`.
- Avoid decorative comments (banner lines, boxes of `#` characters) and comments that
  restate the code (`i += 1  # add one to i`).
- A short comment above a block beats a long comment at the end of a line.

#### Present tense only

Comments describe the code **as it stands now**, in the present tense. The reader
sees only the current version, so anything about the code's past is confusing noise.

- **Never refer to a prior version of the code**, an earlier approach, or how the
  code used to behave.
- **Never describe a past mistake or bug**, or frame the current code as a "fix,"
  "correction," or improvement over something that came before.
- Do not use words like *now*, *again*, *still*, *no longer*, *previously*, *used
  to*, or *the old way* to contrast with an earlier state.
- State what the code does and **why it is written this way**, on its own terms. If
  a design choice needs justifying, justify it by the requirement it meets, not by
  the alternative it replaced.

Example: instead of "The king-safety check is the fix: the old filter let a pinned
knight move," write "A move is legal only if the mover's king is not attacked after
the move is made. Testing after the move catches pins, discovered checks, and en
passant captures that expose the king."

### File naming

- Use lowercase `snake_case` for all files, Python and data alike.
- File names should be **descriptive and topic-based**, for example
  `move_generation.py`, `transposition_table.py`.
- No file name holds a space (David, 2026-11-01): `remus_versions.md`,
  `lucas_chess_guide.md`. Join the words of a file name with underscores.

### File input and output

Every file a script reads or writes lives in the same folder as the script, or in a
folder named relative to it. A bare filename resolves against the current working
directory, so the same script run from another folder writes somewhere else. Anchor
the path to the script and pass `file_path` to every call that touches the file:

```python
file_path = Path(__file__).parent / "opening_book.txt"
print(f"Reading {file_path}")
```

Print the full `file_path` rather than the bare name when reporting a save.

Build `file_path` from `Path(__file__).parent`, never from the bare name, and pass
`file_path` itself to every call that touches the file. Computing `file_path` and then
handing the bare name to `open()` looks correct at a glance and silently reads or
writes in the wrong folder.

### No notebooks

Remus is plain `.py` files. The workshop uses no Jupyter notebooks, so
the notebook rules and the `nb_check.py` hook from the course repositories do not
apply here.

### Linting and formatting

`uv run ruff check .` must pass before a commit. The settings in `pyproject.toml` match
the course repositories: ruff's default rules plus `I` (import order), `C4`
(comprehensions), `ISC` (string concatenation), `SIM` (simpler code), and `TCH`
(type-checking imports), with the default line length of 88. `uv run ruff format .`
formats the code.

The code must also show no type errors in VS Code's Pylance. Check an engine with
`uvx pyright engines_instructor/Remus2`. When a function can return `None`, such as `think()` or
`legal_book_move()`, test for `None` before using the result.

### American spelling

Every word in this repository uses American spelling: prose, docstrings, comments,
Markdown, variable and function names, and printed output. Never use British spelling.

<!-- cspell:disable-next-line -->
- `traveled`, `canceled`, `modeled`, `labeled` (one `l`), not `travelled`,
  `cancelled`, `modelled`, `labelled`.
- `color`, `favor`, `behavior`, `neighbor`, `center`, `defense`, `offense`, `gray`,
  `catalog`, `analyze`, `realize`, `recognize`, `organize`, `judgment`, `toward`,
  `while` (not `whilst`), `among` (not `amongst`).
- Library identifiers keep their own spelling, so `engine.analyse(...)` from
  python-chess stays as the API names it.
- Do not rewrite British spellings inside third-party data, quoted sources, or proper
  nouns.

### Mermaid diagram colors

Use one stable palette by module across every engine version, Python and Rust,
generated SVGs, and inline Mermaid blocks. A module keeps its color when another
module is added or removed. Do not assign colors by node order, programming
language, execution frequency, or whether a node is a decision. `main.rs` is the
Rust counterpart of `uci.py` and shares its color.

| Module or role | Color | Fill |
| --- | --- | --- |
| `uci.py`, `main.rs` | Blue | `#dbe8fb` |
| `search.py`, `search.rs` | Orange | `#fde2cf` |
| `opening_book.py`, `opening_book.rs` | Light green | `#e3f4d7` |
| `transposition_table.py`, `transposition_table.rs` | Lavender | `#efe0f7` |
| `exchange.py`, `exchange.rs` | Yellow | `#fff1c2` |
| `evaluation.py`, `evaluation.rs` | Light red | `#fbd9d9` |
| `move_generation.py`, `move_generation.rs` | Teal | `#d4f1f1` |
| `make_move.py`, `make_move.rs` | Tan | `#ece4d6` |
| `attacks.py`, `attacks.rs` | Pink | `#fadcec` |
| `board.py`, `board.rs` | Blue gray | `#e2e5f3` |
| `tablebase.py`, `tablebase.rs` | Mint | `#cdeedc` |
| `search_control.py`, `search_control.rs` | Sage | `#dce8d4` |
| `parallel_search.rs` | Steel blue | `#c8def0` |
| `search_metrics.rs` | Blue teal | `#d0e7ed` |
| `table_metrics.rs` | Mauve | `#ead8e5` |
| `diagnostics.rs` | Apricot | `#f5d5b8` |
| Lucas Chess, launchers, external programs, output files | Gray | `#f1f5f9` |
| Opponent participant in sequence diagrams | Muted green | `#e0edd7` |
| Explanatory notes | Cream | `#fff6dd` |

Use text `#172033`, borders `#475569`, and arrows `#64748b` on a white background.
Flowcharts use Arial and explicit `module_<role>` classes, such as `module_search`
and `module_uci`. Keep filenames or clear responsibility labels visible; color
is a supporting cue, never the only way to identify a module. Conceptual search
and ponder nodes use the color of the module they represent. A standard-library
call shown inside an engine function uses the calling engine module's color.
External artifacts and programs have the external gray role.

Sequence diagrams color participant lanes rather than individual commands:
Remus's UCI boundary uses blue, Lucas uses gray, and the opponent uses muted green.
Actor boxes use white with dark text and borders; explanatory notes use cream.
Messages retain the arrow color regardless of sender. A lane denotes the protocol
participant, not all the internal modules that process a message.

Keep the color reference table in this guide. Engine READMEs show the diagrams
without a module/color/fill legend table.

The shared designer palette is `admin/mermaid_palette.py`. The Python and Rust
renderers, inline-diagram styler, and UCI renderer all import it. Keep this table
and the shared palette in agreement. New runtime modules require an explicit
palette entry; do not silently fall back to the Mermaid theme. Regenerate the
`.mmd` sources and SVGs after palette or diagram changes, including SVG copies of
inline diagrams, and verify node assignments and rendered fills. Preserve the
archival snapshot in `.archive/` and third-party diagrams.

### Dashes

Never use an em dash, an en dash, a minus sign, or any other long dash character
(U+2013, U+2014, U+2212, and their relatives) anywhere: not in prose, comments,
docstrings, Markdown, or f-strings. Use a colon, a comma, parentheses, or a plain
hyphen-minus (`-`) instead. Write ranges as `5 to 14` or `5-14`, results as `1-0`,
and negative numbers and scores with the ordinary hyphen.

### Editorial and style guidelines

These apply to the workshop text and plan, README, docstrings, comments, and chat replies.

1. **Avoid robotic abstractions and nested clauses.** Prefer direct, active verbs over
   phrasal verbs like "comes with" or "results in." Say who applies a rule or what the
   program counts. Eliminate dangling final pronouns by naming the noun.
2. **Ban conversational filler.** Never open with "It's important to remember that...",
   "Crucially, ...", "Furthermore, ...", "In summary, ...", or "As previously
   mentioned...". Avoid the word "testament." Do not end an explanation with a
   superficial positive summary such as "making it both powerful and flexible." State
   what the code does and stop.
3. **Explain through concrete examples.** Start with the piece, position, square, or
   operation the reader can picture, then name the general principle. Prefer "a knight
   on a1 has two moves, to b3 and c2" to an unexplained rule about edge squares.
   Explain technical terms at first use.
4. **Make references clear.** Repeat a short, specific noun when "it," "this," or
   "that" could refer to two things. Split a sentence that changes subject midway.
5. **Preserve meaning while improving the prose.** Keep numbers, units, and
   qualifications. Do not attribute understanding to a program: describe the operation
   performed or the output recorded.
6. **Read for natural rhythm.** Vary sentence length. Use paragraphs for connected
   explanations, numbered lists for steps, and tables for comparisons. Use bold
   sparingly. Remove closing sentences that only praise the method or repeat the point.
7. **Avoid weak endings and redundancy.** Give a sentence a concrete ending. Remove a
   relative clause that repeats what the main verb already says.

Do not vouch for the text either: words such as `honestly`, `frankly`, `to be fair`,
`plainly stated`, or `it is worth stating that` spend a sentence announcing a sentence.
Give the reason to believe a claim, such as the perft count that confirms it, instead.

## Part 3: References

### Sources

- [Sargon (chess), Wikipedia](https://en.wikipedia.org/wiki/Sargon_(chess))
- [Dan Spracklen, Chess Programming Wiki](https://chessprogramming.org/Dan_Spracklen)
- [List of kings of Akkad, Wikipedia](https://en.wikipedia.org/wiki/List_of_kings_of_Akkad)
- [The Akkadian Period, The Met](https://www.metmuseum.org/es/essays/the-akkadian-period-ca-2350-2150-b-c)
