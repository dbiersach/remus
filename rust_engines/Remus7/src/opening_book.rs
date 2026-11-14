//! Play a stored opening line while the game follows one. The compiler copies
//! opening_book.txt into the program, so the book cannot go missing.

use std::time::{SystemTime, UNIX_EPOCH};

const BOOK_TEXT: &str = include_str!("../opening_book.txt");

/// One line of the book: the side it defends ("" for both) and its moves.
pub struct BookLine {
    side: &'static str,
    moves: Vec<&'static str>,
}

/// Read the book.
pub fn load_book() -> Vec<BookLine> {
    let mut book = Vec::new();
    for text in BOOK_TEXT.lines() {
        let mut side = "";
        let mut line = text;
        if let Some(rest) = text.strip_prefix("White:") {
            side = "White";
            line = rest;
        } else if let Some(rest) = text.strip_prefix("Black:") {
            side = "Black";
            line = rest;
        }
        if !line.trim().is_empty() && !line.starts_with('#') {
            book.push(BookLine {
                side,
                moves: line.split_whitespace().collect(),
            });
        }
    }
    book
}

/// Return a number that differs from one call to the next.
fn clock_seed() -> u64 {
    let since = SystemTime::now().duration_since(UNIX_EPOCH);
    since.map_or(2016, |time| time.as_nanos() as u64) | 1
}

/// Return the book's next moves after the moves played, in random order.
pub fn book_moves(book: &[BookLine], played: &[&str], white: bool) -> Vec<&'static str> {
    let mut moves = Vec::new();
    for line in book {
        let defends = line.side.is_empty() || (line.side == "White") == white;
        if defends && line.moves.len() > played.len() && line.moves[..played.len()] == *played {
            moves.push(line.moves[played.len()]);
        }
    }
    // Shuffle with a small xorshift generator seeded from the clock
    let mut state = clock_seed();
    for i in (1..moves.len()).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        moves.swap(i, (state % (i as u64 + 1)) as usize);
    }
    moves
}
