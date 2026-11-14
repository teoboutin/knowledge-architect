---
kind: design
---
# The record of a whole-set goal review must hold reviewers' material that is not in checked forms

## Summary

The record of a review of the whole goal set is a committed document the checker reads, and it
must hold each round's boundary blocks, which come from reviewers who quote the tree in forms the
checker refuses, and name each round's version by a commit of the branch. One session in a project
that installs the workflow met the check failing by the thousand and rewrote the material by an
agent. No text says who converts it, nor which rounds' blocks the record keeps.

## Details

### What

The text, step 4 of `skill@knowledge-architect-goal-setting@whole-set-review`: the record keeps,
"for each round, the findings counted per reviewer, rule, grade, route tag and old or new wording,
each finding's outcome, each default with its ground, each question put to the owner, labelled
`G<n>`, with the owner's answer verbatim, and each goal's boundary block, from the round's
reviewers; and the list of questions still open. Commit it at each round, with the round's drafts."
Step 4 of `skill@knowledge-architect-goal-setting@drawing-out-intent` adds "Keep the version each
round read, ... in the record for a larger one", which is where the record names the branch's
commits. The decision is `design@agent-skills@whole-set-rounds`: the record holds "the owner's
answers verbatim and each round's boundary blocks, from that round's reviewers", and "the boundary
blocks of that round alone describe" the goals the owner rules on; a session's scratch space, the
nearest rival, "is lost to the next session".

The instance: thaum's move of its pin to 0.10.0, 2026-11-13, W4 and C1 of its workflow file. The
reviewers' material quoted the tree with bare slugs, anchorless paths, `file:line` locations and
rule numbers, so the record failed the check by the thousand, 1,597 findings at six rounds as the
file reports. It also cited the branch's own commits as the trees each round read, which
`cargo klarch commits` refuses on a branch, per `design@check@branch-shas-are-refused`, and a squash
removes. An agent rewrote the material into checked forms, 976 references, 404 paths and 107 rule
quotes, and replaced 258 SHAs by labels, as the file reports; the figures are not re-measured.

The suspected mechanism, in one sentence: the record is a checked document, and the blocks it must
hold come from reviewers whose reports are not written in checked forms.

The shapes:

- every block in the scratch directory, for the transcript reviewer, and the record keeping counts,
  outcomes, questions and the owner's answers in checked forms, W4's proposal; it loses the blocks
  the owner rules on, the head's reason, and meets the reason of
  `design@agent-skills@committed-audit-report`, that evidence in a scratch directory was needed
  after the pass;
- the record keeping only the stopping round's blocks, in checked forms, the earlier rounds' in
  scratch, which keeps what the owner rules on and drops what the head argues no need for;
- the session writing every block in checked forms, the text as it stands, at the cost the
  instance measured;
- the record exempted from the check, which strains `goal@documentation-stays-consistent`.

`design@agent-skills@goal-draft-shows-reach` keeps blocks out of the goals home wherever they are
kept. Ruled out for naming a round's version: a checked form for a commit named by its subject,
`rejected@agent-skills@checked-commit-form-by-subject`, against which the file brings no new
argument; each round's drafts are committed, per
`design@agent-skills@goal-batch-drafts-committed-each-round`, so each round's version is a commit
with a subject until a squash removes it.

Re-entry point: a design discussion under `skill@knowledge-architect-design` on a branch of its
own, or the next whole-set review that meets the cost.

### Why it matters

The owner rules on the goals with the blocks of the round that stopped, per
`goal@the-owner-decides`, so the blocks must survive the session; held as the text asks, they cost
a conversion no text assigns, and a record that names its rounds by branch commits loses them at a
squash. It strains `design@agent-skills@whole-set-rounds`.

### What would close it

The discussion settles which blocks the record keeps, in which forms and who writes them, and how
a round's version is named when the round commits are squashed, and rewrites
`design@agent-skills@whole-set-rounds` and the two steps of the goal-setting skill to it; or it
keeps the text with its cost stated in the head.
