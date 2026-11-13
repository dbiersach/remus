//! Talk to a chess GUI such as Lucas Chess through the Universal Chess Interface (UCI).
//! The GUI sends one command per line, and the engine answers each one. Besides what
//! Lucas Chess sends, "position fen", "go depth", "go perft", and "eval" serve the
//! workshop designer's check that this engine matches Remus5.

mod attacks;
mod board;
mod evaluation;
mod exchange;
mod make_move;
mod move_generation;
mod opening_book;
mod parallel_search;
mod search;
mod search_control;
mod search_metrics;
mod table_metrics;
mod tablebase;
mod transposition_table;

use std::io::Write;
use std::time::Instant;

use attacks::in_check;
use board::{Move, NO_MOVE, Position, move_text, new_move, parse_square};
use evaluation::evaluate;
use make_move::make_move;
use move_generation::{MoveList, generate_moves};
use opening_book::{BookLine, book_moves, load_book};
use search::{DEFAULT_THREADS, MAX_THREADS, Search};
use search_control::{LOG_ENABLED, Mode, PonderCredit, SEARCH_LOG_ENABLED, log};

const STACK_BYTES: usize = 64 * 1024 * 1024; // Room for the deepest search
const HIT_GAP: f64 = 2.0; // Seconds from a stopped ponder search to a go that counts as a hit

/// The position of a ponder search that ended with stop, what that search earned
/// (its duration, depth, score, and signs of doubt), and when it ended.
struct Pondered {
    position_words: Vec<String>,
    credit: PonderCredit,
    ended: Instant,
}

/// Return the number after a word of "go", such as wtime, or 0.
fn read_number(words: &[&str], name: &str) -> i64 {
    for i in 0..words.len().saturating_sub(1) {
        if words[i] == name {
            return words[i + 1].parse().unwrap_or(0);
        }
    }
    0
}

/// Return the move that UCI text such as "e7e8q" names.
fn parse_move(text: &str) -> Move {
    let promotion = text
        .chars()
        .nth(4)
        .and_then(|letter| ".pnbrqk".find(letter))
        .unwrap_or(0) as u8;
    new_move(
        parse_square(&text[0..2]),
        parse_square(&text[2..4]),
        promotion,
    )
}

/// Return a legal move from the opening book, or NO_MOVE.
fn legal_book_move(book: &[BookLine], position: &Position, played: &[&str]) -> Move {
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for text in book_moves(book, played, position.white) {
        for &mut mv in list.as_mut_slice() {
            if move_text(mv) == text && !in_check(&make_move(position, mv), position.white) {
                return mv;
            }
        }
    }
    NO_MOVE
}

/// Count the leaf positions of the legal move tree.
fn perft(position: &Position, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    let mut count = 0;
    for &mut mv in list.as_mut_slice() {
        let child = make_move(position, mv);
        if !in_check(&child, position.white) {
            count += perft(&child, depth - 1);
        }
    }
    count
}

/// Return the preferred legal move, or a legal fallback after an immediate stop.
fn legal_choice(position: &Position, preferred: Move) -> Move {
    let mut first = NO_MOVE;
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for &mut mv in list.as_mut_slice() {
        if !in_check(&make_move(position, mv), position.white) {
            if mv == preferred {
                return mv;
            }
            if first == NO_MOVE {
                first = mv;
            }
        }
    }
    first
}

