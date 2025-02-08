use cozy_chess::{Board, Color, Piece};

pub fn eval_count_material_opening(board: &Board) -> i32{
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

    score += white_pawns.len() as i32 * get_piece_value_opening(Piece::Pawn);
    score += white_knights.len() as i32 * get_piece_value_opening(Piece::Knight);
    score += white_bishops.len() as i32 * get_piece_value_opening(Piece::Bishop);
    score += white_rooks.len() as i32 * get_piece_value_opening(Piece::Rook);
    score += white_queens.len() as i32 * get_piece_value_opening(Piece::Queen);
    score -= black_pawns.len() as i32 * get_piece_value_opening(Piece::Pawn);
    score -= black_knights.len() as i32 * get_piece_value_opening(Piece::Knight);
    score -= black_bishops.len() as i32 * get_piece_value_opening(Piece::Bishop);
    score -= black_rooks.len() as i32 * get_piece_value_opening(Piece::Rook);
    score -= black_queens.len() as i32 * get_piece_value_opening(Piece::Queen);

    // Bishop pair bonus
    if white_bishops.len() >= 2 {
        score += 50;
    }
    if black_bishops.len() >= 2 {
        score -= 50;
    }

    score
}

pub fn eval_count_material_end(board: &Board) -> i32{
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

    score += white_pawns.len() as i32 * get_piece_value_end(Piece::Pawn);
    score += white_knights.len() as i32 * get_piece_value_end(Piece::Knight);
    score += white_bishops.len() as i32 * get_piece_value_end(Piece::Bishop);
    score += white_rooks.len() as i32 * get_piece_value_end(Piece::Rook);
    score += white_queens.len() as i32 * get_piece_value_end(Piece::Queen);
    score -= black_pawns.len() as i32 * get_piece_value_end(Piece::Pawn);
    score -= black_knights.len() as i32 * get_piece_value_end(Piece::Knight);
    score -= black_bishops.len() as i32 * get_piece_value_end(Piece::Bishop);
    score -= black_rooks.len() as i32 * get_piece_value_end(Piece::Rook);
    score -= black_queens.len() as i32 * get_piece_value_end(Piece::Queen);

    // Bishop pair bonus
    if white_bishops.len() >= 2 {
        score += 100;
    }
    if black_bishops.len() >= 2 {
        score -= 100;
    }

    score
}

pub fn get_piece_value_opening(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 80,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 470,
        Piece::Queen => 900,
        _ => 0,
    }
}

pub fn get_piece_value_end(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 530,
        Piece::Queen => 900,
        _ => 0,
    }
}
