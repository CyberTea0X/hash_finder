use std::{thread::{available_parallelism, self}, sync::{mpsc, Mutex, Arc}};

use clap::Parser;
use hash_finder::{Cli, HashFindWorker};
use num_bigint::{ToBigUint};

fn main() {
    let config = Cli::parse();
    let available_cores = match config.cores {
        Some(c) => c,
        None => available_parallelism().unwrap().get(),
    };
    let step = match config.step {
        Some(s) => s,
        None => 10000,
    };
    let (tx, rx) = mpsc::channel();
    let current = Arc::new(Mutex::new(config.start.to_biguint().unwrap()));
    let end = "0".repeat(config.zeroes as usize);
    let mut remaining = config.count;
    if remaining == 0 {
        return
    }

    for _ in 0..available_cores {
        HashFindWorker::new(tx.clone(), end.clone(), current.clone(), step)
            .start()
    }
    for (num, hash) in rx {
        println!("{num}, \"{hash}\"");
        remaining -= 1;
        if remaining == 0 {
            return;
        }
    }
}
