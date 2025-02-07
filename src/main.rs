use cozy_chess::Board;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use threadpool::ThreadPool;

fn perft_seq(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    board.generate_moves(|moves| {
        for mv in moves {
            let mut board = board.clone();
            board.play_unchecked(mv);
            nodes += perft_seq(&board, depth - 1);
        }
        false
    });

    nodes
}

fn perft_para(board: &Board, depth: u8, max_threads: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let mut handles: Vec<JoinHandle<u64>> = vec![];

    board.generate_moves(|moves| {
        for mv in moves {
            // if we reached the limit for threads, wait for one to join
            while handles.len() >= max_threads {
                if let Some(handle) = handles.pop() {
                    nodes += handle.join().unwrap();
                }
            }

            let mut handle_board = board.clone();
            let handle: JoinHandle<u64> = thread::spawn(move || {
                handle_board.play_unchecked(mv);
                return perft_seq(&handle_board, depth - 1);
            });
            handles.push(handle);
        }
        false
    });

    for handle in handles {
        nodes += handle.join().unwrap();
    }

    nodes
}

fn perft_para_pool(board: &Board, depth: u8, pool: Arc<ThreadPool>) -> u64 {
    if depth == 0 {
        return 1;
    }

    // If the pool is full, we cannot add more workers. do perft normally
    if pool.queued_count() >= pool.max_count() {
        return perft_seq(&board, depth);
    }

    // We can add more workers!
    let (tx, rx) = channel();

    board.generate_moves(|moves| {
        for mv in moves {
            let tx = tx.clone();
            let mut handle_board = board.clone();
            let pool_clone = Arc::clone(&pool);
            pool.execute(move || {
                handle_board.play_unchecked(mv);
                let count = perft_para_pool(&handle_board, depth - 1, pool_clone);
                tx.send(count).expect("Failed to send result");
            })
        }
        false
    });

    drop(tx);

    let nodes: u64 = rx.iter().sum();
    nodes
}

fn main() {
    let mut board = Board::default();

    println!("perft_seq:");
    let start = Instant::now();
    let nodes = perft_seq(&board, 5);
    let elapsed = start.elapsed().as_millis();
    let nps = if elapsed > 0 {
        nodes * 1000 / elapsed as u64
    } else {
        0
    };
    println!("nodes: {nodes}, time: {elapsed}ms, nps: {nps}");

    println!("perft_para:");
    let start = Instant::now();
    let nodes = perft_para(&board, 5, 8);
    let elapsed = start.elapsed().as_millis();
    let nps = if elapsed > 0 {
        nodes * 1000 / elapsed as u64
    } else {
        0
    };
    println!("nodes: {nodes}, time: {elapsed}ms, nps: {nps}");

    println!("perft_para_pool:");
    let start = Instant::now();
    let nodes = perft_para_pool(&board, 5, Arc::new(ThreadPool::new(8)));
    let elapsed = start.elapsed().as_millis();
    let nps = if elapsed > 0 {
        nodes * 1000 / elapsed as u64
    } else {
        0
    };
    println!("nodes: {nodes}, time: {elapsed}ms, nps: {nps}");
}
