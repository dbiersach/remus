//! Native alpha-beta search with Young Brothers Wait sibling splitting.
//! One iterative-deepening owner coordinates persistent workers and a shared table.

use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::attacks::{QUEEN_LINES, in_check};
use crate::board::{BISHOP, KING, KNIGHT, ROOK};
use crate::board::{
    EMPTY, MATE_SCORE, Move, NO_MOVE, PAWN, Position, QUEEN, move_promotion, move_start,
    move_target, move_text,
};
use crate::evaluation::{PIECE_VALUES, evaluate};
use crate::exchange::exchange_score;
use crate::make_move::{make_move, make_null_move};
use crate::move_generation::{MoveList, StackList, generate_captures, generate_moves};
use crate::parallel_search::{Candidate, Job, Pool, Progress, SPLIT_DEPTH, SPLIT_PLY, Split};
use crate::search_control::SearchControl;
use crate::search_metrics::{Research, SearchMetrics};
use crate::table_metrics::TableMetrics;
use crate::transposition_table::{EXACT, LOWER, Table, UPPER};

const FUTILITY_MARGIN: i32 = 200; // A quiet move seldom gains more than this
const DELTA_MARGIN: i32 = 200; // A capture seldom gains more than its victim plus this
const ASPIRATION_WINDOW: i32 = 50; // The first window around the last score
const CAPTURE: i32 = 10_000; // Every capture and queen promotion scores above this
const HISTORY_LIMIT: i32 = 3999; // Quiet history stays below both killer moves
const MAX_PLY: usize = 128; // Killer moves are kept for this many plies
const MAX_DEPTH: i32 = 64; // Iterative deepening stops here
const EARLY_PICKS: usize = 3; // Moves chosen one at a time before the rest are sorted
const MAX_HELP_DEPTH: usize = 2; // Bound suspended search frames on each worker stack
pub const DEFAULT_THREADS: usize = 4; // Search threads, including the main thread
pub const MAX_THREADS: usize = 64; // The largest Threads value accepted

/// The sort entries of one position's moves.
type EntryList = StackList<u64, 256>;

/// The search stops by returning this error up through every call.
struct SearchTimeout;

/// What one search thread keeps from one position to the next.
struct Worker {
    nodes: u64,
    metrics: TableMetrics,
    work: SearchMetrics,
    research_level: u32,
    help_depth: usize,
    pending_nodes: u64,
    generation: u64,
    pool: Arc<Pool>,
    branch: Option<Arc<Split>>,
    control: Option<SearchControl>, // Only the main thread reads commands and the clock
    stop: Arc<AtomicBool>,
    total_nodes: Arc<AtomicU64>,
    best_move: Move,
    history: Vec<u64>,
    killers: [[Move; 2]; MAX_PLY],
    quiet_history: [[[i32; 64]; 64]; 2],
    table: Arc<Table>,
}

/// The main thread's worker, its helpers, and the table they share.
pub struct Search {
    main: Worker,
    helpers: Vec<(mpsc::Sender<Job>, JoinHandle<()>)>,
    pool: Arc<Pool>,
    stop: Arc<AtomicBool>,
    total_nodes: Arc<AtomicU64>,
    table: Arc<Table>,
    pub diagnostics: bool,
}

/// Return a list entry that sorts by priority and then by move, as Remus5 sorts the
/// pairs (priority, move).
#[inline(always)]
fn sort_entry(priority: i32, mv: Move) -> u64 {
    (u64::from(priority as u32) << 16) | u64::from(mv)
}

#[inline(always)]
fn entry_move(entry: u64) -> Move {
    entry as Move
}

/// Return the captured kind, including a pawn taken en passant, or EMPTY.
#[inline(always)]
fn captured_kind(position: &Position, mv: Move) -> u8 {
    if position.board[move_start(mv)] & 7 == PAWN && move_target(mv) as i8 == position.en_passant {
        return PAWN;
    }
    position.board[move_target(mv)] & 7
}

/// The priority of a move before the table, killers, and history have their say.
#[inline(always)]
fn capture_priority(position: &Position, mv: Move) -> i32 {
    let mut priority = 0;
    let victim = captured_kind(position, mv);
    if victim != EMPTY {
        let attacker = PIECE_VALUES[(position.board[move_start(mv)] & 7) as usize];
        priority = CAPTURE + 10 * PIECE_VALUES[victim as usize] - attacker;
    }
    if move_promotion(mv) == QUEEN {
        priority += CAPTURE + 9000;
    }
    priority
}

/// Fill the entries with the moves of the position and their priorities. The entries
/// stand in the order of generation: place_next() or a sort puts the likely best first.
#[inline]
fn ordered_moves(
    position: &Position,
    first: Move,
    killers: [Move; 2],
    quiet_history: &[[i32; 64]; 64],
    entries: &mut EntryList,
) {
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for &mut mv in list.as_mut_slice() {
        let mut priority = capture_priority(position, mv);
        if captured_kind(position, mv) == EMPTY && move_promotion(mv) == 0 {
            priority = quiet_history[move_start(mv)][move_target(mv)];
            if mv == killers[0] {
                priority = 5000;
            } else if mv == killers[1] {
                priority = 4000;
            }
        }
        if mv == first {
            priority = 1_000_000;
        }
        entries.add(sort_entry(priority, mv));
    }
}

