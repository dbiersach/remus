# Search decisions for Remus7-Rust

This page explains the implemented Rust engine and the experiments that can guide
further work. Sections describing current behavior refer to the production source.
Optimization candidates and research questions are proposals, not enabled features
or claims of measured gains. Runtime options and build instructions are in the
[engine README](README.md).

## Parallelism

YBWC coordinates one iterative-deepening search. Helpers search assigned siblings
after the first legal child completes. This differs from Remus6-Rust's Lazy SMP,
where helpers independently search the root and cooperate through the shared table.

Remus7 retains the existing Rust board representation, copy-make operations,
evaluation, shared table storage, and atomic table access. Thread creation is outside
the recursive search. A pool of persistent workers receives reserved jobs through
standard-library channels. Split results and work claims use a small mutex; chess
search and table probes execute outside that mutex.

An owner reserves idle helpers for channel jobs and consumes siblings itself.
While draining a split, it can claim one unassigned candidate from a strictly
descendant split. Waiting owners advertise this restricted availability; a
descendant can publish candidates even without an idle channel worker. The owner
still executes its candidates if nobody helps. Ancestry restrictions prevent
circular dependencies, and helping nests at most twice per worker. The main
worker polls input while helping and during bounded waits. Branch cancellation
follows split ancestry; it does not cancel unrelated work.

Full-depth sibling confirmations use their original alpha snapshot. A stale alpha
may waste work, but a failing probe remains below the current threshold. A
successful reduced probe is verified at full depth. Each move's reduction eligibility
comes from its original legal ordering and the owning worker's killer moves.

Every helper flushes node counts and releases its task history before reporting
completion. A split owner drains helpers before returning. The search's stop flag,
node totals, and table age can consequently be reused between sessions without old
jobs modifying the next search. All worker handles are joined on resize and exit.

The engine uses the last completed root iteration after cancellation. An iteration
that fails an aspiration window is retried only after its first attempt drains.
The shared table may retain completed descendant results, but an interrupted
ancestor never publishes an exact result.

### Why the first child matters

Suppose the first root move establishes +80 centipawns. A sibling can be rejected
once an opponent reply establishes that the sibling cannot exceed +80. Searching
the first move before distributing siblings gives helpers a useful threshold. At
an internal node, that first move may already reach beta, the threshold that makes
the remaining siblings unnecessary. Waiting avoids dispatching that work at all.

The rule applies separately at each eligible node. The first root subtree can
contain its own splits, so YBWC does not require that entire subtree to be searched
by one worker. Root-only splitting is a useful experimental control for measuring
the benefit of internal splits.

