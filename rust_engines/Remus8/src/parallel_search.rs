//! YBWC splits use reserved idle workers and restricted descendant helping.
//! Waiting owners may claim unassigned work strictly below their own split.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::Relaxed};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex, Weak};

use crate::board::{Move, Position};
use crate::search_metrics::SearchMetrics;
use crate::table_metrics::TableMetrics;

pub const SPLIT_DEPTH: i32 = 5; // Smaller subtrees stay on the current worker
pub const SPLIT_PLY: usize = 16; // Bound split metadata along checking lines

#[derive(Clone, Copy)]
pub struct Candidate {
    pub mv: Move,
    pub reduce: bool,
}

pub enum Job {
    Search(Arc<Split>),
    Quit,
}

pub struct Pool {
    idle: Mutex<Vec<Sender<Job>>>,
    available: AtomicUsize,
    waiting: AtomicUsize,
    help: Mutex<HelpRegistry>,
    pub generation: AtomicU64,
    pub splits: AtomicU64,
    pub tasks: AtomicU64,
    pub cutoffs: AtomicU64,
    table_metrics: Mutex<TableMetrics>,
    worker_metrics: Mutex<[SearchMetrics; 64]>,
}

#[derive(Default)]
struct HelpRegistry {
    waiting: Vec<Arc<Split>>,
    open: Vec<Weak<Split>>,
}

/// Registration precedes execution and lasts through restoration of worker state.
pub struct HelpTask {
    pub split: Arc<Split>,
    pub candidate: Candidate,
    pub alpha: i32,
}

impl Drop for HelpTask {
    fn drop(&mut self) {
        self.split.helper_done();
    }
}

impl Pool {
    pub fn new() -> Self {
        Self {
            idle: Mutex::new(Vec::new()),
            available: AtomicUsize::new(0),
            waiting: AtomicUsize::new(0),
            help: Mutex::new(HelpRegistry::default()),
            generation: AtomicU64::new(0),
            splits: AtomicU64::new(0),
            tasks: AtomicU64::new(0),
            cutoffs: AtomicU64::new(0),
            table_metrics: Mutex::new(TableMetrics::default()),
            worker_metrics: Mutex::new([SearchMetrics::default(); 64]),
        }
    }

    pub fn add_worker_metrics(&self, id: usize, metrics: SearchMetrics) {
        self.worker_metrics.lock().unwrap()[id].add(metrics);
    }

    pub fn worker_metrics(&self) -> [SearchMetrics; 64] {
        *self.worker_metrics.lock().unwrap()
    }

    pub fn add_table_metrics(&self, metrics: TableMetrics) {
        self.table_metrics.lock().unwrap().add(metrics);
    }

    pub fn table_metrics(&self) -> TableMetrics {
        *self.table_metrics.lock().unwrap()
    }

    pub fn has_idle(&self) -> bool {
        self.available.load(Relaxed) != 0
    }

    pub fn has_waiter(&self, parent: Option<&Arc<Split>>) -> bool {
        if self.waiting.load(Relaxed) == 0 {
            return false;
        }
        let Some(parent) = parent else { return false };
        self.help.lock().unwrap().waiting.iter().any(|scope| {
            !scope.is_canceled() && (Arc::ptr_eq(parent, scope) || parent.descends_from(scope))
        })
    }

    pub fn begin_wait(&self, scope: &Arc<Split>) {
        let mut help = self.help.lock().unwrap();
        help.waiting.push(scope.clone());
        self.waiting.store(help.waiting.len(), Relaxed);
    }

    pub fn end_wait(&self, scope: &Arc<Split>) {
        let mut help = self.help.lock().unwrap();
        help.waiting.retain(|item| !Arc::ptr_eq(item, scope));
        self.waiting.store(help.waiting.len(), Relaxed);
    }

    pub fn publish(&self, split: &Arc<Split>) {
        self.help.lock().unwrap().open.push(Arc::downgrade(split));
    }

    pub fn retire(&self, split: &Arc<Split>) {
        self.help.lock().unwrap().open.retain(|item| {
            item.upgrade()
                .is_some_and(|item| !Arc::ptr_eq(&item, split))
        });
    }

