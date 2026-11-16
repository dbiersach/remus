//! The position, the starting position, and the names of squares and moves.
//!
//! Square numbers match Remus4: row 0 is rank 8 and column 0 is file a, so a8 is 0,
//! h8 is 7, a1 is 56, and h1 is 63. A piece is its kind, from 1 for a pawn to 6 for a
//! king, plus 8 if it is White.
//!
//! The position also holds bitboards: one 64-bit number per kind of piece and per side,
//! where bit n is set when square n holds such a piece. Bitboards answer "where are the
//! rooks?" and "what does this rook reach?" in a few machine instructions.

pub const EMPTY: u8 = 0;
pub const PAWN: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISHOP: u8 = 3;
pub const ROOK: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;
pub const WHITE: u8 = 8; // Added to the kind of a White piece
pub const MATE_SCORE: i32 = 100_000; // Larger than any material count

/// A move packed as from * 512 + to * 8 + promotion. Comparing two packed moves gives
/// the same order as comparing the tuples (from, to, promotion) in Remus4. The number 0
/// means "no move": a8 to a8 is never played.
pub type Move = u16;

pub const NO_MOVE: Move = 0;

#[inline(always)]
pub const fn new_move(start: usize, target: usize, promotion: u8) -> Move {
    ((start << 9) | (target << 3) | promotion as usize) as Move
}

#[inline(always)]
pub const fn move_start(mv: Move) -> usize {
    (mv >> 9) as usize
}

#[inline(always)]
pub const fn move_target(mv: Move) -> usize {
    ((mv >> 3) & 63) as usize
}

#[inline(always)]
pub const fn move_promotion(mv: Move) -> u8 {
    (mv & 7) as u8
}

/// One step of the SplitMix64 random number generator, usable at compile time.
const fn next_random(state: u64) -> (u64, u64) {
    let state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (state, z ^ (z >> 31))
}

/// A random number for every piece on every square, for each en passant square, for
/// each set of castling rights, and for Black to move. The key of a position is the
/// exclusive-or of the numbers that describe it, which make_move() updates in a few
/// steps instead of reading all 64 squares.
pub struct KeyTable {
    pub piece: [[u64; 64]; 15],
    pub en_passant: [u64; 64],
    pub castling: [u64; 16],
    pub black: u64,
}

const fn build_keys() -> KeyTable {
    let mut table = KeyTable {
        piece: [[0; 64]; 15],
        en_passant: [0; 64],
        castling: [0; 16],
        black: 0,
    };
    let mut state = 2016_u64;
    let mut piece = 0;
    while piece < 15 {
        let mut square = 0;
        while square < 64 {
            let (next, value) = next_random(state);
            state = next;
            table.piece[piece][square] = value;
            square += 1;
        }
        piece += 1;
    }
    let mut square = 0;
    while square < 64 {
        let (next, value) = next_random(state);
        state = next;
        table.en_passant[square] = value;
        square += 1;
    }
    let mut rights = 0;
    while rights < 16 {
        let (next, value) = next_random(state);
        state = next;
        table.castling[rights] = value;
        rights += 1;
    }
    let (_, value) = next_random(state);
    table.black = value;
    table
}

pub static KEYS: KeyTable = build_keys();

/// The position. Copying this small struct is the copy of copy-make.
#[derive(Clone, Copy)]
pub struct Position {
    pub board: [u8; 64],
    pub by_kind: [u64; 7], // The squares of each kind of piece, of either side
    pub by_side: [u64; 2], // The squares of White's pieces, then Black's
    pub key: u64,          // The number that names the position
    pub white: bool,       // True when White is to move
    pub castling: u8,      // 1, 2 for White's rights, 4, 8 for Black's
    pub en_passant: i8,    // The square a pawn just skipped over, or -1
    pub halfmove: i32,     // The moves since the last capture or pawn move
}

