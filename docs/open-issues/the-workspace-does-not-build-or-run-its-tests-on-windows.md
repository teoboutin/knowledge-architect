---
kind: todo
---
# The workspace does not build or run its tests on Windows

## Summary

The tests of six crates use the Unix API of the standard library with no `cfg` that keeps it to
Unix, so they do not compile for Windows. Other tests make files under names Windows refuses, or
start programs Windows does not ship, and one function of `path@mutate@src/harness.rs` answers
wrongly there. CI runs on Linux alone, so nothing reports any of it.

## Details

### What

Nothing here was run on Windows: every point is read from the code, on Linux, and whether the
libraries compile for Windows is not established. The tests and the functions are named in prose, per
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`. In the non-test code of `gates` and `git`, the
uses of `std::os::unix` sit behind `cfg(unix)`, with another arm for the other platforms.

- **The Unix API with no `cfg`.** `git grep -o 'std::os::unix::fs::symlink' -- '*.rs' | wc -l` prints
  37. 27 of them sit in a test function with no `cfg` above the use or on the function, counted by a
  script over each use: in `path@check@src/build_origin.rs`, `path@check@src/commands/mod.rs`,
  `path@cli@tests/init.rs`, `path@git@src/git.rs`, `path@mutate@tests/harness.rs`,
  `path@mutate@tests/modes.rs`, `path@testing@tests/binary.rs` and `path@transcript@src/reader.rs`.
  Three tests with no `cfg` use `PermissionsExt` of the same module:
  `a_failure_after_the_first_write_exits_one` of `path@cli@tests/init.rs`,
  `a_refusal_before_the_write_leaves_no_record` of `path@mutate@tests/modes.rs` and
  `check_staged_as_a_pre_commit_hook_judges_the_index_the_commit_records` of
  `path@testing@tests/binary.rs`.
- **Names Windows refuses**, as `design@model@unportable-names-refused` lists them:
  `a_symlink_with_a_refused_name_is_one_finding` of `path@testing@tests/binary.rs` makes a link named
  `a:b`, and three tests with no `cfg` make a file whose name holds a line feed:
  `the_untracked_note_covers_a_phase_two_stop_and_escapes_the_name` and
  `a_name_holding_a_newline_is_one_finding_on_one_line_and_its_contents_are_read_by_nothing` of the same
  file, and `a_newline_in_a_tracked_name_does_not_shift_the_answers_after_it` of `path@git@src/git.rs`.
- **Programs Windows does not ship**, none of the tests under a `cfg`: tests of
  `path@mutate@tests/modes.rs` run `sh`, `true`, `false`, `grep` and `mkfifo`, and write a `#!/bin/sh`
  program; the test of `path@cli@tests/init.rs` that calls `mkfifo`; three tests of
  `path@gates@src/process.rs` run `sh` and `true`; and `path@testing@tests/binary.rs` writes `#!/bin/sh`
  hooks and an editor. Git for Windows ships a `sh`; whether
  these tests find it there is not established.
- **The liveness of a run**: `process_is_alive` of `path@mutate@src/harness.rs` runs `kill -0` outside
  Linux. Where no `kill` program answers, a live run reads as gone, which `path@mutate@README.md`
  states, so `restore` can write over the file of a run that is still going.
- **A directory may not be removable there** while a file in it is open, which the `tempfile` guards
  of the tests do not report. Not established for any test.

### Why it matters

`goal@every-target-project-can-adopt-it` and `goal@serves-one-kind-of-project` exclude no platform in
their conditions, so a target project developed on Windows is served; the rival reading, that a
platform is one more thing knowledge-architect may ask of a project, finds no such ask in their text.
The owner reported a consumer project that the owner wants to start building on Windows soon. Where a
project takes the shape that `skill@knowledge-architect-setup@rust-project` offers, its workspace links
`gates` and `mutate` into its maintenance crate, and no run on Windows shows that they build or behave
there.

### What would close it

A CI job on Windows that builds the workspace and runs its tests, passing, with each use above
guarded, rewritten or repaired, and each test skipped there named with its reason.