/// Put the entry that belongs at this index in its place, so the moves come out in
/// the order a full sort gives. Each of the first few indexes takes the largest entry
/// left, which costs one pass over the list. The index after those sorts the rest.
/// Most cutoffs come from the first few moves, and then the sort never runs.
#[inline(always)]
fn place_next(entries: &mut [u64], index: usize) {
    if index < EARLY_PICKS {
        let mut best = index;
        for other in index + 1..entries.len() {
            if entries[other] > entries[best] {
                best = other;
            }
        }
        entries.swap(index, best);
    } else if index == EARLY_PICKS {
        entries[index..].sort_unstable_by(|a, b| b.cmp(a));
    }
}

/// Return true if the side to move has a legal move. The caller says whether that
/// side is in check.
fn has_legal_move(position: &Position, check: bool) -> bool {
    if !check {
        // Out of check, a move can leave its own king attacked only by moving the
        // king, by capturing en passant, or by leaving a line through the king. A
        // pawn off those lines with an empty square ahead has a legal step, and three
        // bitboard operations find one without generating a move.
        let own = position.by_side[usize::from(!position.white)];
        let king = (own & position.by_kind[KING as usize]).trailing_zeros() as usize;
        let free = own & position.by_kind[PAWN as usize] & !QUEEN_LINES[king & 63];
        let ahead = if position.white { free >> 8 } else { free << 8 };
        if ahead & !position.occupied() != 0 {
            return true;
        }
    }
    // Stop at the first move that leaves the moving side's king safe
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for &mut mv in list.as_mut_slice() {
        if !in_check(&make_move(position, mv), position.white) {
            return true;
        }
    }
    false
}

/// Return true if the side has a piece other than its king and pawns.
#[inline(always)]
fn has_pieces(position: &Position) -> bool {
    let kinds = &position.by_kind;
    let pieces = kinds[KNIGHT as usize]
        | kinds[BISHOP as usize]
        | kinds[ROOK as usize]
        | kinds[QUEEN as usize];
    pieces & position.by_side[usize::from(!position.white)] != 0
}

impl Search {
    pub fn new() -> Search {
        let stop = Arc::new(AtomicBool::new(false));
        let total_nodes = Arc::new(AtomicU64::new(0));
        let table = Arc::new(Table::new());
        let pool = Arc::new(Pool::new());
        let main = Worker::new(
            Some(SearchControl::new()),
            stop.clone(),
            total_nodes.clone(),
            table.clone(),
            pool.clone(),
        );
        let mut search = Search {
            main,
            helpers: Vec::new(),
            pool,
            stop,
            total_nodes,
            table,
            diagnostics: true,
        };
        search.set_threads(DEFAULT_THREADS);
        search
    }

    pub fn control(&mut self) -> &mut SearchControl {
        self.main.main_control()
    }

    /// Resize only between searches, after every split has drained its helpers.
    pub fn set_threads(&mut self, threads: usize) {
        self.shutdown_helpers();
        for id in 1..threads.clamp(1, MAX_THREADS) {
            let (sender, receiver) = mpsc::channel();
            let return_sender = sender.clone();
            let mut worker = Worker::new(
                None,
                self.stop.clone(),
                self.total_nodes.clone(),
                self.table.clone(),
                self.pool.clone(),
            );
            let started = std::thread::Builder::new()
                .name(format!("remus7-{id}"))
                .stack_size(crate::STACK_BYTES)
                .spawn(move || {
                    while let Ok(job) = receiver.recv() {
                        match job {
                            Job::Search(split) => {
                                let generation = worker.pool.generation.load(Relaxed);
                                if worker.generation != generation {
                                    worker.begin_search();
                                    worker.generation = generation;
                                }
                                let active = Instant::now();
                                let nodes = worker.nodes;
                                worker.work = SearchMetrics::default();
                                worker.research_level = u32::from(split.research);
                                worker.metrics = TableMetrics::default();
                                worker.history.clone_from(&split.history);
                                worker.branch = Some(split.clone());
                                worker.search_split(&split);
                                worker.branch = None;
                                worker.history.clear();
                                worker.flush_nodes();
                                worker.work.nodes = worker.nodes - nodes;
                                worker.work.jobs = 1;
                                worker.work.active_ns = (active.elapsed().as_nanos() as u64)
                                    .saturating_sub(worker.work.wait_ns);
                                if split.research {
                                    worker.work.research_nodes = worker.work.nodes;
                                }
                                worker.pool.add_worker_metrics(id, worker.work);
                                worker.pool.add_table_metrics(worker.metrics);
                                worker.pool.release(return_sender.clone());
                                split.helper_done();
                            }
                            Job::Quit => break,
                        }
                    }
                });
            match started {
                Ok(handle) => {
                    self.pool.release(sender.clone());
                    self.helpers.push((sender, handle));
                }
                Err(_) => {
                    println!(
                        "info string worker startup limited Threads to {}",
                        self.helpers.len() + 1
                    );
                    break;
                }
            }
        }
    }

