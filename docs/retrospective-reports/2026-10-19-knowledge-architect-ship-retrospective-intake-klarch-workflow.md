# 2026-10-19-knowledge-architect-ship-retrospective-intake-klarch-workflow

- **Received file**: 2026-10-19-knowledge-architect-ship-retrospective-intake-klarch-workflow.md,
  the workflow file of the session that shipped the retrospective intake as an installed skill: a
  design discussion, a spec, its eight steps on one branch, three review rounds, the merge.
- **Version used**: built from source, `v0.7.0-16-g3362e5a`.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**:
  - W1 is analysed with W3 of 2026-10-19-knowledge-architect-rejected-register-klarch-workflow, in
    that file; W2 with W4 of 2026-10-18-thaum-chance-slice-merge-klarch-workflow, in that file;
  - for W3: `a-reviewer-cannot-write-the-file-its-brief-names`, now `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access`,;
  - for C1: `tripwire@core@planned-path-outside-plans`, not met, since it watches a path;
  - for C2 and P1 of 2026-10-19-knowledge-architect-ship-retrospective-intake:
    `spec@plans@defined-ids-outside-references`, `tripwire@core@candidate-rule-silence`,
    `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`;
  - for C3: `tripwire@core@checker-source-literals-are-data-self-location`, not met;
  - `tripwire@agent-skills@plain-text-pointer-found`, judged in its own section, for this file and
    2026-10-19-thaum-boundary-fuzzing-design-klarch-workflow;
  - `tripwire@agent-skills@argued-closure-reopened`, not met: it needs two threads closed `argued`
    reopened in one discussion, and the file names one withdrawn proposal reopened;
  - `tripwire@agent-skills@agent-text-written-without-the-standard`, not established: the file does
    not say whether the session that wrote the sentence delivering a checker rule had loaded the
    agent-configuration skill.

## W1

Analysed with W3 of 2026-10-19-knowledge-architect-rejected-register-klarch-workflow, in that file:
the closing rule's scope is a `todo` or `deferred` issue, per
`design@agent-skills@undesigned-work-is-an-issue`.

- **Default**: no change, as that section says.
- **Outcome**: no change, as ruled at W3 of 2026-10-19-knowledge-architect-rejected-register-klarch-workflow, where the owner ruled the findings with overlaps.

## W2

Analysed with W4 of 2026-10-18-thaum-chance-slice-merge-klarch-workflow, in that file:
observations that `issue@agent-skills@the-retrospective-counts-no-review-cost` already covers.

- **Default**: no change, as that section says.
- **Outcome**: no change, as ruled at W4 of 2026-10-18-thaum-chance-slice-merge-klarch-workflow, where the owner ruled the findings with overlaps.

## W3. A general-purpose reviewer was refused the session's transcript

- **a. What it says.** The fidelity reviewer, dispatched as a general-purpose subagent, had its read
  of the session's transcript refused by the harness's permission classifier; the named transcript
  reviewer read the same file. An observation.
- **b.** Not a claim about the text.
- **c.** `a-reviewer-cannot-write-the-file-its-brief-names`, now `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access`, records three
  observations of the same pairing, a general-purpose subagent refused and an installed agent not,
  for a write. This is the same pairing for a read.
- **d. Its kind.** An observation of the harness.
- **f. A better fix.** Rivals: add it to that issue as a fourth observation, widening its subject
  from a write to a file access; a sibling entry for reads; nothing. One entry keeps the pairing in
  one place, which is what the observations are evidence of.
- **h. The proposal.** Handle now: add the observation to that issue, its subject widened to a
  refused file access.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C1. No checked form names a skill the work will create

- **a. What it says.** A spec and several commit messages named the skill to be created before it
  existed. A planned path named its file; its name had no checked form, and was written in plain
  text, beside no issue reference. The fix proposed: a planned form for a harness kind, reported
  once the skill exists. Its rival: no form, the name written beside an issue entry.
- **b. Whether it still applies.** Yes. `design@core@planned-path-form` gives a planned form for a
  path, in the plans directory alone. The primer's bare-slug rule speaks of an entry, which a skill
  is not. A gap of the checker gets an entry in this repository's register, per
  `primer@where-knowledge-goes`, and none exists.
- **c. Whether it is already recorded.** No. `rejected@core@planned-path-in-every-document` lost a
  planned path legal in every document, on the ground that a hypothetical file is served by the
  escape anchor; no anchor serves a skill's name.
- **d. Its kind.** Missing: a form for a name the checker has no way to express; and a lapse, the
  name written bare with no issue beside it.
- **f. A better fix.** The proposal is a design of the checker with a rival that already lost for
  paths, so it is not settled here. The primer's own answer to a missing form is the issue entry.
- **g. Its route.** An issue.
- **h. The proposal.** Open an issue: a `design` issue in core, "No checked form names a skill or
  an agent the work will create", with the two rivals.
- **Default**: open a `design` issue in core.
- **Outcome**: No issue. I think the primer already says that something that will be created or has just been removed should be refenced in plain words. If it does not, it should. This is the general ruling, there is a design head stating it, I'm nearly certain.

