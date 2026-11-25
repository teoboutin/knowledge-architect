---
kind: defect
---
# The liveness of a run is read wrongly outside Linux

## Summary

Outside Linux, `restore` reads whether the run that owns the record still goes on by running
`kill -0` with the pid the record holds. On macOS the pid of another user's process reads as gone,
and pid 0 as alive. On Windows pid 0 reads as alive, and whether a live run's pid reads as alive is not
established: where it reads as gone, `restore` writes over the file of a run that is still going.

## Details

### What

The function `process_is_alive` of `path@mutate@src/harness.rs`, called by `owner_is_alive` of the
same file, asks `/proc` on Linux and runs `kill -0 <pid>` elsewhere, and reads any failure of the
program as "gone", as its doc comment says.

Observed on GitHub's hosted runners, on main's code with rough repairs, under which it compiles on
Windows and names a path to git with slashes, and with every file checked out as committed, by
`cargo test --locked --workspace --no-fail-fast`:

- on macOS and on Windows, three tests fail: `a_record_this_process_claimed_does_not_read_as_owned_by_a_live_run`
  of `path@mutate@tests/harness.rs`, which writes pid 1 and panics with "pid 1 is always running", and
  `restore_refuses_while_the_owning_run_is_alive` and `a_record_over_an_unmutated_file_is_cleared_by_restore`
  of `path@mutate@tests/modes.rs`;
- on Windows, with a Git for Windows `kill` on the search path, the last of the three, which writes
  pid 0, prints "is still going".

Forced on Linux, with the condition on Linux replaced by `false`, the same three fail. The causes
read: `kill -0 1` as a user who is not root fails with a permission error, and `kill -0 0` signals the
caller's own group, which is alive. On Windows the `kill` of Git for Windows takes an MSYS pid, not the
Windows pid that `std::process::id` gives (assumption, from the MSYS pid space; not run). No live run's
pid was asked there: the tests write pid 1, which Windows has not, and pid 0.

Ruled out: nothing else of the three tests' paths differs by system, since each passes on Linux.

A lead: the file lock that `issue@mutate@a-reused-pid-reads-as-a-live-run` names replaces the pid, on
the three systems if the lock behaves as it does on Linux, which is not established.

### Why it matters

`restore` over a run that is still going un-mutates its file underneath it, and a live run read from
another user's pid is cleared the same way. It strains `design@mutate@discard-is-the-callers-word`
and `design@mutate@restore-by-comparison` only in that the refusal guarding a live run cannot tell one.
Development on Windows is left for later by `spec@plans@gates-on-windows-and-macos`; on macOS the hand
mutation is to work, its tests included.

### What would close it

A liveness check that gives the right answer on Linux, Windows and macOS, by the lock or another
means, with tests that hold a live owner of the test's own on each.
