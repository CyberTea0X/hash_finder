use std::{env, error::Error, fmt::Display};

/// Состояние чтения с консоли
pub enum ReadState {
    Reading,
    ReadingN,
    ReadingF,
}

#[derive(Debug, Clone)]
pub struct Config {
    zeroes: u32,
    count: u32,
}

impl Config {
    pub fn new(zeroes: u32, count: u32) -> Self { Self { zeroes, count } }

    /// Парсит конфиг программы из строк обозначающих окружение
    pub fn parse(args: &[String]) -> Result<Config, ParseConfigError> {
        let mut state = ReadState::Reading;
        let mut zeroes: Option<u32> = None;
        let mut count: Option<u32> = None;
        if args.len() == 1 {
            return Err(ParseConfigError::NotEnoughArguments);
        }
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
        let zeroes = zeroes.ok_or(ParseConfigError::ParseN)?;
        let count = count.ok_or(ParseConfigError::ParseF)?;

        Ok(Config::new(zeroes, count))
    }
}

#[derive(Debug, Clone)]
pub enum ParseConfigError {
    ParseN,
    ParseF,
    NotEnoughArguments,
}

impl Display for ParseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseConfigError::ParseN => write!(f, "N is not specified"),
            ParseConfigError::ParseF => write!(f, "F is not specified"),
            ParseConfigError::NotEnoughArguments => write!(f, "No arguments were specified"),
        }
    }
}

/// Показывает синтаксис команды
pub fn show_syntax() {
    println!("\
NAME
hash_finder - tool for finding sha256 hashes

SYNOPSIS
hash_finder -N COUNT_OF_ZEROS -F COUNT_OF_HASHES

DESCRIPTION
hash finder is a console application for finding sha256 hashes that end in
to the specified number of zeros.
             ");
}

