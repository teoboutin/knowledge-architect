# knowledge-architect

@.claude/knowledge-architect/PRIMER.md

A documentation checker and an agent workflow, shipped as Rust crates. The checker verifies that a
project's documents stay consistent with its code and with each other, through declared registers
and checked references. The workflow is a set of agent skills that the checker embeds and writes
into a project on request.
The main goals are declared `register@knowledge-architect@goal` and Components refine or add to them.

The repository is a virtual workspace with Components:

| Component | directory | what it is |
| --- | --- | --- |
| `knowledge-architect` | the root | the project itself |
| `core` | crates/core | package `knowledge-architect`: the checker library and the binary `klarch` |
| `agent-skills` | crates/agent-skills | package `knowledge-architect-agent-skills`: the text the checker installs into a project |
| `gates` | crates/gates | package `knowledge-architect-gates`: the library that runs a project's merge gates |
| `xtask` | tools/xtask | the maintenance tool, `cargo x gates`, `cargo x changelog` and `cargo x skill`; never published |

## Language, tone and style `##language-and-style`

Here `primer@working-prose` binds every text a session writes, text end users read included.

### When presenting content to the user

Answer with precise content, not vague or ambiguous descriptions.

- For prose, a verbatim is better than a reformulation.
- For code, an example or a snippet is better than a description of the effect.

State the consequences of a request explicitly. Never assume the user has considered all of them.

### When writing into durable files in the project

- Prefer lists and tables over long prose paragraphs when listing different items.
- **Current reality only**: no changelogs outside CHANGELOG.md, no "formerly known as".

## Mechanical validation of documents `##mechanical-validation`

The manifest's pin, `checker-version`, is `"self"`, which the checker accepts only in the tree it
is built from, per `design@core@checked-sentinel-values`, so no version is pinned here.

## Where knowledge goes `##where-knowledge-goes`

This repository's own rows, each refining or adding to the primer's, per
`design@agent-skills@knowledge-table-home`:

| the statement is about | home | it leaves when |
| --- | --- | --- |
| a decision about the published workflow: what the installed text tells every adopting project | the agent-skills design home, `register@agent-skills@design`; this repository follows the published workflow, with the departures this file states | the decision changes |
| what is outstanding about the agent configuration itself | the agent-config location's issue directory, `register@agent-config@issue` | the issue closes |
| how a user can use a published crate, beyond the primer's README row | its `path@*@README.md`; its crates.io page is a short `CRATES-IO.md` in the crate, named by `readme` in its Cargo.toml, which points to the README and to docs.rs, per `design@knowledge-architect@crates-io-page-file` | the contract changes |
| the description of a crate's library API | the crate-level documentation of its lib.rs, which docs.rs renders | its contract changes |
| what a consumer must change, can start using, or will see agents do differently, per release | CHANGELOG.md at the root: one section per version, and the `Next release` section | a released section's content never changes; its structure may, and the `Next release` section may change at any time, per `design@knowledge-architect@changelog-section-fixed-at-release` |
| a finding established outside this project that a decision here leans on | `docs/grounding/<subject>.md` in the Component the decision belongs to | the source is superseded, or a better one lands |
| a Component that maintains the repository rather than shipping in it | `tools/<name>/`, one directory per tool | the tool is retired |
| the analysis of a workflow file or an audit file this repository receives, or of its own retrospective project file, the outcome of each finding, and the owner's ruling on each one the owner holds, refining the primer's row for a project's retrospective file | one file per received file in `path@knowledge-architect@docs/retrospective-reports/`, per `design@knowledge-architect@committed-findings-analysis`, written under `skill@knowledge-architect-retrospective-intake` with what `skill@klarch-retrospective-intake` adds | the commit that carries out its last outcome, or a later commit of the same branch, deletes it |

**The primer's last row, ask the owner, is for a statement with no home, not for a choice between
two**, per `skill@knowledge-architect-agent-configuration@project-knowledge-rows`, read whole before
adding a row.

**Never record a number that will go stale, unless staleness is the point.** A figure carries the
claim it supports, the command that re-takes it, and the direction that would reopen the argument.
It does not carry the magnitude, which goes in the commit and cannot go stale there. A threshold, a
count over a frozen corpus, which names the corpus and the state it was counted in, and a zero keep
their digits, because in those the value _is_ the claim.
An issue entry keeps its magnitudes, per the cold-reader standard of
`skill@knowledge-architect-issue-tracking`.

## Verify before relying on anything `##verify-before-relying`

`primer@verify-before-relying` holds the rules. This repository adds four:

- A figure in the core's documents measured in another project, thaum, names thaum as its source.
- **Before touching the code of a Component, ground first, per
  `skill@klarch-development@ground-before-editing`.**
