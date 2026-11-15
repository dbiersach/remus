# The versions of Remus

Remus grows from a random legal move chooser into a family of searching chess
engines. Student pairs build Remus1 to Remus4 in Python over four workshop days.
Remus5 to Remus8 are supplied Rust engines that extend the same ideas through
compiled execution, additional chess knowledge, and parallel search.

Follow the learning sequence, then use the reference tables to explore a technique:

1. [The Remus progression at a glance](#the-remus-progression-at-a-glance)
2. [Building a chess engine: Remus1 to Remus4](#building-a-chess-engine-remus1-to-remus4)
3. [Making the engine faster: Remus5 to Remus8](#making-the-engine-faster-remus5-to-remus8)
4. [Checking that the engine works](#checking-that-the-engine-works)
5. [Measuring speed and playing strength](#measuring-speed-and-playing-strength)
6. [Running Remus on the workshop laptops](#running-remus-on-the-workshop-laptops)
7. [Enhancement reference](#enhancement-reference)

## The Remus progression at a glance

The progression develops five connected ideas: legal play, better decisions,
faster execution, parallel search, and evidence from tests and games.

**TLOC means Total Lines of Code.** It counts current runtime source, excluding
blank lines, standalone comments, and Python docstrings. Rust documentation
comments are excluded too. Supplied verifiers are not part of these totals.
The supplied data files do not have to be typed and don't represent code.

### Engine defaults and Total Lines of Code (TLOC)

The table lists source sizes and engine defaults alongside recorded Elo estimates.
Remus7 and Remus8 default to four search threads and 512 MiB Hash; their Elo
measurements used 16 threads and a 2 GiB table on the benchmark computer.
Those ratings do not measure the workshop laptop configuration. Rank compares
numbers with human rating bands, not earned FIDE titles.

Ponder and Diagnostics show how to enable optional features; their advertised
check options default to false. The [laptop settings section](#running-remus-on-the-workshop-laptops)
explains saved GUI overrides and resource use.

| Engine | ELO | Rank | Language | TLOC | Search threads | Transposition table | Ponder | Diagnostics |
| --- | ---: | --- | --- | ---: | --- | --- | --- | --- |
| [Remus1](../Day%201%20-%20Remus1%20plays%20legal%20chess/Remus1/Remus1.md) | 300 | Below Class D | Python | 189 | 1 | None | Not supported | No persistent logs |
| [Remus2](../Day%202%20-%20Remus2%20searches%20ahead/Remus2/Remus2.md) | 1840 | Class A player | Python | 300 | 1 | None | Not supported | No persistent logs |
| [Remus3](../Day%203%20-%20Remus3%20adds%20the%20biggest%20gains/Remus3/Remus3.md) | 2086 | Expert | Python | 484 | 1 | 500 MiB | Not supported | No persistent logs |
| [Remus4](../Day%204%20-%20Remus4%20adds%20everything%20else/Remus4/Remus4.md) | 2195 | Expert | Python | 630 | 1 | 500 MiB | Not supported | No persistent logs |
| [Remus5](../rust_engines/Remus5/remus5.md) | 2738 | Super Grandmaster | Rust | 1,849 | 1 | 512 MiB | Not supported | Set `SearchDiagnostics` = True |
| [Remus6](../rust_engines/Remus6/remus6.md) | 2809 | Super Grandmaster | Rust | 2,893 | 1 | 512 MiB | Set `Ponder` = True | Set `SearchDiagnostics` or `PonderDiagnostics` = True |
| [Remus7](../rust_engines/Remus7/remus7.md) | 3026 | Super Grandmaster | Rust | 3,312 | 4 | 512 MiB | Set `Ponder` = True | Set `SearchDiagnostics` or `PonderDiagnostics` = True |
| [Remus8](../rust_engines/Remus8/remus8.md) | 3099 | Super Grandmaster | Rust | 3,921 | 4 | 512 MiB | Set `Ponder` = True | Set `SearchDiagnostics` or `PonderDiagnostics` = True |

See [benchmark conditions and confidence intervals](#about-the-remus-elo-estimates)
for the scope of the rating estimates. Remus1's figure is a rough estimate.

## Building a chess engine: Remus1 to Remus4

The learning sequence starts with rules, adds a way to compare positions, then
improves which continuations the engine explores. Student pairs type from period
screenshots and carry their completed runtime modules into the next day's engine
folder. They keep that folder's supplied guide, launcher, verifier, and data.

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

From Remus2 onward, each Python engine guide's file table compares its version
with the preceding version.
A zero denotes an absent file. The file tables below show counted source lines;
the enhancement reference gives approximate per-feature typing costs.

### Remus1: legal chess before search

Remus1 represents the board, generates candidate moves, applies each candidate to
a copied position, and rejects moves that leave its king attacked. It chooses
randomly among the legal moves. Its 189 lines establish the rules and the UCI
connection to Lucas Chess without searching ahead. UCI, the Universal Chess
Interface, is the text protocol through which Lucas Chess asks an engine for moves.

That provides a working game before students add a preference among legal moves.
In Remus2, evaluation assigns a score to a position and search compares possible
continuations. Alpha-beta search skips branches that cannot change the best move.
Quiescence search follows captures beyond the main search depth to avoid scoring
an unfinished exchange. MVV-LVA means most valuable victim, least valuable
attacker: try a pawn capturing a queen before a queen capturing a pawn.

### Remus2: looking ahead

Remus2 is the engine every later step builds on. It keeps Remus1's rules and replaces
the random choice with a score and a search, in the order Day 2 types them:

| Part | Enhancement | Why it comes here |
| --- | --- | --- |
| Material and piece-square tables | `evaluate()`: 100 for a pawn, 300 for a knight or bishop, 500 for a rook, 900 for a queen, plus a bonus or penalty for each piece's square | A search needs a score to compare positions, so the evaluation comes first |
| MVV-LVA ordering | `ordered_moves()` tries the best capture first: pawn takes queen before queen takes pawn | Alpha-beta cuts off sooner when the best move comes first |
| Quiescence | `quiescence()` plays out the captures at the end of the search before it uses the score | A search that stops in the middle of an exchange counts a queen as won when it is about to be taken back |
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

### Remus3: the biggest gains for the fewest lines

A transposition table remembers search results so the engine can reuse them. An
opening book supplies stored opening lines while the game follows a known line.

The identifiers below link this learning sequence to the [enhancement reference](#the-enhancements-grouped-by-the-file-they-change).
Remus3 holds eleven steps, which split the new lines about evenly between Remus3 and
Remus4, so each takes one day of typing:

| Step | Enhancement | Why it comes early |
| --- | --- | --- |
| S1 | Killer moves | The best gain for the fewest lines, and it teaches why move order matters to alpha-beta |
| E1 | Pawn structure | The largest evaluation gain; students see Remus value passed pawns |
| N1, N2 | Position key, then the transposition table | The largest single gain; the key comes first because the table needs it |
| N3 | Opening book | Variety and sound openings from a supplied data file, with few lines to type |
| S8, M2 | Aspiration windows, then time management | They help most once the table makes each iteration fast, and they finish the iterative deepening loop |
| S2 | Null-move pruning | The largest increase in search depth for about a dozen lines |
| S4 | Principal variation | The `info` line shows the predicted continuation read from the table |
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

### Remus4: everything else

Remus4 adds the other twelve steps, in the order a class can type them:

| Steps | Enhancements | Why here |
| --- | --- | --- |
| M1 | Draw sense | Reuses the position key to score repetitions as draws during search |
| E2, E3 | Mating bonus, then tapered evaluation | Reward positions that drive the lone enemy king to the edge and bring its own king toward the center |
| S3 | Late move reductions | Deeper search with about a dozen lines |
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

The main themes are evaluation (what makes a position desirable), search (which
continuations deserve attention), reuse (remembering previous work), and game
management (time, openings, and draws). The [reference catalogs](#enhancement-reference)
keep the E, S, M, and N identifiers used in the progression tables.

## Making the engine faster: Remus5 to Remus8

### Supplied Rust engines

Remus5 through Remus8 are Rust-only engines. Their source, executables, overview
guides, and line-by-line references live under `rust_engines/RemusN`. Students do
not type them. The [overview table](#engine-defaults-and-total-lines-of-code-tloc)
compares their source sizes, settings, and recorded ratings.

| Version | What it adds |
| --- | --- |
| [Remus5](../rust_engines/Remus5/remus5.md) | Remus4 algorithms implemented in Rust with a larger bucketed table |
| [Remus6](../rust_engines/Remus6/remus6.md) | Enhanced single-threaded search and positional evaluation, root tablebase advice, and UCI pondering |
| [Remus7](../rust_engines/Remus7/remus7.md) | Lazy SMP (symmetric multiprocessing): search threads share a table |
| [Remus8](../rust_engines/Remus8/remus8.md) | Young Brothers Wait Concept (YBWC): search the first move before splitting the remaining moves, with restricted worker helping |

### Remus5: the Python ideas in compiled code

[Remus5](../rust_engines/Remus5/remus5.md) implements Remus4's core algorithms in
Rust, with a larger fixed 512 MiB bucketed transposition table and persistent
search diagnostics.
Its [measured examples](../rust_engines/Remus5/remus5.md#measured-examples) illustrate
the speed difference on the benchmark computer; they are not a universal,
hardware-independent speed ratio.

### Remus6: stronger evaluation and less work per position

Remus6 adds check-aware quiescence, terminal-position handling, en passant capture
accounting, quiet-move history, backward-pawn and central-hole penalties, knight
outposts, and nearby king cover. These additions are shared by Remus7 and Remus8.
Their presence alone does not establish an Elo gain.

Remus6 also introduces the seven speed techniques retained in Remus7 and Remus8.
They reduce repeated calculations, avoid unnecessary move generation and sorting,
and overlap memory access with useful work. Their purpose is to make the same
native search cheaper, rather than change its chess choices. Remus5 uses the
plain forms so its modules remain close to their Python counterparts.

The [T1 to T7 reference](#rust-speed-techniques-t1-to-t7) explains each technique.
The [measurement section](#speed-techniques-of-the-rust-engines-6-through-8)
separates the combined T1 to T6 results from the later T7 measurements.

### Remus7 and Remus8: sharing search across cores

Remus7 uses Lazy SMP: workers search with private search state and share results
through one transposition table. Remus8 uses the Young Brothers Wait Concept:
search the first move before distributing remaining moves among workers. Workers
can also help with eligible outstanding tasks.

The common goal is to search useful continuations sooner. Coordination, repeated
work, and competition for memory mean that four threads need not deliver four
times the speed of one. Comparing these approaches requires both elapsed-time
measurements and checks that their results remain legal.

### Tablebases and pondering

A tablebase supplies exact endgame information for covered positions. Its advice
enters at the root, where the engine chooses its next move, rather than at every
position inside the search.

Remus6 to Remus8 can request root tablebase advice for eligible positions with
at most seven pieces and no castling rights. They preserve the actual halfmove
clock and en passant square, validate suggested moves with native rules, and fall
back to native search on failures or uncertain responses. Fixed-depth checks and
ponder searches bypass the service.

Pondering uses the opponent's thinking time to prepare search results. Remus6 to
Remus8 support GUI-controlled pondering; the [UCI guide](uci.md#pondering) explains
the command sequence and the [engine guides](#engine-defaults-and-total-lines-of-code-tloc)
explain how a matching prediction affects the next move's budget.

## Checking that the engine works

### Checking each version

Run `verify_remusN.py` in the day's engine folder at the scheduled checkpoints. It prints the
checks and expected values, reports any problems, and ends with the version's
success line when its reference cases pass. Running an individual engine module
only checks that it and its imports load; it does not test all its functions.

The designer's separate native checks cover published legal-move counts (perft), mate
puzzles, fixed-depth searches, and UCI exchanges. Recorded depth-4 searches from
the starting position visit 17,360 nodes in Remus2, 2,944 in Remus3, and 3,005 in
Remus4, with `g1f3` as the best move in each. These counts compare one position
at one depth; fewer nodes alone do not establish faster play or greater strength.

A strength comparison needs enough games under controlled conditions, with colors
balanced and uncertainty reported. Winning more games than losing in a short
round is not sufficient evidence to accept an enhancement. The
[Elo measurement section](#about-the-remus-elo-estimates) describes the recorded
rating experiments.

### Checking that the search is the same

The designer's regression checks compare implementations on fixed inputs. For
speed changes intended to preserve search behavior, one-worker searches compare
scores, node counts, principal variations (predicted lines of play), and best moves.
The recorded checks cover:

- Remus6 against the archived Python reference: legal-move counts from six
  published positions, evaluation on 19,574 positions from random games, and
  every `info` line and best move of fixed-depth searches with and without history.
- Remus7 and Remus8 against preserved native executables, using one worker.
- A second set of 22 positions covering checks, en passant, promotions, stalemates,
  the fifty-move clock, and repeated positions. The plain form and the T1 to T6
  implementations return the same `info` lines in all three engines.
- Remus8 parallel search: legal moves and lines of play, split work, cancellation,
  and worker accounting.
- Remus6 positional evaluation, transposition tables, UCI cache behavior,
  tablebase fallback, and pondering with the speed techniques in place.

These checks establish agreement on their cases, not a proof for every possible
position. Parallel workers can visit different nodes because their scheduling
changes. Correctness checks, speed measurements, and playing-strength experiments
answer different questions.

## Measuring speed and playing strength

NPS measures search throughput. Fixed-depth comparisons examine how quickly an
implementation completes specified work. Elo experiments estimate strength from
game results against opponents. A round robin compares versions directly. Keep
those measurements separate and read each result with its test conditions.

### Nodes per second across the Remus engines

Nodes per second (NPS) counts search visits to positions, including quiescence
search. A node does not necessarily call the evaluation function, and the same
position can be visited more than once. These recorded results give a sense of
scale; they use different positions and search depths, so they are not a controlled
speed comparison across all versions. Rates are rounded.

**Kiwipete** is the nickname of a standard chess test position by Peter McKenzie.
White has 48 legal moves, including captures and both castling moves. Its many
possible continuations exercise move generation and king-safety checks; deeper
lines also include en passant and promotions. The [Chess Programming Wiki shows
the position and its published move counts](https://chessprogramming.org/Perft_Results#Position_2).

Remus uses it in two ways. A legal-move counting test, called **perft**, explores
every legal continuation to a chosen depth: Kiwipete has 48 one-move sequences,
2,039 two-move sequences, and 97,862 three-move sequences. Here a move means one
side's turn, also called a ply. Matching these counts checks the move generator
on that position. A search benchmark instead asks the engine to choose a move
from the same position and measures its work and elapsed time. Search skips
branches and can extend others, so its node count is not the perft count.
Kiwipete supplies a repeatable, demanding position; the suite also includes
openings and endings because one position cannot establish overall speed or
playing strength.

| Engine | Search threads | Approximate nodes per second | Recorded benchmark |
| --- | ---: | ---: | --- |
| Remus1 | 1 | Not applicable | Chooses a random legal move without searching ahead |
| Remus2 | 1 | 72,900 | Five-position suite through depth 4, 15 runs |
| Remus3 | 1 | 44,100 | Five-position suite through depth 4, 15 runs |
| Remus4 | 1 | 19,600 | Five-position suite through depth 4, 15 runs |
| Remus5 | 1 | 2,440,000 | Kiwipete through depth 7: 292,713 nodes in 0.12 seconds |
| Remus6 | 1 | 5,590,000 | Six-position fixed-depth suite with T1 to T6 |
| Remus7 | 1 | 5,600,000 | Six-position fixed-depth suite with T1 to T6 |
| Remus8 | 1 | 5,720,000 | Six-position fixed-depth suite with T1 to T6 |

The Python figures use the shortened engines' saved benchmark results: total nodes
across all 15 runs divided by their total search time, excluding imports. Each run
searches the starting position, an Italian opening, Kiwipete, a rook ending, and a
pawn race with iterative deepening through depth 4. Remus1's saved perft timing
measures legal move generation, so it is not included as search NPS.

The Remus5 rate is calculated from its published
[fixed-depth example](../rust_engines/Remus5/remus5.md#measured-examples); the recorded
0.12-second time limits its precision. Remus6 to Remus8 use the recorded rates in the
[six-technique comparison](#speed-techniques-of-the-rust-engines-6-through-8) in the next subsection.
That comparison also reports four-thread rates of 16,600,000 NPS for Remus7 and
16,900,000 NPS for Remus8. These are not measurements at the 16-thread settings used
for their Elo tests, and no 16-thread NPS is inferred from them.

More work per node can lower NPS while improving move choices or reducing the number
of nodes needed. The lower Python rates in later versions therefore do not imply
weaker play. Hardware, position, depth, table settings, and thread count all affect
NPS; it does not by itself measure Elo. These figures describe the recorded builds,
not a fresh benchmark of any subsequent engine edits.

### Speed techniques of the Rust engines 6 through 8

The following comparison measures T1 to T6 together. With one worker, the tested
builds return the same scores, node counts, and principal variations.

| Engine | Search threads | Plain form | With the six techniques | Ratio |
| --- | ---: | ---: | ---: | ---: |
| Remus6 | 1 | 4.38 | 5.59 | 1.28 |
| Remus7 | 1 | 4.45 | 5.60 | 1.26 |
| Remus8 | 1 | 4.36 | 5.72 | 1.31 |
| Remus7 | 4 | 12.5 | 16.6 | 1.33 |
| Remus8 | 4 | 11.7 | 16.9 | 1.44 |

The numbers are millions of search nodes per second on the workshop designer's computer,
summed over six fixed-depth searches: the starting position, an Italian opening, the
Kiwipete test position, a quiet middlegame, a rook ending, and a pawn race. With one
worker both builds search the same 34 million positions, so the ratio is also the ratio
of the two run times. With four search threads the threads reach the shared table in a
different order on every run, the two builds visit different positions, and only the
rate compares. The ratios cover T1 to T6; T7 is measured separately in the [technique reference](#rust-speed-techniques-t1-to-t7).

### About the Remus ELO Estimates

FIDE gives titles only to people, so no engine can earn one. The ratings in the
[engine table](#engine-defaults-and-total-lines-of-code-tloc) summarize the benchmark described
here; they are not human FIDE ratings or expected results for a workshop test. The class labels are numerical comparisons only;
sharing a number does not establish equivalent strength against people. Remus2, Remus3,
and Remus4 each played 200 games against Stockfish held to a set
strength (its UCI_Elo setting), at 15 minutes per side with no increment, half with
White and half with Black, on one fast processor core per game. Each engine met
Stockfish at six levels between 1320 and 2950, and its rating is the one that best
explains its score against those levels. The 27 games that Stockfish lost on time
are left out, because Stockfish ran out of time in long endgames whatever the
position on the board.

Reported 95% confidence intervals are Remus2: 1785 to 1895; Remus3: 2029 to 2143; Remus4: 2133 to 2257; Remus5: 2591 to 2885; Remus6: 2697 to 2921; Remus7: 2815 to 3236; Remus8: 2900 to 3297.

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

Remus5 and Remus6 played against limited-strength Stockfish at 15 minutes per side
with no increment at 2500, 2650, 2800, 2950, and 3100, half with each color:
Remus6 played 50 games, 10 at each level, and Remus5 played 29, five to seven at
each level. Each searches on one thread, so each played eight games at once, each on its own
fast core shared with that game's Stockfish, as in the 200-game runs. A full-strength
Stockfish judge on the slow cores ended a game when one side stayed ten pawns ahead
for three looks in a row or the score stayed level late in the game, and it scored
the final position of any time forfeit; the limited Stockfish players used a 200 ms
Move Overhead, and none lost on time. Remus6 pondered on Stockfish's time and used
its default root tablebase lookups; Remus5 offers neither.

Remus7 and Remus8 each played 20 games under the same time control, with pondering
and tablebase lookups. They used a 2048 MiB table and 16
search threads and played one game at a time. In Remus7's games Stockfish ran alone
on one fast core; in Remus8's games the operating system placed both engines, so
Stockfish could share a core with Remus8 and play slightly below its setting. Each
ladder holds one game with each color at 2200, 2400, 2600, 2800, 3000, and 3190,
Stockfish's highest setting, then eight games at the two levels 100 points below
and above the rating from those first twelve games. Twenty games give a much wider
interval than 200. In these 20-game runs, games lost on time are left out, as
described above.

In class, record wins, draws, losses, and unfinished games as observations. One
short round against a limited-strength Stockfish setting does not measure a rating.
Passing `verify_remusN.py` (with your version number in place of N) checks its reference cases, not Elo, and games using the
registered student engines exercise the typed code only for the positions reached.

### Round robin of Remus1 to Remus8

Every version played every other version twice, once with each color: 56 games at
15 minutes per side with no increment. Each engine kept its default settings, except
that Remus6, Remus7, and Remus8 pondered on their opponent's time; Remus7 and Remus8
used their default four search threads and 512 MiB tables. Four games ran at once,
each on two fast processor cores shared by both engines. A full-strength Stockfish
judge ended 38 decided games early, as in the rating runs, and no game was lost on
time. Each engine played 14 games, so 14 points is a perfect score. Equal points are
ranked by the result between the two engines.

| Rank | Engine | Points | Wins | Draws | Losses |
| ---: | --- | ---: | ---: | ---: | ---: |
| 1 | Remus8 | 11.5 | 10 | 3 | 1 |
| 2 | Remus7 | 11.5 | 10 | 3 | 1 |
| 3 | Remus6 | 10.5 | 9 | 3 | 2 |
| 4 | Remus5 | 10.5 | 9 | 3 | 2 |
| 5 | Remus4 | 5.5 | 5 | 1 | 8 |
| 6 | Remus3 | 4.5 | 4 | 1 | 9 |
| 7 | Remus2 | 2 | 2 | 0 | 12 |
| 8 | Remus1 | 0 | 0 | 0 | 14 |

Each row gives the points that engine scored in its two games against the engine
named at the top of the column.

| Engine | Remus8 | Remus7 | Remus6 | Remus5 | Remus4 | Remus3 | Remus2 | Remus1 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Remus8 | | 1.5 | 1 | 1 | 2 | 2 | 2 | 2 |
| Remus7 | 0.5 | | 2 | 1 | 2 | 2 | 2 | 2 |
| Remus6 | 1 | 0 | | 1.5 | 2 | 2 | 2 | 2 |
| Remus5 | 1 | 1 | 0.5 | | 2 | 2 | 2 | 2 |
| Remus4 | 0 | 0 | 0 | 0 | | 1.5 | 2 | 2 |
| Remus3 | 0 | 0 | 0 | 0 | 0.5 | | 2 | 2 |
| Remus2 | 0 | 0 | 0 | 0 | 0 | 0 | | 2 |
| Remus1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | |

The table separates the Python engines from the Rust engines: every Rust engine
won all eight of its games against Remus1 to Remus4, and each Python version beat
every Python version below it. Among the four Rust engines, two games per pairing
cannot rank them reliably; the top four finished within one point of each other.

<a id="why-remus5-stays-close-to-remus8"></a>

### Why more speed does not produce proportional playing strength

Remus5 is the Rust implementation of workshop Remus4: one search thread and the Remus4
evaluation. In the recorded benchmark, Remus8 used 16 search threads, a 2048 MiB table,
pondering, tablebase lookups, and positional evaluation, yet it measured only about 360
Elo higher, 3099 against 2738. The following explanations interpret these results; the
matches do not isolate each cause.

One explanation is diminishing returns from deeper search. The rule of thumb used in
this comparison is roughly 50 to 70 Elo per doubling of search speed at these depths,
with smaller gains as the search gets deeper; it is not a measured conversion for every
Remus version. Sixteen threads do not search 16 times faster, because parallel alpha-
beta spends work on positions that a single thread would cut off. The illustrative
estimate of a 5 to 8 times speedup gives about three doublings, or roughly 150 to 200
Elo under that rule of thumb. The step from Python Remus4 to Rust Remus5 is more than
100 times faster, about seven doublings from a much shallower search, and it gains about
540 Elo, from 2195 to 2738.

Another explanation is that the benefits of additional features overlap. Stockfish
played the reply that Remus predicted for about 41% of Remus's moves, so fewer than half
the moves produced a matching predicted reply. That frequency alone does not measure
pondering's total benefit. A larger table helps only until it holds the positions the
search reaches: fixed-depth searches find nearly the same node counts with 512 MiB, 1
GiB, and 2 GiB tables. The evaluation sets the ceiling, because a deeper search finds
the best move according to the engine's evaluation. The positional terms of Remus6 to
Remus8 are modest. Remus6 shows how small these steps are: it adds pondering, tablebase
lookups, the positional terms, and the speed techniques to a single thread, and its 2809
sits about 70 points above Remus5's 2738, inside the 95% intervals of both engines.

Measurement uncertainty also matters. Stockfish's UCI_Elo setting stops at 3190, and
Remus8 scores 6 of 11 points at 2990 and above, so the ladder has little room above it.
Remus5's 29 judged games give an interval of about 150 points on each side and Remus8's
20 games about 200, against about 60 for 200 games. The reported comparison places the
gap at approximately 110 to 610 points. That range describes the uncertainty in this
comparison; it does not guarantee the true difference or predict results on the workshop
laptops.

## Running Remus on the workshop laptops

### Hash, threads, and saved settings

The workshop laptops have 8 GB of RAM and four CPU cores (two CPUs with two cores
each). Remus7 and Remus8 default to Hash 512 MiB and Threads 4. Student setup's
registration script sets both values in their Lucas Chess external-engine entries,
replacing older selections while preserving other options. Existing tournament
entries are separate; select Hash 512 and Threads 4 there too.

The table is shared by the workers in one process, not across processes. Two
simultaneous Remus7 or Remus8 processes request 1 GiB of hash memory and eight
search threads in total. Stacks, other engine state, Windows, and Lucas Chess need
additional memory. Overlapping searches, including pondering, share the laptop's
four cores. See the [Lucas Chess settings guide](lucas_chess_guide.md#supplied-rust-engine-settings).

Remus1 chooses a random legal move. Remus2 through Remus6 use one chess search
thread. Remus7 and Remus8 parallelize search. The [overview table](#engine-defaults-and-total-lines-of-code-tloc)
lists the settings for all eight versions.

### Pondering, tablebases, and opening books

Remus6 to Remus8 advertise `Ponder` with default false. For pondering in Lucas
Chess R6, select true in the external-engine and tournament entries so Lucas
activates its pondering controller. A true advertised default can leave that
controller inactive even when its checkbox is selected. Remus1 to Remus5 do not
support pondering. Remus8 also exposes `PonderHitMinPercent`, default 25, range
0 to 100. See the [UCI guide](uci.md#pondering) for handoff behavior.

`LichessTablebase` defaults to true in Remus6 to Remus8. The engine falls back to
native search when advice is unavailable, so it can play offline. Fixed-depth
checks and ponder searches bypass the service.

Book-enabled Python engines read their supplied opening-book file at startup;
Rust embeds its book at build time. Every book move is checked for legality.
Disable GUI opening books with explicit `<None>` in tournament entries so Remus
uses its own move selection. Book pondering waits without claiming native search
credit. See [Remus opening books](lucas_chess_guide.md#remus-opening-books).

### Diagnostics

Diagnostic options default to false. `SearchDiagnostics` enables search logging;
Remus6 to Remus8 also offer `PonderDiagnostics`. Logs are written beside the Rust
executable in per-process files under `logs/`. See [search measurements](search_logging.md)
for filenames, diagnostic fields, and retention. Normal UCI depth, score, and move
output is available without enabling these logs.

## Enhancement reference

Use this section to look up the implementation behind a progression step.
The Python catalogs group enhancements by the files they change; the Rust catalog
groups techniques that reduce execution cost. Each detailed explanation appears
here once, while the earlier sections explain when and why it enters the engine.

### How to read the tables

- **Python lines in the feature catalog** are estimates for an individual enhancement; each may differ by up to a third. The overview table uses counted TLOC, rather than these per-enhancement estimates.
- **New lines to type** differs from the net change in lines of code. Remus2 replaces 12 search lines and adds 123 lines, increasing the total by 111. Engine-guide file tables show current and preceding line counts and the net change.
- **Elo** in the feature catalog is an estimated gain for one enhancement, not a
  measured Remus result or a target for a student test. Whole-version match results
  do not isolate a feature's effect. Gains overlap as the engine grows and cannot
  be added to predict a rating.
- Rows marked **(foundation)** provide supporting data or output for later steps.
- **Version** names the first engine that implements the enhancement.

### The enhancements, grouped by the file they change

#### (E) Evaluation Only: `evaluation.py`

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

#### (S) Search Only: `search.py`

These enhancements change which moves the search tries and how deeply. Selective
search can change moves, scores, and node counts. Use the [verification
workflow](#checking-that-the-engine-works) to check behavior; fewer nodes or an
unchanged move alone do not establish a better engine.

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

#### (M) Multiple Files

| Step | Enhancement | What it adds | Files | Python lines | Elo | Version |
| --- | --- | --- | --- | ---: | ---: | --- |
| M1 | Draw sense | Score threefold repetition, the fifty-move rule, and insufficient material as draws inside the search. Needs a halfmove clock in the position, a history of position keys, and the moves of `position startpos moves ...` kept in that history | `board.py`, `make_move.py`, `search.py`, `uci.py` | 28 | +4 | Remus4 |
| M2 | Time management | Spend more time when the best move changes between iterations, less when it holds, and never more than a hard limit per move | `search.py`, `uci.py` | 16 | +20 | Remus3 |

Draw sense gains little Elo, but it matters more than that number suggests in real
games: without it Remus repeats moves into a draw when it is winning, and misses a
repetition that would save a lost game.

#### (N) New Files

| Step | Enhancement | What it adds | Files | Python lines | Elo | Version |
| --- | --- | --- | --- | ---: | ---: | --- |
| N1 | Position key (foundation) | A number that names a position, for the transposition table and the repetition history: Python's `hash()` of the tuple `(tuple(board), white, castling, en_passant)` | `board.py` | 2 | 0 | Remus3 |
| N2 | Transposition table | Remember each searched position's score, depth, bound, and best move; reuse the score when it suffices, and try the stored move first | **New file** `transposition_table.py`, plus `search.py` | 35 | +101 | Remus3 |
| N3 | Opening book | Play a stored opening line while the game follows one. The book holds UCI moves (`e2e4 e7e5 g1f3`), not SAN, so it needs no notation code | **New file** `opening_book.py`, plus `uci.py`, plus the supplied `opening_book.txt` data | 28 | +58 | Remus3 |
| N4 | Static exchange evaluation | Play out every capture on one square, cheapest attacker first, to see if a capture wins material; quiescence then skips the losing captures | **New file** `exchange.py`, plus `search.py` | 45 | +15 | Remus4 |

### Rust speed techniques: T1 to T7

Remus6, Remus7, and Remus8 share these seven techniques. The `Code` column points
to their implementations; the detailed explanations follow the table.

| Step | Technique | Code | What it saves | Gain alone | Version |
| --- | --- | --- | --- | ---: | --- |
| T1 | Branch-free ray lookup | `rising_ray()` and `falling_ray()` in `attacks.rs` | A badly predicted test in each of the four rays of every bishop, rook, and queen lookup | 8% | Remus6 |
| T2 | Legal-move shortcut | `has_legal_move()` in `search.rs`, with the table `QUEEN_LINES` in `attacks.rs` | A full move generation at most quiescence positions | 10% | Remus6 |
| T3 | Lazy move ordering | `place_next()` in `search.rs` | The sort of the whole move list when one of the first three moves causes the cutoff | 4% | Remus6 |
| T4 | Central holes from one mask | `positional_terms()` in `evaluation.rs` | Sixteen square-by-square tests in every evaluation | 2% | Remus6 |
| T5 | Evaluation on demand | `alpha_beta()` in `search.rs` | The evaluation of positions whose pruning never reads the score | 1 to 2% | Remus6 |
| T6 | Table prefetch | `Table::prefetch()` in `transposition_table.rs`, called from `alpha_beta()` | Part of the wait for a table bucket to arrive from memory | 1% | Remus6 |
| T7 | Pawn-structure cache | `probe_pawns()` and `pawn_structure()` in `evaluation.rs` | Rescoring doubled, isolated, passed, and backward pawns, pawn attacks, and file counts when the pawns match a recent position | 5% | Remus6 |

`Version` identifies the first engine with the technique. All seven begin in Remus6
and are retained in Remus7 and Remus8; Remus1 to Remus5 do not implement these optimizations.

Each gain is that technique alone, in nodes per second with one search thread. T1, T2, and
T4 are measured in Remus6. T3, T5, and T6 are measured in the move loop of the
Remus4 search, which the three engines share. The gains overlap, so together they give
the [combined speed ratios](#speed-techniques-of-the-rust-engines-6-through-8), not their sum. T1 to T6 add 49 code lines to Remus6
and 51 to each of Remus7 and Remus8; T7 adds 52 to each of the three. T7 is measured
in Remus6 on 17 fixed-depth searches, with identical node counts, scores, and moves:
7 to 10% in middlegames and almost nothing in pawn and rook endings.

#### T1: Branch-free ray lookup

A rook on a8 looks down the a-file. `RAYS` holds that whole ray: a7, a6, a5, a4, a3, a2,
and a1. With a piece on a5, the rook reaches a7, a6, and a5, so the lookup finds the
first piece on the ray and removes the squares beyond it, which are the ray from a5
itself.

The plain form first checks whether the ray contains a piece, because an empty
ray has no first piece to look up. The answer changes from one lookup to the next
with no pattern. The processor often predicts the answer incorrectly. Each wrong
prediction costs about as much as fifteen ordinary operations. Every bishop, rook, or queen lookup makes
that test four times, once per ray, and the engine makes those lookups in the mobility
count of `evaluate()`, in `is_attacked()`, in move generation, and in the exchange
evaluation.

`rising_ray()` covers the four rays whose square numbers grow. It sets bit 63, the
square h1, among the pieces on the ray before it finds the first one. No rising ray
continues past h1, so the `RAYS` entry of h1 is empty: on an empty ray the lookup finds
h1, removes nothing, and returns the whole ray, with no test. `falling_ray()` does the
same for the four rays whose square numbers shrink, with bit 0, the square a8. The
result is the same set of squares in every case.

#### T2: Legal-move shortcut

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

#### T3: Lazy move ordering

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

#### T4: Central holes from one mask

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

#### T5: Evaluation on demand

`alpha_beta()` reads the static score of a position in two tests only. Null-move pruning
reads it at depth 3 and up, out of check, away from the root, and not right after
another null move. Futility pruning reads it at depth 1, out of check. The plain form
calls `evaluate()` at every position before those tests. Remus6 through Remus8 compute
`needs_score` from the same conditions and call `evaluate()` only when it is true. The
search skips this static evaluation at depth 2 and in check. At depth 3 and up,
it also skips the evaluation at the root and immediately after a null move.

#### T6: Table prefetch

The transposition table holds 512 MiB, far more than the processor's cache, so reading
the bucket of a position takes roughly 100 nanoseconds, about half of the time the
engine spends on an average position. The search can locate the next position's bucket
as soon as `make_move()` returns that position's key.

`Table::prefetch(key)` asks the processor to start loading the bucket and returns at
once. `alpha_beta()` calls it right after `make_move()`, so the legality test and the
check test of the move run while the bucket travels from memory, and the child's
`probe()` spends less time waiting for memory. `alpha_beta()` calls prefetch only
at depth 2 and up. A child of a depth-1 position goes to quiescence unless it is
in check. Quiescence does not use the table, so a prefetch there costs more than it saves. The null-move search prefetches its
position the same way, and in Remus8 so does `search_split_move()` when a worker
takes a move from a split. A prefetch reads no value, so the shared table of the
parallel engines needs no added synchronization.

#### T7: Pawn-structure cache

Most moves leave every pawn where it stands, so thousands of positions in one search
share the same pawns. The doubled, isolated, passed, and backward pawn scores depend
on the pawns alone, and so do the pawn attacks and the count of pawns on each file.
`probe_pawns()` keeps those results in a table of 8,192 entries, about 1 MiB, indexed
by a mix of the two sides' pawn bitboards. An entry stores both bitboards and answers
only an exact match, so a lookup never returns another structure's score. On a miss
`pawn_structure()` scores the pawns and replaces the entry. Each search thread keeps
its own table, so the parallel engines need no synchronization for it.

The terms that also depend on other pieces are still computed every time from the
cached parts: the pawn race reads the cached passed pawns with the kings and the side
to move, rooks read the cached file counts, and king cover reads the pawns and file
counts with the king's square and the game phase.

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
