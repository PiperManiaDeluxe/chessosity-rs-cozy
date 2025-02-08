use cozy_chess::{Board, Color, Piece};

pub fn eval_count_material(board: &Board) -> i32{
    let mut score: i32 = 0;
    let white_pawns = board.colored_pieces(Color::White, Piece::Pawn);
    let white_knights = board.colored_pieces(Color::White, Piece::Knight);
    let white_bishops = board.colored_pieces(Color::White, Piece::Bishop);
    let white_rooks = board.colored_pieces(Color::White, Piece::Rook);
    let white_queens = board.colored_pieces(Color::White, Piece::Queen);
    let black_pawns = board.colored_pieces(Color::Black, Piece::Pawn);
    let black_knights = board.colored_pieces(Color::Black, Piece::Knight);
    let black_bishops = board.colored_pieces(Color::Black, Piece::Bishop);
    let black_rooks = board.colored_pieces(Color::Black, Piece::Rook);
    let black_queens = board.colored_pieces(Color::Black, Piece::Queen);

    score += white_pawns.len() as i32 * get_piece_value(Piece::Pawn);
    score += white_knights.len() as i32 * get_piece_value(Piece::Knight);
    score += white_bishops.len() as i32 * get_piece_value(Piece::Bishop);
    score += white_rooks.len() as i32 * get_piece_value(Piece::Rook);
    score += white_queens.len() as i32 * get_piece_value(Piece::Queen);
    score -= black_pawns.len() as i32 * get_piece_value(Piece::Pawn);
    score -= black_knights.len() as i32 * get_piece_value(Piece::Knight);
    score -= black_bishops.len() as i32 * get_piece_value(Piece::Bishop);
    score -= black_rooks.len() as i32 * get_piece_value(Piece::Rook);
    score -= black_queens.len() as i32 * get_piece_value(Piece::Queen);

    // Bishop pair bonus
    if white_bishops.len() >= 2 {
        score += 50;
    }
    if black_bishops.len() >= 2 {
        score -= 50;
    }

    score
}

pub fn get_piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 305,
        Piece::Bishop => 333,
        Piece::Rook => 563,
        Piece::Queen => 950,
        _ => 0,
    }
}

