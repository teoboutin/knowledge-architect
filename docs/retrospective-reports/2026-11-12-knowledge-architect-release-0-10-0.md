# 2026-11-12-knowledge-architect-release-0-10-0

- **Received file**: 2026-11-12-knowledge-architect-release-0-10-0.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `bc2c7036` (`v0.9.0-451-gbc2c7036`) when the work began; branch `release-0.10.0`; clean tree."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `rejected@agent-skills@dispatch-size-hint`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`: touches, respect.
  - `design@knowledge-architect@changelog-entries`: constrains, respect.
  - `design@knowledge-architect@changelog-reviewed-at-the-release`: constrains, respect.
  - `design@knowledge-architect@no-future-breaking-cost-argument`: constrains, respect.
  - `issue@knowledge-architect@a-mechanical-changelog-check`: touches, respect.
  - `design@agent-skills@questions-go-to-their-authority`: constrains, respect.
  - `design@knowledge-architect@publish-after-merge`: constrains, respect.
  - `design@knowledge-architect@owner-word-mechanism`: constrains, respect.
  - `design@knowledge-architect@publish-credential`: constrains, respect.
  - `design@knowledge-architect@new-crate-placeholder`: constrains, respect.
  - `goal@the-owner-decides`: constrains, respect.
  - `goal@the-owner-s-word-is-verified`: constrains, respect.

## changelog-axis-range-too-long: the release's changelog axis is one agent over a range too long for one

Members: 2026-11-12-knowledge-architect-release-0-10-0 P2.

- **a. What it says.** The changelog axis of the 0.10.0 release ran over `v0.9.0..<head>`, 452 commits, in one agent. It returned two gaps, each stated in its report: 48 commits that edit shipped skill text and write no changelog line not read one by one, and the future-breaking argument searched by pattern, not read. Two more agents closed them: none of the 48 owed an entry, and the arguments were read over a filtered set of 585 sentences. The first pass also found 8 real findings, all repaired. It blames `agent@klarch-changelog-reviewer`, "Read every design head, `rejected` entry and commit message the range adds or rewrites". Cause, per the report: a structure the work did not need, one agent per axis fixed over the whole range. Proposal: step 9 of `skill@klarch-release` dispatches the axis as several agents, by question or by sub-range, at the session's judgement; one instruction reworded, none added. Rival: move the future-breaking reading to each branch's review, a change to the review axes and a decision for the design skill.
- **b. Whether it still applies.** At the version, `bc2c7036`, step 9 at line 108 and the agent's sentence at line 58 stand; `git log --oneline bc2c7036..origin/main -- .claude/skills/klarch-release/SKILL.md .claude/agents/klarch-changelog-reviewer.md` prints nothing. On origin/main, `path@agent-config@skills/klarch-release/SKILL.md` line 108: "`agent@klarch-changelog-reviewer`, dispatched on `v<previous>..<commit under review>`, the head's commit at the dispatch", one dispatch; the agent's definition opens with "`skill@klarch-release` dispatches you on a release branch, with the range `v<previous>..<commit under review>`" and its scope is "every change in the range". It applies.
- **c. Whether it is already recorded.** No issue holds it. The search: `rejected@agent-skills@dispatch-size-hint` (`constrains`, respect): "context windows differ by model, so one bound fits none of them alike", so a size or token bound per agent is out, and a split at the session's judgement is not; `design@knowledge-architect@changelog-entries` (`constrains`, respect), `register@knowledge-architect@design`, head the section "changelog-entries": "An entry describes the release's net effect: a change reversed before the release has none", which a split must keep whole; `design@knowledge-architect@changelog-reviewed-at-the-release` (`constrains`): the review stays at the release, so the report's rival is the shape this head rejects for entries, a design question; `design@knowledge-architect@no-future-breaking-cost-argument` (`constrains`): a split by question keeps one agent on that question; `issue@knowledge-architect@a-mechanical-changelog-check` (`touches`): a mechanical check would narrow the reviewer's predicates, the owner's design discussion; `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing` (`touches`): this edits the local skill only.
- **d. Its kind.** A defect of the text: a structure the work did not need. Step 9 fixes one dispatch for the axis whatever the range's size, against `primer@dispatching-work`, under which the session judges how the work is divided unless a step's property fixes it; the 452-commit range left two gaps and two more dispatches.
- **e. Its scope.** `goal@agents-get-a-complete-workflow`. The owners: this repository's own skill, `skill@klarch-release`, and its own agent, `agent@klarch-changelog-reviewer`. It reverses no head: no head fixes the axis's count of agents.
- **f. A better fix.** The report's split "by question or by sub-range" loses, by sub-range, the predicate on net effect: an agent given part of the range cannot see a change reversed in another part, per `design@knowledge-architect@changelog-entries`. The nearest rival is a split by question alone. The fact that decides: the walk "Is an entry missing?" over 452 commits may itself be more than one agent reads, and a sub-range serves it with nothing lost, so the session may split either way, with net effect judged by an agent given the whole range. The agent must also accept a narrowed brief, which its definition does not say today; a step-9 edit alone leaves each split agent judging every predicate.
- **g. Its route.** A text edit of this repository's own skill and agent, under `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** In step 9, after the sentence that ends "per `design@knowledge-architect@changelog-reviewed-at-the-release`.", add the first block; the sentence itself is kept, so its colon still introduces what the axis judges. In `path@agent-config@agents/klarch-changelog-reviewer.md`, after its first sentence, add the second block. Instructions: 2 added (net effect judged over the whole range; the agent judges only what its brief names), 0 removed; the dispatch of one agent, fixed by step 9's sentence, is left to the session's judgement by the first block, which counts as no instruction since it lifts a structure; net +2. Nearest rival: a split by question alone, defeated in item f.

