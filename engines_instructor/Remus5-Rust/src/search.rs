//! Choose a move by alpha-beta search. Every step matches search.py in Remus5, so a
//! search to a fixed depth matches when table replacement does not intervene.

use std::io::Write;
use std::time::Instant;

use crate::attacks::{QUEEN_LINES, in_check};
use crate::board::{BISHOP, KING, KNIGHT, ROOK};
use crate::board::{
    EMPTY, MATE_SCORE, Move, NO_MOVE, PAWN, Position, QUEEN, move_promotion, move_start,
    move_target, move_text,
};
use crate::evaluation::{PIECE_VALUES, evaluate};
use crate::exchange::exchange_score;
use crate::make_move::{make_move, make_null_move};
use crate::move_generation::{MoveList, StackList, generate_captures, generate_moves};
use crate::search_control::SearchControl;
use crate::table_metrics::TableMetrics;
use crate::transposition_table::{EXACT, LOWER, Table, UPPER};

const FUTILITY_MARGIN: i32 = 200; // A quiet move seldom gains more than this
const DELTA_MARGIN: i32 = 200; // A capture seldom gains more than its victim plus this
const ASPIRATION_WINDOW: i32 = 50; // The first window around the last score
const CAPTURE: i32 = 10_000; // Every capture and queen promotion scores above this
const HISTORY_LIMIT: i32 = 3999; // Quiet history stays below both killer moves
const MAX_PLY: usize = 128; // Killer moves are kept for this many plies
const MAX_DEPTH: i32 = 64; // Iterative deepening stops here
const EARLY_PICKS: usize = 3; // Moves chosen one at a time before the rest are sorted

/// The sort entries of one position's moves.
type EntryList = StackList<u64, 256>;

/// The search stops by returning this error up through every call.
struct SearchTimeout;

/// Everything the search keeps from one position to the next.
pub struct Search {
    nodes: u64,
    pub control: SearchControl,
    best_move: Move,
    history: Vec<u64>,
    killers: [[Move; 2]; MAX_PLY],
    quiet_history: [[[i32; 64]; 64]; 2],
    table: Table,
    metrics: TableMetrics,
}

/// Return a list entry that sorts by priority and then by move, as Remus5 sorts the
/// pairs (priority, move).
#[inline(always)]
fn sort_entry(priority: i32, mv: Move) -> u64 {
    (u64::from(priority as u32) << 16) | u64::from(mv)
}

#[inline(always)]
fn entry_move(entry: u64) -> Move {
    entry as Move
}

/// Return the captured kind, including a pawn taken en passant, or EMPTY.
#[inline(always)]
fn captured_kind(position: &Position, mv: Move) -> u8 {
    if position.board[move_start(mv)] & 7 == PAWN && move_target(mv) as i8 == position.en_passant {
        return PAWN;
    }
    position.board[move_target(mv)] & 7
}

/// The priority of a move before the table, killers, and history have their say.
#[inline(always)]
fn capture_priority(position: &Position, mv: Move) -> i32 {
    let mut priority = 0;
    let victim = captured_kind(position, mv);
    if victim != EMPTY {
        let attacker = PIECE_VALUES[(position.board[move_start(mv)] & 7) as usize];
        priority = CAPTURE + 10 * PIECE_VALUES[victim as usize] - attacker;
    }
    if move_promotion(mv) == QUEEN {
        priority += CAPTURE + 9000;
    }
    priority
}

/// Fill the entries with the moves of the position and their priorities. The entries
/// stand in the order of generation: place_next() or a sort puts the likely best first.
#[inline]
fn ordered_moves(
    position: &Position,
    first: Move,
    killers: [Move; 2],
    quiet_history: &[[i32; 64]; 64],
    entries: &mut EntryList,
) {
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for &mut mv in list.as_mut_slice() {
        let mut priority = capture_priority(position, mv);
        if captured_kind(position, mv) == EMPTY && move_promotion(mv) == 0 {
            priority = quiet_history[move_start(mv)][move_target(mv)];
            if mv == killers[0] {
                priority = 5000;
            } else if mv == killers[1] {
                priority = 4000;
            }
        }
        if mv == first {
            priority = 1_000_000;
        }
        entries.add(sort_entry(priority, mv));
    }
}

