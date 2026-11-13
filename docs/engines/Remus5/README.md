# Remus5: tactical search and positional evaluation

[Overview](#overview) |
[Enhancements over the previous version](#enhancements-over-the-previous-version) |
[Source files and TLOC](#source-files-and-tloc) |
[Running and configuring](#running-and-configuring) |
[Architecture and call diagrams](#architecture-and-call-diagrams) |
[Search and evaluation](#search-and-evaluation) |
[Memory and transposition table](#memory-and-transposition-table) |
[Opening book and tablebases](#opening-book-and-tablebases) |
[Time management and pondering](#time-management-and-pondering) |
[Diagnostics and performance](#diagnostics-and-performance) |
[Verification and limitations](#verification-and-limitations) |
[Further reading](#further-reading)

## Overview

Remus5 extends [Remus4](../Remus4/README.md) with four search enhancements: check-aware
quiescence, stalemate detection at the search leaves, consistent en passant capture
handling, and history ordering for quiet moves. Its positional evaluation adds backward
pawns, central holes, knight outposts, and nearby king cover. The opening book, board
representation, and copy-make move generation match Remus4. Remus5 also provides
optional root tablebase advice and GUI-driven UCI pondering. The Python engine uses only
the standard library and speaks UCI through `remus.bat` . Its transposition table
occupies a fixed 512 MiB allocation.

The [Python source](../../../engines_instructor/Remus5/) contains 1430 code lines, 736
more than Remus4's 694. This is a net difference in source length, not a count of lines
to type or replace. Remus5 is an experimental engine for strength comparisons; the
five-day workshop continues to use Remus1 through Remus4.

## Enhancements over the previous version

Compared with Remus4, Remus5 adds check-aware quiescence, terminal-position checks at
search leaves, consistent en passant accounting, quiet-move history, and four positional
evaluation terms. It uses a fixed 512 MiB table and adds optional root tablebase advice
and GUI-controlled pondering.

Legal book continuations take priority over native search, including after a ponder hit.
Book ponder requests wait without searching or earning native time credit. Shallow,
deteriorating, or unstable native hits can reinvest more of the saved clock time. See
[Time management and pondering](#time-management-and-pondering) for the thresholds,
safeguards, and examples.

## Source files and TLOC

Comparison baseline: [Remus4](../Remus4/README.md) .

TLOC counts nonblank runtime source lines, excluding standalone comments and docstrings.
Change is the net difference, not lines edited or typed. A zero means that file is
absent from that version. Supporting files and external checks are excluded.

| File | Remus5 TLOC | Remus4 TLOC | Change | Responsibility |
| --- | ---: | ---: | ---: | --- |
| `board.py` | 23 | 23 | 0 | Piece numbers, the position tuple, starting position, position keys, and UCI move text |
| `make_move.py` | 24 | 24 | 0 | Play a move on a copied board and maintain the fifty-move clock |
| `attacks.py` | 34 | 34 | 0 | Detect attacks and check |
| `move_generation.py` | 65 | 65 | 0 | Generate pseudo-legal moves, including castling, en passant, and promotions |
| `evaluation.py` | 285 | 195 | +90 | Material, piece-square tables, mobility, pawn structure, king safety, and endgame terms |
| `exchange.py` | 39 | 36 | +3 | Static exchange evaluation, including removal of the captured en passant pawn |
| `opening_book.py` | 21 | 21 | 0 | Read opening lines and return candidate book moves |
| `transposition_table.py` | 68 | 22 | +46 | Store search depths, scores, bounds, and best moves |
| `search.py` | 283 | 212 | +71 | Alpha-beta search, quiescence, terminal-position checks, draw handling, and move ordering |
| `search_control.py` | 239 | 0 | +239 | Input queue, cancellation, ponder credit, and move clock |
| `tablebase.py` | 158 | 0 | +158 | Bounded root advice, native legality checks, cache, and fallback |
| `uci.py` | 191 | 62 | +129 | UCI commands, game history, legal book moves, and time allocation |
| **Total** | **1,430** | **694** | **+736** | |

### Supporting files

| File | Responsibility |
| --- | --- |
| `remus.bat` | Launch `uci.py` in the project Python environment |
| `opening_book.txt` | Opening lines in UCI move notation |

## Running and configuring

Select [`remus.bat`](../../../engines_instructor/Remus5/remus.bat) as the UCI engine and
use its engine folder as the working directory. The launcher uses the repository's
Python 3.14 environment. Engine code imports only the standard library. The UCI identity
is `Remus5` . HTTP and JSON use `urllib` and `json` ; a daemon input-reader thread and
queue keep native search responsive to commands. Packed cache entries use `mmap` and
`struct` .

The experimental engine uses a separate external-engine entry in Lucas Chess. Enable
Ponder in both that entry and its tournament entry to use pondering.

### UCI options

| Option | Type | Default | Allowed values | Purpose |
| --- | --- | --- | --- | --- |
| `LichessTablebase` | check | true | true, false | Allow bounded root endgame advice |
| `Ponder` | check | false | true, false | Provide predictions and accept GUI-driven pondering |
| `PonderDiagnostics` | check | false | true, false | Report ponder timing and outcomes |

PonderDiagnostics also writes the Python logs described below. Disabling
LichessTablebase selects native move choice without network advice.

### Supported commands

`uci` , `isready` , `position startpos moves ...` , timed `go` , and `quit` form the
normal GUI conversation. The Python UCI adapter does not accept arbitrary FEN positions.
It also handles `setoption` , `ucinewgame` , `go infinite` , `go ponder` , `ponderhit` ,
and live `stop` commands. An input-reader thread queues commands while one search thread
owns the chess state.

## Architecture and call diagrams

Start with the module overview, then open the function view for the module you
want to follow. These diagrams describe the Python Remus5 source; they are
not Rust call graphs or a trace of one particular search.

The diagrams use Mermaid's ELK layout. Each module has a stable box color across Python and Rust: `uci.py` and `main.rs`
share blue, and both search modules use orange. Colors identify source modules,
not execution frequency. The legend below lists this engine's module colors. An arrow
points from a caller to a callee, and a loop marks recursion. A small bridge at a
crossing means the lines cross without joining. Arrows show possible calls, not
execution order or call frequency.

### Module color legend

The same module keeps its color across engine versions and languages. The UCI adapter is blue (`uci.py` in Python, `main.rs` in Rust). Labels and color names identify each module without relying on color alone.

| Module or role | Color | Fill |
| --- | --- | --- |
| `uci.py` | ![Blue](../color_swatches/uci.svg) Blue | `#dbe8fb` |
| `search.py` | ![Orange](../color_swatches/search.svg) Orange | `#fde2cf` |
| `opening_book.py` | ![Light green](../color_swatches/opening_book.svg) Light green | `#e3f4d7` |
| `transposition_table.py` | ![Lavender](../color_swatches/transposition_table.svg) Lavender | `#efe0f7` |
| `exchange.py` | ![Yellow](../color_swatches/exchange.svg) Yellow | `#fff1c2` |
| `evaluation.py` | ![Light red](../color_swatches/evaluation.svg) Light red | `#fbd9d9` |
| `move_generation.py` | ![Teal](../color_swatches/move_generation.svg) Teal | `#d4f1f1` |
| `make_move.py` | ![Tan](../color_swatches/make_move.svg) Tan | `#ece4d6` |
| `attacks.py` | ![Pink](../color_swatches/attacks.svg) Pink | `#fadcec` |
| `board.py` | ![Blue gray](../color_swatches/board.svg) Blue gray | `#e2e5f3` |
| `tablebase.py` | ![Mint](../color_swatches/tablebase.svg) Mint | `#cdeedc` |
| `search_control.py` | ![Sage](../color_swatches/search_control.svg) Sage | `#dce8d4` |
| External programs, launchers, and files | ![Gray](../color_swatches/external.svg) Gray | `#f1f5f9` |

### Module overview

Each box is a source file. An arrow means a function or initialization in the
first module calls a function in the second. Calls within a module are omitted;
multiple calls between the same two modules share one arrow.

![Remus5 module overview](mermaid/modules.svg)

### Function views

A focused view includes calls originating in the named module. External callees
retain their module labels, but their own calls are not expanded. A function with
no outgoing calls appears when a selected caller uses it. The complete graph
contains all recognized connections and recursive calls, and needs more zooming
than the focused views.

| View | Boxes | Arrows | Editable source |
| --- | ---: | ---: | --- |
| [Complete function graph](mermaid/full_elk.svg) | 73 | 126 | [Mermaid](mermaid/full_elk.mmd) |
| [Module overview](mermaid/modules.svg) | 12 | 25 | [Mermaid](mermaid/modules.mmd) |
| [UCI and move selection](mermaid/uci.svg) | 26 | 36 | [Mermaid](mermaid/uci.mmd) |
| [Search](mermaid/search.svg) | 23 | 41 | [Mermaid](mermaid/search.mmd) |
| [Pondering, commands, and deadlines](mermaid/search_control.svg) | 14 | 15 | [Mermaid](mermaid/search_control.mmd) |
| [Tablebase lookup and validation](mermaid/tablebase.svg) | 13 | 13 | [Mermaid](mermaid/tablebase.mmd) |
| [Evaluation](mermaid/evaluation.svg) | 10 | 8 | [Mermaid](mermaid/evaluation.mmd) |

### Viewing and editing

Open an SVG in a browser to zoom without losing text quality. SVGs need no
Mermaid installation or online account. Matching `.mmd` files contain editable
diagram text, including layout and colors. Render them with Mermaid CLI 12.2.0
for the supplied layout. Other Mermaid hosts can differ in version and layout
support.

The graph contains statically recognized calls to engine functions, including
module initialization and thread targets. Standard-library internals and function
definitions with no recognized connections are outside its scope.

See [Mermaid layouts](https://mermaid.js.org/config/layouts.html) for ELK settings
and [Mermaid CLI](https://github.com/mermaid-js/mermaid-cli) for local rendering.

## Search and evaluation

### 1. Check-aware quiescence

A capture can give check even after the main search reaches its depth limit. For
example, with White's king on h1 and pawn on g2, and Black's king on f2 and queen on g3,
Black's `g3g2` captures the pawn and gives mate. Quiescence must recognize the mate when
it examines the resulting position.

`quiescence()` checks whether the side to move is in check at every recursive call. In
check, it considers all legal evasions: king moves, blocking moves, captures, and
promotions. It does not accept the static evaluation as a stand-pat score, and it
disables delta pruning and static exchange pruning for those evasions. Ordinary
alpha-beta cutoffs still apply.

Standing pat means accepting the position's static score without searching another move.
That option is useful in a quiet position, but cannot represent a legal response to
check. With White's king on h1, Black's king on f3, and Black's rook on h8, the only
legal response is the quiet move `h1g1` ; quiescence searches it.

The search carries the distance from the root, measured in plies, into quiescence. One
ply is one move by one side. Checkmate scores use `-(MATE_SCORE - ply)` , so the engine
prefers a shorter forced mate and delays a forced loss.

**Expected outcome:** more reliable scores for checking captures and forced responses at
the search boundary. Searching quiet evasions can increase the work per position, so
this enhancement does not guarantee a higher search depth.

### 2. Stalemate detection before stand pat

With White's king on h1, Black's king on f2, and Black's queen on g3, White has no legal
move and is not in check. The score must be zero, regardless of Black's material
advantage or the alpha-beta window.

`has_legal_move()` stops at the first move that leaves the moving king safe. Quiescence
calls it before accepting a static score or pruning moves. If there is no legal move,
quiescence returns a mate score when the king is checked, or zero for stalemate. Finding
no captures alone does not establish stalemate: quiet legal moves also count.

The null-move pruning condition also requires at least one legal move. This keeps a
hypothetical pass from supplying a cutoff in a stalemated position.

**Expected outcome:** the search values stalemate as a draw at its leaves and can
distinguish it from winning material. Checking for a legal move adds work, although the
helper stops as soon as it finds one.

### 3. En passant throughout the search

After Black plays `d7d5` beside a White pawn on e5, `e5d6` captures a pawn even though
d6 is empty. `captured_kind()` returns `PAWN` for that move and supplies the captured
kind to move ordering, delta pruning, and the quiet-move classification.

This gives en passant a capture priority and includes it in quiescence. It also keeps
the move out of quiet-move futility pruning, late move reductions, killer updates, and
quiet-history rewards. A remembered killer move receives its killer priority only when
it is a quiet, nonpromotion move in the current position.

Static exchange evaluation (SEE) estimates material gained or lost by captures and
recaptures on one square. For en passant, `exchange_score()` credits the captured pawn
and removes it from the copied board before finding recaptures. With a Black rook on d4,
removing the pawn on d5 opens the rook's path to d6, where it can recapture White's
pawn. The exchange therefore scores zero rather than a free pawn.

The usual king-safety check applies after making the move. An en passant capture that
opens a rook or bishop attack on the moving side's king is rejected.

**Expected outcome:** en passant receives the same search treatment as other pawn
captures, including material accounting and legality. Its overall Elo contribution is
likely small because the opportunity occurs relatively infrequently.

### 4. History ordering for quiet moves

Suppose `g1f3` repeatedly causes a cutoff in different branches of the search. A cutoff
means the move is already good enough that the remaining alternatives cannot affect the
current alpha-beta decision. Its history score gives the search a reason to try that
quiet move early in another position.

`quiet_history` stores a score for each side and move. A quiet move that causes a beta
cutoff gains `depth * depth` , so deeper cutoffs receive more weight. Scores are capped
at 3,999. At the start of each `think()` call, every score is halved using integer
division, giving recent searches more influence.

The move-ordering priorities are:

1. The transposition-table move, with priority 1,000,000.
2. Captures ordered by victim and attacker value, with an extra bonus for queen
   promotions; noncapturing queen promotions also receive a capture-level priority.
3. The two quiet killer moves for the current ply, with priorities 5,000 and 4,000.
4. Other quiet, nonpromotion moves, with history scores from 0 to 3,999.

Noncapturing underpromotions retain priority zero. Equal priorities use the move's
square numbers and promotion kind as a deterministic tie breaker. History affects
ordering; it does not directly change the evaluation, pruning margins, or reduction
formula. Because reductions depend on move order, it can affect the selected move as
well as the number of positions searched.

**Expected outcome:** earlier cutoffs and fewer unproductive searches, especially when
principal variation search depends on trying a strong move first. A working estimate is
**+20 to +60 Elo** for history ordering alone, subject to equal-time match testing.

### Supporting search safeguards

Quiescence records the position keys along its recursive path and applies the same
repetition and fifty-move checks as the main search. Quiet check evasions can repeat
positions, so capture-only termination is not sufficient. The engine restores the
quiescence history with `try` and `finally` , including when the clock interrupts the
search.

Checkmate takes precedence over a fifty-move draw in both searches. A shared limit of
128 plies bounds long checking sequences and access to the killer table. At that limit,
a nonterminal position that does not qualify as a draw receives its static evaluation.
This is a depth-limit approximation, not proof that the position is quiet. Away from
check, quiescence remains selective: delta pruning and SEE can omit captures, and quiet
moves other than queen promotions are not searched.

### Positional evaluation

A black pawn on d6 can become a lasting target when the e-pawn advances to e5 and the
c-pawn cannot support it. A white pawn on e4 controls d5, making ...d5 difficult. The
evaluation subtracts 18 centipawns for this backward pawn and another 8 when no enemy
pawn screens its file. A neighboring friendly pawn on the same rank or behind the pawn
prevents the penalty, as does direct pawn protection. Isolated pawns retain their
separate penalty without this addition.

An empty central square on the defending side's third or fourth rank costs 12 centipawns
when an enemy pawn protects it, an enemy knight exists, and no friendly pawn on an
adjacent file can advance to attack the square. For example, d5 can become such a hole
after Black plays ...e5. The term covers files c through f. It scores the opportunity;
it does not assert that a knight can reach the square without a tactical problem.

A knight on ranks four through six, viewed from its own side, earns 15 centipawns on
files b through g when no enemy pawn can challenge it, plus 15 for friendly pawn
protection. An occupied square receives the knight bonus instead of the empty-hole
penalty. Potential pawn challenges are conservative: an adjacent pawn behind the square
counts even if another piece currently blocks its advance.

King cover applies on the king's first three ranks. A pawn one rank ahead earns 10
centipawns; a pawn two ranks ahead earns 5. Each nearby file without a friendly pawn
costs 15. These terms fade with the remaining pieces, reaching zero in a pawn-only
endgame. The king's piece-square tables also reflect its location.

The weights are modest heuristics, with 100 centipawns equal to a pawn. Search combines
these terms with material, mobility, passed pawns, and tactical results. The terms do
not force an opening move, and fixed-position checks do not establish a playing-strength
gain.

## Memory and transposition table

The table has a fixed 512 MiB allocation: 8,388,608 buckets of four 16-byte entries.
Multiply-shift indexing selects a bucket. A matching key or empty slot is preferred; a
full bucket replaces the entry with the lowest stored depth minus eight times its age in
searches. Ages wrap after 64 searches. Mate scores are adjusted for the distance from
the root. Entries persist across moves; capacity pressure replaces individual entries
rather than clearing the whole table.

Python stores packed entries in an anonymous memory mapping using only `mmap` and
`struct` . Pages are zero-filled on access; allocation failure leaves native search
available without caching. The HTTP response cache is separate. Each process owns its
own allocation, with runtime and other engine memory additional. There is no `Hash`
option, automatic RAM sizing, or runtime resize. Allow space for both engines in each
simultaneous tournament game and for Lucas Chess.

## Opening book and tablebases

### Opening book

The supplied `opening_book.txt` contains 43 openings and 13 trap defenses in UCI move
notation, with 17 openings restricted to White. Every selected move is checked with the
native generator and king-safety test. Without a legal continuation, the engine uses
native move selection. Set Lucas Chess's external book to `<None>` , including in
tournament entries, so a GUI book cannot override the engine's book.

Python reads the file beside `opening_book.py` at startup. Edit the text file and
restart the process to use revised lines; no compilation is needed. Missing book data
leaves native search available.

### Choosing an endgame move

`LichessTablebase` defaults to true. Set `setoption name LichessTablebase value false`
to disable network advice. An accepted response emits `info string Lichess tablebase`
before `bestmove` ; fallback emits ordinary native search information.

With White's king on g7 and pawn on h7, and Black's king on e8, White can promote with
`h7h8q` . The adapter serializes that position as `4k3/6KP/8/8/8/8/8/8 w - - 0 1` and
requests `https://tablebase.lichess.org/standard?fen=<URL-encoded FEN>` .

Lichess lists legal moves best first. The position's category describes the player to
move, while each move's category describes the opponent after that move. A winning root
therefore needs a move labeled `loss` . Remus5 uses this server order and checks the
first entry; it does not sort signed DTZ values or select the first entry labeled `win`
. DTZ concerns the next pawn move or capture that resets the fifty-move counter, not
necessarily the fastest mate.

The adapter accepts `win` , `loss` , `draw` , `cursed-win` , and `blessed-loss` when the
root and child results agree. Cursed wins and blessed losses count as draws under the
fifty-move rule. `unknown` , `maybe-win` , `maybe-loss` , `syzygy-win` , and
`syzygy-loss` select native search because this adapter requires a definite result with
the current counter. An illegal, empty, malformed, or inconsistent first entry also
selects search. It does not skip a bad first entry and claim a lower-ranked entry is
optimal.

The native search never probes Lichess. The engine makes one root request per uncached
eligible position, subject to its timing and cooldown limits. It can therefore consult
the service on successive turns, not just once when seven pieces remain. No tablebase
score is inserted into the native transposition table.

### FEN and draw history

FEN exists only at the Python HTTP boundary in `tablebase.py` . The native board, move
generator, evaluation, and search retain the Remus5 representation. FEN includes the
side to move, castling rights, en passant target, and actual halfmove counter. The
fullmove number is 1 because it does not affect tablebase results and the native
position does not store it.

Positions with castling rights, more than seven pieces, or a halfmove counter of 100 or
greater use native search. FEN does not carry repetition history. The adapter skips a
lookup if a position repeats within the reversible history, and rejects a candidate that
revisits a recorded position without resetting the halfmove counter. These conservative
guards leave repetition-sensitive decisions to the existing search; a remote result
alone cannot establish the full game's draw status.

### Network behavior and timing

- A lookup waits at most one second and at most one quarter of the move's hard
  time allowance. If that allowance would give the request less than 20 ms, the
  engine searches immediately. Elapsed lookup time is subtracted from both native
  search budgets.
- Each engine process permits one outstanding request. A slow worker cannot block
  the UCI thread beyond the lookup budget or cause another concurrent request in
  that process. Late responses are associated with their original FEN.
- A process caches up to 256 JSON responses by complete API FEN, including the
  halfmove counter. It clears the cache when admitting another entry at capacity.
  Cached moves are validated against the current position and history on each use.
- Successful JSON responses impose a one-second request cooldown. Transport and
  parsing failures impose five seconds. HTTP 429 imposes at least 60 seconds and
  honors a longer `Retry-After` value in seconds or HTTP-date form. Cooldowns never
  make the engine sleep instead of searching.
- Responses larger than 65,536 bytes are rejected. HTTPS certificate validation
  stays enabled. An unavailable service, DNS or TLS failure, timeout, invalid
  JSON, HTTP error, or unsupported position falls back to native search.

The wait limit applies to the tablebase adapter. Native search retains Remus5's periodic
clock checks and first-legal-move behavior. The cache and cooldown are local to one
engine process; separate GUI engine instances do not share them. Service availability
and lookup latency can affect how often advice is used.

## Time management and pondering

### Move budgets

With a supplied clock, the initial plan is one thirtieth of the remaining time plus half
the increment. The hard budget is the smaller of three plans and one fifth of the
remaining time. Without clock information the default is two seconds. `movetime`
supplies both budgets explicitly. Iterative deepening can enlarge the plan when the
preferred move changes, within the hard budget. The soft stopping target at a completed
iteration is half the adjusted plan; an unfinished iteration can continue toward the
hard budget. Periodic clock checks and the time to finish work can cause small timing
overshoots.

### Lucas Chess setup and hit handling

Enable **Ponder** in the UCI options for the Remus5 external-engine entry. Tournament
entries keep their own engine settings: enable **Ponder** in that tournament's Remus5
entry too, or import the configured entry again. Keep Lucas Chess's external opening
book at `<None>` . Restart an engine process to load its code and options.

For example, after Remus5 chooses `e2e4` , it can report `bestmove e2e4 ponder e7e5` .
The GUI sends the position including both moves, followed by `go ponder` . Remus5
searches its response to `e7e5` while the opponent thinks.

A standard `ponderhit` continues that search and starts the hard move clock. Lucas Chess
R6 also uses `stop` , the actual `position` , and a fresh `go` when the prediction
matches. Remus5 recognizes this handoff when the position command's tokens, including
the full move history, match exactly and the new search begins within two seconds of the
stopped search ending. Different histories, a new game, an option change, or an expired
handoff receive no credit. Equivalent positions expressed with different command tokens
conservatively miss.

### Adaptive thinking after a hit

The stop-and-go form starts a native search with retained cache and heuristic
information. Completed ponder time counts toward the soft target once the new search
recovers the pondered depth. The gap between commands earns no time credit. Stable hits
reserve 25% of the adjusted move plan on the engine's own clock, capped at 25% of the
hard budget. With at least 60 seconds left on its clock, Remus5 reinvests time when the
hit depth is below 8, the score falls by at least 35 centipawns between completed
iterations of depth 4 or greater, or the best move changes in the latest such iteration.
A deeper cached score is preserved while a Lucas restart replays shallow iterations.
Score deterioration and uncertainty at the hit carry across that restart.

An uncertain hit reserves 50% of the adjusted plan and aims to complete a depth beyond
the hit depth. If that iteration is too expensive, it may return after one full adjusted
plan; both allowances are capped at half the hard budget. For example, a 30-second plan
normally reserves 7.5 seconds after a hit; an uncertain hit reserves 15 seconds and can
try up to 30 seconds for another depth. Clocks below 60 seconds and explicit `movetime`
commands retain the smaller allowance. These thresholds guide time allocation; they do
not establish an Elo gain or guarantee a better move. The hard deadline, explicit
stop/quit, terminal positions, and a proven mate take precedence. The engine returns a
completed iteration when available. Cached results can also help after a missed
prediction, but a miss receives no ponder time credit.

### Predictions, book priority, and finished searches

The standard `Ponder` check option defaults to false. Enable it with
`setoption name Ponder value true` . The predicted reply comes from a legal cached move,
an opening-book continuation, or a legal fallback. A prediction alone never starts a
search. Pondering bypasses online tablebases. A legal book continuation takes priority
on a timed turn, including a Lucas hit. During `go ponder` , Remus5 holds a known book
move until `ponderhit` or `stop` , without searching. Book waiting earns no native
ponder credit. Outside the book, waiting time goes to native analysis. `isready` ,
`stop` , and `quit` remain responsive during search. A finished ponder search, including
mate or stalemate, waits for `ponderhit` or `stop` before returning `bestmove` .

Lucas Chess R6 supplies the required pondering controller. Lucas Chess R2 does not
support this feature. For native-search comparisons, leave `Ponder` disabled; with
pondering enabled, both engines can use CPU at the same time. The search uses the
existing 512 MiB table and one search thread per engine.

## Diagnostics and performance

### Ponder diagnostics

`PonderDiagnostics` is a UCI check option, default false. With
`setoption name PonderDiagnostics value true` , each completed ponder request emits one
`info string ponder` line before `bestmove` . It reports:

- `outcome`: `hit`, or `stopped` when canceled before a hit.
- `ponder_ms`: time before the hit or cancellation; a book result waits without searching.
- `hit_depth` and `hit_nodes`: completed depth and sampled node count at a standard hit.
  A stop-and-go hit records the prior completed depth and zero new-search nodes at handoff.
- `final_depth` and `final_nodes`: completed depth and total work at return.
- `own_ms`: time from a standard hit or matching new `go` to the result.
- `hit_kind`: `uci`, `stop_go`, or `none` for a canceled ponder without a hit.

The node counter is sampled when input is polled, at most every 1,024 nodes. A stopped
request can mean a prediction miss, game end, or other cancellation; its count alone is
not an exact miss rate. GUI-observed response time also includes input delivery and
polling latency. These measurements describe search efficiency, not playing strength or
Elo.

#### Persistent Python logs

With PonderDiagnostics enabled, Python Remus5 appends JSON lines to
`ponder_logs/ponder_<pid>_<session>.jsonl` beside `uci.py` . One file per process keeps
concurrent tournament workers separate; timestamps and search IDs identify events. The
option defaults to false. Enable it together with Ponder in the external-engine and
tournament entries. No log is created while diagnostics are off.

Records include incoming commands, `go ponder` starts, complete position histories,
legal `bestmove ... ponder ...` predictions, standard hits, and linked Lucas stop-and-go
hits. Each search records its initial plan, hard budget, completed depth, total nodes,
charged-clock time, and process CPU timestamps. A restarted Lucas search identifies the
stopped ponder search whose work it credits. Normal searches are recorded too, so
post-hit clock use can be compared with ordinary moves. Book choices have zero native
nodes and are excluded from native search-rate summaries. `source` distinguishes native,
book, and tablebase results. Book ponder waits are counted separately from native
attempts and hits. Native results record `reinvest` , `shallow_hit` , `deteriorating` ,
`unstable` , and the extra-time cap `hit_cap_ms` ; the cap applies when `reinvest` is
true.

Count a linked stop-and-go as one hit, not a stopped miss followed by a second ponder
attempt. Unmatched stops can mean a prediction miss, game end, or cancellation. Ponder
time measures work performed before our clock starts; it is not a direct measurement of
time saved. Initial plans and ordinary-move averages are useful context, but a causal
saving requires a matched no-ponder comparison.

Logging occurs at UCI/search boundaries, not at each node. An unavailable log path
prints one UCI warning and leaves native play and stdout diagnostics available. Logs are
retained until removed by the user. Node rates derived from process CPU time exclude
idle waiting; they are not the same as nodes divided by GUI clock time.

### Playing-strength evidence

The combined search and evaluation terms have no measured Elo gain. More detailed
position scoring costs time per search node. Equal-time, color-balanced games are needed
to determine whether the move-quality gain offsets that cost.

Perft counts establish move-generation correctness, and fixed-depth comparisons check
native search behavior. Neither establishes an Elo gain. For a strength comparison, use
the same Stockfish binary and settings, opening positions, time control, hardware, and
color balance for Remus4 and Remus5. Report the difference with a statistical
uncertainty estimate. Results against one Stockfish configuration describe that matchup,
not a universal rating.

## Verification and limitations

Move generation is checked against published perft positions. Fixed-depth searches,
tactical positions, and UCI conversations check native behavior without requiring
tournament games. Perft validates move generation, not playing strength; node counts and
search depth alone do not establish an Elo gain.

Tablebase availability does not guarantee a shortest mate or an Elo gain. Native search
and its selective approximations apply whenever no acceptable advice arrives. Pondering
is independent of parallel chess search; this engine has one chess search thread.

## Further reading

- [Version guide](../remus_versions.md): feature progression and comparisons.
- [Lucas Chess guide](../../lucas_chess/lucas_chess_guide.md): workshop setup and play.
- [UCI protocol introduction](../../chess_tutor/uci.md): engine and GUI communication.
- [Rust counterpart](../Remus5-Rust/README.md): separate Rust implementation and usage.
- [Lichess API documentation](https://lichess.org/api#tag/Tablebase) describes the public service.
- [Lichess tablebase server](https://github.com/lichess-org/lila-tablebase/blob/main/README.md) documents response categories, move ordering, and DTZ fields.
- [Lichess API guidance](https://lichess.org/page/api-tips) specifies sequential requests and rate-limit backoff.
