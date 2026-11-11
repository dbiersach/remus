//! Score a position in centipawns for the side to move. Every table and term matches
//! evaluation.py in Remus5, and every division rounds down as Python's // does.

use crate::attacks::{KNIGHT_ATTACKS, bishop_attacks, rook_attacks};
use crate::board::{BISHOP, KING, KNIGHT, PAWN, Position, QUEEN, ROOK};

/// Return a 64-square table built from files a to d of every rank.
const fn mirrored_table(left_half: [i32; 32]) -> [i32; 64] {
    let columns = [0, 1, 2, 3, 3, 2, 1, 0];
    let mut table = [0; 64];
    let mut row = 0;
    while row < 8 {
        let mut col = 0;
        while col < 8 {
            table[row * 8 + col] = left_half[row * 4 + columns[col]];
            col += 1;
        }
        row += 1;
    }
    table
}

#[rustfmt::skip]
static PAWN_TABLE: [i32; 64] = mirrored_table([
      0,   0,   0,   0,
     50,  50,  50,  50,
     10,  10,  20,  30,
      5,   5,  10,  25,
      0,   0,   0,  20,
      5,  -5, -10,   0,
      5,  10,  10, -20,
      0,   0,   0,   0,
]);

#[rustfmt::skip]
static KNIGHT_TABLE: [i32; 64] = mirrored_table([
    -50, -40, -30, -30,
    -40, -20,   0,   0,
    -30,   0,  10,  15,
    -30,   5,  15,  20,
    -30,   0,  15,  20,
    -30,   5,  10,  15,
    -40, -20,   0,   5,
    -50, -40, -30, -30,
]);

#[rustfmt::skip]
static BISHOP_TABLE: [i32; 64] = mirrored_table([
    -20, -10, -10, -10,
    -10,   0,   0,   0,
    -10,   0,   5,  10,
    -10,   5,   5,  10,
    -10,   0,  10,  10,
    -10,  10,  10,  10,
    -10,   5,   0,   0,
    -20, -10, -10, -10,
]);

#[rustfmt::skip]
static KING_MIDDLE_TABLE: [i32; 64] = mirrored_table([
    -30, -40, -40, -50,
    -30, -40, -40, -50,
    -30, -40, -40, -50,
    -30, -40, -40, -50,
    -20, -30, -30, -40,
    -10, -20, -20, -20,
     20,  20,   0,   0,
     20,  30,  10,   0,
]);

#[rustfmt::skip]
static ROOK_TABLE: [i32; 64] = mirrored_table([
      0,   0,   0,   0,
      5,  10,  10,  10,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
      0,   0,   0,   5,
]);

#[rustfmt::skip]
static KING_END_TABLE: [i32; 64] = mirrored_table([
    -50, -40, -30, -20,
    -30, -20, -10,   0,
    -30, -10,  20,  30,
    -30, -10,  30,  40,
    -30, -10,  30,  40,
    -30, -10,  20,  30,
    -30, -30,   0,   0,
    -50, -30, -30, -30,
]);

// What each kind of piece is worth, from empty (0) to king (6)
pub const PIECE_VALUES: [i32; 7] = [0, 100, 300, 300, 500, 900, 0];

// The bonus of a passed pawn by the moves it needs to promote
const PASSED_BONUS: [i32; 7] = [0, 120, 80, 50, 30, 15, 10];
const MOBILITY_BONUS: i32 = 2; // Per square a knight, bishop, rook, or queen reaches
const BISHOP_PAIR: i32 = 30;
const ROOK_OPEN_FILE: i32 = 20; // A file with no pawns
const ROOK_HALF_OPEN_FILE: i32 = 10; // A file with only enemy pawns
const DOUBLED_PAWN: i32 = 15; // For each extra pawn on a file
const ISOLATED_PAWN: i32 = 15; // No friendly pawn on a file beside it
const UNSTOPPABLE_PAWN: i32 = 500; // A passed pawn the enemy king cannot catch
const SHIELD_PAWN: i32 = 10; // For each pawn in front of a king on its back rank
const OPEN_KING_FILE: i32 = 15; // For each file at the king with no friendly pawn

const fn file_mask(col: i32) -> u64 {
    if col < 0 || col > 7 {
        0
    } else {
        0x0101_0101_0101_0101_u64 << col
    }
}

