use std::time::Instant;
use crate::perft::uci_perft;
use crate::uci::uci_loop::UciData;

pub fn do_uci_command_perft(uci_data: &UciData, tokens: &Vec<String>) {
    let depth = tokens[1].parse::<u8>().unwrap_or(5);
    let start = Instant::now();
    let nodes = uci_perft(&uci_data.board, depth);
    let elapsed = start.elapsed().as_millis();
    let nps = if elapsed > 0 {
        nodes * 1000 / elapsed as u64
    } else {
        0
    };
    println!("nodes: {nodes}, time: {elapsed}ms, nps: {nps}");
}