```text
The axis is one agent or several, each briefed with some of its predicates or a sub-range of that
range, at your judgement; an entry's net effect, per `design@knowledge-architect@changelog-entries`,
is judged by an agent given the whole range.
```

```text
Where the brief names some of the predicates below, or a sub-range of that range, judge those
alone, and say in your report which you judged.
```

Default: handle now, the two edits above.
Outcome: decided by the session: handle now. Ground: step 9 fixes a dispatch the work does not need fixed, per `primer@dispatching-work` and `skill@knowledge-architect-retrospective-intake@admission-rules`; a split at judgement keeps clear of `rejected@agent-skills@dispatch-size-hint`; the clause on net effect keeps `design@knowledge-architect@changelog-entries` whole; the review stays at the release, per `design@knowledge-architect@changelog-reviewed-at-the-release`.

## release-tag-push-is-owner-act: release step 11 gives the session the tag push and a precondition only the owner can check

Members: 2026-11-12-knowledge-architect-release-0-10-0 P1.

- **a. What it says.** `skill@klarch-release` step 11: "**Tag**, with a clean tree on main, once every new crate of step 6 has its placeholder and its trusted publisher. The tag push starts `path@knowledge-architect@.github/workflows/release.yml`", then four commands, `git push origin v<version>` among them. The session ran them; the harness's auto-mode classifier refused the push as creating a public surface, and the owner pushed. Nothing was lost: the refusal came before `git tag`. For the precondition, nothing gives the session a check of a trusted publisher, which crates.io shows only to a logged-in owner; the session tagged on the owner's statement and said so. Cause, per the report: a structure the work did not need. Proposal: the session prepares the tag command and checks the tree; the owner pushes after confirming each new crate's trusted publisher; "a rewording ... and adds no instruction". Rival: the session pushes under a permission rule in the project's settings, which still leaves the precondition unverifiable.
- **b. Whether it still applies.** At the version, `bc2c7036`, `git show bc2c7036:.claude/skills/klarch-release/SKILL.md` holds step 11 at line 120. `git log --oneline bc2c7036..origin/main -- .claude/skills/klarch-release/SKILL.md` prints nothing, and on origin/main `path@agent-config@skills/klarch-release/SKILL.md` lines 120 to 129 hold the step unchanged, `git push origin v<version>` its last command. It applies. The precondition, measured on 2026-11-14: `https://crates.io/api/v1/crates/knowledge-architect` answers without login with `trustpub_only: True`, and `https://crates.io/api/v1/trusted_publishing/github_configs?crate=knowledge-architect` answers 403 without login. So the session can read whether a crate requires trusted publishing, and cannot read its publisher's configuration. The report's claim that the push is refused is its evidence alone; I did not reproduce it.
- **c. Whether it is already recorded.** No issue holds it. The search: `design@knowledge-architect@publish-credential` (`constrains`, respect, agent-15): the report's premise omits "the crate's `trustpub_only` field in `https://crates.io/api/v1/crates/<crate>` re-reads it", the requirement, though not the publisher, which the measurement in item b confirms. `design@knowledge-architect@publish-after-merge` (`constrains`): "Before the tag is pushed, main's tree is compared with the reviewed head's", which stays before the push whoever pushes; the head names no actor for the push ("Then the version's tag is pushed on main's head"). `design@knowledge-architect@owner-word-mechanism` (`constrains`): the owner's word to publish is the approval of the `release` environment's deployment, so the push is not the word. `design@knowledge-architect@new-crate-placeholder` (`constrains`): "The owner then gives the crate its trusted publisher and turns on" the requirement, so the precondition is the owner's own act. `design@agent-skills@questions-go-to-their-authority` (`constrains`): "Which further actions a project reserves to its owner is the project's to state." `goal@the-owner-s-word-is-verified` (`constrains`): the session checks what a word assumes against its measurements. `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing` (`touches`, respect): an outcome that ships a release procedure goes through a design discussion; this one edits the local skill only.
- **d. Its kind.** A defect of the text, two parts. Missing, for the precondition: a provable gap, since the step asks the session to establish a fact crates.io serves only to the crate's owners (403, item b), and judgement cannot derive it. Wrong, for the push, on the report's evidence: the step tells the session an act the harness refused it.
- **e. Its scope.** `goal@the-owner-decides` and `goal@the-owner-s-word-is-verified`. The owner of the fix: this repository's own skill, `skill@klarch-release`. It reverses no head: no head names who pushes the tag, and `design@knowledge-architect@publish-after-merge`'s tree comparison stays before the push.
- **f. A better fix.** The report's rival, a permission rule for the push, changes the harness's permissions, which no agent message authorises and which the owner sets; it leaves the precondition as it is. The fact that decides between it and the proposal: the owner is needed at step 11 for the publisher whenever a crate is new, and the push is refused to the session in auto mode, so the owner's push costs nothing the rival saves. The report's proposal misses one part the record gives: the session can read `trustpub_only`, which checks the owner's confirmation against a measurement, as `goal@the-owner-s-word-is-verified` asks. The report counts its proposal as adding no instruction; it changes who carries two acts and adds a confirmation to ask and a field to read.
- **g. Its route.** A text edit of this repository's own skill, under `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** In `path@agent-config@skills/klarch-release/SKILL.md`, step 11, replace the sentence "The tag push starts `path@knowledge-architect@.github/workflows/release.yml`:" and the code block after it by the block below, the step's first sentence kept. Instructions: 2 added (ask the owner to confirm each new crate's publisher; read its `trustpub_only`), 1 removed (the session's push), 1 reassigned to the owner (the push); net +1. Nearest rival: the permission rule, defeated in item f.

