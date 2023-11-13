use std::{
    path::PathBuf,
    sync::{atomic::AtomicUsize, Arc, Mutex},
    thread,
    time::Instant,
};

use crate::play::play_all_games;

use crate::tileboard::read_tile_board_from_csv;

mod dice;
mod play;
mod tileboard;

const CSV_FILE_NAME: &str = "data/tiles.csv";
const GAMES_TO_PLAY: usize = 100_000;
const PLAYER_COUNT: usize = 3;
type TurnCount = u64;

fn main() {
    let start = Instant::now();
    let csv_file_path = PathBuf::from(CSV_FILE_NAME);
    let tile_board = Arc::new(read_tile_board_from_csv(&csv_file_path).unwrap());
    let cpus = num_cpus::get();
    let mut thread_handles = Vec::with_capacity(cpus);
    let games_played = Arc::new(AtomicUsize::new(0));
    let results = Arc::new(Mutex::new(Vec::<TurnCount>::with_capacity(GAMES_TO_PLAY)));
    for _ in 0..cpus {
        let thread_tile_board = tile_board.clone();
        let thread_games_played = games_played.clone();
        let thread_results = results.clone();
        thread_handles.push(thread::spawn(move || {
            play_all_games(
                &thread_tile_board,
                thread_games_played,
                GAMES_TO_PLAY,
                PLAYER_COUNT,
                thread_results,
            )
        }));
    }
    for handle in thread_handles {
        let _ = handle.join();
    }

    println!("Calculating results...");
    match results.lock() {
        Ok(mut results_guard) => print_results(&mut results_guard),
        Err(e) => {
            eprint!("Mutex poisoned! Thread must have panicked at some point");
            print_results(&mut e.into_inner())
        }
    };
    println!();
    println!("Finished in {} seconds", start.elapsed().as_secs_f64());
}

fn print_results(results: &mut [TurnCount]) {
    println!("INSTANCES: {}", results.len());
    println!("MEAN: {}", mean(results));
    results.sort_unstable();
    println!("MEDIAN: {}", median(results));
    println!("MIN: {}", results.get(0).unwrap());
    println!("MAX: {}", results.get(results.len() - 1).unwrap());
}

fn mean(results: &[TurnCount]) -> f64 {
    results.iter().sum::<TurnCount>() as f64 / results.len() as f64
}

fn median(sorted_results: &mut [TurnCount]) -> TurnCount {
    let index = sorted_results.len() / 2;
    sorted_results.get(index).unwrap().to_owned()
}
