//! Choose a move by alpha-beta search. Every step matches search.py in Remus4, so a
//! search to a fixed depth visits the same positions and reports the same score.

use std::io::Write;
use std::time::Instant;

use crate::attacks::in_check;
use crate::board::{BISHOP, KNIGHT, ROOK};
use crate::board::{
    EMPTY, MATE_SCORE, Move, NO_MOVE, Position, QUEEN, move_promotion, move_start, move_target,
    move_text,
};
use crate::diagnostics::Diagnostics;
use crate::evaluation::{PIECE_VALUES, evaluate};
use crate::exchange::exchange_score;
use crate::make_move::{make_move, make_null_move};
use crate::move_generation::{MoveList, StackList, generate_captures, generate_moves};
use crate::table_metrics::TableMetrics;
use crate::transposition_table::{EXACT, LOWER, Table, UPPER};

const FUTILITY_MARGIN: i32 = 200; // A quiet move seldom gains more than this
const DELTA_MARGIN: i32 = 200; // A capture seldom gains more than its victim plus this
const ASPIRATION_WINDOW: i32 = 50; // The first window around the last score
const CAPTURE: i32 = 10_000; // Every capture and queen promotion scores above this
const MAX_PLY: usize = 128; // Killer moves are kept for this many plies
const MAX_DEPTH: i32 = 64; // Iterative deepening stops here

/// The sort entries of one position's moves.
type EntryList = StackList<u64, 256>;

/// The search stops by returning this error up through every call.
struct SearchTimeout;

/// Everything the search keeps from one position to the next.
pub struct Search {
    pub diagnostics: Diagnostics,
    nodes: u64,
    metrics: TableMetrics,
    deadline: Option<Instant>,
    best_move: Move,
    history: Vec<u64>,
    killers: [[Move; 2]; MAX_PLY],
    table: Table,
}

/// Return a list entry that sorts by priority and then by move, as Remus4 sorts the
/// pairs (priority, move).
#[inline(always)]
fn sort_entry(priority: i32, mv: Move) -> u64 {
    (u64::from(priority as u32) << 16) | u64::from(mv)
}

#[inline(always)]
fn entry_move(entry: u64) -> Move {
    entry as Move
}

/// The priority of a move before the table and the killer moves have their say.
#[inline(always)]
fn capture_priority(position: &Position, mv: Move) -> i32 {
    let mut priority = 0;
    let target = position.board[move_target(mv)];
    if target != EMPTY {
        let victim = PIECE_VALUES[(target & 7) as usize];
        let attacker = PIECE_VALUES[(position.board[move_start(mv)] & 7) as usize];
        priority = CAPTURE + 10 * victim - attacker;
    }
    if move_promotion(mv) == QUEEN {
        priority += CAPTURE + 9000;
    }
    priority
}

