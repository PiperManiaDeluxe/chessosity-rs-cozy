use cozy_chess::{Board, Color, Piece};

pub fn eval_pst_opening(board: &Board) -> i32 {
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

    let mut score: i32 = 0;

    for pawn in white_pawns {
        score += PAWN_OPENING_PST[pawn as usize];
    }
    for knight in white_knights {
        score += KNIGHT_OPENING_PST[knight as usize];
    }
    for bishop in white_bishops {
        score += BISHOP_OPENING_PST[bishop as usize];
    }
    for rook in white_rooks {
        score += ROOK_OPENING_PST[rook as usize];
    }
    for queen in white_queens {
        score += QUEEN_OPENING_PST[queen as usize];
    }

    for pawn in black_pawns {
        score -= PAWN_OPENING_PST[pawn as usize];
    }
    for knight in black_knights {
        score -= KNIGHT_OPENING_PST[knight as usize];
    }
    for bishop in black_bishops {
        score -= BISHOP_OPENING_PST[bishop as usize];
    }
    for rook in black_rooks {
        score -= ROOK_OPENING_PST[rook as usize];
    }
    for queen in black_queens {
        score -= QUEEN_OPENING_PST[queen as usize];
    }

    let white_king = board.king(Color::White);
    let black_king = board.king(Color::Black);

    score += KING_OPENING_PST[white_king as usize];
    score -= KING_OPENING_PST[black_king as usize];

    score
}

pub fn eval_pst_end(board: &Board) -> i32 {
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

    let mut score: i32 = 0;

    for pawn in white_pawns {
        score += PAWN_OPENING_PST[pawn as usize];
    }
    for knight in white_knights {
        score += KNIGHT_OPENING_PST[knight as usize];
    }
    for bishop in white_bishops {
        score += BISHOP_OPENING_PST[bishop as usize];
    }
    for rook in white_rooks {
        score += ROOK_OPENING_PST[rook as usize];
    }
    for queen in white_queens {
        score += QUEEN_OPENING_PST[queen as usize];
    }

    for pawn in black_pawns {
        score -= PAWN_OPENING_PST[63 - pawn as usize];
    }
    for knight in black_knights {
        score -= KNIGHT_OPENING_PST[63 - knight as usize];
    }
    for bishop in black_bishops {
        score -= BISHOP_OPENING_PST[63 - bishop as usize];
    }
    for rook in black_rooks {
        score -= ROOK_OPENING_PST[63 - rook as usize];
    }
    for queen in black_queens {
        score -= QUEEN_OPENING_PST[63 - queen as usize];
    }

    let white_king = board.king(Color::White);
    let black_king = board.king(Color::Black);

    score += KING_END_PST[white_king as usize];
    score -= KING_END_PST[black_king as usize];

    score
}

// Tables taken from https://www.chessprogramming.org/Simplified_Evaluation_Function

#[rustfmt::skip]
const PAWN_OPENING_PST: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
const KNIGHT_OPENING_PST: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[rustfmt::skip]
const BISHOP_OPENING_PST: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

#[rustfmt::skip]
const ROOK_OPENING_PST: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

#[rustfmt::skip]
const QUEEN_OPENING_PST: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[rustfmt::skip]
const KING_OPENING_PST: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

#[rustfmt::skip]
const KING_END_PST: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];
