//! Read UCI commands during search; only the search thread owns chess state.

use std::collections::VecDeque;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Ponder,
    Infinite,
}

const SHALLOW_DEPTH: i32 = 8; // Below this completed depth, a hit deserves another look
const SCORE_DROP: i32 = 35; // Centipawns lost between mature completed iterations
const MATURE_DEPTH: i32 = 4; // Scores of shallower iterations swing too much to judge

/// What an explicitly stopped native ponder search hands to a matching go:
/// its elapsed duration, completed depth, mature score, and signs of doubt.
#[derive(Clone, Copy)]
pub struct PonderCredit {
    pub seconds: Duration,
    pub depth: i32,
    pub score: Option<i32>,
    pub deteriorating: bool,
    pub unstable: bool,
}

pub static LOG_ENABLED: AtomicBool = AtomicBool::new(false); // PonderDiagnostics controls output
pub static SEARCH_LOG_ENABLED: AtomicBool = AtomicBool::new(false); // SearchDiagnostics controls output
static LOG_START: OnceLock<Instant> = OnceLock::new();
static LOG_FILE: Mutex<()> = Mutex::new(()); // One thread writes a line at a time

/// Append one line to ponder_log.txt beside the executable while PonderDiagnostics
/// is on: the process id, milliseconds since the engine started, and the text.
pub fn log(text: &str) {
    if !LOG_ENABLED.load(Relaxed) {
        return;
    }
    append_log("ponder_log.txt", text);
}

/// Save search commands and reports independently of ponder logging.
pub fn search_log(text: &str) {
    if SEARCH_LOG_ENABLED.load(Relaxed) {
        let unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        append_log("search_log.txt", &format!("unix_ms={unix_ms} {text}"));
    }
}

/// Append beside the executable; unavailable diagnostic files never stop play.
fn append_log(filename: &str, text: &str) {
    let start = *LOG_START.get_or_init(Instant::now);
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let path = exe.with_file_name(filename);
    let _lock = LOG_FILE.lock();
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        let line = format!(
            "{} {ms:>11.1} {text}
",
            std::process::id()
        );
        let _ = file.write_all(line.as_bytes());
    }
}

pub struct SearchControl {
    commands: Receiver<String>,
    pending: VecDeque<String>,
    pub mode: Mode,
    pub stopped: bool,
    stop_received: bool,
    hit_kind: &'static str,
    started: Instant,
    longest: f64,
    search_started: Instant,
    pub was_ponder: bool,
    hit_time: Option<Instant>,
    pub nodes: u64,
    completed_depth: i32,
    hit_nodes: u64,
    hit_depth: i32,
    soft_limit: f64,
    pub can_reinvest: bool, // The clock holds a reserve for extra time after a hit
    pub source: &'static str, // "native", "book", or "tablebase"
    score: Option<i32>,     // The score of the deepest mature iteration
    score_depth: i32,
    deteriorating: bool, // The score fell by SCORE_DROP between mature iterations
    unstable: bool,      // The best move changed in the latest mature iteration
    uncertain_hit: bool, // The hit came on a shallow, falling, or changing result
    credit_depth: i32,   // The soft target applies once this depth is complete
    search_id: u64,
    diagnostics_started: Instant, // Current invocation, excluding earlier ponder credit
}

impl SearchControl {
    pub fn new() -> Self {
        LOG_START.get_or_init(Instant::now);
        let (sender, commands) = mpsc::channel();
        std::thread::spawn(move || {
            for line in std::io::stdin().lock().lines() {
                let Ok(line) = line else { break };
                log(&format!("< {line}"));
                search_log(&format!("< {line}"));
                if sender.send(line).is_err() {
                    return;
                }
            }
            let _ = sender.send("quit".to_string());
        });
        Self {
            commands,
            pending: VecDeque::new(),
            mode: Mode::Normal,
            stopped: false,
            stop_received: false,
            hit_kind: "none",
            started: Instant::now(),
            longest: f64::INFINITY,
            search_started: Instant::now(),
            was_ponder: false,
            hit_time: None,
            nodes: 0,
            completed_depth: 0,
            hit_nodes: 0,
            hit_depth: 0,
            soft_limit: f64::INFINITY,
            can_reinvest: false,
            source: "native",
            score: None,
            score_depth: 0,
            deteriorating: false,
            unstable: false,
            uncertain_hit: false,
            credit_depth: 1,
            search_id: 0,
            diagnostics_started: Instant::now(),
        }
    }

