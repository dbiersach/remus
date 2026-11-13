# Search logs and table-size experiments

Remus4-Rust, Remus6-Rust, and Remus7-Rust offer `SearchDiagnostics`. Only
Remus6-Rust and Remus7-Rust offer `Hash`; Remus4-Rust uses a fixed 512 MiB table.

| Option | Default | Choices |
| --- | --- | --- |
| `SearchDiagnostics` | true | true or false |
| `Hash` (Remus6-Rust and Remus7-Rust) | 512 | 512, 1024, or 2048 MiB |

`Hash` is a UCI combo option. Its numbers are binary mebibytes: 1024 MiB is 1 GiB,
and 2048 MiB is 2 GiB. Set it in the engine options for the intended Lucas Chess
entry, including the tournament's separate engine entry. No engine restart is
needed for an idle process to accept this command:

```text
setoption name Hash value 1024
isready
```

Resizing in Remus6-Rust and Remus7-Rust occurs between searches. They cancel and
drain an active search before processing the option. Remus4-Rust ignores Hash
commands and retains its fixed allocation. Changing size clears the cache;
selecting the current effective size preserves it. Unsupported values are ignored.
The old allocation is released before allocating the replacement. Allocation
failure leaves native search available without caching, with `hash_mib=0` in the
log. There is no automatic smaller-table retry.

Each process owns its table. Remus6-Rust and Remus7-Rust share that table among
all their workers, rather than allocating the selected amount per worker. Memory
for stacks, positions, and other search state is additional.

## Recorded measurements

With `SearchDiagnostics=true`, each engine appends `search_log.txt` beside its own
executable. It records received commands, root identity, budgets, completed
iterations, returned moves and their sources, and a final native-search summary.
The summaries identify `serial`, `lazy_smp`, or `ybwc` search. Book selections and
tablebase selections have no native-search summary. An unavailable log file does
not stop play. Files grow across launches; archive them with the engine stopped.

Lines include the process ID, milliseconds since process startup, and `unix_ms`.
Search reports also include a per-process `search` ID, `wall_ms` for the current
invocation, `own_ms` on the engine's clock, and `completed_depth`. Incoming commands
in the multithreaded engines can precede completion of the previous search because
their command readers run independently. Use the search IDs to group reports.

Completed-iteration lines contain depth, `score cp`, cumulative nodes, and a
principal variation. Scores are from the root side's perspective, with the engine's
existing mate-range encoding. `completed_depth` describes completed work, not a
promise that every returned move belongs to that iteration: Remus4 and normal
Remus6 searches can return a provisional move from an interrupted iteration.
Remus7 prefers its last completed iteration when available.

| Final summary field | Meaning |
| --- | --- |
| `workers` | Actual search-worker count, including the main worker |
| `nodes` | All searched nodes, including unfinished iterations and worker remainders |
| `native_ms` | Native search wall time, including worker startup/drain where applicable |
| `nps` | Total nodes divided by native elapsed seconds, across all workers |
| `hash_mib` | Effective allocated table size; zero means caching is unavailable |
| `hash_fill_pct` | Remus6-Rust and Remus7-Rust: estimated occupied-slot percentage, including older searches |
| `tt_probes` | Alpha-beta table lookups, excluding PV extraction and predicted-reply lookup |
| `tt_hits` | Those probes that return a matching entry, regardless of usable depth |
| `tt_cutoffs` | Immediate returns justified by sufficient stored depth and a usable exact value or bound |
| `tt_stores` | Search attempts to store a completed node result |
| `tt_replacements` | Stores selecting an occupied slot for a different key |
| `tt_hit_pct` | 100 times hits divided by probes |
| `tt_cutoff_pct` | 100 times table cutoffs divided by probes |
| `tt_replacement_pct` | 100 times replacements divided by store attempts |

Zero denominators yield zero percentages. Quiescence nodes contribute to `nodes`
but do not perform these alpha-beta table probes. A node is a search visit, not
necessarily a unique position or a call to static evaluation. A matching entry can
improve move ordering without supplying a cutoff. Replacements exclude empty-slot
inserts and updates to the same key. If caching is unavailable, store attempts can
still be counted while no replacement occurs.

