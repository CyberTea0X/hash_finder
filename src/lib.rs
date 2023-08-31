use std::{sync::{mpsc::{Sender, SendError}, Arc, Mutex}, thread};

use clap::Parser;
use num_bigint::BigUint;

#[derive(Parser)]
#[command(name = "hash_finder")]
#[command(author = "CyberTea0X <mihailusov_778@mail.ru>")]
#[command(version = "0.1.0")]
#[command(about = "hash_finding utility")]
pub struct Cli {
    /// The number of zeros that should be at the end of the hash
    #[arg(short = 'N', help = "The number of zeros that should be at the end of the hash")]
    pub zeroes: u32,

    /// Number of hashes to find
    #[arg(short = 'F', help = "Number of hashes to find")]
    pub count: u32,

    /// Number of threads to be used
    #[arg(short = 'C', help = "Number of threads to use. Default value depends on OC")]
    pub cores: Option<usize>,

    /// The number of hashes that a single thread processes
    #[arg(short = 'S', help = "Number of hashes processed in one step by one thread. defaults to 10000")]
    pub step: Option<u32>,

    /// The number from which the search for hashes starts
    #[arg(long="start", help = "The number from which the hash search starts", default_value_t=1)]
    pub start: u32,
}

/// A structure that contains everything needed for a parallel
/// search for hashes with the specified number of zeros at the end. 
pub struct HashFindWorker {
    tx: Sender<(BigUint, String)>,
    ends_with: String,
    current: Arc<Mutex<BigUint>>,
    step: u32,
}

impl HashFindWorker {
    pub fn new(tx: Sender<(BigUint, String)>, ends_with: String, current: Arc<Mutex<BigUint>>, step: u32) -> Self { Self { tx, ends_with, current, step } }


    /// Starts a hash search. Hash search will stop when destroyed
    /// channel that receives messages
    pub fn start(self) {
        thread::spawn(move || {
            self.work()
        });
    }

    /// Looks for step hashes. If it finds it, it sends the number and hash
    pub fn work(self) -> Result<(), SendError<(BigUint, String)>> {
        loop {
            let mut lock = self.current.lock().unwrap();
            let mut c = lock.clone();
            *lock += self.step;
            drop(lock);
            for _ in 0..self.step {
                let hash = sha256::digest(c.to_string());
                if hash.ends_with(self.ends_with.as_str()) {
                    self.tx.send((c.clone(), hash))?;
                }
                c += 1u32;
            }
        }
    }
}
