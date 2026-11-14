---
kind: design
---
# The hand mutation runs one mutation per invocation, so a list of mutations is a loop each session writes

## Summary

`cargo x mutate run` takes one mutation. A session that shows each claim of a unit of work runs
dozens, by a shell loop of its own, and filters the output for the verdicts; one filter hid every
verdict of a batch of eleven. `design@mutate@mutation-command` chose one mutation per run until a
second real use, which this is, so the choice reopens.

## Details

### What

- `design@mutate@mutation-command`: "A run takes one mutation: the owner chose that over a recorded
  list of mutations, the sweep generalised, which no goal ranks, and which waits for a second real
  use." `run` takes one `--replace` or one `--patch`, per `path@mutate@README.md`, "The two input
  forms".
- `rejected@mutate@sweep-shipped`: "A generic sweep would need each project to declare its sites
  and its mutation, a recorded list that no second project has asked for."

The instance: the session on the transcript reader's gaps in this repository, 2026-11-12, C1 of its
workflow file. It ran 36 mutations, each an invocation of `cargo x mutate run` with its own build
probe and test command, wrapped in a shell function and a loop, and filtered the output for the
verdict lines; one filter matched only the `baseline:` lines, hid every verdict of a batch of
eleven, and the batch ran again. It proposes a batch mode: a file of mutations, each naming its
replacement, build probe and test, printing one verdict line per mutation.

Why the head reopens without the owner: its choice is a fork's choice in the weak form, which
reopens on a fact that ranks the shapes, per `primer@design-heads`, and the owner named the fact it
waits for. The first real use was thaum's sweep; this loop is a second, a list of 36 mutations each
written and named by its caller, in a second project, this repository.

The new argument `rejected@mutate@sweep-shipped` needs: a batch of caller-written mutations is no
sweep, since the tool would still generate and enumerate none; and the second project that entry
says has not asked is this repository, whose session asked.

Bounds: `design@mutate@one-file-per-run` and `design@mutate@one-command-owns-the-interval` stand
under a batch: each mutation still names one file, and each apply, run and restore stays inside the
command.

The rival: no change, the loop kept a session's own. One session met it; about thirty runs in
another session of the same batch are reported with no loop named.

Re-entry point: the design discussion under `skill@knowledge-architect-design` the reopening opens,
going to the owner where it ends at a fork again.

### Why it matters

Each session writes the loop and its filter again, and a wrong filter hides verdicts, so a claim
reads as shown, or a batch is run twice; the task follows from the list with no judgement, which
`goal@tasks-a-tool-does-better-are-the-tool-s` gives a tool, for a task
`design@mutate@mutation-command` already took in, and the verdicts serve
`goal@the-workflow-leads-to-better-code`.

### What would close it

The discussion rewrites `design@mutate@mutation-command` for a batch of caller-written mutations,
argued against `rejected@mutate@sweep-shipped`, built with a test in `path@mutate@tests/modes.rs`
of a list printing one verdict per mutation; or it keeps one mutation per run on an argument the
discussion records, and this entry is deleted with it.
