# The versions of Remus

The workshop builds four Remus engines, one for each of Days 1 to 4, and the student
pairs type every line of each one, copying the instructor's version:

| Version | What it adds | New lines to type | Total TLOC |
| --- | --- | ---: | ---: |
| [Remus1](Remus1/README.md) | Legal chess, a random legal move with no search | 193 | 193 |
| [Remus2](Remus2/README.md) | The evaluation, alpha-beta, quiescence, MVV-LVA ordering, a check extension, and iterative deepening | 161 | 342 |
| [Remus3](Remus3/README.md) | Killer moves, pawn structure, a transposition table, an opening book, aspiration windows, time management, null-move pruning, the principal variation, the bishop pair, and the rook table | 193 | 535 |
| [Remus4](Remus4/README.md) | Every other enhancement: draw sense, the endgame terms, late move reductions, principal variation search, pruning margins, static exchange evaluation, and the last evaluation terms | 159 | 694 |

New lines are the lines a student pair types for that version, and total lines count the
whole engine. Both leave out blank lines, comments, and docstrings. Before Remus2 the
student pairs finish [Remus1](Remus1/README.md) at the end of Day 1: the board, the
moves, and `uci.py` of Remus2, with a 12-line `search.py` that plays a random legal
move. Remus2 replaces that file, so its new lines are `evaluation.py` and the 93 lines
of its own `search.py` . Each version is the one before plus its enhancements, so a
student pair that finishes Remus2 types only the new lines of Remus3, and then only the
new lines of Remus4.

