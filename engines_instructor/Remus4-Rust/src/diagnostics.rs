//! Save search measurements at command and iteration boundaries, outside the node loop.

use std::io::Write;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub static ENABLED: AtomicBool = AtomicBool::new(false);
static STARTED: OnceLock<Instant> = OnceLock::new();

/// Append beside the executable. Missing permissions or storage never stop play.
pub fn log(text: &str) {
    if !ENABLED.load(Relaxed) {
        return;
    }
    let started = *STARTED.get_or_init(Instant::now);
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(exe.with_file_name("search_log.txt"))
    {
        let unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let _ = writeln!(
            file,
            "{} {:>11.1} unix_ms={unix_ms} {text}",
            std::process::id(),
            started.elapsed().as_secs_f64() * 1000.0
        );
    }
}

pub struct Diagnostics {
    search_id: u64,
    started: Instant,
    pub completed_depth: i32,
}

impl Diagnostics {
    pub fn new() -> Self {
        STARTED.get_or_init(Instant::now);
        Self {
            search_id: 0,
            started: Instant::now(),
            completed_depth: 0,
        }
    }

    pub fn begin(&mut self) {
        self.search_id += 1;
        self.started = Instant::now();
        self.completed_depth = 0;
    }

    pub fn report(&self, text: &str) {
        if ENABLED.load(Relaxed) {
            let ms = self.started.elapsed().as_secs_f64() * 1000.0;
            log(&format!(
                "search={} wall_ms={ms:.1} own_ms={ms:.1} completed_depth={} {text}",
                self.search_id, self.completed_depth
            ));
        }
    }
}
