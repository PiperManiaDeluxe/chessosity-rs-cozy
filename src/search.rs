use std::{
    collections::HashMap,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    time::Instant,
};

use cozy_chess::{Board, Color, GameStatus, Move};
use num_format::{Locale, ToFormattedString};

use crate::eval::eval;

pub struct MoveScore {
    pub score: i32,
    pub mv: Option<Move>,
}

#[derive(Copy, Clone)]
enum SearchBound {
    Exact,
    Lower,
    Upper,
}

#[derive(Copy, Clone)]
struct CacheEntry {
    pub val: i32,
    pub depth: u8,
    pub bound: SearchBound,
    pub chess_move: Option<Move>,
}

pub fn search_best_move(
    board: &Board,
    time_limit: u64,
    max_depth: u8,
    is_going: &Arc<AtomicBool>,
) -> MoveScore {
    let start = Instant::now();
    let mut depth = 1;
    let mut best_move = MoveScore { score: 0, mv: None };
    let mut nodes: u64 = 0;
    let mut transposition_table: HashMap<u64, CacheEntry> = HashMap::new();

    while Instant::now().duration_since(start).as_millis() < time_limit as u128
        && depth <= max_depth
        && is_going.load(SeqCst)
    {
        match minimax(
            board,
            depth,
            i32::MIN,
            i32::MAX,
            start,
            time_limit,
            &mut nodes,
            &mut transposition_table,
            is_going,
        ) {
            Ok(m) => best_move = m,
            Err(_) => break,
        }

        let elapsed_ms = start.elapsed().as_millis();
        let nodes_per_s = if elapsed_ms > 0 {
            (nodes as f64 / elapsed_ms as f64 * 1000.0) as u64
        } else {
            0
        };

        println!(
            "info depth {} score cp {} nodes {} nps {} time {} bestmove {}",
            depth,
            best_move.score,
            nodes,
            nodes_per_s,
            elapsed_ms,
            best_move.mv.unwrap_or_else(|| Move::from_str("a1a1").unwrap())
        );

        depth += 1;
    }

    best_move
}

fn minimax(
    board: &Board,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    start_time: Instant,
    time_limit: u64,
    nodes: &mut u64,
    transposition_table: &mut HashMap<u64, CacheEntry>,
    is_going: &Arc<AtomicBool>,
) -> Result<MoveScore, ()> {
    if start_time.elapsed().as_millis() > time_limit as u128 || !is_going.load(SeqCst) {
        return Err(());
    }

    *nodes += 1;

    let original_alpha = alpha;
    let original_beta = beta;

    if let Some(tt_entry) = transposition_table.get(&board.hash()) {
        if tt_entry.depth >= depth && depth > 0 {
            match tt_entry.bound {
                SearchBound::Exact => {
                    return Ok(MoveScore {
                        score: tt_entry.val,
                        mv: tt_entry.chess_move,
                    })
                }
                SearchBound::Lower => alpha = alpha.max(tt_entry.val),
                SearchBound::Upper => beta = beta.min(tt_entry.val),
            }
            if alpha >= beta {
                return Ok(MoveScore {
                    score: tt_entry.val,
                    mv: tt_entry.chess_move,
                });
            }
        }
    }

    if depth == 0 || board.status() != GameStatus::Ongoing {
        return Ok(MoveScore {
            score: eval(board),
            mv: None,
        });
    }

    let mut best_move = MoveScore {
        score: match board.side_to_move() {
            Color::White => i32::MIN,
            Color::Black => i32::MAX,
        },
        mv: None,
    };

    board.generate_moves(|moves| {
        for mv in moves {
            let mut new_board = board.clone();
            new_board.play(mv);
            let score = minimax(
                &new_board,
                depth - 1,
                alpha,
                beta,
                start_time,
                time_limit,
                nodes,
                transposition_table,
                is_going,
            );

            if let Err(()) = score {
                return true;
            }
            let score = score.unwrap().score;

            match board.side_to_move() {
                Color::White => {
                    if score > best_move.score {
                        best_move.score = score;
                        best_move.mv = Some(mv);
                    }
                    if score >= beta {
                        break;
                    }
                    alpha = alpha.max(score);
                }
                Color::Black => {
                    if score < best_move.score {
                        best_move.score = score;
                        best_move.mv = Some(mv);
                    }
                    if score <= alpha {
                        break;
                    }
                    beta = beta.min(score);
                }
            }
        }
        false
    });

    let bound = if best_move.score <= original_alpha {
        SearchBound::Upper
    } else if best_move.score >= original_beta {
        SearchBound::Lower
    } else {
        SearchBound::Exact
    };

    transposition_table.insert(
        board.hash(),
        CacheEntry {
            val: best_move.score,
            depth,
            bound,
            chess_move: best_move.mv,
        },
    );

    Ok(best_move)
}