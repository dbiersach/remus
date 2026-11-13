# Remus4-Rust

Remus4-Rust is the instructor's Remus4 written in Rust, with a larger table. It has the
same rules, the same evaluation, the same search, and the same opening book. It shows the
class that speed matters in a chess engine, not just the algorithms. No student pair
types it or edits it, and it lives only in
[`engines_instructor/Remus4-Rust`](../../../engines_instructor/Remus4-Rust). It includes
a real program, `remus4_rust.exe`, so a laptop needs no Rust installed.
Only a rebuild of the program needs Rust: `rust-toolchain.toml` names the stable compiler
and its parts, and rustup installs them the first time `cargo` runs in the folder.

## The same engine, faster

Each search below started from the same position, 1. a3, for 5 seconds:

| Engine | Depth reached | Positions searched |
| --- | ---: | ---: |
| Remus4 | 7 | 44,625 |
| Remus4-Rust | 14 | 21,885,237 |

Searched to the same depth, the two engines report the same score, visit the same
number of positions, and pick the same move, until the larger table starts to matter on
long searches. Kiwipete, a busy test position, to depth 7: both visit 292,713 positions,
Remus4 in 18.25 seconds and Remus4-Rust in 0.12 seconds. In the first table each engine
stops at the last depth it finished, so those two counts are not a pure speed ratio.

These times come from one computer, and another laptop gives other times.

## Why Rust runs so much faster

Python reads each line of Remus4 as the program runs, and every number and list in it
is a separate object in memory. Rust compiles to machine instructions ahead of time, and
Remus4-Rust uses three more tricks:

- **Bitboards.** A 64-bit number holds one yes-or-no per square, so a single instruction
  counts the squares a rook reaches.
- **No allocation in the search.** Copying a position is a copy of one small block of
  memory, and move lists live on the stack.
- **A running position key.** `make_move()` updates the key with a few exclusive-or
  steps instead of reading all 64 squares.

None of these changes which positions the search visits, so they change only the time.

## The larger table

At millions of positions per second, Remus4's table of 2 million entries fills within
seconds. Remus4-Rust allocates a fixed 512 MiB for 33,554,432 entries, and a full bucket of four
entries replaces its oldest and shallowest entry instead of starting the whole table
over. On four test positions, 20 seconds of thinking reached one ply deeper than the
same engine with Remus4's table. Kiwipete to depth 13 took 13.9 seconds, against 32.9.

## Configuration and diagnostics

| Option | Default | Purpose |
| --- | --- | --- |
| `SearchDiagnostics` | true | Append search timing, nodes per second, and table-use counters to `search_log.txt` beside the executable |

The table belongs to this single-threaded engine process and has a fixed 512 MiB
capacity, like Remus5-Rust. There is no `Hash` option or runtime resizing. Stale
`Hash` commands from saved GUI settings are ignored and leave the cache intact.
Allocation failure disables caching; native search remains available.
Python Remus4 retains its fixed two-million-entry dictionary and has no Hash option.

[Search logs and table experiments](../search_logging.md) explains the counters,
node-rate comparisons, and logging controls. Diagnostic output does not change
normal UCI search results. A timed move can use a provisional root choice from an
interrupted iteration, so its returned move need not match the last completed PV.

## Its files

Native chess files in `src/` match the Remus4 Python modules, and `main.rs` matches
`uci.py`. `diagnostics.rs` handles persistent logs; `table_metrics.rs` holds counters. The code lines leave out blank lines and comments:

| File | Rust code lines |
| --- | ---: |
| `attacks.rs` | 140 |
| `board.rs` | 204 |
| `diagnostics.rs` | 60 |
| `evaluation.rs` | 311 |
| `exchange.rs` | 95 |
| `main.rs` | 188 |
| `make_move.rs` | 75 |
| `move_generation.rs` | 162 |
| `opening_book.rs` | 48 |
| `search.rs` | 387 |
| `table_metrics.rs` | 34 |
| `transposition_table.rs` | 130 |
| **Total** | **1,834** |


## Call structure

![Search and diagnostics](mermaid/search.svg)

[Mermaid source](mermaid/search.mmd)
