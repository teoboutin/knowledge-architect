# 2026-11-12-knowledge-architect-transcript-reader-gaps

- **Received file**: 2026-11-12-knowledge-architect-transcript-reader-gaps.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, at main `v0.9.0-360-gc9cf649c` (`c9cf649c`), branch `transcript-reader-gaps`, tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@pinned-binary-runs-inside-the-project`: constrains, respect.
  - `design@agent-skills@installed-text-works-with-what-an-installing-project-holds`: constrains, respect.
  - `design@agent-skills@intake-takes-the-project-file`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`: touches, respect.
  - `design@check@a-foreign-build-is-refused`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@knowledge-architect@scratch-worktree-outside-git-rules`: constrains, respect.
  - `design@knowledge-architect@a-build-is-tied-to-its-checkout`: constrains, respect.
  - `goal@serves-one-kind-of-project`: constrains, respect.
  - `goal@every-target-project-can-adopt-it`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `design@check@plain-word-lint`: conflicts, check-reversal.
  - `design@knowledge-architect@check-crate`: constrains, respect.
  - `goal@documentation-has-a-structure-and-a-model`: constrains, respect.

## searcher-build-broken-by-live-edit: the dispatcher's edit of the Rust source broke the searchers' `cargo klarch` builds

Members: 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow C3; 2026-11-12-knowledge-architect-transcript-reader-gaps P1 (interaction twins).

- **a. What it says.** During the first record search of that session, the session wrote code in the live tree while 18 searchers ran `cargo klarch worklist --check`. The tree briefly did not compile (`E0308` in a file of the `transcript` crate), and most searchers fell back to a release binary built before the edit, each reporting it. For the second search, the session wrote its code in a scratch worktree and carried the diff back with `git apply --index` after the searchers returned, "That worked". P1 blames `instructions@verify-mechanically` and `skill@klarch-development@other-checkouts`, which covers a reviewer's worktree and not the dispatcher's own edits, and proposes one instruction in that section: while subagents run `cargo klarch` over the live tree, write code in a scratch worktree and bring the diff back (+1). C3, the workflow side: "The installed text tells the searcher that nobody edits the tree; it tells the dispatcher nothing about the tree while searchers run. In a project that pins a released checker, no edit can break the searchers' checker, so the gap may bear on this repository alone." C3 proposes no installed edit.
- **b. Whether it still applies.** Yes, as blamed. At origin/main `instructions@verify-mechanically` says "`cargo klarch` runs the checker built from this checkout, in release mode", and `skill@klarch-development@other-checkouts` speaks of a reviewer's worktree, a mutation of the cargo configuration and of `path@mutate@src/`, and of no edit by the dispatcher. The installed searcher definition, `path@agent-skills@content/agents/record-searcher.md`, lines 33 and 34: "You run in the session's tree, which nobody edits while you search." No installed text tells the dispatcher so; the review skill keeps reviewers out of the live tree for this reason, "A run in the live tree races the dispatcher's own edits" (`skill@knowledge-architect-review@review-invariants`), and keeps the searcher in it.
- **c. Whether it is already recorded.** No entry records it. Bearing: `goal@every-target-project-can-adopt-it` and `design@agent-skills@installed-text-works-with-what-an-installing-project-holds` (`constrains`): a gap of this repository's checker built from the live tree lands in this repository's own texts, not the installed text. `design@knowledge-architect@a-build-is-tied-to-its-checkout` and `design@agent-skills@pinned-binary-runs-inside-the-project` (`constrains`) keep each build inside its tree, which a scratch worktree for the dispatcher's edits respects; the searcher's claim, under `design@check@a-foreign-build-is-refused`, that such a worktree "meets the same refusal" holds only for running the checker from that worktree, which P1's workaround did not do (assumption: P1 does not say where its own builds ran). `issue@agent-skills@how-parallel-work-is-partitioned-among-agents` (`touches`) concerns plan documents, and its trigger is not met.
- **d. Its kind.** On the workflow side, an observation: in an installing project the searchers' entries and the check's snapshot are copies (`design@query@occasion-shaped-output`, "`--verify` reads no model"), and the checker is a pinned binary, so an edit breaks nothing the files report. A second session of this repository, goals-first-slice (standing answer, line 143), "edited the tree while record searchers were reading it", and "The searchers graded from snapshot files", with no harm reported. On this repository's side, missing: a trap of this repository's build, a fact the session could derive from `instructions@verify-mechanically`, and did derive by the second search of the same session.
- **e. Its scope.** This repository's own text, `skill@klarch-development`, owned by the root; no goal is threatened. The installed text is out of scope by `goal@every-target-project-can-adopt-it`.
- **f. A better fix.** Rival to P1's instruction: no change, the session's judgement having found the worktree at the next search. What decides is `design@agent-skills@behaviour-additions-need-real-use`: an instruction about how an agent behaves is added on a real-use instance "with the owner naming what the session would have lacked without it". The instance is real and the mechanism is stated ("`cargo klarch` builds the checker from the working tree"); the cost was one search's checks run on an older binary, which the searchers reported, and no verdict is said to be wrong. Whether a session lacked something is a fact the owner holds. If the owner names it, the repair is P1's one sentence, placed in `skill@klarch-development@other-checkouts`, the text loaded when a session works on the Rust source, with 1 instruction added and 0 removed; its nearest rival, the root `CLAUDE.md`, loads it in every session, which never meets the occasion otherwise.
- **g. Its route.** For this repository's side, a text edit of the project's own skill, `skill@knowledge-architect-agent-configuration`, if the owner admits it; for the workflow side, nothing.
- **h. The proposal.** Workflow side: no change, decided by the session, since in an installing project the searchers read copies and a pinned binary, as C3 itself states, so the gap bears on this repository alone, per `goal@every-target-project-can-adopt-it`. This repository's side: put to the owner whether a session lacked the instruction; on the owner's word, handle now with the text below; otherwise no change.

Text to add, on the owner's word, to `path@agent-config@skills/klarch-development/SKILL.md`, section "A mutation or a review in another checkout", after its first paragraph (1 instruction added, 0 removed):

```markdown
**While subagents run `cargo klarch` over the main tree, as record searchers do, write Rust code in
a scratch worktree** and bring the diff back once they return: `cargo klarch` builds the checker from
the working tree, so an edit that does not compile breaks every subagent's build.
```

Default: no change.
Outcome: for the owner (a fact only the owner holds: whether a session would have lacked the instruction, per `design@agent-skills@behaviour-additions-need-real-use`): pending.

## skill-name-subject-prefix-refused: a commit-subject prefix that is a skill's name is refused as a plain word

Members: 2026-11-12-knowledge-architect-transcript-reader-gaps P2.

- **a. What it says.** A commit to the skill's own file, `path@agent-config@skills/klarch-development/SKILL.md`,
  "took the subject prefix `[klarch-development]`. `cargo klarch commits` refused it as a plain word naming
  `skill@klarch-development`. The session amended the prefix to `[agent-config]`, the location's
  name, which earlier commits use." Kind: "Unclear: no text of the project names the subject prefix
  a commit to a project skill takes, and the natural candidate is refused." Proposal: "Name the
  prefix where the git-flow subjects are named, or let the commits check accept a bracketed subject
  prefix as a label."
- **b. Whether it still applies.** Yes. At `v0.10.0-59-g605d1538`, `design@check@plain-word-lint`
  reports "A word outside every backtick span and every quotation ... that is exactly a defined id",
  and a word is bounded by "no letter, digit, hyphen, `/`, `@`, `#` or brace on either side", so a
  bracket bounds it; the finding's repair, in `path@check@src/check/references.rs`, is "write" the
  reference. No text of the project names a subject prefix at all: `design@knowledge-architect@git-flow`
  and `instructions@git-workflow` say nothing of one, and a search of the tracked Markdown for
  "subject prefix", "prefix" beside "subject" and a bracketed anchor name returns no rule. The
  practice is visible in the history: of the last 400 subjects of origin/main, every one but the
  release commit opens with a bracketed word, most often an anchor's name (111 `[review]`, 50
  `[check]`, 45 `[agent-skills]`), and three open with `[agent-config]`.