/// Fill the entries with the moves of the position, the likely best first.
#[inline]
fn ordered_moves(position: &Position, first: Move, killers: [Move; 2], entries: &mut EntryList) {
    let mut list = MoveList::new();
    generate_moves(position, &mut list);
    for &mut mv in list.as_mut_slice() {
        let mut priority = capture_priority(position, mv);
        if mv == killers[0] {
            priority = 5000;
        } else if mv == killers[1] {
            priority = 4000;
        }
        if mv == first {
            priority = 1_000_000;
        }
        entries.add(sort_entry(priority, mv));
    }
    entries.as_mut_slice().sort_unstable_by(|a, b| b.cmp(a));
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
            diagnostics: Diagnostics::new(),
            nodes: 0,
            metrics: TableMetrics::default(),
            deadline: None,
            best_move: NO_MOVE,
            history: Vec::with_capacity(1024),
            killers: [[NO_MOVE; 2]; MAX_PLY],
            table: Table::new(),
        }
    }

    /// Count one more position, and look at the clock every 1,024.
    #[inline(always)]
    fn count_node(&mut self) -> Result<(), SearchTimeout> {
        self.nodes += 1;
        if self.nodes.is_multiple_of(1024)
            && self.best_move != NO_MOVE
            && let Some(deadline) = self.deadline
            && Instant::now() > deadline
        {
            return Err(SearchTimeout);
        }
        Ok(())
    }

    /// Search only winning or even captures and queen promotions until the position is
    /// quiet. The side to move may also "stand pat".
    fn quiescence(
        &mut self,
        position: &Position,
        mut alpha: i32,
        beta: i32,
    ) -> Result<i32, SearchTimeout> {
        self.count_node()?;
        let white = position.white;
        let stand_pat = evaluate(position);
        if stand_pat >= beta {
            return Ok(stand_pat);
        }
        alpha = alpha.max(stand_pat);

        let mut list = MoveList::new();
        generate_captures(position, &mut list);
        let mut entries = EntryList::new();
        for &mut mv in list.as_mut_slice() {
            let priority = capture_priority(position, mv);
            if priority >= CAPTURE {
                entries.add(sort_entry(priority, mv));
            }
        }
        let moves = entries.as_mut_slice();
        moves.sort_unstable_by(|a, b| b.cmp(a));

        for &mut entry in moves {
            let mv = entry_move(entry);
            if move_promotion(mv) == 0 {
                // Delta pruning: even winning the victim cannot reach alpha
                let victim = PIECE_VALUES[(position.board[move_target(mv)] & 7) as usize];
                if stand_pat + victim + DELTA_MARGIN < alpha {
                    continue;
                }
                let exchange =
                    exchange_score(&position.board, &position.by_kind, &position.by_side, mv);
                if exchange < 0 {
                    continue; // The capture loses material
                }
            }
            let child = make_move(position, mv);
            if in_check(&child, white) {
                continue; // Illegal: it leaves the king attacked
            }
            let score = -self.quiescence(&child, -beta, -alpha)?;
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
        // The slice history[len - halfmove:] of Remus4, where a negative start counts
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
        if ply > 0 && self.is_draw(position) {
            return Ok(0);
        }
        if ply >= MAX_PLY {
            return Ok(evaluate(position));
        }
        let check = in_check(position, white);
        if check {
            depth += 1; // Never stop the search in check
        }
        if depth <= 0 {
            return self.quiescence(position, alpha, beta);
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
        let static_score = evaluate(position);
        // Null move: let the opponent move twice. If the side to move still stands above
        // beta with a shallower search, its real moves will too.
        if can_null
            && !check
            && ply > 0
            && depth >= 3
            && static_score >= beta
            && has_pieces(position)
        {
            let null = make_null_move(position);
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
        ordered_moves(position, first, self.killers[ply], &mut entries);
        for &mut entry in entries.as_mut_slice() {
            let mv = entry_move(entry);
            let child = make_move(position, mv);
            if in_check(&child, white) {
                continue; // Illegal: it leaves the king attacked
            }
            legal_count += 1;
            let quiet = position.board[move_target(mv)] == EMPTY && move_promotion(mv) == 0;
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
                if quiet && self.killers[ply][0] != mv {
                    self.killers[ply] = [mv, self.killers[ply][0]];
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
        let start = Instant::now();
        self.deadline = if depth_limit.is_some() {
            None
        } else {
            Some(start + std::time::Duration::from_secs_f64(longest.max(0.0)))
        };
        self.nodes = 0;
        self.metrics = TableMetrics::default();
        self.best_move = NO_MOVE;
        self.table.new_search();
        let mut score = 0;
        let mut stdout = std::io::stdout();
        'deepening: for depth in 1..=MAX_DEPTH {
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
            let line = self.principal_variation(position, depth);
            let info = format!(
                "info depth {depth} score cp {score} nodes {} pv{line}",
                self.nodes
            );
            let _ = writeln!(stdout, "{info}");
            let _ = stdout.flush();
            self.diagnostics.completed_depth = depth;
            self.diagnostics.report(&info);
            // A best move that changes needs more time to settle
            if previous != NO_MOVE && self.best_move != previous {
                seconds = (seconds * 1.5).min(longest);
            }
            if score.abs() > MATE_SCORE - 100 {
                break;
            }
            match depth_limit {
                Some(limit) if depth >= limit => break,
                None if start.elapsed().as_secs_f64() > seconds / 2.0 => break,
                _ => {}
            }
        }
        let seconds = start.elapsed().as_secs_f64();
        self.diagnostics.report(&format!(
            "info string serial workers=1 nodes={} native_ms={:.1} nps={:.0} hash_mib={} {}",
            self.nodes,
            seconds * 1000.0,
            self.nodes as f64 / seconds.max(0.000_001),
            self.table.size_mib(),
            self.metrics
        ));
        self.best_move
    }
}
