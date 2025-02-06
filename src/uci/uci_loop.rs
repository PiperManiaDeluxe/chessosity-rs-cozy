use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use cozy_chess::Board;
use crate::uci::uci_command_go::do_uci_command_go;
use crate::uci::uci_command_isready::do_uci_command_isready;
use crate::uci::uci_command_perft::do_uci_command_perft;
use crate::uci::uci_command_position::do_uci_command_position;
use crate::uci::uci_command_uci::do_uci_command_uci;

#[derive(Clone)]
pub struct UciData {
    pub board: Board,
    pub current_move_history: Vec<u64>,
    pub is_playing: Arc<AtomicBool>,
}

impl UciData {
    pub fn new() -> Self {
        UciData {
            board: Board::default(),
            current_move_history: vec![Board::default().hash()],
            is_playing: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub fn do_uci_loop(){
    let mut uci_data = UciData::new();
    let mut go_thread: Option<std::thread::JoinHandle<()>> = None;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Create an owned, trimmed version of the input.
        let trimmed = input.trim();

        // Convert each token into an owned String.
        let tokens: Vec<String> = trimmed
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // If there are no tokens, continue to the next iteration.
        if tokens.is_empty() {
            continue;
        }

        // Match on the first token.
        match tokens[0].as_str() {
            "uci" => do_uci_command_uci(),
            "isready" => do_uci_command_isready(),
            "perft" => do_uci_command_perft(&uci_data, &tokens),
            "position" => do_uci_command_position(&mut uci_data, &tokens),
            "go" => {
                let mut new_uci_data = uci_data.clone();
                // Clone the tokens vector to move it into the thread.
                let tokens = tokens.clone();
                go_thread = Some(std::thread::spawn(move || {
                    do_uci_command_go(&mut new_uci_data, &tokens);
                }));
            }
            "stop" => {
                uci_data.is_playing.store(false, std::sync::atomic::Ordering::SeqCst);
                if let Some(handle) = go_thread.take() {
                    handle.join().unwrap();
                }
            }
            "quit" => break,
            _ => {}
        }
    }
}

