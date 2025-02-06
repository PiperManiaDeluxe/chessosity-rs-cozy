use cozy_chess::{Board, Color};
use crate::eval::eval_count_material::eval_count_material;
use crate::eval::eval_is_mate::eval_is_mate;
use crate::eval::eval_mop_up::eval_mop_up;
use crate::eval::game_phase::get_game_phase;

pub fn eval(board: &Board, distance_from_root: u8) -> i32 {
    let mut score_opening = eval_opening(board, distance_from_root);
    let mut score_endgame = eval_endgame(board, distance_from_root);

    let phase = get_game_phase(board);

    ((score_opening * (256 - phase)) + (score_endgame * phase)) / 256
}

pub fn eval_opening(board: &Board, distance_from_root: u8) -> i32 {
    let mut score = eval_count_material(board);
    score += eval_is_mate(board, distance_from_root);
    score
}

pub fn eval_endgame(board: &Board, distance_from_root: u8) -> i32 {
    let mut score = eval_count_material(board);
    score += eval_mop_up(board, distance_from_root);
    score += eval_is_mate(board, distance_from_root);
    score
}
