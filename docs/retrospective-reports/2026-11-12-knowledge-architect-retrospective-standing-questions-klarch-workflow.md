# 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow

- **Received file**: 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from source: main at `v0.9.0-384-g5ce466db`, branch `retrospective-standing-questions`, tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@query@occasion-shaped-output`: conflicts, check-reversal.
  - `design@agent-skills@verdicts-reach-the-conformance-review`: constrains, respect.
  - `design@agent-skills@session-acts-on-the-verdicts`: constrains, respect.
  - `design@agent-skills@outcome-over-display`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.
  - `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@transcript@transcript-command-reads-any-project`: constrains, respect.
  - `design@transcript@cut-text-reported`: conflicts, check-reversal.
  - `design@agent-skills@transcript-found-by-listing`: constrains, respect.
  - `design@knowledge-architect@transcript-crate`: constrains, respect.
  - `design@query@record-is-every-register-entry`: conflicts, check-reversal.
  - `design@query@packed-by-search-unit`: constrains, respect.
  - `design@agent-skills@needless-instruction`: constrains, respect.
  - `design@agent-skills@search-reads-the-whole-record`: conflicts, check-reversal.
  - `design@agent-skills@search-runs-once-the-shapes-settle`: conflicts, check-reversal.
  - `rejected@agent-skills@delta-search-at-convergence`: constrains, respect.
  - `issue@agent-skills@a-lighter-pass-for-untouched-components`: touches, respect.
  - `design@agent-skills@one-session-cost-weighed-before-removal`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.
  - `goal@the-workflow-improves-through-real-use`: constrains, respect.

## respect-verdict-asks-get-no-outcome: an ask inside a `respect` verdict's reason got no outcome

Members: 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow W2.

- **a. What it says.** Of 131 generated lines, most `constrains, respect`, "Four reports carried a
  specific ask inside a `respect` verdict (narrow a question, add a field, add each instance to an
  issue, record a later loss); the session wrote no outcome for them, and the transcript reviewer
  reported them as items with no durable outcome." The blamed sentence, loop step 1 of
  `skill@knowledge-architect-design@discussion-loop`: "The round after the search states the outcome
  of each verdict other than `unrelated`: the answer the discussion gives, that the design respects
  it, or that it stays open." The two readings: the generated line itself satisfies "respects it", or
  each ask in a reason needs an outcome. Proposed: step 1 says an ask in a searcher's reason takes an
  outcome like a verdict, or the generated lines carry an outcome slot.
- **b. Whether it still applies.** The sentence reads so at `5ce466db` (the file's version, which
  holds the verdict shape: `git merge-base --is-ancestor 48f64a10 5ce466db`) and at origin/main,
  `v0.10.0-59-g605d1538`, in `path@agent-skills@content/skills/design/SKILL.md`. The same step's
  bullet list, a few lines above it, says what a `respect` verdict asks: "`respect`: the shapes keep
  within the quoted sentence". The searcher's text defines the reason as "one line: what in the work
  meets what in the entry, the seed it cites, or, for `unrelated`, how the quoted subject differs from
  the work's", which holds no ask. The transcript reviewer lists "a finding of a reviewer or of
  another agent, as it arrived in the session" among what must outlive the session, in
  `agent@knowledge-architect-transcript-reviewer`.
- **c. Whether it is already recorded.** The verdict's reading is settled by
  `design@agent-skills@verdicts-reach-the-conformance-review`: "A `constrains` verdict asks the work
  only to keep within the sentence it quotes". An outcome slot in the generated lines widens what
  `design@query@occasion-shaped-output` writes ("with no quotation and no reason"), graded
  `conflicts`.
- **d. Its kind.** Not a defect of the step: its own bullet says what a `respect` verdict asks, and
  the generated line with that outcome answers it. An ask beyond the quoted sentence is a finding of
  another agent, outside the verdict, and the transcript reviewer's list gives it an outcome, which
  is how these four reached a durable outcome before the merge. The asks in the reasons are a lapse
  of the searchers against the reason's definition, and the missing outcomes a lapse of the session
  against the transcript reviewer's rule, which caught them. One session.
- **f. A better fix.** Rivals: an instruction that each ask in a reason takes an outcome adds a
  rule for one interaction, where the transcript review already gives every agent's finding an
  outcome; an outcome slot in the generated lines conflicts with
  `design@query@occasion-shaped-output`. A conformance rule is worth text only where the default
  behaviour is systematically wrong, per `skill@knowledge-architect-retrospective-intake@admission-rules`;
  one session shows no such default.

Default: no change.
Outcome: decided by the session: no change. Ground: the step's own `respect` bullet and
`design@agent-skills@verdicts-reach-the-conformance-review` settle the verdict's outcome; an ask
beyond the quoted sentence is a finding of another agent, which the transcript review gives an
outcome, as it did here; one instance shows no systematic default.

## whole-record-search-for-narrow-work: a search of the whole record runs for a narrow or follow-up piece of work

Members: 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow W1; 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow W3; 2026-11-14-thaum-casting-process-design-klarch-workflow C2.

- **a. What it says.** Three sessions report the cost of a search of the whole record for narrow
  work. standing-questions W1 (needless, "Low confidence: one session"): an in-change discussion
  editing one skill, two fragments and their design home ran 18 agents; 4, given clusters of the
  checker and the model (223 entries), graded every entry `unrelated`, and 2 returned one entry each.
  transcript-reader-gaps W3 (unclear): a follow-up request the same day, labelling one more harness
  line shape, ran 18 agents, about 2.7 million subagent tokens; 6 graded everything `unrelated`; of
  6 verdicts that asked something, 4 repeated the first search's, and 2 were new (the issue the work
  closed, one `touches`). casting C2 (no finding, a measured cost): a second search for seven threads
  packed the whole record into 15 agents and found three conflicts that changed the design. The
  blamed text, loop step 1 of `skill@knowledge-architect-design@discussion-loop`: "Once the shapes
  settle, and before any thread closes, a search of the whole record runs, with the draft as its
  work". Fixes: weigh whether the scope may follow the work's reach, no instruction until a second
  session reports it (standing-questions); none, a searcher seeded with the first search's verdicts
  named as what would remove it (transcript-reader-gaps); none (casting).
- **b. Whether it still applies.** Yes: the step reads so at origin/main, `v0.10.0-59-g605d1538`,
  in `path@agent-skills@content/skills/design/SKILL.md`.
- **c. Whether it is already recorded.** Yes. `design@agent-skills@search-reads-the-whole-record`:
  "The search gives no cluster, remainder or goals file a lighter reading because the work seems not
  to touch it", on the owner's belief that the whole record is the safer reading. Its parked rival is
  `issue@agent-skills@a-lighter-pass-for-untouched-components` (kind `design`), whose re-entry point
  is "a change to how the record search clusters, packs or writes the record, whose work weighs the
  search's cost; the first piece of work in the owner's planned project of about 30 Components; or
  the owner's word", and which says "An intake of a retrospective file that reports the search's cost
  is no re-entry point" and holds the costs measured so far. A follow-up narrowed to what changed is
  limited by `rejected@agent-skills@delta-search-at-convergence`, whose missed entries "were each
  recorded days before the search ran". transcript-reader-gaps W3's own figures weigh against a
  seeded follow-up: 2 of its 6 asking verdicts were new against the first search of that day.
- **d. Its kind.** An observation of cost, on a recorded decision; standing-questions W1 labels it
  needless, but its own "weighed against its other occasions" grants that the instruction's reason
  holds for other work, and `design@agent-skills@one-session-cost-weighed-before-removal` weighs a
  one-session cost against every other occasion.
- **e. Its scope.** The installed design skill and `design@agent-skills@search-reads-the-whole-record`,
  with `design@query@record-is-every-register-entry` (graded `conflicts` against a scope by
  Component). A narrowing would reverse both heads, against `goal@agents-work-without-drift`, which
  comes first where two purposes compete.
- **f. A better fix.** None now. The searcher's owner-question, whether three reports now reopen the
  head, is answered by the record: the issue names its re-entry points and states that an intake is
  none, so the costs are added to it as evidence and the owner's word stays one of its re-entry
  points.
- **h. The proposal.** Add the three costs to `issue@agent-skills@a-lighter-pass-for-untouched-components`,
  after its paragraph on the intake of forty retrospective files, in the commit that writes the
  analysis:

  ```text
  The intake of forty-seven retrospective files, 2026-11-14, added three costs, by the files' reports: the in-change discussion of 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow, W1, whose work edited one Component's installed text, ran 18 agents, of which 4, given clusters of the checker and the model, 223 entries in all, graded every entry `unrelated`, and 2 returned one entry each; the follow-up request of 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow, W3, made the day of a first search over the same record, ran 18 agents, about 2.7 million subagent tokens, of which 6 graded every entry `unrelated`, and of its 6 verdicts that asked something 4 repeated the first search's and 2 were new; and the second search of the design discussion of 2026-11-14-thaum-casting-process-design-klarch-workflow, C2, for seven threads opened after the first, packed the whole record into 15 agents and found three conflicts that changed the design.
  ```

Default: no change to the text; the three costs added to the recorded issue.
Outcome: decided by the session: no change, the costs added as evidence to
`issue@agent-skills@a-lighter-pass-for-untouched-components`. Ground: the issue's re-entry points
exclude an intake; `design@agent-skills@search-reads-the-whole-record` stands; two of
transcript-reader-gaps W3's six asking verdicts were new, against a seeded follow-up.

## transcript-list-cut-notices-other-sessions: a filtered `transcript list` prints the harness's cut notices of sessions the filter left out

Members: 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow C2; 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow C1(b).

- **a. What it says.** `transcript list --branch <branch>` and `--containing "<words>"` printed
  lines opening "# cut by the harness:" for subagent transcripts of other sessions, one naming
  "/t/x.output", before the one row that answered (standing-questions C2, `v0.9.0-384`); the same
  notices about other sessions' subagent files, "which the reader must skip" (conformance C1(b),
  `v0.10.0-35`). Proposed fix: print the notices only for the sessions listed.
- **b. Whether it still applies.** Yes. At origin/main,
  `cargo klarch transcript list --branch planned-splits` lists two sessions, `80036d81` and
  `3e0588d2`, and its header prints 34 cut notices, in files of nine other sessions and of their
  subagents, 14 of them in one file,
  `<transcript directory of this repository>/221be2f0-cdd5-5e76-b391-13ea5eba2fec.jsonl`; none is in a file
  of the two sessions listed (counted by `grep '^# cut'` over the output, grouped by file). The
  header's line comes from `design@transcript@cut-text-reported`: "Every verb's header prints one
  line per such marker in a kept line its fields label injected or tool"; the header is taken over
  every file the verb reads, as the unrecognised values are, per
  `design@transcript@transcript-format-tolerated-and-reported`.
- **c. Whether it is already recorded.** No entry records it. The verdict on
  `design@transcript@cut-text-reported` is `conflicts`, `check-reversal`: a header narrowed to the
  listed sessions changes what the head says every verb's header prints.
- **d. Its kind.** Needless output of the checker: a notice whose reader is the reader of the cut
  session, printed to the reader of a query that excludes it.
- **e. Its scope.** The owner of the fix is `transcript`; the fix rewrites
  `design@transcript@cut-text-reported`, so it goes through `skill@knowledge-architect-design`.
- **f. A better fix.** The question the discussion answers is what a header is about. An
  unrecognised value is a fact about the reader, true whichever session holds it, which argues for a
  header over every file read; a cut text is a fact about one session's content, which a reader
  needs only for a session it reads. The rival is no change, since the notices are cheap; the
  deciding fact is that in the run above the header carried 34 notices, every one about a session it
  excluded, before the two rows asked for. The same discussion can settle
  `transcript-shows-only-persisted-preview`, a second kind of cut the head does not name.
- **g. Its route.** An issue only: a design discussion and a branch of `transcript`.
- **h. The proposal.** Open a design issue in `transcript`,
  `the-header-names-the-cuts-of-sessions-a-filter-left-out`, with the run above, the proposal and
  the distinction between a fact about the reader and a fact about a session.

Default: open the issue.
Outcome: decided by the session: open an issue, `transcript`, kind `design`, `the-header-names-the-cuts-of-sessions-a-filter-left-out`. Ground: the notices stand at origin/main by measurement, and the fix narrows what `design@transcript@cut-text-reported` says every header prints.

## standing-2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow, lines 80 to 117,
the first file to answer the questions its own branch added. Its counts are in the lines for
`issue@agent-skills@the-review-cost-counts-are-not-compared-yet`; its tool report is item 2 of the
owner item in the analysis of 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.

Carried by a subject, named only:

- the first repair commit's message claiming a change "in the skill" the diff did not hold, and a
  draft arguing that a rival file "never goes upstream" (lines 84 to 88):
  `message-claims-written-unchecked`;
- the 18 searchers' quotation repairs (line 94): `quotation-check-refuses-cut-punctuation`; the 7,
  then 3, then 1 detached worktrees and scratch directories made by a shell loop (lines 95 and 96):
  `reviewer-worktrees-by-hand`.

Analysed here:

- **A record searcher wrote its helper scripts outside its own scratch directory, in the shared
  work-list directory** (lines 89 and 90). Kind: a lapse of one agent against its brief, which gives
  each agent its own directory, as `primer@dispatching-work` asks of agents dispatched together;
  nothing was lost.
- **The instructions each change adds counted by hand for the net figure** (line 97). Kind: a task
  carried by hand, but not one with no judgement: what counts as an instruction is the admission
  standard's definition, applied by reading, so no command takes it.
- **Work an agent could have carried** (lines 99 to 101): implementation and repairs carried by the
  session, "since the verdicts and the reviewers' findings it acted on were in its context. Cost: a
  long context for the last rounds". Kind: the dispatch judgement of `primer@dispatching-work`, the
  piece the session "will reason later over"; an observation.
- **Missed entry**: "None found. Every entry the reviews named ... had been returned by the search;
  the reviews found the session had not acted on some" (lines 107 to 109). Kind: an observation of the
  search; the verdicts returned and not acted on are the session's side, of which W2 of this file,
  `respect-verdict-asks-get-no-outcome`, analyses one kind.
- **Installed file changed**: the source, as the work. **Project skill addition**: no.
  **Backticks**: no. **Code rules**: no Rust. **Agents unfinished**: none. **A conclusion wrong**:
  none; one figure re-counted. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: one lapse against a brief with nothing lost,
one count that needs judgement, and observations.

## tripwire-rule-title-wider-than-its-argument: one branch toward its two-branch clause; unfired

Members: `tripwire@agent-skills@rule-title-wider-than-its-argument` (verdict `unrelated`); W3 of
2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow, in the subject
`touched-head-owes-whole-audit`.

- **The clause.** "the reviews of two separate branches each find a title wider than its argument
  in a head their branch writes or rewrites, or an audit finds one on the main branch in a head
  written or rewritten after the decision."
- **The one branch.** W3 of the standing-questions file, lines 55 to 58: "adding a member to a head
  led the first decision-record review to retitle it; the re-review of the repair found the new
  title read as a universal and asked for a third wording." A title read as a universal is a title
  wider than its argument, found by a review in a head the branch rewrote, so the branch counts as
  one of the two. No other file of the batch reports one: `grep -n -i "wider than\|universal\|title"` over the 26
  workflow files finds, beside this W3, rule quotations, the restatements of an old title a reversal check
  missed (transcript-unrecognised-lines, standing), a stale index line after a retitle, and a
  head reread against its title (component-arguments W4), none a title
  wider than its argument.
  Unfired.
- **The horizon** is not counted in analyses: "20 commits on the main branch that add a level-three
  heading to a design home, and one pass of the design-record axis", its lines blamed to 2ad7d3d9
  and b985e14c. This intake's commit adds no heading to a design home, so it is no occasion. The
  count of one branch is kept in this analysis and in history, as the standard of `skill@knowledge-architect-issue-tracking@tripwire-entry` keeps
  no count in the entry.

Default: no change; the tripwire stands at one branch of two.
Outcome: decided by the session: no change, unfired. Ground: the clause needs two branches and the
batch shows one; the intake's commit is no occasion of its horizon.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## reviewer-worktrees-by-hand: each reviewer's worktree and scratch directory made by hand

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `reviewer-worktrees-by-hand`, which gives the outcome.

## touched-head-owes-whole-audit: a head that a change touches owes its whole conformance, older debts included, at any edit

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-issue-triage.md`, in its section on `touched-head-owes-whole-audit`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
