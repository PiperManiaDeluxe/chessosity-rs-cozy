// TODO: Add chess960 support

use cozy_chess::Board;
use std::time::Instant;

// Simple benchmark and move integrity test.
// Counts all possible moves / positions from a given board state after depth plys.
// Source: https://github.com/analog-hors/cozy-chess/blob/master/cozy-chess/examples/perft.rs
pub fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut nodes = 0;
        board.generate_moves(|moves| {
            for mv in moves {
                let mut board = board.clone();
                board.play_unchecked(mv);
                nodes += perft(&board, depth - 1);
            }
            false
        });
        nodes
    }
}

// Tests for this file
#[cfg(test)]
mod test {
    use super::*;
    use log::info;
    use num_format::Locale::fa;
    use num_format::{Locale, ToFormattedString};
    use test_log::test;

    // Tests perft on multiple different positions, data from https://www.chessprogramming.org/Perft_Results
    #[test]
    fn test_perft() {
        let test_start = Instant::now();
        info!("Perft test started");

        // region: Perft test cases
        const PERFT_TEST_CASES: &[(&str, &[(u8, u64)])] = &[
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Starting position
                &[
                    (0, 1),
                    (1, 20),
                    (2, 400),
                    (3, 8_902),
                    (4, 197_281),
                    (5, 4_865_609),
                    (6, 119_060_324),
                ],
            ),
            (
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                &[
                    (0, 1),
                    (1, 48),
                    (2, 2039),
                    (3, 97_862),
                    (4, 4_085_603),
                    (5, 193_690_690),
                ],
            ),
            (
                "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
                &[
                    (0, 1),
                    (1, 14),
                    (2, 191),
                    (3, 2_812),
                    (4, 43_238),
                    (5, 674_624),
                    (6, 11_030_083),
                ],
            ),
            (
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", // Position 4 (not mirrored)
                &[
                    (0, 1),
                    (1, 6),
                    (2, 264),
                    (3, 9_467),
                    (4, 422_333),
                    (5, 15_833_292),
                ],
            ),
            (
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1", // Position 4 mirrored
                &[
                    (0, 1),
                    (1, 6),
                    (2, 264),
                    (3, 9_467),
                    (4, 422_333),
                    (5, 15_833_292),
                ],
            ),
            (
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                &[
                    (0, 1),
                    (1, 44),
                    (2, 1_486),
                    (3, 62_379),
                    (4, 2_103_487),
                    (5, 89_941_194),
                ],
            ),
            (
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
                &[
                    (0, 1),
                    (1, 46),
                    (2, 2_079),
                    (3, 89_890),
                    (4, 3_894_594),
                    (5, 164_075_551),
                ],
            ),
        ];
        // endregion

        // Validate every test case to every depth
        for (case_fen, case_results) in PERFT_TEST_CASES {
            if let Ok(position) = Board::from_fen(case_fen, false) {
                for (depth, nodes) in *case_results {
                    let time_start = Instant::now();
                    let tested_nodes = perft(&position, *depth);
                    let time_sec = time_start.elapsed().as_secs_f64();
                    let nps = if time_sec > 0.0 {
                        (tested_nodes as f64 / time_sec) as u64
                    } else {
                        0
                    };
                    // Format nps as integer with commas
                    info!(
                    "FEN: {case_fen} Depth: {depth} Tested nodes: {tested_nodes} Expected nodes: {nodes} Time: {:?} NPS: {}",
                    time_start.elapsed(),
                    nps.to_formatted_string(&Locale::en)
                );
                    assert_eq!(*nodes, tested_nodes, "FEN: {case_fen} Depth: {depth}");
                }
            } else {
                panic!("Invalid FEN: {case_fen}");
            }
        }

        info!("Perft test done in {:?}", test_start.elapsed());
    }
}
