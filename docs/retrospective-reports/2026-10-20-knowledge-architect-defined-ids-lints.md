# 2026-10-20-knowledge-architect-defined-ids-lints

- **Received file**: 2026-10-20-knowledge-architect-defined-ids-lints.md, this repository's project
  file of the session that carried out the spec on defined ids outside references, its reviews and
  its merge.
- **Version used**: built from its own source, main at `v0.7.0-77-g67d36d1`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`. The root CLAUDE.md and
  `skill@klarch-development` are unchanged between the two.
- **Standing entries that bear on it**:
  - for the cluster under P1: `rejected@core@check-refusing-a-pipe`,
    `design@gates@gates-refuse-a-pipe`, `design@knowledge-architect@push-after-the-commits-check`,
    `design@agent-skills@gates-convention`, `design@core@phases-gate-the-report`,
    `design@core@exit-code-ladder`, `tripwire@core@phases-gate-the-report-two`,
    `tripwire@agent-skills@rarer-rule-in-the-model`,
    `issue@knowledge-architect@command-output-is-not-declared-a-contract`,
    `goal@xtask@repeated-tasks-are-automated`, `goal@agent-skills@installed-text-works-anywhere`;
  - for P2: `design@knowledge-architect@no-operation-loses-content`,
    `design@knowledge-architect@scratch-worktree-outside-git-rules`,
    `issue@agent-config@a-rebase-continue-strips-a-message-s-hash-lines`;
  - for the cluster under P3: `design@agent-skills@standing-argument-in-head`,
    `design@core@two-segment-ids`, `design@core@plain-word-lint`,
    `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes`,
    `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`, which
    is about a census run by agents and does not bear.

## P1, with C1 of 2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow, W6 and W11 of 2026-10-20-thaum-retrospective-intake-klarch-workflow. A push or a commit chained on a piped verdict

- **a. What they say.**
  - P1 and C1: the session ran `cargo klarch commits origin/main..HEAD 2>&1 | tail -1 && git push`
    twice, one of them a force push, and the same shape before `git commit` three times. Every check
    passed. The motive each time was to keep only the verdict line. C1 proposes an option on `check`
    and `commits` that prints only the verdict, or the findings and the verdict; P1's rival is an
    xtask alias that runs `commits` bare and then pushes.
  - W6: in thaum the same lapse appeared in five of nine sessions, three times as a push of a commit
    that failed the gate. With `SHELLOPTS=pipefail` in the `env` of the project's checked-in harness
    settings, measured there, `false | tail -1 && echo chained-ran` prints nothing and exits 1, and a
    passing check through a pipe still runs its chained command. The costs fail safe: a producer cut
    short by `head` exits 141, a `grep` that matches nothing fails its pipeline. W6 proposes that the
    setup skill recommends the setting for the `claude` harness.
  - W11: the primer's account, "the shell takes the exit status of `tail`", is false under that
    option; the instruction itself holds, since a filter still drops lines.
  - The short-deck-soak workflow file reports a push chained after `commits` with `;`, which ran over
    a failing verdict.
- **b. Whether they still apply.** Yes. `primer@checker-model`: "Never chain a command on a verdict
  that went through a pipe: in `check | tail && git commit`, the shell takes the exit status of
  `tail`, so the commit runs whatever the check found." `instructions@git-workflow`, point 1: "A push
  after `;`, or after a check whose output went through a pipe, runs whatever the check found."
  `klarch check --help` lists only `--fix` and `--staged`, and `commits` only its range. Measured on
  this machine: `env SHELLOPTS=pipefail bash -c 'false | tail -1 && echo ran'` prints nothing and
  exits 1; without the option it prints `ran`. In this harness's shell tool, the prefix form
  `SHELLOPTS=pipefail bash -c …` is refused with "SHELLOPTS: readonly variable" and runs without the
  option, so the setting works only from the environment a shell starts with. The setup skill has no
  section on harness settings, and no installed text names them.
- **c. Whether they are already recorded.** In part. `rejected@core@check-refusing-a-pipe`, `live`,
  lost because "a process inside a pipeline does not set the pipeline's status"; under `pipefail`
  that premise is false in bash, and its other costs stand. `design@gates@gates-refuse-a-pipe` stands
  on its second reason, that a filter drops lines.
  `design@knowledge-architect@push-after-the-commits-check` says "after a pipe, the shell runs it
  whatever the check found", false under the option. `design@core@phases-gate-the-report` keeps a
  summary that a verdict-only output would have to keep in its last line. No entry records the motive
  or the setting.
- **d. Their kind.** A lapse that recurs at one point of delivery, in two projects: the rule is read
  at the session's start and needed at each shell command. W11: wrong in a shell with the option.
- **e. Their scope.** `goal@knowledge-architect@setup-brings-quality-tools`,
  `goal@agent-skills@installed-text-works-anywhere`, since the option is bash's and the setting the
  `claude` harness's. Installed text, the checker, and this repository's own harness settings.
- **f. A better fix.** Rivals:
  - a verdict-only option, C1, which removes the motive for a session that uses it, and stops no
    pipe;
  - the checker refusing a pipe, which lost, and which the `;` case shows would not cover;
  - an xtask alias, which serves this repository only;
  - the harness setting, which makes a chain on a failing pipe fail for every command a session
    runs, the checker's or not, and needs no option; it does not cover `;` either, which the push
    rule's own words already name;
  - no change, the lapse rule's outcome for a lapse that does not recur, which this one does.

  The deciding fact: the setting removes the failure the rule exists for, measured in thaum's
  sessions and here, without a change to the checker's output contract. It is a recommendation for
  one harness, beside the primer's instruction, which stays for every other one. It is a decision
  about how an adopting project is set up, so it goes to `skill@knowledge-architect-design` on its
  in-change path, with this repository's own adoption of it in its harness settings. Instructions
  added: 1 in the setup skill; the primer's sentence reworded to say "in a shell without `pipefail`".
- **g. Their route.** A decision, under `skill@knowledge-architect-design`'s in-change path, then
  an installed text edit and this repository's settings file; the heads whose premise the option
  changes are read under `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** Handle now: the setup skill recommends `SHELLOPTS=pipefail` in the `env` of
  the project's checked-in settings for the `claude` harness, with its fail-safe costs; the primer's
  sentence names the shell without the option; this repository sets it in its own settings;
  `design@knowledge-architect@push-after-the-commits-check` and `rejected@core@check-refusing-a-pipe`
  are brought to the option's effect. No verdict-only option.
