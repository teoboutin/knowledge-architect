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
every register `home_text` knows: goal, design, rejected, tripwire, issue, spec, milestone and any
other.

The sentence is not `create`'s alone. The setup skill tells every adopting project "An empty
register home says it holds no entry yet." (`skill@knowledge-architect-setup`, its source in
`path@agent-skills@content/skills/setup/SKILL.md`), and
`design@check@nothing-of-a-project-is-compiled-in` describes the starting text as "a title and a
line on what it holds, or that it holds no entry yet". Dropping the sentence from `home_text`
therefore changes that instruction of the installed text and that head, a decision for
`skill@knowledge-architect-design`, not a fix of `create` alone.

The goals home of a Component is no case of this entry. A Component states no goal, so the twelve
goals homes of this repository's Components hold no entry on purpose, and each says so in a
sentence that no entry will make false: "No entry: the goals of the project are stated in the goals
home of its root". The goal row of `home_text` still writes "No entry yet: the goals are stated with
the owner." into a new Component's goals home, which says that a goal may come. The milestone that
moves the goals to the root has a new Component written no goals home, a change of
`design@create@component-command`, and that row leaves with that work. The entry stands for every
other register.

### Why it matters

A home's README is the directions a reader gets for that directory, per `primer@where-knowledge-goes`,
and a false sentence there is met by every session that opens it, against
`goal@documentation-stays-consistent`.

### What would close it

`home_text` writes no sentence that turns false at the first entry, or the check reports a home
that says "No entry yet." over an entry; a test of `new-component` or of the check showing it.
Either closure is decided under `skill@knowledge-architect-design` first, as the paragraph above
says.
