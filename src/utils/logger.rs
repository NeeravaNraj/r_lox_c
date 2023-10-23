use std::fmt::Display;

use crate::frontend::tokenization::span::Span;

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

    pub fn get_level_color(&self) -> String {
        match self {
            Self::Info => "\x1B[1m\x1B[38;2;70;190;255m",
            Self::Warn => "\x1B[1m\x1B[38;2;255;230;105m",
            Self::Error => "\x1B[1m\x1B[38;2;255;115;115m",
            Self::Fatal => "\x1B[1m\x1B[38;2;255;50;50m",
            Self::Debug => "\x1B[1m\x1B[38;2;165;140;255m",
        }
        .to_string()
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Info => "info\x1B[0m",
            Self::Warn => "warning\x1B[0m",
            Self::Error => "error\x1B[0m",
            Self::Fatal => "fatal\x1B[0m",
            Self::Debug => "debug\x1B[0m",
        };

        write!(f, "{}{s}", self.get_level_color())
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
    }

    fn get_file_line(file: &str, line: usize) -> Option<String> {
        let Ok(source) = std::fs::read_to_string(file) else {
            return None
        };
        source.lines().nth(line - 2).map(|s| String::from(s))
    }

    pub fn interpreter_log(level: Level, span: Span, message: String) {
        let message_decoration = "\x1B[1m\x1B[38;5;255m";
        // Initial message
        let mut base = format!("{level}: {message_decoration}{message}\x1B[0m\n");

        // src file path and line number
        base.push_str(
            format!(
                " \x1B[1m\x1B[38;5;012m-->\x1B[38;5;255m {}:{}:{}\x1B[0m\n",
                span.file,
                span.location.line - 1,
                span.location.start
            )
            .as_str(),
        );

        let Some(code) = Log::get_file_line(span.file, span.location.line as usize) else {
            base.push_str(
                format!(
                    "\x1B[1m\x1B[38;5;255mCould not read source for path `{}`.",
                    span.file
                ).as_str()
            );
            Log::print(level, base);
            return;
        };

        // source code line
        base.push_str(
            format!(
                "\x1B[1m\x1B[38;5;012m   |\n{:<3}|\x1B[0m {}\n",
                span.location.line - 1,
                code
            )
            .as_str(),
        );

        // pointers
        base.push_str(
            format!(
                "\x1B[1m\x1B[38;5;012m{:>3}| {}{}{}",
                " ",
                level.get_level_color(),
                String::from(" ").repeat(span.location.start - 1),
                String::from("^").repeat(span.location.end - span.location.start)
            )
            .as_str(),
        );

        Log::print(level, base);
    }

    fn print(level: Level, value: String) {
        if level.is_error() {
            eprintln!("{value}")
        } else {
            println!("{value}")
        }
        print!("\x1B[0m"); // reset
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($fmt:tt)+) => {
        {
            use crate::utils::logger::{Log, Level};
            Log::log($level, format!($($fmt)+));
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($fmt:tt)+) => {
        use crate::log;
        log!(Level::Info, $($fmt)+);
    };
}

#[macro_export]
macro_rules! warn {
    ($($fmt:tt)+) => {
        use crate::utils;
        log!(Level::Warn, $($fmt)+);
    };
}

#[macro_export]
macro_rules! error {
    ($($fmt:tt)+) => {
        {
            use crate::log;
            log!(Level::Error, $($fmt)+);
        }
    };
}

#[macro_export]
macro_rules! error_at {
    ($span:expr, $($fmt:tt)+) => {
        {
            use crate::utils::logger::{Log, Level};
            Log::interpreter_log(Level::Error, $span, format!($($fmt)+));
        }
    };
}

#[macro_export]
macro_rules! fatal {
    ($($fmt:tt)+) => {
        use crate::log;
        log!(Level::Fatal, $($fmt)+);
    };
}

#[macro_export]
macro_rules! debug {
    ($($fmt:tt)+) => {
        use crate::log;
        log!(Level::Debug, $($fmt)+);
    };
}