## C2, with P1 of 2026-10-19-knowledge-architect-ship-retrospective-intake. Citations without backticks on `%%` lines are judged by nothing

- **a. What they say.** 15 citations on 15 `%%` lines of the shipped text's source reached main
  without backticks, through fourteen review passes. C2 proposes a finding for a reference-shaped
  token outside backticks on such a line. P1 proposes repairing the 15 now, and a clause in
  `path@agent-skills@CLAUDE.md`: "each as a backticked reference, which the check resolves".
- **b. Whether it still applies.** The 15 are repaired: `7120304`, "[agent-skills] The comment lines
  of the intake and the retrospective cite their heads as backticked references, which the check
  resolves", on main. The clause is not in `path@agent-skills@CLAUDE.md`, lines 31-33, unchanged.
  The check does not exist yet.
- **c. Whether it is already recorded.** Yes: `spec@plans@defined-ids-outside-references` plans the
  class A finding in every prose region the checker reads, and `%%` lines are in the walk. The
  narrower proposal, the finding limited to `%%` lines, is recorded in that spec as a design that
  lost, on its seventh argument: "a special case for one file kind adds a rule with no reason behind
  it".
- **d. Its kind.** A blind spot, planned; the instances, a lapse, repaired.
- **f. A better fix.** One fix for the cluster: the spec's class A, which delivers the rule at every
  edit. The clause would restate the primer's rule that a reference is a backticked span, in one
  file, until the check lands.
- **h. The proposal.** No change.
- **Default**: no change: the instances are repaired by `7120304`, and the check is scheduled by
  `spec@plans@defined-ids-outside-references`.
- **Outcome**: accepted.

## C3. A reviewer cannot take baseline counts at the base of a range

- **a. What it says.** The binary built from this checkout refuses a worktree at another commit,
  "this binary was built from another checkout". One reviewer extracted the base with `git archive`,
  another reported the counts as not taken. The fix proposed: accept a worktree of the same
  repository, or a command that counts at a given commit.
- **b. Whether it still applies.** The refusal is `design@core@a-foreign-build-is-refused`. The
  review skill gives each subagent its own worktree, with its own target directory inside it,
  `path@agent-skills@content/skills/review/SKILL.md` lines 99-117, and every build is tied to its
  checkout by the `[env]` table of `path@knowledge-architect@.cargo/config.toml`. So `cargo klarch`
  run inside a worktree at the base builds and runs that commit's checker, which the refusal does
  not stop. Assumption: not run in this intake.
- **c. Whether it is already recorded.** The first proposal is defeated by
  `design@core@a-foreign-build-is-refused` and `rejected@core@run-time-checker-directory-discovery`:
  "correct paths would still judge the tree with the wrong code".
- **d. Its kind.** A lapse of the reviewer, which ran the live checkout's binary over another
  checkout, if the assumption holds.
- **h. The proposal.** No change.
- **Default**: no change: a worktree at the base runs its own checker, as the review skill sets it
  up.
- **Outcome**: accepted.

## The standing answers

- **a.** No installed file edited by hand. Three primer rules acted on as if absent: a default
  written into a head before the owner ruled; the owner's words quoted; 15 plain-text citations
  (C2). One reference written bare for lack of a form (C1).
- **b.** The rules quoted hold at both commits.
- **d.** Lapses, caught before the merge, except the 15 citations, repaired after it.
- **Default**: no change, lapses.
- **Outcome**: accepted.

## `tripwire@agent-skills@plain-text-pointer-found`: whether it fired

- **The firing clause**, its second part: "a retrospective's answer to its question on references
  written without backticks reports a pointer written as bare plain text to clear a finding". Its
  response: "open a `defect` for the instance. At the second instance, reopen
  `design@agent-skills@plain-text-is-no-repair`, with a mechanical check for plain-text pointers
  among the candidates". It holds one instance.
- **The evidence**:
  - 2026-10-19-thaum-boundary-fuzzing-design-klarch-workflow, its standing answers and C1: a
    planned directory in a commit message, reported "names no anchor" when backticked, written bare
    with no issue beside it. A checked form existed: the escape anchor serves a path the tree does
    not hold, as `primer@where-knowledge-goes` says, and the finding's repair names it, though as
    "for a path outside this tree";
  - this file's C1: the name of a skill still to be created, written bare, beside no issue. A
    reference to a skill that does not exist is a finding, so the bare name cleared one.
- **The judgement proposed**: fired, at its second and third instances. Both instances are in commit
  messages, which main's history keeps as they are, so the defect the response opens records them
  rather than repairs them. The reopening's candidate, a mechanical check, is in part
  `spec@plans@defined-ids-outside-references`'s class A; neither instance is reference-shaped, so
  class A would not have caught them.
- **Default**: fired; one `defect` issue in agent-skills names both instances, and the reopening of
  `design@agent-skills@plain-text-is-no-repair` is recorded in a `design` issue in agent-skills,
  naming class A as a partial candidate and the escape anchor's repair wording as a cause of the
  first instance.
- **Outcome**: accepted.
