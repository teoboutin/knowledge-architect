---
kind: defect
---
# The hand mutation refuses, misreads or mutates wrongly in several ways on Windows

## Summary

Development on Windows meets the hand mutation in ways of its own: how `--file` is spelled, which
program a command names, the line endings of a file, the case of `.git`, the printed reverse, and what
Git Bash makes of an argument. Two of them can give a wrong mutant or a wrong refusal, and most were read
in the code and not run. Two tests are skipped on Windows for this entry.

## Details

### What

Each is of `path@mutate@src/`, read in the code unless it says otherwise:

- `target_is_mutable` of `path@mutate@src/harness.rs` takes the path as given. On Windows a `--file`
  spelled with `\` is one the patch's `a/` and `b/` headers, which `single_hunk_diff` writes from the same
  text, do not name as git does; and a rooted path with no drive, such as `/src/f.rs`, is no absolute path
  there, so the refusal of an absolute path does not reach it, and `Path::join` keeps the root's drive alone.
- The same function compares the component `.git` byte for byte. On a disk that ignores case, as Windows' and macOS's by default, `.GIT/HEAD` reaches the
  git directory.
- `run_command` of `path@mutate@src/run.rs` and the probe start a program by its name. On Windows the
  standard library's lookup finds no `.cmd` or `.bat` file by a bare name, and looks in the system's
  directory, whose `bash.exe` starts WSL's, before the caller's path.
- Line endings. Run on Linux over a file that holds CRLF, with the working copy of this entry's branch:
  a multi-line `--replace` whose text holds LF is refused, "the pattern does not occur in the target", exit
  2; and a `--patch` whose lines hold LF is refused, "the patch does not apply to f.txt", exit 2. Under Git
  for Windows' default every checked-out file holds CRLF unless the project's attributes say otherwise.
- The hint that names `git apply -R` prints the record's directory joined with the system's separator.
  Observed on a Windows runner: `git apply -R .git/knowledge-architect-mutate\mutation.patch`, which a
  POSIX shell reads with the `\` as an escape.
- Git Bash rewrites an argument that opens with `/` into a Windows path before the program starts, so
  `--with '/ 2'` can write another text into the mutant than the caller named. Read in the documentation
  of MSYS2, not run.

The tests of `path@mutate@tests/modes.rs` start `sh`, `true`, `grep` and others as the command under
`--`. Run on Linux at this entry's opening, with a path that holds git alone: 29 of the 44 tests of that
file fail, and one of the 32 of `path@mutate@tests/harness.rs`. On GitHub's Windows runner Git for Windows
provides these programs.

Skipped on Windows for this entry, in `path@mutate@tests/modes.rs`:
`a_run_killed_while_planning_its_patch_leaves_its_scratch_in_the_claim_alone`, whose git is a shell script,
which Windows does not find as `git`; and `the_restore_writes_through_no_symlink_the_command_left`, whose
command runs `ln -s`: on a Windows runner the run printed `SURVIVED` and exited 0 where the test expects 1,
since MSYS2's `ln -s` copies the file by default (assumption, from its documentation as remembered).

Ruled out: what the machine's git converts in the scratch repository of a `--patch` run, which gave a
false `SURVIVED`, per `design@mutate@git-converts-no-line-ending`. The liveness of a run is
`issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`.

### Why it matters

A mutant that is not the one the caller named gives a verdict over another mutation, against what
`design@mutate@mutation-command` is for, and a refusal with no right reason stops a claim's check.
The owner left development on Windows for later, while a project developed there is one that
`design@knowledge-architect@platforms-served` serves.

### What would close it

Each item above repaired, or refused with its reason, and shown by a test that runs on Windows; the two
tests run there, with a compiled program for the git that kills its parent, and a link made by a program
of the test; and the tests' commands given by programs that exist on the three systems.
