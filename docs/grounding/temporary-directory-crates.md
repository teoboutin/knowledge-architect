# Crates that make a temporary directory, and what each removes

Findings read in the sources of crates, at the versions named, as crates.io served them on
2026-11-24. They ground `design@knowledge-architect@nothing-sweeps-a-killed-run` and the entries it
cites. Each is read again at the source it names: `https://static.crates.io/crates/<name>/<name>-<version>.crate`.
None was run on Windows.

## tempfile 3.27.0

- **No removal where the destructor does not run.** The crate documentation,
  `path@elsewhere@tempfile-3.27.0/src/lib.rs`, "Resource Leaking": "Destructors may fail to run if
  the process exits through an unhandled signal interrupt (like `SIGINT`), or if the instance is
  declared statically (like with [`lazy_static`]), among other possible reasons."

## mod-tempdir 1.2.0

- **The sweep takes the default names only.** `cleanup_orphans_in` of
  `path@elsewhere@mod-tempdir-1.2.0/src/cleanup.rs`, whose private `try_remove_one` reads a name that
  opens with `.tmp-` or `.tmpfile-` and a process id. The doc of `cleanup_orphans`: "Entries that do
  not match the crate's default prefix patterns, including any caller-supplied `with_prefix(...)`
  paths, are never touched."
- **It decides by process id.** `pid_alive` of `path@elsewhere@mod-tempdir-1.2.0/src/sys.rs` asks
  `/proc/<pid>` on Linux, `kill` with signal 0 on other Unix systems and `OpenProcess` on Windows. The
  same doc: "PIDs are reused by the operating system, so an orphan whose PID now belongs to an
  unrelated live process is kept until that process exits."
- **A caller calls it.** `cleanup_orphans` and `cleanup_orphans_in` are public functions, the first
  calling the second, that no other code of the crate calls, and their last argument is a minimum age in hours.

## testdir 0.10.0

- **It keeps by count.** `KEEP_DEFAULT` of `path@elsewhere@testdir-0.10.0/src/lib.rs` is 8, and
  `NumberedDirBuilder::count` of `path@elsewhere@testdir-0.10.0/src/builder.rs` takes a `NonZeroU8`. No type
  of the crate implements `Drop`: the older directories go when a new one is created,
  `remove_obsolete_dirs` of `path@elsewhere@testdir-0.10.0/src/numbered_dir.rs`.
- **Its default place is the cargo target directory**, `cargo_target_dir` and `init_testdir` of
  `path@elsewhere@testdir-0.10.0/src/private.rs`.
- **Its dependencies**, `path@elsewhere@testdir-0.10.0/Cargo.toml`, hold `cargo_metadata` and
  `sysinfo`.

## plumcraft-scratch 0.7.0

- **It belongs to another project**: the names are fixed, `ROOT_PREFIX` and `owner` of
  `path@elsewhere@plumcraft-scratch-0.7.0/src/lib.rs` taking those that open with `plumcraft-`.
- **Nothing is removed at the drop**: a process's own root stays until a later process sweeps it,
  `sweep`, by a process id asked of `kill -0`, and `alive` answers true where the platform is not
  Unix, so nothing is swept there.

## test-temp-dir 0.9.1

- **It wraps tempfile**: `TestTempDir` of `path@elsewhere@test-temp-dir-0.9.1/src/lib.rs` holds a
  `tempfile::TempDir`. It has no sweep.
