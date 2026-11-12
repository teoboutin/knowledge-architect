---
name: klarch-release
description: MUST use when the owner asks to publish a version of knowledge-architect to crates.io. Covers choosing the version, the changelog section, the checks of the shipped text before publishing, the package lists, the dry run, the review and merge that come before publishing, the owner's word to publish, the tag and the publish, and what to do when a publish fails part-way.
---

# Release

Scope: publishing one version of the published crates, from main: every crate under crates/, whose
package names
`cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish != []) | .name'`
prints. xtask, under tools/, is never published. The versioning policy is
`design@knowledge-architect@versioning-policy`, and every published crate shares one version, per
`design@knowledge-architect@version-lockstep`.

**Everything that can be undone comes before the publish, which cannot.** A merge to main is
repaired by a later commit; a version on crates.io can be yanked and never deleted. So the release
is reviewed and merged first, and published from main last, on the owner's word, per
`design@knowledge-architect@publish-after-merge`.

Not covered here: **a repair to a crate's code** found on the release branch,
`skill@klarch-development`; **the changelog entries of each change**, which each branch writes, per
`instructions@git-workflow`, point 1; **the review itself**, `skill@knowledge-architect-review`,
which step 9 dispatches.

## The release branch `##release-branch`

On a branch of its own, `release-<version>`, under the Git rules of the root `CLAUDE.md`:

1. **The version.** Choose it under the versioning policy, and set it at every site:
   - `[workspace.package] version` in the root Cargo.toml;
   - the exact pins `version = "=<version>"` between the workspace crates, in the Cargo.toml of
     each crate under crates/ that depends on another, which the grep below finds;
   - Cargo.lock, which any cargo command rewrites for every workspace package;
   - the status line: the opening sentence of `## Status` in the root README.md. Change the
     number, and any other part of the sentence the release makes false.

   `git grep -n '<previous version>'` finds every site that still names the previous one. A
   statement about a past version, such as a released changelog section or an issue entry's
   evidence, and an example inside a skill, such as an xtask manifest's own version, are not
   sites.
2. **The changelog.** Rename CHANGELOG.md's `## Next release` to `## <version>`, or, where
   `## <version>` already exists, move the entries into it and delete the `Next release`
   heading. Then run `cargo x changelog` to write the crates'
   copies, per
   `design@knowledge-architect@the-changelog-ships-in-every-crate`. Each branch wrote its own
   entries there, per `design@knowledge-architect@the-branch-writes-its-changelog-entries`. The
   version chosen at step 1 follows the highest bump class among its entries, a patch at least.
3. **Commit**, with `cargo klarch check --staged` before and
   `cargo klarch commits origin/main..HEAD` after: the whole branch, per
   `instructions@verify-mechanically`, `design@agent-skills@staged-check-before-each-commit`, and
   `issue@check@branch-sha-citations-are-judged-within-the-range-only`.
   Cargo refuses to package an uncommitted tree, so the commit comes before the next steps. Then
   push the branch and open a draft pull request, per `instructions@git-workflow`, point 1.
4. **The shipped text cites no entry.** List every backticked span with an `@` in the installed
   copies, which hold the text as it ships, its `%%` comments removed and its placeholders filled,
   and read each one:

   ```sh
   grep -rnoE '`[^`]*@[^`]*`' .claude/knowledge-architect .claude/skills/knowledge-architect-* .claude/agents/knowledge-architect-* .claude/workflows/knowledge-architect-*
   ```

   A span is a reference candidate when its head before the first `@` is a kind or an anchor, per
   `design@model@at-span-candidate-rule`. Every candidate either carries a placeholder in
   angle brackets, as in `design@<anchor>@<id>` or the goal form `goal@<id>`, is a path or a
   register's home every conforming project holds, `path@*@<path>`, `path@plans@<path>`,
   `register@*@<register>`, `register@plans@<register>` or `register@goal`, or cites the shipped
   set's own skills, agents, primer and sections, as `skill@<name>@<slug>` or `primer@<slug>`, which a test of the checker resolves
   within the set. No candidate names an entry. This is done by hand until
   `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` closes, per
   `design@agent-skills@shipped-text-cites-no-entry`.
5. **Every name the shipped text uses is shipped**, per `design@agent-skills@no-external-handoff`:

   ```sh
   grep -rhoE 'knowledge-architect-[a-z-]+' .claude/knowledge-architect .claude/skills/knowledge-architect-* .claude/agents/knowledge-architect-* .claude/workflows/knowledge-architect-* | sort -u
   ```

   Each name must be a shipped skill (`content/skills/<name>/`) or a shipped agent
   (`content/agents/<name>.md`) or a shipped saved workflow (`content/workflows/<name>.js`), each
   installed as `knowledge-architect-<name>`, or one of the published crates. Every document the shipped text relies on in a project, such as the knowledge
   table, is written by an installed skill.