    /// Drain and join the pool before releasing any reference to the shared table.
    pub fn set_hash(&mut self, mib: usize) {
        if self.table.size_mib() != mib {
            let threads = self.helpers.len() + 1;
            self.shutdown_helpers();
            self.main.table = Arc::new(Table::disabled());
            self.table = self.main.table.clone();
            self.table = Arc::new(Table::with_size(mib));
            self.main.table = self.table.clone();
            self.set_threads(threads);
        }
        crate::search_control::search_log(&format!(
            "hash requested_mib={mib} hash_mib={}",
            self.table.size_mib()
        ));
    }

    fn shutdown_helpers(&mut self) {
        for (sender, _) in &self.helpers {
            let _ = sender.send(Job::Quit);
        }
        for (_, handle) in self.helpers.drain(..) {
            handle.join().expect("search worker failed");
        }
        self.pool.clear();
    }

    pub fn ponder_reply(&self, position: &Position, mv: Move) -> Move {
        self.main.ponder_reply(position, mv)
    }

    pub fn think(
        &mut self,
        position: &Position,
        game_history: &[u64],
        seconds: f64,
        longest: f64,
        depth_limit: Option<i32>,
    ) -> Move {
        let started = Instant::now();
        self.table.new_search();
        self.stop.store(false, Relaxed);
        self.total_nodes.store(0, Relaxed);
        self.pool.begin();
        let active = Instant::now();
        let mv = self
            .main
            .think(position, game_history, seconds, longest, depth_limit);
        self.main.work.active_ns =
            (active.elapsed().as_nanos() as u64).saturating_sub(self.main.work.wait_ns);
        self.main.work.jobs = 1;
        self.main.work.nodes = self.main.nodes;
        self.stop.store(true, Relaxed);
        self.main.flush_nodes();
        let total = self.total_nodes.load(Relaxed);
        self.main.main_control().nodes = total;
        let mut metrics = self.main.metrics;
        metrics.add(self.pool.table_metrics());
        let hash_mib = self.table.size_mib();
        let worker_metrics = self.pool.worker_metrics();
        let mut work = self.main.work;
        for helper in worker_metrics.iter().skip(1).take(self.helpers.len()) {
            work.add(*helper);
        }
        if self.diagnostics {
            let summary = format!(
                "info string ybwc workers={} splits={} tasks={} cutoffs={} nodes={total}",
                self.helpers.len() + 1,
                self.pool.splits.load(Relaxed),
                self.pool.tasks.load(Relaxed),
                self.pool.cutoffs.load(Relaxed)
            );
            println!("{summary}");
            let seconds = started.elapsed().as_secs_f64();
            let hash_fill_pct = self.table.fill_percentage();
            for (id, helper) in worker_metrics
                .iter()
                .enumerate()
                .take(self.helpers.len() + 1)
            {
                let metrics = if id == 0 { self.main.work } else { *helper };
                self.main
                    .main_control()
                    .log_search(&format!("worker id={id} {metrics}"));
            }
            let active_pct = 100.0 * work.active_ns as f64
                / (seconds.max(0.000_001) * 1e9 * (self.helpers.len() + 1) as f64);
            self.main.main_control().log_search(&format!(
                "{summary} native_ms={:.1} nps={:.0} hash_mib={hash_mib} hash_fill_pct={hash_fill_pct:.2} {metrics} {work} worker_active_pct={active_pct:.2}",
                seconds * 1000.0,
                total as f64 / seconds.max(0.000_001)
            ));
        }
        mv
    }
}

impl Drop for Search {
    fn drop(&mut self) {
        self.shutdown_helpers();
    }
}

impl Worker {
    fn new(
        control: Option<SearchControl>,
        stop: Arc<AtomicBool>,
        total_nodes: Arc<AtomicU64>,
        table: Arc<Table>,
        pool: Arc<Pool>,
    ) -> Worker {
        Worker {
            nodes: 0,
            metrics: TableMetrics::default(),
            work: SearchMetrics::default(),
            research_level: 0,
            help_depth: 0,
            pending_nodes: 0,
            generation: 0,
            pool,
            branch: None,
            control,
            stop,
            total_nodes,
            best_move: NO_MOVE,
            history: Vec::with_capacity(1024),
            killers: [[NO_MOVE; 2]; MAX_PLY],
            quiet_history: [[[0; 64]; 64]; 2],
            table,
        }
    }

    /// The main thread's control; only the main worker reads commands and the clock.
    fn main_control(&mut self) -> &mut SearchControl {
        self.control
            .as_mut()
            .expect("only the main worker owns the control")
    }

