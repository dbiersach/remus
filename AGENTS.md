# Remus Project Rules and Python Style Guide

Remus is a teaching chess project for the **Turning a Chess Engine into Science AI**
workshop. Student pairs type Remus1 to Remus4 in Python from screenshots. Annotated
solutions live privately under `instructor/Remus1` through `Remus4`; testing copies
live in each Day folder's `RemusN` subfolder. Remus5 through Remus8 are Rust-only
engines under `rust_engines`, with all their engine documentation beside their source.
[`docs/remus_versions.md`](docs/remus_versions.md) describes the versions and
[`workshop_syllabus.md`](workshop_syllabus.md) describes the five days.

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

- The workshop is open to students of all ages who know some coding and chess.
  Use the plain word "students", with no school or age classification.
- Students work in student pairs, switching driver and navigator every period.
  Four 90-minute periods run 8:30 to 10:00, 10:30 to 12:00, 1:00 to 2:30, and 3:00 to 4:30.
- Windows 11, VS Code, and Lucas Chess are the workshop platform. No GUI is built.
- Student pairs type Python from period screenshots, never from instructor source.
  Student-facing documents must neither mention nor link private solution folders.
- Minimize student typing while keeping clarity, playing strength, and performance.
  Exclude supplied verifiers and data from every typing or runtime code-count table.
  Remus1 to Remus4 have 189, 300, 484, and 630 runtime lines of code.
- On Day 1 student pairs create runtime modules in the Day 1 Remus1 folder. On Days
  2 to 4 they copy only the previous runtime modules, excluding its verifier, into
  the next engine folder and type the changes. Keep the destination's guide,
  launcher, verifier, and data. Student pairs never type Rust.
- A helper module's silent run checks syntax/imports/top-level loading, not function
  behavior. Running uci.py waits silently for protocol input. The supplied version
  verifier checks reference cases; it does not establish Elo or exhaustive correctness.
- Lucas Chess plays the student-written day-local engines after verification.
  Teacher demonstrations can use private completed copies on the teacher's laptop.
- Students create no online accounts. Initial setup clones the public repository,
  runs setup_workshop.ps1 -StudentCopy, installs extensions, and runs uv sync.
  It removes Git metadata, private solutions, designer tools, and temporary runtime
  Python files while keeping each verify_remusN.py. It deletes itself only on success.
  Never run cleanup on the author checkout or over completed student work.
- After setup students use VS Code's Run Python File button, Lucas Chess, and File
  Explorer. No student types Git or uv commands again during the workshop; supplied
  remusN.bat launchers invoke uv automatically. The teacher writes no code in class.
- Say "student pairs" or "the student pair", not bare "pairs" for the audience.
- Documentation describes present behavior. Keep change history in a handoff or
  commit, not READMEs. Explain expected output and exactly what each run checks.

### Hard rules for the engine code

1. **Clarity over speed.** Plain lists, tuples, dicts, and functions. Slow is fine,
   but the engine copies a position instead of undoing a move because the copy is both
   shorter and faster.
2. **No third-party libraries inside the Python engine.** Standard library only.
   No bitboards, magic numbers, compiled extensions, or framework machinery.
   Remus6, Remus7, and Remus8 may use `reqwest`, `serde_json`, and `httpdate` only for HTTPS, JSON,
   and HTTP retry dates at the tablebase boundary. Their native chess code uses no
   external chess library.
3. **Engine is the single source of truth** for board, rules, legal moves, search,
   evaluation, and legal move choice. A GUI only sends moves and displays results.
   Remus6, Remus7, and Remus8 may use Lichess advice to choose a root move, after
   validating that move with the native generator and king-safety check. Their
   fallback search and evaluation remain the shared Remus6 native implementation.
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
   holds it. Remus1 is one exception: Remus2 replaces its 12-code-line `search.py`.
   Remus6 through Remus8 add positional evaluation for backward pawns, central holes,
   knight outposts, and nearby king cover; keep these additions identical across these
   implementations. Remus1 through Remus4 retain their
   workshop evaluation. Another explicit exception is table capacity: Remus5 and Remus6 use fixed 512 MiB tables. Remus7 and Remus8
   expose Hash with a 512 MiB default and 1024/2048 MiB experimental sizes. Remus3
   and Remus4 use two-million-entry Python dictionaries. Remus1 and Remus2 have no transposition table.