/// Put the entry that belongs at this index in its place, so the moves come out in
/// the order a full sort gives. Each of the first few indexes takes the largest entry
/// left, which costs one pass over the list. The index after those sorts the rest.
/// Most cutoffs come from the first few moves, and then the sort never runs.
#[inline(always)]
fn place_next(entries: &mut [u64], index: usize) {
    if index < EARLY_PICKS {
        let mut best = index;
        for other in index + 1..entries.len() {
            if entries[other] > entries[best] {
                best = other;
            }
        }
        entries.swap(index, best);
    } else if index == EARLY_PICKS {
        entries[index..].sort_unstable_by(|a, b| b.cmp(a));
    }
}

/// Return true if the side to move has a legal move. The caller says whether that
/// side is in check.
fn has_legal_move(position: &Position, check: bool) -> bool {
    if !check {
        // Out of check, a move can leave its own king attacked only by moving the
        // king, by capturing en passant, or by leaving a line through the king. A
        // pawn off those lines with an empty square ahead has a legal step, and three
        // bitboard operations find one without generating a move.
        let own = position.by_side[usize::from(!position.white)];
        let king = (own & position.by_kind[KING as usize]).trailing_zeros() as usize;
        let free = own & position.by_kind[PAWN as usize] & !QUEEN_LINES[king & 63];
        let ahead = if position.white { free >> 8 } else { free << 8 };
        if ahead & !position.occupied() != 0 {
            return true;
        }
    }
    // Stop at the first move that leaves the moving side's king safe
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for &mut mv in list.as_mut_slice() {
        if !in_check(&make_move(position, mv), position.white) {
            return true;
        }
    }
    false
}

/// Return true if the side has a piece other than its king and pawns.
#[inline(always)]
fn has_pieces(position: &Position) -> bool {
    let kinds = &position.by_kind;
    let pieces = kinds[KNIGHT as usize]
        | kinds[BISHOP as usize]
        | kinds[ROOK as usize]
        | kinds[QUEEN as usize];
    pieces & position.by_side[usize::from(!position.white)] != 0
}

impl Search {
    pub fn new() -> Search {
        Search {
            nodes: 0,
            control: SearchControl::new(),
            best_move: NO_MOVE,
            history: Vec::with_capacity(1024),
            killers: [[NO_MOVE; 2]; MAX_PLY],
            quiet_history: [[[0; 64]; 64]; 2],
            table: Table::new(),
            metrics: TableMetrics::default(),
        }
    }

    /// Count one more position, and look at the clock every 1,024.
    #[inline(always)]
    fn count_node(&mut self) -> Result<(), SearchTimeout> {
        self.nodes += 1;
        if self.nodes.is_multiple_of(1024) && self.control.poll(self.nodes) {
            return Err(SearchTimeout);
        }
        Ok(())
    }