    /// The positions all threads have searched: the shared total, which grows 1,024 at
    /// a time, plus this thread's positions since its last addition.
    fn reported_nodes(&self) -> u64 {
        self.total_nodes.load(Relaxed) + self.pending_nodes
    }

    fn flush_nodes(&mut self) {
        self.total_nodes.fetch_add(self.pending_nodes, Relaxed);
        self.pending_nodes = 0;
    }

    /// Only the main worker consumes UCI commands; all workers observe cancellation.
    fn poll(&mut self) -> Result<(), SearchTimeout> {
        let total = self.reported_nodes();
        if let Some(control) = &mut self.control
            && control.poll(total)
        {
            self.stop.store(true, Relaxed);
        }
        if self.stop.load(Relaxed)
            || self
                .branch
                .as_ref()
                .is_some_and(|branch| branch.is_canceled())
        {
            return Err(SearchTimeout);
        }
        Ok(())
    }

    #[inline(always)]
    fn count_node(&mut self) -> Result<(), SearchTimeout> {
        self.nodes += 1;
        self.pending_nodes += 1;
        if self.pending_nodes >= 1024 {
            self.flush_nodes();
            self.poll()?;
        }
        Ok(())
    }

    /// Count verification attempts; nested attempts contribute nodes only once per
    /// worker. Helper threads account their own work rather than borrowing node totals.
    #[allow(clippy::too_many_arguments)]
    fn research(
        &mut self,
        position: &Position,
        depth: i32,
        ply: usize,
        alpha: i32,
        beta: i32,
        kind: Research,
    ) -> Result<i32, SearchTimeout> {
        match kind {
            Research::Pvs => self.work.pvs_researches += 1,
            Research::Lmr => self.work.lmr_researches += 1,
            Research::Aspiration => self.work.aspiration_researches += 1,
        }
        let nodes = self.nodes;
        self.research_level += 1;
        let result = self.alpha_beta(position, depth, ply, alpha, beta, true);
        self.research_level -= 1;
        if self.research_level == 0 {
            self.work.research_nodes += self.nodes - nodes;
        }
        result
    }

    /// Prepare for a new search: recent cutoffs matter more than older ones.
    fn begin_search(&mut self) {
        self.nodes = 0;
        self.metrics = TableMetrics::default();
        self.work = SearchMetrics::default();
        self.research_level = 0;
        self.pending_nodes = 0;
        self.best_move = NO_MOVE;
        for side in &mut self.quiet_history {
            for row in side {
                for entry in row {
                    *entry /= 2;
                }
            }
        }
    }

    /// Search siblings using the alpha snapshot captured when each move is claimed.
    /// A successful reduced probe receives full-depth verification before publication.
    fn search_split(&mut self, split: &Arc<Split>) {
        while self.poll().is_ok() {
            let Some((candidate, alpha)) = split.claim() else {
                break;
            };
            if !self.search_split_move(split, candidate, alpha) {
                break;
            }
        }
    }

    fn search_split_move(&mut self, split: &Arc<Split>, candidate: Candidate, alpha: i32) -> bool {
        self.pool.tasks.fetch_add(1, Relaxed);
        if self.poll().is_err() {
            if split.is_canceled() {
                self.work.cutoff_discarded_tasks += 1;
            }
            return false;
        }
        let child = make_move(&split.position, candidate.mv);
        self.table.prefetch(child.key);
        let length = self.history.len();
        let nodes = self.nodes;
        let accounted = self.work.cutoff_discarded_nodes;
        let stopped = self.work.stop_discarded_nodes;
        let result = self.search_sibling(
            &child,
            split.depth,
            split.ply,
            alpha,
            split.beta,
            candidate.reduce,
        );
        self.history.truncate(length);
        if split.is_canceled() {
            self.work.cutoff_discarded_tasks += 1;
            self.work.cutoff_discarded_nodes +=
                (self.nodes - nodes).saturating_sub(self.work.cutoff_discarded_nodes - accounted);
            return false;
        }
        let Ok(score) = result else {
            self.work.stop_discarded_nodes +=
                (self.nodes - nodes).saturating_sub(self.work.stop_discarded_nodes - stopped);
            return false;
        };
        match split.publish(candidate.mv, score) {
            Some(true) => {
                self.pool.cutoffs.fetch_add(1, Relaxed);
            }
            Some(false) => {}
            None => {
                self.work.cutoff_discarded_tasks += 1;
                self.work.cutoff_discarded_nodes += (self.nodes - nodes)
                    .saturating_sub(self.work.cutoff_discarded_nodes - accounted);
            }
        }
        true
    }

