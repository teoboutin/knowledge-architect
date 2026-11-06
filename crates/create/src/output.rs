//! `outln!` over the checker's writer, so a stdout closed early ends this crate's commands as it ends
//! every other, per `design@cli@exit-code-ladder`. This crate depends on `check`, so it holds no copy
//! of the writer.

macro_rules! outln {
    () => { $crate::check::commands::output::write(format_args!("\n")) };
    ($($arg:tt)*) => { $crate::check::commands::output::write(format_args!("{}\n", format_args!($($arg)*))) };
}

pub(crate) use outln;
