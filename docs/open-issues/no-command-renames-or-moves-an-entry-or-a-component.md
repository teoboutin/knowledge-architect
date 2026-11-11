---
kind: todo
---
# No command renames an entry or a Component, or moves entries between Components

## Summary

The installed text asks a session to rename an entry's slug with every reference to it, to rename
a Component, and to move entries into a new Component, each re-pointed. The references to rewrite
follow from the entity table, and no command rewrites them: sessions do it by `git mv`, `sed` and
scripts of their own over the check's list of dangling references, and those scripts rewrote text
they should have left.

## Details

### What

The installed text that asks each task, in the source of the skills:

- `path@agent-skills@content/skills/decision-recording/SKILL.md`: "renaming one means rewriting
  every reference in the same change. Grep before you rename.";
- `path@agent-skills@content/skills/component-creation/SKILL.md`, point 4 of the creation
  procedure: move the record that is now the new Component's, each entry re-pointed;
- `path@agent-skills@content/skills/goal-setting/SKILL.md`: "When one is removed, every reference
  to it dangles".

`klarch --help` lists no command that renames or moves anything.

Instances, from retrospective files of this repository's sessions, 2026-11-01 to 2026-11-06:

- the session that planned the development milestone: an issue entry and a goal renamed by
  `git mv`, a `sed` over the files a grep found, and a script;
- the session that narrowed the goals: two goals renamed and one removed, re-pointed over 26 files
  by `sed`;
- the slice that renamed a Component and moved its crate's directory: the references rewritten
  by several scripts; the check found every dangling reference, but not 33 lines of bare
  paths nor the names in prose, and the first regular expression over those "rewrote too much, and
  it broke one code line" (the 33 lines are the file's count, not re-counted);
- the revision of the goal standard: one goal slug renamed over 11 files by a script, which also
  rewrote a quotation and an argument of a plan document, repaired by hand;
- the slice that created the `model` Component: 62 heads, 34 `rejected` entries, 3 tripwires and
  10 issues moved, re-pointed over 95 files, by a script;
- the slice that moved the commands out of the checker: heads, `rejected` entries and issues moved
  into new Components by a script of about 100 lines.

What a command may not rewrite, per the record:

- `design@agent-skills@a-past-sentence-is-rewritten`: a verbatim quotation of the owner that names a
  renamed entry is left as it is, with a reference to the current entry beside it; a record of a
  deliberation names a removed entry by its bare slug. A rewrite of those is the session's.
- `design@model@an-entity-belongs-to-its-anchor`: moving a document moves the entities in it, so a
  move rewrites the anchor segment of each reference and no more.
- `design@check@quotation-read-by-no-id-lint` already defines a quotation, which the checker
  computes.
- `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves` records one
  form that a move breaks beyond a mechanical edit.

Under `design@check@every-path-names-its-anchor`, one fixed-string search per anchor lists every
reference to it, so a Component is renamed by one search and replace over its references. The bare
paths and the prose names are outside what the check judges, so no strain of that head is shown.

Ruled out: a report of bare paths added to `check` alone, which finds them and rewrites nothing.
Not established: which Component would own the command; it reads the entity table of `model` and
writes documents, as `create` and `check --fix` do.

### Why it matters

The references follow from the entity table with no judgement, per
`goal@tasks-a-tool-does-better-are-the-tool-s`, which owes the task a command once the
owner takes its generic part in. Each session's script is written again, and two of them rewrote a
quotation, a plan argument and a code line.

### What would close it

The owner takes the tasks in or declines them. If taken in: a design discussion places the
command, which rewrites every reference the entity table holds, moves entries with the `rejected`
entries their citing heads cover and the tripwires guarding them, and lists each quotation, bare
slug, bare path and commit message it leaves to the session; with a test over a mock project. If
declined, this entry is deleted with that reason in the commit.