    /// Borrow one descendant task, preserving the suspended search's full context.
    fn help_descendant(&mut self, scope: &Arc<Split>) -> bool {
        if self.help_depth >= MAX_HELP_DEPTH || self.poll().is_err() {
            return false;
        }
        let Some(task) = self.pool.try_help(scope) else {
            return false;
        };
        self.pool.end_wait(scope);
        let started = Instant::now();
        let waits = self.work.wait_ns;
        let nodes = self.nodes;
        let history = std::mem::replace(&mut self.history, task.split.history.clone());
        let branch = self.branch.replace(task.split.clone());
        let research = self.research_level;
        let best = self.best_move;
        self.research_level = u32::from(task.split.research);
        self.help_depth += 1;
        self.work.help_tasks += 1;
        self.search_split_move(&task.split, task.candidate, task.alpha);
        self.help_depth -= 1;
        if self.help_depth == 0 {
            self.work.help_nodes += self.nodes - nodes;
            self.work.help_ns +=
                (started.elapsed().as_nanos() as u64).saturating_sub(self.work.wait_ns - waits);
        }
        if research == 0 && task.split.research {
            self.work.research_nodes += self.nodes - nodes;
        }
        self.research_level = research;
        self.best_move = best;
        self.history = history;
        self.branch = branch;
        self.flush_nodes();
        // Completion is visible only after this worker restores its suspended state.
        drop(task);
        self.pool.begin_wait(scope);
        true
    }

    /// Help only descendants; cancellation drains participants without starting work.
    fn drain_split(&mut self, split: &Arc<Split>) {
        let can_help = self.help_depth < MAX_HELP_DEPTH;
        if can_help {
            self.pool.begin_wait(split);
        }
        loop {
            let mut state = split.progress.lock().unwrap();
            if state.helpers == 0 {
                state.closed = true;
                break;
            }
            drop(state);
            let _ = self.poll();
            let canceled = self.stop.load(Relaxed) || split.is_canceled();
            if !canceled && self.help_descendant(split) {
                continue;
            }
            let waiting = Instant::now();
            state = split.progress.lock().unwrap();
            if state.helpers != 0 {
                let (next, _) = split
                    .changed
                    .wait_timeout(state, Duration::from_millis(1))
                    .unwrap();
                state = next;
            }
            drop(state);
            let elapsed = waiting.elapsed().as_nanos() as u64;
            self.work.wait_ns += elapsed;
            // Classify by state at the start of the wait, with at most 1 ms resolution.
            if canceled {
                self.work.cancel_wait_ns += elapsed;
            } else {
                self.work.result_wait_ns += elapsed;
            }
        }
        if can_help {
            self.pool.end_wait(split);
        }
        self.pool.retire(split);
    }

    fn search_sibling(
        &mut self,
        child: &Position,
        depth: i32,
        ply: usize,
        alpha: i32,
        beta: i32,
        reduce: bool,
    ) -> Result<i32, SearchTimeout> {
        let mut score = -self.alpha_beta(
            child,
            depth - 1 - i32::from(reduce),
            ply + 1,
            -alpha - 1,
            -alpha,
            true,
        )?;
        if score > alpha && (reduce || score < beta) {
            score = -self.research(
                child,
                depth - 1,
                ply + 1,
                -beta,
                -alpha,
                if reduce { Research::Lmr } else { Research::Pvs },
            )?;
        }
        Ok(score)
    }

    /// Reserve real idle workers, then search and drain this split before returning.
    #[allow(clippy::too_many_arguments)]
    fn split_remaining(
        &mut self,
        position: &Position,
        entries: &mut [u64],
        depth: i32,
        ply: usize,
        alpha: i32,
        beta: i32,
        best_score: i32,
        best: Move,
        check: bool,
    ) -> Option<Result<(i32, Move, usize), SearchTimeout>> {
        if depth < SPLIT_DEPTH
            || ply > SPLIT_PLY
            || (!self.pool.has_idle() && !self.pool.has_waiter(self.branch.as_ref()))
        {
            return None;
        }
        // place_next() orders only the moves the search has reached, and a split
        // hands out every move left, so those go into order here
        entries.sort_unstable_by(|a, b| b.cmp(a));
        let mut candidates = Vec::with_capacity(entries.len());
        let mut legal_count = 1;
        for &entry in entries.iter() {
            let mv = entry_move(entry);
            let child = make_move(position, mv);
            if in_check(&child, position.white) {
                continue;
            }
            legal_count += 1;
            let quiet = captured_kind(position, mv) == EMPTY && move_promotion(mv) == 0;
            let calm = quiet && !check && !in_check(&child, !position.white);
            let reduce = depth >= 3 && legal_count > 3 && calm && !self.killers[ply].contains(&mv);
            candidates.push(Candidate { mv, reduce });
        }
        if candidates.len() < 2 {
            return None;
        }
        let helpers = self.pool.reserve(candidates.len() - 1);
        if helpers.is_empty() && !self.pool.has_waiter(self.branch.as_ref()) {
            return None;
        }
        let split = Arc::new(Split {
            position: *position,
            history: self.history.clone(),
            moves: candidates,
            depth,
            ply,
            beta,
            parent: self.branch.clone(),
            canceled: AtomicBool::new(false),
            research: self.research_level != 0,
            progress: Mutex::new(Progress {
                next: 0,
                helpers: helpers.len(),
                closed: false,
                alpha,
                score: best_score,
                best,
            }),
            changed: Condvar::new(),
        });
        self.pool.splits.fetch_add(1, Relaxed);
        self.pool.publish(&split);
        for helper in helpers {
            if helper.send(Job::Search(split.clone())).is_err() {
                split.helper_done();
            }
        }
        let parent = self.branch.replace(split.clone());
        self.search_split(&split);
        self.branch = parent;
        self.drain_split(&split);
        let state = split.progress.lock().unwrap();
        if split.is_canceled() {
            self.work.cutoff_unclaimed_tasks += (split.moves.len() - state.next) as u64;
        }
        let result = (state.score, state.best, legal_count);
        drop(state);
        Some(self.poll().map(|()| result))
    }

