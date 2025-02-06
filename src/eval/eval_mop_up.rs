use cozy_chess::{Board, Color};

pub fn eval_mop_up(board: &Board, distance_from_root: u8) -> i32 {
    let opponent_king_square = board.king(if board.side_to_move() == Color::White {
        Color::Black
    } else {
        Color::White
    }) as i32;
    let friendly_king_square = board.king(board.side_to_move()) as i32;

    let opponent_king_center_distance = center_manhattan_distance(opponent_king_square);

    let mut score = opponent_king_center_distance;

    let king_distance = manhattan_distance(opponent_king_square, friendly_king_square);
    score += 14 - king_distance;

    score -= distance_from_root as i32;

    score * 10
}

pub fn center_manhattan_distance(square: i32) -> i32 {
    let rank = square / 8;
    let file = square % 8;

    const CENTER: i32 = 3;

    (rank - CENTER).abs() + (file - CENTER).abs()
}

pub fn manhattan_distance(square1: i32, square2: i32) -> i32 {
    let rank1 = square1 / 8;
    let file1 = square1 % 8;

    let rank2 = square2 / 8;
    let file2 = square2 % 8;

    (rank1 - rank2).abs() + (file1 - file2).abs()
}
