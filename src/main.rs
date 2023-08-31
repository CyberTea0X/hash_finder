use std::thread::available_parallelism;

use hash_finder::{Config, ParseConfigError, show_syntax};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let config = match Config::parse(args.as_slice()) {
        Ok(c) => c,
        Err(ParseConfigError::NotEnoughArguments) => return show_syntax(),
        Err(e) => panic!("{}", e),
    };
    let available_cores = available_parallelism().unwrap().get();
}