    pub fn next_command(&mut self) -> String {
        self.pending
            .pop_front()
            .unwrap_or_else(|| self.commands.recv().unwrap_or_else(|_| "quit".to_string()))
    }

    pub fn begin(&mut self, mode: Mode, longest: f64) {
        self.mode = mode;
        self.stopped = false;
        self.stop_received = false;
        self.hit_kind = "none";
        self.started = Instant::now();
        self.search_id += 1;
        self.diagnostics_started = self.started;
        self.longest = longest;
        self.search_started = self.started;
        self.was_ponder = mode == Mode::Ponder;
        self.hit_time = None;
        self.nodes = 0;
        self.completed_depth = 0;
        self.hit_nodes = 0;
        self.hit_depth = 0;
        self.soft_limit = f64::INFINITY;
        self.can_reinvest = false;
        self.source = "native";
        self.score = None;
        self.score_depth = 0;
        self.deteriorating = false;
        self.unstable = false;
        self.uncertain_hit = false;
        self.credit_depth = 1;
    }

    /// Tag reports with their search and separate wall time from own-clock time.
    pub fn log_search(&self, text: &str) {
        if SEARCH_LOG_ENABLED.load(Relaxed) {
            search_log(&format!(
                "search={} wall_ms={:.1} own_ms={:.1} completed_depth={} hit_kind={} {text}",
                self.search_id,
                self.diagnostics_started.elapsed().as_secs_f64() * 1000.0,
                self.elapsed() * 1000.0,
                self.completed_depth,
                self.hit_kind
            ));
        }
    }

    fn handle(&mut self, line: String) {
        match line.split_whitespace().next() {
            Some("isready") => {
                let mut out = std::io::stdout();
                let _ = writeln!(out, "readyok");
                let _ = out.flush();
            }
            Some("ponderhit") => {
                if self.mode == Mode::Ponder {
                    self.mode = Mode::Normal;
                    self.started = Instant::now();
                    self.hit_time = Some(self.started);
                    self.hit_kind = "uci";
                    self.hit_nodes = self.nodes;
                    self.hit_depth = self.completed_depth;
                    self.uncertain_hit =
                        self.hit_depth < SHALLOW_DEPTH || self.deteriorating || self.unstable;
                    self.log_search(&format!("ponderhit hit_nodes={}", self.hit_nodes));
                }
            }
            Some("stop") => {
                self.stopped = true;
                self.stop_received = true;
            }
            Some("position" | "go" | "setoption" | "ucinewgame" | "quit") => {
                self.pending.push_back(line);
                self.stopped = true;
            }
            _ => {}
        }
    }

    pub fn poll(&mut self, nodes: u64) -> bool {
        self.nodes = nodes;
        while !self.stopped {
            let Ok(line) = self.commands.try_recv() else {
                break;
            };
            self.handle(line);
        }
        self.stopped
            || (self.mode == Mode::Normal
                && (self.elapsed() >= self.longest
                    || (self.was_ponder
                        && self.completed_depth >= self.credit_depth
                        && self.search_started.elapsed().as_secs_f64() >= self.soft_limit
                        && self.hit_finished())))
    }

    /// Reserve a quarter of the adjusted plan, within a quarter of the hard limit.
    fn hit_minimum(&self) -> f64 {
        (self.soft_limit / 2.0).min(self.longest / 4.0)
    }

    /// Return true when a hit earns extra time: the clock holds a reserve and the
    /// result is shallow, falling, or changing.
    fn reinvest(&self) -> bool {
        self.can_reinvest && (self.uncertain_hit || self.deteriorating || self.unstable)
    }

    /// Return true when the search has spent enough of its own clock after a hit. A
    /// stable hit needs the ordinary reserve. An uncertain hit reserves half the plan
    /// and aims for one more completed depth, within one full plan and half the hard
    /// budget.
    fn hit_finished(&self) -> bool {
        if !self.reinvest() {
            return self.elapsed() >= self.hit_minimum();
        }
        let minimum = self
            .soft_limit
            .min(self.longest / 2.0)
            .max(self.hit_minimum());
        let limit = (2.0 * self.soft_limit).min(self.longest / 2.0);
        self.elapsed() >= minimum
            && (self.completed_depth > self.hit_depth || self.elapsed() >= limit)
    }

