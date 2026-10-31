# 2026-10-25-rootpoint-span-commands-slice-klarch-workflow

- **Received file**: 2026-10-25-rootpoint-span-commands-slice-klarch-workflow.md, the workflow file of
  a session of a project that installs the workflow, which implemented one slice of a milestone. The
  project's own file of the same retrospective was read for grounding only.
- **Version used**: knowledge-architect pinned at `=0.8.0`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**: `design@agent-skills@default-ruled-before-merge`,
  `goal@agent-skills@no-contradiction-in-workflow`,
  `tripwire@agent-skills@expectation-set-closes-a-contradiction`,
  `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default`.

## W1: the planning skill stops a slice at a choice between two shapes, where the primer builds the default

- **a.** The design audit found two gaps, each a choice between two shapes of a command's grammar that
  the plan documents did not rule between. The session put both to the owner with a default each,
  built the defaults and went on, per `primer@question-authority`: "A default put to the owner is
  built, and the branch that builds it merges once the owner has ruled on it". The planning skill's
  point 2 calls such a gap **Load-bearing**, "a choice between two shapes neither of which the
  document rules out", and says it "stops the slice, the whole of which rests on the decided design
  the gap defeats". Three reviewers raised the contradiction, and the owner of that project ruled that
  the primer applies. The proposal: move the choice between two shapes out of **Load-bearing**; it is
  put to the owner with a default, built, and merged once the owner rules.
- **b.** Still applies: `path@agent-skills@content/skills/planning/SKILL.md`, lines 374-376 and `:426-429`
  at HEAD, the same words as at `v0.8.0`. The primer's sentence is unchanged.
- **c.** No issue. `design@agent-skills@default-ruled-before-merge` holds back only "A question with
  no default the work can build on, a contradiction or an action that cannot be undone", and says
  the primer's rule applies as the planning skill's slice procedure builds it: the head and the
  bullet diverge. The stop's reason entered in `04fa50be`, where it replaced a clause that
  contradicted it; it argues for a gap that defeats a decided design, a material gap, not for a fork
  the design never decided. `tripwire@agent-skills@expectation-set-closes-a-contradiction` does not
  fire: this outcome cites no expectation set.
- **d.** Wrong: two installed instructions give opposite moves for one case, which is always in
  scope.
- **e.** The installed text, against `goal@agent-skills@no-contradiction-in-workflow`. The fix brings
  the planning skill into line with the primer and with `design@agent-skills@default-ruled-before-merge`;
  it reverses no head.
- **f.** Rival: keep the stop and narrow the primer. It loses: the primer's rule has a head, and the
  stop's own reason does not reach a fork. Rival: no change. It loses: the contradiction leaves a
  session no move satisfying both.
- **g.** The installed text at its source. The **Load-bearing** bullet keeps a material gap and a gap
  that needs a ruling the document marks as the owner's; its first sub-bullet, a choice among shapes
  put to the owner in one message with a default, becomes a sort of its own, built on its default.
  Instructions: one moved and reworded, 0 net.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: a contradiction between two installed
  instructions is repaired, per `goal@agent-skills@no-contradiction-in-workflow`, toward the side a
  head argues, `design@agent-skills@default-ruled-before-merge`.

## W2: a reviewer routes an item "to another axis" that runs blind to it

- **a.** The routing reviewer ended two items of "Met outside the task" with "Whether either earns a
  head is the decision-recording axis"; that axis ran in parallel and judged neither, and the
  dispatcher gave them no outcome until the transcript reviewer found them. The proposal: one sentence
  that an item a reviewer routes to another axis is the dispatcher's to judge.
- **b.** `skill@knowledge-architect-review@what-review-leaves` already says, unchanged since
  `v0.8.0`, "**Every item of a report gets an outcome**", a note outside the reviewer's axis
  included; the routing reviewer is told to list such an item for the dispatcher.
- **c.** No record.
- **d.** A lapse of the dispatcher: the instruction covers every item of a report.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: the instruction exists and covers the case;
  a lapse is not repaired by adding text, per
  `skill@knowledge-architect-retrospective-intake@admission-rules`.

## The standing answers

- **a.** The session did not carry four "Decided" items of the reviewers into its own report until
  the transcript reviewer found them; it wrote an argument into a plan document and a head without
  checking it; it committed without `check --staged`, and `commits` reported the unstaged index.
- **b.** Each rule exists at `v0.8.0` and at HEAD: `primer@decided-section`,
  `primer@verify-before-relying`, `primer@checker-model`.
- **d.** Lapses, caught before the merge.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: lapses of clear instructions.

## The checker: no finding

- **a.** Each finding the checker gave was a defect of the session's text, and each message named
  its repair.
- **d.** An observation.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: observations need no action.
