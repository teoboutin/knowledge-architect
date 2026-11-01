# knowledge-architect-mutate

The hand mutation: apply one mutation you write, run a command against it, report whether the
command noticed, and put the file back. It is the check that a test discriminates, performed over
work that is not committed yet. A project's maintenance crate runs it as `cargo x mutate`, per the
setup skill's section on a Rust project; why the tool is shaped this way is `register@mutate@design`.

```sh
cargo x mutate run --file <F> --replace <OLD> --with <NEW> [opts] -- <command>
cargo x mutate run --file <F> --patch <P.diff>            [opts] -- <command>
    --build "<command>"   probe that the mutant builds
    --no-baseline         skip the check that <command> passes over the unmutated tree
cargo x mutate restore              # unwind a run that was killed
cargo x mutate restore --discard    # drop the record, writing no file
```

**The verdict is the exit code of the command after `--`**, run from the project's root: nonzero is
`caught`, zero is `SURVIVED`, no test distinguished the mutation. It is never the exit code of
`cargo x mutate run`, whose table is the crate-level documentation of `path@mutate@src/lib.rs`, with
"no project" added by the maintenance crate:

| code | meaning | what to do |
| ---- | ------- | ---------- |
| `0` | the run completed | read the verdict it printed |
| `1` | it acted and something is owed on the tree | look at the tree: no verdict was reached, the target was left mutated, or a record still stands |
| `2` | it never started | a bad argument, no project or no git repository, or a precondition that refused before the target was written |

`1` does not by itself say the tree was changed: a command that could not be spawned reaches no
verdict and exits 1 over a tree the tool restored.

```
$ cargo x mutate run --file crates/mutate/src/harness.rs \
    --replace '0 => Err(PlanError::NotFound),' --with '0 => Ok(source.to_string()),' \
    --build 'cargo test -p knowledge-architect-mutate --no-run --quiet' \
    -- cargo test -p knowledge-architect-mutate --quiet
baseline: running the command over the unmutated tree
mutation applied to crates/mutate/src/harness.rs, first changed line 65
build probe: cargo test -p knowledge-architect-mutate --no-run --quiet
running: cargo test -p knowledge-architect-mutate --quiet
...
crates/mutate/src/harness.rs restored
crates/mutate/src/harness.rs: caught
```

## What to respect

- **The two input forms differ only in what they can express.** `--replace` refuses unless the
  pattern occurs exactly once, the assertion a `str.replace` or a `sed` cannot make. `--patch`
  expresses a mutation that is not one substitution, a deleted block or reordered lines, and
  `git apply` refusing is that form's assertion.
- **The mutation is shown present before the command runs.** The tool reads the file back and
  compares it with what it wrote. Without the line `mutation applied to …`, no verdict below it
  means anything.
- **Pass `--build` whenever the mutant might not compile.** A mutant that fails to build makes the
  command fail for a reason that is no test noticing, and with no probe that reads as `caught`.
  With a probe the verdict is `unmutatable here` instead. The probe runs with no shell: it is
  split on whitespace, so `&&`, a pipe or a quote reaches the program as an argument. Give one
  command; a build of several packages names them all with `-p`. A probe that cannot be spawned
  gives no verdict, and an empty one is refused.
- **The baseline runs by default.** The command runs once over the unmutated tree first, and a
  failure there refuses the run, since over a red base every mutation reads as `caught`.
  `--no-baseline` skips it when the failure is expected and understood. A change saved to the file
  while the baseline runs refuses the run, which writes nothing.
- **A run over uncommitted work is safe.** The restore writes the pre-image only when the file is
  byte-identical to the mutant the tool wrote. When something else edited the file meanwhile, it
  writes nothing and prints the `git apply -R` that reverses the hunk alone, and the `-C1` form,
  which recovers an edit two or more lines from the mutation. An edit adjacent to the mutated line,
  or on it, reverses under neither, and the pre-image in the record is then the way back.
- **A killed run is recovered with `cargo x mutate restore`.** The record, the pre-image among it,
  is on disk before the target is touched, in the git directory, at the path
  `git rev-parse --git-path knowledge-architect-mutate` names, where neither `cargo clean` nor
  `git clean` reaches it. The project must therefore lie inside a git working tree.
- **Discharging a record is your word, not the tool's.** After you have unwound a mutation by hand,
  `cargo x mutate restore --discard` drops the record; it writes no file.
- **One mutation at a time.** A `run` claims the record by creating its directory, so of two
  concurrent runs exactly one proceeds. `restore` refuses while the run that claimed the record is
  alive, since putting the pre-image back underneath it would have its command measured over
  unmutated code; the owner is read from `/proc` on Linux and with `kill -0` on another Unix, and
  elsewhere a run reads as gone.
- **The target is one regular file of the working tree, reached through no symlink.** An absolute
  path, a path holding `..`, a path through a symlink, a path inside the git directory and a file
  that is not regular are refused. A `--patch` that names any other file is refused, and so is one
  that changes nothing.
- **A command that deletes the target gets the pre-image written back**, since there is nothing
  there to lose and refusing would strand the record; a symlink left in its place is not written
  through.
- **A mutant that cannot be written in full leaves the record standing** when the file no longer
  holds its content, and the run exits 1.
- **A hard kill leaves the command running**, since the tool waits on it without owning its process
  group: `issue@mutate@a-hard-kill-orphans-the-command`.

## The library

A maintenance binary carries the two modes under a command of its own and hands the parsed value to
`knowledge_architect_mutate::run` with the project's root, which it finds itself. The crate-level
documentation, rendered on docs.rs, describes the API.
