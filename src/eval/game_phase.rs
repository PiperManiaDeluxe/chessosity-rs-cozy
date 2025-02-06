use cozy_chess::{Board, Color, Piece};

pub fn get_game_phase(board: &Board) -> i32 {
    const PAWN_PHASE: i32 = 0;
    const KNIGHT_PHASE: i32 = 1;
    const BISHOP_PHASE: i32 = 1;
    const ROOK_PHASE: i32 = 2;
    const QUEEN_PHASE: i32 = 4;

    let total_phase =
        PAWN_PHASE * 16 + KNIGHT_PHASE * 4 + BISHOP_PHASE * 4 + ROOK_PHASE * 4 + QUEEN_PHASE * 2;

    let mut phase = total_phase;

    phase -= board.colored_pieces(Color::White, Piece::Pawn).len() as i32 * PAWN_PHASE;

    phase -= board.colored_pieces(Color::Black, Piece::Pawn).len() as i32 * PAWN_PHASE;
    phase -= board.colored_pieces(Color::White, Piece::Knight).len() as i32 * KNIGHT_PHASE;
    phase -= board.colored_pieces(Color::Black, Piece::Knight).len() as i32 * KNIGHT_PHASE;
    phase -= board.colored_pieces(Color::White, Piece::Bishop).len() as i32 * BISHOP_PHASE;
    phase -= board.colored_pieces(Color::Black, Piece::Bishop).len() as i32 * BISHOP_PHASE;
    phase -= board.colored_pieces(Color::White, Piece::Rook).len() as i32 * ROOK_PHASE;
    phase -= board.colored_pieces(Color::Black, Piece::Rook).len() as i32 * ROOK_PHASE;
    phase -= board.colored_pieces(Color::White, Piece::Queen).len() as i32 * QUEEN_PHASE;
    phase -= board.colored_pieces(Color::Black, Piece::Queen).len() as i32 * QUEEN_PHASE;

   (phase * 256 + (total_phase / 2)) / total_phase
}
