---
kind: defect
---
# Two audit workflows can give two groups one label, and list a sample entry twice in one brief

## Summary

The saved workflows of the design-record axis and of the tripwire axis share two defects, found by
reading the scripts at the adversarial review of the tripwire axis. A packed group of several
anchors is labelled `packed-<n>`, which an anchor named `packed` also produces, so two groups can
share a worktree and a scratch path. And a calibration-sample entry that falls in a drafter's own
group is listed twice in that drafter's brief, while the auditor's return rule says each entry
appears once.

## Details

### What

- **The label collision.** The function `cut` of
  `path@agent-skills@content/workflows/design-record-audit.js` labels a group of one anchor
  `<anchor>-<n>` and a group packing several anchors `packed-<n>`; the function `cut` of
  `path@agent-skills@content/workflows/tripwire-audit.js` does the same. A project whose manifest
  declares a Component or a location named `packed` gets labels `packed-1`, `packed-2` from both
  rules, so two groups share a worktree path and a scratch path, and the setup step fails on the
  second `git worktree add`, or two auditors write one directory. Not established by a run: read in
  the scripts.
- **The sample listed twice.** Both workflows give every drafter the shared calibration sample
  beside its own group. A sample entry that is also in the drafter's group appears twice in its
  brief, while the auditors' return rule, in `agent@knowledge-architect-design-record-auditor` and
  `agent@knowledge-architect-tripwire-auditor`, says "Every entry of your group and of the sample
  appears once in the list". Not established by a run: which of the two an auditor obeys is unknown.
- Ruled out: neither defect is reached in this repository today, since no anchor is named `packed`,
  and the sample is drawn per group so that an overlap depends on the draw.

### Why it matters

An audit pass is one run of its saved workflow, per `design@agent-skills@one-run-pass`: a setup
that fails stops the pass, and two auditors writing one scratch directory corrupt each other's
drafts with no error. A drafter that returns an entry once where the brief lists it twice, or twice
where the rule says once, makes the script's count of drafts disagree with its groups.

### What would close it

A label for a packed group that no anchor name can take, such as one holding a character the id
grammar refuses, in both scripts; and the sample's entries removed from a drafter's sample list when
they are in its own group, or the return rule stated for that case; each with the script's parse
check rerun.