The atomic shared tables classify replacements using the entries observed while
choosing a slot. Concurrent writers can change a slot before publication, so these
are replacement decisions, not an exact global count of distinct lost positions.
Counts do not measure occupancy or prove which evicted entries would be useful.
YBWC's `cutoffs` field describes split-group cutoffs and is separate from
`tt_cutoffs`.

The two multithreaded engines sample 4,096 evenly spaced buckets (16,384 slots)
for `hash_fill_pct` once per completed or interrupted native search, after workers
finish. This estimates physical occupancy across all search ages; it is not the
hit rate, replacement rate, or percentage belonging to the current search.
Sampling reads the table without changing entries or adding per-node counters.
It is skipped when search diagnostics are disabled. An unavailable table reports
zero; use `hash_mib` to distinguish it from an empty allocated table. Two decimal
places describe the sample, not the accuracy of whole-table occupancy. Sampling
and log-writing time are outside `native_ms`, but remain part of response latency.

Tables persist across moves, pondering, and `ucinewgame`; only process restart or
an effective Hash-size change creates an empty table. Search-age changes affect
replacement preference, not occupancy. Full buckets replace individual entries
without clearing the entire allocation. Samples can report full occupancy while
many retained entries are old or irrelevant to the current position.

Counters are local to a search worker. Remus6 combines them after joining workers;
Remus7 publishes helper batches at task completion and reads the combined totals
after splits drain. No per-probe global atomic increment or file write is required.
Counters remain active with `SearchDiagnostics=false`; that switch disables log
output, not instrumentation. The counters and boundary file writes still have a
cost, so compare runs using the same instrumented builds and diagnostic settings.
`PonderDiagnostics` is independent where offered.

## Comparing sizes

Use the same positions, histories, worker count, executable build, and diagnostic
settings for 512, 1024, and 2048 MiB. Measure fresh-table and retained-table searches
separately, and repeat runs to report medians and variation. Record both elapsed
time to a completed depth and nodes, scores, moves, hits, cutoffs, and replacements.
A higher hit percentage alone does not prove a useful gain, and lower NPS can
accompany a faster result when the table prevents work.

Distinguish time on the engine's own clock from ponder time. A ponder miss searches
a different root from the actual move, though retained entries can still help.
Fixed-position checks do not establish playing strength. Starting new engine games
for strength measurements requires the workshop designer's approval.

## MT replacement policy and worker efficiency

Remus6-Rust and Remus7-Rust protect a matching entry when its stored depth
exceeds the incoming depth. At equal depth, an exact entry survives an incoming
lower or upper bound. Equal-depth exact updates and deeper updates are accepted.
For different keys, the least valuable slot in the four-entry bucket is selected
using `depth - 8 * searches_old`. Search age wraps after 64 generations. These
lock-free decisions use observed entries; concurrent stores can race.

Only a matching entry that supplies an immediate, depth-qualified search cutoff
receives an age refresh. The refresh uses a compare-exchange and does not change
score, move, depth, or bound. Age bits are excluded from the key-validation
checksum, allowing an age update in one atomic word. A changed payload, changed
key, or already-current age skips the refresh. PV extraction does not refresh age.

The MT summaries add these fields. Each `worker id=N` line carries the same search
ID as its final summary; worker zero is the main search worker. Node costs include
quiescence and interrupted work. Worker counters reset at each native search.

