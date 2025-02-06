use crate::uci::uci_loop::UciData;
use cozy_chess::Board;

pub fn do_uci_command_position(uci_data: &mut UciData, tokens: &Vec<String>) {
    if tokens.len() < 2 {
        return;
    }

    const FEN_PARTS: usize = 6;

    match tokens[1].as_str() {
        "startpos" => {
            uci_data.board = Board::default();
            uci_data.current_move_history = vec![uci_data.board.hash()];

            if tokens.len() > 2 && tokens[2] == "moves" {
                for mv in &tokens[3..] {
                    let mv_chess = cozy_chess::util::parse_uci_move(&uci_data.board, &*mv).unwrap();
                    uci_data.board.play(mv_chess);
                    uci_data.current_move_history.push(uci_data.board.hash());
                }
            }
        }
        "fen" => {
            let fen = tokens[2..2 + FEN_PARTS].join(" ");
            uci_data.board = Board::from_fen(&fen, false).unwrap();
            uci_data.current_move_history = vec![uci_data.board.hash()];

            if tokens.len() > 2 + FEN_PARTS && tokens[2 + FEN_PARTS] == "moves" {
                for mv in &tokens[3 + FEN_PARTS..] {
                    let mv_chess = cozy_chess::util::parse_uci_move(&uci_data.board, &*mv).unwrap();
                    uci_data.board.play(mv_chess);
                    uci_data.current_move_history.push(uci_data.board.hash());
                }
            }
        }
        _ => {}
    }
}
