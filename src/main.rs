use std::env;

/// Состояние чтения с консоли
enum ReadState {
    Reading,
    ReadingN,
    ReadingF,
}

fn show_syntax() {

}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut state = ReadState::Reading;
    let mut zeroes: Option<u32> = None;
    let mut count: Option<u32> = None;
    // Читаем консольные аргументы
    for arg in args.iter() {
        match state {
            ReadState::Reading => match arg.as_str() {
                "-N" => state = ReadState::ReadingN,
                "-F" => state = ReadState::ReadingF,
                _ => continue,
            },
            ReadState::ReadingN => {
                zeroes = arg.parse().ok();
                state = ReadState::Reading;
            }
            ReadState::ReadingF => {
                count = arg.parse().ok();
                state = ReadState::Reading;
            }
        }
    }
    // Валидация
    let zeroes = match zeroes {
        Some(z) => z,
        None => panic!("N is not specified")
    };

    let count = match count {
        Some(c) => c,
        None => panic!("F is not specified")
    };
    println!("{}, {}", zeroes, count);
}
