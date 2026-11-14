# The versions of Remus

Student pairs build four Python engines, one for each of Days 1 to 4, by typing
the code shown in the period screenshots. Later versions reuse completed modules
and add the changes taught that day:

| Version | What it adds | New lines to type | Lines of code |
| --- | --- | ---: | ---: |
| [Remus1](../Day%201%20-%20Remus1%20plays%20legal%20chess/Remus1/Remus1.md) | Legal chess, a random legal move with no search | 189 | 189 |
| [Remus2](../Day%202%20-%20Remus2%20searches%20ahead/Remus2/Remus2.md) | The evaluation, alpha-beta, quiescence, MVV-LVA ordering, a check extension, and iterative deepening | 123 | 300 |
| [Remus3](../Day%203%20-%20Remus3%20adds%20the%20biggest%20gains/Remus3/Remus3.md) | Killer moves, pawn structure, a transposition table, an opening book, aspiration windows, time management, null-move pruning, the principal variation, the bishop pair, and the rook table | 184 | 484 |
| [Remus4](../Day%204%20-%20Remus4%20adds%20everything%20else/Remus4/Remus4.md) | Every other enhancement: draw sense, the endgame terms, late move reductions, principal variation search, pruning margins, static exchange evaluation, and the last evaluation terms | 146 | 630 |

New lines are the lines a student pair types for that version, and total lines count the
whole engine. Both leave out blank lines, comments, and docstrings. Before Remus2 the
student pairs finish [Remus1](../Day%201%20-%20Remus1%20plays%20legal%20chess/Remus1/Remus1.md) at the end of Day 1: the board, the
moves, and `uci.py` of Remus2, with a 12-line `search.py` that plays a random legal
move. Remus2 replaces that file, so its new lines are `evaluation.py` and the 93 lines
of its own `search.py`. Each version is the one before plus its enhancements, so a
student pair that finishes Remus2 types only the new lines of Remus3, and then only the
new lines of Remus4.