7. **Rules before search.** Move generation must match the published perft counts
   before any search change is trusted.
8. The opening book is a boundary feature. Validate every book move with the native
   generator; an absent book must never stop play. Remus opening choices come only from
   that engine's supplied `opening_book.txt`: Python reads it and Rust embeds it
   at build time. Disable Lucas Chess external books for every Remus entry,
   including Remus5, Remus6, Remus7, and Remus8. Tournament engine entries use
   explicit `<None>`, not `<By default>`, so they cannot inherit a GUI book.
   When no legal book continuation exists, use native move selection. Remus1
   and Remus2 have no opening book and keep their native behavior.
9. Remus6 through Remus8 tablebase requests belong only at the root, never inside native search.
   Query at most seven pieces, including kings, with no castling rights. Preserve
   the actual halfmove clock and en passant square in the API FEN. Keep repetition
   checks native: FEN does not carry game history. Unknown or uncertain results,
   invalid moves, malformed replies, failures, and timeouts select native search.
   Make at most one request at a time per engine process, cache by complete API FEN,
   and wait at least 60 seconds after HTTP 429, honoring a longer Retry-After.
   Bound the lookup wait to one second and one quarter of the move budget, subtract
   elapsed lookup time from the search budget, and offer the `LichessTablebase` UCI
   option (default true). Fixed-depth Rust checks bypass the service.

10. Remus6 uses a fixed 512 MiB table, with four 16-byte entries per 64-byte
    bucket, multiply-shift indexing, and depth/age replacement. Its Windows virtual
    memory allocation starts zero-filled and disables caching if allocation fails.
    It exposes no Hash option and does not retry smaller capacities. Remus5 also
    has a fixed 512 MiB table but retains its low-bit indexing and native allocator.
    Remus7 and Remus8 share a configurable table across workers, defaulting to
    512 MiB and four threads, with 1024/2048 MiB experimental sizes.

11. Remus6, Remus7, and Remus8 support standard UCI pondering with a `Ponder` check
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

    All three engines preserve legal book continuations during both hit forms. A book
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
  sends `position startpos moves ...` for every game. Native Python code uses the position tuple. Rust
  retains its `position fen` UCI input for designer checks; `tablebase.rs` serializes
  outbound API FEN. The API fullmove number is 1 because it does not affect a probe.

### Architecture boundaries

| Part | Responsibility |
| --- | --- |
| Native Python engine | Board, rules, moves, search, evaluation, move choice |
| UCI adapter, `uci.py` | `uci`, `isready`, `position startpos moves ...`, `go`, `quit`; only protocol lines on stdout; `bestmove 0000` only with no legal move |
| Tablebase adapter, Remus6 through Remus8 | Root advice from Lichess over HTTPS; FEN/JSON boundary, cache, cooldown, bounded wait, native legality validation, and search fallback |
| GUI | Lucas Chess, which starts `remusN.bat` and hosts Remus as an external UCI engine |
| Checks | `admin/verify_engines.py`, outside the engines |

The engine must run without Lucas Chess or `python-chess` installed. Tablebase-enabled engines must
also play without internet access or a successful tablebase response.

### Verification

- Write expected results before running. After engine changes run
  `uv run admin/verify_engines.py` for published perft, mate puzzles, fixed-depth
  searches, and UCI checks. Search-preserving changes must preserve moves, scores,
  and node counts, apart from separately justified table-collision effects.