    /// Search captures and queen promotions, or every legal evasion in check.
    /// Checkmate and stalemate take precedence over standing pat or pruning.
    fn quiescence(
        &mut self,
        position: &Position,
        mut alpha: i32,
        beta: i32,
        ply: usize,
    ) -> Result<i32, SearchTimeout> {
        self.count_node()?;
        let white = position.white;
        let check = in_check(position, white);
        if !has_legal_move(position, check) {
            return Ok(if check { -(MATE_SCORE - ply as i32) } else { 0 });
        }
        if self.is_draw(position) {
            return Ok(0);
        }
        let stand_pat = evaluate(position);
        if ply >= MAX_PLY {
            return Ok(stand_pat); // Bound the stack during long checking sequences
        }
        if !check {
            if stand_pat >= beta {
                return Ok(stand_pat);
            }
            alpha = alpha.max(stand_pat);
        }

        let mut entries = EntryList::new();
        if check {
            ordered_moves(
                position,
                NO_MOVE,
                [NO_MOVE; 2],
                &self.quiet_history[usize::from(!white)],
                &mut entries,
            );
            entries.as_mut_slice().sort_unstable_by(|a, b| b.cmp(a));
        } else {
            let mut list = MoveList::new();
            generate_captures(position, &mut list);
            for &mut mv in list.as_mut_slice() {
                let priority = capture_priority(position, mv);
                if priority >= CAPTURE {
                    entries.add(sort_entry(priority, mv));
                }
            }
            entries.as_mut_slice().sort_unstable_by(|a, b| b.cmp(a));
        }
        for &mut entry in entries.as_mut_slice() {
            let mv = entry_move(entry);
            let child = make_move(position, mv);
            if in_check(&child, white) {
                continue;
            }
            if !check && move_promotion(mv) == 0 {
                let victim = PIECE_VALUES[captured_kind(position, mv) as usize];
                if stand_pat + victim + DELTA_MARGIN < alpha {
                    continue;
                }
                let exchange =
                    exchange_score(&position.board, &position.by_kind, &position.by_side, mv);
                if exchange < 0 {
                    continue;
                }
            }
            // Evasions can repeat a position, so quiescence keeps the path too.
            self.history.push(position.key);
            let result = self.quiescence(&child, -beta, -alpha, ply + 1);
            self.history.pop();
            let score = -result?;
            if score >= beta {
                return Ok(score);
            }
            alpha = alpha.max(score);
        }
        Ok(alpha)
    }

    /// Return true for the fifty-move rule or a repeated position.
    #[inline(always)]
    fn is_draw(&self, position: &Position) -> bool {
        if position.halfmove >= 100 {
            return true;
        }
        // The slice history[len - halfmove:] of Remus5, where a negative start counts
        // back from the end of the list
        let length = self.history.len() as i64;
        let mut start = length - i64::from(position.halfmove);
        if start < 0 {
            start = (length + start).max(0);
        }
        self.history[start as usize..].contains(&position.key)
    }