- **c. Whether it is already recorded.** The lint is recorded, as the owner's rule:
  `design@check@plain-word-lint` (`conflicts`, `check-reversal`) holds "**The rule is the owner's**".
  The second proposal, an exemption for a bracketed prefix, contradicts it, so it would go to the
  owner through `skill@knowledge-architect-design`. No entry records the prefix convention.
- **d. Its kind.** No defect. The prefix is a convention by practice, which no instruction states
  and no act needs: the commit's message is judged on its references, and its subject's prefix
  carries no rule. The checker refused the one form that names an entry, at the commit, and named
  the repair; the session amended once, to the location's name. Where the text is silent the
  session judges, per `primer@room-to-judge`, and the checker caught the one choice it refuses.
- **f. A better fix.** The first proposal adds a conformance rule for a move the session made right
  in every subject but this one: one instance in the 400 sampled, against "a conformance rule ... is
  worth text only where the default behaviour is systematically wrong", per
  `skill@knowledge-architect-retrospective-intake@admission-rules`. The second weakens a rule of the
  owner for a cost of one amend. No change.

Default: no change.
Outcome: decided by the session: no change, no defect. Ground: no instruction names a prefix and no
act needs one; the plain-word finding refused the one prefix that names an entry and named its
repair, at a cost of one amend; an exemption would contradict `design@check@plain-word-lint`, the
owner's rule, on one instance.
