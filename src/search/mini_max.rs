use crate::eval::eval::eval;
use crate::search::is_threefold::is_threefold;
use crate::search::order_moves::order_moves;
use crate::search::quiescence::quiescence;
use crate::search::transposition_table::{
    TranspositionTable, TranspositionTableEntry, TranspositionTableEntryType,
};
use cozy_chess::{BitBoard, Board, BoardBuilder, Color, GameStatus, Move};
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
) -> (i32, Option<Move>, bool, Vec<Move>) {
    let mut best_pv: Vec<Move> = Vec::new();

    if !is_playing.load(std::sync::atomic::Ordering::SeqCst) {
        return (0, None, true, best_pv);
    }

    let hash = board.hash();
    if let Some(&ref entry) = transposition_table.get(hash) {
        if entry.depth >= depth {
            return (entry.score, entry.best_move, false, entry.pv.clone());
        }
    }

    if board.status() != GameStatus::Ongoing {
        return (eval(&board, distance_from_root), None, false, best_pv);
    }
    if is_threefold(hash, &hash_history) {
        return (0, None, false, best_pv);
    }
    if depth == 0 {
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

    let maximizing = board.side_to_move() == Color::White;
    *node_count += 1;

    let mut moves = Vec::new();
    board.generate_moves(|mvs| {
        moves.extend(mvs);

        false
    });

    moves = order_moves(&board, moves);

    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    let mut best_move: Option<Move> = None;

    for mv in moves {
        let mut new_board = board.clone();
        let mut new_hash_history = hash_history.clone();
        new_board.play(mv);
        new_hash_history.push(new_board.hash());

        let (score, _, early_stop, child_pv) = mini_max(
            &new_board,
            transposition_table,
            new_hash_history,
            depth - 1,
            alpha,
            beta,
            distance_from_root + 1,
            is_playing,
            node_count,
        );

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
                break;
            }
            beta = beta.min(best_score);
        }
    }

    if let Some(mv) = best_move {
        transposition_table.insert(
            hash,
            TranspositionTableEntry {
                depth,
                score: best_score,
                best_move: Some(mv),
                entry_type: if best_score <= alpha {
                    TranspositionTableEntryType::LowerBound
                } else if best_score >= beta {
                    TranspositionTableEntryType::UpperBound
                } else {
                    TranspositionTableEntryType::Exact
                },
                pv: best_pv.clone(),
            },
        );
    }

    (best_score, best_move, false, best_pv.clone())
}
