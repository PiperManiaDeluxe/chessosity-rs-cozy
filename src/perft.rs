use cozy_chess::Board;

// Testing / benchmark command (see: https://www.chessprogramming.org/Perft)
// Returns the number of leaf nodes in the game tree at a given depth.
pub fn uci_perft(board: &Board, depth: u8) -> u64{
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    board.generate_moves(|moves| {
        for mv in moves {
            let mut board = board.clone();
            board.play_unchecked(mv);
            nodes += uci_perft(&board, depth - 1);
        }
        false
    });

    nodes
}