- Run each day-local verify_remusN.py and `admin/verify_student_uci.py` when changing
  verifiers or student copies. Module loading is not a substitute for these checks.
- `uv run admin/build_rust.py --engine N` builds current Rust versions N=5..8.
  Build sequentially. Verify Remus5 and Remus6 against their Python references with
  `admin/verify_rust.py --engine N`; for Remus7/8 supply a preserved executable with
  `--baseline` and one search thread. Parallel node counts may vary with scheduling.
- Designer tools may read the archived Python5 implementation as a comparison oracle
  through `admin/workshop_paths.py`; never modify that archive. Legacy boundary and
  pondering fixture tools retain historical labels through explicit path adapters.
  Their references are not active workshop engine names.
- Tablebase, cache, cancellation, and pondering changes require the corresponding
  designer fixture checks. Live HTTPS checks are optional and must distinguish a
  native fallback from a successful tablebase result. Native search must work offline.
- Preserve existing logs and completed match evidence. **Matches are paused at
  David's request. Do not start or resume games without fresh approval.**
- After source/diagram changes run `admin/verify_engine_docs.py`. After screenshot
  source or line-reference changes run `admin/make_code_images.py --check`, then
  capture changed screenshots. Check actual PNG/JPEG signatures and extensions.

### Project setup and layout

- Python 3.14 is managed by uv. `.venv` is the sole workshop environment.
  matplotlib supplies science graphs; python-chess and Ruff are designer/verifier
  dependencies. Engines import only standard-library and sibling engine modules.
- `instructor/Remus1` through `Remus4` hold annotated solutions. Each private
  RemusN_details.md contains only Line-by-line source reference and Source coverage
  and identity. `instructor/science` holds completed science programs.
- `Day N - .../RemusN` contains the merged RemusN.md, remusN.bat, verify_remusN.py,
  and supplied data. Complete generated student Python is retained temporarily for
  testing and Lucas Chess. Setup on a student clone removes runtime .py files only,
  preserving its version verifier. Never hand-edit generated runtime modules.
- `admin/make_student_engines.py` generates stripped modules in memory and checks
  existing day copies. `--populate-days` explicitly writes testing copies. There is
  no engines_student folder. Remus2 to Remus4 keep piece_square_tables.json; Remus3 and Remus4 keep
  opening_book.txt. Verifiers are supplied and excluded from code totals.
- Day and period syllabuses sit directly in each Day folder. Day 5 also contains
  hidden_peak.py and phase_hunt.py starters; their graphs are supplied, not typed.
- `rust_engines/Remus5` through `Remus8` contain all active Rust code, Cargo metadata,
  embedded books, remusN.exe, remusN.md, and RemusN_details.md. There is no -Rust
  suffix and no active Python version above Remus4. Preserve source algorithms
  during layout and identity changes.
- `docs` is flat: shared .md/.pdf guides and install_lucas_engines.py. The installer
  registers day-local Python launchers and rust_engines executables, with Lucas
  books disabled. Registration does not test chess behavior.
- `images/Day1` through `Day5` hold period-prefixed code and science images;
  images/chess_tutor, images/lucas_chess, and images/color_swatches hold guide assets.
  `mermaid/Remus1` through `Remus8` and mermaid/chess_tutor hold diagram sources,
  rendered SVGs, and manifests. Never recreate asset folders inside Day or docs.
- Designer scripts live under ignored `admin`, resolve the workspace from their own
  path, and are listed in admin/README.md. Use workshop_paths.py for shared paths.
  Private game records live under admin/played_games. Preserve historical records
  and frozen benchmark inputs rather than relabeling their contents.
- `.archive` is private and ignored. Existing snapshots must never be edited or
  deleted. The retired Python5 source and documentation are preserved there.
- Maintain source counts, links, code screenshots, diagram manifests, guides, briefs,
  and syllabus workflow together. Display "Lines of code", not physical-line counts,
  for student typing totals. The line numbers used beside screenshots remain useful.

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
