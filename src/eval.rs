use cozy_chess::{Board, GameStatus};
use crate::eval_count_material::eval_count_material;
use crate::eval_mate_in::eval_is_mate;

pub fn eval(board: &Board, distance_from_root: u32) -> i32 {
    let mut score = 0;

    // Material
    score += eval_count_material(board);
    // Mate
    score += eval_is_mate(board, distance_from_root);
    // Is draw
    if board.status() == GameStatus::Drawn {
        score = 0;
    }

    score
}