use crate::eval::eval::eval;
use crate::eval::eval_count_material::get_piece_value;
use crate::search::transposition_table::TranspositionTable;
use cozy_chess::{Board, Color, Move, Piece};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn quiescence(
    board: &Board,
    hash_history: Vec<u64>,
    mut alpha: i32,
    mut beta: i32,
    distance_from_root: u8,
    is_playing: &Arc<AtomicBool>,
    node_count: &mut u64,
) -> i32 {
    if !is_playing.load(std::sync::atomic::Ordering::SeqCst) {
        return 0;
    }

    *node_count += 1;

    let stand_pat = eval(board, distance_from_root);
    let maximizing = board.side_to_move() == Color::White;

    if maximizing {
        if stand_pat >= beta {
            return beta;
        }
        alpha = alpha.max(stand_pat);
    } else {
        if stand_pat <= alpha {
            return alpha;
        }
        beta = beta.min(stand_pat);
    }

    let mut moves = Vec::new();
    board.generate_moves(|mvs| {
        // Only consider capture moves
        for mv in mvs {
            if board.piece_on(mv.to).is_some() {
                moves.push(mv);
            }
        }
        false
    });

    // Sort moves by capture value
    moves.sort_by_cached_key(|mv| {
        let target = board.piece_on(mv.to).unwrap_or(Piece::Pawn);
        -get_piece_value(target) // Negative for descending sort
    });

    for mv in moves {
        let mut new_board = board.clone();
        let mut new_hash_history = hash_history.clone();
        new_board.play(mv);
        new_hash_history.push(new_board.hash());

        let score = quiescence(
            &new_board,
            new_hash_history,
            alpha,
            beta,
            distance_from_root + 1,
            is_playing,
            node_count,
        );

        if maximizing {
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break;
            }
        } else {
            if score < beta {
                beta = score;
            }
            if beta <= alpha {
                break;
            }
        }
    }

    if maximizing {
        alpha
    } else {
        beta
    }
}
