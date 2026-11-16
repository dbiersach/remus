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

/// Keep both diagnostic streams in one process session, independent of the GUI's folder.
fn log_path(kind: &str) -> std::io::Result<std::path::PathBuf> {
    static SESSION: OnceLock<u128> = OnceLock::new();
    let session = SESSION.get_or_init(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    });
    let folder = std::env::current_exe()?.with_file_name("logs");
    std::fs::create_dir_all(&folder)?;
    Ok(folder.join(format!("{kind}_{}_{session}.txt", std::process::id())))
}

/// The search thread alone writes logs; unavailable files never stop native play.
fn append(kind: &str, text: &str) {
    let started = *STARTED.get_or_init(Instant::now);
    let Ok(path) = log_path(kind) else {
        return;
    };
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
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
        append("search", text);
    }
}

pub fn ponder_log(text: &str) {
    if PONDER_ENABLED.load(Relaxed) {
        append("ponder", text);
    }
}

pub fn command(text: &str) {
    if SEARCH_ENABLED.load(Relaxed) || PONDER_ENABLED.load(Relaxed) {
        let line = format!("< {text}");
        search_log(&line);
        ponder_log(&line);
    }
}
