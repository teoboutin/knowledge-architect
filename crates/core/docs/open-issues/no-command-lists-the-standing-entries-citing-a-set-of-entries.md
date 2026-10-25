---
kind: todo
---
# No command lists the texts that cite any of a set of entries, nor names the entry that holds each citation

## Summary

`show` prints the file and line of every reference to one entry. It does not name the entry that
holds the reference, and it takes one entry per call. Two work lists need more: the texts citing
any entry whose head changed in a range of commits, and the head that cites a given `rejected`
entry.

## Details

### What

What the commands print today:

- `show <ref>` prints the entry, then `referenced at:` and one `<file>:<line>` per reference. A
  tripwires home holds many entries in one file, so a line number has to be mapped by hand to the
  heading above it. A reference from a design head, a `rejected` entry or a plan document is
  listed beside those from standing entries.
- `tripwires --guarding <ref>` prints the tripwire rows guarding one entry. It takes one reference.
- `issues` filters by anchor, by kind, by group and by text in the id or the title. It has no
  filter on what an entry cites.

The record search no longer needs it: `klarch worklist --seed` marks each entry that cites a seed,
per `design@core@worklist-writes-and-verifies-the-reports`. The owner judged the command worth an
issue when the search still mapped seeds by hand, and left its design for later. Nothing about the
command's shape is decided: a new command, a `--citing <ref>` filter repeated on `issues` and
`tripwires`, or several references accepted by `--guarding`.

The first work list is every text citing an entry whose head changed. The closing steps of
`skill@knowledge-architect-decision-recording@before-you-finish` run `show` on each head a change
edits and read every text that cites it, since "an edit that keeps the slug dangles no reference,
so no check reports a citing text it made false". In the owner-authority milestone, two routing
reviews found a citing sentence made false that way, the reference still resolving. That list is
every text citing any entry whose head changed in a range of commits, not only the issue and
tripwire entries, and its set of references comes from the range. The retrospective proposed a
`--changed` option of `show` taking a range. The design of this entry's command decides whether
one command serves both lists.

The second work list asks for the citing entry: a pin move of thaum to 0.8.0, whose migration cites
each `rejected` entry from the heads whose argument covers it. To find which head cites an entry,
the session and three reviewers wrote scripts mapping each line `show` prints to the level-three
heading above it. The narrowest shape that serves it, and the first list as well, is
`show` printing the reference of the enclosing entry beside each location; the orphan check of
`design@core@orphan-rejected-entry` already maps a citation to the design entry whose section holds
it. Analysed at the intake of 2026-10-24.

### Why it matters

`goal@knowledge-architect@structure-and-workflow-work-together` states that "the tool computes the
workflow's work lists: what a reversed decision touches, and which issues and tripwires a change
must re-read". For a change that touches several decisions, the tool computes the second list one
entry at a time and by file and line, and the agent turns that into entries. That costs one call
per entry and a hand mapping, and a mapping done by hand can drop an entry.

### What would close it

A command, or a filter of an existing one, that serves both work lists above: the texts citing any
entry of a set or of a range's changed heads, each named by the entry that holds the citation, one
row per entry. It is tested over a mock project in which two tripwires of one home and one issue
cite two different decisions. Or the owner rules that both lists keep using `show`, and this entry
is deleted with that reason in the commit.
