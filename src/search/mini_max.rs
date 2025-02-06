use crate::eval::eval::eval;
use crate::search::is_threefold::is_threefold;
use crate::search::order_moves::order_moves;
use crate::search::quiescence::quiescence;
use crate::search::transposition_table::{
    TranspositionTable, TranspositionTableEntry, TranspositionTableEntryType,
};
use cozy_chess::{BitBoard, Board, BoardBuilder, Color, GameStatus, Move};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn mini_max(
    board: &Board,
    transposition_table: &mut TranspositionTable,
    hash_history: Vec<u64>,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    distance_from_root: u8,
    is_playing: &Arc<AtomicBool>,
    node_count: &mut u64,
    killer_moves: &mut HashMap<u8, Vec<Move>>,
) -> (i32, Option<Move>, bool, Vec<Move>) {
    let mut best_pv: Vec<Move> = Vec::new();

    if !is_playing.load(std::sync::atomic::Ordering::SeqCst) {
        return (0, None, true, best_pv);
    }

    let hash = board.hash();
    // TT lookup: if an entry exists and its depth is sufficient, try to cut off.
    if let Some(entry) = transposition_table.get(hash) {
        if entry.depth >= depth && depth > 0 {
            match entry.entry_type {
                TranspositionTableEntryType::Exact => {
                    // Exact values can be returned immediately.
                    if entry.score.abs() < 900_000 {
                        return (entry.score, entry.best_move, false, entry.pv.clone());
                    }
                }
                TranspositionTableEntryType::LowerBound => {
                    // If the TT says the score is at least a lower bound and that lower bound is ≥ β,
                    // we can return immediately.
                    if entry.score >= beta {
                        return (entry.score, entry.best_move, false, entry.pv.clone());
                    }
                }
                TranspositionTableEntryType::UpperBound => {
                    // Similarly, if the TT says the score is at most an upper bound and that bound is ≤ α,
                    // we can return immediately.
                    if entry.score <= alpha {
                        return (entry.score, entry.best_move, false, entry.pv.clone());
                    }
                }
            }
        }
    }

    if board.status() != GameStatus::Ongoing {
        return (eval(&board, distance_from_root + 1), None, false, best_pv);
    }
    if is_threefold(hash, &hash_history) {
        return (0, None, false, best_pv);
    }
    if depth == 0 {
        let score = eval(&board, distance_from_root + 1);

        if score.abs() < 900000 {
            let score = quiescence(
                board,
                hash_history,
                alpha,
                beta,
                distance_from_root,
                is_playing,
                node_count,
            );
            return (score, None, false, best_pv);
        }

        return (score, None, false, best_pv);
    }

    let maximizing = board.side_to_move() == Color::White;
    *node_count += 1;

    let mut moves = Vec::new();
    board.generate_moves(|mvs| {
        moves.extend(mvs);

        false
    });

    moves = order_moves(
        &board,
        moves,
        &(*killer_moves)
            .get(&distance_from_root)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect(),
    );

    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    let mut best_move: Option<Move> = None;

    let mut i = 0;

    for mv in moves {
        let is_capture = board.piece_on(mv.to).is_some();

        let mut needs_full_search = true;

        let mut score = 0;
        let mut early_stop = false;
        let mut child_pv = vec![];

        const REDUCE_DEPTH_A: i32 = 1;
        const REDUCE_DEPTH_B: i32 = 2;

        // Do a shallow search for the later moves
        if i >= 3 && !is_capture && (depth as i32 - 1 - REDUCE_DEPTH_A) >= 0 {
            let mut reduce_depth = REDUCE_DEPTH_A;

            if i >= 7 && (depth as i32 - 1 - REDUCE_DEPTH_B) >= 0{
                // Do an even shallower search fo the even later moves
                reduce_depth = REDUCE_DEPTH_B
            }

            let mut new_board = board.clone();
            let mut new_hash_history = hash_history.clone();
            new_board.play(mv);
            new_hash_history.push(new_board.hash());

            let (new_score, _, new_early_stop, new_child_pv) = mini_max(
                &new_board,
                transposition_table,
                new_hash_history,
                depth - 1 - reduce_depth as u8,
                alpha,
                beta,
                distance_from_root + 1,
                is_playing,
                node_count,
                &mut *killer_moves,
            );

            score = new_score;
            early_stop = new_early_stop;
            child_pv = new_child_pv;

            if maximizing{
                needs_full_search = new_score > alpha;
            } else {
                needs_full_search = new_score < beta;
            }
        }

        let mut new_hash_history = hash_history.clone();

        if needs_full_search{
            let mut new_board = board.clone();
            let mut new_hash_history = hash_history.clone();
            new_board.play(mv);
            new_hash_history.push(new_board.hash());

            let (new_score, _, new_early_stop, new_child_pv) = mini_max(
                &new_board,
                transposition_table,
                new_hash_history,
                depth - 1,
                alpha,
                beta,
                distance_from_root + 1,
                is_playing,
                node_count,
                &mut *killer_moves,
            );

            score = new_score;
            early_stop = new_early_stop;
            child_pv = new_child_pv;
        }

        if early_stop {
            return (0, None, true, Vec::new());
        }

        let current_pv = {
            let mut line = vec![mv];
            line.extend(child_pv);
            line
        };

        if maximizing {
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
                best_pv = current_pv.clone();
            }
            if best_score >= beta {
                let killer_moves_vec = killer_moves
                    .entry(distance_from_root)
                    .or_insert_with(Vec::new);
                killer_moves_vec.push(mv);
                if killer_moves_vec.len() > 32 {
                    killer_moves_vec.remove(0);
                }
                break;
            }
            alpha = alpha.max(best_score);
        } else {
            if score < best_score {
                best_score = score;
                best_move = Some(mv);
                best_pv = current_pv.clone();
            }
            if best_score <= alpha {
                let killer_moves_vec = killer_moves
                    .entry(distance_from_root)
                    .or_insert_with(Vec::new);
                killer_moves_vec.push(mv);
                if killer_moves_vec.len() > 32 {
                    killer_moves_vec.remove(0);
                }
                break;
            }
            beta = beta.min(best_score);
        }

        i += 1;
    }

    // Determine what kind of bound to store in the TT.
    let entry_type = if best_score.abs() >= 900_000 {
        TranspositionTableEntryType::Exact
    } else if best_score <= alpha {
        TranspositionTableEntryType::UpperBound
    } else if best_score >= beta {
        TranspositionTableEntryType::LowerBound
    } else {
        TranspositionTableEntryType::Exact
    };

    if let Some(mv) = best_move {
        transposition_table.insert(
            hash,
            TranspositionTableEntry {
                depth,
                score: best_score,
                best_move: Some(mv),
                entry_type,
                pv: best_pv.clone(),
                hash, // Store the hash so we can verify later if needed.
            },
        );
    }

    (best_score, best_move, false, best_pv.clone())
}
