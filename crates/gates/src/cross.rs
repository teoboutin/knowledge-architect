//! Clippy for another system than the machine's, as a gate a project adds to its list.
//!
//! A module of its own: its ground is how the `cc` crate and clippy behave for a target the machine
//! cannot link, which nothing else of the gates rests on.

use crate::gates::{Distiller, Gate};

/// The program named as the compiler and the archiver of the target: it succeeds and makes nothing.
const STUB: &str = "true";

/// The gate that runs clippy, warnings denied, for `target`, a target triple such as
/// `x86_64-pc-windows-gnu`, over the whole workspace and every cargo target of it. It answers `None`
/// on a machine whose own system is the target's, where the recommended list's `clippy` judges the
/// same code. A project that wants it adds it to its list:
///
/// ```
/// use knowledge_architect_gates::{cross_clippy, rust_project, Checker};
///
/// let checker = Checker { package: "xtask", prefix: &["klarch"] };
/// let mut gates = rust_project(checker, "origin/main");
/// gates.extend(cross_clippy("x86_64-pc-windows-gnu"));
/// ```
///
/// The gate's name is `clippy-<target>`. The toolchain that runs it must hold the target's standard
/// library: `rustup target add <target>`, or the `targets` of the project's `rust-toolchain.toml`.
///
/// No cross compiler is needed: clippy links nothing, so the gate's `envs` name the program `true`,
/// which must be on the path, as the target's C compiler, C++ compiler and archiver, in each spelling
/// the `cc` crate reads, and empty `RUSTC_WRAPPER`, since a wrapper such as `sccache` can refuse it. A
/// build script that compiles C then succeeds with nothing compiled; one that looks for a library or
/// runs the compiler to choose a feature can fail, or be answered wrongly. A project with a real cross
/// compiler removes those variables from the gate's `envs`. An `-msvc` target fails under the stub.
pub fn cross_clippy(target: &str) -> Option<Gate> {
    gate_on(target, std::env::consts::OS)
}

/// [`cross_clippy`] on a machine of the system `machine`, spelled as `std::env::consts::OS`.
fn gate_on(target: &str, machine: &str) -> Option<Gate> {
    if system(target) == Some(machine) {
        return None;
    }
    let args = [
        "clippy",
        "--workspace",
        "--all-targets",
        "--target",
        target,
        "--",
        "-D",
        "warnings",
    ];
    // The `cc` crate reads a variable spelled with the target's hyphens before the one spelled with
    // underscores, so a gate that set the second alone would give way to a compiler the caller
    // exported under the first.
    let spellings = [target.to_owned(), target.replace('-', "_")];
    let mut envs: Vec<(String, String)> = ["CC", "CXX", "AR"]
        .iter()
        .flat_map(|tool| spellings.iter().map(move |t| format!("{tool}_{t}")))
        .map(|name| (name, STUB.to_owned()))
        .collect();
    envs.push(("RUSTC_WRAPPER".to_owned(), String::new()));
    Some(Gate {
        name: format!("clippy-{target}"),
        program: "cargo".to_owned(),
        args: args.iter().map(|a| (*a).to_owned()).collect(),
        envs,
        resolves: true,
        rebased_only: false,
        distiller: Distiller::WithoutProgress,
    })
}

/// The system of a target triple, spelled as `std::env::consts::OS`, where it is one of the three a
/// machine of a project commonly is. Any other target, Android's two environments of Linux among
/// them, has none, so no machine's own clippy stands for it.
fn system(target: &str) -> Option<&'static str> {
    let segments: Vec<&str> = target.split('-').collect();
    if segments.contains(&"windows") {
        Some("windows")
    } else if segments.contains(&"darwin") {
        Some("macos")
    } else {
        match segments[..] {
            [.., "linux", env] if !matches!(env, "android" | "androideabi") => Some("linux"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The claim: the gate is named for its target, runs clippy for it with warnings denied over every
    /// cargo target, resolves dependencies, and sets the compilers and the archiver in both spellings
    /// the `cc` crate reads, and no wrapper. Mutations checked: `AR` dropped; the spelling with
    /// hyphens dropped, and the one with underscores; `RUSTC_WRAPPER` dropped; `-D warnings` dropped;
    /// each caught.
    #[test]
    fn the_gate_runs_clippy_for_the_target_under_the_stub() {
        let gate = gate_on("x86_64-pc-windows-gnu", "linux").expect("a gate");
        assert_eq!(gate.name, "clippy-x86_64-pc-windows-gnu");
        assert_eq!(gate.program, "cargo");
        assert_eq!(
            gate.args,
            [
                "clippy",
                "--workspace",
                "--all-targets",
                "--target",
                "x86_64-pc-windows-gnu",
                "--",
                "-D",
                "warnings"
            ]
        );
        let envs: Vec<(&str, &str)> = gate
            .envs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(
            envs,
            [
                ("CC_x86_64-pc-windows-gnu", "true"),
                ("CC_x86_64_pc_windows_gnu", "true"),
                ("CXX_x86_64-pc-windows-gnu", "true"),
                ("CXX_x86_64_pc_windows_gnu", "true"),
                ("AR_x86_64-pc-windows-gnu", "true"),
                ("AR_x86_64_pc_windows_gnu", "true"),
                ("RUSTC_WRAPPER", ""),
            ]
        );
        assert!(gate.resolves && !gate.rebased_only);
        assert_eq!(gate.distiller, Distiller::WithoutProgress);
    }

    /// The claim: the gate is absent exactly on a machine of the target's own system, and present on
    /// every machine for a target of no such system. Mutation checked: the exclusion of `android`
    /// removed; caught.
    #[test]
    fn the_gate_is_absent_on_a_machine_of_the_target_s_system_alone() {
        for (target, own) in [
            ("x86_64-pc-windows-gnu", Some("windows")),
            ("aarch64-pc-windows-gnullvm", Some("windows")),
            ("aarch64-apple-darwin", Some("macos")),
            ("x86_64-unknown-linux-gnu", Some("linux")),
            ("aarch64-unknown-linux-musl", Some("linux")),
            ("aarch64-linux-android", None),
            ("armv7-linux-androideabi", None),
            ("wasm32-unknown-unknown", None),
            ("aarch64-apple-ios", None),
        ] {
            for machine in ["linux", "windows", "macos", "freebsd"] {
                assert_eq!(
                    gate_on(target, machine).is_none(),
                    own == Some(machine),
                    "{target} on {machine}"
                );
            }
        }
    }
}
