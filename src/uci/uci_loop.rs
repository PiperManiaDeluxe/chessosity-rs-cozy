use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::AtomicBool;
use cozy_chess::Board;
use crate::search::transposition_table::TranspositionTable;
use crate::uci::uci_command_go::do_uci_command_go;
use crate::uci::uci_command_perft::do_uci_command_perft;
use crate::uci::uci_command_position::do_uci_command_position;
use crate::uci::uci_command_testeval::do_uci_command_testeval;
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

/// A shared structure that will eventually hold the initialized transposition table.
struct SharedTT {
    table: Mutex<Option<TranspositionTable>>,
    condvar: Condvar,
}

impl SharedTT {
    fn new() -> Self {
        SharedTT {
            table: Mutex::new(None),
            condvar: Condvar::new(),
        }
    }
}

pub fn do_uci_loop() {
    let mut uci_data = UciData::new();
    let mut go_thread: Option<std::thread::JoinHandle<()>> = None;

    // Create a shared transposition table wrapper that starts out empty.
    let shared_tt = Arc::new(SharedTT::new());

    // Spawn a thread that initializes the transposition table.
    {
        let shared_tt_clone = Arc::clone(&shared_tt);
        std::thread::spawn(move || {
            // Create the table (this may be an expensive operation).
            let table = TranspositionTable::new(134217728);
            // Lock the mutex and store the table.
            let mut lock = shared_tt_clone.table.lock().unwrap();
            *lock = Some(table);
            // Notify all threads waiting for the table.
            shared_tt_clone.condvar.notify_all();
        });
    }

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

        match tokens[0].as_str() {
            "uci" => {
                do_uci_command_uci();
            }
            "isready" => {
                // Wait until the transposition table is initialized.
                let mut lock = shared_tt.table.lock().unwrap();
                while lock.is_none() {
                    lock = shared_tt.condvar.wait(lock).unwrap();
                }
                println!("readyok");
            }
            "perft" => {
                do_uci_command_perft(&uci_data, &tokens);
            }
            "position" => {
                do_uci_command_position(&mut uci_data, &tokens);
            }
            "go" => {
                let mut new_uci_data = uci_data.clone();
                let tokens = tokens.clone();
                let shared_tt_clone = Arc::clone(&shared_tt);
                go_thread = Some(std::thread::spawn(move || {
                    // Make sure the table is ready before starting the search.
                    let mut lock = shared_tt_clone.table.lock().unwrap();
                    while lock.is_none() {
                        lock = shared_tt_clone.condvar.wait(lock).unwrap();
                    }
                    // Unwrap safely because the condition variable guarantees initialization.
                    let tt = lock.as_mut().unwrap();
                    do_uci_command_go(&mut new_uci_data, &tokens, tt);
                }));
            }
            "stop" => {
                uci_data
                    .is_playing
                    .store(false, std::sync::atomic::Ordering::SeqCst);
                if let Some(handle) = go_thread.take() {
                    handle.join().unwrap();
                }
            }
            "testeval" => {
                do_uci_command_testeval(&uci_data);
            }
            "ucinewgame" => {
                // Spawn a thread that initializes the transposition table.
                {
                    let mut lock = shared_tt.table.lock().unwrap();
                    *lock = None;
                    let shared_tt_clone = Arc::clone(&shared_tt);
                    std::thread::spawn(move || {
                        // Create the table (this may be an expensive operation).
                        let table = TranspositionTable::new(134217728);
                        // Lock the mutex and store the table.
                        let mut lock = shared_tt_clone.table.lock().unwrap();
                        *lock = Some(table);
                        // Notify all threads waiting for the table.
                        shared_tt_clone.condvar.notify_all();
                    });
                }

                // Wait for that thread to finish
                let mut lock = shared_tt.table.lock().unwrap();
                while lock.is_none() {
                    lock = shared_tt.condvar.wait(lock).unwrap();
                }

                uci_data.board = Board::default();
                uci_data.current_move_history = vec![uci_data.board.hash()];

                println!("readyok");
            }
            "quit" => std::process::exit(0),
            _ => {}
        }
    }
}
