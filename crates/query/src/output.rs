//! Every write to stdout goes through `outln!`, so a reader that closes the pipe early ends the run
//! quietly instead of with a panic, per `design@cli@exit-code-ladder`.
//!
//! A copy of the writer of `check`, since `query` may depend on no crate of the checks, and no crate below this one is a
//! home for printing; `issue@knowledge-architect@a-typed-command-prints-and-exits-rather-than-returning-data` removes
//! the copies with the last command that returns data.
//!
//! Rust ignores `SIGPIPE` before `main`, so a write to a closed pipe returns
//! `ErrorKind::BrokenPipe`, and the standard `print!` macros turn that into a panic. A closed stdout
//! means the run could not deliver its output, which is the ladder's exit 2; it is never a verdict.
//! Nothing is printed to stderr for it: the reader closed the pipe on purpose. The `SIGPIPE` default
//! disposition is not restored instead, because that would also kill the process on a write to a
//! git child that exited early, where the `git` crate reads the error and reports git's own message.

use std::io::{ErrorKind, Write};

/// Writes formatted output to stdout, ending the process with exit 2 on a broken pipe.
pub(crate) fn write(args: std::fmt::Arguments<'_>) {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    if let Err(e) = lock.write_fmt(args) {
        if e.kind() != ErrorKind::BrokenPipe {
            eprintln!("error: cannot write to stdout: {e}");
        }
        std::process::exit(2);
    }
}

macro_rules! outln {
    () => { $crate::output::write(format_args!("\n")) };
    ($($arg:tt)*) => { $crate::output::write(format_args!("{}\n", format_args!($($arg)*))) };
}

pub(crate) use outln;