/// For each side and square, the squares where an enemy pawn can stop a pawn on that
/// square: ahead of it, on its own file or a file beside it.
const fn build_passed_masks() -> [[u64; 64]; 2] {
    let mut table = [[0; 64]; 2];
    let mut square = 0;
    while square < 64 {
        let row = (square / 8) as i32;
        let col = (square % 8) as i32;
        let files = file_mask(col - 1) | file_mask(col) | file_mask(col + 1);
        // Row 0 is rank 8, so the rows ahead of a White pawn have smaller numbers
        let rows_above = if row == 0 {
            0
        } else {
            u64::MAX >> (64 - 8 * row)
        };
        let rows_below = if row == 7 {
            0
        } else {
            u64::MAX << (8 * (row + 1))
        };
        table[0][square] = files & rows_above;
        table[1][square] = files & rows_below;
        square += 1;
    }
    table
}

static PASSED_MASKS: [[u64; 64]; 2] = build_passed_masks();

const SIGNS: [i32; 2] = [1, -1];

/// Score the pawn shield of one king, larger with more pieces left.
#[inline]
fn king_safety(own_pawns: u64, counts: &[i32; 10], king: i32, side: usize, phase: i32) -> i32 {
    if king / 8 != (if side == 0 { 7 } else { 0 }) {
        return 0; // Only a king on its back rank has a shield
    }
    let col = king % 8;
    let forward = if side == 0 { -8 } else { 8 };
    let mut score = 0;
    for c in (col - 1).max(0)..(col + 2).min(8) {
        if own_pawns & (1_u64 << (king + forward + c - col)) != 0 {
            score += SHIELD_PAWN;
        }
        if counts[(c + 1) as usize] == 0 {
            score -= OPEN_KING_FILE;
        }
    }
    (score * phase).div_euclid(24)
}

/// Score the pawns, the rooks that use their files, and the pawn shield of each king,
/// from White's side.
#[inline]
fn pawn_terms(
    pawns: &[u64; 2],
    rooks: &[u64; 2],
    kings: &[i32; 2],
    phase: i32,
    white: bool,
) -> i32 {
    let mut score = 0;
    // Entry col + 1 counts file col, so the a and h files have an empty file on each side
    let mut counts = [[0_i32; 10]; 2];
    for side in 0..2 {
        let mut left = pawns[side];
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            counts[side][square % 8 + 1] += 1;
        }
    }
    for side in 0..2 {
        let sign = SIGNS[side];
        let enemy = 1 - side;
        for count in counts[side] {
            score -= sign * DOUBLED_PAWN * (count - 1).max(0);
        }
        let mut left = pawns[side];
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            let row = (square / 8) as i32;
            let col = square % 8;
            if counts[side][col] == 0 && counts[side][col + 2] == 0 {
                score -= sign * ISOLATED_PAWN;
            }
            if PASSED_MASKS[side][square] & pawns[enemy] != 0 {
                continue;
            }
            let steps = if side == 0 { row } else { 7 - row }; // Moves to promotion
            score += sign * PASSED_BONUS[steps as usize];
            // The pawn race: with no pieces left, a pawn whose promotion square the enemy
            // king cannot reach (the rule of the square) becomes a queen
            let king = kings[enemy];
            let mut king_steps = (king / 8 - 7 * side as i32)
                .abs()
                .max((king % 8 - col as i32).abs());
            if white != (side == 0) {
                king_steps -= 1; // The enemy king moves first
            }
            if phase == 0 && king_steps > steps.min(5) {
                score += sign * UNSTOPPABLE_PAWN;
            }
        }
        let mut left = rooks[side];
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            let entry = square % 8 + 1;
            if counts[side][entry] == 0 {
                let open_file = counts[enemy][entry] == 0;
                score += sign
                    * if open_file {
                        ROOK_OPEN_FILE
                    } else {
                        ROOK_HALF_OPEN_FILE
                    };
            }
        }
        score += sign * king_safety(pawns[side], &counts[side], kings[side], side, phase);
    }
    score
}