fn run() {
    let book = load_book();
    let mut tablebase = tablebase::Tablebase::new(tablebase::ENDPOINT);
    let mut use_tablebase = true;
    let mut use_ponder = false;
    let mut ponder_diagnostics = false;
    let mut search = Search::new();
    let mut position = Position::start();
    let mut played: Vec<String> = Vec::new(); // The moves of a game from the start
    let mut use_book = true;
    let mut game_history: Vec<u64> = Vec::new(); // The keys of the positions before this one
    let mut out = std::io::stdout();
    let mut pondered: Option<Pondered> = None;
    let mut position_words = vec!["position".to_string(), "startpos".to_string()];

    loop {
        let line = search.control().next_command();
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        match words[0] {
            "uci" => {
                let _ = writeln!(out, "id name Remus7-Rust");
                let _ = writeln!(
                    out,
                    "option name Hash type combo default 512 var 512 var 1024 var 2048"
                );
                let _ = writeln!(out, "option name LichessTablebase type check default true");
                let _ = writeln!(out, "option name Ponder type check default false");
                let _ = writeln!(
                    out,
                    "option name PonderHitMinPercent type spin default 25 min 0 max 100"
                );
                let _ = writeln!(
                    out,
                    "option name SearchDiagnostics type check default false"
                );
                let _ = writeln!(
                    out,
                    "option name Threads type spin default {DEFAULT_THREADS} min 1 max {MAX_THREADS}"
                );
                let _ = writeln!(
                    out,
                    "option name PonderDiagnostics type check default false"
                );
                let _ = writeln!(out, "uciok");
            }
            "setoption" => {
                pondered = None;
                if words.len() == 5 && words[1] == "name" && words[3] == "value" {
                    let enabled = words[4].eq_ignore_ascii_case("true");
                    match words[2].to_ascii_lowercase().as_str() {
                        "lichesstablebase" => use_tablebase = enabled,
                        "ponder" => use_ponder = enabled,
                        "hash" => {
                            if let Ok(mib @ (512 | 1024 | 2048)) = words[4].parse::<usize>() {
                                search.set_hash(mib);
                            }
                        }
                        "ponderhitminpercent" => {
                            search.control().hit_min_percent =
                                words[4].parse::<u32>().unwrap_or(25).min(100);
                        }
                        "searchdiagnostics" => {
                            search.diagnostics = enabled;
                            SEARCH_LOG_ENABLED.store(enabled, std::sync::atomic::Ordering::Relaxed);
                        }
                        "ponderdiagnostics" => {
                            ponder_diagnostics = enabled;
                            LOG_ENABLED.store(enabled, std::sync::atomic::Ordering::Relaxed);
                            log("PonderDiagnostics on");
                        }
                        "threads" => {
                            search.set_threads(words[4].parse().unwrap_or(DEFAULT_THREADS))
                        }
                        _ => {}
                    }
                }
            }
            "ucinewgame" => {
                pondered = None;
                position_words = vec!["position".to_string(), "startpos".to_string()];
                position = Position::start();
                played.clear();
                game_history.clear();
            }
            "isready" => {
                let _ = writeln!(out, "readyok");
            }
            "position" => {
                position_words = words.iter().map(|word| (*word).to_string()).collect();
                // "position startpos moves e2e4 e7e5" or "position fen ... moves e2e4"
                let moves_at = words.iter().position(|word| *word == "moves");
                use_book = words.get(1) == Some(&"startpos");
                position = if use_book {
                    Position::start()
                } else {
                    Position::from_fen(&words[2..moves_at.unwrap_or(words.len())])
                };
                played.clear();
                game_history.clear();
                if let Some(at) = moves_at {
                    for text in &words[at + 1..] {
                        game_history.push(position.key);
                        played.push((*text).to_string());
                        position = make_move(&position, parse_move(text));
                    }
                }
            }
            "go" => {
                let previous_ponder = pondered.take();
                if words.contains(&"perft") {
                    let count = perft(&position, read_number(&words, "perft") as u32);
                    let _ = writeln!(out, "perft {count}");
                    continue;
                }
                let depth_limit = if words.contains(&"depth") {
                    Some(read_number(&words, "depth") as i32)
                } else {
                    None
                };
                let white = position.white;
                let clock_left = read_number(&words, if white { "wtime" } else { "btime" }) as f64;
                let increment = read_number(&words, if white { "winc" } else { "binc" }) as f64;
                // Plan a thirtieth of the clock plus half the increment, and never more
                // than three times that or a fifth of the clock
                let mut seconds = 2.0;
                let mut longest = 2.0;
                if clock_left > 0.0 {
                    seconds = (clock_left / 30.0 + increment / 2.0) / 1000.0;
                    longest = (3.0 * seconds).min(clock_left / 5000.0);
                }
                if words.contains(&"movetime") {
                    seconds = read_number(&words, "movetime") as f64 / 1000.0;
                    longest = seconds;
                }
                let mode = if words.contains(&"ponder") {
                    Mode::Ponder
                } else if words.contains(&"infinite") {
                    Mode::Infinite
                } else {
                    Mode::Normal
                };
                // A go on the position just pondered, right after its stop, is a hit
                let mut credit = None;
                if let Some(last) = previous_ponder
                    && mode == Mode::Normal
                    && depth_limit.is_none()
                    && last.position_words == position_words
                    && last.ended.elapsed().as_secs_f64() < HIT_GAP
                {
                    credit = Some(last.credit);
                }
                search.control().begin(
                    mode,
                    if depth_limit.is_some() {
                        f64::INFINITY
                    } else {
                        longest
                    },
                );
                // A hit may spend extra time only from a clock with a minute in hand
                search.control().can_reinvest =
                    clock_left >= 60_000.0 && !words.contains(&"movetime");
                if let Some(credit) = credit {
                    search.control().credit_ponder(credit);
                    log(&format!(
                        "stop and go on the pondered position: hit after {:.1} s of pondering to depth {}",
                        credit.seconds.as_secs_f64(),
                        credit.depth
                    ));
                }
                let started = Instant::now();
                search.control().log_search(&format!(
                    "begin root_key={:016x} white={} halfmove={} history_plies={} plan_s={seconds:.3} hard_s={longest:.3} command={line}",
                    position.key, position.white, position.halfmove, game_history.len()
                ));
                log(&format!(
                    "plan seconds={seconds:.2} longest={longest:.2} ponder={}",
                    mode == Mode::Ponder
                ));
                let mut mv = NO_MOVE;
                let mut source = "native";
                if use_tablebase
                    && depth_limit.is_none()
                    && mode == Mode::Normal
                    && !search.control().was_ponder
                {
                    mv = tablebase.lookup(&position, &game_history, longest);
                }
                if mv != NO_MOVE {
                    source = "tablebase";
                    let _ = writeln!(out, "info string Lichess tablebase");
                }
                // A book move also answers a ponder request and a hit: the engine
                // then waits for the hit or stop without searching
                if mv == NO_MOVE && use_book && depth_limit.is_none() && mode != Mode::Infinite {
                    let texts: Vec<&str> = played.iter().map(String::as_str).collect();
                    mv = legal_book_move(&book, &position, &texts);
                    if mv != NO_MOVE {
                        source = "book";
                    }
                }
                search.control().source = source;
                if mv == NO_MOVE {
                    let elapsed = started.elapsed().as_secs_f64();
                    mv = search.think(
                        &position,
                        &game_history,
                        (seconds - elapsed).max(0.0),
                        (longest - elapsed).max(0.0),
                        depth_limit,
                    );
                }
                mv = legal_choice(&position, mv);
                search.control().wait();
                if let Some(credit) = search.control().ponder_credit() {
                    pondered = Some(Pondered {
                        position_words: position_words.clone(),
                        credit,
                        ended: Instant::now(),
                    });
                }
                if ponder_diagnostics {
                    search.control().report();
                }
                // With no legal move (checkmate or stalemate) UCI says "0000"
                let mut text = if mv == NO_MOVE {
                    "0000".to_string()
                } else {
                    move_text(mv)
                };
                if use_ponder && mv != NO_MOVE {
                    let mut reply = search.ponder_reply(&position, mv);
                    let child = make_move(&position, mv);
                    if reply == NO_MOVE && use_book {
                        let mut continuation = played.clone();
                        continuation.push(move_text(mv));
                        let texts: Vec<&str> = continuation.iter().map(String::as_str).collect();
                        reply = legal_book_move(&book, &child, &texts);
                    }
                    reply = legal_choice(&child, reply);
                    if reply != NO_MOVE {
                        text.push_str(&format!(" ponder {}", move_text(reply)));
                    }
                }
                let _ = writeln!(out, "bestmove {text}");
                let _ = out.flush();
                log(&format!("> bestmove {text}"));
                search
                    .control()
                    .log_search(&format!("result source={source} bestmove {text}"));
            }
            "eval" => {
                let _ = writeln!(out, "eval {}", evaluate(&position));
            }
            "quit" => break,
            _ => {}
        }
        let _ = out.flush();
    }
}

fn main() {
    // The search calls itself once per ply, so it runs on a thread with a large stack
    let worker = std::thread::Builder::new()
        .stack_size(STACK_BYTES)
        .spawn(run);
    if let Ok(handle) = worker {
        let _ = handle.join();
    }
}