- **No run is built to observe how an agent follows the workflow's instructions**, whether to
  originate an edit, choose a shape, accept a piece of work or check that a text runs as written;
  the harness's mechanics, such as one dispatch of a new agent to see that its tools are granted,
  and the checker's and the installer's behaviour tested over mock projects are admitted, per
  `design@agent-skills@synthetic-evidence-not-built`.
- Before diagnosing anything as a problem, run also:

```sh
ls docs/retrospective-reports/    # the retrospective analyses whose outcomes are not all carried out
ls docs/audit-reports/            # the audit report files whose outcomes are not all carried out
ls docs/goal-reviews/             # the records of goal-setting batches whose goals are not all ruled
```

## Verify mechanically `##verify-mechanically`

The gates are how a session checks that it broke nothing project-wide. **One command runs every
gate, and its exit code is trustworthy:**

```sh
cargo x gates
```

It runs five gates in cost order: `fmt`, `check` (`cargo klarch check`), `commits`
(`cargo klarch commits origin/main..HEAD`), `clippy` with `-D warnings`, and `test`. It does not
stop at a failure unless `--fail-fast` asks for it. It writes each gate's complete output under
`path@knowledge-architect@target/gates/`, and prints one verdict line per gate plus a distilled
extract per failure; when the extract is not enough, the named log holds every byte. CI runs the
same tool on every ready pull request, as
`cargo --locked x gates --locked --fail-fast --require-rebased --full`. `--locked` makes lock drift
fail, and `--require-rebased` adds a sixth gate, `rebased`, first, which fails unless HEAD contains
origin/main. Exit 0 means every gate passed. The gate list's primary home is
`design@xtask@gates-list-primary-home`, and `path@xtask@README.md` is the usage. This is a
restatement.

**Do not filter the tool's output through pipes.** A hand-built filter pipeline hides lines,
and loses the exit code in a shell without `pipefail`. `cargo x gates` refuses a pipe on stdout for that reason, outside GitHub
Actions, per `design@gates@gates-refuse-a-pipe`. When the text is wanted, redirect it to a
file.

`cargo klarch` runs the checker built from this checkout, in release mode. A single gate is still
run by hand when that is cheaper:

```sh
cargo fmt --all --check
cargo klarch check
cargo klarch commits origin/main..HEAD
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

**Which gates you owe depends on what you touched.** Anything that writes a reference or touches a
register entry owes `cargo klarch check`. Any change to the Rust source owes fmt, clippy and the
test suite. Every commit owes `commits`: its message and its tree. **Its range, after each commit, is the
whole branch, `cargo klarch commits origin/main..HEAD`.** The range is the whole branch because this repository refuses citations of its branch's own
commits by SHA, and `commits` sees a citation only of a commit in the range it judges, per
`design@core@branch-shas-are-refused`; the checker-side fix is
`issue@core@branch-sha-citations-are-judged-within-the-range-only`. A commit whose message fails
`commits` is repaired by a reword, per `## Git` point 2. A branch about to merge owes all of them: one
`cargo x gates --require-rebased`.

## Skills `##repository-skills`

**A skill is one activity**: the scope over which a complete set of procedures makes sense, per
`skill@knowledge-architect-agent-configuration@where-text-goes`. Read the matching skill before
doing that kind of work.

**This repository installs its own skills**, with `cargo klarch install-agent-skills`, and
commits them under .claude. The harness lists each installed skill with its description, so a
session finds which applies there. An axis of a review named by an agent, installed or the
project's own, is dispatched as that agent, and any other axis as a fresh general-purpose
subagent, per
`skill@knowledge-architect-review`.

**Grounding in what a shipped skill, agent, saved workflow or the primer says, and why, goes
through `cargo x skill <name>`**, whose usage is `path@xtask@README.md`: the installed copy of a
skill, an agent or the primer holds none of the `%%` comments, which carry the reasons and the
citations of the record, per `design@agent-skills@shipped-text-line-comments`, and the source file
alone names each fragment by a placeholder. A check of the text as it ships still reads the
installed copy.

**This repository's own skills and agents take the prefix `klarch-`, the binary's name**, not the
project's name that `design@agent-skills@project-name-prefix` gives: a name beginning with
`knowledge-architect-` is the installer's namespace, so the checker reports such a skill as
unshipped, per `design@core@owned-namespace-check`, and the install deletes it.

