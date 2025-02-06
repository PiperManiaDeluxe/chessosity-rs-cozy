use crate::search::mini_max::mini_max;
use crate::search::transposition_table::TranspositionTable;
use crate::uci::uci_loop::UciData;
use cozy_chess::{Color, Move};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;
use crate::uci::format_move::format_move;

pub fn set_go_timer(is_playing: &Arc<AtomicBool>, time: u64) {
    is_playing.store(true, std::sync::atomic::Ordering::SeqCst);

    let playing_clone = is_playing.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(time));
        playing_clone.store(false, std::sync::atomic::Ordering::SeqCst);
    });
}

pub fn do_uci_command_go(uci_data: &mut UciData, tokens: &Vec<String>) {
    let mut max_depth = 64;
    let mut time = 0;

    let mut white_time: u64 = 3 * 60 * 1000;
    let mut black_time: u64 = 3 * 60 * 1000;
    let mut white_inc: u64 = 2 * 1000;
    let mut black_inc: u64 = 2 * 1000;

    if tokens.len() > 1 {
        if tokens[1] == "infinite" {
            max_depth = 64;
            time = 0;
        } else if tokens[1] == "depth" {
            max_depth = tokens[1].parse::<u8>().unwrap();
            time = 0;
        } else {
            max_depth = 64;

            for i in 1..tokens.len() {
                match tokens[i].as_str() {
                    "wtime" => white_time = tokens[i + 1].parse::<u64>().unwrap(),
                    "btime" => black_time = tokens[i + 1].parse::<u64>().unwrap(),
                    "winc" => white_inc = tokens[i + 1].parse::<u64>().unwrap(),
                    "binc" => black_inc = tokens[i + 1].parse::<u64>().unwrap(),
                    _ => {}
                }
            }

            time = if uci_data.board.side_to_move() == Color::White {
                (white_time / 20) + (white_inc / 2)
            } else {
                (black_time / 20) + (black_inc / 2)
            };
        }
    }

    let mut current_depth = 1;
    let mut best_move: Option<Move> = None;
    let mut best_score: Option<i32> = None;
    let mut best_pv: Option<Vec<Move>> = None;
    let mut best_pv_string = String::new();
    let timer_handle: Option<std::thread::JoinHandle<()>> = None;

    if time > 0 {
        set_go_timer(&uci_data.is_playing, time);
    } else {
        uci_data
            .is_playing
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    let mut node_count = 0;

    let start = Instant::now();

    let mut tt = TranspositionTable::new();

    while current_depth <= max_depth
        && uci_data
            .is_playing
            .load(std::sync::atomic::Ordering::SeqCst)
    {
        let new_board = uci_data.board.clone();
        let (score, mv, early_stop, pv) = mini_max(
            &new_board,
            &mut tt,
            uci_data.current_move_history.clone(),
            current_depth,
            i32::MIN,
            i32::MAX,
            0,
            &uci_data.is_playing,
            &mut node_count,
        );

        if early_stop {
            break;
        }

        if let Some(mv) = mv {
            best_move = Some(mv);
            best_score = Some(score);
            best_pv = Some(pv);

            // Build the best pv string
            best_pv_string = String::new();
            if let Some(pv) = &best_pv {
                for mv in pv {
                    best_pv_string.push_str(&format!(" {}", format_move(mv, &uci_data.board)));
                }
            }
            best_pv_string = best_pv_string.trim().to_string();
        }

        let elapsed_ms = start.elapsed().as_millis();
        let nodes_per_s = if elapsed_ms > 0 {
            (node_count as f64 / elapsed_ms as f64) as u64 * 1000
        } else {
            0
        };

        println!(
            "info depth {} score cp {} nodes {} nps {} time {} bestmove {} pv {}",
            current_depth,
            best_score.unwrap(),
            node_count,
            nodes_per_s,
            elapsed_ms,
            format_move(&best_move.unwrap(), &uci_data.board),
            best_pv_string
        );

        current_depth += 1;
    }

    if let Some(mv) = best_move {
        println!("bestmove {}", format_move(&mv, &uci_data.board));
    }

    if let Some(timer_handle) = timer_handle {
        timer_handle.join().unwrap();
    }
}
