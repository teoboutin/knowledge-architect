# 2026-11-25-knowledge-architect-intake-items-divided

- **Received file**: 2026-11-25-knowledge-architect-intake-items-divided.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, main at `v0.10.0-299-g3fb1d582`; this repository's main commit was `3fb1d582`; the branch was `intake-items-divided`, and the tree held no uncommitted change at the start."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@installed-text-works-with-what-an-installing-project-holds`: constrains, respect.
  - `issue@agent-skills@the-agent-configuration-skill-names-no-place-for-a-local-uncommitted-instruction`: touches, respect.
  - `design@agent-skills@installed-fix-goes-upstream`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: touches, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@every-target-project-can-adopt-it`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.

## old-gh-fails: `gh` 2.46.0 fails `gh pr edit`, `gh pr view` and `gh issue view`, and the instructions name no way out

Members: 2026-11-25-knowledge-architect-intake-items-divided P1; 2026-11-25-knowledge-architect-intake-items-divided-klarch-workflow W4.

- **a. What it says.** P1: `gh pr edit 83 --body ...` printed a deprecation notice on Projects
  (classic) and exited 1 without changing the body; chained with `;`, its exit code did not stop the
  chain, and the session read the message as a notice, then found the stale body and set it with
  `gh api -X PATCH`. Cause, per the file: `gh` 2.46.0, the version Ubuntu 26.04's archive offers,
  asks for a retired field before it edits, and also fails `gh pr view` and `gh issue view` without
  `--json`; fixed upstream in 2.71.0 and 2.73.0. The owner reported that it recurs across sessions
  of this project, and proposed to mention a solution in the Git instructions; the owner then
  installed `gh` 2.105.0. Proposal: no text
  in `CLAUDE.md`, "a tool's version is a suspect statement"; nearest rival, one line naming
  `gh api -X PATCH`, which would not repair `pr view` or `issue edit`; whether an install script
  joins the dotfiles is the owner's. W4: the installed intake skill runs `gh issue edit`, which on
  2.46.0 would fail; the skill's fallback covers it; no text needed.
- **b. Whether it still applies.** No longer on this machine: `gh --version` prints "gh version
  2.105.0 (2026-11-17)", and `dpkg -s gh` gives "Version: 2.105.0". The texts: no instruction of
  this repository names `gh pr edit` or `gh issue view`; `path@knowledge-architect@CLAUDE.md` names
  `gh pr view` twice, near lines 291 and 306, each with `--json`, the form the file says works on
  2.46.0. The installed intake skill, `path@agent-skills@content/skills/retrospective-intake/SKILL.md`
  near line 272, names `gh issue edit` under the outcome "sent upstream", with the fallback "Where
  `gh` is absent or fails, give the owner the text and the address"; in this repository that
  outcome does not apply, per `skill@klarch-retrospective-intake@routes-and-outcomes`. No
  instruction asks a session to edit a pull request's body: a search of the root `CLAUDE.md`, the
  installed text's source and this repository's skills for one returns nothing, so the session ran
  `gh pr edit` by its own judgement.
- **c. Whether it is already recorded.** An earlier occurrence was ruled in the intake of a9e3b3c9,
  "[intake] The analyses of seventeen received retrospective files, every outcome decided by the
  session": its analysis of the session that built `klarch transcript` reads "`gh pr edit` failed
  on GitHub's retired Projects field, and the body was set through the API; no instruction names
  `gh pr edit`", outcome "no change". `design@agent-skills@statement-names-an-act` (`constrains`,
  `respect`): "A statement about state outside the project is suspect, and is written only where an
  act needs it." `design@agent-skills@installed-text-works-with-what-an-installing-project-holds`
  (`constrains`, `respect`) asks that each installed instruction can be followed with what an
  installing project holds; the fallback is that. `design@agent-skills@installed-fix-goes-upstream`
  (`constrains`, `respect`): the installed `gh issue edit` route stands with its fallback.
  `issue@agent-skills@the-agent-configuration-skill-names-no-place-for-a-local-uncommitted-instruction`
  (`touches`, `respect`) names "a tool that only that machine has" among the cases with no place;
  the old package on the owner's other machines is the inverse case, a tool version some machines
  lack, and no instruction is proposed for it. `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`
  (`touches`, `respect`) bears on the session not finding the earlier ruling, which the cluster on
  checks before relying takes; its close is not met.
- **d. Its kind.** An observation of a tool's version on a machine, now upgraded. The `;` chain is
  covered by no rule: the rules on chaining name a check's verdict and the push after it, in
  `instructions@verify-mechanically` and `instructions@git-workflow` point 1; reading an exit 1 as a
  notice is a lapse of `primer@verify-before-relying`, "what it returns on success _and_ on
  failure". W4: an observation; the fallback covers it.
- **e. Its scope.** The machine's packages, which the project does not hold; `goal@every-target-project-can-adopt-it`
  (`constrains`, `respect`): the failure is particular to one package version, so nothing of it
  ships. The dotfiles are outside this repository.
- **f. A better fix.** The owner's proposal, a solution named in the Git section, is the file's
  rival, a line naming `gh api -X PATCH`: it adds an instruction for one command and would not
  repair `gh pr view` without `--json` or `gh issue edit`; the upgrade repairs all three. The
  deciding facts: of the commands the file reports failing, the instructions name only `gh pr view`,
  and only with `--json`, the form the file reports working; the command that failed was run by the
  session's own judgement, which no instruction asks; and a line in the committed Git section would
  state a package version of some machines to every machine and every contributor, a statement
  about state outside the project that no instructed act needs, per
  `design@agent-skills@statement-names-an-act`. That the other commands the instructions name, such
  as `gh pr create --draft` and `gh pr merge`, work on 2.46.0 is an assumption the file does not
  test. On the owner's other machines, which the file says still hold 2.46.0, the repair is the
  same upgrade, the owner's own outside this repository. An occurrence on a machine with `gh`
  2.73.0 or later reopens it, as the file says.
- **g. Its route.** Nothing in the project. Whether an install script for a current `gh` joins the
  owner's dotfiles is outside this repository, the owner's own.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change, against the owner's proposal of a line in the Git section, of which the owner is told. Ground: the cause is a machine's package, now 2.105.0 here; the failing commands the file names are not named by the instructions, but `gh pr view` with `--json`, which works; a statement of a tool's version is suspect and no instructed act needs it, per `design@agent-skills@statement-names-an-act`; the earlier occurrence was ruled the same way in a9e3b3c9.

## verify-before-relying-lapses: a search cut by a pipe, and a claim of origin resting on it

Members: 2026-11-25-knowledge-architect-intake-items-divided P2.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `verify-before-relying-lapses`, which gives the outcome.