- **Default**: handle now, as h says.
- **Outcome**: now, recommend from setup skill, and also recommed as a migratrion entry of the changelog. Also applie it here. It is a solution to a long standing problem of the project, and it works perfectly well. I would eliver it more strongly than just a recommmendation too.

## P2. Staging before a mutation, and the ban on `checkout -- <path>` while changes are uncommitted

- **a. What it says.** Point 2 of the root CLAUDE.md bans `checkout -- <path>` while uncommitted
  changes exist, and asks that a file be staged before a mutation "so the revert does not depend on a
  restore". The session staged the file, mutated it in the live tree, and reverted with
  `git checkout -- <path>`, which restores from the index: the banned command. Nothing was lost. The
  fix proposed: the staging bullet names that revert as an exception to the ban.
- **b. Whether it still applies.** Yes. `instructions@git-workflow`, point 2: "**While uncommitted
  changes exist in the tree, run no operation that touches history or restores files.** Examples
  (non-exhaustive): `reset --hard`, `checkout -- <path>` … The one exception is
  `git rebase --continue`"; and "**Before applying a mutation that will need reverting, stage the
  file**, so the revert does not depend on a restore." `skill@klarch-development@discriminating-tests`:
  "**A mutation runs in a scratch worktree by default.** … Where the live tree is used anyway, the
  file is staged before the mutation, per `instructions@git-workflow`, so the revert does not depend
  on a restore." Neither names how the live-tree revert is made.
- **c. Whether it is already recorded.** The proposal is a rival the owner already ruled against:
  `design@knowledge-architect@scratch-worktree-outside-git-rules` records that the owner ruled that
  the Git rules do not apply in a scratch worktree "over a second exception in the rule on losing
  content that admitted only the revert of a mutation by `git checkout -- <path>`".