    /// Return the score for the side to move, searching depth moves ahead.
    fn alpha_beta(
        &mut self,
        position: &Position,
        mut depth: i32,
        ply: usize,
        mut alpha: i32,
        beta: i32,
        can_null: bool,
    ) -> Result<i32, SearchTimeout> {
        self.count_node()?;
        let white = position.white;
        let key = position.key;
        let check = in_check(position, white);
        if ply > 0 && self.is_draw(position) {
            if check && !has_legal_move(position, true) {
                return Ok(-(MATE_SCORE - ply as i32));
            }
            return Ok(0);
        }
        if ply >= MAX_PLY {
            return self.quiescence(position, alpha, beta, ply);
        }
        if check {
            depth += 1; // Never stop the search in check
        }
        if depth <= 0 {
            return self.quiescence(position, alpha, beta, ply);
        }
        let mut first = NO_MOVE;
        self.metrics.probes += 1;
        if let Some(stored) = self.table.probe(key, ply as i32) {
            self.metrics.hits += 1;
            first = stored.mv;
            if ply > 0
                && stored.depth >= depth
                && (stored.bound == EXACT
                    || (stored.bound == LOWER && stored.score >= beta)
                    || (stored.bound == UPPER && stored.score <= alpha))
            {
                self.metrics.cutoffs += 1;
                self.metrics.refreshes += u64::from(self.table.refresh(key, &stored));
                return Ok(stored.score);
            }
        }
        // Only two tests read the score, both out of check: null move at depth 3 and
        // up, and futility at depth 1. Every other node skips the evaluation.
        let needs_score = !check && (depth == 1 || (depth >= 3 && can_null && ply > 0));
        let static_score = if needs_score { evaluate(position) } else { 0 };
        // Null move: let the opponent move twice. If the side to move still stands above
        // beta with a shallower search, its real moves will too.
        if can_null
            && !check
            && ply > 0
            && depth >= 3
            && static_score >= beta
            && has_pieces(position)
            && has_legal_move(position, false)
        {
            let null = make_null_move(position);
            self.table.prefetch(null.key);
            self.history.push(key);
            let result = self.alpha_beta(&null, depth - 3, ply + 1, -beta, -beta + 1, false);
            self.history.pop();
            if -result? >= beta {
                return Ok(beta);
            }
        }
        self.history.push(key);
        let original_alpha = alpha;
        let result = (|| {
            let mut best_score = -MATE_SCORE - 1;
            let mut best = NO_MOVE;
            let mut legal_count = 0;
            let mut entries = EntryList::new();
            ordered_moves(
                position,
                first,
                self.killers[ply],
                &self.quiet_history[usize::from(!white)],
                &mut entries,
            );
            let entries = entries.as_mut_slice();
            for index in 0..entries.len() {
                place_next(entries, index);
                let mv = entry_move(entries[index]);
                let child = make_move(position, mv);
                if depth > 1 {
                    // The child looks itself up in the table; at depth 1 it goes
                    // straight to quiescence, which does not
                    self.table.prefetch(child.key);
                }
                if in_check(&child, white) {
                    continue; // Illegal: it leaves the king attacked
                }
                legal_count += 1;
                let quiet = captured_kind(position, mv) == EMPTY && move_promotion(mv) == 0;
                let calm = quiet && !check && !in_check(&child, !white);
                // Futility: one move from the leaves, a quiet move cannot lift a position far
                // below alpha
                if depth == 1 && calm && legal_count > 1 && static_score + FUTILITY_MARGIN <= alpha
                {
                    continue;
                }
                let mut score;
                if legal_count == 1 {
                    score = -self.alpha_beta(&child, depth - 1, ply + 1, -beta, -alpha, true)?;
                } else {
                    // Late move reduction: a late quiet move gets one ply less
                    let reduce = depth >= 3
                        && legal_count > 3
                        && calm
                        && mv != self.killers[ply][0]
                        && mv != self.killers[ply][1];
                    // Principal variation search: prove the move is no better than alpha with
                    // a null window, and search again if it is
                    let reduced_depth = depth - 1 - i32::from(reduce);
                    score = -self.alpha_beta(
                        &child,
                        reduced_depth,
                        ply + 1,
                        -alpha - 1,
                        -alpha,
                        true,
                    )?;
                    if score > alpha && (reduce || score < beta) {
                        score = -self.research(
                            &child,
                            depth - 1,
                            ply + 1,
                            -beta,
                            -alpha,
                            if reduce { Research::Lmr } else { Research::Pvs },
                        )?;
                    }
                }
                if score > best_score {
                    best_score = score;
                    best = mv;
                }
                if score > alpha {
                    alpha = score;
                    if ply == 0 {
                        self.best_move = mv;
                    }
                }
                if legal_count == 1
                    && score < beta
                    && let Some(result) = self.split_remaining(
                        position,
                        &mut entries[index + 1..],
                        depth,
                        ply,
                        alpha,
                        beta,
                        best_score,
                        best,
                        check,
                    )
                {
                    let (split_score, split_best, count) = result?;
                    best_score = split_score;
                    best = split_best;
                    legal_count = count;
                    if best_score > alpha && ply == 0 {
                        self.best_move = best;
                    }
                    if best_score >= beta {
                        self.record_cutoff(position, best, depth, ply);
                    }
                    break;
                }
                if score >= beta {
                    if quiet {
                        let entry = &mut self.quiet_history[usize::from(!white)][move_start(mv)]
                            [move_target(mv)];
                        *entry = HISTORY_LIMIT.min(*entry + depth * depth);
                        if self.killers[ply][0] != mv {
                            self.killers[ply] = [mv, self.killers[ply][0]];
                        }
                    }
                    break;
                }
            }
            Ok((best_score, best, legal_count))
        })();
        self.history.pop();
        let (best_score, best, legal_count) = result?;
        if legal_count == 0 {
            // Checkmate, worse the sooner it comes, or stalemate
            return Ok(if check { -(MATE_SCORE - ply as i32) } else { 0 });
        }
        let mut bound = EXACT;
        if best_score >= beta {
            bound = LOWER;
        } else if best_score <= original_alpha {
            bound = UPPER;
        }
        self.metrics.stores += 1;
        let stored = self
            .table
            .store(key, depth, best_score, bound, best, ply as i32);
        self.metrics.replacements += u64::from(stored.replaced);
        self.metrics.protected += u64::from(stored.protected);
        Ok(best_score)
    }

