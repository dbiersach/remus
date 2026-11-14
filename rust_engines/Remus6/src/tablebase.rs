//! Query Lichess only at the root, keeping FEN, HTTPS, and JSON at the boundary.

use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant, SystemTime};

use serde_json::Value;

use crate::attacks::in_check;
use crate::board::{Move, NO_MOVE, Position, WHITE, move_text};
use crate::make_move::make_move;
use crate::move_generation::{MoveList, generate_moves};

pub const ENDPOINT: &str = "https://tablebase.lichess.org/standard";
const MAX_BYTES: u64 = 65_536; // Bound the JSON response held in memory
const CACHE_SIZE: usize = 256; // Positions retained in this engine process
const MAX_WAIT: f64 = 1.0; // At most one second and one quarter of the move budget
const MIN_WAIT: f64 = 0.02; // Very short moves use native search immediately

type Response = (Option<Value>, Instant);

/// Serialize the native board and actual halfmove counter for one API request.
pub fn position_fen(position: &Position) -> String {
    let mut text = String::new();
    // Row 0 is rank 8 and column 0 is file a, exactly the order FEN uses.
    for row in 0..8 {
        if row > 0 {
            text.push('/');
        }
        let mut gap = 0;
        for col in 0..8 {
            let piece = position.board[row * 8 + col];
            if piece == 0 {
                gap += 1;
                continue;
            }
            if gap > 0 {
                text.push_str(&gap.to_string());
                gap = 0;
            }
            let letter = b".pnbrqk"[(piece & 7) as usize] as char;
            text.push(if piece & WHITE != 0 {
                letter.to_ascii_uppercase()
            } else {
                letter
            });
        }
        if gap > 0 {
            text.push_str(&gap.to_string());
        }
    }
    let mut rights = String::new();
    for (letter, bit) in [('K', 1), ('Q', 2), ('k', 4), ('q', 8)] {
        if position.castling & bit != 0 {
            rights.push(letter);
        }
    }
    if rights.is_empty() {
        rights.push('-');
    }
    let target = if position.en_passant < 0 {
        "-".to_string()
    } else {
        let square = position.en_passant as u8;
        format!(
            "{}{}",
            (b'a' + square % 8) as char,
            (b'8' - square / 8) as char
        )
    };
    // Fullmove number does not affect a probe. The engine does not store it.
    format!(
        "{text} {} {rights} {target} {} 1",
        if position.white { "w" } else { "b" },
        position.halfmove
    )
}

/// Probe only small positions without castling rights or repeated history.
pub fn eligible(position: &Position, history: &[u64]) -> bool {
    if position.castling != 0 || position.halfmove >= 100 || position.occupied().count_ones() > 7 {
        return false;
    }
    let mut seen = HashSet::from([position.key]);
    let start = history
        .len()
        .saturating_sub(position.halfmove.max(0) as usize);
    for key in &history[start..] {
        if !seen.insert(*key) {
            return false;
        }
    }
    true
}

/// Map only definite API categories to the outcome under the fifty-move rule.
fn outcome(category: &Value) -> Option<i32> {
    match category.as_str()? {
        "win" => Some(1),
        "loss" => Some(-1),
        "draw" | "cursed-win" | "blessed-loss" => Some(0),
        _ => None,
    }
}

/// Accept the first ranked move only when its result and native legality agree.
pub fn response_move(position: &Position, history: &[u64], data: &Value) -> Move {
    let Some(root_outcome) = outcome(&data["category"]) else {
        return NO_MOVE;
    };
    let Some(first) = data["moves"].as_array().and_then(|moves| moves.first()) else {
        return NO_MOVE;
    };
    if outcome(&first["category"]) != Some(-root_outcome) {
        return NO_MOVE;
    }
    let Some(text) = first["uci"].as_str() else {
        return NO_MOVE;
    };
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for &mut mv in list.as_mut_slice() {
        if move_text(mv) != text {
            continue;
        }
        let child = make_move(position, mv);
        if in_check(&child, position.white) {
            return NO_MOVE;
        }
        let start = history
            .len()
            .saturating_sub(position.halfmove.max(0) as usize);
        if child.halfmove > 0 && history[start..].contains(&child.key) {
            return NO_MOVE;
        }
        return mv;
    }
    NO_MOVE
}