    /// Search captures and queen promotions, or every legal evasion in check.
    /// Checkmate and stalemate take precedence over standing pat or pruning.
    fn quiescence(
        &mut self,
        position: &Position,
        mut alpha: i32,
        beta: i32,
        ply: usize,
    ) -> Result<i32, SearchTimeout> {
        self.count_node()?;
        let white = position.white;
        let check = in_check(position, white);
        if !has_legal_move(position, check) {
            return Ok(if check { -(MATE_SCORE - ply as i32) } else { 0 });
        }
        if self.is_draw(position) {
            return Ok(0);
        }
        let stand_pat = evaluate(position);
        if ply >= MAX_PLY {
            return Ok(stand_pat); // Bound the stack during long checking sequences
        }
        if !check {
            if stand_pat >= beta {
                return Ok(stand_pat);
            }
            alpha = alpha.max(stand_pat);
        }

        let mut entries = EntryList::new();
        if check {
            ordered_moves(
                position,
                NO_MOVE,
                [NO_MOVE; 2],
                &self.quiet_history[usize::from(!white)],
                &mut entries,
            );
            entries.as_mut_slice().sort_unstable_by(|a, b| b.cmp(a));
        } else {
            let mut list = MoveList::new();
            generate_captures(position, &mut list);
            for &mut mv in list.as_mut_slice() {
                let priority = capture_priority(position, mv);
                if priority >= CAPTURE {
                    entries.add(sort_entry(priority, mv));
                }
            }
            entries.as_mut_slice().sort_unstable_by(|a, b| b.cmp(a));
        }
        for &mut entry in entries.as_mut_slice() {
            let mv = entry_move(entry);
            let child = make_move(position, mv);
            if in_check(&child, white) {
                continue;
            }
            if !check && move_promotion(mv) == 0 {
                let victim = PIECE_VALUES[captured_kind(position, mv) as usize];
                if stand_pat + victim + DELTA_MARGIN < alpha {
                    continue;
                }
                let exchange =
                    exchange_score(&position.board, &position.by_kind, &position.by_side, mv);
                if exchange < 0 {
                    continue;
                }
            }
            // Evasions can repeat a position, so quiescence keeps the path too.
            self.history.push(position.key);
            let result = self.quiescence(&child, -beta, -alpha, ply + 1);
            self.history.pop();
            let score = -result?;
            if score >= beta {
                return Ok(score);
            }
            alpha = alpha.max(score);
        }
        Ok(alpha)
    }

    /// Return true for the fifty-move rule or a repeated position.
    #[inline(always)]
    fn is_draw(&self, position: &Position) -> bool {
        if position.halfmove >= 100 {
            return true;
        }
        // The slice history[len - halfmove:] of Remus5, where a negative start counts
        // back from the end of the list
        let length = self.history.len() as i64;
        let mut start = length - i64::from(position.halfmove);
        if start < 0 {
            start = (length + start).max(0);
        }
        self.history[start as usize..].contains(&position.key)
    }

