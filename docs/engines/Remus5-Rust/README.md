# Remus5-Rust

Remus5-Rust implements [Remus5](../Remus5/README.md) in native Rust, with the
same single-threaded chess search, positional evaluation, fixed 512 MiB table,
optional root Lichess advice, and GUI-driven pondering. It is an experimental
instructor engine outside the five-day workshop.

Select [`remus5_rust.exe`](../../../engines_instructor/Remus5-Rust/remus5_rust.exe)
as an external UCI engine in the chess GUI. It identifies itself as `Remus5-Rust`
and embeds its opening book. Windows 11 needs no Rust or Python installation to
run the executable. Its `LichessTablebase` option defaults to true; false disables
lookups. The engine also plays when the network or service is unavailable.

Enable the **Ponder** UCI option in Lucas Chess R6's external-engine entry and
in each tournament entry that uses Remus5-Rust. It defaults to false. See
[pondering in Lucas Chess R6](../Remus5/README.md#pondering-in-lucas-chess-r6)
for hit/miss behavior and setup. The engine waits for `go ponder`, continues
on `ponderhit`, credits completed ponder work toward its search target, and
returns one result on `stop`. A matching Lucas stop-and-go within two seconds
receives ponder credit after recovering its previous completed depth. Both hit
forms reserve 25% of the adjusted plan on the engine's clock, capped at 25% of
the hard budget; hard deadlines and cancellation take precedence. With at least
60 seconds on the clock and no `movetime`, an uncertain hit (hit depth below 8, a
score that fell 35 centipawns between iterations of depth 4 or more, or a best
move that changed in the latest such iteration) reserves 50% of the plan and
tries for one more completed depth, within one full plan and half the hard
budget. The score and both signs of doubt carry across a Lucas stop-and-go. A
book position answers `go ponder` and a hit from the book: the engine waits for
`ponderhit` or `stop` without searching, and that wait earns no credit. Terminal and
proven-mate results can return sooner. The hard deadline starts at the hit or
matching `go`. Position-command tokens and full move history must match; new-game
and option commands invalidate pending credit.
Pondering performs native searches without online tablebase requests. Optional
`PonderDiagnostics` reports ponder time, hit and final search depth, node counts,
and own-clock time through UCI `info string` lines. `hit_kind` distinguishes
standard hits (`uci`) from Lucas handoffs (`stop_go`). `reinvest`, `shallow_hit`,
`deteriorating`, and `unstable` are 1 or 0, and `hit_cap_ms` is the extra-time
cap that applies when `reinvest` is 1. These diagnostics go to
UCI stdout; this Rust engine does not write Python Remus5's per-process JSON logs.

## Source and dependencies

The [source folder](../../../engines_instructor/Remus5-Rust/) implements chess
with bitboards, copy-make positions, and a fixed 512 MiB transposition table.
The native chess algorithms match [Python Remus5](../Remus5/README.md), including
its [positional evaluation](../Remus5/README.md#positional-evaluation).
The board representation differs: Python uses a 64-element list; Rust uses
bitboards. Both validate moves natively.
`search.rs` polls the UCI search controller for cancellation and ponder timing.
`main.rs`, `search_control.rs`, and `tablebase.rs` implement the boundary behavior.

| File | Code lines | Responsibility |
| --- | ---: | --- |
| `src/main.rs` | 294 | UCI options, legal predictions, root lookup, and search control |
| `src/search_control.rs` | 275 | Input queue, cancellation, and the ponder clock |
| `src/tablebase.rs` | 253 | FEN, HTTPS, JSON validation, cache, cooldown, and bounded wait |
| `src/board.rs` | 204 | Native position, bitboards, keys, and UCI move text |
| `src/attacks.rs` | 147 | Attacks and king safety |
| `src/make_move.rs` | 75 | Copy-make moves and halfmove counter |
| `src/move_generation.rs` | 166 | Pseudo-legal moves and special moves |
| `src/evaluation.rs` | 408 | Material, positional weaknesses, outposts, and king cover |
| `src/exchange.rs` | 99 | Static exchange evaluation |
| `src/search.rs` | 483 | Alpha-beta search, cancellation, and ponder timing |
| `src/transposition_table.rs` | 154 | Fixed 512 MiB search table, multiply-shift indexing, and Windows allocation |
| `src/opening_book.rs` | 48 | Embedded opening lines |
| **Total** | **2606** | |

Counts exclude blank lines and comments, including Rust documentation comments.
The attack lookups, the legal-move test, move ordering, the central-hole term,
the static evaluation, and the table probe use the
[speed techniques of the Rust engines 5 through 7](../remus_versions.md#speed-techniques-of-the-rust-engines-5-through-7),
which leave the searched positions and every score unchanged.
`Cargo.toml` and `Cargo.lock` hold build settings and dependency versions;
`rust-toolchain.toml` and `.cargo/config.toml` select the toolchain and CPU target.
`opening_book.txt` supplies the embedded book.

The Python call tree shows the corresponding engine flow; Rust keeps adapter state
in `Tablebase` methods and native search state in `Search`.

![Corresponding Python module overview](../Remus5/mermaid/modules.svg)

[Function call diagrams](../Remus5/README.md#the-call-tree) show the native helpers.

`reqwest` supplies HTTPS with certificate validation, `serde_json` parses JSON,
and `httpdate` handles HTTP retry dates. These dependencies do not implement chess
rules or search. The compiled executable includes the Rust dependencies and uses
profile-guided optimization. Maintaining source requires the Rust toolchain; the
workspace's designer build and verification tools accept `--engine 5`.

## Transposition-table memory

The engine allocates a fixed 512 MiB table at startup: 8,388,608 buckets of four
16-byte entries. Multiply-shift indexing selects the bucket. Matching keys update
in place; full buckets replace the entry with the lowest depth minus its search
age penalty. Windows supplies zero-filled pages on first access and releases the
region when the table is dropped. An unsuccessful allocation disables caching
while leaving native search available.

There is no UCI Hash setting. Each engine process owns its 512 MiB allocation, and
other process memory is additional. Python Remus5 uses the same allocation size,
bucket dimensions, and replacement policy through standard-library `mmap` and
`struct`; its position keys differ from Rust's keys.

## UCI and verification behavior

The engine accepts `position startpos moves ...`, and retains `position fen`,
`go depth`, `go perft`, and `eval` for designer analysis. Fixed-depth and perft
commands bypass tablebases. A timed `go` may query the service, with at most one
second and one quarter of its hard move budget spent waiting. The native search
receives the remaining time.

Tablebase categories, FEN fields, repetition guards, cache limits, and failure
handling match the [Python adapter](../Remus5/README.md). An accepted reply prints
`info string Lichess tablebase`; any unusable reply leaves move selection to the
native search. A tablebase result is not stored in the native transposition table.