    fn record_cutoff(&mut self, position: &Position, mv: Move, depth: i32, ply: usize) {
        if captured_kind(position, mv) == EMPTY && move_promotion(mv) == 0 {
            let entry = &mut self.quiet_history[usize::from(!position.white)][move_start(mv)]
                [move_target(mv)];
            *entry = HISTORY_LIMIT.min(*entry + depth * depth);
            if self.killers[ply][0] != mv {
                self.killers[ply] = [mv, self.killers[ply][0]];
            }
        }
    }

    /// Return the line of best play that the table holds, as UCI text.
    fn principal_variation(&self, position: &Position, depth: i32) -> String {
        let mut line = String::new();
        let mut current = *position;
        for _ in 0..depth {
            let Some(stored) = self.table.probe(current.key, 0) else {
                break;
            };
            let mut moves = MoveList::new();
            generate_moves(&current, &mut moves);
            if !moves.as_mut_slice().contains(&stored.mv)
                || in_check(&make_move(&current, stored.mv), current.white)
            {
                break;
            }
            line.push(' ');
            line.push_str(&move_text(stored.mv));
            current = make_move(&current, stored.mv);
        }
        line
    }

    /// Return a legal cached reply after the selected move.
    fn ponder_reply(&self, position: &Position, mv: Move) -> Move {
        let child = make_move(position, mv);
        if let Some(stored) = self.table.probe(child.key, 0) {
            let mut moves = MoveList::new();
            generate_moves(&child, &mut moves);
            for &mut reply in moves.as_mut_slice() {
                if reply == stored.mv && !in_check(&make_move(&child, reply), child.white) {
                    return reply;
                }
            }
        }
        NO_MOVE
    }

    /// Search one move deeper at a time and return the best move, or NO_MOVE when there
    /// is no legal move. The search plans to use seconds, takes longer when the best
    /// move keeps changing, and never runs past longest. A depth limit replaces the
    /// clock, for the workshop designer's checks.
    fn think(
        &mut self,
        position: &Position,
        game_history: &[u64],
        mut seconds: f64,
        longest: f64,
        depth_limit: Option<i32>,
    ) -> Move {
        self.begin_search();
        let mut score = 0;
        let mut completed_move = NO_MOVE;
        let mut stdout = std::io::stdout();
        'deepening: for depth in 1..=MAX_DEPTH {
            let total = self.reported_nodes();
            if self.main_control().poll(total) {
                break;
            }
            let previous = self.best_move;
            let (mut alpha, mut beta) = (-MATE_SCORE - 1, MATE_SCORE + 1);
            if depth >= 4 {
                alpha = score - ASPIRATION_WINDOW;
                beta = score + ASPIRATION_WINDOW;
            }
            let mut retry = false;
            loop {
                self.history.clear();
                self.history.extend_from_slice(game_history);
                let attempt_nodes = self.nodes;
                let stopped_nodes = self.work.stop_discarded_nodes;
                let result = if retry {
                    self.research(position, depth, 0, alpha, beta, Research::Aspiration)
                } else {
                    self.alpha_beta(position, depth, 0, alpha, beta, true)
                };
                match result {
                    Ok(found) => score = found,
                    Err(SearchTimeout) => {
                        self.work.stop_discarded_nodes += (self.nodes - attempt_nodes)
                            .saturating_sub(self.work.stop_discarded_nodes - stopped_nodes);
                        break 'deepening;
                    }
                }
                // Outside the window: search again with that side open
                if score <= alpha {
                    alpha = -MATE_SCORE - 1;
                    retry = true;
                } else if score >= beta {
                    beta = MATE_SCORE + 1;
                    retry = true;
                } else {
                    break;
                }
            }
            if self.best_move == NO_MOVE {
                break; // No legal move
            }
            completed_move = self.best_move;
            let line = self.principal_variation(position, depth);
            let info = format!(
                "info depth {depth} score cp {score} nodes {} pv{line}",
                self.reported_nodes()
            );
            let _ = writeln!(stdout, "{info}");
            let _ = stdout.flush();
            // A best move that changes needs more time to settle
            let changed = previous != NO_MOVE && self.best_move != previous;
            if changed {
                seconds = (seconds * 1.5).min(longest);
            }
            self.main_control()
                .completed(depth, seconds / 2.0, score, changed);
            self.main_control().log_search(&info);
            if score.abs() > MATE_SCORE - 100 {
                break;
            }
            match depth_limit {
                Some(limit) if depth >= limit => break,
                None if self.main_control().soft_finished(seconds / 2.0) => break,
                _ => {}
            }
        }
        let total = self.reported_nodes();
        let control = self.main_control();
        control.nodes = total;
        if completed_move != NO_MOVE {
            self.best_move = completed_move;
        }
        self.best_move
    }
}