The [Distributed Game-Tree Search paper](https://journals.sagepub.com/doi/10.3233/ICG-1989-12203)
describes YBWC. Remus's particular thresholds, worker reservations, and memory
layout are implementation choices that need measurement on the target machines.

Parallelism also introduces overhead. Workers may explore branches that a later
alpha improvement would prune. Subtrees differ in size, and task dispatch, history
copies, shared counters, and result publication consume time. The objective is
less elapsed time for useful search, not merely more nodes per second.

### Ownership and module boundaries

| State | Current owner | Reason |
| --- | --- | --- |
| Move clock, commands, completed root choice | Main search worker and `SearchControl` | One component decides when to stop and emits UCI results |
| Killers, quiet history, local node counts | Each `Worker` | Frequent heuristic updates avoid shared locks |
| Position, repetition path, depth, root-relative ply | Worker executing the active subtree, initialized from its split | Repetition and mate distance depend on this task's ancestry |
| Candidates, alpha, beta, best result, helper count | `Split`, with mutex-protected `Progress` | Workers claim distinct moves and combine results consistently |
| Helper availability and search generation | `Pool` | Reservations bind jobs to actual idle workers |
| Cached position results | One shared `Table` | Workers reuse completed search information without allocating separate caches |

The active repetition path and the worker's heuristics serve different purposes.
When a helper receives a job, it copies that split's history. Recursive calls push
and restore keys, including when a search returns an interruption. The history
contains the parent key exactly once before the helper searches a child. Ply
continues to count from the actual root; restarting it at zero would corrupt mate
distances and the table's mate-score conversion.

Copy-make positions keep a parent's board intact while a worker searches a child.
Copy history at task transfer, not at every ordinary recursive call. Descendant
helping saves and restores each suspended path explicitly, along with its branch,
verification context, and root move. Killers and quiet history remain worker-local
heuristics. A private history list per thread alone is insufficient when that
thread holds several suspended tasks.

The source boundaries are:

| Source | Responsibility |
| --- | --- |
| [search.rs](../../../engines_instructor/Remus7-Rust/src/search.rs) | Worker state, negamax, quiescence, PVS, iterative deepening, and split participation |
| [parallel_search.rs](../../../engines_instructor/Remus7-Rust/src/parallel_search.rs) | Idle-helper reservations, candidates, split progress, and ancestor cancellation |
| [search_control.rs](../../../engines_instructor/Remus7-Rust/src/search_control.rs) | Input queue, command interpretation, clocks, pondering, and timing diagnostics |
| [transposition_table.rs](../../../engines_instructor/Remus7-Rust/src/transposition_table.rs) | Configurable allocation, atomic entry access, indexing, replacement, and mate-score conversion |
| [main.rs](../../../engines_instructor/Remus7-Rust/src/main.rs) | UCI options, root advice, legal fallback, and engine lifecycle |

### Split lifecycle and scheduling

1. Perform the ordinary terminal, draw, depth, table, and pruning checks.
2. Search the first legal child. Return immediately on a valid beta cutoff.
3. If the remaining work meets the split thresholds, build ordered legal
   candidates and reserve idle helpers under the pool lock.
4. Publish one shared split. Its owner and helpers claim siblings dynamically,
   recording the current alpha when each move is claimed.
5. Search outside the split lock. Publish the result under the lock and cancel
   the sibling group if a verified result reaches beta.
6. While draining, claim eligible descendant work one move at a time. A claim
   increments the descendant's participant count under its progress lock before
   execution. Restore suspended state and flush nodes before releasing that count.
7. Close the split under its progress lock when no participants remain, then retire
   it from the registry. Late claims cannot enter a closed split. Drain all work
   before returning or retrying the parent with a different aspiration window.

```text
Ordered legal children A, B, C, D, E
    |
    +-- A first: establish a bound, or return on cutoff
    |
    +-- owner claims B
    +-- helper claims C
    +-- helper claims D
    +-- next available participant claims E
```

The split lock protects short claims and result updates. Recursive search, table
access, command handling, and output execute outside that lock. A condition-variable
notification means progress may have occurred; the waiter must recheck the helper
count. The main worker uses bounded waits to keep processing UCI commands.

A generic recursive task pool can deadlock if every worker waits for queued child
tasks that nobody can execute. Remus reserves a specific idle helper before
channel dispatch. Restricted helping does not queue an obligation to an unavailable
worker: the split owner can execute all unclaimed moves itself. Helpers claim
only strict descendants of the split they are draining, never an ancestor or
unrelated branch. The registry lock precedes a split lock; neither is held during
search or command processing. Participant registration and split closure share
the progress lock to prevent completion races.

Cancellation disables further helping. Workers drain outstanding participants,
checking commands at bounded waits. `worker_result_wait_ms` measures explicit
waits begun before cancellation; `worker_cancel_wait_ms` measures waits begun after
scope cancellation or global stop. Their sum is `worker_wait_ms`, subject to display
rounding. Classification uses the state at wait entry, with a requested timeout
of one millisecond; scheduling delays can make a wait longer. Registry scans and
lock acquisition outside the explicit wait timer count as active elapsed time.
`help_tasks`, `help_nodes`, and `help_active_ms` measure borrowed work; nested helping
nodes and active time count once. These are subsets of total worker work, not
additional nodes or CPU utilization. General work stealing remains an experiment.

### Bounds and stale alpha

An alpha-beta score can be an exact value, an upper bound, or a lower bound. For
ordinary full-depth alpha-beta with parent window `(alpha_start, beta)`:

| Returned parent score | Interpretation | Effect |
| --- | --- | --- |
| At or below `alpha_start` | Upper bound unless exactness is independently established | Does not improve alpha |
| Strictly between the limits | Exact within that search model | Can improve the best move and alpha |
| At or above beta | Lower bound | Sufficient for a cutoff |

The window belongs to one parent node. Negamax gives its child the reversed,
negated window. Negating a returned bound also exchanges upper and lower bounds.
There is no global alpha for the whole engine.

For example, a helper starts at alpha 80, and another worker raises alpha to 110.
An exact score of 95 cannot improve the parent. An upper bound of 80 also cannot
improve it. But a null-window result establishing only a lower bound of 81 does
not say whether the move exceeds 110. That result cannot be accepted as an exact
score or discarded merely because its reported bound is less than 110.

Remus resolves a successful probe using its original alpha snapshot. The essential
operation in `search_sibling` is:

```rust
let mut score = -self.alpha_beta(
    child,
    depth - 1 - i32::from(reduce),
    ply + 1,
    -alpha - 1,
    -alpha,
    true,
)?;
if score > alpha && (reduce || score < beta) {
    score = -self.alpha_beta(child, depth - 1, ply + 1, -beta, -alpha, true)?;
}
```

This is an excerpt from the search method, not a standalone program. An unreduced
probe that already proves beta can cut off. A reduced probe that exceeds alpha
receives full-depth verification, including when it appears to exceed beta. A
reduced fail-low remains part of Remus's selective heuristic, not a full-depth
minimax proof. The bound table above does not turn selective pruning into an
exhaustive search guarantee.

Candidate ordering determines late-move reductions. Finishing early does not make
a late move an early move. Each split freezes reduction eligibility from the
original legal move order and the owner's killer moves.

Limiting simultaneous promising-move re-searches is an optimization candidate.
It may reduce redundant work but can also delay a valuable improvement. Updating
the window during an active probe requires explicit bound accounting; changing an
alpha variable alone is insufficient.

### Cancellation and failure limits

The whole-search stop flag handles time limits, stop, replacement, and shutdown.
Each split has a separate cancellation flag and a parent link. A beta cutoff
cancels that split's sibling work and descendant splits, leaving unrelated work
available. The owner records the cutoff evidence before signaling cancellation.

Cancellation is checked at task boundaries and periodically during recursive
search, alongside batches of 1,024 nodes. That interval is a tuning candidate, not
a wall-clock response guarantee. Measure stop latency in slow positions and with
different worker counts. During waits, check commands without holding the split
lock.

The current lifetime rule is to drain all helpers before returning from a split.
That rule permits reuse of the session stop flag and counters and prevents old
results from reaching a later aspiration attempt. Separate attempt identifiers
would become necessary if future scheduling allowed overlapping attempts or
undrained result messages; identifiers alone would not make those messages safe.

An interrupted task propagates `SearchTimeout`, not an invented score. Completed
descendant entries can remain in the table, while an unfinished ancestor must not
be labeled exact. The main worker keeps a completed root choice and the UCI layer
provides a legal fallback if interruption precedes the first completed iteration.

Helper startup failure reduces the worker count. Ordinary cancellation is handled
within search, but a Rust panic is different: the release profile uses
`panic = "abort"`. Graceful recovery from an arbitrary worker panic is not
implemented. A future recovery design would need explicit error propagation and
completion accounting so that no parent waits forever or accepts incomplete work.

### Shared-table and path-sensitive results

One allocation serves all workers: `Hash` selects 512 MiB, 1024 MiB, or
2048 MiB (2 GiB); the default is 512 MiB. Changing size drains work and joins helpers before releasing the old
allocation; the pool is recreated with the new shared table. A same-size request
preserves entries. Unsupported values are ignored. Table age advances once per new search,
not per helper, split, or standard ponder hit. Stacks, heuristics, and split
metadata are additional memory. Allocation failure disables the cache rather than
creating smaller or per-worker tables.

The Rust table protects deeper same-position results and equal-depth exact values.
Useful depth-qualified cutoffs refresh age with a single atomic compare-exchange.
The six age bits are excluded from the key/payload consistency check; score, move,
depth, and bound remain part of that check. Probes used only for PV reconstruction
do not refresh age. The table uses atomic words. This
is not a guarantee that a 16-byte record is written atomically. Finite keys and
concurrent publication still require collision-aware reasoning. Stress checks
should force competing reads and writes to the same bucket and accept legitimate
cache misses.

Striped bucket locks are an alternative reference design, not the production
implementation. Such a reference could help investigate entry-coherence questions:
all reads and replacement decisions for a bucket would use the same lock. A
nonblocking variant could treat a busy probe as a miss or skip a store. Measure
both extra misses and lock contention before considering either design; an
unlocked read of a lock-protected record is not an acceptable shortcut.

The position key does not encode the repetition path or halfmove clock. Native
draw checks precede applicable table cutoffs, but the reuse of ancestor bounds
influenced by history deserves dedicated fixtures. Parallel sharing cannot make a
path-dependent value independent of its history. A table-key or draw-policy change
is a separate chess-search change and must be validated independently.

The displayed principal variation is reconstructed from table moves that are
checked for legality. A shared table is not a transactional record of one complete
line. An explicit completed root line, and stopping reconstruction on a repeated
position, are possible improvements to investigate; current reconstruction is
depth-bounded and stops when no legal cached continuation is available.

### Root advice and command ownership

Only the coordinator consumes UCI commands and produces search reports. Helpers
perform native search and never issue tablebase requests. Root advice retains the
seven-piece limit, castling restriction, actual halfmove and en passant fields,
bounded wait, cooldown, native legality validation, and fallback behavior. Book
moves also pass native legality checks. Fixed-depth searches bypass both sources
of root advice.

The lookup wait is limited to one second and one quarter of the move budget, and
elapsed lookup time reduces the native search budget. At most one request is in
flight per engine process. HTTP 429 starts a cooldown of at least 60 seconds,
honoring a longer Retry-After. These boundaries apply independently of how many
search workers are available.

Workers publish node totals in batches and flush remainders before completion.
Interim totals may lag active work; final drained totals must not omit or count a
remainder twice. `Threads` changes take effect between searches, after active work
drains. Persistent workers must sleep when idle rather than continue background
analysis merely because a predicted ponder reply was returned.

## Ponder-hit time

The supplied game analysis describes two quick ponder-hit decisions around moves
21 and 22, with substantial time remaining. It suggests that treating all ponder
time as sufficient preparation can cause the engine to move before examining a
critical position for long enough on its own clock. The report also says longer
analysis preferred different moves, but estimates only a modest score difference.
Those observations motivate a time-policy experiment; they do not establish that
more time would have changed the result of that game.

`PonderHitMinPercent` defaults to 25. For a planned move budget of 4 seconds, the
engine ordinarily spends at least 1 second after the hit before its soft stop can
accept completed ponder work. The minimum is also limited by the same percentage
of the hard budget. A hard deadline, explicit stop, search replacement, terminal
position, or a completed mate can end work sooner.

The policy applies to standard `ponderhit` and to Lucas Chess's matching
stop-and-go sequence. It does not restart a standard ponder search or discard its
table, workers, and completed iterations. The stop-and-go form starts a search
using retained table/heuristic information and credits the previous ponder duration
once the required depth is complete.

The option can be set to 0 for direct comparison with immediate mature-hit behavior.
It is preferable to an unconditional fixed delay because the minimum scales down
with short clocks and remains subject to the existing hard deadline. A minimum
time allocation does not promise a ply of extra depth. The existing policy also
extends the soft plan when the preferred move changes.

### Lucas Chess default-option handling

The installed Lucas Chess R6 engine manager starts an engine with
`get_changed_options()`, not with every displayed option. In `Engine.set_uci_option`,
selecting a value equal to its advertised default removes that option from the
changed-options list. This includes selecting false and then true for an engine
whose Ponder default is true.

`EngineRun` begins with `control_ponder = None`. Its `_set_option` method creates
the `Ponder` controller only when it sends `setoption name Ponder value true`.
Consequently, a checked Ponder box and an engine's default-true declaration do not
by themselves activate this GUI controller. The engine can return legal predicted
replies while receiving no `go ponder` commands at all.

Remus7 advertises `Ponder=false`. Selecting true in Lucas therefore creates an
explicit saved override and activates the controller. The external-engine list
and an existing tournament's engine entries are separate saved copies; enable
Ponder in both. Existing entries can cache the engine's advertised defaults, so
refresh their UCI options from the executable before enabling Ponder if they show
a true default. Verify `go ponder` in the command log after changing settings.

The controller behavior can be reproduced with extracted methods from the
installed Python bytecode and an in-memory command sink, without starting a GUI,
engine, or game. Thread count is independent of this issue.

## Opening-book interpretation

The supplied report identifies a Berlin Defense line that ends at 5...Nd6, followed
by a native choice of 7...Ne4. The report recommends considering 7...Nf5 or avoiding
that continuation, and also reports that both engines evaluated Black's position
unfavorably.

Remus7 uses the current Remus6-Rust book. Its Berlin branch selects the Beverwijk
Variation with 4...Bc5, and its Ruy Lopez Exchange branch uses ...Bg4. These moves
are stored book choices, validated with the native generator, rather than search
or evaluation special cases. The moves are legal; improved playing strength is not
established by that fact.

Book selection, evaluation preferences, and search time are different variables.
`go depth N` bypasses the book for independent native-search measurements. Further book changes
should be checked for legality and evaluated at fixed depths in the exact relevant
positions. The evaluation has no opening-specific adjustment based on this report.

Lucas Chess external books must remain disabled, including explicit `<None>` in
tournament entries. Otherwise a GUI-selected move could be mistaken for Remus's
embedded-book or search choice. The engine validates every embedded-book move
using its own legal generator.

## Interpreting measurements

One-worker fixed-depth search is the reference for changes to concurrency. With
multiple workers, arrival order affects table contents and move ordering, and the
selective search may explore a different tree. Compare time to completed depth,
total nodes, chosen moves, and scores together. A higher node rate can accompany
more redundant work.

Use repeated fixed-position measurements with a fresh table and a retained table
as separate experiments. Compare one-worker Remus7 against both multiworker Remus7
and Remus6-Rust. Avoid extrapolating a tactical-position timing to every game.
Position evaluations, perft, and concurrency checks do not establish Elo gains.

The supplied match report contains five adjudicated or timed outcomes and says
that only the final game used the particular time-control change it discusses.
That sample does not establish the stronger engine, and deeper reported search
does not compensate automatically for an unfavorable opening or evaluation error.
Additional engine games require David's approval; fixed-position verification
does not start games.

### Measurements to collect

Record the executable build and optimization settings, thread count, Hash size, position and
history, target depth, diagnostic settings, elapsed time, total nodes, score, and
selected move. Keep fresh-table and retained-table experiments separate. Use
repeated runs and report both medians and spread. Match reductions and pruning
settings when the goal is to isolate scheduling overhead.

| Measure | Interpretation |
| --- | --- |
| One-worker Remus7 time / multiworker Remus7 time | Scaling within the same engine |
| Reference-engine time / Remus7 time | Practical improvement against the chosen reference and thread count |
| Parallel nodes / sequential nodes | Work inflation for the same controlled target |
| Time waiting for workers, claims, or results | Possible scheduler bottlenecks |
| Nodes searched after cancellation becomes available | Work lost to delayed cancellation |
| Confirming re-searches and simultaneous confirmations | Cost of promising moves found by several workers |
| Active-worker count and time in the first root subtree | Load balance and limits of root-only splitting |

The ordinary UCI output supplies depth, score, nodes, and principal variation.
`SearchDiagnostics` supplies worker count, split count, claimed sibling tasks,
cutoffs, and final nodes, and saves these alongside commands, completed iteration
lines, timing, and move sources in `search_log.txt`. Its native-search node rate
uses final drained nodes divided by native elapsed time. It measures throughput,
not useful-work efficiency. `PonderDiagnostics` separates ponder work from time on
the engine's clock. The two log switches are independent. Waiting time,
canceled-node counts, re-search counts, peak active
workers, and path-copy costs need additional profiling; current counters do not
provide those measurements.

Disable diagnostic output and file logging for clean timing comparisons. The
existing split/task/cutoff counters remain active when their summary is disabled,
so changing that option does not measure the counters' cost. Test intrusive
instrumentation separately from production-speed measurements.

Begin with physical-core counts, then test additional logical threads. Record
host load, processor affinity, and whether other engines are searching. Several
engines compete for CPU resources, memory bandwidth, and processor caches. A
thread count that helps one isolated engine may hurt a simultaneous tournament.

## Optimization experiments

The engine already uses persistent workers, local heuristics, batched node counts,
history copies at dispatch, stable candidates, PVS probes, and internal splits.
The following are controlled experiments, not instructions to enable every
possible optimization at once.

| Experiment | Rationale | Concern and acceptance evidence |
| --- | --- | --- |
| Compare root-only and internal splitting | Measure how much work is trapped in the first root subtree | Use the same search rules and report waiting time as well as elapsed time |
| Vary minimum split depth and maximum split ply | Balance dispatch cost against available parallel work | Test tactical, quiet, and endgame positions; depth 5 and ply 16 are current choices, not universal optima |
| Vary minimum remaining siblings | A single large sibling may justify a handoff in a different scheduler | The owner needs useful work elsewhere; otherwise handing off only adds latency |
| Limit simultaneous confirming re-searches | Avoid several expensive full-window searches after successful probes | Serialization may delay the best improvement; preserve bound correctness |
| Compare retained and reset worker heuristics | Measure the benefit of killer/history knowledge across moves | Include reproducibility, scores, and moves, not only throughput |
| Profile split allocation and history copying | Reduce bookkeeping without changing chess logic | Retain independent paths and bounded lifetimes for every active task |
| Batch split/task diagnostic counters | Reduce atomic writes shared by workers | Preserve final totals; measure the counters themselves rather than just toggling output |
| Vary cancellation polling interval | Reduce work after a cutoff or deadline | Report tail stop latency alongside throughput and deadline behavior |
| Compare atomic table access with a synchronized reference | Separate table-access costs from scheduling costs | More cache misses or altered ordering can outweigh lower synchronization cost |
| Broaden restricted helping to general work stealing | Keep workers useful beyond their own dependency subtree | Requires stronger lifetime, state-isolation, and deadlock arguments before performance claims |
| Compare post-hit percentages and position instability policies | Spend time where extra analysis may change the move | Keep the hard deadline and compare completed work, time use, and move quality |

Change one mechanism at a time. Per-node futures, globally shared mutable
heuristics, and overlapping aspiration retries introduce additional costs and
correctness obligations. Consider them only after a measured bottleneck identifies
a reason to do so.

## Verification and development order

Write expected results before running a check. Rules come first: the six published
perft positions must retain their counts before search results are trusted. A
refactor that should not alter search must retain deterministic one-worker scores,
moves, and node counts. Keep verification programs outside engine folders.

For a scheduler change, begin with a sequential reference and root-only dispatch,
then enable internal splits and selective techniques separately. A small exhaustive
tree can establish the scheduler's minimax result and optimal-move set. A separate
reference mode without selective pruning would strengthen chess-position checks;
the production engine does not expose such a mode. Legal output and a plausible
mate score alone do not prove that every parallel bound was combined correctly.

The maintained checks cover published perft, one-worker parity, positive mate
results, legal principal variations, repeated parallel searches, worker resizing,
stop, replacement, pondering, idle behavior, and shutdown. Focused synchronization
cases cover unique claims, stale-alpha publication, branch cancellation, helper
reservation/completion, and concurrent table access. These checks are a baseline,
not an exhaustive concurrency proof.

Additional targeted fixtures should cover the following expectations:

| Case | Required expectation |
| --- | --- |
| First legal child reaches beta | No siblings are dispatched at that split |
| Alpha changes during a null-window probe | The original window determines the result's meaning; a lower bound is not treated as exact |
| A reduced probe exceeds alpha or beta | Full-depth verification precedes acceptance of an improving result |
| Aspiration failure with active helpers | Helpers drain before the wider attempt; incomplete ancestors are not cached as exact |
| Same board under different histories or halfmove clocks | Task paths stay separate, and context-dependent draw values are not blindly reused |
| A mating node reached at different plies | Root-relative mate scores and table conversions agree |
| One or two workers with nested, unbalanced splits | Search terminates without waiting for unavailable workers |
| Stop during quiescence, split creation, or a result update | One legal fallback or completed result is returned; no fake score is published |
| Search replacement or thread resize | Old tasks cannot update the new search or outlive its state |
| Early/mature ponder hit, miss, terminal position, and stop-and-go | Hard timing, completed work, predictions, and one-result behavior remain correct |
| Unavailable table or failed helper startup | Search continues with the available resources |
| Tablebase failure, malformed response, or cooldown | Native fallback remains legal and workers make no HTTP requests |

Use barriers or controlled channels in external fixtures to force difficult
interleavings. Repeated stress runs alone may never reach the problematic ordering.
Audit error paths as well as successful completions, especially before introducing
panic recovery or changing the current drain-before-return rule.

Native chess behavior is shared with the Remus family. A change to common rules or
evaluation requires the corresponding cross-version checks. A concurrency-only
experiment should keep that chess baseline stable. Fixed-depth and protocol
checks do not authorize starting engine games.

## Research questions

1. **When does splitting pay for itself?** Which positions and remaining depths
   justify dispatch, path copying, synchronization, and extra speculative search?
2. **How much does internal splitting contribute?** Compare root-only scheduling
   with the current internal split policy and measure first-subtree utilization.
3. **Where is parallel time lost?** Separate idle time, split-lock contention,
   shared-counter traffic, table/cache contention, and work that becomes unnecessary.
4. **Which worker count and thresholds generalize?** Check different position types,
   physical versus logical cores, and simultaneous engine processes.
5. **How valuable are persistent heuristics?** Does retained killer/history data
   justify the extra run-to-run variation, including when worker assignment changes?
6. **Can confirming searches be coordinated better?** Would limiting simultaneous
   re-searches reduce wasted work without delaying a useful alpha improvement?
7. **Are history-sensitive table bounds sufficiently isolated?** Test positions
   with identical boards but different repetition paths or halfmove clocks before
   changing the cache key or draw policy.
8. **What should a stopped search return?** The implemented policy prefers the last
   completed iteration. A provisional-move alternative would need explicit evidence
   that partial work improves decisions, especially around ponder maturity.
9. **How much own-clock thinking should a hit receive?** Compare the current 25%
   floor with other fractions and instability-sensitive policies while preserving
   hard deadlines. More time does not automatically produce another completed ply.
10. **What evidence supports stronger play?** Keep opening-book choices, evaluation
    changes, scheduling, and time allocation as separate variables. A deeper search
    or a small match sample cannot establish the contribution of each one.

## Fixed-depth timing sample

These measurements use the development machine, depth 11, a fresh process and
table for each search, and the median of three runs. Startup is excluded. Both
engines use four search workers. Host load and processor affinity are not
controlled, so these are local measurements rather than portable performance
guarantees. Opening books and tablebase advice are bypassed by fixed-depth search.

| Position | Remus6-Rust seconds | Remus7-Rust seconds |
| --- | ---: | ---: |
| Starting position | 0.377 | 0.330 |
| Italian position | 0.697 | 0.326 |
| Kiwipete | 4.549 | 1.358 |
| Rook ending | 0.122 | 0.051 |
| Pawn race | 0.006 | 0.004 |

At four workers, the medians span roughly 1.1 to 3.4 times faster than the installed
Remus6-Rust executable in these positions. Eight workers improve some larger
searches but do not improve every position. The short pawn-race measurements are
especially sensitive to scheduling overhead.

The depth-11 parallel runs sometimes differ in score and selected move because
the selective search and table ordering differ. For example, the starting-position
scores across three four-worker Remus7 runs are +21, +29, and +21 centipawns.
The one-worker result is +14 in each run. These timings compare completed reported
depths, not identical search trees, and do not establish a playing-strength gain.


### Measuring table reuse

The search summary records the effective `hash_mib` and per-search probe, hit,
cutoff, store, and replacement counters. A hit matches a position key; a cutoff
also has sufficient depth and a usable bound, allowing the node to return early.
A replacement evicts a different key from an occupied bucket. In the atomic table
this records the worker's observed replacement decision, not a serialized count
of all physical overwrites. Principal-variation reconstruction is excluded.

Each worker increments ordinary local counters. Helpers merge them under one
short mutex at job completion, and the coordinator reports the aggregate after
work drains. Switching off diagnostics suppresses output but leaves counters active.
[Search logs and table experiments](../search_logging.md) defines the fields.

Compare 512, 1024, and 2048 MiB with the same positions, histories, thread counts,
and completed depths. Keep fresh and retained tables separate. Larger tables can
reduce replacements and search nodes while lowering raw NPS through memory-access
costs. Measure time to completed depth alongside useful cutoff rate; a high hit
rate alone does not establish stronger play or a benefit from more memory.

Worker timing, verification-node accounting, and discarded sibling costs are defined
in [search measurements](../search_logging.md#mt-replacement-policy-and-worker-efficiency).