The compiled [Remus5](../rust_engines/Remus5/remus5.md) is Remus4 written in Rust, with a
larger fixed 512 MiB table and persistent search diagnostics. Student pairs do not type
it. Its [measured examples](../rust_engines/Remus5/remus5.md#measured-examples) illustrate the
speed difference on the benchmark computer; they are not a universal
hardware-independent speed ratio.

The feature catalog explains the workshop enhancements and the files they change.
The Rust sections describe the supplied engines and their speed techniques.
Whole-engine rating methods and limitations appear in the final section.

## Engine defaults and Total Lines of Code (TLOC)

The table lists engine defaults except the Remus7 and Remus8 ELO measurement
settings: 16 search threads and a 2 GiB transposition table. Their advertised defaults
are four threads and 512 MiB. Saved GUI settings can override engine defaults.
Remus1 chooses a random legal move. Remus2 through Remus6 use one chess search
thread; Remus7 and Remus8 parallelize chess search.

For pondering with Remus6 to Remus8 in Lucas Chess R6, keep the advertised `Ponder` default
false and select true in both its external-engine and tournament settings. The
explicit true override activates Lucas's pondering controller. A true advertised
default can leave that controller inactive even when the checkbox is checked.
The table reports engine defaults; the recommended Lucas Ponder selection for those
three engines is true. Remus1 to Remus5 do not support pondering.

| Engine | ELO | Rank | Language | TLOC | Search threads | Transposition table | Ponder | Diagnostics |
| --- | ---: | --- | --- | ---: | --- | --- | --- | --- |
| [Remus1](../Day%201%20-%20Remus1%20plays%20legal%20chess/Remus1/Remus1.md) | 300 | Below Class D | Python | 189 | 1 | None | Not supported | No persistent logs |
| [Remus2](../Day%202%20-%20Remus2%20searches%20ahead/Remus2/Remus2.md) | 1840 | Class A player | Python | 300 | 1 | None | Not supported | No persistent logs |
| [Remus3](../Day%203%20-%20Remus3%20adds%20the%20biggest%20gains/Remus3/Remus3.md) | 2086 | Expert | Python | 484 | 1 | 500 MiB | Not supported | No persistent logs |
| [Remus4](../Day%204%20-%20Remus4%20adds%20everything%20else/Remus4/Remus4.md) | 2195 | Expert | Python | 630 | 1 | 500 MiB | Not supported | No persistent logs |
| [Remus5](../rust_engines/Remus5/remus5.md) | 2897 | Super Grandmaster | Rust | 1,849 | 1 | 512 MiB | Not supported | SearchDiagnostics false |
| [Remus6](../rust_engines/Remus6/remus6.md) | Not measured | Not measured | Rust | 2,797 | 1 | 512 MiB | false | SearchDiagnostics false; PonderDiagnostics false; per-process text logs |
| [Remus7](../rust_engines/Remus7/remus7.md) | Not measured | Not measured | Rust | 3,216 | 16 | 2 GiB | false | SearchDiagnostics false; PonderDiagnostics false |
| [Remus8](../rust_engines/Remus8/remus8.md) | 3099 | Super Grandmaster | Rust | 3,825 | 16 | 2 GiB | false | SearchDiagnostics false; PonderDiagnostics false |

Estimated Elo and human-level labels summarize the recorded benchmarks, not human
FIDE ratings. Versions without a reported estimate are marked Not measured. See
[benchmark conditions and confidence intervals](#where-the-remus-engines-would-land)
for their scope and limitations.

Rust diagnostics use per-process files in `logs/` beside each executable. Search
files are `search_<pid>_<session>.txt`; Remus6 through Remus8 also write
`ponder_<pid>_<session>.txt` when enabled. The matching suffix pairs the two streams.
See [search measurements](search_logging.md) for aggregation and retention.

Remus8 additionally exposes `PonderHitMinPercent`, default 25, range 0 to 100.
Remus6 through Remus8 expose `LichessTablebase`, default true. The
transposition table in each parallel engine is shared by its workers, not across
engine processes. All book-enabled Python engines read their own file at startup;
Rust embeds its book at build time. Disable GUI books with explicit `<None>` in tournaments.

Each engine guide uses the same main headings and file-table columns. Python comparisons use
the preceding Python version; Remus6 through Remus8 use the preceding Rust version. Remus5
has no preceding Rust edition and identifies Python Remus4 as its counterpart, not its
predecessor. A zero denotes an absent file. Supporting files and external checks do not
contribute to runtime lines of code.

## How to read the tables

- **Python lines in the feature catalog** are estimates for an individual enhancement; each may differ by up to a third. **TLOC (total lines of code) and engine/file line counts** are counted from current runtime source, excluding blank lines, standalone comments, and Python docstrings. Rust documentation comments are excluded too.
- **New lines to type** differs from the net change in lines of code. Remus2 replaces 12 search lines and adds 123 lines, increasing the total by 111. Engine-guide file tables show current and preceding line counts and the net change.
- **Elo** in the feature catalog is an estimated gain for one enhancement, not a
  measured Remus result or a target for a student test. Whole-version match results
  do not isolate a feature's effect. Gains overlap as the engine grows and cannot
  be added to predict a rating.
- Rows marked **(foundation)** provide supporting data or output for later steps.
- **Version** names the engine that first holds the step.

## The enhancements, grouped by the file they change

### (E) Evaluation Only: `evaluation.py`

Each row describes a term in `evaluate()`. These terms score positions rather than
choose legal moves. Errors can change move choices or prevent the engine from running,
so evaluation changes still need the version verifier.

| Step | Enhancement | What it adds | Python lines | Elo | Version |
| --- | --- | --- | ---: | ---: | --- |
| E1 | Pawn structure | Penalties for doubled and isolated pawns, a bonus for passed pawns that grows as they advance | 30 | +49 | Remus3 |
| E2 | Mating bonus | When ahead in material, push the lone enemy king to the edge and bring the own king close, so K+Q and K+R against K end in mate | 12 | +19 | Remus4 |
| E3 | Tapered evaluation | A game phase from the pieces left, a supplied endgame king table that brings the king to the center, and a blend of the middlegame and endgame scores | 12 | +40 | Remus4 |
| E4 | Bishop pair | +30 for a side that keeps both bishops | 7 | +10 | Remus3 |
| E5 | Rooks on open files | +20 for a rook on a file with no pawns, +10 on a file with only enemy pawns | 15 | +10 | Remus4 |
| E6 | Rook table | Load the supplied rook piece-square table that Remus2 leaves out: rooks on the seventh rank and central files | 1 | +5 | Remus3 |
| E7 | Mobility | A few centipawns for each square a knight, bishop, rook, or queen reaches | 20 | +20 | Remus4 |
| E8 | Pawn race | In pawn endings, a passed pawn the enemy king cannot catch (the rule of the square) counts nearly a queen | 22 | +10 | Remus4 |
| E9 | King safety | A bonus for each pawn in front of a castled king, a penalty for open files next to it | 18 | +4 | Remus4 |

### (S) Search Only: `search.py`

These change which moves the search tries and how deeply. A bug can produce legal
but poor choices. At the scheduled checkpoints, `verify_remusN.py` (with your version number in place of N) compares the
student engine with stored results for that version or period; its engine guide describes
the expected output. Passing those cases does not prove that pruning is safe in
every position or that strength improved. Selective search changes can alter moves,
scores, and node counts; fewer nodes and an unchanged move are not universal
requirements for an enhancement.

| Step | Enhancement | What it adds | Python lines | Elo | Version |
| --- | --- | --- | ---: | ---: | --- |
| S1 | Killer moves | Remember two quiet moves per ply that caused a cutoff, and try them right after the captures | 12 | +67 | Remus3 |
| S2 | Null-move pruning | Let the opponent move twice with a shallower search; if the side to move still stands above beta, cut off at once. Copy-make makes the "null move" three values: the same board, the other side to move, and no en passant square | 13 | +25 | Remus3 |
| S3 | Late move reductions | Search late quiet moves one ply shallower, and again at full depth only if they look good | 12 | +28 | Remus4 |
| S4 | Principal variation (foundation) | Read the line of best play from the transposition table and show it in the info line | 12 | +5 | Remus3 |
| S5 | Principal variation search | Search every move after the first with a null window (alpha, alpha + 1), and again with a full window only if it beats alpha | 11 | +22 | Remus4 |
| S6 | Delta pruning | In quiescence, skip a capture that cannot lift the score to alpha even with 200 centipawns to spare | 5 | +5 | Remus4 |
| S7 | Futility pruning | One move from the leaves, skip quiet moves when the side to move is far below alpha | 8 | +15 | Remus4 |
| S8 | Aspiration windows | Start each iteration with a narrow window around the last score, and widen it on a fail | 15 | +10 | Remus3 |

### (M) Multiple Files

| Step | Enhancement | What it adds | Files | Python lines | Elo | Version |
| --- | --- | --- | --- | ---: | ---: | --- |
| M1 | Draw sense | Score threefold repetition, the fifty-move rule, and insufficient material as draws inside the search. Needs a halfmove clock in the position, a history of position keys, and the moves of `position startpos moves ...` kept in that history | `board.py`, `make_move.py`, `search.py`, `uci.py` | 28 | +4 | Remus4 |
| M2 | Time management | Spend more time when the best move changes between iterations, less when it holds, and never more than a hard limit per move | `search.py`, `uci.py` | 16 | +20 | Remus3 |

Draw sense gains little Elo, but it matters more than that number suggests in real
games: without it Remus repeats moves into a draw when it is winning, and misses a
repetition that would save a lost game.

### (N) New Files

| Step | Enhancement | What it adds | Files | Python lines | Elo | Version |
| --- | --- | --- | --- | ---: | ---: | --- |
| N1 | Position key (foundation) | A number that names a position, for the transposition table and the repetition history: Python's `hash()` of the tuple `(tuple(board), white, castling, en_passant)` | `board.py` | 2 | 0 | Remus3 |
| N2 | Transposition table | Remember each searched position's score, depth, bound, and best move; reuse the score when it suffices, and try the stored move first | **New file** `transposition_table.py`, plus `search.py` | 35 | +101 | Remus3 |
| N3 | Opening book | Play a stored opening line while the game follows one. The book holds UCI moves (`e2e4 e7e5 g1f3`), not SAN, so it needs no notation code | **New file** `opening_book.py`, plus `uci.py`, plus the supplied `opening_book.txt` data | 28 | +58 | Remus3 |
| N4 | Static exchange evaluation | Play out every capture on one square, cheapest attacker first, to see if a capture wins material; quiescence then skips the losing captures | **New file** `exchange.py`, plus `search.py` | 45 | +15 | Remus4 |

## Remus2: looking ahead

Remus2 is the engine every later step builds on. It keeps Remus1's rules and replaces
the random choice with a score and a search, in the order Day 2 types them:

| Part | Enhancement | Why it comes here |
| --- | --- | --- |
| Material and piece-square tables | `evaluate()`: 100 for a pawn, 300 for a knight or bishop, 500 for a rook, 900 for a queen, plus a bonus or penalty for each piece's square | A search needs a score to compare positions, so the evaluation comes first |
| MVV-LVA ordering | `ordered_moves()` tries the best capture first: pawn takes queen before queen takes pawn | Alpha-beta cuts off sooner when the best move comes first |
| Quiescence | `quiescence()` plays out the captures at the end of the search before it trusts the score | A search that stops in the middle of an exchange counts a queen as won when it is about to be taken back |
| Alpha-beta | `alpha_beta()` looks ahead, assumes both sides play their best, and skips the moves that cannot change the result | The heart of the engine: the same best move as minimax, from far fewer positions |
| Check extension | A side in check searches one ply deeper | A search never stops on a position where the king is in check |
| Iterative deepening | `think()` searches to depth 1, then 2, then 3, until its time runs out | The engine always has a move ready, and each search starts with the best move of the one before |

The parts change the files of Remus1 like this:

| File | Remus1 | Remus2 | What changes |
| --- | ---: | ---: | --- |
| `evaluation.py` | 0 | 30 | New: load four supplied piece-square tables from JSON, set piece values, and define `evaluate()` |
| `search.py` | 12 | 93 | Replaced: the random `think()` gives way to `ordered_moves()`, `count_node()`, `quiescence()`, `alpha_beta()`, and a `think()` that deepens one ply at a time |
| `uci.py` | 44 | 44 | The name line reads `id name Remus2` |
| `board.py`, `make_move.py`, `attacks.py`, `move_generation.py` | 133 | 133 | Unchanged |
| **Total** | **189** | **300** | |

## Remus3: the biggest gains for the fewest lines

Remus3 holds eleven steps, which split the new lines about evenly between Remus3 and
Remus4, so each takes one day of typing:

| Step | Enhancement | Why it comes early |
| --- | --- | --- |
| S1 | Killer moves | The best gain for the fewest lines, and it teaches why move order matters to alpha-beta |
| E1 | Pawn structure | The largest evaluation gain; students see Remus value passed pawns |
| N1, N2 | Position key, then the transposition table | The largest single gain; the key comes first because the table needs it |
| N3 | Opening book | Variety and sound openings; a typing-light step built on a data file |
| S8, M2 | Aspiration windows, then time management | They help most once the table makes each iteration fast, and they finish the iterative deepening loop |
| S2 | Null-move pruning | The largest depth gain for about a dozen lines |
| S4 | Principal variation | The `info` line shows the line Remus expects, read from the table |
| E4, E6 | Bishop pair, rook table | Short evaluation terms that need no change to the rest of `evaluate()` |

The steps change the files of Remus2 like this:

| File | Remus2 | Remus3 | What changes |
| --- | ---: | ---: | --- |
| `board.py` | 15 | 23 | `MATE_SCORE`, `position_key()`, and `move_text()` |
| `evaluation.py` | 30 | 68 | `pawn_terms()`: doubled, isolated, and passed pawns; the rook table and the bishop pair |
| `search.py` | 93 | 173 | Killer moves, the table probe and store, null-move pruning, the principal variation in the info line, the aspiration window, and the time that grows when the best move changes |
| `uci.py` | 44 | 59 | The opening book, and a planned time with a hard limit |
| `transposition_table.py` | 0 | 22 | New: `store()` and `probe()` |
| `opening_book.py` | 0 | 21 | New: `load_book()` and `book_moves()`, with `opening_book.txt` |
| `make_move.py`, `attacks.py`, `move_generation.py` | 118 | 118 | Unchanged |
| **Total** | **300** | **484** | |

## Remus4: everything else

Remus4 adds the other twelve steps, in the order a class can type them:

| Steps | Enhancements | Why here |
| --- | --- | --- |
| M1 | Draw sense | Reuses the position key to score repetitions as draws during search |
| E2, E3 | Mating bonus, then tapered evaluation | Remus learns to finish endgames: drive the king to the edge, then centralize its own king |
| S3 | Late move reductions | A depth gainer of about a dozen lines |
| S5 | Principal variation search | PVS needs the first move to be the best one, which the table supplies |
| E5 | Rooks on open files | A short evaluation term built on the pawn counts of each file |
| S6, S7 | Delta pruning, futility pruning | Two small pruning rules that share the idea of a margin |
| N4 | Static exchange evaluation | The longest single step; it sharpens quiescence once the rest is in place |
| E7, E8, E9 | Mobility, pawn race, king safety | The remaining evaluation terms, each worth a few points |

| File | Remus3 | Remus4 | What changes |
| --- | ---: | ---: | --- |
| `board.py` | 23 | 23 | The halfmove clock in the starting position |
| `make_move.py` | 23 | 24 | The halfmove clock counts toward the fifty-move rule |
| `evaluation.py` | 68 | 135 | The endgame king table and taper, mobility, rooks on open files, the pawn race, king safety, the mating bonus, and insufficient material |
| `search.py` | 173 | 212 | Draw sense and its history around the null move, late move reductions, PVS, futility and delta pruning, and static exchange in quiescence |
| `uci.py` | 59 | 62 | The history of position keys for draw sense |
| `exchange.py` | 0 | 36 | New: `cheapest_attacker()` and `exchange_score()` |
| `attacks.py`, `move_generation.py`, `transposition_table.py`, `opening_book.py` | 138 | 138 | Unchanged |
| **Total** | **484** | **630** | |

## Checking each version

Run the Day-local `verify_remusN.py` at the scheduled checkpoints. It prints the
checks and expected values, reports any problems, and ends with the version's
success line when its reference cases pass. Running an individual engine module
only checks that it and its imports load; it does not test all its functions.

The designer's separate native checks cover published perft positions, mate
puzzles, fixed-depth searches, and UCI exchanges. Recorded depth-4 searches from
the starting position visit 17,360 nodes in Remus2, 2,944 in Remus3, and 3,005 in
Remus4, with `g1f3` as the best move in each. These compare one position at one
depth; fewer nodes alone do not establish faster play or greater strength.

A strength comparison needs enough games under controlled conditions, with colors
balanced and uncertainty reported. Winning more games than losing in a short
round is not sufficient evidence to accept an enhancement. The
[closing Elo discussion](#where-the-remus-engines-would-land) describes the recorded
rating experiments. The original-versus-shortened comparison remains paused; its
unfinished results do not establish whether the code reductions preserve Elo.

## Supplied Rust engines

Remus5 through Remus8 are Rust-only engines. Their source, executables, overview
guides, and line-by-line references live under `rust_engines/RemusN`. Students do
not type them. Their source sizes, settings, and recorded ratings are listed once
in [Engine defaults and Total Lines of Code (TLOC)](#engine-defaults-and-total-lines-of-code-tloc).

| Version | What it adds |
| --- | --- |
| [Remus5](../rust_engines/Remus5/remus5.md) | Remus4 algorithms implemented in Rust with a larger bucketed table |
| [Remus6](../rust_engines/Remus6/remus6.md) | Enhanced single-threaded search and positional evaluation, root tablebase advice, and UCI pondering |
| [Remus7](../rust_engines/Remus7/remus7.md) | Lazy SMP parallel search with a shared table |
| [Remus8](../rust_engines/Remus8/remus8.md) | YBWC parallel search with sibling splitting and restricted worker helping |

Remus6 adds check-aware quiescence, terminal-position handling, en passant capture
accounting, quiet-move history, backward-pawn and central-hole penalties, knight
outposts, and nearby king cover. These additions are shared by Remus7 and Remus8.
Their presence alone does not establish an Elo gain.

Remus6 to Remus8 can request root tablebase advice for eligible positions with
at most seven pieces and no castling rights. They preserve the actual halfmove
clock and en passant square, validate suggested moves with native rules, and fall
back to native search on failures or uncertain responses. Fixed-depth checks and
ponder searches bypass the service.

When pondering is enabled in Lucas Chess, Remus6 to Remus8 can search during the
opponent's turn after `go ponder`. A matching hit starts the engine's move clock. Book continuations remain subject to native legality checks;
book pondering waits without claiming native search credit. See each engine guide
for time-budget details and supported options.

## Speed techniques of the Rust engines 6 through 8

Remus6, Remus7, and Remus8 share six techniques that make each position
cheaper to search. They are intended to preserve native search choices. In the fixed-depth
comparison positions, one-worker searches return the same scores, node counts, and
principal variations with or without the techniques. Evaluation cross-checks compare
the tested positions; they are not a proof for every possible position. Remus5 uses
the plain form of every step below, so each of its files stays close to the Remus4 Python file of the same name.

| Engine | Search threads | Plain form | With the six techniques | Ratio |
| --- | ---: | ---: | ---: | ---: |
| Remus6 | 1 | 4.38 | 5.59 | 1.28 |
| Remus7 | 1 | 4.45 | 5.60 | 1.26 |
| Remus8 | 1 | 4.36 | 5.72 | 1.31 |
| Remus7 | 4 | 12.5 | 16.6 | 1.33 |
| Remus8 | 4 | 11.7 | 16.9 | 1.44 |

The numbers are millions of positions per second on the workshop designer's computer,
summed over six fixed-depth searches: the starting position, an Italian opening, the
"kiwipete" test position, a quiet middlegame, a rook ending, and a pawn race. With one
worker both builds search the same 34 million positions, so the ratio is also the ratio
of the two run times. With four search threads the threads reach the shared table in a
different order on every run, the two builds visit different positions, and only the
rate compares.

| Step | Technique | Code | What it saves | Gain alone |
| --- | --- | --- | --- | ---: |
| T1 | Branch-free ray lookup | `rising_ray()` and `falling_ray()` in `attacks.rs` | A badly predicted test in each of the four rays of every bishop, rook, and queen lookup | 8% |
| T2 | Legal-move shortcut | `has_legal_move()` in `search.rs`, with the table `QUEEN_LINES` in `attacks.rs` | A full move generation at most quiescence positions | 10% |
| T3 | Lazy move ordering | `place_next()` in `search.rs` | The sort of the whole move list when one of the first three moves causes the cutoff | 4% |
| T4 | Central holes from one mask | `positional_terms()` in `evaluation.rs` | Sixteen square-by-square tests in every evaluation | 2% |
| T5 | Evaluation on demand | `alpha_beta()` in `search.rs` | The evaluation of positions whose pruning never reads the score | 1 to 2% |
| T6 | Table prefetch | `Table::prefetch()` in `transposition_table.rs`, called from `alpha_beta()` | Part of the wait for a table bucket to arrive from memory | 1% |

Each gain is that technique alone, in positions per second with one search thread. T1, T2, and
T4 are measured in Remus6. T3, T5, and T6 are measured in the move loop of the
Remus4 search, which the three engines share. The gains overlap, so together they give
the ratios of the first table, not their sum. The six techniques add 49 code lines to
Remus6 and 51 to each of Remus7 and Remus8.

### T1: Branch-free ray lookup

A rook on a8 looks down the a-file. `RAYS` holds that whole ray: a7, a6, a5, a4, a3, a2,
and a1. With a piece on a5, the rook reaches a7, a6, and a5, so the lookup finds the
first piece on the ray and removes the squares beyond it, which are the ray from a5
itself.

The plain form first asks "is any piece on this ray?", because an empty ray has no first
piece to look up. The answer changes from one lookup to the next with no pattern, the
processor predicts it wrongly a large share of the time, and each wrong prediction costs
about as much as fifteen ordinary operations. Every bishop, rook, or queen lookup makes
that test four times, once per ray, and the engine makes those lookups in the mobility
count of `evaluate()`, in `is_attacked()`, in move generation, and in the exchange
evaluation.

`rising_ray()` covers the four rays whose square numbers grow. It sets bit 63, the
square h1, among the pieces on the ray before it finds the first one. No rising ray
continues past h1, so the `RAYS` entry of h1 is empty: on an empty ray the lookup finds
h1, removes nothing, and returns the whole ray, with no test. `falling_ray()` does the
same for the four rays whose square numbers shrink, with bit 0, the square a8. The
result is the same set of squares in every case.

### T2: Legal-move shortcut

The quiescence search of Remus6 asks at every position whether the side to move has a
legal move, so that it scores checkmate and stalemate before it stands pat. The plain
form generates every pseudo-legal move, about 35 in a middlegame, and makes them one at
a time until one leaves the king safe.

`has_legal_move()` takes a second argument, whether the side to move is in check, which
`quiescence()` and `alpha_beta()` have already worked out. Out of check, a move can
leave its own king attacked in only three ways: it moves the king, it captures en
passant, or it takes a piece off a rank, file, or diagonal that runs through the king.
`QUEEN_LINES[king]` holds the squares of those lines. In the starting position the White
king stands on e1, and its lines hold the first rank, the e-file, and the diagonals
through d2 and f2. The pawns on a2, b2, c2, g2, and h2 stand off those lines, and the
square ahead of each is empty, so White has a legal move. Three bitboard operations find
that: keep the pawns off the lines, shift them one row ahead, and keep the empty
squares.

When the side is in check, or no such pawn exists (a pawnless ending, or pawns that are
all blocked or on the king's lines), the function runs the plain test, so its answer
never changes. The gain is 13 to 17% in the four opening and middlegame positions and
nothing in the two endings, where few pawns are left.

### T3: Lazy move ordering

`ordered_moves()` gives every move a priority and packs the priority and the move into
one number. The plain form sorts all of those numbers before the search tries the first
move. Most cutoffs come from the table move, a capture, or a killer move, which stand
among the first few, and then the sorted order of the other 30 moves is never read.

`ordered_moves()` leaves the entries in the order of generation. At the top of the move
loop, `place_next(entries, index)` puts the right entry at `index`. For the first three
indexes (`EARLY_PICKS`) it makes one pass over the entries that are left and swaps the
largest into place. At the fourth index it sorts the rest once. No two entries are
equal, because the move is part of each entry, so the moves come out in exactly the
order of the full sort.

Two places need every move in order at once and sort the list themselves. The check
evasions of `quiescence()` are few and are all tried. In Remus8,
`split_remaining()` hands every move that is left to the workers of a split, so it sorts
those moves before it builds its list of candidates.

### T4: Central holes from one mask

A central hole is an empty square on files c to f of a side's third or fourth rank that
an enemy pawn attacks and no pawn of that side can challenge. The plain form visits the
eight squares of each side one at a time and tests each for a piece and for an enemy
pawn attack.

`positional_terms()` builds the candidates with three bitboard operations: a constant
mask of the eight squares (c3 to f3 and c4 to f4 for White), the empty squares, and the
squares that enemy pawns attack. The loop then visits only the squares that pass all
three, in most positions none or one, and asks `pawn_can_challenge()` about each. The
term adds a sum of equal penalties, so the order of the squares does not matter and the
score is the same.

### T5: Evaluation on demand

`alpha_beta()` reads the static score of a position in two tests only. Null-move pruning
reads it at depth 3 and up, out of check, away from the root, and not right after
another null move. Futility pruning reads it at depth 1, out of check. The plain form
calls `evaluate()` at every position before those tests. The engines 6 through 8 compute
`needs_score` from the same conditions and call `evaluate()` only when it is true. A
position at depth 2 and a position in check never evaluate, and at depth 3 and up
neither does the root or the position right after a null move.

### T6: Table prefetch

The transposition table holds 512 MiB, far more than the processor's cache, so reading
the bucket of a position takes roughly 100 nanoseconds, about half of the time the
engine spends on an average position. The search knows which bucket the next position
needs as soon as `make_move()` returns that position's key.

`Table::prefetch(key)` asks the processor to start loading the bucket and returns at
once. `alpha_beta()` calls it right after `make_move()`, so the legality test and the
check test of the move run while the bucket travels from memory, and the child's
`probe()` waits for less. It calls prefetch only at depth 2 and up: a child of a depth-1
position goes to quiescence unless it is in check, quiescence does not use the table,
and a prefetch there costs more than it saves. The null-move search prefetches its
position the same way, and in Remus8 so does `search_split_move()` when a worker
takes a move from a split. A prefetch reads no value, so the shared table of the
parallel engines needs no added synchronization.

### Checking that the search is the same

- `verify_rust.py --engine 6` compares Remus6 with the archived Python reference: perft from six
  positions, `evaluate()` on 19,574 positions from random games, and every `info`
  line and best move of fixed-depth searches with and without a game history.
- `verify_rust.py --engine 7` and `--engine 8`, each with `--baseline`, compare
  one-worker searches with an executable built from the plain form.
- A second set of 22 positions (checks, en passant, promotions, stalemates, the
  fifty-move clock, and a repeated position in the game history) gives the same
  `info` lines in the plain form and with the six techniques in all three engines.
- `verify_remus8.py`, `verify_remus8_helping.py`, and `verify_remus8_diagnostics.py`
  check the parallel search of Remus8: legal results and lines of play, splits,
  stops, and worker accounting.
- `verify_positional_evaluation.py`, `verify_transposition_tables.py`,
  `verify_cache_uci.py`, `verify_tablebase.py`, `verify_ponder_control.py`, and
  `verify_pondering.py` pass on Remus6 with the techniques in place.

### Techniques that measure too little to keep

Each of these also leaves the search unchanged. The engines 6 through 8 leave them out
because the gain does not pay for the code.

| Technique | Measured | Why it stays out |
| --- | --- | --- |
| Magic bitboards for bishop, rook, and queen lookups | 14% alone, 2 to 3% beyond T1 | An 860 KB table, `unsafe` statics, and 128 constants to find or store |
| A cache of evaluations by position key | Under 1% at 256 KB to 1 MB, 5% slower at 4 MB | No gain at any size |
| Skipping the exchange evaluation when the victim is worth at least the attacker | Under 1% | The exchange evaluation takes about 3% of the search time in all |
| Skipping the legality test of a move by a piece off its king's lines | Under 1% | Within the error of the timing |
| Passing the parent's "gives check" test to the child | 3% alone, under 1% with T1 to T6 | One more argument through every call of `alpha_beta()` |
| Generating the moves in sorted order | No better than T3 with quiet-move history | Rewrites `generate_moves()` around one loop over the pieces |
| Testing the pawn moves before generating the piece moves in the plain legal-move test | Under 1% beyond T2 | Two more functions in `move_generation.rs` |
| Delta pruning and the exchange test before `make_move()` in quiescence | Under 1% | Within the error of the timing |
| Selecting every move one at a time, with no sort | 6% slower | Positions that try every move pay a pass per move |
| Prefetch at depth 1 as well | 1% slower | Those children seldom read the table |

## Where the Remus engines would land

FIDE gives titles only to people, so no engine can earn one. The ratings in the
[engine table](#engine-defaults-and-total-lines-of-code-tloc) summarize the benchmark described
here; they are not human FIDE ratings or expected results for a workshop test. The class labels are numerical comparisons only;
sharing a number does not establish equivalent strength against people. Remus2, Remus3,
Remus4, and Remus5 each played 200 games against Stockfish held to a set
strength (its UCI_Elo setting), at 15 minutes per side with no increment, half with
White and half with Black, on one fast processor core per game. Each engine met
Stockfish at six levels between 1320 and 2950, and its rating is the one that best
explains its score against those levels. The 27 games that Stockfish lost on time
are left out, because Stockfish ran out of time in long endgames whatever the
position on the board.

Reported 95% confidence intervals are Remus2: 1785 to 1895; Remus3: 2029 to 2143; Remus4: 2133 to 2257; Remus5: 2831 to 2963; Remus8: 2900 to 3297.

Remus1's 300 figure is a rough estimate, not a fitted benchmark result. Random
move choice does not guarantee a loss in every game.

A 95% confidence interval describes a procedure that, under its statistical
assumptions, would cover the underlying parameter in about 95% of repeated
experiments. It is not a 95% range for individual game outcomes. These intervals
do not account for every systematic effect of the opponent, hardware, time control,
opening choices, or excluded time losses. There is no fixed conversion here from
Stockfish's UCI_Elo setting to a person's FIDE rating.

Remus5 shares Remus4's core chess algorithms but uses a different implementation
and a larger table. Equal-time searches can choose different moves. The rating gap
in this benchmark does not isolate speed, table capacity, or any one enhancement,
and does not establish a strength difference against human players.

Remus8's estimate uses 20 games against limited-strength Stockfish at 15 minutes
per side with no increment, one game at a time. Remus8 uses a 2048 MiB table,
16 search threads, pondering, and its default root tablebase lookups. Stockfish
runs on a separate fast core. The ladder includes one game with each color at
2200, 2400, 2600, 2800, 3000, and 3190, then eight games around the initial estimate.
Twenty games give a much wider interval than 200. Games lost on time are excluded,
as described above.

In class, record wins, draws, losses, and unfinished games as observations. One
short round against a limited-strength Stockfish setting does not measure a rating.
Passing `verify_remusN.py` (with your version number in place of N) checks its reference cases, not Elo, and games using the
registered student engines exercise the typed code only for the positions reached.