    /// Return the score for the side to move, searching depth moves ahead.
    fn alpha_beta(
        &mut self,
        position: &Position,
        mut depth: i32,
        ply: usize,
        mut alpha: i32,
        beta: i32,
        can_null: bool,
    ) -> Result<i32, SearchTimeout> {
        self.count_node()?;
        let white = position.white;
        let key = position.key;
        let check = in_check(position, white);
        if ply > 0 && self.is_draw(position) {
            if check && !has_legal_move(position, true) {
                return Ok(-(MATE_SCORE - ply as i32));
            }
            return Ok(0);
        }
        if ply >= MAX_PLY {
            return self.quiescence(position, alpha, beta, ply);
        }
        if check {
            depth += 1; // Never stop the search in check
        }
        if depth <= 0 {
            return self.quiescence(position, alpha, beta, ply);
        }
        let mut first = NO_MOVE;
        self.metrics.probes += 1;
        if let Some(stored) = self.table.probe(key, ply as i32) {
            self.metrics.hits += 1;
            first = stored.mv;
            if ply > 0
                && stored.depth >= depth
                && (stored.bound == EXACT
                    || (stored.bound == LOWER && stored.score >= beta)
                    || (stored.bound == UPPER && stored.score <= alpha))
            {
                self.metrics.cutoffs += 1;
                return Ok(stored.score);
            }
        }
        // Only two tests read the score, both out of check: null move at depth 3 and
        // up, and futility at depth 1. Every other node skips the evaluation.
        let needs_score = !check && (depth == 1 || (depth >= 3 && can_null && ply > 0));
        let static_score = if needs_score { evaluate(position) } else { 0 };
        // Null move: let the opponent move twice. If the side to move still stands above
        // beta with a shallower search, its real moves will too.
        if can_null
            && !check
            && ply > 0
            && depth >= 3
            && static_score >= beta
            && has_pieces(position)
            && has_legal_move(position, false)
        {
            let null = make_null_move(position);
            self.table.prefetch(null.key);
            self.history.push(key);
            let result = self.alpha_beta(&null, depth - 3, ply + 1, -beta, -beta + 1, false);
            self.history.pop();
            if -result? >= beta {
                return Ok(beta);
            }
        }
        self.history.push(key);
        let original_alpha = alpha;
        let mut best_score = -MATE_SCORE - 1;
        let mut best = NO_MOVE;
        let mut legal_count = 0;
        let mut entries = EntryList::new();
        ordered_moves(
            position,
            first,
            self.killers[ply],
            &self.quiet_history[usize::from(!white)],
            &mut entries,
        );
        let moves = entries.as_mut_slice();
        for index in 0..moves.len() {
            place_next(moves, index);
            let mv = entry_move(moves[index]);
            let child = make_move(position, mv);
            if depth > 1 {
                // The child looks itself up in the table; at depth 1 it goes straight
                // to quiescence, which does not
                self.table.prefetch(child.key);
            }
            if in_check(&child, white) {
                continue; // Illegal: it leaves the king attacked
            }
            legal_count += 1;
            let quiet = captured_kind(position, mv) == EMPTY && move_promotion(mv) == 0;
            let calm = quiet && !check && !in_check(&child, !white);
            // Futility: one move from the leaves, a quiet move cannot lift a position far
            // below alpha
            if depth == 1 && calm && legal_count > 1 && static_score + FUTILITY_MARGIN <= alpha {
                continue;
            }
            let mut score;
            if legal_count == 1 {
                score = -self.alpha_beta(&child, depth - 1, ply + 1, -beta, -alpha, true)?;
            } else {
                // Late move reduction: a late quiet move gets one ply less
                let reduce = depth >= 3
                    && legal_count > 3
                    && calm
                    && mv != self.killers[ply][0]
                    && mv != self.killers[ply][1];
                // Principal variation search: prove the move is no better than alpha with
                // a null window, and search again if it is
                let reduced_depth = depth - 1 - i32::from(reduce);
                score =
                    -self.alpha_beta(&child, reduced_depth, ply + 1, -alpha - 1, -alpha, true)?;
                if score > alpha && (reduce || score < beta) {
                    score = -self.alpha_beta(&child, depth - 1, ply + 1, -beta, -alpha, true)?;
                }
            }
            if score > best_score {
                best_score = score;
                best = mv;
            }
            if score > alpha {
                alpha = score;
                if ply == 0 {
                    self.best_move = mv;
                }
            }
            if score >= beta {
                if quiet {
                    let entry = &mut self.quiet_history[usize::from(!white)][move_start(mv)]
                        [move_target(mv)];
                    *entry = HISTORY_LIMIT.min(*entry + depth * depth);
                    if self.killers[ply][0] != mv {
                        self.killers[ply] = [mv, self.killers[ply][0]];
                    }
                }
                break;
            }
        }
        self.history.pop();
        if legal_count == 0 {
            // Checkmate, worse the sooner it comes, or stalemate
            return Ok(if check { -(MATE_SCORE - ply as i32) } else { 0 });
        }
        let mut bound = EXACT;
        if best_score >= beta {
            bound = LOWER;
        } else if best_score <= original_alpha {
            bound = UPPER;
        }
        self.metrics.stores += 1;
        self.metrics.replacements += u64::from(
            self.table
                .store(key, depth, best_score, bound, best, ply as i32),
        );
        Ok(best_score)
    }

    /// Return the line of best play that the table holds, as UCI text.
    fn principal_variation(&self, position: &Position, depth: i32) -> String {
        let mut line = String::new();
        let mut current = *position;
        for _ in 0..depth {
            let Some(stored) = self.table.probe(current.key, 0) else {
                break;
            };
            if stored.mv == NO_MOVE {
                break;
            }
            line.push(' ');
            line.push_str(&move_text(stored.mv));
            current = make_move(&current, stored.mv);
        }
        line
    }

