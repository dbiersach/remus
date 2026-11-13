# Remus2: legal chess and alpha-beta

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

Remus2 is [Remus1](../Remus1/README.md) , the random mover the student pairs finish on
Day 1, with `evaluation.py` and a real `search.py` in place of Remus1's random one. It
plays legal chess with an alpha-beta search, a quiescence search that plays out the
captures at the end of each line, MVV-LVA move ordering (most valuable victim, least
valuable attacker), a check extension, and iterative deepening. It speaks UCI to Lucas
Chess through `remus.bat` and uses only the standard library.

[Remus3](../Remus3/README.md) adds the biggest gains for the fewest lines, and
[Remus4](../Remus4/README.md) adds every other enhancement.
[`remus_versions.md`](../remus_versions.md) lists each enhancement and the version that
holds it.

## Enhancements over the previous version

Remus2 adds evaluation, alpha-beta search, capture quiescence, MVV-LVA ordering, a check
extension, and iterative deepening. It replaces the 12-line random search with 93 search
lines and adds 68 evaluation lines: 161 new lines to type, but a net increase of 149
TLOC.

## Source files and TLOC

Comparison baseline: [Remus1](../Remus1/README.md) .

TLOC counts nonblank runtime source lines, excluding standalone comments and docstrings.
Change is the net difference, not lines edited or typed. A zero means that file is
absent from that version. Supporting files and external checks are excluded.

| File | Remus2 TLOC | Remus1 TLOC | Change | Responsibility |
| --- | ---: | ---: | ---: | --- |
| `board.py` | 15 | 15 | 0 | The piece numbers, the starting position, and square names |
| `make_move.py` | 23 | 23 | 0 | Play a move on a copy of the board |
| `attacks.py` | 34 | 34 | 0 | Is a square attacked? Is a king in check? |
| `move_generation.py` | 65 | 65 | 0 | List the moves of the side to move, castling included |
| `evaluation.py` | 68 | 0 | +68 | Material plus four piece-square tables |
| `search.py` | 93 | 12 | +81 | Move ordering, quiescence, alpha-beta, iterative deepening |
| `uci.py` | 44 | 44 | 0 | The UCI loop: `uci`, `isready`, `position`, `go`, `quit` |
| **Total** | **342** | **193** | **+149** | |

### Supporting files

| File | Responsibility |
| --- | --- |
| `remus.bat` | Launch `uci.py` in the project Python environment |

## Running and configuring

During Day 1 setup, `install_lucas_engines.py` registers the instructor's
[`remus.bat`](../../../engines_instructor/Remus2/remus.bat) with Lucas Chess. Students
select that existing entry; they do not add an engine manually in class. For a
separate home setup, use the engine folder as the launcher's working directory and
follow the Lucas Chess guide. The launcher uses the repository's
Python 3.14 environment. Engine code imports only the standard library. The UCI identity
is `Remus2` .

Lucas Chess holds the instructor's engine, as the
[Lucas Chess guide](../../lucas_chess/lucas_chess_guide.md) shows. It names itself
`Remus2` .

Opening `uci.py` with Run Python File starts a silent UCI command loop, with no
board window. Close that run's terminal when finished. Running a helper module
such as `make_move.py` loads its imports and definitions and normally exits without
output; this checks for load errors, not whether its functions work. In Lucas
Chess, a game should display moves and clocks. It uses the instructor's registered
engine, not the student pair's files; see the verifier instructions below.

### UCI options

This engine advertises no configurable UCI options.

### Supported commands

`uci` , `isready` , `position startpos moves ...` , timed `go` , and `quit` form the
normal GUI conversation. The Python UCI adapter does not accept arbitrary FEN positions.

## Architecture and call diagrams

Start with the module overview, then open the function view for the module you
want to follow. These diagrams describe the Python Remus2 source; they are
not Rust call graphs or a trace of one particular search.

The diagrams use Mermaid's ELK layout. Each module has a stable box color across Python and Rust: `uci.py` and `main.rs`
share blue, and both search modules use orange. Colors identify source modules,
not execution frequency. An arrow
points from a caller to a callee, and a loop marks recursion. A small bridge at a
crossing means the lines cross without joining. Arrows show possible calls, not
execution order or call frequency.

### Module overview

Each box is a source file. An arrow means a function or initialization in the
first module calls a function in the second. Calls within a module are omitted;
multiple calls between the same two modules share one arrow.

![Remus2 module overview](mermaid/modules.svg)

### Function views

