//! Talk to a chess GUI such as Lucas Chess through the Universal Chess Interface (UCI).
//! The GUI sends one command per line, and the engine answers each one. Besides what
//! Lucas Chess sends, "position fen", "go depth", "go perft", and "eval" serve the
//! workshop designer's check that this engine matches Remus4.

mod attacks;
mod board;
mod diagnostics;
mod evaluation;
mod exchange;
mod make_move;
mod move_generation;
mod opening_book;
mod search;
mod table_metrics;
mod transposition_table;

use std::io::{BufRead, Write};

use attacks::in_check;
use board::{Move, NO_MOVE, Position, move_text, new_move, parse_square};
use evaluation::evaluate;
use make_move::make_move;
use move_generation::{MoveList, generate_moves};
use opening_book::{BookLine, book_moves, load_book};
use search::Search;

const STACK_BYTES: usize = 64 * 1024 * 1024; // Room for the deepest search

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

fn run() {
    let book = load_book();
    let mut search = Search::new();
    let mut position = Position::start();
    let mut played: Vec<String> = Vec::new(); // The moves of a game from the start
    let mut use_book = true;
    let mut game_history: Vec<u64> = Vec::new(); // The keys of the positions before this one
    let mut out = std::io::stdout();

    for line in std::io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        diagnostics::log(&format!("< {line}"));
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        match words[0] {
            "uci" => {
                let _ = writeln!(out, "id name Remus4-Rust");
                let _ = writeln!(
                    out,
                    "option name SearchDiagnostics type check default false"
                );
                let _ = writeln!(out, "uciok");
            }
            "isready" => {
                let _ = writeln!(out, "readyok");
            }
            "setoption" => {
                if words.len() == 5
                    && words[1] == "name"
                    && words[2].eq_ignore_ascii_case("SearchDiagnostics")
                    && words[3] == "value"
                {
                    diagnostics::ENABLED.store(
                        words[4].eq_ignore_ascii_case("true"),
                        std::sync::atomic::Ordering::Relaxed,
                    );
                }
            }
            "position" => {
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
                search.diagnostics.begin();
                search.diagnostics.report(&format!(
                    "begin root_key={:016x} white={} halfmove={} history_plies={} plan_s={seconds:.3} hard_s={longest:.3} command={line}",
                    position.key, position.white, position.halfmove, game_history.len()
                ));
                let mut mv = NO_MOVE;
                let mut source = "book";
                if use_book && depth_limit.is_none() {
                    let texts: Vec<&str> = played.iter().map(String::as_str).collect();
                    mv = legal_book_move(&book, &position, &texts);
                }
                if mv == NO_MOVE {
                    source = "native";
                    mv = search.think(&position, &game_history, seconds, longest, depth_limit);
                }
                // With no legal move (checkmate or stalemate) UCI says "0000"
                let text = if mv == NO_MOVE {
                    "0000".to_string()
                } else {
                    move_text(mv)
                };
                let _ = writeln!(out, "bestmove {text}");
                let _ = out.flush();
                search
                    .diagnostics
                    .report(&format!("result source={source} bestmove {text}"));
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
