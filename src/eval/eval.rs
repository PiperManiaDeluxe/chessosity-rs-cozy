use cozy_chess::Board;
use crate::eval::eval_count_material::eval_count_material;
use crate::eval::eval_is_mate::eval_is_mate;

pub fn eval(board: &Board, distance_from_root: u8) -> i32 {
    let mut score = eval_count_material(board);
    score += eval_is_mate(board, distance_from_root);
    score
}