use std::time::Instant;
use cozy_chess::Board;
use num_format::{Locale, ToFormattedString};
use crate::game::Game;
use crate::perft::perft;
use crate::time_manager::manage_time;

mod perft;
mod game;
mod eval;
mod eval_count_material;
mod time_manager;
mod eval_mate_in;

fn main(){
    let mut board = Board::default();
    let mut go_thread: Option<std::thread::JoinHandle<()>> = None;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        let tokens: Vec<&str> = command.split_whitespace().collect();

        match tokens[0]{
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
            "go" => {
                let mut white_time: u64 = 3 * 60 * 1000;
                let mut black_time: u64 = 3 * 60 * 1000;
                let mut white_inc: u64 = 2 * 1000;
                let mut black_inc: u64 = 2 * 1000;

                if tokens.len() > 1 {
                    if tokens[1] == "infinite" {
                        let new_board = board.clone();
                        go_thread = Some(std::thread::spawn(move || {
                            let mut game = Game::new_from_board(new_board, 64, 0);
                            game.go();
                        }));
                        continue;
                    }
                    if tokens[1] == "depth" {
                        let depth = tokens.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);
                        let new_board = board.clone();
                        go_thread = Some(std::thread::spawn(move || {
                            let mut game = Game::new_from_board(new_board, depth, 0);
                            game.go();
                        }));
                        continue;
                    }
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

                let new_board = board.clone();
                go_thread = Some(std::thread::spawn(move || {
                    let time = manage_time(&new_board, white_time, black_time, white_inc, black_inc);
                    let mut game = Game::new_from_board(new_board, 64, time);
                    game.go();
                }));
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
