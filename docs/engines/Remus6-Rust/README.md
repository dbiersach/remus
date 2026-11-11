# Remus6-Rust

Remus6-Rust is [Remus5-Rust](../Remus5/README.md#remus5-rust-and-use) with the
[Remus6 tablebase boundary](../Remus6/README.md): optional Lichess advice at the
root, native move validation, a bounded wait, response caching, rate-limit
cooldowns, and native-search fallback. It is an experimental instructor engine
outside the five-day workshop.

Select [`remus6_rust.exe`](../../../engines_instructor/Remus6-Rust/remus6_rust.exe)
as an external UCI engine in the chess GUI. It identifies itself as `Remus6-Rust`
and embeds its opening book. Windows 11 needs no Rust or Python installation to
run the executable. Its `LichessTablebase` option defaults to true; false disables
lookups. The engine also plays when the network or service is unavailable.

## Source and dependencies

The [source folder](../../../engines_instructor/Remus6-Rust/) uses the Remus5-Rust
bitboards, copy-make positions, search, evaluation, and a transposition table
with a fixed 2 GiB allocation.
The native modules match Remus5-Rust except for table capacity. Only `main.rs` and the additional
`tablebase.rs` implement the boundary behavior.

| File | Code lines | Responsibility |
| --- | ---: | --- |
| `src/main.rs` | 185 | UCI, engine option, root lookup, and remaining time |
| `src/tablebase.rs` | 253 | FEN, HTTPS, JSON validation, cache, cooldown, and bounded wait |
| `src/board.rs` | 204 | Native position, bitboards, keys, and UCI move text |
| `src/attacks.rs` | 140 | Attacks and king safety |
| `src/make_move.rs` | 75 | Copy-make moves and halfmove counter |
| `src/move_generation.rs` | 166 | Pseudo-legal moves and special moves |
| `src/evaluation.rs` | 311 | Static evaluation |
| `src/exchange.rs` | 99 | Static exchange evaluation |
| `src/search.rs` | 438 | Remus5-Rust native search |
| `src/transposition_table.rs` | 144 | Fixed 2 GiB search table, multiply-shift indexing, and Windows allocation |
| `src/opening_book.rs` | 48 | Embedded opening lines |
| **Total** | **2063** | |

Counts exclude blank lines and comments, including Rust documentation comments.
`Cargo.toml` and `Cargo.lock` hold build settings and dependency versions;
`rust-toolchain.toml` and `.cargo/config.toml` select the toolchain and CPU target.
`opening_book.txt` supplies the embedded book.

The Python call tree shows the corresponding engine flow; Rust keeps adapter state
in `Tablebase` methods and native search state in `Search`.

![Corresponding Python module overview](../Remus6/mermaid/modules.svg)

[Function call diagrams](../Remus6/README.md#the-call-tree) show the native helpers.

`reqwest` supplies HTTPS with certificate validation, `serde_json` parses JSON,
and `httpdate` handles HTTP retry dates. These dependencies do not implement chess
rules or search. The compiled executable includes the Rust dependencies and uses
profile-guided optimization. Maintaining source requires the Rust toolchain; the
workspace's designer build and verification tools accept `--engine 6`.

## Transposition-table memory

The engine allocates a fixed 2 GiB table at startup: 33,554,432 buckets of four
16-byte entries. Multiply-shift indexing selects the bucket. Matching keys update
in place; full buckets replace the entry with the lowest depth minus its search
age penalty. Windows supplies zero-filled pages on first access and releases the
region when the table is dropped. An unsuccessful allocation disables caching
while leaving native search available.

There is no UCI Hash setting. Each engine process owns its 2 GiB allocation, and
other process memory is additional. Python Remus6 uses the same allocation size,
bucket dimensions, and replacement policy through standard-library `mmap` and
`struct`; its position keys differ from Rust's keys.

## UCI and verification behavior

The engine accepts `position startpos moves ...`, and retains `position fen`,
`go depth`, `go perft`, and `eval` for designer analysis. Fixed-depth and perft
commands bypass tablebases. A timed `go` may query the service, with at most one
second and one quarter of its hard move budget spent waiting. The native search
receives the remaining time.

Tablebase categories, FEN fields, repetition guards, cache limits, and failure
handling match the [Python adapter](../Remus6/README.md). An accepted reply prints
`info string Lichess tablebase`; any unusable reply leaves move selection to the
native search. A tablebase result is not stored in the native transposition table.
