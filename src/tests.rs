use std::sync::{mpsc, Arc, Mutex};

use hash_finder::{Cli, HashFindWorker};
#[cfg(test)]
use num_bigint::BigUint;
use num_bigint::ToBigUint;

/// Finds hashes according to config
pub fn hash_finder(c: &Cli) -> Vec<(BigUint, String)> {
    let (tx, rx) = mpsc::channel();
    let current = Arc::new(Mutex::new(0u32.to_biguint().unwrap()));
    let end = "0".repeat(c.zeroes as usize);
    let mut remaining = c.count;

    for _ in 0..c.cores.unwrap() {
        HashFindWorker::new(tx.clone(), end.clone(), current.clone(), c.step.unwrap()).start()
    }
    let mut hashes = Vec::new();
    for (num, hash) in rx {
        hashes.push((num, hash));
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    return hashes;
}

/// Checks hashes for validity according to config
pub fn hashes_is_valid(hashes: &[(BigUint, String)], c: &Cli) -> bool {
    let must_end_with = "0".repeat(c.zeroes as usize);
    if hashes.len() != c.count as usize {
        return false;
    }
    for hash in hashes {
        if !hash.1.ends_with(&must_end_with) {
            return false;
        }
        if hash.1 != sha256::digest(hash.0.to_string()) {
            return false;
        }
    }
    return true;
}

#[test]
pub fn one_hash() {
    let config = Cli {
        zeroes: 3,
        count: 1,
        cores: Some(1),
        step: Some(1000),
        start: 1,
    };
    let hashes = hash_finder(&config);
    println!("one_hash:\n{:#?}", hashes);
    assert!(hashes_is_valid(hashes.as_slice(), &config))
}

#[test]
pub fn multi_threading() {
    let config = Cli {
        zeroes: 3,
        count: 1,
        cores: Some(16),
        step: Some(1000),
        start: 1,
    };
    let hashes = hash_finder(&config);
    println!("multi_threading:\n{:#?}", hashes);
    assert!(hashes_is_valid(hashes.as_slice(), &config))
}

#[test]
pub fn zero_zeroes() {
    let config = Cli {
        zeroes: 0,
        count: 1,
        cores: Some(16),
        step: Some(10000),
        start: 1,
    };
    let hashes = hash_finder(&config);
    println!("zero_zeroes:\n{:#?}", hashes);
    assert!(hashes_is_valid(hashes.as_slice(), &config))
}

#[test]
pub fn ten_hashes() {
    let config = Cli {
        zeroes: 3,
        count: 10,
        cores: Some(16),
        step: Some(10000),
        start: 1,
    };
    let hashes = hash_finder(&config);
    println!("ten_hashes:\n{:#?}", hashes);
    assert!(hashes_is_valid(hashes.as_slice(), &config))
}

#[test]
pub fn start_1000() {
    let config = Cli {
        zeroes: 3,
        count: 10,
        cores: Some(16),
        step: Some(10000),
        start: 1000,
    };
    let hashes = hash_finder(&config);
    println!("ten_hashes:\n{:#?}", hashes);
    assert!(hashes_is_valid(hashes.as_slice(), &config))
}
