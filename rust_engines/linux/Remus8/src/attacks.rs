//! Find out what each piece reaches and whether a square is attacked.
//!
//! The tables hold, for each square, a bitboard of what a knight, king, or pawn on that
//! square reaches, and the squares along each of the eight rays. The compiler fills them
//! in while it builds the program, so they cost nothing when the program runs.

use crate::board::{BISHOP, KING, KNIGHT, PAWN, Position, QUEEN, ROOK};

const KNIGHT_JUMPS: [(i32, i32); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

// The rays: the first four raise the square number and the last four lower it.
// Rooks use rays 0, 1, 4, and 5, and bishops use rays 2, 3, 6, and 7.
const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, -1),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (-1, 1),
];

const fn bit_if_on_board(row: i32, col: i32) -> u64 {
    if row >= 0 && row < 8 && col >= 0 && col < 8 {
        1_u64 << (row * 8 + col)
    } else {
        0
    }
}

const fn build_steps(steps: &[(i32, i32); 8]) -> [u64; 64] {
    let mut table = [0; 64];
    let mut square = 0;
    while square < 64 {
        let row = (square / 8) as i32;
        let col = (square % 8) as i32;
        let mut i = 0;
        while i < 8 {
            table[square] |= bit_if_on_board(row + steps[i].0, col + steps[i].1);
            i += 1;
        }
        square += 1;
    }
    table
}

const fn build_rays() -> [[u64; 64]; 8] {
    let mut table = [[0; 64]; 8];
    let mut direction = 0;
    while direction < 8 {
        let mut square = 0;
        while square < 64 {
            let mut row = (square / 8) as i32 + DIRECTIONS[direction].0;
            let mut col = (square % 8) as i32 + DIRECTIONS[direction].1;
            while bit_if_on_board(row, col) != 0 {
                table[direction][square] |= bit_if_on_board(row, col);
                row += DIRECTIONS[direction].0;
                col += DIRECTIONS[direction].1;
            }
            square += 1;
        }
        direction += 1;
    }
    table
}

const fn build_pawn_attacks() -> [[u64; 64]; 2] {
    let mut table = [[0; 64]; 2];
    let mut square = 0;
    while square < 64 {
        let row = (square / 8) as i32;
        let col = (square % 8) as i32;
        // Row 0 is rank 8, so a White pawn attacks toward row 0
        table[0][square] = bit_if_on_board(row - 1, col - 1) | bit_if_on_board(row - 1, col + 1);
        table[1][square] = bit_if_on_board(row + 1, col - 1) | bit_if_on_board(row + 1, col + 1);
        square += 1;
    }
    table
}

pub static KNIGHT_ATTACKS: [u64; 64] = build_steps(&KNIGHT_JUMPS);
pub static KING_ATTACKS: [u64; 64] = build_steps(&DIRECTIONS);
pub static PAWN_ATTACKS: [[u64; 64]; 2] = build_pawn_attacks();
static RAYS: [[u64; 64]; 8] = build_rays();

const fn build_queen_lines() -> [u64; 64] {
    let rays = build_rays();
    let mut table = [0; 64];
    let mut square = 0;
    while square < 64 {
        let mut direction = 0;
        while direction < 8 {
            table[square] |= rays[direction][square];
            direction += 1;
        }
        square += 1;
    }
    table
}

/// For each square, the squares on its rank, file, and two diagonals. A piece that
/// stands off the lines of its own king cannot uncover that king by moving.
pub static QUEEN_LINES: [u64; 64] = build_queen_lines();

/// The squares along a ray that raises the square number, up to the first piece.
#[inline(always)]
fn rising_ray(direction: usize, square: usize, occupied: u64) -> u64 {
    let ray = RAYS[direction][square];
    // The top bit stands in for a piece on h1, square 63. No rising ray goes past h1,
    // so its table entry is empty and an open ray keeps all of its squares. The
    // lookup then needs no test for "no piece on the ray", a test the processor
    // predicts badly.
    let first = ((ray & occupied) | (1 << 63)).trailing_zeros() as usize;
    ray ^ RAYS[direction][first]
}

/// The squares along a ray that lowers the square number, up to the first piece.
#[inline(always)]
fn falling_ray(direction: usize, square: usize, occupied: u64) -> u64 {
    let ray = RAYS[direction][square];
    // The low bit stands in for a piece on a8, square 0: no falling ray goes past a8
    let last = 63 - ((ray & occupied) | 1).leading_zeros() as usize;
    ray ^ RAYS[direction][last]
}

/// The squares a rook on the square reaches, including the first piece on each line.
#[inline(always)]
pub fn rook_attacks(square: usize, occupied: u64) -> u64 {
    rising_ray(0, square, occupied)
        | rising_ray(1, square, occupied)
        | falling_ray(4, square, occupied)
        | falling_ray(5, square, occupied)
}

/// The squares a bishop on the square reaches, including the first piece on each line.
#[inline(always)]
pub fn bishop_attacks(square: usize, occupied: u64) -> u64 {
    rising_ray(2, square, occupied)
        | rising_ray(3, square, occupied)
        | falling_ray(6, square, occupied)
        | falling_ray(7, square, occupied)
}

/// Return true if a piece of the given color attacks the square.
#[inline]
pub fn is_attacked(position: &Position, square: usize, by_white: bool) -> bool {
    let side = usize::from(!by_white);
    let attackers = position.by_side[side];
    let kinds = &position.by_kind;
    // Look out from the square: a pawn of the other color on the square would attack
    // exactly the squares where a pawn of this color attacks it
    if PAWN_ATTACKS[1 - side][square] & attackers & kinds[PAWN as usize] != 0 {
        return true;
    }
    if KNIGHT_ATTACKS[square] & attackers & kinds[KNIGHT as usize] != 0 {
        return true;
    }
    if KING_ATTACKS[square] & attackers & kinds[KING as usize] != 0 {
        return true;
    }
    let queens = kinds[QUEEN as usize];
    let occupied = position.occupied();
    let straight = attackers & (kinds[ROOK as usize] | queens);
    if straight != 0 && rook_attacks(square, occupied) & straight != 0 {
        return true;
    }
    let diagonal = attackers & (kinds[BISHOP as usize] | queens);
    diagonal != 0 && bishop_attacks(square, occupied) & diagonal != 0
}

/// Return true if the king of the given color is attacked.
#[inline]
pub fn in_check(position: &Position, white: bool) -> bool {
    let kings = position.by_side[usize::from(!white)] & position.by_kind[KING as usize];
    is_attacked(position, kings.trailing_zeros() as usize, !white)
}