````text
    crates.io shows a crate's trusted publisher only to its owners, so ask the owner to confirm it
    for each new crate, and check that the crate's `trustpub_only` field, at step 6's second
    address, is `true`. Run the first two commands below, then give the owner the last two: the
    tag push, which starts `path@knowledge-architect@.github/workflows/release.yml`, is the
    owner's, since the harness's auto mode refuses it to the session.

    ```sh
    git checkout main && git pull --ff-only
    test "$(git rev-parse HEAD^{tree})" = "$(git rev-parse <the head recorded at step 10>^{tree})"
    git tag -a v<version> -m "Release <version>"
    git push origin v<version>
    ```
````

Default: handle now, the step-11 edit above.
Outcome: decided by the session: handle now. Ground: the publisher's configuration is served to the crate's owners alone (403 measured), a provable gap; `design@knowledge-architect@publish-credential` gives the session `trustpub_only` as the check of the owner's confirmation, per `goal@the-owner-s-word-is-verified`; no head names who pushes, and `design@knowledge-architect@owner-word-mechanism` makes the deployment approval, not the push, the owner's word, so moving the push changes no decision; the rival changes permissions only the owner sets.

## release-warning-count-by-file: release step 8 counts one warning per test file, and two files share a name

Members: 2026-11-12-knowledge-architect-release-0-10-0 P3.

- **a. What it says.** `skill@klarch-release` step 8: "Its warning "ignoring test `<name>`" for each file `<name>.rs` that `git ls-files ':(glob)crates/*/tests/*.rs'` lists is expected". The glob listed 14 files and the dry run printed 13 distinct names, since `public_paths.rs` is in both crates/cli/tests and crates/testing/tests; the session resolved it by diffing the two lists. Proposal: "for each name `<name>` among the files ... lists"; a rewording, no instruction added.
- **b. Whether it still applies.** At the version, `bc2c7036`, step 8 holds the sentence at line 103; on origin/main, `path@agent-config@skills/klarch-release/SKILL.md` line 103 holds it unchanged. Measured on origin/main: `git ls-files ':(glob)crates/*/tests/*.rs' | wc -l` prints 14, and `... | xargs -n1 basename | sort | uniq -d` prints `public_paths.rs`. It applies. Whether cargo prints one warning per file or per name I did not measure (it needs the full dry run); the reworded sentence holds either way, since it compares names.
- **c. Whether it is already recorded.** No issue holds it. The search's only verdict other than the seeds is `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing` (`touches`, respect), which this local rewording leaves as it is.
- **d. Its kind.** A defect of the text, unclear: "for each file" sets a count the dry run's names do not meet when two crates share a test file's name.
- **e. Its scope.** `goal@agents-get-a-complete-workflow`. The owner: this repository's own skill. It reverses nothing.
- **f. A better fix.** The nearest rival is a per-crate list, the glob run per crate. The fact that decides: the warning names the test, not its crate, so a comparison by name is what the session can make from the output, and it needs no measurement of how cargo prints a shared name.
- **g. Its route.** A text edit of this repository's own skill, under `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** In step 8, replace "Its warning "ignoring test `<name>`" for each file `<name>.rs` that `git ls-files ':(glob)crates/*/tests/*.rs'` lists is expected" by the block below. Instructions: 0 added, 0 removed, 1 reworded. Nearest rival: the per-crate list, defeated in item f.

```text
Its warning "ignoring test `<name>`" for each name `<name>` among the files `<name>.rs` that
`git ls-files ':(glob)crates/*/tests/*.rs'` lists is expected
```

Default: handle now, the rewording above.
Outcome: decided by the session: handle now. Ground: the glob lists 14 files of 13 names on origin/main, so "for each file" cannot be matched against the warnings' names; the rewording adds no instruction.
