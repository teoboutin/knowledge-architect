---
kind: todo
---
# The installed set has no skill for developing, for work on a branch under a linear history, or for releasing

## Summary

`goal@knowledge-architect@agents-get-a-complete-workflow` lists developing, working on a branch under
a linear history, and releasing among the activities the workflow covers, and
`goal@agent-skills@one-skill-per-activity` asks one installed skill each. The installed set holds
none of the three. This repository keeps its own procedures for them locally: `skill@klarch-development`,
`skill@klarch-release`, and the section `## Git` of its root `CLAUDE.md`.

## Details

### What

- Developing: the steps the goal lists (where to ground, what to be careful of, how to test properly,
  when code is reviewed during development, on which axes, and how it is repaired), and the default
  shape of code with a review that weighs it, which
  `goal@knowledge-architect@the-workflow-leads-to-better-code` asks for.
- Branch work: rebasing, rewording a commit's message, folding a repair, with no content lost, and
  merging once the conditions of `goal@knowledge-architect@serves-one-kind-of-project` hold.
- Releasing: the generic steps and guidelines, the versioning and publishing policy staying with each
  project.
- The owner named thaum's `thaum-development` skill as a source for the generic shape of developing.

### Why it matters

Both goals are unmet while the skills are missing, and
`goal@knowledge-architect@generic-tools-and-procedures-are-shipped` is unmet while this repository's
own skills hold procedures it would ship once the owner takes them in.

### What would close it

The three skills ship in the installed set, each reached through a design discussion under
`skill@knowledge-architect-design`, and this repository's local copies keep only what is particular
to it.
