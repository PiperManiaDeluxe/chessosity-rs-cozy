use crate::eval::eval_count_material::get_piece_value;
use cozy_chess::{BitBoard, Board, Move};

pub fn order_moves(board: &Board, moves: Vec<Move>) -> Vec<Move> {
    let mut ordered_moves = moves.clone();

    ordered_moves.sort_by(|a, b| {
        // Prioritize captures
        let is_a_capture = board.piece_on(a.to).is_some();
        let is_b_capture = board.piece_on(b.to).is_some();

        if is_a_capture && !is_b_capture {
            return std::cmp::Ordering::Less;
        } else if !is_a_capture && is_b_capture {
            return std::cmp::Ordering::Greater;
        } else if is_a_capture && is_b_capture {
            // If both are captures, prioritize the capture that cant be recaptured
            let mut a_can_be_recaptured = false;
            let mut b_can_be_recaptured = false;

            // Play A and see if it can be recaptured
            let mut board_a = board.clone();
            board_a.play_unchecked(*a);

            board_a.generate_moves(|mvs| {
                if mvs.into_iter().any(|m| m.to == a.to) {
                    a_can_be_recaptured = true;
                    return true;
                }
                false
            });

            // Play B and see if it can be recaptured
            let mut board_b = board.clone();
            board_b.play_unchecked(*b);
            board_b.generate_moves(|mvs| {
                if mvs.into_iter().any(|m| m.to == b.to) {
                    b_can_be_recaptured = true;
                    return true;
                }
                false
            });

            if a_can_be_recaptured && !b_can_be_recaptured {
                return std::cmp::Ordering::Greater;
            } else if !a_can_be_recaptured && b_can_be_recaptured {
                return std::cmp::Ordering::Less;
            }

            // If both have the same recapture status, prioritize the capture with the higher material delta
            let a_capture_value = get_piece_value(board.piece_on(a.to).unwrap());
            let b_capture_value = get_piece_value(board.piece_on(b.to).unwrap());

            return b_capture_value.cmp(&a_capture_value);
        }

        return std::cmp::Ordering::Equal;
    });

    ordered_moves
}
