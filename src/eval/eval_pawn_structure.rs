use cozy_chess::{BitBoard, Board, Color, Piece};

pub const A_FILE: u64 = 0x101010101010101;
pub const B_FILE: u64 = 0x202020202020202;
pub const C_FILE: u64 = 0x404040404040404;
pub const D_FILE: u64 = 0x808080808080808;
pub const E_FILE: u64 = 0x1010101010101010;
pub const F_FILE: u64 = 0x2020202020202020;
pub const G_FILE: u64 = 0x4040404040404040;
pub const H_FILE: u64 = 0x8080808080808080;

pub const ARR_FILES: [u64; 8] = [
    A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE,
];
pub const ARR_NEIGHBOR_FILES: [u64; 8] = [
    B_FILE,
    A_FILE | C_FILE,
    B_FILE | D_FILE,
    C_FILE | E_FILE,
    D_FILE | F_FILE,
    E_FILE | G_FILE,
    F_FILE | H_FILE,
    G_FILE,
];

pub const STACKED_PAWN_PENALTY: i32 = 25;
pub const ISOLATED_PAWN_PENALTY: i32 = 50;
pub const PASSED_PAWN_BONUS: i32 = 175;
pub const PASSED_PAWN_RANK_BONUS: i32 = 25;

pub fn eval_pawn_structure(board: &Board) -> i32 {
    let mut score = 0;

    let white_pawns = board.colored_pieces(Color::White, Piece::Pawn);
    let black_pawns = board.colored_pieces(Color::Black, Piece::Pawn);

    // Eval white pawns
    for pawn in white_pawns {
        // Detect doubled / stacked pawns
        let file = pawn as i32 % 8;
        let pawns_on_file = (BitBoard(ARR_FILES[file as usize]) & white_pawns).len();
        if pawns_on_file > 1 {
            score -= (pawns_on_file as i32 - 1) * STACKED_PAWN_PENALTY;
        }

        // Detect isolated pawns
        let is_isolated = (BitBoard(ARR_NEIGHBOR_FILES[file as usize]) & white_pawns).len() == 0;
        if (is_isolated) {
            score -= ISOLATED_PAWN_PENALTY;
        }

        // Detect passed pawns
        let rank = pawn as i32 / 8;
        let passed_mask = if rank < 7 {
            let rank_mask = u64::MAX << ((rank + 1) * 8);
            (ARR_FILES[file as usize] | ARR_NEIGHBOR_FILES[file as usize]) & rank_mask
        } else {
            0
        };

        if (BitBoard(passed_mask) & black_pawns).len() == 0 {
            score += PASSED_PAWN_BONUS;
            score += rank * PASSED_PAWN_RANK_BONUS;
        }
    }

    // Eval black pawns
    for pawn in black_pawns {
        // Detect doubled / stacked pawns
        let file = pawn as i32 % 8;
        let pawns_on_file = (BitBoard(ARR_FILES[file as usize]) & black_pawns).len();
        if pawns_on_file > 1 {
            score += (pawns_on_file as i32 - 1) * STACKED_PAWN_PENALTY;
        }

        // Detect isolated pawns
        let is_isolated = (BitBoard(ARR_NEIGHBOR_FILES[file as usize]) & black_pawns).len() == 0;
        if (is_isolated) {
            score += ISOLATED_PAWN_PENALTY;
        }

        // Detect passed pawns
        let rank = pawn as i32 / 8;
        let passed_mask = if rank > 0 {
            // All squares on ranks strictly less than the pawn’s rank.
            let rank_mask = (1u64 << (rank * 8)) - 1;
            (ARR_FILES[file as usize] | ARR_NEIGHBOR_FILES[file as usize]) & rank_mask
        } else {
            0
        };

        if (BitBoard(passed_mask) & white_pawns).len() == 0 {
            score -= PASSED_PAWN_BONUS;
            score -= (7 - rank) * PASSED_PAWN_RANK_BONUS;
        }
    }

    score
}