    /// Registry-to-split is the only lock order; callers hold no split lock.
    pub fn try_help(&self, scope: &Arc<Split>) -> Option<HelpTask> {
        if scope.is_canceled() {
            return None;
        }
        let help = self.help.lock().unwrap();
        for item in help.open.iter().rev() {
            let Some(split) = item.upgrade() else {
                continue;
            };
            if !split.descends_from(scope) {
                continue;
            }
            let mut state = split.progress.lock().unwrap();
            if state.closed || split.is_canceled() || state.next >= split.moves.len() {
                continue;
            }
            let candidate = split.moves[state.next];
            state.next += 1;
            state.helpers += 1;
            let alpha = state.alpha;
            drop(state);
            return Some(HelpTask {
                split,
                candidate,
                alpha,
            });
        }
        None
    }

    pub fn release(&self, sender: Sender<Job>) {
        let mut idle = self.idle.lock().unwrap();
        idle.push(sender);
        self.available.store(idle.len(), Relaxed);
    }

    pub fn reserve(&self, limit: usize) -> Vec<Sender<Job>> {
        let mut idle = self.idle.lock().unwrap();
        let count = limit.min(idle.len());
        let start = idle.len() - count;
        let reserved = idle.split_off(start);
        self.available.store(idle.len(), Relaxed);
        reserved
    }

    pub fn clear(&self) {
        self.idle.lock().unwrap().clear();
        self.available.store(0, Relaxed);
        *self.help.lock().unwrap() = HelpRegistry::default();
        self.waiting.store(0, Relaxed);
    }

    pub fn begin(&self) {
        self.generation.fetch_add(1, Relaxed);
        self.splits.store(0, Relaxed);
        self.tasks.store(0, Relaxed);
        self.cutoffs.store(0, Relaxed);
        *self.table_metrics.lock().unwrap() = TableMetrics::default();
        *self.worker_metrics.lock().unwrap() = [SearchMetrics::default(); 64];
    }
}

pub struct Progress {
    pub next: usize,
    pub helpers: usize,
    pub closed: bool,
    pub alpha: i32,
    pub score: i32,
    pub best: Move,
}

pub struct Split {
    pub position: Position,
    pub history: Vec<u64>,
    pub moves: Vec<Candidate>,
    pub depth: i32,
    pub ply: usize,
    pub beta: i32,
    pub parent: Option<Arc<Split>>,
    pub canceled: AtomicBool,
    pub research: bool,
    pub progress: Mutex<Progress>,
    pub changed: Condvar,
}

impl Split {
    /// Strict ancestry prevents helping an ancestor, sibling, or unrelated root.
    pub fn descends_from(&self, ancestor: &Arc<Split>) -> bool {
        let mut parent = self.parent.as_ref();
        while let Some(split) = parent {
            if Arc::ptr_eq(split, ancestor) {
                return true;
            }
            parent = split.parent.as_ref();
        }
        false
    }
    /// A cancellation affects this split and descendants, not unrelated nodes.
    pub fn is_canceled(&self) -> bool {
        let mut scope = Some(self);
        while let Some(split) = scope {
            if split.canceled.load(Relaxed) {
                return true;
            }
            scope = split.parent.as_deref();
        }
        false
    }

    /// Claim a move and the alpha snapshot used for both probe and re-search.
    pub fn claim(&self) -> Option<(Candidate, i32)> {
        let mut state = self.progress.lock().unwrap();
        if state.closed || self.is_canceled() || state.next >= self.moves.len() {
            return None;
        }
        let candidate = self.moves[state.next];
        state.next += 1;
        Some((candidate, state.alpha))
    }

    /// Publish a verified full-depth result in the parent's score convention.
    pub fn publish(&self, mv: Move, score: i32) -> Option<bool> {
        let mut state = self.progress.lock().unwrap();
        if self.is_canceled() {
            return None;
        }
        if score > state.score {
            state.score = score;
            state.best = mv;
        }
        state.alpha = state.alpha.max(score);
        if score >= self.beta {
            self.canceled.store(true, Relaxed);
            self.changed.notify_all();
            return Some(true);
        }
        Some(false)
    }

    /// Called after a helper flushes its nodes and releases all task state.
    pub fn helper_done(&self) {
        let mut state = self.progress.lock().unwrap();
        state.helpers -= 1;
        self.changed.notify_all();
    }
}
