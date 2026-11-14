# 2026-11-12-knowledge-architect-new-crate-placeholder

- **Received file**: 2026-11-12-knowledge-architect-new-crate-placeholder.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from source: `v0.9.0-379-gf9137c97`. The project's main commit at the session's start: `f9137c97`. The session worked on main, then on the branch `release-first-versions-by-placeholder`; no uncommitted changes at its start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@knowledge-table-home`: constrains, respect.
  - `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect.
  - `rejected@agent-skills@both-directions-written-by-hand`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@agent-skills@head-prevails-over-diverging-code`: constrains, respect.
  - `design@agent-skills@divergence-routed-by-its-authority`: constrains, respect.
  - `design@knowledge-architect@publish-credential`: constrains, respect.
  - `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`: touches, respect.
  - `design@knowledge-architect@new-crate-placeholder`: constrains, respect.
  - `design@knowledge-architect@placeholder-at-the-merge-of-its-crate`: constrains, respect.

## grounding-files-list-citers: the grounding files open with a hand-written list of the entries that cite them

Members: 2026-11-12-knowledge-architect-new-crate-placeholder P3.

- **a. What it says.** `path@agent-skills@docs/grounding/crate-boundaries.md` and `path@agent-skills@docs/grounding/goal-standard.md` each open by naming the entries that lean on them. The session copied the shape into a new grounding file, whose list was incomplete on its first commit, and the routing axis reported it as a hand-written list of what references an entry, which `cargo klarch show` computes. The root `CLAUDE.md` row for `docs/grounding/<subject>.md` says nothing of a file's opening, so the precedent was the only guide. Its proposal: replace each opening by a sentence of its subject, in a commit of its own, per row 2 of `primer@met-outside-the-task`; 0 instructions. Its rival: a sentence in the root row, which adds an instruction where the rule on hand-written lists already decides.
- **b. Whether it still applies.** On origin/main, both openings stand. `crate-boundaries.md` lines 3 to 5: "Findings established outside this project that `design@agent-skills@component-crate-mapping`, `design@agent-skills@split-cost-by-publication` and `design@agent-skills@shared-test-component` lean on." It names three heads; `cargo klarch show path@agent-skills@docs/grounding/crate-boundaries.md` lists six sites that cite the file, among them lines 57 and 327 of `register@knowledge-architect@design` and the component-creation skill's source, line 86, so the list is incomplete. `goal-standard.md` lines 3 to 8 name a skill section, an agent and four heads, and line 19 leans on that list: "argued in the heads above". The file the session wrote, `path@knowledge-architect@docs/grounding/crates-io-publishing.md`, opens with its subject, as repaired. **The shape spread once more after the report**: `path@agent-skills@docs/grounding/module-split.md`, added by bfec4e1c (`v0.10.0-29-gbfec4e1c`, "[harvest] Splits planned before the work: ..."), opens with "which `design@agent-skills@module-split-guidelines` and `rejected@agent-skills@module-split-at-a-size-threshold` lean on", and `cargo klarch show` on that file lists one citing site, line 909 of `register@agent-skills@rejected`. It applies, at three files.
- **c. Whether it is already recorded.** No issue holds it. The search: `design@agent-skills@a-reference-claims-a-revisit` (`constrains`, respect), `path@agent-skills@docs/design/documents-point.md` line 37: "**An entry never lists what references it.** The inbound list is `show`'s to compute, and a hand-written one is stale at the next reference written elsewhere."; `rejected@agent-skills@both-directions-written-by-hand` (`constrains`, respect): "since a list kept by hand is stale at the next reference written", which limits the repair to removing the lists; `design@agent-skills@knowledge-table-home` (`constrains`, respect), against a project row that only repeats a rule; `goal@records-reach-their-reader` (`constrains`, respect): `show` gives a session changing an item its citers.
- **d. Its kind.** A defect of the record, wrong precedent: the files break the rule of `design@agent-skills@a-reference-claims-a-revisit`, and each copy is wrong on its first commit; a third copy shows the precedent recurring.
- **e. Its scope.** `goal@records-reach-their-reader`. The owners: the agent-skills Component's grounding files. It reverses nothing: removing a reference the test of `design@agent-skills@a-reference-claims-a-revisit` does not owe changes no decision, since a grounding file stays while its source holds, per the root `CLAUDE.md` row, whatever heads lean on it.
- **f. A better fix.** The rival is the sentence in the root row. The fact that decides: the rule exists, the routing axis applied it, and the cause of every copy is the precedent itself, so removing the precedent from all three files removes the cause; the row would add an instruction no act needs once no file shows the shape. The report's fix reached two files; the third, added since, takes the same edit.
- **g. Its route.** Row 2 of `primer@met-outside-the-task`: a fix checkable from the diff alone, changing no behaviour, decision or test outcome, in a commit of its own.
- **h. The proposal.** Three edits, in one commit. Instructions: 0 added, 0 removed (records, not agent-facing text). Nearest rival: the root row's sentence, defeated in item f.
  - `crate-boundaries.md`: lines 3 to 5 become the first block.
  - `goal-standard.md`: lines 3 to 8, up to "Each finding names its source and how it was read:", become the second block; in line 18 to 19, "argued in the heads above" becomes "argued in the design heads on the goal standard".
  - `module-split.md`: in lines 3 to 5, the clause "which `design@agent-skills@module-split-guidelines` and `rejected@agent-skills@module-split-at-a-size-threshold` lean on" becomes "on its threshold of lines for a module split and its cut of modules by scope of review"; the rest of the sentence and the next are kept.

```text
Findings established outside this project on what Cargo allows across a crate's boundary, on
publishing many small crates from one repository, and on one project's shared testing crate. Each
is read again at the source it names.
```

```text
Findings established outside this project on how other fields state and review high-level goals,
which the design of the goal standard leans on. Each finding names its source and how it was read:
```

Default: handle now, the three edits in a commit of their own.
Outcome: decided by the session: handle now. Ground: the openings break "An entry never lists what references it" of `design@agent-skills@a-reference-claims-a-revisit`, are incomplete as `cargo klarch show` measures, and were copied into a third file after the report; row 2 of `primer@met-outside-the-task` takes the fix.

## publish-credential-head-diverged: the head `design@knowledge-architect@publish-credential` and the release skill diverged

Members: 2026-11-12-knowledge-architect-new-crate-placeholder P2.

- **a. What it says.** `design@knowledge-architect@publish-credential` said "the release skill stops a release that would need it", while the skill, since an earlier review's repair, carried such a release on to its merge; the earlier commit had put the question to the owner, and the record held no ruling. The report says it was repaired by the branch's decision and the owner's ruling. No proposal.
- **b. Whether it still applies.** `git log -S "stops a release that would" -- docs/design.md` gives 2ada12fd, which wrote the sentence, and 75e41f61, "[release] A new crate's name is taken by a placeholder version during its release, so the release workflow publishes every real version", which removed it (`git show 75e41f61 -- docs/design.md` deletes the line, and its message records the reversal: "Reversal. `design@knowledge-architect@publish-credential` said "the release skill stops a release ..."). 75e41f61 is an ancestor of origin/main, `v0.9.0-380-g75e41f61`. The head on origin/main, `register@knowledge-architect@design` line 799, holds no sentence on stopping a release, as the search's verdict on it says (agent-15). The divergence closed by a reversal recorded in its commit, as `design@agent-skills@head-prevails-over-diverging-code` and `design@agent-skills@divergence-routed-by-its-authority` (`constrains`, respect) require.

Default: no change.
Outcome: decided by the session: no change, already repaired by 75e41f61 on main. Ground: the head on origin/main no longer states the diverging sentence, and the commit that removed it records the reversal, item b.

## release-404-read-as-new-crate: release step 6 read a version's 404 as a new crate

Members: 2026-11-12-knowledge-architect-new-crate-placeholder P1.

- **a. What it says.** `skill@klarch-release` step 6, at the session's start: "For each crate, the API answers 404 for a first release, or shows the crate owned by the owner's account", and "A 404 means a crate's first version". The address checked is a version's, which answers 404 for any absent version, so every crate of a valid release read as new. The report says it was repaired in the branch: step 6 reads the crate's own address too. No proposal.
- **b. Whether it still applies.** At the version, `f9137c97`, `git show f9137c97:.claude/skills/klarch-release/SKILL.md` holds both sentences at lines 81 and 82. On origin/main, `path@agent-config@skills/klarch-release/SKILL.md` step 6 reads two addresses, and line 90 says: "The first answers 404 whenever the version is absent; the second answers 404 only for a new crate". `git log -S "the second answers 404 only for a new" -- .claude/skills/klarch-release/SKILL.md` gives 858e45fa, "[review] Repairs of the review before the merge: a new crate told apart from an absent version, ...", an ancestor of origin/main (`git merge-base --is-ancestor` exits 0), `v0.9.0-381-g858e45fa`. The repair the report credits is on main. The search's verdicts (`design@agent-skills@project-file-goes-to-an-intake`, `design@knowledge-architect@new-crate-placeholder`, `design@knowledge-architect@placeholder-at-the-merge-of-its-crate`, all `constrains`, respect) agree that step 6 must tell a new crate from an absent version, which the repaired text does.

Default: no change.
Outcome: decided by the session: no change, already repaired by 858e45fa on main. Ground: the repaired step 6 is on origin/main, item b.
