# 2026-11-12-knowledge-architect-dispatch-judgement

- **Received file**: 2026-11-12-knowledge-architect-dispatch-judgement.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.9.0-371-g7173deed`." / "The project's main commit when the work began: `7173deed`." / branch `dispatch-judgement`, clean.
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@pinned-binary-runs-inside-the-project`: constrains, respect.
  - `design@agent-skills@installed-text-works-with-what-an-installing-project-holds`: constrains, respect.
  - `design@agent-skills@intake-takes-the-project-file`: constrains, respect.
  - `design@agent-skills@plan-reviewed-as-a-commit`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `design@check@a-foreign-build-is-refused`: constrains, respect.
  - `design@check@checked-sentinel-values`: constrains, respect.
  - `rejected@check@run-time-checker-directory-discovery`: constrains, respect.
  - `rejected@check@sentinels-accepted-everywhere`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`: touches, respect.
  - `design@knowledge-architect@a-build-is-tied-to-its-checkout`: constrains, respect.
  - `goal@serves-one-kind-of-project`: constrains, respect.
  - `goal@every-target-project-can-adopt-it`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.

## worktree-reviewer-cannot-run-checker: reviewers in worktrees met the refusal of a foreign build

Members: 2026-11-12-knowledge-architect-dispatch-judgement P1; the interaction of
2026-11-12-knowledge-architect-dispatch-judgement-klarch-workflow, whose workflow side is none.

- **a. What it says.** "The release binary built in the main checkout refuses a worktree: 'this
  binary was built from another checkout'. This follows from the checker-version sentinel,
  `"self"`." Of 15 reviewers, "some built the checker in their worktree, with `CARGO_TARGET_DIR` set
  to its `target/`, a cold release build each"; "two ... ran no checker and said so"; "one found the
  form `cargo run -q --release -p knowledge-architect -- check` by itself"; "The session's briefs had
  to tell each reviewer to build there, after the first one reported the refusal." Proposal: one
  sentence in `skill@klarch-development` or in the root's "Verify mechanically": "a reviewer in a
  worktree runs the checker as `cargo run --release -p knowledge-architect -- <command>`, with
  `CARGO_TARGET_DIR` inside the worktree, at the cost of a cold build" (+1). The workflow file:
  "The installed text is right as written, so there is no workflow finding."
- **b. Whether it still applies.** The diagnosis does not. The message the reviewers met is the
  refusal of `design@check@a-foreign-build-is-refused`, at `path@check@src/build_origin.rs` line 93
  both at `v0.9.0-371-g7173deed` and at `v0.10.0-59-g605d1538`, which names its cause, "a target
  directory shared between two checkouts does this", not the sentinel of
  `design@check@checked-sentinel-values`. The move the proposal names already exists, at the file's
  version and on main:
  - `skill@knowledge-architect-review@review-invariants`: "**The subagent's build output stays inside
    its worktree.** ... In a Rust project, the subagent sets `CARGO_TARGET_DIR` to a directory inside
    its worktree that the project's ignore rules cover, such as its `target/`" (present at
    7173deed in `path@agent-skills@content/skills/review/SKILL.md`);
  - `skill@klarch-development@other-checkouts`: "a reviewer's worktree builds into a target directory
    of its own, `export CARGO_TARGET_DIR="$PWD/target"` inside it" (the same text at 7173deed);
  - the alias `cargo klarch`, in `path@knowledge-architect@.cargo/config.toml`, is
    `run -q --release -p knowledge-architect --`, so run inside a worktree it is the proposed
    command, tied to that worktree by `design@knowledge-architect@a-build-is-tied-to-its-checkout`.
- **c. Whether it is already recorded.** The cost is: `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`
  states "The cost per reviewer is one full build, since a worktree with a target directory of its
  own builds from empty", and that "A shared build directory is ruled out by the invariant's own
  reason and by `design@check@a-foreign-build-is-refused`"; its closing work is a command that makes
  the worktrees and "prints the paths for the briefs". The verdicts on
  `design@knowledge-architect@a-build-is-tied-to-its-checkout`, `rejected@check@run-time-checker-directory-discovery`
  and `rejected@check@sentinels-accepted-everywhere` keep the file's rival, a binary that accepts
  another checkout, out: "correct paths would still judge the tree with the wrong code".
- **d. Its kind.** A lapse, of the briefs: the instruction exists and is clear, and the session's
  first briefs did not carry it to the reviewers, who read their brief and their definition, not
  the review skill. Some definitions point the reviewer to the brief for it: the routing reviewer's
  says "Run the checker in a worktree of your own, never in the live tree, as the dispatcher's brief
  says, with any build output inside it". The recurrence is within one session, at one point of delivery, the brief, and
  the session corrected its briefs after the first report.
- **f. A better fix.** The file's sentence restates the alias and the invariant, which a reviewer
  does not read either, so it moves no delivery. The nearest rival is a rewording of the invariant
  so that the dispatcher's brief names the target directory, which moves the instruction to the
  text the reviewer reads; one session's first briefs are no recurrence across sessions, per the
  lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`, and the recorded
  issue's command, which prints the paths for the briefs, would carry it. No text change; the
  instance goes to the recorded issue.
- **h. The proposal.** Add the instance to the review-worktrees issue.

Text to add to `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`,
at the end of its list of instances:

```markdown
- the slice on the dispatch judgement, 2026-11-12: of 15 reviewers sent at the plan and at the
  merge, the first reported the refusal of `design@check@a-foreign-build-is-refused`, met with the
  release binary built in the main checkout; the session's later briefs told each reviewer to build
  in its worktree, a cold release build each; two reviewers ran no checker. The invariant's
  sentence on `CARGO_TARGET_DIR` is the subagent's act, read by the dispatcher, so a brief that
  omits it leaves the reviewer without it.
```

Default: no text change; the instance is added to the recorded issue.
Outcome: decided by the session: open an issue, as an instance added to
`issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`. Ground:
the refusal is `design@check@a-foreign-build-is-refused`'s, not the sentinel's; the move the file
proposes is already stated by `skill@knowledge-architect-review@review-invariants`,
`skill@klarch-development@other-checkouts` and the `cargo klarch` alias, so the case is a lapse of the
briefs, which the lapse rule does not repair by adding text; the cost is the recorded issue's.
