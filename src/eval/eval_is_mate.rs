use cozy_chess::{Board, Color, GameStatus};

pub fn eval_is_mate(board: &Board, distance_from_root: u8) -> i32 {
    if board.status() == GameStatus::Won {
        if board.side_to_move() == Color::White {
            1000000 - (1000 * distance_from_root as i32) as i32
        } else {
            -1000000 + (1000 * distance_from_root as i32) as i32
        }
    } else {
        0
    }
}