6. **crates.io.** For each crate, the version itself must be absent, and the crate is either one
   crates.io does not hold yet or one owned by the owner's account. The checks of each crate:

   ```sh
   curl -s -A 'knowledge-architect release check' https://crates.io/api/v1/crates/<crate>/<version>
   curl -s -A 'knowledge-architect release check' https://crates.io/api/v1/crates/<crate>
   ```

   The first answers 404 whenever the version is absent; the second answers 404 only for a new
   crate, and otherwise lists the crate, whose owners `https://crates.io/api/v1/crates/<crate>/owners`
   lists. A new crate takes a placeholder version, per
   `design@knowledge-architect@new-crate-placeholder`: tell the owner which crates, and carry the
   release on to its merge. After the merge of step 10 and before the tag of step 11, the owner,
   never the agent, publishes each placeholder with an API token, from a scratch directory outside
   every repository:

   ```sh
   cargo new --lib --vcs none <crate> && cd <crate>
   : > src/lib.rs
   # In Cargo.toml: version = "0.0.0", license = "MIT OR Apache-2.0", and
   # description = "Placeholder: the first version is published by the release workflow of https://github.com/teoboutin/knowledge-architect"
   cargo publish
   ```

   then, in the crate's settings on crates.io, adds its trusted publisher, this repository, the
   workflow file release.yml and the environment `release`, and turns on "Require trusted
   publishing for all new versions". crates.io limits how many new crates one account publishes in a
   short time, per `path@knowledge-architect@docs/grounding/crates-io-publishing.md`: a publish it
   refuses answers HTTP 429 with the time to retry, after which the owner publishes that placeholder
   again. A placeholder is published only in the release that publishes its crate's real version,
   per `design@knowledge-architect@placeholder-only-in-its-release`. The workflow's `publish` job
   then publishes the real version of every crate.
7. **The package lists.** `cargo package --list -p <crate>` for each crate. Each lists its sources,
   its Cargo.toml, its README, its CRATES-IO.md, its licence files and its CHANGELOG.md, and agent-skills also its
   build script, content/, snippets/ and fragments/, beside the three files Cargo generates for every package,
   `.cargo_vcs_info.json`, `Cargo.lock` and `Cargo.toml.orig`, per
   `design@knowledge-architect@package-include-whitelist`. Compare each
   list with these, and report to the owner every file the step does not name. The owner may read
   the lists; the release does not wait for it.
8. **The dry run.** `cargo publish --workspace --dry-run` passes, skips xtask, and orders
   each crate before the crates that depend on it. Its warning "ignoring test `<name>`" for each
   file `<name>.rs` that `git ls-files ':(glob)crates/*/tests/*.rs'` lists is expected: the
   whitelist keeps the tests out.
9. **The gates and the review.** `cargo x gates --require-rebased`, then the push, and the review
   under `skill@knowledge-architect-review`. Its axes include
   `agent@klarch-changelog-reviewer`, dispatched on `v<previous>..<commit under review>`, the head's commit at the dispatch: every change of the range
   has the entries the policy owes, each entry's class is right, and the version is their highest
   class, per `design@knowledge-architect@changelog-reviewed-at-the-release`. A repair lands as `skill@knowledge-architect-review@what-review-leaves` says, and
   each of steps 2 and 4 to 8 whose result the repair changes runs again on the head.

## Merge, then publish `##merge-then-publish`

10. **Merge** under the root `CLAUDE.md`'s merge predicate. Before the merge, record the reviewed
    head, `git rev-parse HEAD` on the branch, in the conversation, since a shell variable may not
    outlive the command that set it: the merge deletes every ref that names it, and step 11
    compares with it. From the merge until the publish, main's
    README status line names a version that is not yet on crates.io.
11. **Tag**, with a clean tree on main, once every new crate of step 6 has its placeholder and its
    trusted publisher. The tag push starts
    `path@knowledge-architect@.github/workflows/release.yml`:

    ```sh
    git checkout main && git pull --ff-only
    test "$(git rev-parse HEAD^{tree})" = "$(git rev-parse <the head recorded at step 10>^{tree})"
    git tag -a v<version> -m "Release <version>"
    git push origin v<version>
    ```
12. **The owner's approval.** The workflow's `verify` job checks that the tag names a commit on
    main and the version every crate carries. Its `publish` job then waits for the approval of the
    environment `release`, which is the owner's word to publish, per
    `design@knowledge-architect@owner-word-mechanism`. Give the owner the run's address,
    `gh run list --workflow release.yml --branch v<version> --limit 1 --json url,databaseId`. Only
    the owner approves: never approve a deployment, though the agent's `gh` can. Then follow the
    run with `gh run watch <run id> --exit-status`, and once it passes, check step 6's address for
    each crate: the version must now be present. The publish credential is a token crates.io issues
    to the run, per `design@knowledge-architect@publish-credential`, so nobody logs in to crates.io
    to publish a real version.

## When the publish fails `##failed-publish`

The `publish` job uploads the crates one by one, in the order of its list in
`path@knowledge-architect@.github/workflows/release.yml`, and skips a crate whose version crates.io
already holds. A crate it uploaded stays published.

- **The `verify` job failed, the owner declined, or nothing was uploaded**: delete the tag, with
  `git push origin :refs/tags/v<version>` and `git tag -d v<version>`, repair on a branch, run
  again on its head the steps that step 9 names for a repair, and release the same version again
  from step 9.
- **Some crates were uploaded**: re-run the failed job, `gh run rerun <run id> --failed`, from the
  same tag, per `design@knowledge-architect@partial-publish-rerun`; where the re-run waits for an
  approval, the owner gives it as at step 12. If that cannot succeed without a change to the
  code, the uploaded crates keep the version, and the next release is the next patch version for
  every published crate, per `design@knowledge-architect@version-lockstep`; the uploaded version is yanked only
  if it is broken.
- **The release is abandoned after step 6's placeholders were published**: the owner deletes each
  placeholder's crate on crates.io, which crates.io allows within 72 hours of its publication, per
  `path@knowledge-architect@docs/grounding/crates-io-publishing.md`, since a placeholder outlives
  no release that publishes its crate, per
  `design@knowledge-architect@placeholder-only-in-its-release`. A release of the same version again
  keeps them.
- **In either case**, main's README status line is wrong until a repair lands, through a branch and a
  pull request like any other change.
