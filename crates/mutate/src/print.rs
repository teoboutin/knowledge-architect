//! The crate's output. Every line goes through these two macros, which ignore a write error: a
//! reader that closes the pipe truncates the output rather than panicking the run between the
//! mutation and the restore, where a panic would leave the target mutated.

/// Writes one line to stdout, as `println!` does, ignoring a write error.
macro_rules! say {
    ($($arg:tt)*) => {{
        use std::io::Write as _;
        let _ = writeln!(std::io::stdout(), $($arg)*);
    }};
}

/// Writes one line to stderr, as `eprintln!` does, ignoring a write error.
macro_rules! complain {
    ($($arg:tt)*) => {{
        use std::io::Write as _;
        let _ = writeln!(std::io::stderr(), $($arg)*);
    }};
}

pub(crate) use {complain, say};