The compiled [Remus4-Rust](Remus4-Rust/README.md) , is Remus4 written in Rust, with a
larger fixed 512 MiB table and persistent search diagnostics. Student pairs do not type
it. Its [measured examples](Remus4-Rust/README.md#measured-examples) illustrate the
speed difference on the benchmark computer; they are not a universal
hardware-independent speed ratio.

This page lists every enhancement of Remus3 and Remus4, the file each one changes, the
code it adds, and the Elo it should gain, then shows what each of Remus2, Remus3, and
Remus4 adds.

## Engine defaults and source sizes

These are advertised engine defaults, not necessarily the values saved in a GUI entry.
Python Remus1 is a random mover; the other Python engines and Rust4/5 use one chess
search thread. Only Rust6 and Rust7 parallelize chess search.

For pondering in Lucas Chess R6, keep the engine's advertised `Ponder` default
false and select true in both its external-engine and tournament settings. The
explicit true override activates Lucas's pondering controller. A true advertised
default can leave that controller inactive even when the checkbox is checked.
The table reports engine defaults; the recommended Lucas Ponder selection is true.

| Engine | TLOC | Search threads | Transposition table | Advertised Ponder default | Diagnostic defaults |
| --- | ---: | --- | --- | --- | --- |
| [Remus1](Remus1/README.md) | 193 | 1 | None | Not supported | No persistent logs |
| [Remus2](Remus2/README.md) | 342 | 1 | None | Not supported | No persistent logs |
| [Remus3](Remus3/README.md) | 535 | 1 | 2,000,000-entry dictionary | Not supported | No persistent logs |
| [Remus4](Remus4/README.md) | 694 | 1 | 2,000,000-entry dictionary | Not supported | No persistent logs |
| [Remus4-Rust](Remus4-Rust/README.md) | 1,849 | 1 | Fixed 512 MiB | Not supported | SearchDiagnostics false |
| [Remus5](Remus5/README.md) | 1,430 | 1 | Fixed 512 MiB | false | PonderDiagnostics false; optional per-process JSON logs |
| [Remus5-Rust](Remus5-Rust/README.md) | 2,797 | 1 | Fixed 512 MiB | false | SearchDiagnostics false; PonderDiagnostics false; per-process text logs |
| [Remus6-Rust](Remus6-Rust/README.md) | 3,216 | 4, configurable 1 to 64 | Hash 512 MiB, choices 512/1024/2048 | false | SearchDiagnostics false; PonderDiagnostics false |
| [Remus7-Rust](Remus7-Rust/README.md) | 3,825 | 4, configurable 1 to 64 | Hash 512 MiB, choices 512/1024/2048 | false | SearchDiagnostics false; PonderDiagnostics false |

Rust diagnostics use per-process files in `logs/` beside each executable. Search
files are `search_<pid>_<session>.txt`; Rust5 through Rust7 also write
`ponder_<pid>_<session>.txt` when enabled. The matching suffix pairs the two streams.
See [search measurements](search_logging.md) for aggregation and retention.

Remus7-Rust additionally exposes `PonderHitMinPercent` , default 25, range 0 to 100.
Remus5 Python and Rust5 through Rust7 expose `LichessTablebase` , default true. The
table is shared by workers within a process, not across engine processes. All
book-enabled Python engines read their own file at startup; Rust embeds its book at
build time. Disable GUI books with explicit `<None>` in tournaments.

Each README uses the same main headings and file-table columns. Python comparisons use
the preceding Python version; Rust5 through Rust7 use the preceding Rust version. Rust4
has no preceding Rust edition and identifies Python Remus4 as its counterpart, not its
predecessor. A zero denotes an absent file. Supporting files and external checks do not
contribute to runtime-source TLOC.

## How to read the tables

- **Python lines in the feature catalog** are estimates for an individual enhancement; each may differ by up to a third. **TLOC in engine/file tables** is counted from current runtime source, excluding blank lines, standalone comments, and Python docstrings. Rust documentation comments are excluded too.
- **New lines to type** differs from net TLOC change. Remus2 replaces 12 search lines and adds 161 lines, increasing the total by 149. README file tables show current TLOC, predecessor TLOC, and the net change.
- **Elo** is the expected gain over the engine before the step. No games have measured
  these numbers for Remus2, Remus3, and Remus4 yet, so they are estimates, and gains
  overlap as the engine grows: two enhancements that both speed up the same search
  add less together than apart.
- Three rows marked **(foundation)** gain little alone but are needed by later rows.
- **Version** names the engine that first holds the step.

## The enhancements, grouped by the file they change

### (E) Evaluation Only: `evaluation.py`

Each row adds one term to `evaluate()` and touches no other file, so these are the
safest steps: a bug here makes Remus play worse, never illegally.

| Step | Enhancement | What it adds | Python lines | Elo | Version |
| --- | --- | --- | ---: | ---: | --- |
| E1 | Pawn structure | Penalties for doubled and isolated pawns, a bonus for passed pawns that grows as they advance | 30 | +49 | Remus3 |
| E2 | Mating bonus | When ahead in material, push the lone enemy king to the edge and bring the own king close, so K+Q and K+R against K end in mate | 12 | +19 | Remus4 |
| E3 | Tapered evaluation | A game phase from the pieces left, an endgame king table that brings the king to the center, and a blend of the middlegame and endgame scores | 21 | +40 | Remus4 |
| E4 | Bishop pair | +30 for a side that keeps both bishops | 7 | +10 | Remus3 |
| E5 | Rooks on open files | +20 for a rook on a file with no pawns, +10 on a file with only enemy pawns | 15 | +10 | Remus4 |
| E6 | Rook table | The rook piece-square table that Remus2 leaves out: rooks on the seventh rank and central files | 10 | +5 | Remus3 |
| E7 | Mobility | A few centipawns for each square a knight, bishop, rook, or queen reaches | 20 | +20 | Remus4 |
| E8 | Pawn race | In pawn endings, a passed pawn the enemy king cannot catch (the rule of the square) counts nearly a queen | 22 | +10 | Remus4 |
| E9 | King safety | A bonus for each pawn in front of a castled king, a penalty for open files next to it | 18 | +4 | Remus4 |

### (S) Search Only: `search.py`

These change which moves the search tries and how deeply. A bug here can lose games
badly without making an illegal move, so each step should keep a fixed-depth search's
best move on a few test positions while visiting fewer positions.

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
| N3 | Opening book | Play a stored opening line while the game follows one. The book holds UCI moves (`e2e4 e7e5 g1f3`), not SAN, so it needs no notation code | **New file** `opening_book.py`, plus `uci.py`, plus `opening_book.txt` (56 lines of data) | 28 | +58 | Remus3 |
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
| `evaluation.py` | | 68 | New: `mirrored_table()`, the four piece-square tables, the piece values, and `evaluate()` |
| `search.py` | 12 | 93 | Replaced: the random `think()` gives way to `ordered_moves()`, `count_node()`, `quiescence()`, `alpha_beta()`, and a `think()` that deepens one ply at a time |
| `uci.py` | 44 | 44 | The name line reads `id name Remus2` |
| `board.py`, `make_move.py`, `attacks.py`, `move_generation.py` | 137 | 137 | Unchanged |
| **Total** | **193** | **342** | |

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
| `evaluation.py` | 68 | 115 | `pawn_terms()`: doubled, isolated, and passed pawns; the rook table and the bishop pair |
| `search.py` | 93 | 173 | Killer moves, the table probe and store, null-move pruning, the principal variation in the info line, the aspiration window, and the time that grows when the best move changes |
| `uci.py` | 44 | 59 | The opening book, and a planned time with a hard limit |
| `transposition_table.py` | 0 | 22 | New: `store()` and `probe()` |
| `opening_book.py` | 0 | 21 | New: `load_book()` and `book_moves()`, with `opening_book.txt` |
| `make_move.py`, `attacks.py`, `move_generation.py` | 122 | 122 | Unchanged |
| **Total** | **342** | **535** | |

The expected gains add up to about +350 Elo over Remus2, less once they overlap.

## Remus4: everything else

Remus4 adds the other twelve steps, in the order a class can type them:

| Steps | Enhancements | Why here |
| --- | --- | --- |
| M1 | Draw sense | Reuses the position key; stops Remus drawing won games by repetition |
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
| `evaluation.py` | 115 | 195 | The endgame king table and taper, mobility, rooks on open files, the pawn race, king safety, the mating bonus, and insufficient material |
| `search.py` | 173 | 212 | Draw sense and its history around the null move, late move reductions, PVS, futility and delta pruning, and static exchange in quiescence |
| `uci.py` | 59 | 62 | The history of position keys for draw sense |
| `exchange.py` | 0 | 36 | New: `cheapest_attacker()` and `exchange_score()` |
| `attacks.py`, `move_generation.py`, `transposition_table.py`, `opening_book.py` | 142 | 142 | Unchanged |
| **Total** | **535** | **694** | |

The expected gains add up to about +195 Elo over Remus3, less once they overlap.

## Checking each version

The workshop designer's `verify_engines.py` checks all four engines without playing a
game: perft counts from six published positions, ten mate puzzles, five positions
searched to a fixed depth, and a short UCI conversation. The fixed-depth searches show
what each step buys. To depth 4 from the starting position Remus2 visits 17,360
positions, Remus3 2,944, and Remus4 3,005, and all three choose `g1f3` .

Each step should also be measured in games: the new engine plays the engine before it
with the same clock, and keeps the step only if it wins more than it loses.

## Experimental instructor engines

These versions live only in `engines_instructor/` and add no workshop days or student
typing assignments.

| Version | Language | Search and boundary features | Code lines |
| --- | --- | --- | ---: |
| [Remus5](Remus5/README.md) | Python | Tactical and positional enhancements, fixed 512 MiB table, optional root tablebase advice, UCI pondering | 1,430 |
| [Remus5-Rust](Remus5-Rust/README.md) | Rust | The same single-threaded search and boundary behavior | 2,785 |
| [Remus6-Rust](Remus6-Rust/README.md) | Rust only | Lazy SMP with independent root searches and one shared table | 3,205 |
| [Remus7-Rust](Remus7-Rust/README.md) | Rust only | YBWC, sibling splitting, persistent workers, and restricted descendant helping | 3,814 |

Remus5 adds check-aware quiescence, terminal-position handling, en passant capture
accounting, quiet-move history ordering, and
[positional evaluation](Remus5/README.md#positional-evaluation) . Backward pawns,
central holes, knight outposts, and nearby king cover use the same weights in Python and
the Rust engines 5 through 7. No Elo gain is established for these terms. Python source
counts exclude blank lines, comments, and docstrings; Remus5 contains 736 more code
lines than Remus4, not a student typing assignment.

Tablebase advice is confined to root positions with at most seven pieces and no castling
rights. The boundary preserves the halfmove clock, checks native move legality and
repetition concerns, and falls back to native search when the service is unavailable or
uncertain. `LichessTablebase` defaults to true. Fixed-depth Rust checks bypass the
service. Pondering uses native search only, waits for `go ponder` , and handles both
`ponderhit` and matching Lucas stop-and-go handoffs. `Ponder` defaults to false; enable
it in the GUI. A post-hit time floor protects fresh thinking time while respecting the
hard deadline. With a minute or more on the clock, a hit on a shallow, falling, or
changing result reserves half the plan and tries for one more depth, in Python Remus5
and in the Rust engines 5 through 7. In all four, a legal book continuation takes
priority over native search, including on a hit; a book ponder result waits without
native work or native ponder credit. Root advice can precede book selection on a normal
turn.

All four match complete position-command tokens, including supplied history and FEN
fields, and exclude the inter-command gap from credited work. Only an explicitly
stopped native ponder search without a prior ponderhit can earn credit. A normal timed
go must arrive within two seconds. New-game and option commands invalidate pending
credit; every go consumes it. Both go ponder and credited restarts bypass tablebases.
Diagnostics distinguish `hit_kind=uci`, `hit_kind=stop_go`, and `hit_kind=none`.

Remus5 Python and Rust each use a fixed 512 MiB table with four 16-byte entries per
bucket, multiply-shift indexing, and depth/age replacement. Python uses `mmap` and
`struct` ; Rust uses Windows virtual memory. Neither exposes Hash. Allocation failure
disables caching while preserving native search. Python Remus3 and Remus4 use
two-million-entry dictionaries; Remus1 and Remus2 have no table.

Remus4-Rust uses a fixed 512 MiB table without Hash. Remus6-Rust and Remus7-Rust expose
Hash sizes of 512, 1024, or 2048 MiB, defaulting to 512 MiB. All three offer persistent
[search measurements](search_logging.md) . The parallel engines default to four search threads
sharing one table. Their tables protect deeper results and refresh the age of useful
hits. Logs include cache fill, hits, replacements, active worker elapsed time, and
re-search/interrupted-search node costs. Lazy SMP does not populate the common
split-wait or split-cutoff fields; these zeros do not measure scheduler waits or helper
joins. Remus7 separates result waiting from cancellation waiting and reports the tasks,
nodes, and active elapsed time contributed through descendant helping. Only Remus6-Rust
and Remus7-Rust have parallel chess search.

## Speed techniques of the Rust engines 5 through 7

Remus5-Rust, Remus6-Rust, and Remus7-Rust share six techniques that make each position
cheaper to search. None of them changes which positions the search visits. With one
worker, a fixed-depth search prints the same score, the same count of positions, and the
same line of best play with or without them, and `evaluate()` returns the same score for
every position. Remus4-Rust uses the plain form of every step below, so each of its
files stays close to the Remus4 Python file of the same name.

| Engine | Search threads | Plain form | With the six techniques | Ratio |
| --- | ---: | ---: | ---: | ---: |
| Remus5-Rust | 1 | 4.38 | 5.59 | 1.28 |
| Remus6-Rust | 1 | 4.45 | 5.60 | 1.26 |
| Remus7-Rust | 1 | 4.36 | 5.72 | 1.31 |
| Remus6-Rust | 4 | 12.5 | 16.6 | 1.33 |
| Remus7-Rust | 4 | 11.7 | 16.9 | 1.44 |

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
T4 are measured in Remus5-Rust. T3, T5, and T6 are measured in the move loop of the
Remus4 search, which the three engines share. The gains overlap, so together they give
the ratios of the first table, not their sum. The six techniques add 49 code lines to
Remus5-Rust and 51 to each of Remus6-Rust and Remus7-Rust.

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
count of `evaluate()` , in `is_attacked()` , in move generation, and in the exchange
evaluation.

`rising_ray()` covers the four rays whose square numbers grow. It sets bit 63, the
square h1, among the pieces on the ray before it finds the first one. No rising ray
continues past h1, so the `RAYS` entry of h1 is empty: on an empty ray the lookup finds
h1, removes nothing, and returns the whole ray, with no test. `falling_ray()` does the
same for the four rays whose square numbers shrink, with bit 0, the square a8. The
result is the same set of squares in every case.

### T2: Legal-move shortcut

The quiescence search of Remus5 asks at every position whether the side to move has a
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
loop, `place_next(entries, index)` puts the right entry at `index` . For the first three
indexes (`EARLY_PICKS`) it makes one pass over the entries that are left and swaps the
largest into place. At the fourth index it sorts the rest once. No two entries are
equal, because the move is part of each entry, so the moves come out in exactly the
order of the full sort.

Two places need every move in order at once and sort the list themselves. The check
evasions of `quiescence()` are few and are all tried. In Remus7-Rust,
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
calls `evaluate()` at every position before those tests. The engines 5 through 7 compute
`needs_score` from the same conditions and call `evaluate()` only when it is true. A
position at depth 2 and a position in check never evaluate, and at depth 3 and up
neither does the root or the position right after a null move.

### T6: Table prefetch

The transposition table holds 512 MiB, far more than the processor's cache, so reading
the bucket of a position takes roughly 100 nanoseconds, about half of the time the
engine spends on an average position. The search knows which bucket the next position
needs as soon as `make_move()` returns that position's key.

`Table::prefetch(key)` asks the processor to start loading the bucket and returns at
once. `alpha_beta()` calls it right after `make_move()` , so the legality test and the
check test of the move run while the bucket travels from memory, and the child's
`probe()` waits for less. It calls prefetch only at depth 2 and up: a child of a depth-1
position goes to quiescence unless it is in check, quiescence does not use the table,
and a prefetch there costs more than it saves. The null-move search prefetches its
position the same way, and in Remus7-Rust so does `search_split_move()` when a worker
takes a move from a split. A prefetch reads no value, so the shared table of the
parallel engines needs no added synchronization.

### Checking that the search is the same

- `verify_rust.py --engine 5` compares Remus5-Rust with Python Remus5: perft from six
  positions, `evaluate()` on 19,574 positions from random games, and every `info`
  line and best move of fixed-depth searches with and without a game history.
- `verify_rust.py --engine 6` and `--engine 7`, each with `--baseline`, compare
  one-worker searches with an executable built from the plain form.
- A second set of 22 positions (checks, en passant, promotions, stalemates, the
  fifty-move clock, and a repeated position in the game history) gives the same
  `info` lines in the plain form and with the six techniques in all three engines.
- `verify_remus7.py`, `verify_remus7_helping.py`, and `verify_remus7_diagnostics.py`
  check the parallel search of Remus7-Rust: legal results and lines of play, splits,
  stops, and worker accounting.
- `verify_positional_evaluation.py`, `verify_transposition_tables.py`,
  `verify_cache_uci.py`, `verify_tablebase.py`, `verify_ponder_control.py`, and
  `verify_pondering.py` pass on Remus5-Rust with the techniques in place.

### Techniques that measure too little to keep

Each of these also leaves the search unchanged. The engines 5 through 7 leave them out
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
