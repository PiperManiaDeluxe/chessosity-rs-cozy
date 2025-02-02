use crate::eval_count_material::eval_count_material;
use crate::perft::perft;
use crate::search::search_best_move;
use crate::time_manager::manage_time;
use cozy_chess::{Board, Color};
use num_format::{Locale, ToFormattedString};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::time::Instant;

pub fn do_uci_loop() {
    let mut board = Board::default();
    let mut go_thread: Option<std::thread::JoinHandle<()>> = None;
    let is_going = Arc::new(AtomicBool::new(false));

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        let tokens: Vec<&str> = command.split_whitespace().collect();

        match tokens[0] {
            "uci" => {
                println!("id name Chessosity");
                println!("id author Piper Mania Deluxe");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "ucinewgame" => {
                board = Board::default();
            }
            "perft" => {
                let depth = tokens.get(1).and_then(|s| s.parse().ok()).unwrap_or(5);
                let start = Instant::now();
                let nodes = perft(&board, depth);
                let elapsed_ms = start.elapsed().as_millis();
                println!(
                    "nodes: {}, time: {}ms, nps: {}",
                    nodes.to_formatted_string(&Locale::en),
                    elapsed_ms,
                    ((nodes as f32 / (elapsed_ms as f32 / 1000.0)) as u64)
                        .to_formatted_string(&Locale::en)
                );
            }
            "go" => {
                is_going.store(true, SeqCst);
                let new_board = board.clone();

                let mut white_time: u64 = 3 * 60 * 1000;
                let mut black_time: u64 = 3 * 60 * 1000;
                let mut white_inc: u64 = 2 * 1000;
                let mut black_inc: u64 = 2 * 1000;

                if tokens.len() > 1 {
                    for i in 1..tokens.len() {
                        let argument = tokens[i];
                        if argument == "wtime" {
                            white_time = tokens[i + 1].parse().unwrap();
                        } else if argument == "btime" {
                            black_time = tokens[i + 1].parse().unwrap();
                        } else if argument == "winc" {
                            white_inc = tokens[i + 1].parse().unwrap();
                        } else if argument == "binc" {
                            black_inc = tokens[i + 1].parse().unwrap();
                        }
                    }
                }

                let is_going_clone = Arc::clone(&is_going);
                go_thread = Some(std::thread::spawn(move || {
                    let time_limit: u64 = manage_time(
                        &new_board.clone(),
                        white_time,
                        black_time,
                        white_inc,
                        black_inc,
                    );

                    println!("info time limit for move: {}ms", time_limit);

                    let max_depth: u8 = 64;
                    let best_move =
                        search_best_move(&new_board, time_limit, max_depth, &is_going_clone);
                    println!("bestmove {}", best_move.mv.unwrap().to_string());
                }));
            }
            "position" => {
                if tokens[1] == "startpos" {
                    board = Board::default();

                    if tokens.len() > 2 && tokens[2] == "moves" {
                        for &mv in &tokens[3..] {
                            let mv_chess = cozy_chess::util::parse_uci_move(&board, mv).unwrap();
                            board.play(mv_chess);
                        }
                    }
                } else if tokens[1] == "fen" {
                    let fen: String = tokens[2..tokens.len()].join(" ");
                    board = Board::from_fen(&fen, false).unwrap();
                } else if tokens[1] == "moves" {
                    for &mv in &tokens[2..] {
                        let mv_chess = cozy_chess::util::parse_uci_move(&board, mv).unwrap();
                        board.play(mv_chess);
                    }
                }
            }
            "stop" => {
                is_going.store(false, SeqCst);
            }
            "fen" => {
                println!("{}", board.to_string());
            }
            "quit" => {
                break;
            }
            _ => {}
        }
    }
}
