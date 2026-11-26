//! The run of `cargo metadata` under a variable `CARGO` that names no program. A test binary of its
//! own, since the test sets `CARGO` for its whole process, and a test running beside it in the same
//! process would run the Cargo it names.

use knowledge_architect_cargo_reader::{run_metadata, MetadataError};

/// Claim 3: the Cargo that runs is the one `CARGO` names, and a failure to start it names that
/// program, not `cargo`. Mutations checked, each caught: in `run_metadata`,
/// `std::env::var_os("CARGO")` made `None::<std::ffi::OsString>`, and the command's program
/// `cargo.to_string_lossy()` made `"cargo"`.
#[test]
fn the_cargo_named_by_the_variable_runs_and_a_failure_names_it() {
    let dir = tempfile::TempDir::with_prefix("klarch-cargo-reader-variable-")
        .expect("a scratch directory");
    let missing = dir.path().join("no-such-cargo");
    // The one test of this binary, so no other thread reads the environment while it changes.
    std::env::set_var("CARGO", &missing);
    let why = run_metadata(dir.path(), &["--offline"]).expect_err("no such program");
    let MetadataError::Spawn { command, .. } = &why else {
        panic!("not a failure to start: {why}");
    };
    assert_eq!(
        command,
        &format!(
            "{} metadata --format-version 1 --offline",
            missing.display()
        )
    );
}
