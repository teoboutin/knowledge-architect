//! The `cargo` of the tests of `path@xtask@tests/gates_bin.rs`, which put it first on the path of the
//! run they start, under the name `cargo`. A program, not a shell script, since Windows runs no script
//! as a program.
//!
//! It prints one line per invocation, its arguments and the value of each variable a test reads, sleeps
//! briefly so the caller can act while gates are still running, and fails when `FAIL_ON` names a
//! substring of its arguments.

// Read so that cargo rebuilds this target when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

use std::process::ExitCode;

fn main() -> ExitCode {
    std::thread::sleep(std::time::Duration::from_millis(200));
    let args = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let var = |name: &str| std::env::var(name).unwrap_or_default();
    println!(
        "fake cargo: {args} bootstrap=[{}] ci=[{}] rustdocflags=[{}]",
        var("RUSTC_BOOTSTRAP"),
        var("KNOWLEDGE_ARCHITECT_CI"),
        var("CARGO_ENCODED_RUSTDOCFLAGS"),
    );
    let fail_on = var("FAIL_ON");
    if !fail_on.is_empty() && args.contains(&fail_on) {
        println!("induced failure");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
