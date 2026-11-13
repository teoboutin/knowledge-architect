---
kind: todo
---
# A register home that `new-component` writes says "No entry yet." after its first entry lands, and nothing reports it

## Summary

`home_text` in `path@create@src/command.rs` writes "No entry yet." into each register home of a
new Component. The sentence stays when the first entry lands, and no check reads it.

## Details

### What

Seen in the work that created the git Component: the git Component's issue home,
`path@git@docs/open-issues/README.md`, still said "One file per outstanding item of git. No entry
yet." over four entries, until the review before its merge found it. The sentence is written for
every register `home_text` knows but the two plan types: goal, design, rejected, tripwire, issue
and any other. The specs and milestones homes say what they hold, with no such sentence.

The sentence is not `create`'s alone. The setup skill tells every adopting project "An empty
register home says it holds no entry yet." (`skill@knowledge-architect-setup`, its source in
`path@agent-skills@content/skills/setup/SKILL.md`), and
`design@check@nothing-of-a-project-is-compiled-in` describes the starting text as "a title and a
line on what it holds, or that it holds no entry yet". Dropping the sentence from `home_text`
therefore changes that instruction of the installed text and that head, a decision for
`skill@knowledge-architect-design`, not a fix of `create` alone.

The root alone carries the register `goal`, so `new-component` writes no goals home, per
`design@create@component-command`, and `init` writes the root's alone. The goal row of `home_text`
writes "No entry yet: the goals are stated with the owner." into it, which stays true only until
the root's first goal: the root's goals home is a case of this entry, as every other register's
home is.

### Why it matters

A home's README is the directions a reader gets for that directory, per `primer@where-knowledge-goes`,
and a false sentence there is met by every session that opens it, against
`goal@documentation-stays-consistent`.

### What would close it

`home_text` writes no sentence that turns false at the first entry, or the check reports a home
that says "No entry yet." over an entry; a test of `new-component` or of the check showing it.
Either closure is decided under `skill@knowledge-architect-design` first, as the paragraph above
says.
