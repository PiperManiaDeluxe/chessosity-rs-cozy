use cozy_chess::{Board, GameStatus};
use crate::eval_count_material::eval_count_material;
use crate::eval_is_mate::eval_is_mate;

pub fn eval(board: &Board) -> i32 {
    let mut score = 0;

    // Material
    score += eval_count_material(board);
    // Is mate
    score += eval_is_mate(board);
    // Is draw
    if board.status() == GameStatus::Drawn {
        score = 0;
    }

    score
}