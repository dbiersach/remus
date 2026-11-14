//! Play a move. As in Remus4, make_move() copies the position, plays the move on the
//! copy, and returns the new position, so the search never takes a move back.

use crate::board::{EMPTY, KEYS, KING, Move, PAWN, Position, WHITE};
use crate::board::{move_promotion, move_start, move_target};

// The castling rights that a move from or to each square turns off for good
const fn build_rights() -> [u8; 64] {
    let mut table = [0; 64];
    table[60] = 3; // e1
    table[63] = 1; // h1
    table[56] = 2; // a1
    table[4] = 12; // e8
    table[7] = 4; // h8
    table[0] = 8; // a8
    table
}

static RIGHTS_OF_SQUARE: [u8; 64] = build_rights();

/// Return the position after the move, including the special moves.
#[inline]
pub fn make_move(position: &Position, mv: Move) -> Position {
    let mut next = *position;
    let start = move_start(mv);
    let target = move_target(mv);
    let promotion = move_promotion(mv);
    let mut piece = next.board[start];
    let kind = piece & 7;
    let is_pawn = kind == PAWN;
    let distance = start.abs_diff(target);

    // A capture or pawn move restarts the fifty-move count
    if is_pawn || next.board[target] != EMPTY {
        next.halfmove = 0;
    } else {
        next.halfmove += 1;
    }
    // En passant: the captured pawn stands beside the pawn that moves
    if is_pawn && target as i8 == next.en_passant {
        next.remove_piece(start / 8 * 8 + target % 8);
    }
    // Castling: the king moves two squares, and the rook jumps to the square it crossed
    if kind == KING && distance == 2 {
        let rook = if target > start { start + 3 } else { start - 4 };
        let rook_piece = next.board[rook];
        next.remove_piece(rook);
        next.put_piece((start + target) / 2, rook_piece);
    }
    // A two-square pawn move lets an enemy pawn capture it en passant on the next move
    if next.en_passant >= 0 {
        next.key ^= KEYS.en_passant[next.en_passant as usize];
    }
    next.en_passant = -1;
    if is_pawn && distance == 16 {
        let skipped = (start + target) / 2;
        next.en_passant = skipped as i8;
        next.key ^= KEYS.en_passant[skipped];
    }
    if promotion != 0 {
        piece = promotion + (piece & WHITE);
    }
    if next.board[target] != EMPTY {
        next.remove_piece(target);
    }
    next.remove_piece(start);
    next.put_piece(target, piece);

    let rights = next.castling & !(RIGHTS_OF_SQUARE[start] | RIGHTS_OF_SQUARE[target]);
    if rights != next.castling {
        next.key ^= KEYS.castling[next.castling as usize] ^ KEYS.castling[rights as usize];
        next.castling = rights;
    }
    next.white = !next.white;
    next.key ^= KEYS.black;
    next
}

/// Return the position with the other side to move and no en passant square.
#[inline]
pub fn make_null_move(position: &Position) -> Position {
    let mut next = *position;
    if next.en_passant >= 0 {
        next.key ^= KEYS.en_passant[next.en_passant as usize];
    }
    next.en_passant = -1;
    next.halfmove += 1;
    next.white = !next.white;
    next.key ^= KEYS.black;
    next
}
