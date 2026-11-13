# Remus6-Rust

Remus6-Rust is an experimental Windows Rust engine using Lazy SMP. Its main
worker and helpers search the same root independently and share one transposition
table. The main worker selects the move. Native rules, evaluation, root-only
Lichess advice, embedded opening book, and GUI-driven pondering follow Remus5's
design. Student pairs do not type this engine.

Select `engines_instructor\Remus6-Rust\remus6_rust.exe` as an external UCI
engine. Disable GUI opening books, including explicit `<None>` in tournament
entries. Edit the supplied `opening_book.txt` and rebuild to change the embedded
book. Fixed-depth searches bypass book and tablebase advice.

| Option | Default | Purpose |
| --- | --- | --- |
| `Threads` | 4 | Search workers, including the main worker; 1 to 64 |
| `Hash` | 512 | Shared table size: 512, 1024, or 2048 MiB |
| `SearchDiagnostics` | true | Save search, timing, node-rate, and table-use measurements |
| `Ponder` | false | Return a legal predicted reply; GUI sends `go ponder` to start pondering |
| `PonderDiagnostics` | false | Save and report ponder timing separately |
| `LichessTablebase` | true | Permit bounded root advice with native legality checks and fallback |

Enable Ponder explicitly in Lucas's external and tournament engine entries to use
it. `ponderhit` continues the current search. Matching stop-and-go commands can
credit completed ponder work toward the move's soft target. Hard deadlines start
on the engine's clock. Ordinary searches can return a provisional root choice;
ponder maturity uses completed work. A stable hit reserves 25% of the plan.
With at least 60 seconds on the clock and no `movetime`, an uncertain hit reserves
50% of the plan and tries for one more completed depth, within one full plan and
half the hard budget. A hit is uncertain when its depth is below 8, the score fell
35 centipawns between iterations of depth 4 or more, or the best move changed in
the latest such iteration; the score and both signs carry across a Lucas
stop-and-go. A book position answers `go ponder` and a hit from the book: the
engine waits for `ponderhit` or `stop` without searching, and that wait earns no
credit. The `info string ponder` line reports `reinvest`, `shallow_hit`,
`deteriorating`, and `unstable` as 1 or 0, with the extra-time cap `hit_cap_ms`.

`SearchDiagnostics` writes `search_log.txt` beside the executable, independently of
`PonderDiagnostics`. The final `lazy_smp` summary includes the actual worker count
and exact total nodes after helpers join. Depth belongs to the main worker; node
totals include helpers. The search log includes `hash_fill_pct`, sampled across the shared table after
each native search. See [search logs and table experiments](../search_logging.md)
for field definitions, measurement limits, and cache resizing behavior.

The shared table protects deeper results for the same position and equal-depth
exact results against weaker bounds. A depth-qualified table cutoff refreshes the
entry's age once per search generation. Plain probes and PV reconstruction do not
refresh age. Collision replacement combines stored depth and search age.

Per-worker log lines report active elapsed time, explicit split-drain wait time,
and nodes. Summaries include PVS/LMR/aspiration verification counts and their node
cost, plus work discarded after split cutoffs or search stops. These elapsed-time
measurements are not operating-system CPU time; completed descendant table entries
can remain useful even when their enclosing task is discarded.

Build with stable Rust using `cargo build --release --locked` in the engine folder.
Native chess uses the standard library. `reqwest`, `serde_json`, and `httpdate`
serve only the tablebase boundary. The engine remains usable without network
access. Helpers are created for each search and joined before the next command
can replace the shared table.

## Source files

<!-- Code counts are generated from nonblank Rust lines excluding standalone comments. -->

| File | Rust code lines |
| --- | ---: |
| `attacks.rs` | 147 |
| `board.rs` | 204 |
| `evaluation.rs` | 408 |
| `exchange.rs` | 99 |
| `main.rs` | 336 |
| `make_move.rs` | 75 |
| `move_generation.rs` | 166 |
| `opening_book.rs` | 48 |
| `search.rs` | 764 |
| `search_control.rs` | 329 |
| `search_metrics.rs` | 57 |
| `table_metrics.rs` | 39 |
| `tablebase.rs` | 253 |
| `transposition_table.rs` | 253 |
| **Total** | **3,178** |

The attack lookups, the legal-move test, move ordering, the central-hole term,
the static evaluation, and the table probe use the
[speed techniques of the Rust engines 5 through 7](../remus_versions.md#speed-techniques-of-the-rust-engines-5-through-7),
which leave the searched positions and every score unchanged.

## Call structure

![Search and diagnostics](mermaid/search.svg)

[Mermaid source](mermaid/search.mmd)
