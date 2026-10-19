---
kind: observation
---
# A retrospective's version line can name a main commit the repository does not hold

## Summary

Two retrospective files of this repository named, in their version line, a `git describe` whose
commit exists nowhere in the repository, while its tag and its count resolved to a main commit. The
intake read the blamed text at the count. The mechanism is not established.

## Details

### What

- The retrospective's version line, `skill@knowledge-architect-retrospective@what-it-examines`,
  describes "the main commit the session's tree contains" with
  `git describe --tags $(git merge-base HEAD <main branch>)`.
- Two files received at one intake gave `v0.7.0-1-g7af8ca20` and `v0.7.0-14-g1754037f`.
  `git cat-file -t` on either SHA fails in this repository, and the counts resolve: v0.7.0-1 is the
  commit "Cleanup noisy uneeded info from CLAUDE.md" and v0.7.0-14 the commit "[review] The
  owner's ruling on Q2: a rarer rule caught and removed on its own branch does not fire T2", the
  last commit of the branch whose retrospective named v0.7.0-1. The other eleven files received with
  them named a version that resolved, or a published version.
- The installed intake now reads such a line at the commit its count gives, in
  `skill@knowledge-architect-retrospective-intake@what-to-establish`, item b.
- The suspected mechanism, an assumption: the session described a local main branch that held the
  branch's own commits, as a merge done locally before GitHub's rebase merge would, whose SHAs the
  rebase merge then replaced. Not reproduced.

### Why it matters

`design@agent-skills@version-line-names-the-main-commit` names the main commit so that an intake
reads the blamed text as the session read it. A SHA the repository does not hold leaves the intake a
count to trust instead, which is right only while the main branch's history between the tag and the
commit is linear, as `design@knowledge-architect@git-flow` keeps it here.

### What would close it

A session that reproduces a version line naming a SHA its repository does not hold, showing the
mechanism; then either the retrospective computes the line in a way that survives the merge, or the
entry closes on the count fallback, with the reason. Or several releases of retrospectives with no
such line, and the entry closes as not reproduced.