A focused view includes calls originating in the named module. External callees
retain their module labels, but their own calls are not expanded. A function with
no outgoing calls appears when a selected caller uses it. The complete graph
contains all recognized connections and recursive calls, and needs more zooming
than the focused views.

| View | Boxes | Arrows | Editable source |
| --- | ---: | ---: | --- |
| [Complete function graph](mermaid/full_elk.svg) | 22 | 30 | [Mermaid](mermaid/full_elk.mmd) |
| [Module overview](mermaid/modules.svg) | 7 | 9 | [Mermaid](mermaid/modules.mmd) |
| [UCI and move selection](mermaid/uci.svg) | 7 | 6 | [Mermaid](mermaid/uci.mmd) |
| [Search](mermaid/search.svg) | 9 | 14 | [Mermaid](mermaid/search.mmd) |
| [Evaluation](mermaid/evaluation.svg) | 2 | 1 | [Mermaid](mermaid/evaluation.mmd) |

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

Alpha-beta rejects branches that cannot improve the current choice. Capture quiescence
extends tactical exchanges beyond the nominal depth; material and piece-square tables
score the resulting positions.

### Readable representation and search

- **One function moves every piece but the pawn.** `piece_moves()` takes the steps a
  piece may take and whether it slides: a knight or king takes one step in each
  direction, while a bishop, rook, or queen keeps going until a piece or the edge stops
  it.
- **`is_attacked()` uses the same idea in reverse.** It looks out from the square with
  the steps of each kind of piece, and the first piece it meets on each line attacks
  the square if it is the right kind.
- **`ordered_moves()` generates the moves and sorts them,** and hands each one to the
  search with its priority. Quiescence stops at the first quiet move, whose priority
  is 0.
- **A position is one tuple,** `(board, white, castling, en_passant)`, so
  `make_move()` takes a position and returns the next one, and every search call
  passes one value instead of four.
- **Copy, don't undo.** `make_move()` copies the board with `board[:]` and returns
  the new position, so the search never takes a move back.
- **Pieces are numbers.** A piece is its kind, from 1 for a pawn to 6 for a king, plus
  8 if it is White, so `piece & 7` gives the kind. The castling rights are four bits:
  1 and 2 for White's kingside and queenside, 4 and 8 for Black's. The starting board
  is built from one back rank, `BACK_RANK`, that both sides share, and letters appear
  only in UCI move text such as `e7e8q`.
- **Half tables.** Every piece-square table is the same on both sides of the board (a
  knight earns the same bonus on c3 and on f3), so each table lists only files a to d,
  rank 8 first, and `mirrored_table()` copies them in reverse order onto files e to h.
- **No legal-move list.** The search makes each pseudo-legal move and skips it if its
  own king is attacked.

## Memory and transposition table

This engine has no transposition table. Search state belongs to its single process.

## Opening book and tablebases

This engine has neither an opening book nor tablebase support; move selection is native.

## Time management and pondering

The move budget is one thirtieth of the remaining clock plus half the increment, or two
seconds without clock information. `movetime` overrides that budget. Iterative deepening
retains a usable move when time expires. There is no separate adaptive hard-budget
policy or pondering.

## Diagnostics and performance

UCI `info` reports search progress. There are no persistent search or ponder logs and no
cache hit counters.

## Verification and limitations

Open `engines/Remus2/verify_remus.py` in VS Code and press Run Python File at the
scheduled checkpoint. It checks required files and counts legal move sequences
(perft) from six reference positions, comparing them with published counts.
It also compares five evaluation scores and depth-4 search moves, scores, and node
counts with stored instructor results. Those searches call the native search
directly. The UCI check looks for the engine
name, `uciok`, `readyok`, and a final `bestmove` line; it does not validate that
move's legality or every possible protocol exchange.

Expect `Verifying` followed by actual and expected results. Success ends with
`Remus2 passes every test` and elapsed time. A mismatch prints `PROBLEM:`;
import or execution errors can show a traceback. Fix the reported problem and
rerun. Passing covers these reference cases, not every position or code branch,
and does not measure playing strength. Games in Lucas Chess use the instructor's
engine and do not test the student pair's copy.

Remus2 has no repetition or fifty-move search logic and reports no principal
variation. Its UCI input supports games from the starting position, not arbitrary
FEN positions. The verifier constructs its reference positions outside that adapter.

## Further reading

- [Version guide](../remus_versions.md): feature progression and comparisons.
- [Lucas Chess guide](../../lucas_chess/lucas_chess_guide.md): workshop setup and play.
- [UCI protocol introduction](../../chess_tutor/uci.md): engine and GUI communication.
