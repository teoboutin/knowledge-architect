---
kind: todo
---
# A typed command trusts its caller to have configured the manifest, and answers wrongly when it has not

## Summary

Each command's function in `path@check@src/commands/mod.rs`, in `query`, and `new_component` of `create` takes a manifest its extensions have configured,
as each crate's documentation says, and nothing enforces that. Given a manifest nothing configured, it can pass where `klarch` fails. The owner holds that this
precondition should ultimately be enforced rather than stated.

## Details

### What

Measured by an adversarial reviewer with a binary linking `knowledge-architect-check` and calling `commands::*` directly, over a
project with `[bogus-table]` and `x = 1` added to its manifest, staged:

| call | result |
| --- | --- |
| `klarch check` | `[bogus-table] is a table no extension of this binary reads`, exit 1 |
| `commands::check` with `CheckMode::Working` or `CheckMode::Fix`, on `Manifest::load` | `PASSED: no findings`, exit 0 |
| `commands::check` with `CheckMode::Staged`, on the same manifest | exit 1, since `path@check@src/commands/history.rs` configures its own snapshot's manifest |
| `klarch index` and `klarch install-agent-skills` | exit 2 |
| `commands::index` and `commands::install_agent_skills`, on the same manifest | exit 0 |

So one function gives two verdicts by its mode, and a frontend that forgets the call gets a false pass, with its writers running
over a manifest the binary refuses. `cli::run` configures for every command, so `klarch` and an extension's binary do not meet it.

A lead, not a decision: a type only `extension::configure` can return, which each typed command takes in place of `&Manifest`.
Not established: what that does to `Manifest` as the facades publish it.

`design@query@a-function-takes-a-configured-manifest` answers the same question for a tool that links no check, and `issue@create@init-and-new-component-run-an-extension-on-a-manifest-it-did-not-configure` a case in `create`.

### Why it matters

`design@knowledge-architect@a-typed-command-prints-and-returns-its-code` offers these functions to a frontend that is no command line, and
`goal@check@document-rules-are-checked` holds that a tree the checker passes conforms. A precondition a caller can miss, with no
refusal, gives a pass on a tree the same checker fails.

### What would close it

A typed command cannot be given a manifest nothing configured, by its type or by a refusal, with a test over `[bogus-table]`, and
the module's doc stops stating it as a precondition. It may be done with the issue above.