- **d. Its kind.** Unclear: "so the revert does not depend on a restore" says what the revert does
  not do, and not what it is; a session reads the restore from the index as the revert.
- **e. Its scope.** This repository's own root CLAUDE.md and development skill. The proposal would
  reverse the owner's ruling above.
- **f. A better fix.** Rivals: the exception, which the owner ruled against; the revert named as
  the reverse edit, with `git diff -- <path>` against the staged copy showing it restored, the index
  guarding the work if the edit goes wrong, which reads the bullet as its words say; the live-tree
  path removed from the development skill, so every mutation runs in a scratch worktree, which drops
  a case the skill keeps on purpose. The second keeps the ruling and the bullet. Instructions added:
  0, a rewording that names the act the bullet means.
- **g. Its route.** A text edit of this repository's root CLAUDE.md and development skill, under
  `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** Handle now: the staging bullet and the development skill say that the
  live-tree revert undoes the edit, and that `git diff -- <path>` showing no difference with the
  staged copy confirms it.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## P3, with W2 of 2026-10-20-knowledge-architect-defined-ids-lints-klarch-workflow. A census taken by one-off scripts, as a frozen corpus and as a measurement to re-take

- **a. What they say.** A head's argument rested on a census of real text, taken during the design
  discussion by scripts kept in a scratch directory. Three review rounds went to it: the figures had
  no command to take them again; the lints proposed as the instrument could not count one-word ids;
  the final repair named a `git grep -nwF` count and a pointer to the deleted spec that holds the
  census. W2 proposes a sentence in "What a head holds": a measurement whose instrument is not in the
  tree is restated as a command over the tree, or its script goes in the commit message that took it,
  named by its subject. P3 asks whether such a census is "a count over a frozen corpus", which keeps
  its digits, and proposes "a count over a frozen corpus, which the head names with the revision it
  was counted at".
- **b. Whether they still apply.** Yes. `primer@design-heads`, "What a head holds": "the measurement
  it rests on, with the command that takes it again". `instructions@where-knowledge-goes`: "A
  threshold, a count over a frozen corpus and a zero keep their digits, because in those the value
  _is_ the claim", the only occurrence of the phrase in the tree. The head that resulted,
  `design@core@two-segment-ids`, points at the deleted spec through the escape anchor and gives the
  re-take as `git grep -nwF` on each id.
- **c. Whether they are already recorded.** In part. `design@agent-skills@standing-argument-in-head`
  keeps the deliberation "in the spec while the spec exists, and in history after that".
  `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes` asks where a figure
  measured in another project goes, which bears on a census of thaum's text, not on its re-take.
  No entry records an instrument the tree does not hold.
- **d. Their kind.** Unclear, both: the primer asks for a command and does not say what stands for
  one when the instrument was a script kept nowhere; the root's "frozen corpus" does not say what
  freezes a corpus.
- **e. Their scope.** `goal@knowledge-architect@design-is-recorded-with-its-arguments`. W2 is
  installed text, P3 this repository's root CLAUDE.md.
- **f. A better fix.** Rivals: one fix for both, a measurement's command run over a named state of
  its corpus, which makes the corpus frozen and the count re-takable together; W2's two shapes, the
  command over the tree or the script in the commit message; commit the scripts as a tool, which no
  row of the knowledge table homes; no change, which cost three rounds here. W2's second shape keeps
  the instrument where `primer@where-knowledge-goes` routes a measurement that serves no entry, "the
  commit message that took it". Instructions added: 0, two rewordings that say how an existing
  instruction is met.
- **g. Their route.** An installed text edit of `primer@design-heads`, and a text edit of the root
  CLAUDE.md.
- **h. The proposal.** Handle now: "What a head holds" says the command that takes a measurement
  again is one a reader can run on the corpus it names, and a script kept nowhere is replaced by such
  a command, or kept in the message of the commit that took it, which the head names by its subject;
  the root's clause reads "a count over a frozen corpus, which names the corpus and the state it was
  counted in".
- **Default**: handle now, as h says.
- **Outcome**: accepted.