/// In an endgame where one side is ahead, reward pushing the losing king to the edge and
/// bringing the winning king close, from White's side.
#[inline]
fn mating_bonus(material: i32, kings: &[i32; 2], phase: i32) -> i32 {
    if phase > 8 || material.abs() < 400 {
        return 0;
    }
    let winner = if material > 0 { 0 } else { 1 };
    let loser_row = kings[1 - winner] / 8;
    let loser_col = kings[1 - winner] % 8;
    // 0 in the center, 6 in a corner
    let edge = (3 - loser_row).max(loser_row - 4) + (3 - loser_col).max(loser_col - 4);
    let distance = (kings[0] / 8 - kings[1] / 8)
        .abs()
        .max((kings[0] % 8 - kings[1] % 8).abs());
    let bonus = 10 * edge + 5 * (7 - distance);
    if winner == 0 { bonus } else { -bonus }
}

/// Return the score for the side to move: positive when it is ahead.
pub fn evaluate(position: &Position) -> i32 {
    let kinds = &position.by_kind;
    let occupied = position.occupied();
    let mut score = 0; // From White's side
    let mut material = 0;
    let mut phase = 0; // 24 with every piece on the board, 0 with none left
    let mut kings = [0_i32; 2];
    let mut pawns = [0_u64; 2];
    let mut rooks = [0_u64; 2];
    let mut bishop_count = [0_u32; 2];

    for side in 0..2 {
        let sign = SIGNS[side];
        let own = position.by_side[side];
        // Flip the row for Black: Black's a8 is White's a1
        let flip = if side == 0 { 0 } else { 56 };
        let mut side_score = 0;
        let mut reach_count = 0;

        pawns[side] = own & kinds[PAWN as usize];
        rooks[side] = own & kinds[ROOK as usize];
        kings[side] = (own & kinds[KING as usize]).trailing_zeros() as i32;

        let mut left = pawns[side];
        material += sign * 100 * left.count_ones() as i32;
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            side_score += PAWN_TABLE[square ^ flip];
        }
        let mut left = own & kinds[KNIGHT as usize];
        phase += left.count_ones() as i32;
        material += sign * 300 * left.count_ones() as i32;
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            side_score += KNIGHT_TABLE[square ^ flip];
            reach_count += (KNIGHT_ATTACKS[square] & !own).count_ones() as i32;
        }
        let mut left = own & kinds[BISHOP as usize];
        bishop_count[side] = left.count_ones();
        phase += left.count_ones() as i32;
        material += sign * 300 * left.count_ones() as i32;
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            side_score += BISHOP_TABLE[square ^ flip];
            reach_count += (bishop_attacks(square, occupied) & !own).count_ones() as i32;
        }
        let mut left = rooks[side];
        phase += 2 * left.count_ones() as i32;
        material += sign * 500 * left.count_ones() as i32;
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            side_score += ROOK_TABLE[square ^ flip];
            reach_count += (rook_attacks(square, occupied) & !own).count_ones() as i32;
        }
        let mut left = own & kinds[QUEEN as usize];
        phase += 4 * left.count_ones() as i32;
        material += sign * 900 * left.count_ones() as i32;
        while left != 0 {
            let square = left.trailing_zeros() as usize;
            left &= left - 1;
            let reach = rook_attacks(square, occupied) | bishop_attacks(square, occupied);
            reach_count += (reach & !own).count_ones() as i32;
        }
        score += sign * (side_score + MOBILITY_BONUS * reach_count);
    }

    // Insufficient material: a lone knight or bishop cannot mate
    if phase <= 1 && pawns[0] == 0 && pawns[1] == 0 {
        return 0;
    }
    let phase = phase.min(24);
    for side in 0..2 {
        let flip = if side == 0 { 0 } else { 56 };
        let view = kings[side] as usize ^ flip;
        let taper = KING_MIDDLE_TABLE[view] * phase + KING_END_TABLE[view] * (24 - phase);
        score += (SIGNS[side] * taper).div_euclid(24);
    }
    if bishop_count[0] >= 2 {
        score += BISHOP_PAIR;
    }
    if bishop_count[1] >= 2 {
        score -= BISHOP_PAIR;
    }
    score += material + pawn_terms(&pawns, &rooks, &kings, phase, position.white);
    score += mating_bonus(material, &kings, phase);
    if position.white { score } else { -score }
}
