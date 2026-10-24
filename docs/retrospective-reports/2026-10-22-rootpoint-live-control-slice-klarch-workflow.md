# 2026-10-22-rootpoint-live-control-slice-klarch-workflow

- **Received file**: 2026-10-22-rootpoint-live-control-slice-klarch-workflow.md, the workflow file of
  a session in rootpoint that worked one milestone slice from its design audit to a local merge, with
  a design discussion opened at the audit, seven plan reviews and four history edits of commit
  messages.
- **Version used**: the published checker, 0.8.0, read at the tag `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`; the installed text this file blames is the
  same at both.
- **Standing entries that bear on it**:
  - for W1: `design@agent-skills@statement-names-an-act`, `design@agent-skills@capabilities-not-structure`,
    `tripwire@agent-skills@agent-text-findings-need-nothing`, which the file's counts cannot judge;
  - for W2: `design@agent-skills@one-decision-per-head`,
    `rejected@agent-skills@head-split-by-independent-reversal`,
    `tripwire@agent-skills@split-rule-from-exception`, not shown to fire: what was split was a
    delivery and a parameter, not an exception;
  - for W3: `design@agent-skills@losing-alternatives-filter`,
    `rejected@agent-skills@every-losing-thread-recorded`;
  - for the cluster under W4: `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`,
    whose closing condition this file meets; `design@core@a-commit-message-is-a-document`,
    `rejected@core@commit-msg-hook`, `rejected@core@commit-subcommand`,
    `design@core@check-judges-no-given-commit`, `design@core@staged-check-semantics`,
    `design@agent-skills@staged-check-before-each-commit`,
    `issue@core@core-heads-bundle-decisions-with-many-citing-sites`,
    `tripwire@agent-skills@rarer-rule-in-the-model`, not met: no finding message delivers the rule
    proposed;
  - for C2: `design@agent-skills@ruled-items-labelled`, `design@core@plan-items-by-section`,
    `design@core@one-question-one-command`.

## W1. Two runs of the agent-text axis gave opposite outcomes on one clause

- **a. What it says.** A scoped CLAUDE.md sentence described a view; the first run of the agent-text
  reviewer judged it a factual error, since the code also draws a label, and asked that the label be
  named; a second run, at the review before the merge, judged the quoted strings to carry no act and
  asked that they be deleted. The fix proposed: one sentence after "a suspect statement found false
  is a factual error", that a description of the code is complete when it names what the
  instruction's act needs.
- **b. Whether it still applies.** The need test is unchanged,
  `skill@knowledge-architect-agent-configuration@admission-standard`: "**name the act of an agent
  following the workflow that would differ without it**". Two corrections to the premise: the file
  cites the sentence on suspect statements, and "suspect" is defined as "A statement that can turn
  false with no change to the project", which a description of the project's own code is not, so the
  ground of the first verdict is the factual error that "A defect provable by reading needs no other
  evidence" names; and the two runs judged two texts, the second judging the clause after the first
  run's repair.
- **c. Whether it is already recorded.** `design@agent-skills@statement-names-an-act`: "The test's
  reach, stripping what is not fully necessary at the price of some variation between its readers,
  is the owner's weighing: a statement one reader passes is met by a later pass." The same head cites
  `design@agent-skills@capabilities-not-structure` against content rules that grow one clause per
  case observed.
- **d. Its kind.** An observation within the variation the owner weighed and accepted; the two
  verdicts were on two texts, each consistent with the test.
- **f. A better fix.** Rivals: the proposed sentence, a rule written for one interaction, which
  argues against the owner's weighing; no change.
- **h. The proposal.** No change.
- **Default**: no change, as d says.
- **Outcome**: accepted.

## W2. "A rule's delivery belongs to its head" and "one decision per head, by nearest rival" pull apart

- **a. What it says.** The harvest joined into one head a rule, its parameter, its exceptions and its
  delivery; two reviewers split it, since the delivery lost to a rival of its own, a key binding the
  owner ruled out, and the parameter to another. Following the second sentence produced a finding
  under the first. The fix proposed: "… since none means anything without the rule, unless one loses
  to a rival of its own, which makes it a decision."
- **b. Whether it still applies.** Yes, `primer@design-heads`: "**A head holds one decision.** Two
  statements are one decision when they answer one question and lose to the same nearest rival. A
  rule's exception, its parameter and its delivery belong to its head, since none means anything
  without the rule. Two statements that lose to different nearest rivals, or whose arguments share no
  premise, are two decisions."
