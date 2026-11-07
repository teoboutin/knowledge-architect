# 2026-11-01-knowledge-architect-development-milestone-plan

- **Received file**: 2026-11-01-knowledge-architect-development-milestone-plan.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-11-g3d61549a`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`, `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`.

## P1, with P1 of 2026-11-02-knowledge-architect-development-and-components, P2 of 2026-11-06-knowledge-architect-cli-on-top and P5 of 2026-11-06-knowledge-architect-consumers-out: a commit or a push chained after a failing check (cluster H2)

- **a.** What it says. P1 here: one command ran the commits check, then `echo`, `tail` and
  `git push`, joined with semicolons; the check failed, and the push went out. P1 of
  development-and-components: two commits followed a failed staged check, once chained with `&&`
  on `tail` of its output, once after a semicolon. P2 of cli-on-top: twice, a semicolon between
  the staged check, or a failing edit script, and `git commit`; it proposes that this repository's
  setup install a pre-commit hook running `cargo klarch check --staged`, and calls the proposal
  weak. P5 of consumers-out: after a `git rebase --onto` that detached HEAD, the session ran
  `commits` on the detached commits and pushed the branch's name, which still held the commit that
  had failed. Every file calls its instance a lapse; only P2 of cli-on-top proposes a change.
- **b.** Whether it still applies. The instructions stand, unchanged in substance, at every file's
  commit and at HEAD:
  - the push: `path@knowledge-architect@CLAUDE.md`, line 199 at HEAD (line 192 at `3d61549a`):
    "A commit is pushed only after `cargo klarch commits` has passed on it: in a command of its own,
    or behind `&&` on the bare check. A push after `;`, or after a check whose output went through a
    pipe in a shell without `pipefail`, runs whatever the check found";
  - the commit: the installed primer, `path@agent-config@knowledge-architect/PRIMER.md`, lines 322
    and 323: "Run it after staging and before each commit, and repair a failing one before the
    commit"; and line 336: "Never chain a command on a verdict that went through a pipe". The
    primer states no rule on a semicolon after a bare check; the rule that covers that chain is
    "repair a failing one before the commit". `design@agent-skills@harness-shell-fails-on-a-pipe`
    says the primer's rule against the chain "stays, for `;`".
  - The hook: no hook is installed in this checkout (a listing of the checkout's hooks directory, samples excluded, shows
    nothing; `git config --get core.hooksPath` exits 1), and the installed setup skill names none
    (a search of "hook" in `path@agent-skills@content/skills/setup/SKILL.md` finds nothing).
- **c.** Whether it is already recorded. The recurrence is recorded nowhere. Related entries:
  `design@agent-skills@harness-shell-fails-on-a-pipe` delivers `pipefail` at every command, which
  stops the piped chain and not the semicolon; `design@agent-skills@staged-check-before-each-commit`
  is the primer's staged-check instruction; `design@knowledge-architect@push-after-the-commits-check`
  is the push rule; `goal@check@checks-a-commit-from-its-hook` makes judging a commit's tree from the
  pre-commit hook a capability of the checker; `rejected@check@commit-msg-hook` rejected a hook that
  judged messages, its third count being "per-clone configuration every clone had to install and
  every session had to check"; and
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
  records the same mechanism for another instruction, "an instruction read once at the start of a
  long session is not recalled at the moment it binds", names "a project's commit hook" among the
  deliveries its discussion weighs, and has as its re-entry point "raised when work next changes what
  a session reads or runs at a commit".
- **d.** Its kind: a lapse of clear instructions, recurring. The count over this intake's files:
  seven instances in five sessions of this repository, from 2026-11-01 to 2026-11-06 (one push, six
  commits: P1 here; two in P1 of development-and-components; one in the standing answers of
  2026-11-04-knowledge-architect-check-renamed-klarch-workflow; two in P2 of cli-on-top; one in the
  standing answers of 2026-11-06-knowledge-architect-consumers-out-klarch-workflow), and one in a
  session of thaum, in the standing answers of 2026-11-01-thaum-design-record-audit-klarch-workflow.
  P5 of consumers-out is a further lapse of the push rule, one instance, of another shape: the check
  passed on commits the push did not send.
- **e.** Its scope: `goal@knowledge-architect@agents-work-without-drift`. The owner of the fix: the
  commit rule is the installed primer's, so its delivery is the installed text's or the setup's,
  `agent-skills`; the push rule is this repository's root's. No fix is chosen, so none reverses a
  decision yet. A pre-commit hook does not bring back `rejected@check@commit-msg-hook`, which judged
  messages, but it meets that entry's per-clone count, and in this repository it would refuse a
  commit that `commits` accepts, per
  `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, which recalls the
  entry's second count, a hook that needed `--no-verify`.
- **f.** A better fix. A lapse is not repaired by adding text; where it recurs at one point of
  delivery, the delivery moves, per `skill@knowledge-architect-agent-configuration@admission-standard`.
  The rivals: (1) the pre-commit hook of P2 of cli-on-top, which stops the commit whatever the shell
  chained, at the cost of a per-clone install and, in this repository, of the staged-check defect;
  (2) the root's sentence moved to the commit's moment in `## Git`, which P1 of
  development-and-components names, and which moves text the session reads at the start as now; (3)
  no change, since each instance was caught by the next command and cost one amend. Rival 1 or a
  delivery the claim-check issue weighs would serve both instructions at once, which decides that
  the choice belongs in one design discussion with that issue, not here.
- **g.** Its route: an issue only; the choice of delivery is a design discussion under
  `skill@knowledge-architect-design`.
- **h.** The proposal: open an issue in `agent-skills`, kind `design`, proposed id
  `a-command-chained-after-a-failing-check-recurs`: the seven instances, the mechanism (the rule is
  read at the start and needed at each commit, inside a compound command), the candidates (the hook,
  with the per-clone count and the staged-check defect against it; the root's sentence moved; none),
  and as re-entry point the discussion of
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
  which it cites. The nearest rival, adding these instances to that issue, mixes two instructions
  in one entry whose title names one.
- **Default**: open an issue, `agent-skills`, kind `design`, `a-command-chained-after-a-failing-check-recurs`.
- **Outcome**: decided by the session: open an issue. Ground: a lapse recurring at one point of
  delivery is repaired by moving its delivery, per
  `skill@knowledge-architect-agent-configuration@admission-standard`; which delivery is a design
  choice with rivals of different costs, so it needs a discussion, and the discussion of the
  claim-check issue weighs the same deliveries for the same mechanism.

## P2: a text replacement wrote one ruling into three thread items

- **a.** What it says: a replacement of a line that three thread items of the milestone document held
  alike wrote the owner's answer on one thread under two threads it did not concern. The transcript
  review of the plan found it. No instruction of the project covers how a session edits a plan
  document by script; one instance; no proposal.
- **d.** Its kind: a lapse, once: an edit applied to every line that matched rather than to the one
  target. No instruction is missing: the plan's transcript review exists to catch a ruling recorded
  where it was not given, and did.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: one instance; the procedure that guards it,
  the transcript review of `skill@knowledge-architect-planning@plan-reviews`, caught it.
