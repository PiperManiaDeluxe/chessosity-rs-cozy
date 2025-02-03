use cozy_chess::{Board, Color, GameStatus};

pub fn eval_is_mate(board: &Board, distance_from_root: u32) -> i32 {
    if board.status() == GameStatus::Won {
        (100000 - 1000 * distance_from_root as i32)
            * if board.side_to_move() == Color::White {
                -1
            } else {
                1
            }
    } else {
        0
    }
}