- **c. Whether it is already recorded.** `design@agent-skills@one-decision-per-head`, whose title
  already gives the rival test precedence: "A head holds one decision, and parts that lose to
  different nearest rivals are separate heads". Its argument rejects the independent-reversal test
  because it "splits a rule from its exception, which always passes it",
  `rejected@agent-skills@head-split-by-independent-reversal`; `tripwire@agent-skills@split-rule-from-exception`
  watches that an exception is not split from its rule.
- **d. Its kind.** Unclear: the primer's sentence reads as an exemption from the rival test, which
  the head's title denies.
- **e. Its scope.** `goal@knowledge-architect@design-is-recorded-with-its-arguments`; installed text
  and the head whose body holds the same sentences.
- **f. A better fix.** Rivals: the proposal, which brings the primer to the head's title; the
  sentence dropped, which the file rules out since it would split every parameter from its rule. The
  proposal reaches exceptions too, which the head's tripwire guards, so its wording is checked
  against that tripwire in the design skill's in-change path. Instructions added: 0, a rewording.
- **g. Its route.** A decision, under the in-change path of `skill@knowledge-architect-design`, then
  an installed text edit of the primer and the head's body.
- **h. The proposal.** Handle now: the primer and the head say that an exception, a parameter or a
  delivery belongs to its rule's head unless it loses to a nearest rival of its own.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W3. Test 1 for a losing alternative admits nearly every rival of an interface decision

- **a. What it says.** The harvest admitted three losing alternatives on test 1 alone and left two
  others that would change the same interface as "an argument a reader derives again"; the
  decision-record reviewer reported the inconsistency, and the repair added three entries. Read
  literally, test 1 admits every rival of a head that earned its place by entry test 1, so "Most
  designs not kept earn no entry" does not hold there. The fix proposed: test 1 reads "it would change
  an interface others consume, and its refutation cannot be derived again from the head that cites
  it".
- **b. Whether it still applies.** Yes. `skill@knowledge-architect-decision-recording@losing-alternatives`:
  "**Most designs not kept earn no entry.**"; test 1, "it would change an interface others consume,
  as entry test 1 of `primer@design-heads` defines it"; and "Test 3 is the load-bearing one. … What
  decides an entry is whether the refutation can be derived again inside that round", which can be
  read to bind every test or test 3 alone.
- **c. Whether it is already recorded.** `design@agent-skills@losing-alternatives-filter` lists test
  1 as standing alone and argues only from derivation; its list and its argument part in the way the
  file describes. `rejected@agent-skills@every-losing-thread-recorded` is `live` with the owner's
  judgement that the filter is "still a little imperfect".
- **d. Its kind.** Unclear, provable by reading: one sentence of the tests reads two ways, and the
  head's list and argument disagree.
- **e. Its scope.** `goal@knowledge-architect@design-is-recorded-with-its-arguments`; installed text,
  and a head whose statement the narrowing changes.
- **f. A better fix.** Rivals: the proposal, which narrows test 1 to its argument; test 1 kept broad
  and "Most designs not kept earn no entry" dropped, which inverts the home's intent for interface
  decisions; the derivation sentence placed so that it binds every test, which narrows all of them.
  The choice changes a head the owner judged imperfect, so it is a design question.
- **g. Its route.** An issue only: a design question not for this session.
- **h. The proposal.** Open an issue: a `design` entry in agent-skills on whether derivation limits
  every test for a losing alternative, with this case and the head's disagreement as its evidence.
- **Default**: open an issue, as h says.
- **Outcome**: accepted.

## W4 and C1, with C2 of 2026-10-24-thaum-driven-testing-short-deck-soak-klarch-workflow. A commit message is judged only after the commit exists

- **a. What they say.**
  - W4 here: the primer asks for `check --staged` before each commit, which judges the tree and not
    the message. A harvest message named a just-removed tripwire by its bare slug, which resolves
    against the first parent that still held it: a finding of `commits`, seen only at the review
    before the merge and repaired by a history edit. C1 proposes `check --staged --message <file>`,
    judging a draft against the staged tree and HEAD, and W4 a primer line for it once it ships.
  - C2 of the short-deck-soak file: the session judged each message only after the commit; three
    failed and were amended, one had been pushed, by a push chained with `;`, and a force-push
    followed. It proposes `commits --message <file>`, or a `commit-msg` hook the setup skill installs.
- **b. Whether they still apply.** Yes. `primer@checker-model`: "`cargo klarch check --staged` judges
  the tree a commit will record … Run it after staging and before each commit"; it gives no moment for
  `commits`. No such option exists. Measured by a checking agent with the binary at the analysed
  commit: a commit deleting an issue entry with a message naming its bare slug fails `commits` with
  "is written in backticks without its kind". The live-control file's standing answer says those
  bare slugs were allowed "as `primer@where-knowledge-goes` allows"; the sentence there allows a bare
  name "where its tree does not hold it, nor, for a commit message, its first parent's", placed after
  the clause on a skill or an agent, so it can be read to bind that clause alone. That reading is a
  second cause of W4's finding.
