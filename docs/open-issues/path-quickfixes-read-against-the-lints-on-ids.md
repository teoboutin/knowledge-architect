---
kind: question
---
# Does the quick-fixes spec still hold now that the lints on ids are built?

## Summary

`spec@plans@path-quickfixes` cited the spec of the lints on ids, which left the plans directory at
its harvest. Its own work, held on the owner's word, rewrites the head of the bare-id lint and
respells a bare name with `check --fix`; that head was rewritten and renamed by the work that
left. Does the quick-fixes spec still hold, accounting for what landed?

## Details

### What

The leaving spec built three lints on a defined id written outside its reference form and the
grammar of two segments, and harvested them into `design@check@backticked-bare-id`,
`design@check@plain-word-lint`, `design@check@reference-shape-outside-span`,
`design@model@two-segment-ids` and the split of the candidate rule, `design@model@at-span-candidate-rule`.
The quick-fixes spec's rows on `design@check@backticked-bare-id` quote a sentence about a project
skill named by an ordinary word, which the rewritten head no longer holds, and two of its passages
quote the head as "A bare name is a pointer written with no kind", which now reads "A bare id";
its argument item #arg-30 on that head quotes a re-take clause the head
now words otherwise, and a harvest row plans to rewrite its title, its sentence on renaming a skill
and its census clause, none of which stands as written.
It plans a respelling of a
bare name of a skill or an agent, where the lint now reports every defined id and lists every
candidate where several entries define one, choosing none. The leaving spec left to it a
`check --fix` respelling of the findings of all three lints on ids, which it does not hold. Its argument items were renumbered
`arg-<n>` in that work, with their references.

Instances of the respellings the lints on ids now ask, applied by script, from the retrospective
files received at the intake of 2026-11-07:

- a pass of the tripwire axis in thaum, a project that installs the workflow, 2026-11-01: about 180
  references in one audit report file written without backticks or without their kind, each
  finding naming its repair, applied by a script run three times;
- the revision of the goal standard in this repository, 2026-11-04: the same repairs over the
  review's records, applied by a script.

So in one session the lints on ids give a number of one-candidate repairs of the order of the 179
raw-path findings the spec's census counted over many sessions. Two arguments of
`spec@plans@path-quickfixes` rest on that census, taken before the lints on ids landed: #arg-7, that
the finding kinds a respelling would serve are rare, and #arg-26, the owner's judgement that the
work is much for the share of findings it serves.

Three classes of finding differ, and only the first is a respelling:

- a reference written without backticks, and a backticked bare id with one candidate: the span
  already names its target, the respelling #arg-3 of the spec defines;
- a plain word naming an entry: `design@check@plain-word-lint` names two repairs, the reference or
  a rename where the word is meant as an ordinary term, so a fix would choose, which
  `design@check@safe-fix-definition` refuses;
- a reference to an item of a plan document deleted in the same change: the session chooses between
  the bare name and a harvested head, and the fix would also need the staged tree, which `--fix`
  refuses.

The same repairs on a commit message are outside the spec, which says no `--fix` reaches a commit
message; `issue@check@no-command-applies-the-repair-a-message-finding-names` records them.

### Why it matters

A plan document written against heads that have since changed is implemented against text that no
longer states the design, against `goal@knowledge-architect@documentation-stays-consistent`; a
respelling of an id several entries define would make the checker choose a meaning, which
`design@check@backticked-bare-id` refuses.

### What would close it

The design audit of the quick-fixes spec's work, under
`skill@knowledge-architect-planning@working-a-slice`, reads it against the heads above and
amends it, or reopens its design where a decided shape no longer holds.
