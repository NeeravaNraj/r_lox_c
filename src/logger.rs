use std::fmt::Display;

pub enum Level {
    Info,
    Warn,
    Error,
    Fatal,
    Debug,
}

impl Level {
    pub fn is_error(&self) -> bool {
        matches!(self, Level::Error | Level::Fatal)
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Info => "\x1B[1m\x1B[38;2;70;190;255minfo\x1B[0m",
            Self::Warn => "\x1B[1m\x1B[38;2;255;230;105mwarning\x1B[0m",
            Self::Error => "\x1B[1m\x1B[38;2;255;115;115merror\x1B[0m",
            Self::Fatal => "\x1B[1m\x1B[38;2;255;50;50mfatal\x1B[0m",
            Self::Debug => "\x1B[1m\x1B[38;2;165;140;255mdebug\x1B[0m"
        };

        write!(f, "{s}")
    }
}

pub struct Log;

impl Log {
    pub fn log(level: Level, message: String) {
        let message_decoration = "\x1B[1m\x1B[38;5;255m";
        if level.is_error() {
            eprintln!("{level}: {message_decoration}{message}");
        } else {
            println!("{level}: {message_decoration}{message}");
        }
        print!("\x1B[0m"); // reset
    }
}

#[macro_export]
macro_rules! info {
    ($($fmt:tt)+) => {
        {
            use crate::logger::{Log, Level};
            Log::log(Level::Info, format!($($fmt)+));
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($fmt:tt)+) => {
        {
            use crate::logger::{Log, Level};
            Log::log(Level::Warn, format!($($fmt)+));
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($fmt:tt)+) => {
        {
            use crate::logger::{Log, Level};
            Log::log(Level::Error, format!($($fmt)+));
        }
    };
}

#[macro_export]
macro_rules! fatal {
    ($($fmt:tt)+) => {
        {
            use crate::logger::{Log, Level};
            Log::log(Level::Fatal, format!($($fmt)+));
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($fmt:tt)+) => {
        {
            use crate::logger::{Log, Level};
            Log::log(Level::Debug, format!($($fmt)+));
        }
    };
}
