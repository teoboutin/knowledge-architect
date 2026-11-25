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

The threads and the arguments are in [deliberation](deliberation.md); the record search's generated
lines in [the discussion's search](search/discussion-1.txt); the transcript in
[the transcript](transcripts/1.txt), in which paths under a home directory, identifiers of sessions
and agents, the harness's notifications and details of the owner's other projects are removed.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, "The work of a spec", with
`skill@klarch-development` and `skill@knowledge-architect-development` for the Rust code, and
`path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", for the setup skill's text.

**Iterating on the other systems.** CI does not run on a draft, per
`design@knowledge-architect@ci-skips-a-draft`, and no trigger is added to the CI workflow. The
probe branch `windows-macos-probe` is on origin, with no pull request, and never merges. To run a
state of the work on the three systems, the probe branch is rebuilt as that state plus one commit
that adds its workflow, which triggers on a push to that branch, and pushed with force. Its workflow
runs, on each system, the gates as CI does but without `--fail-fast`, and
`cargo --locked test --locked --workspace --no-fail-fast` with `KNOWLEDGE_ARCHITECT_CI` set to `1`,
since the `test` gate stops at the first test binary that fails. The branch's present head holds
rough repairs, which are evidence and not the shape of the work. The branch is deleted at the end,
on origin and locally. A push to it, as any push, follows
`design@knowledge-architect@push-after-the-commits-check`.

## Names

- **the three systems**: the GitHub-hosted runners `ubuntu-latest`, `windows-latest` and
  `macos-latest`, as they stand when the work runs.
- **the probe**: the branch above, and its two workflow runs, 45592208005 and 45606918811.
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
- **lacked by nature**: a behaviour a system does not have at all. Three kinds are admitted: Unix
  mode bits, a named pipe, and a file name the system refuses.

## What the work is

**The purpose**, as the owner stated it in round 2: a project that depends on knowledge-architect
keeps building on Windows and macOS, to deliver its product there, and tests itself there with its
own gates command, with no second place that decides what runs. The gates library exists to unify
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
  `query` 44, `check` 38, `create` 36, `git` 9. Most print a project-relative path, which
  `PathBuf::join` builds with `\` on Windows. `path_text` in `path@query@src/graph.rs` writes `/`.
  `impl Display for Site` in `path@model@src/entity.rs` prints most sites of `query`.
- `apply_in` in `path@mutate@src/run.rs` runs `git apply` in a scratch repository under the user's
  `core.autocrlf`.
- No `.gitattributes`. No tracked file holds a carriage return.
- 24 test functions do not compile for Windows, which blocks 8 test targets; 18 of them use a
  symbolic link and nothing else of the Unix API.

Measured by the probe, with rough compile repairs and every file checked out as committed:

| | Linux | Windows | macOS |
| --- | --- | --- | --- |
| the tests alone, to the end, cold | 2:45, all pass | 6:37, 118 fail | 5:18, 5 fail |
| the gates `check`, `commits`, `clippy` and `doc` together, cold | 71 s | 70 s | 44 s |
| the gates command in the dev profile | passes | the `test` gate fails: "failed to remove file … xtask.exe", "Access is denied. (os error 5)" | fails on a test |
| the gates command in the release profile | passes | passes that point, then fails on tests | fails on a test |
| `path@testing@tests/scaffold.rs` | passes | passes | passes |
| a project `init --rust` writes, its maintenance crate given a test and a dev-dependency, running its own gates, with `x` in the dev and in the release profile | both pass | both pass | both pass |

On Windows, `fmt`, `check`, `commits` and `doc` pass, and `clippy` fails on two items that only code
under `cfg(unix)` uses: the constant `CORRECT_SPAN_REPAIR` of `path@testing@tests/binary.rs` and
`use super::*` of the module `checker` of `path@testing@tests/model_over_mocks.rs`.

The causes of the 118 failures, predicted by a simulation on Linux over a patched standard library
(107 of them) and by reading (10); a test can have two causes:

| cause | tests, as the simulation counted them | where |
| --- | --- | --- |
| a printed path holds `\` | 62 | the product, but for one test of `path@xtask@src/changelog.rs` |
| a path written raw into a JSON string | 29 | the helpers `transcript_case`, `copy_rendered` and `transcript_config` of `path@testing@tests/binary.rs` |
| `core.autocrlf` in a repository a test or the product makes | 14 | 8 tests through `apply_and_reverse` of `path@mutate@tests/harness.rs`, 1 in `path@mutate@src/run.rs`, 5 in `path@mutate@tests/modes.rs`; one of the 5 printed `SURVIVED` for a patch that changes nothing |
| a path joined, then compared or sent as text | 5 or 6 | `stage_generated`, `ignore_query` |
| the liveness of a run | 3 | `a_record_this_process_claimed_does_not_read_as_owned_by_a_live_run`, `restore_refuses_while_the_owning_run_is_alive`, `a_record_over_an_unmutated_file_is_cleared_by_restore` |
| a name the system refuses | 5 | a line feed in four tests, `a:b` in `a_symlink_with_a_refused_name_is_one_finding` |
| a named pipe, `ln -s` | 2 | `a_target_through_a_symlink_in_git_or_not_a_file_is_refused`, `the_restore_writes_through_no_symlink_the_command_left` |
| the temporary directory on another drive than the checkout | 1 | `new_component_accepts_a_relative_root` of `path@testing@tests/extension_api.rs`, met by no prediction |
| a short file name in the temporary directory | 1 | `a_patch_run_works_in_a_linked_worktree` of `path@mutate@tests/modes.rs` |

Without every file checked out as committed, 65 more tests fail (simulated on Linux). The 5 macOS
failures: the liveness tests above, and `a_patch_run_works_in_a_linked_worktree` and
`init_lists_the_repository_the_kept_gitignore_and_the_enclosing_project` of
`path@cli@tests/init.rs`, which compare a path given with the path its symbolic link resolves to.
Two whole test files are `cfg(unix)`: `path@xtask@tests/gates_bin.rs`, 12 tests, and
`path@xtask@tests/mutate_bin.rs`, 3 tests.

The closing of `the-workspace-does-not-build-or-run-its-tests-on-windows`, an issue of the root: the
commit that adds this spec closes it, since the spec schedules its work, per
`skill@knowledge-architect-planning@spec-or-milestone`. Its reading of the goals is under
#platforms-served; the liveness of a run is `issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`;
its lead that a directory holding an open file may not be removable matched no failure of the probe,
and its lead that git's read-only objects block a removal is ruled out by the standard library's
removal on Windows, which ignores that attribute (read).

**Outside the work**, each an issue that step 7 opens unless it says otherwise:

| anchor | what | evidence |
| --- | --- | --- |
| mutate | the liveness of a run on Windows and macOS | `issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`, opened with this spec |
| mutate | on Windows: `--file` with `\` or a rooted path; program lookup that skips `.cmd` files and finds System32 first; a multi-line `--replace` with LF over a CRLF file; a patch's line endings against the file's; `.git` compared case-sensitively, on macOS too | read; the two on line endings run on Linux over CRLF files |
| transcript | `HOME` alone names the directory, which PowerShell does not set; a session of a removed directory is dropped, since a canonical root holds a verbatim prefix | read |
| gates | no pipe is refused outside Unix, a divergence from `design@gates@gates-refuse-a-pipe` | read |
| query | `worklist` prints verbatim paths and `agent-<n>\report.md`, which the record-search workflow's patterns do not match | read |
| agent-skills | the installed shell commands in PowerShell, Claude Code's primary shell on Windows where its tool is on, and in zsh: heredocs, `SHELLOPTS`, and `--version =<version>` under zsh | the harness's documentation read; zsh not run |
| model | a path reference, a relative link or a manifest row holding `\` or a drive prefix is accepted on Windows and reported missing on Linux, and `C:/…` stops the run there; refusing it reaches `skip-files` rows, the way a project keeps a file of such a name | read |
| git | git's ignore match ignores case on a case-insensitive disk, so a reference passes locally and fails on Linux | read |
| create | on a case-insensitive disk `init` keeps a `Readme.md` that the check then misses; on macOS an absolute DIR under /tmp is refused as outside the project | read |
| the root | a pull request's first CI run starts with no cache, since nothing runs on main to fill one | run 45581213477, "No cache found" |

Whether Windows projects are asked to use Git Bash is the owner's to rule, with the agent-skills row.

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

Rewritten by the harvest, with every text that references each, which the harvest reads:

| decision | what changes | texts that reference it |
| --- | --- | --- |
| `design@xtask@gates-list-primary-home` | split in step 8, since it bundles the list's home with the rule that CI and a local run differ by flags alone; the list gains the cross gate; CI runs on three systems behind a gathering job; the cross gate is absent on a machine of the target's own system | `skill@klarch-development@gate-in-this-repository`, `path@knowledge-architect@CLAUDE.md`, `path@knowledge-architect@README.md`, `register@gates@design`, `design@knowledge-architect@ci-skips-a-draft`, `path@xtask@README.md`, `path@xtask@src/gates.rs` |
| `design@gates@recommended-gate-list` | a gate of the list is told from a gate the library offers beside it | `register@agent-skills@design`, `register@check@design`, `register@create@design`, `register@gates@design`, `register@xtask@design` |
| `design@knowledge-architect@ci-skips-a-draft` | its cost is restated: a failure of two systems is found only once the pull request is ready, reduced by the cross gate and the probe | `path@knowledge-architect@CLAUDE.md`, `register@knowledge-architect@design`, `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements` |
| `design@agent-skills@xtask-pins-checker` | the aliases' profile, its argument and its cost | `path@agent-skills@content/skills/setup/SKILL.md`, `register@agent-skills@design`, `register@create@design`, `register@knowledge-architect@design`, `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head` |
| `design@knowledge-architect@toolchain-is-pinned` | the file names a target too | `issue@knowledge-architect@the-subject-on-how-work-reaches-main-holds-tooling-heads` |
| `design@check@generated-files-are-pure` | a file that differs by line endings alone is current | `register@check@design`, `path@cli@README.md`, `register@model@design`, `register@model@rejected`, `issue@knowledge-architect@a-conflict-in-a-generated-file-is-resolved-by-hand` |
| `design@model@a-file-register-index-is-rows` | its bytes are the contract but for line endings | `register@check@design`, `path@cli@README.md`, `register@model@design`, `path@model@src/index.rs`, `issue@knowledge-architect@command-output-is-not-declared-a-contract` |
| `design@git@a-path-is-bytes` | outside Unix a path is text, named to git with `/` | `issue@git@paths-are-decoded-outside-the-listings`, `path@git@src/git.rs` |

Touched by the harvest without reversing or rewriting their decisions: none known; step 8 is filled
by the design-record audit over the eight heads above.

Entries of the record search whose outcome changed something:

- `design@git@an-operation-in-git-s-terms-is-the-library-s`, `design@knowledge-architect@git-crate`
  and `design@knowledge-architect@query-crate`: the printing function is in `model` and `git` keeps
  a private spelling, so none of the three changes (#path-spelling-home).
- `design@knowledge-architect@a-copy-of-the-writer-per-crate`: argues another case, so the copies of
  the symbolic-link helper have their own argument (#symlink-helper-home).
- `design@check@index-takes-no-dry-run`, `design@check@index-staged-write` and
  `design@agent-skills@overlay-by-separate-skills`: the writing commands stay as they are, so none
  of the three changes (#generated-text-compared-without-line-endings).
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
- `goal@the-owner-s-word-is-verified`: the skip criterion was put back to the owner, round 8
  (#platform-skips).
- `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`: trigger met, put off by the owner
  in round 8; step 7 adds the instance (#asks-discussion-put-off).
- `issue@agent-skills@a-one-time-technical-setup-has-no-place-a-project-gets-it-from`: trigger met;
  a project meets the cross gate in `path@gates@README.md`, the documentation of the function, one
  sentence of the setup skill and the changelog, and the alias in the snippet, the setup skill and
  a Migration entry. The issue stays open, and step 7 adds the instance.
- `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps` and
  `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`: the heads
  listed here are written by the harvest alone, and the one bundle is split in step 8. Both stay
  open, and step 7 adds the instance.
- `issue@git@git-rs-is-past-one-read`, `issue@check@history-rs-is-past-one-read`,
  `issue@model@entity-rs-is-past-one-read`, `issue@model@manifest-rs-is-past-one-read`,
  `issue@model@resolver-rs-is-past-one-read`,
  `issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read`,
  `issue@testing@binary-rs-is-past-one-read` and `issue@transcript@reader-rs-is-past-one-read`: no
  split here; step 7 adds this work as an instance of each whose file it changed
  (#no-module-split-in-the-port).
- `issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`: corrected in a commit
  of its own on this branch; the work does not touch what `index --staged` does.
- `issue@check@a-linked-manifest-splits-the-readers`: its citation of the closed issue is re-pointed
  to this spec, and at the harvest to the head of #platforms-served.

The other verdicts: [the discussion's search](search/discussion-1.txt).

## Criteria

### No platform condition is added to a target project `##no-platform-condition`

Binding, from `goal@every-target-project-can-adopt-it` and `goal@serves-one-kind-of-project`. Met by
#platforms-served.

### No workflow file decides which gates run on a system `##gates-decided-by-the-list`

Binding, stated by the owner in round 2: the gates library and the project's list decide. Met by
#gates-on-three-systems.

### A failure on any of the three systems blocks a merge to main `##any-system-blocks-the-merge`

Binding, stated by the owner in rounds 2 and 5. Met by #blocking-jobs and #one-required-context.

### A project that depends on the crates builds on Windows `##dependents-build-on-windows`

Binding, stated by the owner in round 2. Met by #paths-to-git-with-slashes and
#release-maintenance-build.

### A test is skipped on Windows only where it covers development workflow or a behaviour lacked by nature `##windows-skips-are-bounded`

Binding, stated by the owner in round 5 and widened by the owner in round 8 to the three kinds
lacked by nature. A skip for development workflow names its issue; one for a behaviour lacked by
nature gives its reason and no issue. Met by #platform-skips.

### A merge waits for the slowest job only `##wait-is-the-slowest-job`

Weighed, from the owner's concern of round 1 and the owner's argument of round 2 that the jobs run
in parallel. Met by #gates-on-three-systems and #blocking-jobs.

## New names, in one place

```text
planned names, with their file
  cross_clippy(target: &str) -> Option<Gate>   crates/gates/src/cross.rs, re-exported by lib.rs
  the gate `clippy-<target>`                   what cross_clippy builds
  slashed(path: &Path) -> String               crates/model/src/finding.rs, re-exported by lib.rs
  the jobs `gates` (matrix) and `check`        .github/workflows/ci.yml
  the artifacts `gate-logs-<os>`               .github/workflows/ci.yml
  symlink(target, link) in test code           one copy per package: check, cli, git, mutate,
                                               testing, transcript
  .gitattributes                               the project's root
existing names the work changes
  path_bytes, object_name, ignore_query        crates/git/src/git.rs
  compare                                      crates/check/src/check/generated.rs
  path_text                                    crates/query/src/graph.rs, replaced by slashed
  gates()                                      tools/xtask/src/gates.rs
  x                                            .cargo/config.toml, snippets/cargo-config.toml
```

## Decided design

### #gates-on-three-systems

The job `gates` runs the gates command of CI in a matrix over the three systems, with
`fail-fast: false`, the project's whole list on each. Nothing in the workflow names a gate. Argument:
a choice of gates written in a workflow is a second place that decides what runs, which the criterion
excludes, and the gates other than `test` take about 70 s cold on Windows. Nearest rival: a subset of
gates on the two other systems, the owner's first ask, which the owner retracted on the purpose stated
in round 2. `cargo test` run directly lost on the same ground, and hides a defect of `gates` itself.

Conditions: each matrix job keeps `fetch-depth: 0` and the pull request's head as its `ref`, or
`commits` judges a shorter range with no error; each names its artifact for its system, since one run
refuses two artifacts of one name (from the action's documentation as remembered, not run); the step
that runs `nproc`, which macOS lacks, is replaced by one that every system has; the steps run under a
shell every system has.

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
`check` and `commits` run the running binary itself, which cargo finds fresh: the probe ran such a
project's whole gates on the three systems. Costs: a release rebuild of the maintenance crate after an
edit of it or of a crate it links, and, in a project whose release settings are slow, those settings.
The setup skill's sentence that `cargo klarch` alone runs in release mode is repaired in the same
edit. Nearest rival: a cargo profile for the maintenance crate, which keeps a project's release
settings out and builds its dependencies once more; it lost since the `klarch` alias already builds
that crate in release in every consumer. A target directory of its own lost: its path is read from
the working directory. A library that renames or copies the running executable lost: the file has two
hard links.

### #cross-clippy-gate

`gates` exports `cross_clippy`, which takes a target and returns one gate, or nothing where the
machine's own system is the target's, by the mapping table below. The gate is named
`clippy-<target>` and runs `cargo clippy --workspace --all-targets --target <target> -- -D warnings`,
resolving dependencies, with the distiller `WithoutProgress`. Its `envs` set `CC_<target>`,
`CXX_<target>` and `AR_<target>`, the target's hyphens written as underscores, to `true`, and
`RUSTC_WRAPPER` to the empty string, since `sccache` refuses the stub. Clippy never links, so nothing
compiled is missed. A project adds the gate to its list, so it is opt-in and no flag is added; the
fields are public, so a project with a real cross compiler removes the variables. It sits in a module
of its own.

This repository's list gains the gate for `x86_64-pc-windows-gnu`, after `clippy`, and
`path@knowledge-architect@rust-toolchain.toml` declares that target, which rustup installs on first
use (run, in an isolated setup). No gate for macOS: no code here is particular to it.

Measured on the owner's workstation: 4.1 s cold and 1.5 s after an edit; 27 MB downloaded and 133 MB
on disk per toolchain; 487 MB per target directory. Over 16 crates that build C, 13 pass with the
stub and 1 without it; `openssl`, `openssl` vendored and `aws-lc-rs` fail. The `-msvc` target fails
with the stub, so code particular to `-msvc` is not judged. What it finds is compile errors and
lints, not test failures. What is documented for a consumer: those limits; that the gate rests on the
`cc` crate accepting objects that were never made, which nothing documents; and that a build script
which probes the compiler to choose a feature can be answered wrongly.

The owner took it in, opt-in, in round 6, and reported that a consumer it starts failing for opts out
and loses little. Nearest rival: the gate in this repository's list alone, under which each consumer
finds the stub and the `sccache` refusal again. The Windows job alone lost: it reports once the pull
request is ready. A real cross compiler asked of every machine lost on its installation.

### #line-endings-of-this-repository

`.gitattributes` holds `* -text`: every file is checked out as committed. Nearest rival:
`* text=auto eol=lf`, which also rewrites on commit; no tracked file holds a carriage return, so the
two differ only for a file added later, and the session chose no rewriting. The file does not hide
from CI what a consumer meets: the test of step 5 checks out under `core.autocrlf`.

### #generated-text-compared-without-line-endings

`compare` finds a generated file current when it differs from the regenerated text by line endings
alone, as the installed files are judged, through `lf` where it is. `index`, `index --staged`,
`check --fix` and `install-agent-skills` write as today, where bytes differ: rewriting a CRLF file is
harmless. Without it, the `check` gate of a consumer fails on a fresh clone under Git for Windows'
default, measured on Linux under `core.autocrlf=true`: 16 indexes here and 3 in a new project.
Nearest rival: an ask that each project declares its line endings, which adds an ask and leaves the
failure until it is followed. A wider shape, under which the writing commands leave such a file, lost:
it reverses three heads to avoid a harmless rewrite.

The build of `agent-skills` gives one text whatever the line endings of its checkout: `build.rs`
normalises what it reads, and `SNIPPETS` holds the normalised text and not the file. Measured: a
checker built from a CRLF checkout ships stray carriage returns, 22 findings that the install does
not clear.

### #paths-to-git-with-slashes

Outside Unix, `path@git@src/git.rs` names a path to git with `/`, by one private spelling used by
`path_bytes`, `object_name` and `ignore_query`, and by the user messages of that file that print a
project-relative path. `path_bytes` returns `Cow<'_, [u8]>` on every system. On Unix nothing changes:
a path stays bytes. `object_name` and `ignore_query` decode lossily today, which
`issue@git@paths-are-decoded-outside-the-listings` keeps. Pathspecs given as arguments, `git_tracks`
in `path@check@src/commands/mod.rs` and `refuse_ignored` in `path@create@src/command.rs` keep their
spelling: git for Windows normalises `\` there (read in git's source), and `check` passed on Windows
in the probe. Nearest rival: the compile repair alone, which leaves `index --staged` refused.

### #printed-paths-with-slashes

A project-relative path that `model`, `check`, `query` or `create` prints, or writes in a repair it
suggests, is spelled with `/` on every system, by `slashed` in place of `.display()`. An absolute
path is printed as the system writes it, and a manifest row the model refuses is printed as written.
`git` spells its own by its private function. `transcript` and `mutate` are development workflow and
keep `.display()`. Argument: a suggested repair such as `path@<anchor>@<dir>\<file>` passes on Windows and
fails on Linux, and 62 tests fail on the mix. Nearest rival: every `PathBuf` built with `/`, over
more sites of `join` and `push`, each one a place to forget. A change of `Finding` alone lost: other
sites print, and a finding's text can hold `\` as itself. The guard is the Windows job.

### #path-spelling-home

`slashed` is public in `model`, beside `escaped`, and `check`, `query` and `create` call it; it
replaces `path_text`. It rewrites the separator on Windows alone, where a name cannot hold `\`.
Nearest rival: one public function of `git`, which every one of these crates depends on; it lost on
`design@git@an-operation-in-git-s-terms-is-the-library-s`, under which who calls a function is no
ground, and printing a path is no operation on a repository.

### #symlink-helper-home

The helper that makes a symbolic link on both systems, a file or a directory, is a copy in the test
code of each package that uses it: `check`, `cli`, `git`, `mutate`, `testing`, `transcript`. It fails
where the machine refuses links; nothing is skipped. A relative target is written with `\` on Windows.
Argument: no crate may depend on `testing`, making a link is no operation in git's terms, `mutate`
depends on no crate of the workspace, and a crate for it would take no decision. Nearest rival: a skip
where links are refused, under `KNOWLEDGE_ARCHITECT_CI` as
`design@gates@ci-flag-for-ci-only-requirements` has it, whose rule would be copied six times for a
development machine that this work does not serve.

### #no-module-split-in-the-port

No module is split. The work writes into modules past one read; nearly every edit replaces one call
at one site, so it pushes no signal it does not find, and a move beside edits across every part of a
module would hide them. `cross_clippy` goes in a module of its own in `gates`, since
`path@gates@src/gates.rs` holds two parts that change apart and the gate has a ground of its own. Each
standing issue on a module past one read that the work writes into gains the work as an instance, and
no issue is opened for `tree.rs`, `references.rs` and `registers.rs`, whose signals are weighed here.
Nearest rival: a split of each before the work, a move larger than the work.

### #mutate-tests-on-windows

`mutate` is development workflow. Its tests all pass on macOS. On Windows a test is repaired where
the repair is cheap, and otherwise skipped naming its issue. The repairs: the two liveness tests that
write pid 1 write the pid of a live child of the test, and the test that writes pid 0 writes the pid
of a child that was waited for; the scratch repository of a `--patch` run, `apply_and_reverse` and
`linked_worktree` run git with `core.autocrlf=false`, the first in the product, since a false
`SURVIVED` was measured; both sides of a path comparison are canonicalised. On Windows the liveness
tests that need a live owner are skipped, naming
`issue@mutate@a-run-s-liveness-is-read-wrongly-outside-linux`. Nearest rival: the lock that issue
names, in the product, a change of `mutate`'s behaviour that the purpose does not need.

### #platform-skips

- A test of development workflow is skipped on Windows only, with a comment citing its issue.
- A test of a behaviour lacked by nature, of the three kinds in Names, is `cfg(unix)`, with its reason
  in a comment and no issue. Where one test holds several cases, only the case lacked is left out.
- Every other test runs on the three systems. A test now under `cfg(unix)` that uses only a symbolic
  link loses the mark, and `path@xtask@tests/mutate_bin.rs`, which needs git and `true` alone, loses
  its own.

The owner ruled the first in round 5 and, on the session's question, the second in round 8. Nearest rival: an issue for each test lacked by nature, which no work could close.

### #platforms-served

A head of the root says: the build of the workspace and its gates run on Linux, Windows and macOS, and
CI blocks on the three; development on Windows is outstanding work, held in issues. It carries the
reading of the goals that the closed issue argued: `goal@every-target-project-can-adopt-it` and
`goal@serves-one-kind-of-project` exclude no platform in their conditions, so a target project
developed or delivered on Windows is served; the rival reading, that a platform is one more thing
knowledge-architect may ask of a project, finds no such ask in their text. Leaving development on
Windows for later is an order of known work, which `goal@serves-one-kind-of-project` gives
knowledge-architect's owner, and no narrowing of the conditions. The owner reported a consumer project
that the owner wants to build on Windows soon.

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

The spelling of a path outside Unix, `git`'s private one and `slashed`:

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
  #backslash-in-a-pointer, withdrawn from the work after the search: the purpose does not need it,
  since a document that passes on Linux holds none that resolves; it removes the `skip-files` row by
  which a project on Linux keeps a file whose name holds `\`; and, as a stricter check of past
  commits, it would have the spec merge first. It becomes an issue.
- A temporary `push` trigger in the CI workflow, #iteration-before-ready, superseded by the probe
  branch of "How the work is done": it contradicts `design@knowledge-architect@ci-skips-a-draft`,
  its fields are empty on a push, and its successful run would satisfy the merge predicate.
- A public spelling function in `git`: lost to #path-spelling-home.
- A shared crate, `testing` or `git` as the home of the symbolic-link helper, and a skip where links
  are refused: lost to #symlink-helper-home.
- The writing commands leaving a file that differs by line endings: lost to
  #generated-text-compared-without-line-endings.
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
   Dropped: it is loud, and a consumer opts out, as the owner reported.
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
   `path@knowledge-architect@.github/workflows/ci.yml`. Claims:
   - the matrix lists the three systems, its one gates step is the gates command of CI, its checkout
     keeps `fetch-depth: 0` and the head's `ref`, and its artifact's name holds the system: a test of
     `path@xtask@src/gates.rs` reads the workflow, beside `ci_runs_the_gates_with_the_ci_flag`;
     mutations: a system removed, `fetch-depth` removed, the name without the system;
   - the job `check` needs `gates`, runs `always()` off a draft, and fails on any result but
     `success`: the same test; mutations: `always()` removed, the comparison changed to
     `!= 'failure'`, the `needs` removed.

   Fails alone on: the workflow test. The jobs themselves are seen when the pull request is marked
   ready; before, the probe runs the same commands.
2. **Windows compiles.** Builds: #paths-to-git-with-slashes; the symbolic-link helper's copies; the
   `cfg` marks of #platform-skips, the two items clippy refuses among them; `.gitattributes` of
   #line-endings-of-this-repository. Claims:
   - the workspace and its tests compile for Windows, with no warning: the cross clippy run by hand
     until step 4 lands it, `cargo clippy --workspace --all-targets --target x86_64-pc-windows-gnu`
     with the stub; it fails at main `9f1efd7b`, observed;
   - a path is named to git with `/` on Windows and byte for byte on Unix: unit tests of the private
     spelling, which takes the separator rule as an argument so that Linux tests both; mutation: the
     replacement removed;
   - every tracked file is checked out as committed: a test of `xtask` over a repository holding this
     one's `.gitattributes`, cloned under `core.autocrlf=true`, reads the same bytes; mutation: `.gitattributes` emptied.

   Fails alone on: a compile error or warning for the target.
3. **The release alias.** Builds: #release-maintenance-build, here, in the snippet and in the setup
   skill's sentence, with the installed copy and a Migration entry of the changelog. Claims:
   - both aliases name `--release`: a test of `path@xtask@src/gates.rs` reads the cargo configuration
     and the snippet; mutation: `--release` removed from either;
   - no gate replaces the running binary: the probe's Windows run passes the point where the dev
     profile failed; a `test` gate that fails on that removal is the failing observation.

   Fails alone on: "Access is denied" on the running binary.
4. **The cross gate.** Builds: #cross-clippy-gate in `planned@gates@src/cross.rs`, this repository's
   list, `path@knowledge-architect@rust-toolchain.toml`, `path@gates@README.md`, the crate's
   documentation, one sentence of the setup skill, and a changelog entry. Claims:
   - the gate's arguments, variables and name, for a target with hyphens: a unit test; mutations: a
     variable dropped, the hyphens kept, `-D warnings` dropped;
   - the mapping table, each row: a unit test over a function that takes the machine's system as an
     argument; mutation: the `android` exclusion removed;
   - this repository's list holds the gate after `clippy`, and lacks it on Windows: the list test,
     whose expected names follow `cross_clippy`'s answer; mutation: the insertion removed;
   - the gate reports a Windows-only warning: `#[cfg(windows)]` code with an unused import, added by
     hand and removed, observed failing.

   Fails alone on: a build script's failure under the stub.
5. **Line endings.** Builds: #generated-text-compared-without-line-endings in
   `path@check@src/check/generated.rs` and in the build of `agent-skills`. Claims:
   - a generated file that differs by line endings alone is current, and one that differs otherwise is
     not: a unit test of `compare`; mutation: the normalisation removed from either side;
   - a project checked out under `core.autocrlf=true` passes `check`: a test over a mock project or a
     scaffold, which clones it with that setting, on Linux too; it fails at main, observed as 3
     findings;
   - the shipped text of a CRLF checkout is the shipped text of an LF one, `SNIPPETS` included: unit
     tests of the passes of `path@agent-skills@src/render.rs` and of the snippets' listing over CRLF
     text; mutation: the normalisation removed.

   Fails alone on: the autocrlf test.
6. **Printed paths.** Builds: #printed-paths-with-slashes and #path-spelling-home. Claims:
   - `slashed` rewrites the separator under Windows' rule and nothing under Unix's: a unit test with
     the rule as an argument; mutation: the rule swapped;
   - no project-relative path is printed by `.display()` in the four crates: read over the corpus of
     `.display()` sites, each one an absolute path, a refused row as written, or converted; a search
     is a lead, per point 3 of `skill@knowledge-architect-planning@working-a-slice`;
   - the tests that fail on a printed `\` pass on Windows: the probe.

   Fails alone on: a test whose only difference is a separator.
7. **The tests, until the three jobs pass.** Builds: the JSON helpers' escaping; the remaining causes
   of the table; #mutate-tests-on-windows; the macOS failures; the skips of #platform-skips; the
   issues of "Outside the work", and the instances added to the issues that "What is already decided"
   names. Claims: every test of the workspace passes or is skipped by #platform-skips on the three
   systems, by the probe; every skip is listed, with its reason or its issue, in the commit's message;
   `--patch` under `core.autocrlf=true` gives the mutant the patch names: the five tests of
   `path@mutate@tests/modes.rs` that fail under it, observed. Fails alone on: the probe's test run.
8. **The heads conform.** Builds: the split of `design@xtask@gates-list-primary-home`, and what the
   design-record audit over the eight heads of "What is already decided" finds. The audit's result
   fills this step before it starts.
9. **The harvest**, below, with the probe branch's deletion and the pull request marked ready once the
   review is done.

## Order rationale

- 1 before 2: the owner's order of round 7, the jobs first.
- 2 before 3: the alias is seen to work on Windows only once Windows compiles.
- 3 before 4: the cross gate is run through the gates, which the swap stops on Windows.
- 4 before 5: each later step is judged for Windows by the cross gate on the machine that writes it.
- 5 before 6: the comparison and the printed paths fail the same tests, and the comparison is the
  smaller.
- 6 before 7: the printed paths are the largest cause of the failures step 7 reads.
- 7 before 8: the heads describe what is built.
- 8 before 9: the harvest writes into heads that conform.

## Defaults awaiting the owner

None.

## Harvest

| what | home |
| --- | --- |
| #gates-on-three-systems, #blocking-jobs | `design@xtask@gates-list-primary-home` and the head step 8 splits from it |
| #one-required-context | a head of the root, its argument resting on how GitHub counts a skipped job |
| #release-maintenance-build | `design@agent-skills@xtask-pins-checker` |
| #cross-clippy-gate | a head of `register@gates@design`, `design@gates@recommended-gate-list`, `design@knowledge-architect@toolchain-is-pinned` |
| #line-endings-of-this-repository | a comment in `.gitattributes`, or a head of the root if its argument on Git for Windows' default earns one |
| #generated-text-compared-without-line-endings | `design@check@generated-files-are-pure`, `design@model@a-file-register-index-is-rows` |
| #paths-to-git-with-slashes | `design@git@a-path-is-bytes` |
| #printed-paths-with-slashes, #path-spelling-home | a head of `register@model@design` |
| #symlink-helper-home | a comment at each copy |
| #no-module-split-in-the-port | the instances of step 7; no head |
| #mutate-tests-on-windows | comments at the tests and at `apply_in` |
| #platform-skips | a head of the root |
| #platforms-served | a head of the root, to which `issue@check@a-linked-manifest-splits-the-readers` is re-pointed |
| #asks-discussion-put-off | the instance in `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`; no head |
| #single-branch-spec, #backslash-in-a-pointer, #iteration-before-ready | no home: the first leaves with the spec, the second is an issue, and the third's argument is in `design@knowledge-architect@ci-skips-a-draft` |
| `design@knowledge-architect@ci-skips-a-draft` | its cost restated, with the probe branch as what was used |
| the losing alternatives | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` |
| premortem causes 1 and 6 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` |

## Later consequences

- Development on Windows, the owner's later aim, starts from the issues step 7 opens; the owner
  judges that it needs developing from Claude Code on Windows.
- A second target of the cross gate, such as one for macOS, is one line of the list and one target of
  the toolchain file.
