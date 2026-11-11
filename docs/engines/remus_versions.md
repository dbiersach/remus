# The versions of Remus

The workshop builds four Remus engines, one for each of Days 1 to 4, and the student
pairs type every line of each one, copying the instructor's version:

| Version | What it adds | New lines | Total lines |
| --- | --- | ---: | ---: |
| [Remus1](Remus1/README.md) | Legal chess, a random legal move with no search | 193 | 193 |
| [Remus2](Remus2/README.md) | The evaluation, alpha-beta, quiescence, MVV-LVA ordering, a check extension, and iterative deepening | 161 | 342 |
| [Remus3](Remus3/README.md) | Killer moves, pawn structure, a transposition table, an opening book, aspiration windows, time management, null-move pruning, the principal variation, the bishop pair, and the rook table | 193 | 535 |
| [Remus4](Remus4/README.md) | Every other enhancement: draw sense, the endgame terms, late move reductions, principal variation search, pruning margins, static exchange evaluation, and the last evaluation terms | 159 | 694 |

New lines are the lines a student pair types for that version, and total lines
count the whole engine. Both leave out blank lines, comments, and docstrings. Before
Remus2 the student pairs finish [Remus1](Remus1/README.md) at the end of Day 1: the
board, the moves, and `uci.py` of Remus2, with a 12-line `search.py` that plays a
random legal move. Remus2 replaces that file, so its new lines are `evaluation.py`
and the 93 lines of its own `search.py`. Each version is the
one before plus its enhancements, so a student pair that finishes Remus2 types only
the new lines of Remus3, and then only the new lines of Remus4.

The compiled [Remus4-Rust](Remus4-Rust/README.md), is Remus4 written in Rust, with a
larger table. It adds no other enhancement and nobody types it: it searches more than
100 times as many positions per second, to show what speed buys.

This page lists every enhancement of Remus3 and Remus4, the file each one changes, the
code it adds, and the Elo it should gain, then shows what each of Remus2, Remus3, and
Remus4 adds.

## How to read the tables

- **Python lines** count code only, estimated from the finished code. Expect each
  count to be off by up to a third.
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
| `transposition_table.py` | | 22 | New: `store()` and `probe()` |
| `opening_book.py` | | 21 | New: `load_book()` and `book_moves()`, with `opening_book.txt` |
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
| `exchange.py` | | 36 | New: `cheapest_attacker()` and `exchange_score()` |
| **Total** | **535** | **694** | |

The expected gains add up to about +195 Elo over Remus3, less once they overlap.

## Checking each version

The workshop designer's `verify_engines.py` checks all four engines without playing a
game: perft counts from six published positions, ten mate puzzles, five positions
searched to a fixed depth, and a short UCI conversation. The fixed-depth searches show
what each step buys. To depth 4 from the starting position Remus2 visits 17,360
positions, Remus3 2,944, and Remus4 3,005, and all three choose `g1f3`.

Each step should also be measured in games: the new engine plays the engine before
it with the same clock, and keeps the step only if it wins more than it loses.

## Experimental instructor engines

These versions live only in `engines_instructor/` and do not add workshop days or
student typing assignments. Each has a matching Rust executable.

| Version | What it adds | Python code lines | Net increase |
| --- | --- | ---: | ---: |
| [Remus5](Remus5/README.md) | Check-aware quiescence, terminal-position handling, en passant capture accounting, quiet-move history ordering, and a fixed 1 GiB bucket table | 786 | 92 over Remus4 |
| [Remus6](Remus6/README.md) | Root-only Lichess tablebase advice, native legality validation, bounded networking, cache, cooldown, offline option, and native-search fallback | 964 | 178 over Remus5 |

The net increase measures source length, excluding blank lines, comments, and
docstrings. It is not a student typing count. Remus6 retains the 255-line Remus5
`search.py` unchanged; `tablebase.py` holds 158 code lines and `uci.py` holds 82.
The other native Python modules and opening book match Remus5, with a fixed
2 GiB table in Remus6 and 1 GiB in Remus5.

Remus6 and [Remus6-Rust](Remus6-Rust/README.md) serialize FEN at the Lichess API
boundary. They query only root positions with seven or fewer pieces and no castling
rights, preserve the fifty-move counter, and validate the returned move natively.
The first ranked move is accepted only with a definite compatible result and no
repetition concern. Unavailable or uncertain results use native search. The UCI
option `LichessTablebase` defaults to true; false disables lookups. Rust's fixed-depth
checks bypass the service. No Elo gain is established by tablebase or perft checks.

Remus5 and Remus5-Rust use fixed 1 GiB transposition tables; Remus6 and
Remus6-Rust use fixed 2 GiB tables. Both languages store four 16-byte entries
per bucket, select buckets with multiply-shift indexing, and replace entries by
depth and search age. Python uses `mmap` and `struct`; Rust uses Windows virtual
memory. No UCI Hash option is advertised. An unavailable allocation disables
caching while native search remains available. Remus3 and Remus4 Python use
two-million-entry dictionaries, Remus4-Rust uses 512 MiB, and Remus1 and Remus2
have no transposition table.