| Field | Meaning |
| --- | --- |
| `tt_protected` | Same-key store attempts rejected to preserve deeper or equal-depth exact data |
| `tt_refreshes` | Successful age updates on useful table cutoffs, not all table hits |
| `worker_nodes` | Sum of local worker nodes; equals final `nodes` |
| `worker_active_ms` | Assigned search elapsed time, excluding explicitly measured split-drain waiting |
| `worker_wait_ms` | Elapsed split-drain synchronization time; zero for Lazy SMP |
| `worker_result_wait_ms` | Remus7 explicit waits begun while results are needed and the split is not canceled |
| `worker_cancel_wait_ms` | Remus7 explicit waits begun after ancestor/split cancellation or global stop |
| `help_tasks` | Remus7 descendant candidates claimed by a waiting worker |
| `help_nodes` | Remus7 nodes searched during descendant helping; nested helping counts once, included in `worker_nodes` |
| `help_active_ms` | Remus7 helping elapsed time excluding nested explicit waits; included in `worker_active_ms` |
| `worker_jobs` | Assigned search jobs; Lazy SMP assigns one root search per worker |
| `worker_active_pct` | Active elapsed time summed across workers divided by `native_ms * workers`, as a percentage |
| `pvs_researches` | Full-window confirmations after successful unreduced null-window probes |
| `lmr_researches` | Full-depth confirmations after successful reduced probes; also use a full window |
| `aspiration_researches` | Root retries after an aspiration-window failure, including Lazy SMP helper roots |
| `research_nodes` | Local nodes executed within verification/retry work; nested verification nodes count once per worker |
| `cutoff_discarded_nodes` | Nodes in interrupted or unpublished sibling tasks whose split or ancestor has cut off |
| `cutoff_discarded_tasks` | Such sibling tasks; nested tasks are separate tasks |
| `cutoff_unclaimed_tasks` | Candidates left unclaimed in canceled split groups, counted once by each owner |
| `stop_discarded_nodes` | Nodes in interrupted root attempts or sibling tasks stopped by the global search deadline/command |

YBWC propagates the verification context to helper jobs, so helper verification
nodes contribute to `research_nodes` even when their own task does not retry.
Nested discarded-task nodes count once within each cancellation category per
worker. The cutoff and global-stop categories can overlap when an enclosing root
attempt is later stopped; do not add them as independent wasted-work totals.
A cutoff-discarded task can include work performed before the cutoff, and its
completed descendants may populate the table. These are discarded-task costs,
not an exact count of nodes executed after the cancellation instant.

Lazy SMP does not cancel other workers at a local beta cutoff, so its split-cutoff
fields are zero. Its interrupted helper root attempts appear in
`stop_discarded_nodes`. The metrics do not count duplicate positions across workers.

Active elapsed time includes operating-system scheduling delays, I/O and lock
contention outside the explicit split-drain timer. It is not CPU time. Unassigned
helper time and worker startup/drain account for the remaining capacity. Timing
occurs at job and drain boundaries, not on every node. Re-search counters and
node snapshots avoid recursive clock calls. Instrumentation stays active when
`SearchDiagnostics` disables output; compare the same builds and settings.

Remus7's result-wait and cancellation-wait fields partition `worker_wait_ms`, apart
from rounding. Classification uses cancellation state at the start of each bounded
wait, rather than an exact timestamp of a cutoff arriving during that wait. The
requested wait timeout is one millisecond; operating-system scheduling may delay
wakeup. Helping registry scans and locks outside the explicit timer count as
active elapsed time. Helping tasks can themselves contain PVS or LMR verification,
so `help_nodes` and `research_nodes` overlap and must not be added as separate costs.
Use completed-depth time and node counts alongside reduced waiting to judge helping.

An existing Remus6-Rust or Remus7-Rust Lucas Chess entry can retain an explicit Hash value such as 2048 even
when the executable advertises 512 by default. Set Hash to 512 in both the external
entry and the tournament entry when selecting the baseline allocation.

## Ponder handoff labels

Remus6-Rust and Remus7-Rust include `hit_kind` on search-log records and in ponder
reports: `none` means no recognized hit, `uci` means a standard `ponderhit`, and
`stop_go` means a matching explicit Lucas stop-and-go restart earns native credit.
A stopped ponder request and its credited restart have separate search IDs. Count
the restart as one hit; the stopped report alone does not prove a prediction miss.
The restart's credited ponder duration excludes the idle gap between commands.
See [UCI conversations](../chess_tutor/uci.md#lucass-stop-and-go-sequence-when-the-prediction-hits)
for the message sequence and matching rules.