/// Respect Retry-After seconds or an HTTP date, with at least a minute off.
pub fn retry_delay(value: Option<&str>) -> Duration {
    let delay = value
        .and_then(|text| {
            text.parse::<u64>()
                .ok()
                .map(Duration::from_secs)
                .or_else(|| {
                    httpdate::parse_http_date(text)
                        .ok()?
                        .duration_since(SystemTime::now())
                        .ok()
                })
        })
        .unwrap_or(Duration::from_secs(60));
    delay.max(Duration::from_secs(60))
}

/// Fetch one bounded response. TLS certificate validation stays enabled.
fn fetch(fen: &str, seconds: f64, endpoint: &str) -> Response {
    let mut data = None;
    let mut delay = Duration::from_secs(5);
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs_f64(seconds))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent("Remus6/1.0")
            .build()?;
        let response = client
            .get(endpoint)
            .query(&[("fen", fen)])
            .header("Accept", "application/json")
            .send()?;
        if response.status().as_u16() == 429 {
            delay = retry_delay(
                response
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok()),
            );
            return Ok(());
        }
        let mut body = Vec::new();
        response
            .error_for_status()?
            .take(MAX_BYTES + 1)
            .read_to_end(&mut body)?;
        if body.len() as u64 <= MAX_BYTES {
            let parsed: Value = serde_json::from_slice(&body)?;
            if parsed.is_object() {
                data = Some(parsed);
                delay = Duration::from_secs(1);
            }
        }
        Ok(())
    })();
    let _ = result; // A transport or JSON error selects native search.
    let now = Instant::now();
    (
        data,
        now.checked_add(delay)
            .unwrap_or_else(|| now + Duration::from_secs(31_536_000)),
    )
}

/// One request at a time, a small cache, and a process-wide request cooldown.
pub struct Tablebase {
    endpoint: String,
    cache: HashMap<String, Value>,
    pending: Option<(String, Receiver<Response>)>,
    next_request: Instant,
}

impl Tablebase {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            cache: HashMap::new(),
            pending: None,
            next_request: Instant::now(),
        }
    }

    /// Collect a late result by its own FEN, never by the current root position.
    fn collect(&mut self, seconds: f64) {
        let Some((fen, receiver)) = &self.pending else {
            return;
        };
        match receiver.recv_timeout(Duration::from_secs_f64(seconds.max(0.0))) {
            Ok((data, next_request)) => {
                self.next_request = next_request;
                if let Some(data) = data {
                    if self.cache.len() >= CACHE_SIZE {
                        self.cache.clear();
                    }
                    self.cache.insert(fen.clone(), data);
                }
                self.pending = None;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                self.pending = None;
                self.next_request = Instant::now() + Duration::from_secs(5);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }

    /// Wait at most a quarter of the move budget, then let native search continue.
    pub fn lookup(&mut self, position: &Position, history: &[u64], longest: f64) -> Move {
        self.collect(0.0);
        if !eligible(position, history) {
            return NO_MOVE;
        }
        let fen = position_fen(position);
        if let Some(data) = self.cache.get(&fen) {
            return response_move(position, history, data);
        }
        let seconds = MAX_WAIT.min(longest.max(0.0) / 4.0);
        if self.pending.is_some() || Instant::now() < self.next_request || seconds < MIN_WAIT {
            return NO_MOVE;
        }
        let (sender, receiver) = mpsc::channel();
        let request_fen = fen.clone();
        let endpoint = self.endpoint.clone();
        if std::thread::Builder::new()
            .spawn(move || {
                let _ = sender.send(fetch(&request_fen, seconds, &endpoint));
            })
            .is_err()
        {
            return NO_MOVE;
        }
        self.pending = Some((fen.clone(), receiver));
        self.collect(seconds);
        self.cache
            .get(&fen)
            .map_or(NO_MOVE, |data| response_move(position, history, data))
    }
}
