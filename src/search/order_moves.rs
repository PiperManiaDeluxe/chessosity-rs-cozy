use crate::eval::eval_count_material::get_piece_value_opening;
use cozy_chess::{BitBoard, Board, Move};
use std::collections::{HashMap, HashSet};

pub fn order_moves(board: &Board, moves: Vec<Move>, killer_moves: &HashSet<Move>) -> Vec<Move> {
    let mut moves_with_scores: Vec<(Move, i32)> = moves
        .into_iter()
        .map(|mv| {
            let score = move_order_score(board, &mv, killer_moves);
            (mv, score)
        })
        .collect();

    // Sort descending: highest score first.
    moves_with_scores.sort_by(|a, b| b.1.cmp(&a.1));

    // Return the sorted moves.
    moves_with_scores.into_iter().map(|(mv, _)| mv).collect()
}

pub fn move_order_score(board: &Board, mv: &Move, killer_moves: &HashSet<Move>) -> i32 {
    let mut score = 0;

    // Give a very big bonus for killer moves
    if killer_moves.contains(mv) {
        score += 1_000_000;
    }

    // If the move is a capture add a bonus
    if let Some(captured_piece) = board.piece_on(mv.to) {
        let victim_value = get_piece_value_opening(captured_piece);
        let attacker_value = get_piece_value_opening(board.piece_on(mv.from).unwrap());
        score += victim_value * 10 - attacker_value;
    }

    // If the move leads to check add a bonus
    let mut move_board = board.clone();
    move_board.play_unchecked(*mv);
    score += move_board.checkers().len() as i32 * 100_000;

    score
}
