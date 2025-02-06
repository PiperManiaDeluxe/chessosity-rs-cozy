use crate::eval::eval::eval;
use crate::uci::uci_loop::UciData;

pub fn do_uci_command_testeval(uci_data: &UciData){
    let score = eval(&uci_data.board, 0);
    let fen = uci_data.board.to_string();

    println!("info score cp {score} position fen {fen}");
}