impl Position {
    /// Build a position from its plain parts.
    pub fn new(
        board: [u8; 64],
        white: bool,
        castling: u8,
        en_passant: i8,
        halfmove: i32,
    ) -> Position {
        let mut position = Position {
            board: [EMPTY; 64],
            by_kind: [0; 7],
            by_side: [0; 2],
            key: KEYS.castling[castling as usize],
            white,
            castling,
            en_passant,
            halfmove,
        };
        for (square, &piece) in board.iter().enumerate() {
            if piece != EMPTY {
                position.put_piece(square, piece);
            }
        }
        if !white {
            position.key ^= KEYS.black;
        }
        if en_passant >= 0 {
            position.key ^= KEYS.en_passant[en_passant as usize];
        }
        position
    }

    /// The starting position.
    pub fn start() -> Position {
        let back_rank = [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK];
        let mut board = [EMPTY; 64];
        for col in 0..8 {
            board[col] = back_rank[col];
            board[8 + col] = PAWN;
            board[48 + col] = WHITE + PAWN;
            board[56 + col] = WHITE + back_rank[col];
        }
        Position::new(board, true, 15, -1, 0)
    }

    /// Build a position from FEN text, for the workshop designer's checks.
    pub fn from_fen(words: &[&str]) -> Position {
        let mut board = [EMPTY; 64];
        let mut square = 0;
        for letter in words[0].chars() {
            if let Some(gap) = letter.to_digit(10) {
                square += gap as usize;
            } else if letter != '/' {
                let kind = ".pnbrqk".find(letter.to_ascii_lowercase()).unwrap_or(0) as u8;
                board[square] = kind
                    + if letter.is_ascii_uppercase() {
                        WHITE
                    } else {
                        0
                    };
                square += 1;
            }
        }
        let mut castling = 0;
        for (letter, bit) in [('K', 1), ('Q', 2), ('k', 4), ('q', 8)] {
            if words[2].contains(letter) {
                castling += bit;
            }
        }
        let en_passant = if words[3] == "-" {
            -1
        } else {
            parse_square(words[3]) as i8
        };
        let halfmove = words.get(4).and_then(|text| text.parse().ok()).unwrap_or(0);
        Position::new(board, words[1] == "w", castling, en_passant, halfmove)
    }

    /// Place a piece on an empty square.
    #[inline(always)]
    pub fn put_piece(&mut self, square: usize, piece: u8) {
        let bit = 1_u64 << square;
        self.board[square] = piece;
        self.by_kind[(piece & 7) as usize] |= bit;
        self.by_side[usize::from(piece < WHITE)] |= bit;
        self.key ^= KEYS.piece[piece as usize][square];
    }

    /// Lift the piece off a square.
    #[inline(always)]
    pub fn remove_piece(&mut self, square: usize) {
        let piece = self.board[square];
        let bit = 1_u64 << square;
        self.board[square] = EMPTY;
        self.by_kind[(piece & 7) as usize] &= !bit;
        self.by_side[usize::from(piece < WHITE)] &= !bit;
        self.key ^= KEYS.piece[piece as usize][square];
    }

    /// The squares of every piece on the board.
    #[inline(always)]
    pub fn occupied(&self) -> u64 {
        self.by_side[0] | self.by_side[1]
    }
}

/// Return the square that text such as "e2" names.
pub fn parse_square(text: &str) -> usize {
    let bytes = text.as_bytes();
    let row = b'8'.wrapping_sub(bytes[1]) as usize;
    let col = bytes[0].wrapping_sub(b'a') as usize;
    (row * 8 + col) & 63
}

/// Add the name of a square, such as "e2", to the text.
fn push_square(text: &mut String, square: usize) {
    text.push((b'a' + (square % 8) as u8) as char);
    text.push((b'8' - (square / 8) as u8) as char);
}

/// Return the UCI text of a move, such as "e7e8q".
pub fn move_text(mv: Move) -> String {
    let mut text = String::with_capacity(5);
    push_square(&mut text, move_start(mv));
    push_square(&mut text, move_target(mv));
    let promotion = move_promotion(mv) as usize;
    if promotion != 0 {
        text.push(b".pnbrqk"[promotion] as char);
    }
    text
}
