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
5. **Bounded work classed after the owner's replies had returned the work to the loop, in this
   repository.** The discussion on the cleanup of what the tests leave in the temporary directory
   opened on a request with no work under way. The owner's replies brought arguments, new shapes
   and doubts; the record search then ran, and the session classed the work bounded and built it.
   One decision passed an entry test, so the second condition was false; the review before the
   merge found it, after the code was written, and one head became three. By
   `design@agent-skills@in-change-path` the work owed the full path. The owner reported that
   everything that went wrong in that session, related to following the workflow, arose from the
   choice of the bounded path, that the path may have intrinsic defects, and that without it a
   reviewed spec before the work would have caught most of those issues. Per W1 of the
   retrospective file `2026-11-25-knowledge-architect-tmp-cleanup-klarch-workflow`, which counts
   three uses of the bounded path found by their commit messages and the archived retrospective
   files, two of them classed bounded on a false second condition, each found only at the review
   before the merge.
6. **A spec written after its work, on the owner's direction, in this repository.** After instance
   1, the owner directed that the spec be written, reviewed, and the implementation reviewed against
   it. The plan reviews then read a document whose work existed, and the cold implementer's
   completeness findings had nothing to change. No installed text covers a spec written after its
   work, per W3 of the retrospective file
   `2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow`.

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
  grounding, by `skill@knowledge-architect-design@bounded-work`; the path, by `skill@knowledge-architect-design@decision-authority`,
  after the search; and the merge of a plan document, by the planning skill.
- `skill@knowledge-architect-design@bounded-work` says "A reply that brings an argument, a new
  shape or a doubt returns the work to the loop, on the in-change path or the full path", and loop
  step 1 says "The classification after the search still decides the path". Read together they
  admit two readings, whether a search run after such a reply may still class the work bounded; the
  session of instance 5 took the second. That file's three proposals, input to the discussion: remove the
  bounded path, the in-change and full paths covering the rest, with a spec for small changes as
  the cost to weigh (the owner reported thinking of it too); keep it and narrow it, so that a
  discussion in which a reply brought an argument, a shape or a doubt is never bounded, whatever the
  search finds; or have the one message list each decision with the entry test it fails, so the
  owner can contest the class.

### Why it matters

A wrong path skips what the other path owes. Instance 1 skipped, before its work, the code-split
scout, the plan reviews and the committed transcript of the discussion, the record its own work set
out to keep; the spec written after the work, and its review, then supplied them. A
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
