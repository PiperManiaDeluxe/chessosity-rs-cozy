use cozy_chess::Board;

pub fn format_move(mv: &cozy_chess::Move, board: &Board) -> String {
    // Really all we need to change are castling moves
    let from_piece = board.piece_on(mv.from);
    let to_piece = board.piece_on(mv.to);

    if from_piece == Some(cozy_chess::Piece::King) && to_piece == Some(cozy_chess::Piece::Rook) {
        if mv.to == cozy_chess::Square::A1 {
            return "e1c1".to_string();
        } else if mv.to == cozy_chess::Square::H1 {
            return "e1g1".to_string();
        } else if mv.to == cozy_chess::Square::A8 {
            return "e8c8".to_string();
        } else if mv.to == cozy_chess::Square::H8 {
            return "e8g8".to_string();
        }
    }

    mv.to_string()
}