    /// Return a legal cached reply after the selected move.
    pub fn ponder_reply(&self, position: &Position, mv: Move) -> Move {
        let child = make_move(position, mv);
        if let Some(stored) = self.table.probe(child.key, 0) {
            let mut moves = MoveList::new();
            generate_moves(&child, &mut moves);
            for &mut reply in moves.as_mut_slice() {
                if reply == stored.mv && !in_check(&make_move(&child, reply), child.white) {
                    return reply;
                }
            }
        }
        NO_MOVE
    }

    /// Search one move deeper at a time and return the best move, or NO_MOVE when there
    /// is no legal move. The search plans to use seconds, takes longer when the best
    /// move keeps changing, and never runs past longest. A depth limit replaces the
    /// clock, for the workshop designer's checks.
    pub fn think(
        &mut self,
        position: &Position,
        game_history: &[u64],
        mut seconds: f64,
        longest: f64,
        depth_limit: Option<i32>,
    ) -> Move {
        let native_started = Instant::now();
        self.metrics = TableMetrics::default();
        self.nodes = 0;
        self.best_move = NO_MOVE;
        // Recent cutoffs matter more than those from earlier moves of the game.
        for side in &mut self.quiet_history {
            for row in side {
                for entry in row {
                    *entry /= 2;
                }
            }
        }
        self.table.new_search();
        let mut score = 0;
        let mut completed_move = NO_MOVE;
        let mut stdout = std::io::stdout();
        'deepening: for depth in 1..=MAX_DEPTH {
            if self.control.poll(self.nodes) {
                break;
            }
            let previous = self.best_move;
            let (mut alpha, mut beta) = (-MATE_SCORE - 1, MATE_SCORE + 1);
            if depth >= 4 {
                alpha = score - ASPIRATION_WINDOW;
                beta = score + ASPIRATION_WINDOW;
            }
            loop {
                self.history.clear();
                self.history.extend_from_slice(game_history);
                match self.alpha_beta(position, depth, 0, alpha, beta, true) {
                    Ok(found) => score = found,
                    Err(SearchTimeout) => break 'deepening,
                }
                // Outside the window: search again with that side open
                if score <= alpha {
                    alpha = -MATE_SCORE - 1;
                } else if score >= beta {
                    beta = MATE_SCORE + 1;
                } else {
                    break;
                }
            }
            if self.best_move == NO_MOVE {
                break; // No legal move
            }
            completed_move = self.best_move;
            let line = self.principal_variation(position, depth);
            let info = format!(
                "info depth {depth} score cp {score} nodes {} pv{line}",
                self.nodes
            );
            let _ = writeln!(stdout, "{info}");
            let _ = stdout.flush();
            // A best move that changes needs more time to settle
            let changed = previous != NO_MOVE && self.best_move != previous;
            if changed {
                seconds = (seconds * 1.5).min(longest);
            }
            self.control.completed(depth, seconds / 2.0, score, changed);
            self.control.log_search(&info);
            if score.abs() > MATE_SCORE - 100 {
                break;
            }
            match depth_limit {
                Some(limit) if depth >= limit => break,
                None if self.control.soft_finished(seconds / 2.0) => break,
                _ => {}
            }
        }
        let native_seconds = native_started.elapsed().as_secs_f64();
        self.control.log_search(&format!(
            "info string serial workers=1 nodes={} native_ms={:.1} nps={:.0} hash_mib={} {}",
            self.nodes,
            native_seconds * 1000.0,
            self.nodes as f64 / native_seconds.max(0.000_001),
            self.table.size_mib(),
            self.metrics
        ));
        self.control.nodes = self.nodes;
        if self.control.was_ponder && completed_move != NO_MOVE {
            self.best_move = completed_move;
        }
        self.best_move
    }
}