**The findings of both files of `skill@knowledge-architect-retrospective`, and those of the
agentic-workflow axis of `skill@knowledge-architect-project-audit` on the installed text, are handled
here as `design@knowledge-architect@workflow-findings-stay-here` says, read whole.** **So is an
item met outside the task whose fix lies in the installed text or in the checker**: here both are
this project's own, so it takes the rows of `primer@met-outside-the-task`, never the workflow's
file, per that head. Each repair of
an audit's finding on the installed text, its proposal and its check included, is written as
`path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", says, read whole, per that
head. The
audit's checker-rules auditors, L7, also read the checker's design home, `register@core@design`,
beside its README: the installed lens names the README alone, since an adopting project holds no
design home of the checker, and here the checker's rules are decided in that home.

## Git `##git-workflow`

Linear history, no merge commits, and no direct push to main. This section is a restatement. Its home is
`design@knowledge-architect@git-flow`, except where a point or one of its bullets names another; the
commands and procedures that carry those rules out are this file's own. **A scratch worktree made
only for temporary testing is outside every rule of this section**, such as the one
`skill@klarch-development@discriminating-tests` runs mutations in, per
`design@knowledge-architect@scratch-worktree-outside-git-rules`.

1. **All work happens on a branch.**

- No exception by size or kind.
- Once it holds a first commit, the branch is pushed and a **draft** pull request is opened for
  it: `git push -u origin <branch>`, then `gh pr create --draft`. CI does not run on a draft.
- **A commit is pushed only after `cargo klarch commits` has passed on it**: in a command of its
  own, or behind `&&` on the bare check. A push after `;`, or after a check whose output went
  through a pipe in a shell without `pipefail`, runs whatever the check found. Its home is
  `design@knowledge-architect@push-after-the-commits-check`.
- A repair folded into an earlier commit, per `primer@branch-commits`, uses the procedure of
  point 2.
- **The branch writes its CHANGELOG.md entries**, in the `Next release` section, per
  `design@knowledge-architect@the-branch-writes-its-changelog-entries`. A branch that finds no
  `Next release` section creates it above the newest released section. After editing it, run
  `cargo x changelog`, which writes the copy each published crate ships; a test fails while a copy
  differs, per `design@knowledge-architect@the-changelog-ships-in-every-crate`. Which change gets
  an entry, under which subsection, in which shape and with which class are
  `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`:
  read both whole before writing an entry.

2. **No operation that can lose content, committed or not.** Its home is
   `design@knowledge-architect@no-operation-loses-content`, and `design@knowledge-architect@git-flow`
   for "main's history is never rewritten".

- The hazard is **content loss**, in two forms: an operation that removes uncommitted content from
  the working tree, and a history edit that drops a commit no other reference holds.
- **While uncommitted changes exist in the tree, run no operation that touches history or restores
  files.** Examples (non-exhaustive): `reset --hard`, `checkout -- <path>`, `restore`, `clean`,
  `stash`, `commit --amend`, `rebase`. The one exception is `git rebase --continue` on a rebase
  whose conflicts the tree now resolves, as point 3 does.
- **Before applying a mutation that will need reverting, stage the file**, so the revert does not
  depend on a restore: the revert undoes the edit, and `git diff -- <path>` showing no difference
  with the staged copy confirms it.
- **With a clean tree, editing the branch's own history is an ordinary move**: rewording or
  amending a commit message, squashing, rebasing. It is bounded by two verifications before the
  old head is dropped: that the tree was clean, and that no content was lost. For a rewrite that
  keeps the base, `git diff <old-head> HEAD` is empty. For a rebase onto a moved base, compare the
  branch's own delta instead: `git range-diff <old-base>..<old-head> <new-base>..HEAD` reports
  every commit carried over.
- **To reword the message of a commit that is not the newest**, without a rebase that opens an
  editor, which the harness refuses: write a script that reads a message on stdin and writes the reworded
  one, then run `git filter-branch -f --msg-filter '<script>' origin/main..HEAD` with a clean tree.
  Check that `git diff <old-head> HEAD` is empty, which shows no content was lost, and that
  `git range-diff origin/main..<old-head> origin/main..HEAD` marks with `!` each commit the reword
  meant to change and no other, which shows the reword applied: a script whose pattern misses, on a
  wrapped line or a quote, leaves the message as it was and the diff empty all the same. Run both
  checks before any push, then delete the backup refs, in any checkout:
  `git for-each-ref --format='%(refname)' refs/original/ | xargs -r -n1 git update-ref -d`.
- **To fold a repair into an earlier commit**: commit the repair as `git commit --fixup=<commit>`,
  then, with a clean tree, run
  `GIT_SEQUENCE_EDITOR=true git rebase -i --autosquash --keep-base origin/main`, which needs no
  editor and keeps the branch's base even when `origin/main` has moved. Check that
  `git diff <old-head> HEAD` is empty and that the branch holds one commit fewer;
  before git 2.44, `git rebase --autosquash` without `-i` folds nothing and exits 0.
