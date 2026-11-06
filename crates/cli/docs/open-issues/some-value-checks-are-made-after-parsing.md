---
kind: todo
---
# Some checks that need only an argument's value are made after parsing

## Summary

`goal@cli@one-home-for-the-command-line` asks that a check of a plain value, one that names nothing in the project and that the command
is not asked about, is made while parsing. Three are made by the tool today, so the goal is unmet for them.

## Details

### What

- `--since` of `transcript list` and `transcript search`: a `String` in `path@cli@src/cli/transcript.rs`, refused by `check_date` in
  `path@check@src/commands/transcript.rs`, with "--since takes a date as YYYY-MM-DD".
- The pattern of `transcript search`: a `String`, parsed by `regex::Regex::new` in the same file.
- `--name` of `init`: `name.starts_with(OWNED_PREFIX)` in `path@check@src/commands/create.rs`. The tool keeps this rule for a name it takes
  from PATH, so `--name` gains a second call of it. That this check moves is the session's default, which the owner agreed.

Each already exits 2 before any write, so what changes is where the rule sits, and that each then falls under
`goal@cli@a-refused-invocation-does-no-work` as a value the declaration does not accept. A value parser on the declaration can call a rule
the crate of the tool holds, which keeps `cli` free of the tool's rules. `show`, `worklist --seed`, `tripwires --guarding` and `id` stay as
they are: their value names something in the project, or is what the command is asked about.

### Why it matters

An agent adding a new value of the same kind reads these as the pattern, and places the check in the tool.

### What would close it

The three checks are made while parsing, each with a test that runs the binary, exit 2 and nothing written.
