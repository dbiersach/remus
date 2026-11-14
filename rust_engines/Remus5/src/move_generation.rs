//! List the moves of the side to move. As in Remus4 the list holds pseudo-legal moves:
//! some may leave the mover's own king in check, and the search skips those after it
//! makes them. The search sorts each list, so the order of generation does not matter.

use std::mem::MaybeUninit;

use crate::attacks::{
    KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS, bishop_attacks, is_attacked, rook_attacks,
};
use crate::board::{BISHOP, EMPTY, KING, KNIGHT, Move, PAWN, Position, QUEEN, ROOK, new_move};

/// A fixed-size list on the stack whose unused slots stay unwritten. The search makes
/// two of these at every position, and most positions use only a few slots, so filling
/// them with zeros first would cost more than the moves themselves.
pub struct StackList<T: Copy, const SIZE: usize> {
    items: [MaybeUninit<T>; SIZE],
    count: usize,
}

impl<T: Copy, const SIZE: usize> StackList<T, SIZE> {
    #[inline(always)]
    pub fn new() -> Self {
        StackList {
            items: [MaybeUninit::uninit(); SIZE],
            count: 0,
        }
    }

    #[inline(always)]
    pub fn add(&mut self, item: T) {
        self.items[self.count] = MaybeUninit::new(item);
        self.count += 1;
    }

    /// The items added so far, ready to read or sort.
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // Slots 0 up to count hold items that add() wrote
        unsafe { std::slice::from_raw_parts_mut(self.items.as_mut_ptr().cast::<T>(), self.count) }
    }
}

/// A list of moves, room for more than the 218 any chess position allows.
pub type MoveList = StackList<Move, 256>;

impl MoveList {
    /// Add one move for each bit of the targets.
    #[inline(always)]
    fn add_targets(&mut self, start: usize, mut targets: u64) {
        while targets != 0 {
            let target = targets.trailing_zeros() as usize;
            targets &= targets - 1;
            self.add(new_move(start, target, 0));
        }
    }

    /// Add a pawn move, or its four promotions on the last rank.
    #[inline(always)]
    fn add_pawn_move(&mut self, start: usize, target: usize) {
        if !(8..56).contains(&target) {
            self.add(new_move(start, target, QUEEN));
            self.add(new_move(start, target, ROOK));
            self.add(new_move(start, target, BISHOP));
            self.add(new_move(start, target, KNIGHT));
        } else {
            self.add(new_move(start, target, 0));
        }
    }
}

/// Add the moves of the side's knights, bishops, rooks, queens, and king onto the
/// squares in the mask.
#[inline(always)]
fn add_piece_moves(position: &Position, own: u64, occupied: u64, mask: u64, list: &mut MoveList) {
    let kinds = &position.by_kind;
    let mut pieces = own & kinds[KNIGHT as usize];
    while pieces != 0 {
        let start = pieces.trailing_zeros() as usize;
        pieces &= pieces - 1;
        list.add_targets(start, KNIGHT_ATTACKS[start] & mask);
    }
    let mut pieces = own & kinds[BISHOP as usize];
    while pieces != 0 {
        let start = pieces.trailing_zeros() as usize;
        pieces &= pieces - 1;
        list.add_targets(start, bishop_attacks(start, occupied) & mask);
    }
    let mut pieces = own & kinds[ROOK as usize];
    while pieces != 0 {
        let start = pieces.trailing_zeros() as usize;
        pieces &= pieces - 1;
        list.add_targets(start, rook_attacks(start, occupied) & mask);
    }
    let mut pieces = own & kinds[QUEEN as usize];
    while pieces != 0 {
        let start = pieces.trailing_zeros() as usize;
        pieces &= pieces - 1;
        let reach = rook_attacks(start, occupied) | bishop_attacks(start, occupied);
        list.add_targets(start, reach & mask);
    }
    let king = (own & kinds[KING as usize]).trailing_zeros() as usize;
    list.add_targets(king, KING_ATTACKS[king] & mask);
}

/// Add the castling moves. The search checks the square where the king lands.
fn add_castling_moves(position: &Position, list: &mut MoveList) {
    let white = position.white;
    let board = &position.board;
    let king: usize = if white { 60 } else { 4 }; // e1 or e8
    let kingside: u8 = if white { 1 } else { 4 };
    let queenside = kingside * 2;
    let kingside_open =
        position.castling & kingside != 0 && board[king + 1] == EMPTY && board[king + 2] == EMPTY;
    let queenside_open = position.castling & queenside != 0
        && board[king - 3] == EMPTY
        && board[king - 2] == EMPTY
        && board[king - 1] == EMPTY;
    if !(kingside_open || queenside_open) || is_attacked(position, king, !white) {
        return;
    }
    if kingside_open && !is_attacked(position, king + 1, !white) {
        list.add(new_move(king, king + 2, 0));
    }
    if queenside_open && !is_attacked(position, king - 1, !white) {
        list.add(new_move(king, king - 2, 0));
    }
}

/// List the pseudo-legal moves of the side to move.
pub fn generate_moves(position: &Position, list: &mut MoveList) {
    let white = position.white;
    let side = usize::from(!white);
    let own = position.by_side[side];
    let enemy = position.by_side[1 - side];
    let occupied = own | enemy;
    let mut pawns = own & position.by_kind[PAWN as usize];
    // A pawn may capture on the en passant square as well as onto an enemy piece
    let mut pawn_targets = enemy;
    if position.en_passant >= 0 {
        pawn_targets |= 1_u64 << position.en_passant;
    }
    let first_row = if white { 6 } else { 1 };
    while pawns != 0 {
        let start = pawns.trailing_zeros() as usize;
        pawns &= pawns - 1;
        // White pawns move toward row 0, and one row is 8 squares
        let ahead = if white { start - 8 } else { start + 8 };
        if position.board[ahead] == EMPTY {
            list.add_pawn_move(start, ahead);
            if start / 8 == first_row {
                let two_ahead = if white { ahead - 8 } else { ahead + 8 };
                if position.board[two_ahead] == EMPTY {
                    list.add(new_move(start, two_ahead, 0));
                }
            }
        }
        let mut captures = PAWN_ATTACKS[side][start] & pawn_targets;
        while captures != 0 {
            let target = captures.trailing_zeros() as usize;
            captures &= captures - 1;
            list.add_pawn_move(start, target);
        }
    }
    add_piece_moves(position, own, occupied, !own, list);
    if position.castling != 0 {
        add_castling_moves(position, list);
    }
}

/// List only the moves quiescence can search: captures and queen promotions. Remus4
/// sorts every move and stops at the first quiet one, which tries these same moves.
pub fn generate_captures(position: &Position, list: &mut MoveList) {
    let white = position.white;
    let side = usize::from(!white);
    let own = position.by_side[side];
    let enemy = position.by_side[1 - side];
    let occupied = own | enemy;
    let mut pawns = own & position.by_kind[PAWN as usize];
    while pawns != 0 {
        let start = pawns.trailing_zeros() as usize;
        pawns &= pawns - 1;
        let ahead = if white { start - 8 } else { start + 8 };
        if !(8..56).contains(&ahead) && position.board[ahead] == EMPTY {
            list.add(new_move(start, ahead, QUEEN));
        }
        let mut captures = PAWN_ATTACKS[side][start] & enemy;
        while captures != 0 {
            let target = captures.trailing_zeros() as usize;
            captures &= captures - 1;
            list.add_pawn_move(start, target);
        }
    }
    add_piece_moves(position, own, occupied, enemy, list);
}
