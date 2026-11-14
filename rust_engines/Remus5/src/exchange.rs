//! Static exchange evaluation (SEE): play out every capture on one square, cheapest
//! attacker first, and return what the first capture wins or loses.

use crate::attacks::{KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS, bishop_attacks, rook_attacks};
use crate::board::{BISHOP, EMPTY, KING, KNIGHT, Move, PAWN, QUEEN, ROOK, WHITE};
use crate::board::{move_start, move_target};
use crate::evaluation::PIECE_VALUES;

/// The board during the exchange, with only what the exchange needs.
struct ExchangeBoard {
    board: [u8; 64],
    by_kind: [u64; 7],
    by_side: [u64; 2],
}

impl ExchangeBoard {
    /// Lift the piece off the square.
    #[inline(always)]
    fn remove(&mut self, square: usize) {
        let piece = self.board[square];
        let bit = 1_u64 << square;
        self.board[square] = EMPTY;
        self.by_kind[(piece & 7) as usize] &= !bit;
        self.by_side[usize::from(piece < WHITE)] &= !bit;
    }

    /// Move the piece on start to the target square, capturing what stands there.
    #[inline(always)]
    fn capture(&mut self, start: usize, target: usize) {
        let piece = self.board[start];
        let bit = 1_u64 << target;
        self.remove(target);
        self.remove(start);
        self.board[target] = piece;
        self.by_kind[(piece & 7) as usize] |= bit;
        self.by_side[usize::from(piece < WHITE)] |= bit;
    }

    /// Return the square of the cheapest piece that attacks the square, or None. A
    /// knight and a bishop are worth the same, and the lower square number goes first.
    #[inline]
    fn cheapest_attacker(&self, square: usize, by_white: bool) -> Option<usize> {
        let side = usize::from(!by_white);
        let own = self.by_side[side];
        let kinds = &self.by_kind;
        let occupied = self.by_side[0] | self.by_side[1];
        let found = PAWN_ATTACKS[1 - side][square] & own & kinds[PAWN as usize];
        if found != 0 {
            return Some(found.trailing_zeros() as usize);
        }
        let bishop_lines = bishop_attacks(square, occupied);
        let found = own
            & ((KNIGHT_ATTACKS[square] & kinds[KNIGHT as usize])
                | (bishop_lines & kinds[BISHOP as usize]));
        if found != 0 {
            return Some(found.trailing_zeros() as usize);
        }
        let rook_lines = rook_attacks(square, occupied);
        let found = own & rook_lines & kinds[ROOK as usize];
        if found != 0 {
            return Some(found.trailing_zeros() as usize);
        }
        let found = own & (rook_lines | bishop_lines) & kinds[QUEEN as usize];
        if found != 0 {
            return Some(found.trailing_zeros() as usize);
        }
        let found = own & KING_ATTACKS[square] & kinds[KING as usize];
        if found != 0 {
            return Some(found.trailing_zeros() as usize);
        }
        None
    }
}

const KING_VALUE: i32 = 2000; // The king captures last: losing it ends the game

/// Return the material the capture wins when both sides keep recapturing.
pub fn exchange_score(board: &[u8; 64], by_kind: &[u64; 7], by_side: &[u64; 2], mv: Move) -> i32 {
    let start = move_start(mv);
    let target = move_target(mv);
    let mut exchange = ExchangeBoard {
        board: *board,
        by_kind: *by_kind,
        by_side: *by_side,
    };
    // gains[i] is what the side making capture i has won after it
    let mut gains = [0_i32; 40];
    let mut count = 1;
    gains[0] = PIECE_VALUES[(exchange.board[target] & 7) as usize];
    let mut white = exchange.board[start] > WHITE;
    exchange.capture(start, target);
    let mut attacker = exchange.cheapest_attacker(target, !white);
    while let Some(from) = attacker {
        // The king may not capture onto a defended square
        if exchange.board[target] & 7 == KING {
            if count == 1 {
                return -KING_VALUE;
            }
            count -= 1;
            break;
        }
        gains[count] = PIECE_VALUES[(exchange.board[target] & 7) as usize] - gains[count - 1];
        count += 1;
        exchange.capture(from, target);
        white = !white;
        attacker = exchange.cheapest_attacker(target, !white);
    }
    // Work back from the last capture: each side may stop capturing
    for i in (1..count).rev() {
        gains[i - 1] = -(-gains[i - 1]).max(gains[i]);
    }
    gains[0]
}