    /// Apply the soft limit without bypassing the thinking time a hit reserves.
    pub fn soft_finished(&self, target: f64) -> bool {
        self.elapsed() > target
            && (!self.was_ponder
                || (self.completed_depth >= self.credit_depth && self.hit_finished()))
    }

    pub fn wait(&mut self) {
        while self.mode != Mode::Normal && !self.stopped {
            let line = self.commands.recv().unwrap_or_else(|_| "quit".to_string());
            self.handle(line);
        }
    }

    pub fn elapsed(&self) -> f64 {
        if self.mode == Mode::Normal {
            self.started.elapsed().as_secs_f64()
        } else {
            0.0
        }
    }

    /// Remember native ponder work only after an explicit stop without a hit.
    /// Book waiting and cancellation by replacement commands earn no credit.
    pub fn ponder_credit(&self) -> Option<PonderCredit> {
        if self.was_ponder
            && self.hit_time.is_none()
            && self.stop_received
            && self.source == "native"
        {
            Some(PonderCredit {
                seconds: self.search_started.elapsed(),
                depth: self.completed_depth,
                score: self.score,
                deteriorating: self.deteriorating,
                unstable: self.unstable,
            })
        } else {
            None
        }
    }

    /// Treat this search as a ponder hit: some GUIs, Lucas Chess among them, answer a
    /// correct prediction with stop and a new go instead of ponderhit. The time spent
    /// pondering excludes the inter-command gap and counts toward the soft target once the new
    /// search, helped by the table, completes the depth the ponder search reached.
    /// The score and the signs of doubt of that search carry over.
    pub fn credit_ponder(&mut self, credit: PonderCredit) {
        self.was_ponder = true;
        self.credit_depth = credit.depth.max(1);
        self.search_started = self.started - credit.seconds;
        self.hit_time = Some(self.started);
        self.hit_depth = credit.depth;
        self.hit_kind = "stop_go";
        self.score = credit.score;
        self.score_depth = credit.depth;
        self.deteriorating = credit.deteriorating;
        self.unstable = credit.unstable;
        self.uncertain_hit =
            credit.depth < SHALLOW_DEPTH || credit.deteriorating || credit.unstable;
    }

    /// Record a completed depth, the soft target (half the planned seconds), the
    /// score, and whether the best move changed in this iteration.
    pub fn completed(&mut self, depth: i32, soft_limit: f64, score: i32, changed: bool) {
        self.completed_depth = depth;
        self.soft_limit = soft_limit;
        // The shallow iterations a Lucas restart replays keep the deeper score
        if depth >= MATURE_DEPTH.max(self.score_depth) {
            if let Some(last) = self.score
                && last - score >= SCORE_DROP
            {
                self.deteriorating = true;
            }
            self.unstable = changed;
            self.score = Some(score);
            self.score_depth = depth;
        }
    }

    /// Report ponder work separately from time spent on our own clock.
    pub fn report(&self) {
        if !self.was_ponder {
            return;
        }
        let end = Instant::now();
        let ponder_end = self.hit_time.unwrap_or(end);
        let outcome = if self.hit_time.is_some() {
            "hit"
        } else {
            "stopped"
        };
        let own_seconds = self.hit_time.map_or(0.0, |hit| (end - hit).as_secs_f64());
        let ponder_ms = (ponder_end - self.search_started).as_secs_f64() * 1000.0;
        let text = format!(
            "ponder outcome={outcome} ponder_ms={ponder_ms:.1} hit_depth={} hit_nodes={} final_depth={} final_nodes={} own_ms={:.1}",
            self.hit_depth,
            self.hit_nodes,
            self.completed_depth,
            self.nodes,
            own_seconds * 1000.0
        );
        let hit_cap_ms = if self.soft_limit.is_finite() {
            (2.0 * self.soft_limit).min(self.longest / 2.0) * 1000.0
        } else {
            0.0
        };
        let text = format!(
            "{text} hit_kind={} reinvest={} shallow_hit={} deteriorating={} unstable={} hit_cap_ms={hit_cap_ms:.1}",
            self.hit_kind,
            u8::from(self.source == "native" && self.reinvest()),
            u8::from(self.hit_depth < SHALLOW_DEPTH),
            u8::from(self.deteriorating),
            u8::from(self.unstable)
        );
        log(&text);
        let mut out = std::io::stdout();
        let _ = writeln!(out, "info string {text}");
        let _ = out.flush();
    }
}