- **main's history is never rewritten.** It is the shared trunk. A pushed branch of your own may
  be rewritten and force-pushed, since origin holds the old head until then. A branch that a live
  worktree has checked out is rewritten only after that worktree is removed.
- To test a previous state of the project, create a worktree in a place where it pollutes
  nothing, such as the worktrees directory under .claude, which is ignored. Remove the worktree
  and its branch once done.
- **Confirm that what you committed is what you wrote** before moving on. A file reverted to an
  earlier state passes every check it passed then, so only this comparison finds it.

3. **The branch is rebased on main before review and merge.** GitHub's ruleset refuses to merge a
   pull request whose branch is behind main; the local check below finds it earlier.

```sh
git fetch origin main
git merge-base --is-ancestor origin/main HEAD    # if false, rebase
```

A rebase that stops on a conflict in a generated `index.md` is resolved by regenerating it:
`cargo klarch index`, then `git add` on the file and `git rebase --continue`. Its content is
computed, so no hand merge is needed and none is lost.

4. **Work is reviewed before any merge to main.**

- Use `skill@knowledge-architect-review` before the merge.
- The axes are those `skill@knowledge-architect-review@review-axes` says the review before a
  merge owes.
- A repair, a repaired commit message and the record of the review land as
  `skill@knowledge-architect-review@what-review-leaves` says, read whole, by the procedures of
  point 2. The decision is `design@agent-skills@review-repair-appended-or-folded`.
- Every finding gets one of the outcomes of `skill@knowledge-architect-review@what-review-leaves`: repaired, opened as an
  issue entry per `skill@knowledge-architect-issue-tracking`, or judged to need nothing, with the
  reason.
- Once the branch is rebased and the repairs are pushed, mark the pull request ready:
  `gh pr ready`. That starts CI, and every later push re-runs it. First check that GitHub has
  taken the push: `gh pr view <branch> --json headRefOid` must equal `git rev-parse HEAD`. A pull
  request marked ready before that runs CI on its previous head, and a push that landed while it
  was a draft is skipped, so no run covers the current head.

5. **When CI passed on the current head, merge the pull request through GitHub.** main accepts
   no direct push: a ruleset requires a pull request whose branch is up to date with main and
   whose CI check passed, and a linear history. The merge predicate:

```sh
git fetch origin main
git merge-base --is-ancestor origin/main HEAD    # still true, or go back to point 3; a line the
                                                 # branch adds or removes that the rebase changed,
                                                 # as `range-diff` shows, is reviewed as a repair is,
                                                 # per `skill@knowledge-architect-review@what-review-leaves`
gh pr view <branch> --json isDraft,headRefOid    # isDraft false, headRefOid equal to `git rev-parse HEAD`
gh run list --commit $(git rev-parse HEAD) --json conclusion -q 'any(.[]; .conclusion == "success")'
                                                 # true: a run on that head succeeded
gh pr merge <branch> --rebase --delete-branch
git checkout main && git pull --ff-only
```

- The draft flag is read here because a job skipped on a draft reports `skipped`, which GitHub
  counts as passing. For the same reason the predicate asks for a successful run on the head
  rather than the exit code of `gh pr checks`: right after `gh pr ready`, the only run on the head
  can be the one skipped while it was a draft, and `gh pr checks` then exits 0.
- GitHub's rebase merge gives main the branch's commits with the same trees and messages, and new
  SHAs, **except a commit that changes no file, which it drops**:
  a record carried by a message alone rides on a commit that changes a file. So **neither a commit message nor a document cites the
  SHA of a commit of its own branch**: name that commit by its subject. A SHA already on main may
  be cited. `cargo klarch commits` refuses such a citation, per
  `design@core@branch-shas-are-refused`. Its homes are
  `design@knowledge-architect@no-branch-sha-is-cited` and
  `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`.
- A local branch that `--delete-branch` left, as one another worktree has checked out, is deleted
  with `git branch -D <branch>`: its commits are not ancestors of main, since their SHAs differ.
- A merge to main publishes nothing. A release is a separate procedure, per
  `design@knowledge-architect@publish-after-merge`.

## Release status `##release-status`

**The crates are published on crates.io, and at least one consumer is known.**

- **The project stays at 0.x, and breaking changes stay allowed**, until the owner's word, per
  `design@knowledge-architect@stays-at-zero-x`. How each kind of change is versioned is
  `design@knowledge-architect@versioning-policy`.
- **A change's cost is first what a consumer could no longer do, and a migration is the lowest
  cost.** This is a restatement; its home is
  `design@knowledge-architect@capability-cost-before-migration`.
- **Do not argue a decision on the grounds that changing it later would be breaking.** This is a
  restatement; its home is `design@knowledge-architect@no-future-breaking-cost-argument`.