- **c. Whether they are already recorded.** Yes.
  `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text` predicts
  this case, and its close: "A retrospective that reports a message finding first met at the gates, in
  a project that follows the installed text: then `primer@checker-model`'s staged-check line gains
  "and `{command} commits <main>..HEAD` after it"". W4 is that report, met at the review before the
  merge. The proposals for a draft check meet a head and two rejected designs:
  `design@core@a-commit-message-is-a-document`, "**One command judges messages, after the commit
  exists.** … No hook judges a draft before the commit exists … it resolves a message against HEAD,
  which under an amend, a reword or a squash is the commit being replaced";
  `rejected@core@commit-msg-hook`, which lost on three counts, among them "a rebase rewording two
  messages that `commits` accepted was refused by it twice"; `rejected@core@commit-subcommand`. A
  `check --staged --message` would give `check` the rules of `commits`, against
  `design@core@check-judges-no-given-commit`.
- **d. Their kind.** Missing at one point of delivery: the check after each commit, which this
  repository's root CLAUDE.md delivers and the installed text does not; and unclear, the primer's
  sentence on a bare slug in a commit message.
- **e. Their scope.** `goal@core@documents-stay-current`, `goal@knowledge-architect@setup-brings-quality-tools`;
  installed text.
- **f. A better fix.** Rivals: a draft check, a mode or a hook, which the head and two rejected
  designs refuse on reasons both files leave standing; the open issue's own repair, `commits` run
  after each commit, which catches a message while an amend repairs it, as the short-deck-soak
  session did three times; the primer's sentence reworded so that the first-parent clause binds an
  entry's slug and a skill's name alike. The second and the third answer both files' cases with no
  change to the checker. Instructions added: 1, the primer's line, which the owner is told of with
  the intake's net count.
- **g. Their route.** An installed text edit of the primer, and the issue closed.
- **h. The proposal.** Handle now: `primer@checker-model`'s staged-check line gains "and
  `{command} commits <main>..HEAD` after the commit", which closes the issue on its condition; the
  sentence in `primer@where-knowledge-goes` says that the first parent's tree counts for an entry's
  slug as for a skill's name. No draft check.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W5. An owner's ruling that arrives during the last transcript review

This finding is analysed with its cluster in
`path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`,
section "W2, with the findings on how the review before a merge ends".

## C2. No command gives the next free label or item number of a plan across its history

- **a. What it says.** Plan items and labels are never reused across a milestone's documents,
  deleted slice specs included; the session found the next numbers by grepping `git log -p` over the
  milestone's directory. The fix proposed: a listing of the highest number in use per prefix, deleted
  documents included.
- **b. Whether it still applies.** Yes. `primer@labelled-items`: "takes the next number of its prefix
  in the record the label reaches, from 1, and that number is never reused there. That record is a
  plan document, a milestone's documents counting as one". The planning skill numbers arguments "in
  one sequence across a milestone's README and its slice specs". The checker's duplicate-id check,
  `design@core@plan-items-by-section`, sees only the documents in the tree. `klarch --help` lists no
  such command.
- **c. Whether it is already recorded.** No. `design@agent-skills@ruled-items-labelled` supports the
  reading that a label reaching a document that later leaves keeps its number.
- **d. Its kind.** Missing: a command for a count over history that every slice of a milestone takes
  by hand, one instance.
- **e. Its scope.** `goal@knowledge-architect@structure-and-workflow-work-together`, "the tool
  computes the workflow's work lists"; the checker. `design@core@one-question-one-command` bears on
  its shape.
- **f. A better fix.** Rivals: a command over the plan's history; the numbers checked unique across
  history by `commits`; no change, a grep each session can run. The work is not this session's.
- **g. Its route.** An issue only.
- **h. The proposal.** Open an issue: a `todo` entry in the core, a listing of the highest number in
  use per prefix in a plan and its deleted documents, with this case.
- **Default**: open an issue, as h says.
- **Outcome**: accepted.

## The standing answers

- **a.** Rules acted on as if absent, by the session, each caught by a reviewer and repaired by a
  history edit: two claims written from memory into commit messages; the opening words of the
  owner's message quoted in a commit message. The answer on bare slugs says they were allowed by the
  primer, which is false for a commit message, as W4's measurement shows.
- **d.** Lapses, instances of the recurrence and of the attributions analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`;
  the bare-slug answer is a cause of W4, analysed above.
- The ruling is given where each is analysed.
