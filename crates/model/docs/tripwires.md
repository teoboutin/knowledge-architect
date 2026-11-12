# Tripwires — model

Evidence that would reopen a decision about what knowledge-architect reads of a project. Entry shape and
the movement instruction are `skill@knowledge-architect-issue-tracking`.

## Guarding `design@model@at-span-candidate-rule`: a span meant as a reference passes the check unreported `##candidate-rule-silence`

**Fires when:** a review, or a session reading a document, finds a backticked span that was
meant as a reference and for which `cargo klarch check` reported nothing — a typo inside the
kind, an unmigrated span whose head is neither a kind nor an anchor, or a shape the tokenizer
does not record. A slug with one hash sign, a backticked `#<id>` or `<word>#<id>`, does not fire
it: its silence is decided by `design@check@hash-slug-span-read-by-no-lint`, which names the
event that reopens it. `cargo klarch model` filtered on the file shows whether the span was
recorded as a `span` observation at all.
**Response:** reopen `design@model@at-span-candidate-rule` on the shape found, with the rule widened
to it, under a false-positive census over the tree for the widened class, among the candidates; or,
where the decision is not reopened in the same change, open a `defect` naming the span.
**Re-entry:** the review of the migration that follows a grammar change.
**Watches until:** standing: a span the scanner leaves silent is reported by no later run, and the
class stays after each shape found is repaired, so its failure stays silent and permanent.

## Guarding `design@model@git-supplies-the-walk`: two machines walk a different set of files for one commit `##walked-count-differs-between-machines`

The walk is a `git` invocation, so what is checked depends on the git installed and on the
per-clone configuration it reads. The design answers the two loud cases: no binary and no
worktree are exit 2, per `design@git@failed-git-is-an-error`. It pins the per-user ignore file
away, per `design@git@per-user-ignore-file-pinned`. This guards the quiet case that remains: two
machines running the same commit and reading a different set of files, with both runs green.

**Fires when:** the walked-file count CI prints for a commit differs from the count a local run
prints for the same commit. Both are the `walk: n file(s)` line of the summary block, which every
`cargo klarch check` prints on a stop as on a pass.
**Response:** open a `defect` naming both counts and the two git versions.
**Re-entry:** each time CI's output is read against a local run, and at any change to the walk.
**Watches until:** the third release tag after the commit that closes
`issue@model@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`, whose check then
reports the per-clone exclude file, and which narrows this entry to two git versions.
