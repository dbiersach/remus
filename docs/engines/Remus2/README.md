# Remus2: legal chess and alpha-beta

Remus2 is [Remus1](../Remus1/README.md), the random mover the student pairs finish on
Day 1, with `evaluation.py` and a real `search.py` in place of Remus1's random one. It
plays legal chess with an alpha-beta search, a quiescence search
that plays out the captures at the end of each line, MVV-LVA move ordering (most
valuable victim, least valuable attacker), a check extension, and iterative deepening.
It speaks UCI to Lucas Chess through `remus.bat` and uses only the standard library.

[Remus3](../Remus3/README.md) adds the biggest gains for the fewest lines, and
[Remus4](../Remus4/README.md) adds every other enhancement.
[`remus_versions.md`](../remus_versions.md) lists each enhancement
and the version that holds it.

## Files

| File | Code lines | What it does |
| --- | ---: | --- |
| `board.py` | 15 | The piece numbers, the starting position, and square names |
| `make_move.py` | 23 | Play a move on a copy of the board |
| `attacks.py` | 34 | Is a square attacked? Is a king in check? |
| `move_generation.py` | 65 | List the moves of the side to move, castling included |
| `evaluation.py` | 68 | Material plus four piece-square tables |
| `search.py` | 93 | Move ordering, quiescence, alpha-beta, iterative deepening |
| `uci.py` | 44 | The UCI loop: `uci`, `isready`, `position`, `go`, `quit` |
| **Total** | **342** | |

The code lines leave out blank lines, comments, and docstrings.

## The call tree

Start with the module overview, then open the function view for the module you
want to follow. These diagrams describe the Python Remus2 source; they are
not Rust call graphs or a trace of one particular search.

The diagrams use Mermaid's ELK layout. Each module has its own box color. An arrow
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

## How it keeps the code small

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

## What it leaves out

Tests, perft, FEN, notation, undo, the repetition and fifty-move rules, and the
principal variation. Lucas Chess sends `position startpos moves ...` for every game
from the starting position, which is all Remus2 plays. The workshop designer's
`verify_engines.py` checks Remus2 from
outside: its perft counts match the published counts from six positions, and it
finds a forced mate in all ten mate puzzles (the key move in nine).

## Play it

Lucas Chess holds the instructor's engine, as the
[Lucas Chess guide](../../lucas_chess/lucas_chess_guide.md) shows. It names itself
`Remus2`.
