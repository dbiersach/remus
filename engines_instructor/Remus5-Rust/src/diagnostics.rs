//! Append search and ponder measurements beside the executable, outside the node loop.

use std::io::Write;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub static SEARCH_ENABLED: AtomicBool = AtomicBool::new(false);
pub static PONDER_ENABLED: AtomicBool = AtomicBool::new(false);
static STARTED: OnceLock<Instant> = OnceLock::new();

pub fn initialize() {
    STARTED.get_or_init(Instant::now);
}

/// The search thread alone writes logs; unavailable files never stop native play.
fn append(filename: &str, text: &str) {
    let started = *STARTED.get_or_init(Instant::now);
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(exe.with_file_name(filename))
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

pub fn search_log(text: &str) {
    if SEARCH_ENABLED.load(Relaxed) {
        append("search_log.txt", text);
    }
}

pub fn ponder_log(text: &str) {
    if PONDER_ENABLED.load(Relaxed) {
        append("ponder_log.txt", text);
    }
}

pub fn command(text: &str) {
    if SEARCH_ENABLED.load(Relaxed) || PONDER_ENABLED.load(Relaxed) {
        let line = format!("< {text}");
        search_log(&line);
        ponder_log(&line);
    }
}
