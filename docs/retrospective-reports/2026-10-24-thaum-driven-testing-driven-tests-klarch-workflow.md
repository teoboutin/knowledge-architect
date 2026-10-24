# 2026-10-24-thaum-driven-testing-driven-tests-klarch-workflow

- **Received file**: 2026-10-24-thaum-driven-testing-driven-tests-klarch-workflow.md, the workflow
  file of a session in thaum that worked one milestone slice from its design audit to its merge, the
  work delegated to five implementation subagents on separate worktrees.
- **Version used**: the published checker, 0.8.0, read at the tag `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`; the planning and review skills, the primer, the
  standing-entry searcher's definition and the checker's messages are identical at both.
- **Standing entries that bear on it**:
  - for W1: `design@agent-skills@standing-entry-search-agent`,
    `design@agent-skills@standing-entries-searched-before-the-work`,
    `tripwire@agent-skills@search-missed-before-the-work`, not met: a searcher returned the four
    entries, and the record omitted them;
  - for W3: `design@agent-skills@plan-records-the-exchange`,
    `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access`, about another actor;
  - for W2 and C1: see the clusters they point to.

## W1. The seeds of the standing-entry search at a slice's audit can be read two ways

- **a. What it says.** Point 2 seeds the audit's search with "the decisions the milestone document
  lists under "What is already decided"". That section holds the decisions the design rests on and a
  table of the decisions the work reverses, rewrites or extends. Six searchers read it three ways: the
  first part only, both parts, or every reference of the section, issues and tripwires included,
  though the searcher's definition says a seed is never an issue or a tripwire. The groups searched
  with different seeds, and four entries only the wider set reached got no line in the audit's record;
  the transcript reviewer found the gap. The fix proposed: point 2 says whether the decisions listed as
  reversed or rewritten are seeds, and that a seed is never an issue or a tripwire.
- **b. Whether it still applies.** Yes. Point 2 of `skill@knowledge-architect-planning@working-a-slice`:
  "as seeds, the decisions the milestone document lists under "What is already decided" and the
  decisions and goals the slice's spec cites". The row of that section names the decisions the design
  rests on, each decision the work reverses or rewrites with every text that cites it, and the
  standing and design entries the discussion's searches returned. The searcher's body: "**The seeds**:
  the decisions and goals the work names, as references, or none. A seed is never an issue or a
  tripwire." Its description, the text a dispatcher reads, says only "the seeds (the decisions and
  goals the work names, or none)".
- **c. Whether it is already recorded.** `design@agent-skills@standing-entry-search-agent`: "A seed is a
  decision or a goal the work names"; on that definition a decision the work reverses is a seed.
- **d. Its kind.** Unclear, provable by reading: one sentence names a section that holds three kinds of
  entry, and the rule that excludes two of them sits where the dispatcher does not read it.
- **e. Its scope.** `goal@knowledge-architect@agents-get-a-complete-workflow`; installed text, the
  planning skill and the searcher's description.
- **f. A better fix.** Rivals: the proposal; the head's definition delivered in the searcher's
  description, which every dispatcher reads, not only the slice audit; both. The second covers every
  dispatch, and point 2's phrase gains "those the design rests on and those it reverses or rewrites",
  which the head's definition already implies. Instructions added: 0, two rewordings.
- **g. Its route.** An installed text edit.
- **h. The proposal.** Handle now: point 2 names both kinds of decision as seeds; the searcher's
  description says a seed is a decision or a goal, never an issue or a tripwire.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W2. Where the record of a review goes when the transcript review's repairs come after it

This finding is analysed with its cluster in
`path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`,
section "W2, with the findings on how the review before a merge ends".

## W3. A safety classifier of the harness blocked an edit the planning skill asks for

- **a.** Writing a default awaiting the owner into the milestone document, with the owner's closing
  words of the thread it bears on, the harness's classifier refused the edit; the session put the
  default to the owner in the conversation and recorded the ruling in a commit message. The file
  proposes nothing unless it recurs.
- **b.** The planning skill lists a scope change "in the milestone document as a default awaiting the
  owner", and keeps a thread's closing words "verbatim, with their round", per
  `design@agent-skills@plan-records-the-exchange`; the verbatim rule is a thread's, and asks no
  quotation in a default.
- **d.** An observation of the harness, one instance; the session's route kept the ruling.
- **h.** No change.
- **Default**: no change, an observation.
- **Outcome**: accepted.

## C1. A slice's id in a commit subject is reported as a plain word, with a repair that does not fit

This finding is analysed with C1 of 2026-10-20-thaum-pin-move-0-8-0-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-20-thaum-pin-move-0-8-0-klarch-workflow.md`.

## The standing answers

- **a.** Rules acted on as if absent: commit messages of the session and of two implementation
  subagents with claims no check supported, reworded by history edits before the merge; items its
  subagents reported under "Met outside the task" left without an outcome until the transcript
  reviewer listed them; a brief that told implementation subagents to prefer loops of their own, a
  member a head of the project lists by case, without reading the head.
- **b.** The rules stand: `primer@verify-before-relying`; `primer@met-outside-the-task`, "the session
  that dispatched it routes each item by this table"; the backstop of `primer@design-heads`.
- **d.** Lapses, each caught before the merge. The claims are instances of the recurrence analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`;
  the brief's member is an instance judged with the backstop's tripwires in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-20-thaum-pin-move-0-8-0-klarch-workflow.md`.
  The unrouted items are one instance of a clear instruction.
- **h.** No change for the unrouted items.
- **Default**: no change, as h says.
- **Outcome**: accepted.
