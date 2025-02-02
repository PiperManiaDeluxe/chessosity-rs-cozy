use cozy_chess::{Board, Color, GameStatus};

pub fn eval_is_mate(board: &Board) -> i32 {
    if board.status() == GameStatus::Won {
        10000 * if board.side_to_move() == Color::White {
            -1
        } else {
            1
        }
    } else {
        0
    }
}