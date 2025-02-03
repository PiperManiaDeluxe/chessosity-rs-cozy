use crate::eval::eval;
use crate::eval_count_material::get_piece_value;
use cozy_chess::Piece::Pawn;
use cozy_chess::{Board, Color, GameStatus, Move, Piece};
use num_format::Locale::se;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;
// A lot of code / concepts taken from https://github.com/TintifaxTheGreat/c4-e5-chess/blob/master/src/engine/game.rs

pub struct TTStore {
    pub depth: u8,
    pub score: i32,
    pub chess_move: Move,
}

pub struct Game {
    pub max_depth: u8,
    pub board: Board,
    pub move_time: u64,
    pub move_number: u64,
    pub node_count: u64,
    tt: HashMap<u64, TTStore>,
    pub playing: Arc<AtomicBool>,
}

impl Game {
    pub fn new(max_depth: u8, move_time: u64) -> Self {
        Game {
            max_depth,
            board: Board::default(),
            move_time,
            move_number: 0,
            node_count: 0,
            tt: HashMap::new(),
            playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn new_from_board(board: Board, max_depth: u8, move_time: u64) -> Self {
        Game {
            max_depth,
            board,
            move_time,
            move_number: 0,
            node_count: 0,
            tt: HashMap::new(),
            playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_timer(&mut self) -> JoinHandle<()> {
        self.playing
            .store(true, std::sync::atomic::Ordering::SeqCst);
        let playing_clone = self.playing.clone();
        let move_time = self.move_time;
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(move_time));
            playing_clone.store(false, std::sync::atomic::Ordering::SeqCst);
        })
    }

    pub fn go(&mut self) {
        let mut current_depth = 1;
        let mut best_move: Option<Move> = None;

        let timer_handle: Option<JoinHandle<()>> = None;

        if self.move_time > 0 {
            let timer_handle = self.set_timer();
        } else {
            self.playing
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }

        self.node_count = 0;

        let start = Instant::now();

        while current_depth <= self.max_depth
            && self.playing.load(std::sync::atomic::Ordering::SeqCst)
        {
            let new_board = self.board.clone();
            let (score, mv) = self.alpha_beta(&new_board, current_depth, i32::MIN, i32::MAX, 0);
            if let Some(mv) = mv {
                best_move = Some(mv);
            }

            let elapsed_ms = start.elapsed().as_millis();
            let nodes_per_s = if elapsed_ms > 0 {
                (self.node_count as f64 / elapsed_ms as f64 * 1000.0) as u64
            } else {
                0
            };

            println!(
                "info depth {} score cp {} nodes {} nps {} time {} bestmove {}",
                current_depth,
                score,
                self.node_count,
                nodes_per_s,
                elapsed_ms,
                best_move.unwrap()
            );

            current_depth += 1;
        }

        if let Some(mv) = best_move {
            println!("bestmove {}", mv);
        }

        if let Some(timer_handle) = timer_handle {
            timer_handle.join().unwrap();
        }
    }

    fn alpha_beta(
        &mut self,
        board: &Board,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        distance_from_root: u32,
    ) -> (i32, Option<Move>) {
        if !self.playing.load(std::sync::atomic::Ordering::SeqCst) {
            return (0, None);
        }

        let maximizing = board.side_to_move() == Color::White;

        let hash = board.hash();
        if let Some(entry) = self.tt.get(&hash) {
            if entry.depth >= depth {
                return (entry.score, Some(entry.chess_move));
            }
        }

        self.node_count += 1;

        if (board.status() != GameStatus::Ongoing) {
            // soon to use distance_from_root
            return (eval(&board, distance_from_root), None);
        }
        if depth == 0 {
            let score = self.quiescence(board, alpha, beta, distance_from_root);
            return (score, None);
        }

        let mut moves = Vec::new();
        board.generate_moves(|mvs| {
            moves.extend(mvs);
            false
        });

        let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
        let mut best_move: Option<Move> = None;

        for mv in moves {
            let mut new_board = board.clone();
            new_board.play(mv);

            let (score, _) =
                self.alpha_beta(&new_board, depth - 1, alpha, beta, distance_from_root + 1);

            if maximizing {
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
                if best_score >= beta {
                    break;
                }
                alpha = alpha.max(best_score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = Some(mv);
                }
                if best_score <= alpha {
                    break;
                }
                beta = beta.min(best_score);
            }
        }

        if let Some(mv) = best_move {
            self.tt.insert(
                hash,
                TTStore {
                    depth,
                    score: best_score,
                    chess_move: mv,
                },
            );
        }

        (best_score, best_move)
    }

    fn quiescence(
        &mut self,
        board: &Board,
        mut alpha: i32,
        mut beta: i32,
        distance_from_root: u32,
    ) -> i32 {
        self.node_count += 1;

        let stand_pat = eval(board, distance_from_root);
        let maximizing = board.side_to_move() == Color::White;

        // Check if we can prune immediately
        if maximizing {
            if stand_pat >= beta {
                return beta;
            }
            alpha = alpha.max(stand_pat);
        } else {
            if stand_pat <= alpha {
                return alpha;
            }
            beta = beta.min(stand_pat);
        }

        // Generate and sort capture moves
        let mut moves = Vec::new();
        board.generate_moves(|mvs| {
            // Only consider capture moves
            for mv in mvs {
                if board.piece_on(mv.to).is_some() {
                    moves.push(mv);
                }
            }
            false
        });

        // Sort moves by capture value (good move ordering improves pruning)
        moves.sort_by_cached_key(|mv| {
            let target = board.piece_on(mv.to).unwrap_or(Piece::Pawn);
            -get_piece_value(target) // Negative for descending sort
        });

        for mv in moves {
            let mut new_board = board.clone();
            new_board.play(mv);

            let score = self.quiescence(&new_board, alpha, beta, distance_from_root + 1);

            if maximizing {
                if score > alpha {
                    alpha = score;
                }
                if alpha >= beta {
                    break;
                }
            } else {
                if score < beta {
                    beta = score;
                }
                if beta <= alpha {
                    break;
                }
            }
        }

        if maximizing {
            alpha
        } else {
            beta
        }
    }
}
