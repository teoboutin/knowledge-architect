# The gates run and block on Linux, Windows and macOS, and a project that depends on knowledge-architect builds and runs its gates on Windows

## Status and audience

This spec plans the work of one branch and one pull request. It leaves when its work lands, in a
commit of its own after the review before the merge. Where it and a design home disagree on a point
that "What is already decided" does not list as reversed or rewritten, the design home wins. Every
name it uses is defined in it or exists in the tree. Where the owner's word is awaited,
`skill@knowledge-architect-planning@working-a-slice` says what waits.

It was written for a session that did not see the discussion. The discussion is one session, which
begins at the owner's message "I want you to  take on the windows port of this project." Its rounds,
each named by the opening words of the owner's message:

1. "I want you to  take on the windows port"
2. "Q1: this is not a matter of absolute time taken"
3. "Why does linux clippy has warnings right now"
4. "But we can run the clippy gates for windows while on linux"
5. "Agreed on these proposals, full cargo x gates"
6. "Q1 take in and offer it as opt in"
7. "This needs to be a single branch and not a milestone though"
8. "Q3 agreed those can be skipped too"
9. "D1: I would keep skipping"
10. "d3 d4 accepted"

The threads and the arguments are in [deliberation](deliberation.md); the record search's generated
lines in [the discussion's search](search/discussion-1.txt); the transcript in
[the transcript](transcripts/1.txt), which ends at round 8, the owner's word on the last checkpoint
table, and in which paths under a home directory, identifiers of sessions
and agents, the harness's notifications and details of the owner's other projects are removed.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, "The work of a spec", with
`skill@klarch-development` and `skill@knowledge-architect-development` for the Rust code, and
`path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", for the setup skill's text.

**Iterating on the other systems.** The owner reported having no Windows development machine, so the
runners are the only machines of the two other systems this work has. CI does not run on a draft,
per `design@knowledge-architect@ci-skips-a-draft`, and no trigger is added to the CI workflow. The
probe branch `windows-macos-probe` is on origin, with no pull request, and never merges. It departs
from `design@knowledge-architect@git-flow`, under which a pushed branch has a draft pull request: it
holds no work to review, as a scratch worktree holds none per
`design@knowledge-architect@scratch-worktree-outside-git-rules`, and the owner allowed its push in
round 2. To run a state of the work on the three systems, it is rebuilt as that state plus one
commit that adds its workflow, which triggers on a push to that branch, and pushed with force. Its
workflow runs, on each system, under a shell every runner has:

- the gates command of CI without `--fail-fast`;
- `cargo --locked test --locked --workspace --no-fail-fast` with `KNOWLEDGE_ARCHITECT_CI` set to `1`,
  since the `test` gate stops at the first test binary that fails;
- the gates of a project `klarch init --rust` writes, patched to the checkout's crates, its
  maintenance crate given a test and a dev-dependency on `tempfile`, so that its test build of that
  crate differs.

The branch's present head holds rough repairs, which are evidence and not the shape of the work. The
branch is deleted, on origin and locally, where the worktree that has it checked out is removed
first, on the owner's word alone. A push to it, as any push, follows
`design@knowledge-architect@push-after-the-commits-check`. A measurement of this work keeps the
workflow, script or command that took it in the message of the commit that records it.

## Names

- **the three systems**: the GitHub-hosted runners `ubuntu-latest`, `windows-latest` and
  `macos-latest`, as they stand when the work runs.
- **the probe**: the branch above and the runs of its workflow. Its two past runs are 45592208005, at
  the branch's first commit, and 45606918811, at its second.
- **the simulation**: the test suite run on Linux over a standard library built from source and
  patched so that `\` is the main separator, `/` and `\` are both accepted, and the paths it returns
  hold `\`; and, apart from it, the suite run under `core.autocrlf=true`. It gives neither a drive
  prefix, nor a verbatim prefix, nor a short file name. It was run once, for the inventory of the
  reply to round 1, and no script of it is kept.
- **the gates command of CI**:
  `cargo --locked x gates --locked --fail-fast --require-rebased --full --ci`.
- **the running binary**: the executable of the maintenance crate that runs the gates, which
  `cargo x` builds and starts.
- **the swap**: cargo replacing the running binary. In this repository `cargo run -p xtask` and
  `cargo test --workspace` build two variants of it at one path, since the features of a dependency
  differ, and each build puts its own there.
- **the stub**: the program `true` named as the C compiler, the C++ compiler and the archiver of a
  target, so that a build script that compiles C succeeds with nothing compiled.
- **development workflow**: what a person or an agent does while developing a project, such as the
  hand mutation, the transcript reader, the record search's work list and a pre-commit hook. Building
  a project, running its gates and what the gates run are not.
- **lacked by nature**: a behaviour a system does not have at all. Five kinds are admitted: Unix mode
  bits, a named pipe, and a file name the system refuses, a name that is not UTF-8 among them; and, on
  Windows and macOS, the device /dev/full, and a working directory that is removed.

## What the work is

**The purpose**, as the owner stated it in round 2: a project that depends on knowledge-architect
keeps building on Windows, to deliver its product there, and tests itself there with its own gates
command, with no logic duplicated that decides what runs there. macOS is the owner's ask of round 1,
a port with a CI job of its own. The gates library exists to unify
what must block a merge, with as little deviation as possible between a development environment, CI
and the systems. Development of a project on Windows is left out of this work, and the owner wants
it later.

What exists today, read at main `9f1efd7b`, "[knowledge-architect] Repairs of the
fidelity-of-relocation review: the guard's reopening condition, a reference, the lint's scope, two
manifests and the figures of a full inode table":

- `path@knowledge-architect@.github/workflows/ci.yml`: one job, `check`, on `ubuntu-latest`, skipped
  on a draft, with full history, a step that runs `nproc`, one step that runs the gates command of
  CI, and the artifact `gate-logs` on a failure. The ruleset on main requires the status context
  `check`.
- `path@git@src/git.rs`: outside Unix, `path_bytes` returns `Vec<u8>` where `blob_bytes` and
  `read_batch` need `&[u8]`, so the library does not compile for Windows, nor does any crate that
  depends on it. It spells a path with `\` there; `stage_generated` gives that spelling to
  `git update-index --index-info`, which git on Windows refuses (read in git's source, not run).
  `object_name` and `ignore_query` spell a path through a lossy decoding.
- `path@knowledge-architect@.cargo/config.toml` and `path@agent-skills@snippets/cargo-config.toml`:
  `x = "run -q -p xtask --"`, the dev profile. In the snippet, `klarch` already runs the maintenance
  crate with `--release`, and in a project of that shape the gates `check` and `commits` run
  `cargo run -q --release -p xtask -- klarch …`.
- `path@gates@src/gates.rs`: `rust_project` builds the recommended list; every field of `Gate` is
  public; `path@xtask@src/gates.rs` builds this repository's list from it, and its tests pin the
  list's names and read the CI workflow for `--ci`.
- `path@check@src/check/generated.rs`: `compare` compares a generated file with the regenerated text
  byte for byte. `lf` in `path@check@src/agents.rs` normalises line endings for the installed files.
- `path@agent-skills@build.rs` and `path@agent-skills@src/render.rs`: a snippet and a fragment lose
  their last `\n` and keep a `\r` before it. `Snippets::listing` includes the files themselves, so
  text that `build.rs` normalises in memory does not reach `SNIPPETS`.
- Sites of `.display()` in non-test code, counted with the test modules stripped: `model` 62,
  `query` 44, `check` 38, `create` 36, `git` 9. Those that print a project-relative path
  print one that `PathBuf::join` builds with `\` on Windows; most sites of `query` are in `worklist.rs`
  and name its `--out` directory, outside the project, and about two thirds of `create`'s print an
  absolute path or the path the user gave. Other calls print one too, such as `to_string_lossy` in the
  repair `link as` of `path@check@src/check/references.rs`. `path_text` in `path@query@src/graph.rs` writes `/`.
  `impl Display for Site` in `path@model@src/entity.rs` prints most sites of `query`.
- `apply_in` in `path@mutate@src/run.rs` runs `git apply` in a scratch repository under the user's
  `core.autocrlf`.
- No `.gitattributes`. No tracked file holds a carriage return.
- 24 test functions do not compile for Windows, which blocks 8 test targets: 21 use only
  `std::os::unix::fs::symlink` of the Unix API, of which 18 need the link helper alone, two also run
  `mkfifo`, and one links a name Windows refuses; 3 use `PermissionsExt`. Nine more functions, under
  `cfg(unix)`, use a symbolic link alone.
- `path@transcript@src/reader.rs`, its test module included, is compiled into `xtask` by `#[path]`.

Measured by the probe, whose repairs are rough: the workspace compiles on Windows, a path is named to
git with `/` there, which is the product repair of #paths-to-git-with-slashes, and every file is
checked out as committed. So no run exercised main's `\` in what `git` sends.

| | Linux | Windows | macOS |
| --- | --- | --- | --- |
| the job of the tests alone, to the end, cold, with its checkout and toolchain | 2:45, all pass | 6:37, 118 fail | 5:18, 5 fail |
| the gates `check`, `commits`, `clippy` and `doc` together, in the dev-profile run, after `xtask` was built; on Windows `clippy` failed | 71 s | 70 s | 44 s |
| the release build of `xtask` before the first gate | 55 s | 79 s | 71 s |
| `check` to `doc` after that build | 25 s | 30 s | 33 s |
| the gates step on Linux, the same in the dev and in the release profile | 266 s | not complete | not complete |
| the gates command in the dev profile | passes | the `test` gate fails: "failed to remove file … xtask.exe", "Access is denied. (os error 5)" | fails on a test |
| the gates command in the release profile | passes | passes that point, then fails on tests | fails on a test |
| `path@testing@tests/scaffold.rs` | passes | passes | passes |
| a project `init --rust` writes, its maintenance crate given a test and a dev-dependency, running its own gates, with `x` in the dev and in the release profile | both pass | both pass | both pass |

On Windows, `fmt`, `check`, `commits` and `doc` pass, and `clippy` fails on two items that only code
under `cfg(unix)` uses: the constant `CORRECT_SPAN_REPAIR` of `path@testing@tests/binary.rs` and
`use super::*` of the module `checker` of `path@testing@tests/model_over_mocks.rs`.

The causes of the failures, as the simulation counted them over its own runs on Linux; a test can have
two causes, and a test stops at its first failing assertion. Of the 118 that failed on Windows, 107
were predicted by the simulation and 10 by reading:

| cause | tests | where |
| --- | --- | --- |
| a printed path holds `\` | 62 simulated, 60 of them failed on Windows | the product, but for one test of `path@xtask@src/changelog.rs` and three of `model` |
| a path written raw into a JSON string | 29 | the helpers `transcript_case`, `copy_rendered` and `transcript_config` of `path@testing@tests/binary.rs` |
| `core.autocrlf` in a repository a test or the product makes | 14 simulated, 12 failed on Windows | 8 tests through `apply_and_reverse` of `path@mutate@tests/harness.rs`, 1 in `path@mutate@src/run.rs`, 5 in `path@mutate@tests/modes.rs`, of which two passed on Windows, where its `grep` matches over CRLF; one printed `SURVIVED` for a patch that changes nothing |
| a path joined, then compared as text | 2 | two tests of `resolver::tests`, through `ignore_query` |
| a path joined, then sent to git with `\` | not exercised, see above | `stage_generated` |
| the liveness of a run | 3 | `a_record_this_process_claimed_does_not_read_as_owned_by_a_live_run`, `restore_refuses_while_the_owning_run_is_alive`, `a_record_over_an_unmutated_file_is_cleared_by_restore` |
| a name the system refuses | 5 | a line feed in four tests, `a:b` in `a_symlink_with_a_refused_name_is_one_finding` |
| a named pipe, `ln -s` | 2 | `a_target_through_a_symlink_in_git_or_not_a_file_is_refused`, `the_restore_writes_through_no_symlink_the_command_left` |
| a stub program as a shell script on a path joined by `:` | 1 | `a_run_killed_while_planning_its_patch_leaves_its_scratch_in_the_claim_alone` of `path@mutate@tests/modes.rs`, which exits 0 where it expects a kill; that the stub is not found is read, not run |
| an absolute path that does not open with `/` | 1 | `new_component_accepts_a_relative_root` of `path@testing@tests/extension_api.rs`, at `strip_prefix("/")`, met by no prediction; the temporary directory on another drive than the checkout can stop it next, not observed |

Read and not observed: a short file name in the temporary directory, which would fail
`a_patch_run_works_in_a_linked_worktree` of `path@mutate@tests/modes.rs` after its CRLF failure.

Without every file checked out as committed, 65 more tests fail (simulated on Linux). The 5 macOS
failures: the three liveness tests, and `a_patch_run_works_in_a_linked_worktree` and
`init_lists_the_repository_the_kept_gitignore_and_the_enclosing_project` of
`path@cli@tests/init.rs`, which compare a path given with the path its symbolic link resolves to.
Two whole test files are `cfg(unix)`: `path@xtask@tests/gates_bin.rs`, 12 tests, and
`path@xtask@tests/mutate_bin.rs`, 3 tests. Two tests are for Linux alone:
`a_closed_stdout_leaves_every_document_written` of `path@cli@tests/init.rs`, which opens /dev/full,
and `a_working_directory_that_is_gone_is_exit_two_naming_it` of `path@testing@tests/binary.rs`. Two
tests pass on Windows and test nothing there: the one of `path@mutate@tests/harness.rs` that gives
`--file /etc/passwd`, which is no absolute path on Windows, and
`a_patch_run_makes_nothing_in_the_temporary_directory` of `path@mutate@tests/modes.rs`, which sets
`TMPDIR`, which Windows ignores (both read).

Read of GitHub's runners, for whoever reads a later run: from 2026-11-30 the D: drive of the Windows
images is ReFS; at most 5 macOS jobs run at once on a free plan.

The closing of `the-workspace-does-not-build-or-run-its-tests-on-windows`, an issue of the root: the
commit that adds this spec closes it, since the spec schedules its work, per
`skill@knowledge-architect-planning@spec-or-milestone`. Its reading of the goals is under
#platforms-served; the liveness of a run is `issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`;
its lead that a directory holding an open file may not be removable matched no failure of the probe,
and its lead that git's read-only objects block a removal is ruled out by the standard library's
removal on Windows, which ignores that attribute (read).

**Outside the work.** Each row is one issue, opened by step 7, or before it by the commit that adds a
skip citing it; its evidence is taken again from the code when it is opened, except where it says
opened already:

| anchor | what | evidence |
| --- | --- | --- |
| mutate | the liveness of a run on Windows and macOS | `issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`, opened with this spec |
| mutate | on Windows: `--file` with `\` or a rooted path; program lookup that skips `.cmd` files and finds System32 first; a multi-line `--replace` with LF over a CRLF file; a patch's line endings against the file's; `.git` compared case-sensitively, on macOS too; the printed `git apply -R` hint with `\`; Git Bash rewriting an argument that opens with `/`, so that a wrong `--with` is written into the mutant; the tests that start `sh`, `true`, `grep` and others, 33 of which start nothing where git alone is on the path | read; the two on line endings run on Linux over CRLF files; the 33 counted on Linux with such a path |
| transcript | `HOME` alone names the directory, which PowerShell does not set; a session of a removed directory is dropped, since a canonical root holds a verbatim prefix | read |
| gates | no pipe is refused outside Unix, a divergence from `design@gates@gates-refuse-a-pipe`; under `--full`, a chunk cut inside a character loses the rest of the live echo on a Windows console | read |
| query | `worklist` prints verbatim paths and `agent-<n>\report.md`, which the record-search workflow's patterns do not match | read |
| agent-skills | the installed shell commands in PowerShell, Claude Code's primary shell on Windows where its tool is on, and in zsh: heredocs, `SHELLOPTS`, and `--version =<version>` under zsh | the harness's documentation read; zsh not run |
| model | a path reference, a relative link or a manifest row holding `\` or a drive prefix is accepted on Windows and reported missing on Linux, and `C:/…` stops the run there; refusing it reaches `skip-files` rows, the way a project keeps a file of such a name; a directory in a tracked document's place reads as access denied on Windows, so its finding names the wrong repair | read |
| git | git's ignore match ignores case on a case-insensitive disk, so a reference passes locally and fails on Linux | read |
| create | on a case-insensitive disk `init` keeps a `Readme.md` that the check then misses; on macOS an absolute DIR under /tmp is refused as outside the project | read |
| the root | a pull request's first CI run starts with no cache, since nothing runs on main to fill one | run 45581213477, "No cache found" |
| the root | a working directory spelled another way on Windows rebuilds the crates tied to their checkout; on macOS cargo copies a binary at each `cargo run`, so parallel `cargo klarch` calls have a moment with no binary | read |
| the root | a run of CI skipped on a draft fills the run history and reads as passing | `issue@knowledge-architect@a-ci-run-skipped-on-a-draft-reads-as-passing`, opened already |

Whether Windows projects are asked to use Git Bash is the owner's to rule when the agent-skills issue
is taken up; nothing of this work waits on it.

## What is already decided

Rests on, not argued again: `goal@every-target-project-can-adopt-it`,
`goal@serves-one-kind-of-project`, `goal@generic-tools-and-procedures-are-shipped`,
`goal@the-owner-decides`, `design@gates@a-project-holds-its-gate-list`,
`design@gates@the-library-owns-the-flags`, `design@gates@one-spawn-helper`,
`design@gates@verdict-from-exit-codes`, `design@gates@gates-run-all`,
`design@gates@ci-flag-for-ci-only-requirements`, `design@gates@distillers-are-a-closed-set`,
`design@knowledge-architect@gates-crate`,
`design@knowledge-architect@main-is-guarded-by-a-ruleset`,
`design@knowledge-architect@the-merge-waits-for-a-successful-run-on-the-head`,
`design@knowledge-architect@a-build-is-tied-to-its-checkout`,
`design@knowledge-architect@no-crate-depends-on-testing`,
`design@knowledge-architect@git-crate`, `design@git@git-is-asked-through-one-module`,
`design@git@an-operation-in-git-s-terms-is-the-library-s`, `design@model@unportable-names-refused`,
`design@model@git-supplies-the-walk`, `design@check@installed-files-compared-by-bytes`,
`design@check@index-staged-write`, `design@check@index-takes-no-dry-run`,
`design@agent-skills@rust-templates-one-home`, `design@agent-skills@overlay-by-separate-skills`,
`design@agent-skills@split-not-needed-is-an-issue`,
`design@agent-skills@splits-planned-before-work`, `design@create@rust-scaffold`,
`design@create@init-writes-no-ci-workflow`.

Rewritten, with every text that references each, as `cargo klarch show` lists them and a grep of its
slug over the files the checker does not read; step 8 or the harvest reads each:

| decision | what changes | texts that reference it |
| --- | --- | --- |
| `design@xtask@gates-list-primary-home` | split in step 8, since it bundles the list's home, the rule that CI and a local run differ by flags alone, and cargo's own `--locked` ahead of the alias; the list gains the cross gate and the reason for this repository's own target; CI runs the list on three systems behind a gathering job | `skill@klarch-development@gate-in-this-repository`, `path@knowledge-architect@CLAUDE.md`, `path@knowledge-architect@README.md`, the opening of `register@gates@design`, `design@knowledge-architect@ci-skips-a-draft`, `path@xtask@README.md`, `path@xtask@src/gates.rs`, the opening comment of `path@knowledge-architect@.github/workflows/ci.yml` |
| `design@gates@recommended-gate-list` | a gate of the list is told from a gate the library offers beside it, since a gate taken in no longer always enters the list | `design@agent-skills@the-workflow-teaches-no-rust-idiom`, `design@agent-skills@diff-mutation-sweep`, `design@check@installed-binary-version-check`, `design@create@rust-scaffold`, `design@gates@a-project-holds-its-gate-list`, `design@gates@doc-gate-denial`, `design@gates@distill-over-full-logs`, `design@xtask@gates-list-primary-home` |
| `design@knowledge-architect@ci-skips-a-draft` | its argument, by the owner's ruling of round 9; its cost with three systems | `path@knowledge-architect@CLAUDE.md`, `design@knowledge-architect@git-flow`, `design@knowledge-architect@the-merge-waits-for-a-successful-run-on-the-head`, `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements`, the comment of the job in `path@knowledge-architect@.github/workflows/ci.yml` |
| `design@model@a-file-register-index-is-rows` | split in step 8; its part on the byte comparison is reversed by a head of `check` | `design@check@a-file-register-is-a-directory-of-entries`, `design@model@a-harvested-slice-is-frozen`, `path@cli@README.md`, `path@model@src/index.rs`, `issue@knowledge-architect@command-output-is-not-declared-a-contract`, the comment of `commit` in `path@testing@tests/binary.rs`, "the `generated` family compares bytes" |
| `design@git@a-path-is-bytes` | the goal it stands on and its condition outside Unix, per #a-listed-path-outside-unix | `issue@git@paths-are-decoded-outside-the-listings`, `path@git@src/git.rs` |

Touched without reversing or rewriting their decisions:

| decision | what changes |
| --- | --- |
| `design@agent-skills@xtask-pins-checker` | a pointer to the head on the release build, and its accepted cost read again |
| `design@knowledge-architect@toolchain-is-pinned` | a pointer to the head that holds the reason for the target; the comment of `path@knowledge-architect@rust-toolchain.toml`, which cites it, read again |
| `design@check@generated-files-are-pure` | a pointer to the head on the comparison without line endings |
| `design@check@installed-files-compared-by-bytes` | its title, which says "byte for byte" where its body normalises line endings |
| `design@agent-skills@content-mirrors-the-install-layout` | the build's normalisation, a member of the rendering it lists |
| `design@xtask@ci-builds-the-tip-alone` | its command names the job beside the step, since three jobs hold a `gates` step |

The design-record audit, whose verdicts on the heads' text at main 9f1efd7b hold at this spec's commit,
which changes none of them, read the five heads of the first table and the first three of the second: it found
`design@gates@recommended-gate-list` and `design@knowledge-architect@toolchain-is-pinned` conformant and the six others not. Step 8 has the last
three of the second table read by the auditor before its edits. It also read "a gate enters the list on
the owner's word" in `design@gates@recommended-gate-list` as the list's admission property, against the
rival that it is a second decision, on `goal@generic-tools-and-procedures-are-shipped`.

Entries of the record search whose outcome changed something:

- `design@git@an-operation-in-git-s-terms-is-the-library-s`, `design@knowledge-architect@git-crate`
  and `design@knowledge-architect@query-crate`: the printing function is in `model` and `git` keeps
  a private spelling, so none of the three changes (#path-spelling-home).
- `design@knowledge-architect@a-copy-of-the-writer-per-crate`: argues another case, so the copies of
  the symbolic-link helper have their own argument (#symlink-helper-home).
- `design@check@index-takes-no-dry-run`, `design@check@index-staged-write` and
  `design@agent-skills@overlay-by-separate-skills`: `index`, what `index --staged` stages and the
  install stay as they are, so none of the three changes
  (#generated-text-compared-without-line-endings).
- `design@check@fix-refusal-mixed-state`, `design@check@fix-scope` and
  `design@check@fix-before-the-checks`: `check --fix` chooses what it writes, and refuses, by the one
  comparison of currency, so it writes, and names, no file that `check` calls current; their words
  hold.
- `design@check@one-question-one-command`: one function answers currency for the three commands that
  ask it, so no second answer drifts.
- `design@model@one-list-of-generated-files`: the list does not change; its "disagree about what is in
  it" is about the generated text, which is one.
- `issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`: its re-entry point, a
  change that touches `index --staged`, is met by the currency line's comparison, which decides none
  of its shapes; step 5 adds the instance and the discussion stays open.
- `design@agent-skills@shared-code-decisions-go-to-the-lowest-enclosing-component`: the printing rule's
  head is the root's (#printed-paths-with-slashes).
- `design@agent-skills@one-decision-per-head`: the harvest rows below cut each thread by its rival.
- `design@check@the-facade-publishes-what-a-consumer-needs` and `goal@projects-add-their-own-checks`:
  an extension reaches `slashed` through the module `extension` (#path-spelling-home).
- `design@gates@doc-gate-denial` and `rejected@gates@deny-through-rustdocflags`: the cross gate sets the
  form of each variable that is read first (#cross-clippy-gate).
- `design@knowledge-architect@changelog-entries`: step 5 and step 6 write their entries.
- `goal@only-needed-instructions` and `goal@adoption-is-easy`: the setup skill gains no sentence on the
  cross gate, and `path@gates@README.md` says what a project on Windows and macOS reads.
- `design@agent-skills@no-untested-snippet-is-authority`: each code fragment of "Decided design" is an
  illustration of a shape.
- `the-subject-on-how-work-reaches-main-holds-tooling-heads`, an issue of the root: step 8 makes the
  split, since the harvest writes into that subject, and closes the issue.
- `design@agent-skills@splits-planned-before-work`: `lf` is called where it is and is not moved.
- `design@check@every-path-names-its-anchor`, `design@model@git-supplies-the-walk`,
  `design@model@unportable-names-refused` and
  `design@agent-skills@plan-landing-is-not-tied-to-its-work`: the refusal of `\` leaves the work, so
  none is strained and the spec stays on the work's branch (#backslash-in-a-pointer).
- `design@knowledge-architect@git-flow` and `design@knowledge-architect@ci-skips-a-draft`: no
  trigger is added to the CI workflow (#iteration-before-ready).
- `design@gates@gates-refuse-a-pipe`: the divergence outside Unix becomes an issue, step 7.
- `design@agent-skills@split-not-needed-is-an-issue`: no issue is opened for a signal over code the
  work writes (#no-module-split-in-the-port).
- `goal@the-owner-s-word-is-verified`: the skip criterion was put back to the owner, round 8, and its
  two further kinds, round 10 (#platform-skips).
- `goal@the-owner-decides`: the asks it reserves to the owner are those
  `goal@serves-one-kind-of-project` names, so the release alias is settled by argument; the rival
  reading, that every rule the checker or `init` tightens is such an ask, is not its words.
- `goal@every-target-project-can-adopt-it` and `goal@serves-one-kind-of-project`: the owner's purpose is
  read as served by them; the rival, a goal of its own, is not needed since neither excludes a
  platform.
- `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`: trigger met, put off by the owner
  in round 8; step 7 adds the instance (#asks-discussion-put-off).
- `issue@agent-skills@a-one-time-technical-setup-has-no-place-a-project-gets-it-from`: trigger met;
  a project meets the cross gate in `path@gates@README.md`, the documentation of the function and the
  changelog, and the alias in the snippet, the setup skill and
  a Migration entry. The issue stays open, and step 7 adds the instance.
- `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps` and
  `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`: step 8
  conforms the heads that fail, which the harvest then writes into, and splits two bundles. Both stay
  open, and step 7 adds the instance.
- `issue@git@git-rs-is-past-one-read`, `issue@check@history-rs-is-past-one-read`,
  `issue@model@entity-rs-is-past-one-read`, `issue@model@manifest-rs-is-past-one-read`,
  `issue@model@resolver-rs-is-past-one-read`,
  `issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read`,
  `issue@testing@binary-rs-is-past-one-read` and `issue@transcript@reader-rs-is-past-one-read`: no
  split here; step 7 adds this work as an instance of each whose file it changed
  (#no-module-split-in-the-port).
- `issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`: corrected in a commit
  of its own on this branch, where its first shape is a variant of the proposal it named.
- `issue@check@a-linked-manifest-splits-the-readers`: its citation of the closed issue is re-pointed
  to this spec, and at the harvest to the head of #platforms-served.

The other verdicts: [the discussion's search](search/discussion-1.txt).

## Criteria

### No platform condition is added to a target project `##no-platform-condition`

Binding, from `goal@every-target-project-can-adopt-it` and `goal@serves-one-kind-of-project`. Met by
#platforms-served.

### No workflow file decides which gates run on a system `##gates-decided-by-the-list`

Binding, derived by the session from the owner's purpose of round 2, and confirmed in round 8: the gates
library and the project's list decide. Met by
#gates-on-three-systems.

### A failure on any of the three systems blocks a merge to main `##any-system-blocks-the-merge`

Binding, stated by the owner in rounds 2 and 5. Met by #blocking-jobs and #one-required-context.

### A project that depends on the crates builds on Windows `##dependents-build-on-windows`

Binding, stated by the owner in round 2. Met by #paths-to-git-with-slashes and
#release-maintenance-build.

### A test is skipped on Windows only where it covers development workflow or a behaviour lacked by nature `##windows-skips-are-bounded`

Binding, stated by the owner in round 5, and widened by the owner to three kinds lacked by nature in
round 8 and to two more in round 10. A skip for development workflow names its issue, the session's
condition; one for a behaviour lacked by nature gives its reason and no issue. Met by #platform-skips.

### A merge waits for the slowest job only `##wait-is-the-slowest-job`

Weighed, from the owner's concern of round 1 and the owner's argument of round 2 that the jobs run
in parallel. Met by #gates-on-three-systems and #blocking-jobs.

## New names, in one place

```text
planned names, with their file
  cross_clippy(target: &str) -> Option<Gate>       crates/gates/src/cross.rs, re-exported by lib.rs
  gate_on(target: &str, machine: &str) -> Option<Gate>
                                                   the same file, private; cross_clippy calls it with
                                                   std::env::consts::OS
  the gate `clippy-<target>`                       what cross_clippy builds
  slashed(path: &Path) -> String                   crates/model/src/finding.rs, re-exported by lib.rs
                                                   and by check's module `extension`
  respelled(text: &str, windows: bool) -> String   the same file, private; slashed calls it with
                                                   cfg!(windows)
  named_to_git(text: &str, windows: bool) -> String
                                                   crates/git/src/git.rs, private
  current(on_disk: &str, expected: &str) -> bool   crates/check/src/agents.rs, beside lf: the one
                                                   comparison of currency
  snippet_list(snippets: &[(&str, &str)]) -> String
                                                   crates/agent-skills/src/render.rs: what
                                                   Snippets::listing wrote, over texts
  link_file, link_dir (target, link)               test code, one copy per package, see
                                                   #symlink-helper-home
  the binary fake-cargo                            tools/xtask/src/bin/fake_cargo.rs, with
                                                   default-run = "xtask"
  the jobs `gates` (matrix) and `check`            .github/workflows/ci.yml
  the artifacts `gate-logs-<os>`                   .github/workflows/ci.yml
  .gitattributes                                   the project's root
existing names the work changes
  path_bytes, object_name, ignore_query            crates/git/src/git.rs
  compare                                          crates/check/src/check/generated.rs
  the filter `written_here` and the write loop     crates/check/src/commands/mod.rs, of `check --fix`
  the currency line of index_staged                crates/check/src/commands/history.rs
  path_text                                        crates/query/src/graph.rs, replaced by slashed
  gates()                                          tools/xtask/src/gates.rs
  x                                                .cargo/config.toml, snippets/cargo-config.toml
```

## Decided design

A code fragment below is an illustration of a shape, not a tested text.

### #gates-on-three-systems

The job `gates` runs the gates command of CI in a matrix over the three systems, with
`fail-fast: false`, the project's whole list on each. Nothing in the workflow names a gate. Argument:
a choice of gates written in a workflow is a second place that decides what runs, which the criterion
excludes, and the gates other than `test` cost little beside it: 70 s cold on Windows in the dev
profile, and 110 s with the release build of the maintenance crate that #release-maintenance-build
adopts. Nearest rival: a subset of
gates on the two other systems, the owner's first ask, which the owner retracted on the purpose stated
in round 2. `cargo test` run directly lost on the same ground, and hides a defect of `gates` itself.

Conditions: each matrix job keeps `fetch-depth: 0` and the pull request's head as its `ref`, or
`commits` judges a shorter range with no error; each names its artifact for its system, since one run
refuses two artifacts of one name (from the action's documentation as remembered, not run); the job
keeps the draft condition and `timeout-minutes: 30`; its steps run under `shell: bash`, which every
runner has; the step that runs `nproc`, which macOS lacks, prints `cargo --version` and
`rustc -vV` alone.

### #one-required-context

A job named `check` needs the matrix job, runs on
`if: ${{ !github.event.pull_request.draft && always() }}`, and fails unless the matrix's result is
`success`. The ruleset keeps requiring the context `check`, so no setting changes. Without `always()`
the job is skipped when the matrix fails, and GitHub counts a skipped job as passing. Nearest rival:
three required contexts, a setting the owner would change in step with the merge, and the pull request
that adds them could not pass the old one. Two jobs not required by the ruleset lost: the merge
predicate alone would guard them.

### #blocking-jobs

The jobs block every ready pull request, in the one CI workflow, so the merge predicate of
`instructions@git-workflow`, which reads the run's conclusion, does not change. Nearest rival: a run
on main after the merge, under which a break on another system reaches main. No absolute time bounds
the jobs: the owner's argument of round 2, that they run in parallel and the suite grows.

### #release-maintenance-build

The alias is `x = "run -q --release -p xtask --"`, in this repository and in the snippet that
`init --rust` writes and the setup skill shows. The gates `test`, `clippy` and `doc` write to the dev
profile's directory alone, so none replaces the running binary. In a project of the scaffold's shape
`check` and `commits` run the running binary itself, which cargo finds fresh where the gate's cargo reads
the inputs of the cargo that built it: the probe ran such a project's whole gates on the three systems.
Where it does not, the gate builds the binary again and replaces it, which Windows refuses. Two inputs
were observed to do so on Linux, each replacing the binary in a run that passed there: a profile setting given by `--config` to the
cargo that built it alone, and an exported `RUSTC_BOOTSTRAP`, which the gates library removes from every
child. Costs: a release rebuild of the maintenance crate after an
edit of it or of a crate it links, and, in a project whose release settings are slow, those settings.
The setup skill's sentence that `cargo klarch` alone runs in release mode is repaired in the same
edit. Nearest rival: a cargo profile for the maintenance crate, which keeps a project's release
settings out and builds its dependencies once more; it lost since the `klarch` alias already builds
that crate in release in every consumer. A target directory of its own lost: its path is read from
the working directory. A library that renames or copies the running executable lost: the file has two
hard links. The gates step on Linux took 266 s in either profile. One more cost: under the alias the
project's own maintenance commands run with no debug assertions, which the head states. The probe's
release run on Windows stopped at the first failing test binary, so that no later test replaces the
running binary is read: no test builds the workspace's release `xtask`, and `path@testing@tests/scaffold.rs`
builds into a target directory of its own. That test's dev build of `xtask` then serves nothing and is
removed in step 3; its threshold of 60 seconds is read again there.

### #cross-clippy-gate

`gates` exports `cross_clippy`, which takes a target and returns one gate, or nothing where the
machine's own system is the target's, by the mapping table below. The gate is named
`clippy-<target>` and runs `cargo clippy --workspace --all-targets --target <target> -- -D warnings`,
resolving dependencies, with the distiller `WithoutProgress`. Its `envs` set `CC`, `CXX` and `AR` for the
target, each in two forms, `<VAR>_<target>` with the target's hyphens and `<VAR>_<target>` with
underscores, to `true`, and `RUSTC_WRAPPER` to the empty string, since `sccache` refuses the stub. The
`cc` crate reads the form with hyphens first (read in cc 1.5.1, `target_envs`), so a gate that set the
other alone would give way to an exported compiler. Clippy never links, so nothing
compiled is missed. A project adds the gate to its list, so it is opt-in and no flag is added; the
fields are public, so a project with a real cross compiler removes the variables. It sits in a module
of its own. It stays out of the recommended list: there, it would fail in every project until the
target is installed, and in every project whose crates refuse the stub.

This repository's list gains the gate for `x86_64-pc-windows-gnu`, after `clippy`, and
`path@knowledge-architect@rust-toolchain.toml` declares that target, which rustup installs on first
use (run, in an isolated setup). No gate for macOS: no code here is particular to it. The gate runs on
the macOS job too, where it has not been run. The reasons for this target are this repository's, so
they go to the head of `register@xtask@design` on the list, and not to the library's.

Measured on the owner's workstation: 4.1 s cold and 1.5 s after an edit; 27 MB downloaded and 133 MB
on disk per toolchain; 487 MB per target directory. Over 16 crates that build C, each the one
dependency of a new library under `cargo check --target x86_64-pc-windows-gnu`, 13 pass with the
stub and 1, `blake3`, without it: tree-sitter, zstd, rusqlite with its `bundled` feature, ring, libz-sys
with `static`, blake3, mimalloc, bzip2-sys, onig, git2, curl, reqwest and libgit2-sys with `vendored`
pass; openssl, openssl with `vendored` and aws-lc-rs fail. The `-msvc` target fails
with the stub, so code particular to `-msvc` is not judged. What it finds is compile errors and
lints, not test failures. Its other costs, told to the owner before the ruling: each job that reads
the toolchain file downloads the target, the Windows and macOS ones included; each worktree with a
target directory of its own pays the disk; and a lint in shared code is reported twice. What
`path@gates@README.md` tells a consumer: those limits; that the gate rests on the
`cc` crate accepting objects that were never made, which nothing documents; and that a build script
which probes the compiler to choose a feature can be answered wrongly. It says besides that the gates
run on Windows and macOS, why the alias `x` is in release, and that development on Windows is not
covered yet. The setup skill gains no sentence on the gate.

The owner took it in, opt-in, in round 6, arguing that a consumer for which it comes to fail on
something external to this project can opt out and loses little, since it only reports some failures
of Windows slightly earlier. Nearest rival: the gate in this repository's list alone, under which each consumer
finds the stub and the `sccache` refusal again. The Windows job alone lost: it reports once the pull
request is ready. A real cross compiler asked of every machine lost on its installation.

### #line-endings-of-this-repository

`.gitattributes` holds `* text=auto eol=lf`: every file is checked out with LF, and a file that holds
CRLF is committed with LF. Nearest rival: `* -text`, the shape of the checkpoint table, under which
every file is checked out as committed. They differ only for a file added later with CRLF, which the
rival takes in as it is; no tracked file holds a carriage return today. The cost of the shape: a test
file that needs carriage returns is marked as an exception. The session ranked an entry of CRLF into the
history, which a later development on Windows would meet, above that exception, which no goal ranks;
the owner accepted it in round 10, D3. The file does not hide
from CI what a consumer meets: the test of step 5 checks out under `core.autocrlf`.

### #generated-text-compared-without-line-endings

A generated file that differs from the regenerated text by line endings alone is current, as the
installed files are judged. One function, `current`, over `lf` where it is, answers it at every place
that asks: `compare`, the filter by which `check --fix` refuses a partial commit, what `check --fix`
writes, and the line by which `index --staged` names a working copy out of date. `index`, what
`index --staged` stages and `install-agent-skills` write as today, where bytes differ: rewriting a CRLF
file is harmless there, since the command is asked to write. `compare` passes the normalised texts to
`first_difference`, which drops a carriage return. Without it, the `check` gate of a consumer fails on
a fresh clone under Git for Windows'
default, measured on Linux under `core.autocrlf=true`: 16 indexes here and 3 in a new project.
Nearest rival: an ask that each project declares its line endings, which adds an ask and leaves the
failure until it is followed. A wider shape, under which `index` and the install leave such a file too,
lost: it reverses `design@check@index-takes-no-dry-run`, `design@check@index-staged-write` and
`design@agent-skills@overlay-by-separate-skills` to avoid a harmless rewrite. A narrower one, under
which `compare` alone changes, lost: `check --fix` would write, and refuse over, a file that `check`
calls current, and `index --staged` would call it out of date.

The build of `agent-skills` gives one text whatever the line endings of its checkout: `build.rs`
normalises what it reads, and `SNIPPETS` holds the normalised text and not the file, which
`snippet_list` writes. Measured: a
checker built from a CRLF checkout ships stray carriage returns, 22 findings that the install does
not clear. Nearest rival: the line-endings file of this repository alone, which a checkout by another
tool than git, or a later edit of that file, undoes.

### #paths-to-git-with-slashes

On Windows, `path@git@src/git.rs` names a path to git with `/`, by one private spelling,
`named_to_git`, used by `path_bytes`, `object_name` and `ignore_query`, and by the user messages of that
file that print a project-relative path. `path_bytes` returns `Cow<'_, [u8]>` on every system. On Unix nothing changes:
a path stays bytes. `object_name` and `ignore_query` decode lossily today, which
`issue@git@paths-are-decoded-outside-the-listings` keeps. Pathspecs given as arguments, `git_tracks`
in `path@check@src/commands/mod.rs` and `refuse_ignored` in `path@create@src/command.rs` keep their
spelling: git for Windows normalises `\` there (read in git's source), and `check` passed on Windows
in the probe. Nearest rival: the compile repair alone, which leaves `index --staged` refused.

### #a-listed-path-outside-unix

Outside Unix, `as_path` decodes a listed name as UTF-8, since a path there is text. A name that is not
UTF-8, which an index can hold where it was committed on Linux, is lost: it would be decoded with a
replacement character, named back to git so, and answered by no bytes (read, not run). The loss is a
defect, against the goal below, held in `issue@git@paths-are-decoded-outside-the-listings`, whose statement that the listings carry
git's bytes is brought to it, and the comment of `as_path`, under which git could not have produced such
a name there, is corrected. `design@git@a-path-is-bytes` gains that condition and the goal it stands on,
`goal@documentation-stays-consistent`: a tree that passes the checker conforms, in the texts the checker
reads, to every rule it holds. Nearest rival: an error on such a name, outside Unix, which closes the
issue's part there; it is a change of what the checker refuses, for a development machine this work does
not serve, and stays the issue's.

### #printed-paths-with-slashes

A project-relative path that `model`, `check`, `query`, `create` or `xtask` prints, or writes in a repair
it suggests, is spelled with `/` on every system, by `slashed`, whatever call prints it today:
`.display()`, `to_string_lossy`, a `Debug` format. What decides is that the path is project-relative. An
absolute path, or one the user gave, is printed as the system writes it, as the paths of `worklist`'s
`--out` directory are, and a manifest row the model refuses is printed as written. `git` spells its own
by its private function. `transcript` and `mutate` are development workflow and keep theirs. Argument: a
suggested repair such as `path@<anchor>@<dir>\<file>` passes on Windows and
fails on Linux, and 60 tests fail on the mix. Nearest rival: every `PathBuf` built with `/`, over
more sites of `join` and `push`, each one a place to forget. A change of `Finding` alone lost: other
sites print, and a finding's text can hold `\` as itself. The guard is the Windows job.

### #path-spelling-home

`slashed` is public in `model`, beside `escaped`, and `check`, `query` and `create` call it; it
replaces `path_text`, which rewrites `\` on every system: on Linux a name holding `\`, which the walk
refuses, is then printed as it is. It rewrites the separator on Windows alone, where a name cannot hold
`\`. `check`'s module `extension` publishes it beside `rel_from`, so an extension's findings and
generated files spell a path as the checker's do, per `goal@projects-add-their-own-checks`.
Nearest rival: one public function of `git`, which every one of these crates depends on; it lost on
`design@git@an-operation-in-git-s-terms-is-the-library-s`, under which who calls a function is no
ground, and printing a path is no operation on a repository.

### #symlink-helper-home

The helper that makes a symbolic link on the three systems is two functions, `link_file` and `link_dir`,
since a link to a target that does not exist has no kind to read, copied in the test code of each
package that uses it: `check`, `cli`, `git`, `mutate`, `testing`, `transcript`. Where a package's uses
are in its library, the copy is in one `cfg(test)` module; in `transcript` it is inside the test module
of `reader.rs`, which `xtask` compiles by path; where they are in integration tests, it is one file,
`path@testing@tests/links/mod.rs` in `testing` and the same file in `cli` and `mutate`. It fails where the machine refuses links; nothing is skipped. A relative target
is written with `\` on Windows. Argument: no crate may depend on `testing`, making a link is no
operation in git's terms, `mutate` depends on no crate of the workspace, and a crate for it would take
no decision and owe a Component's documents for two short functions. Nearest rival: a skip where links
are refused, under `KNOWLEDGE_ARCHITECT_CI`, whose rule would be copied six times for a machine that
this work does not serve. `design@gates@ci-flag-for-ci-only-requirements` gives such a skip to a test
that needs a tool a machine may lack; the reading taken is that no served system lacks the right to make
a link, so the head does not reach the helper, against the rival reading that the right is such a tool.

### #no-module-split-in-the-port

No module is split. The work writes into modules past one read; nearly every edit replaces one call
at one site, so it pushes no signal it does not find, and a move beside edits across every part of a
module would hide them. `cross_clippy` goes in a module of its own in `gates`, since
`path@gates@src/gates.rs` holds two parts that change apart and the gate has a ground of its own. Each
standing issue on a module past one read that the work writes into gains the work as an instance, and
no issue is opened for `tree.rs`, `references.rs` and `registers.rs`, past one read, for
`commands/mod.rs` of `check`, near it and holding two parts that change apart, and for `command.rs` of
`create`, whose commands sit in one module: the scout's signals over them are weighed here. The build's
normalisation is in the part of `agent-skills` that
`issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills` would move, and
stays there.
Nearest rival: a split of each before the work, a move larger than the work.

### #mutate-tests-on-windows

`mutate` is development workflow. Its tests are all to pass on macOS. On Windows a test is repaired where
the repair is cheap, and otherwise may be skipped naming its issue. The repairs: the two liveness tests
that write pid 1 write the pid of a live child of the test, and the test that writes pid 0 writes the
pid of a child that was waited for; the scratch repository of a `--patch` run, `apply_and_reverse` and
`linked_worktree` run git with `core.autocrlf=false`, the first in the product, since a false
`SURVIVED` was measured; both sides of a path comparison are canonicalised; the two tests that pass on
Windows and test nothing there give a path that is rooted on each system, and set `TMP` and `TEMP` beside
`TMPDIR`. On Windows the liveness
tests that need a live owner are skipped, naming
`issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`, and so are the test whose stub is a shell
script and the one that runs `ln -s`, naming the issue on `mutate` on Windows. Nearest rival: the lock that the liveness issue
names, in the product, a change of `mutate`'s behaviour that the purpose does not need.

### #platform-skips

- A test may be skipped on Windows where it covers development workflow, and only there. The mark is
  `#[cfg_attr(windows, ignore = "...")]`, so it still compiles there, with the issue's reference in a
  comment beside it, and the issue exists in the commit that adds the mark.
- A test of a behaviour lacked by nature, of the five kinds in Names, is under `cfg` for the systems
  that have it, with its reason in a comment and no issue. Where one test holds several cases, only the
  case lacked is left out: the `mkfifo` case of two tests, the mode-bit cases of
  `a_failure_after_the_first_write_exits_one` and `a_refusal_before_the_write_leaves_no_record`, and the
  `chmod` of `check_staged_as_a_pre_commit_hook_judges_the_index_the_commit_records`. That last test is
  of development workflow, and the probe left it out: step 7 runs it on Windows, repairs the editor's
  path if that is all it needs, and otherwise skips it, naming an issue it then opens.
- Every other test runs on the three systems. The nine under `cfg(unix)` that use only a symbolic
  link lose the mark, so the two items clippy refuses on Windows are used there and need none:
  one of `path@check@src/agents.rs`, of `path@check@src/commands/history.rs`, of
  `path@check@src/commands/mod.rs` and of `path@testing@tests/model_over_mocks.rs`, and five of
  `path@testing@tests/binary.rs`. `path@xtask@tests/mutate_bin.rs`, which needs git and `true` alone,
  loses its own.
- `path@xtask@tests/gates_bin.rs` loses its own: its stub `cargo` becomes the binary `fake-cargo`, copied
  as `cargo` with the system's executable suffix into the directory put first on a path joined by
  `std::env::join_paths`. On Unix it is linked, not copied, for
  `issue@xtask@gates-binary-tests-fail-when-a-fake-program-they-wrote-is-busy`. Ten of its tests run on the three systems. The two that assert the refusal of a
  pipe, `a_piped_stdout_is_refused_before_any_gate_runs` and
  `a_piped_stdout_is_refused_when_the_actions_variable_is_not_true`, are skipped on Windows naming the
  issue on that refusal: it holds only outside GitHub Actions, where it protects a person or an agent
  who pipes the output, which on Windows is development. Rival: the refusal built on Windows now.

The owner ruled the permission of the first in round 5, three kinds of the second in round 8 and the two
others in round 10, D4. Nearest rival: an issue for each test lacked by nature, which no work could close.

### #drafts-stay-unchecked

CI does not check a draft pull request. The owner ruled it in round 9, arguing that the workflow has a
branch pushed at several points while its work is under way, that a run over such a state has little
value, and that it is one more thing an agent watches. Nearest rival: a run on each push of a draft,
without `--require-rebased`. With three systems a failure that a session's machine cannot show, found
once the pull request is ready, is more frequent; the cross gate reduces it to failures of tests. That
a skipped run fills the history and reads as passing is
`issue@knowledge-architect@a-ci-run-skipped-on-a-draft-reads-as-passing`.

### #ci-differs-by-flags

Where CI and a local run differ in what they run, the difference is a flag CI passes. Nearest rival: a
run that reads a variable CI sets, which loses since the command copied from CI then no longer reproduces
CI's verdict exactly, a possible source of unexpected results, as the owner argued in round 9 and as
`design@gates@ci-flag-for-ci-only-requirements` argues of the variable `CI` for `--ci`. How many flags
carry the differences is that head's question, not this one's. A gate that the list leaves out on a
machine of the target's own system is left out in CI and locally alike, so it is no difference between
them.

### #platforms-served

A head of the root says: the build of the workspace and its gates run on Linux, Windows and macOS, and
CI blocks on the three. What fails in development on Windows is in the issues step 7 opens. The head
carries the
reading of the goals that the closed issue argued: `goal@every-target-project-can-adopt-it` and
`goal@serves-one-kind-of-project` exclude no platform in their conditions, so a target project
developed or delivered on Windows is served; the rival reading, that a platform is one more thing
knowledge-architect may ask of a project, finds no such ask in their text. The owner reported a consumer project
that the owner wants to build on Windows soon. Not in the head: leaving development on Windows for later
is an order of known work, which `goal@serves-one-kind-of-project` gives knowledge-architect's owner, and
no narrowing of the conditions.

### #asks-discussion-put-off

The discussion of `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` is not held before
this branch lands. The cross gate is opt-in and asks nothing of a project. The issue gains the
instance and keeps its re-entry point. The owner ruled it in round 8, as for the gate `doc`.

### #single-branch-spec

One branch and one pull request, planned by this spec, with the CI jobs first and the port after them
until they pass. The owner ruled it in round 7. Nearest rival: a milestone of several pull requests,
the session's proposal.

## Mapping tables

`cross_clippy` over a target, the system compared being the target's, read from the triple, against
`std::env::consts::OS`:

| the triple | its system | on a machine of that system | elsewhere |
| --- | --- | --- | --- |
| holds the segment `windows` | `windows` | nothing | the gate |
| holds the segment `darwin` | `macos` | nothing | the gate |
| ends in `linux-<env>`, `<env>` other than `android` or `androideabi` | `linux` | nothing | the gate |
| any other | none | the gate | the gate |

The spelling of a path, `git`'s `named_to_git` and `slashed`:

| the path's character | Unix | Windows |
| --- | --- | --- |
| `/` | itself | `/` |
| `\` | itself, a character of a name | `/` |
| any other | itself | itself |

## Losing alternatives

- A milestone of several pull requests: lost to #single-branch-spec, the owner's ruling of round 7.
- An absolute limit of the jobs' time, 8 minutes as the session proposed: lost to #blocking-jobs, the
  owner's argument of round 2.
- The jobs run after the merge, or not required: lost to #blocking-jobs and #one-required-context.
- A subset of gates on the two systems, and `cargo test` run directly: lost to
  #gates-on-three-systems.
- A dedicated cargo profile, a target directory of its own, a renamed running binary, the dev
  profile: lost to #release-maintenance-build.
- The cross gate in this repository's list alone; the Windows job alone; a real cross compiler: lost
  to #cross-clippy-gate.
- The refusal of `\` and of a drive prefix in a reference and a manifest row,
  #backslash-in-a-pointer, agreed by the owner in round 5 and ruled out on the session's argument after
  the search, by the owner's word of round 8: the purpose does not need it,
  since a document that passes on Linux holds none that resolves; it removes the `skip-files` row by
  which a project on Linux keeps a file whose name holds `\`; and, as a stricter check of past
  commits, it would have the spec merge first. It becomes an issue.
- A temporary `push` trigger in the CI workflow, #iteration-before-ready, superseded by the probe
  branch of "How the work is done": it contradicts `design@knowledge-architect@ci-skips-a-draft`,
  its fields are empty on a push, and its successful run would satisfy the merge predicate.
- A public spelling function in `git`: lost to #path-spelling-home.
- A shared crate, `testing` or `git` as the home of the symbolic-link helper, and a skip where links
  are refused: lost to #symlink-helper-home.
- `index` and the install leaving a file that differs by line endings, `compare` alone changed, and an
  ask that each project declares its line endings: lost to
  #generated-text-compared-without-line-endings.
- `* -text` in `.gitattributes`: lost to #line-endings-of-this-repository.
- The cross gate in the recommended list: lost to #cross-clippy-gate.
- Every `PathBuf` built with `/`, and a change of `Finding` alone: lost to #printed-paths-with-slashes.
- A run that reads a variable CI sets: lost to #ci-differs-by-flags.
- A run of CI on each push of a draft: lost to #drafts-stay-unchecked.
- An error on a listed name that is not UTF-8, outside Unix: lost to #a-listed-path-outside-unix.
- The pipe refusal built on Windows: lost to #platform-skips.
- An issue opened for the modules past one read that have none: lost to
  #no-module-split-in-the-port, on `design@agent-skills@split-not-needed-is-an-issue`.

## Readings

The work reads no external specification that the project implements. What it reads of GitHub, git,
cargo and the `cc` crate is under the threads it bears on.

## Premortem

1. **A product test is marked `cfg(unix)` for convenience, and the coverage of Windows shrinks in
   silence.** Stresses #platform-skips. Named as a cause for the harvest to judge as a tripwire.
2. **The gathering job passes when the matrix was cancelled or skipped.** Stresses
   #one-required-context. Becomes a claim of step 1.
3. **The stub changes a build script's feature, and the cross gate passes wrongly.** Stresses
   #cross-clippy-gate. Dropped: the blocking Windows job runs the system's own clippy, so it is
   reported.
4. **A version of `cc` checks its objects, and the cross gate fails.** Stresses #cross-clippy-gate.
   Dropped: it is loud, and a consumer can opt out, as the owner argued.
5. **A project's release settings make `cargo x` slow.** Stresses #release-maintenance-build.
   Dropped: a request reports it.
6. **New code prints a path with `\`.** Stresses #printed-paths-with-slashes. Named as a cause for the
   harvest to judge.

## Acceptance criteria

None: each check that only the built code can apply would show the work unfinished, and none a
decision wrong, so each is a claim of the step that builds it.

## Implementation sequence

Every step's commits pass the gates on Linux; steps 1 to 6 are run on the three systems by the probe
before step 7 starts.

1. **The CI jobs.** Builds: the matrix job `gates` and the gathering job `check` of
   #gates-on-three-systems and #one-required-context, in
   `path@knowledge-architect@.github/workflows/ci.yml`, and the comment of that file that says why it has
   no `push` trigger, which takes the argument of #iteration-before-ready. Claims:
   - the matrix lists the three systems, its one gates step is the gates command of CI, its checkout
     keeps `fetch-depth: 0` and the head's `ref`, its artifact's name holds the system, and it keeps the
     draft condition and the timeout: a test of `path@xtask@src/gates.rs` reads the workflow, beside
     `ci_runs_the_gates_with_the_ci_flag`; mutations: a system removed, `fetch-depth` removed, the name
     without the system, the draft condition removed;
   - the job `check` needs `gates`, runs `always()` off a draft, and fails on any result but
     `success`: the same test; mutations: `always()` removed, the comparison changed to
     `!= 'failure'`, the `needs` removed.

   Fails alone on: the workflow test. The jobs themselves are seen when the pull request is marked
   ready; before, the probe runs the same commands.
2. **Windows compiles.** Builds: #paths-to-git-with-slashes; the comment of `as_path` and the instance of
   `issue@git@paths-are-decoded-outside-the-listings`, per #a-listed-path-outside-unix; the symbolic-link
   helper's copies; the marks that the compile needs, for the kinds lacked by nature; `.gitattributes` of
   #line-endings-of-this-repository; a changelog entry, since the crates then build on Windows. Claims:
   - the workspace and its tests compile for Windows, with no warning: the cross clippy run by hand
     until step 4 lands it, `cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu`
     with the stub; it fails at main `9f1efd7b`, observed;
   - a path is named to git with `/` on Windows and byte for byte on Unix: unit tests of `named_to_git`,
     whose argument `windows` Linux gives both ways; mutation: the replacement removed;
   - every tracked file is checked out with LF, and a file holding CRLF is committed with LF: a test of
     `xtask` over a repository holding this one's `.gitattributes`, under `core.autocrlf=true`;
     mutation: `.gitattributes` emptied.

   Fails alone on: a compile error or warning for the target.
3. **The release alias.** Builds: #release-maintenance-build, here, in the snippet, in the setup skill's
   sentence that says which alias runs in release, with the installed copy, in
   `path@testing@tests/scaffold.rs`, and a Migration entry of the changelog. Claims:
   - both aliases name `--release`: a test of `path@xtask@src/gates.rs` reads the cargo configuration
     and the snippet; mutation: `--release` removed from either;
   - no gate replaces the running binary, under the condition #release-maintenance-build states: a new run of the probe on Windows ends its `test` gate with no
     removal refused, where the dev profile fails.

   Fails alone on: "Access is denied" on the running binary.
4. **The cross gate.** Builds: #cross-clippy-gate in `path@gates@src/cross.rs`, this repository's
   list, `path@knowledge-architect@rust-toolchain.toml`, `path@gates@README.md`, the crate's
   documentation, and a changelog entry. Claims:
   - the gate's arguments, variables and name, for a target with hyphens: a unit test; mutations: a
     variable dropped, either form of one dropped, `-D warnings` dropped;
   - the mapping table, each row: a unit test of `gate_on`; mutation: the `android` exclusion removed;
   - this repository's list holds the gate after `clippy`, and lacks it on Windows: the list test,
     whose expected names follow `cross_clippy`'s answer; mutation: the insertion removed;
   - the gate reports a Windows-only warning: `#[cfg(windows)]` code with an unused import, added by
     hand and removed, observed failing.

   Fails alone on: a build script's failure under the stub.
5. **Line endings.** Builds: #generated-text-compared-without-line-endings in
   `path@check@src/check/generated.rs`, `path@check@src/commands/mod.rs` and
   `path@check@src/commands/history.rs` and in the build of `agent-skills`, the changelog entry of a check
   made looser, and the instance of
   `issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`. Claims:
   - a generated file that differs by line endings alone is current, and one that differs otherwise is
     not: a unit test of `compare`; mutation: the normalisation removed from either side;
   - a project checked out under `core.autocrlf=true` passes `check`, `check --fix` writes no generated
     file and refuses nothing there, and `index --staged` names no working copy out of date: a test over a mock project or a
     scaffold, which clones it with that setting, on Linux too; it fails at main, observed as 3
     findings;
   - the shipped text of a CRLF checkout is the shipped text of an LF one, `SNIPPETS` included: unit
     tests over CRLF text of the passes of `path@agent-skills@src/render.rs` and of `snippet_list`, which
     is there since a build script has no test target; mutation: the normalisation removed.

   Fails alone on: the autocrlf test.
6. **Printed paths.** Builds: #printed-paths-with-slashes and #path-spelling-home, a Migration entry for
   the printed text and a New features entry for `slashed`. Claims:
   - `slashed` rewrites the separator under Windows' rule and nothing under Unix's: a unit test of
     `respelled`; mutation: the rule swapped;
   - no project-relative path is printed otherwise in the crates the thread names: read over the corpus
     of the calls that print a path, `.display()`, `to_string_lossy` and `Debug` formats, each one an
     absolute path or one the user gave, a refused row as written, or converted; a search
     is a lead, per point 3 of `skill@knowledge-architect-planning@working-a-slice`. The commit's message
     holds the argument of #no-module-split-in-the-port;
   - a test whose one cause is a printed `\` passes on Windows: the probe.

   Fails alone on: a test whose only difference is a separator.
7. **The tests, until the three jobs pass.** Builds: the JSON helpers' escaping; the remaining causes
   of the table, `strip_prefix("/")` of `new_component_accepts_a_relative_root` by the path's components,
   and, if the two drives then stop it, a directory on the checkout's drive; #mutate-tests-on-windows;
   the macOS failures, each side of a path comparison canonicalised; `fake-cargo`, tied to the checkout
   as `path@xtask@tests/checkout.rs` asks of every target; the skips of #platform-skips, each with its
   issue; the remaining issues of "Outside the work", and the instances added to the issues that "What is
   already decided" names. Claims: every test of the workspace passes or is skipped by #platform-skips on
   the three systems, by the probe; every skip is listed, with its reason or its issue, in the commit's
   message; `--patch` under `core.autocrlf=true` gives the mutant the patch names: the five tests of
   `path@mutate@tests/modes.rs` that fail under it on Linux, observed. Fails alone on: the probe's test
   run.
8. **The heads conform.** Builds, with no word of the owner changed and no argument written that the head
   lacks; an edit that would write one is the harvest's, from its thread:
   - the subject "4. How work reaches main" of `register@knowledge-architect@design` gives the six heads
     that the issue of the root `the-subject-on-how-work-reaches-main-holds-tooling-heads` lists to a
     subject of their own, in a commit that moves headings alone, which closes the issue;
   - `design@xtask@gates-list-primary-home` is split, in `register@xtask@design`: the list's home keeps the slug, the title and the
     first paragraph; the rule that CI and a local run differ by flags alone, with the reason of each flag and
     what reporting does with no flag, goes to a head that takes
     the slug of `thread@gates-on-windows-and-macos@ci-differs-by-flags`, argued as that thread says;
     cargo's own `--locked` ahead of
     the alias, whose argument is measured, goes to `design@xtask@ci-passes-locked-ahead-of-the-alias`.
     Of the texts above, `path@knowledge-architect@CLAUDE.md` and the opening comment of
     `path@knowledge-architect@.github/workflows/ci.yml` cite the flags, and are re-pointed; the others
     cite the list;
   - `design@knowledge-architect@ci-skips-a-draft` loses its sentence on an approval; its argument is the
     harvest's;
   - `design@agent-skills@xtask-pins-checker`: retitled "A Rust project runs the tools of
     knowledge-architect it pins, and its other repeated tasks, from one maintenance crate", with
     `design@agent-skills@exact-pin` and `design@knowledge-architect@version-lockstep` cited for the
     pin it already states. The sentence that derives from its goal stays as it is, though its words go
     past the goal's: rewording it would write the constraint taken from a goal;
   - `design@check@generated-files-are-pure`: the closing line that cites
     `rejected@model@index-last-change-date`, which its words cover and which says that it loses to it;
   - `design@model@a-file-register-index-is-rows`: the rows from the walked entries go to a head with
     slug `index-rows-from-walked-entries`, and the head is retitled "A register's generated index is a
     banner, a count and one row per entry, with no summary and no date"; its part on the byte
     comparison stays until the harvest. Of the texts above, the paragraph "The rows come from the walked
     entries" of `path@model@src/index.rs` is re-pointed;
   - `design@check@installed-files-compared-by-bytes`: its title says that line endings are normalised.

   The auditor reads that last head, `design@agent-skills@content-mirrors-the-install-layout` and
   `design@xtask@ci-builds-the-tip-alone` first. Each head the step creates or retitles is judged alone by
   the auditor briefed to check. Claims: `cargo klarch check` passes, and each text listed above reads
   right against the head it now names. Fails alone on: a citation left on a head that no longer holds
   what it cites.
9. **The harvest**, below, with the probe branch's deletion and the pull request marked ready once the
   review is done.

## Order rationale

- 1 before 2: the owner's order of round 7, the jobs first.
- 2 before 3: the alias is seen to work on Windows only once Windows compiles.
- 3 before 4: the alias ends the swap that a changed list would otherwise meet on each run of
  `cargo x gates` here.
- 4 before 5: each later step is judged for Windows by the cross gate on the machine that writes it.
- 5 before 6: nothing orders them; the comparison is the smaller, and each fails alone.
- 6 before 7: the printed paths are the largest cause of the failures step 7 reads.
- 7 before 8: a head conformed while the work still changes the code it cites is read twice.
- 8 before 9: the harvest writes into heads that conform.

## Defaults awaiting the owner

All are ruled.

- **D1**, #drafts-stay-unchecked: CI keeps not checking a draft. Ruled in round 9, with the owner's
  argument.
- **D2**, #ci-differs-by-flags: no fork, since the rival loses on an argument, as the owner pointed out in
  round 9; the thread closes argued.
- **D3**, #line-endings-of-this-repository: `* text=auto eol=lf` over `* -text`. Ruled in round 10: the
  owner accepted the session's default.
- **D4**, #platform-skips: the two tests for Linux alone admitted as lacked by nature. Ruled in round 10:
  accepted.

## Harvest

| what | home |
| --- | --- |
| #gates-on-three-systems | `design@xtask@gates-list-primary-home` |
| #blocking-jobs | a head of the root, which cites `goal@every-target-project-can-adopt-it` |
| #one-required-context | a head of the root, its argument resting on how GitHub counts a skipped job |
| #ci-differs-by-flags | the head step 8 cuts out in `register@xtask@design`, citing `design@gates@ci-flag-for-ci-only-requirements` for the rival and its flaw |
| #drafts-stay-unchecked | `design@knowledge-architect@ci-skips-a-draft`, its argument, its rival and its cost with three systems |
| #release-maintenance-build | a head of `register@agent-skills@design`, with a pointer from `design@agent-skills@xtask-pins-checker` |
| #cross-clippy-gate | a head of `register@gates@design`, with pointers from `design@gates@recommended-gate-list`, which says that a gate taken in can be offered beside the list, and from `design@knowledge-architect@toolchain-is-pinned`; the reason for this repository's target in `design@xtask@gates-list-primary-home` |
| #line-endings-of-this-repository | a head of the root, in the weak form, its argument turning on git's reading of the attributes |
| #generated-text-compared-without-line-endings, the comparison | a head of `register@check@design`, reversing the byte comparison of `design@model@a-file-register-index-is-rows`, which is judged as a `rejected` entry of `check`, with a pointer from `design@check@generated-files-are-pure` |
| the same thread, the build | `design@agent-skills@content-mirrors-the-install-layout`, or a head of `register@agent-skills@design` where its rival makes it another decision |
| #paths-to-git-with-slashes | a head of `register@git@design` |
| #a-listed-path-outside-unix | the rewritten `design@git@a-path-is-bytes`, and `issue@git@paths-are-decoded-outside-the-listings` |
| #printed-paths-with-slashes | a head of the root, since it binds the code of several Components that no intermediate one encloses |
| #path-spelling-home | a head of `register@model@design` |
| #symlink-helper-home | a head of the root, cited at each copy |
| #no-module-split-in-the-port | the instances of step 7 and the message of step 6; no head |
| #mutate-tests-on-windows | `core.autocrlf=false` in a head of `register@mutate@design`, cited at its three sites; the test repairs, comments |
| #platform-skips | a head of the root |
| #platforms-served | a head of the root, to which `issue@check@a-linked-manifest-splits-the-readers` and `issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux` are re-pointed |
| #asks-discussion-put-off | the instance in `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`; no head |
| #single-branch-spec, #backslash-in-a-pointer, #iteration-before-ready | no home: the first leaves with the spec, the second is an issue, and the third's argument is the comment of the CI workflow |
| `design@xtask@ci-builds-the-tip-alone` | its command |
| the losing alternatives, and each rival that "Decided design" names | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` |
| premortem causes 1 and 6 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` |

The new heads of the root sit where their subject is: the CI heads in "How work reaches main"; line
endings, skips, platforms and the link helper in the subject step 8 makes; the printing rule in "What the
tools and the workflow hold to together".

## Later consequences

- Development on Windows, the owner's later aim, starts from the issues step 7 opens; the owner
  judges that it probably needs developing from Claude Code on Windows. The order of that work is the
  owner's, in docs/roadmap.md.
- A second target of the cross gate, such as one for macOS, is one line of the list and one target of
  the toolchain file.
