---
kind: design
---
# Sessions misread which course a design discussion takes: bounded work, the in-change path or the full path, and when a spec merges

## Summary

Sessions choose the in-change path for work that has not started, read "an issue fix" as a kind of
work that takes the in-change path, and, on the full path with one spec and one branch, merge the
spec alone before building its work. The installed text that decides these choices is in
`skill@knowledge-architect-design@decision-authority` (the two paths),
`skill@knowledge-architect-design@bounded-work`, and the merge rule of
`skill@knowledge-architect-planning@plan-reviews`. The owner asked that the conditions be discussed
again and clarified, the split into three courses included; the leads below are input to that
discussion, not its conclusions.

## Details

### What

**Instances seen.**

1. **The in-change path taken for work nobody had started, in this repository.** The design
   discussion that made a plan document keep one transcript per discussion, ending at the owner's
   word on the last convergence checkpoint, opened with no change under way. Its first round stated
   "I expect the in-change path: one branch that rewrites the head and the planning skill's text",
   and the session kept that path to the implementation. The design skill gives the full path to
   work "nobody has started", so a spec was owed. The owner asked why no spec was written; the
   session found the misreading then. The branch is the one that adds this entry.
2. **The same misreading in earlier sessions**, on the owner's report: the owner has seen the
   in-change path chosen wrongly before. Dates and branches are not established.
3. **"An issue fix" read as a kind of work**, on the owner's report: the in-change path names "an
   issue fix or a review repair already under way" as examples, and the owner has seen a session
   take the in-change path for an issue whose fix needed a full-path discussion. The owner reports
   that whether an issue is being solved is close to uncorrelated with the path.
4. **A spec merged alone on the full path with one branch**, on the owner's report: the session's
   next act after writing the spec was to merge it alone, as a milestone document is, instead of
   building its work on the same branch. Not established: whether that work's first commit changed
   what the per-commit gate checks, in which case the planning skill's rule did require the spec to
   merge first.

**Suspected mechanism, in one sentence:** the condition that decides the path, whether the change
was under way before the discussion opened, is stated in a subordinate clause beside examples named
by kind of work, while the full path's cases are a parenthetical that reads as a size, so a session
chooses by the size or the kind of the work.

**Leads in the text, read on the tree of the branch that adds this entry:**

- The in-change path's sentence: "It is open when the decision lands in the change under way: its
  work, where that change began before the discussion opened, as an issue fix or a review repair
  already under way does, or its record, …". The deciding test, "began before the discussion
  opened", is a relative clause; the examples name kinds of work, and "already under way" is easy to
  drop when reading "an issue fix".
- The full path's sentence: "Otherwise, and whenever a discussion converges on work that no longer
  lands in the change under way (it needs a branch of its own, several pull requests, or nobody has
  started it)". "A branch of its own" reads as a size, so one branch reads as the in-change path. The
  case that settles most requests, "nobody has started it", is last.
- `design@agent-skills@in-change-path` states the rule a session needs: "A requested change that is
  not bounded, and whose work has not started when its discussion opens, takes the full path too,
  even when one pull request builds it." The installed text holds no sentence saying "even when one
  pull request builds it".
- The two paths are introduced as "differ[ing] in where the deliberation is kept", which frames the
  choice as one of where to write rather than of when the work began.
- Loop step 1 has the session draft "in the form of the path you expect" before the search, and
  says "The classification after the search still decides the path", a classification the step
  does not name. An expectation stated in the first round, as in instance 1, is not revisited by any
  named step.
- `skill@knowledge-architect-planning@spec-or-milestone` defines a spec as "the plan document of
  work done in one branch and one PR", so "one branch" is the size of a spec and also what the
  session of instance 1 took as the in-change path's mark.
- `skill@knowledge-architect-planning@plan-reviews` says a plan document "may be merged on its own,
  whatever the time of its work", and must be merged before its work where that work's first commit
  changes what the per-commit gate checks. The default for a spec built at once, on its work's
  branch, is stated only in the section on the work of a spec, which does not say the spec stays on
  the branch. The rule would have applied to the branch of instance 1: its first commit changes what
  the per-commit gate checks, since the checker then refuses a `transcript.txt` it accepted before,
  so on the full path its spec would have had to merge alone, before its work.
- Three courses are chosen at different moments by different texts: bounded work after the
  grounding, by `skill@knowledge-architect-design@bounded-work`; the path, by `skill@knowledge-architect-design@decision-authority`
  section, after the search; and the merge of a plan document, by the planning skill.

### Why it matters

A wrong path skips what the other path owes. Instance 1 skipped the code-split scout, the plan
reviews and the committed transcript of the discussion, the record its own work set out to keep. A
spec merged alone sends its work to a later branch and a later session, which needs a design audit
the work did not need. Both strain `design@agent-skills@in-change-path` and
`design@agent-skills@design-hands-off-to-planning`, and threaten `goal@agents-get-a-complete-workflow`,
since a session that cannot tell which procedure applies follows the wrong one.
`issue@agent-skills@the-open-space-test-may-admit-every-problem` asks a neighbouring question about
the bounded course.

### What would close it

A design discussion on the courses a design discussion can take, which the owner asked to hold: whether three courses are the right split, the condition that chooses each, stated where a
session chooses, and when a spec merges apart from its work. It closes when that discussion's
change lands, or when the discussion finds the text clear and names what else produced the
instances.

### Re-entry point

That discussion, when the owner opens it; and any change to the two paths of
`skill@knowledge-architect-design@decision-authority`, to
`skill@knowledge-architect-design@bounded-work`, or to the merge rule of
`skill@knowledge-architect-planning@plan-reviews`, which reads this entry before it is written.
