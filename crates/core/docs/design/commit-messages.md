# Core design: Commit messages

## Commit messages

### A commit message is a document under the regime, judged against the tree its commit carries `##a-commit-message-is-a-document`

A commit message is parsed as one markdown document — its subject line, its blank line and its
body — and every rule of the regime runs over it: every reference is judged against the entity
table of `design@core@one-entity-table`, and each registered extension judges the message
against the tree it prepared for that commit.

**A message is history, and it cannot be edited.** `main`'s history is never rewritten, so a
message's claims are fixed the moment it lands: a pointer that stops resolving in a document is
repaired where it stands, and the same pointer in a message can only be read wrong. That is why
the regime matters most on messages.

**A message is judged byte for byte, as the commit holds it.** `commits` reads the message
with `%B` and cleans nothing: git applied its own cleanup before the commit existed, and a
second pass would take bytes of a commit out of the regime — under `-m` a `#` line is committed
verbatim.

**One command judges messages, after the commit exists.** `commits <range>` judges every commit of
the range against its own tree, with its first parent. It is run over `HEAD~1..HEAD` after each
commit, or over the branch where `design@core@branch-shas-are-refused` is turned on, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`, so a finding in the newest
commit's message is repaired by `git commit --amend`, and over the branch by the gates and by CI
before any merge, which is where the guarantee lies: `main` is never rewritten, so what must not
happen is a message with a finding reaching it, and the range check before the merge excludes that.
No hook judges a draft before the commit exists. A hook repeats the range check's message half, not
its tree half, at every commit; it is per-clone state a session has to install and check; and it
resolves a message against HEAD, which under an amend, a reword or a squash is the commit being
replaced, so it refuses history edits the range check accepts.

**Everything a commit is judged against is read from that commit's tree.** The manifest, the
`[walk]` rules, the documents, the generated indexes, the per-instance options and every file an
extension reads all come through git objects, and the model is assembled in memory. Reading them from the
working tree instead would judge a message written a hundred commits ago against decisions that
did not exist then, and the finding list would be a list of things nobody could have known.

**Every commit of the range is judged by the tip checker, and a tree that fails is a finding.**
The tip checker is the binary built from the working tree, and it judges every commit's tree and
every commit's message. **Every commit of the range is judged alike, the last one included, and
wherever HEAD is.** A commit whose tree does not load, or carries findings, fails the run with
exit 1: its tree's findings are printed, each named by the commit and by the file inside it, and
its message is still judged wherever the tree reached the last phase, since its entity table is
then complete. Where the tree stopped earlier, the message is judged against nothing and the run
says so. No commit is set apart as the range's tip: a range that does not end at the checkout,
such as a pre-push hook's or an audit of old history, holds no commit that `check` has judged,
and treating its last commit specially would leave that commit's failure unreported as a finding.
The summary counts the range's commits, how many passed and how many failed. A commit fails when
any finding of the run belongs to it: its tree's, its message's, or a citation of the range by
SHA. Its line names each of those sources, with its count of findings, except a tree that does not
load, which is one finding. So the header agrees with the verdict line, and passed plus failed is
the number of commits. Separate counts of failed trees and
failed messages in the header lose to this: one commit can fail on both, so either such a count
overlaps the other, or one source is left out.

**A branch that makes the checker stricter orders its commits for it.** A new check, or a change
to the manifest format, makes every earlier commit of the same branch fail under the tip checker.
Such a branch puts that change in its first commit, with every fix the tree needs to pass it, or
is squashed to one commit before its review. Checker changes are rare and land on branches of
their own, so this costs a history edit on those branches alone; the alternative that avoids it,
building each commit's own checker, costs one release build per commit on every branch.

**A message's references resolve against its commit's tree or its first parent's, and its lints
stand where either tree holds what they name.** A commit that closes an issue deletes the entry
and names it in the message, and against its own tree alone every such message would dangle. A
reference finding therefore survives where both trees refuse the reference. The unanchored path
and the retired slug reference, per `design@core@every-path-names-its-anchor` and
`design@core@retired-slug-reference-reported`, fire where their span names something of the
project, so a finding of either stands where either tree holds what it names: a message naming,
by a bare path, the file its commit deletes still points at it. The parent's model is the previous commit's wherever
the walk followed the parent chain — a failed commit's tree still serves as the next commit's
parent, since its entities are read and not its verdict — so an extra model is built only at
the range's start and where the chain was not followed. **A tree whose manifest fails phase 1 is
the exception**: nothing of it is read past its manifest, it is judged no further, and it serves
as no parent, so the next commit's message resolves against its own tree alone. Such a commit
has already failed the range, and reading its whole tree for its successor's sake dominates the
time `commits` takes over a range of them. A parent tree is assembled for its
entity table and the facts a path reference and the two lints ask about, and nothing else: no extension is
prepared for it and no check runs over it, because nobody reads its verdict. The two arms are compared by the site each names — the line and the span — rather than by the words each writes,
because two trees can refuse one reference for different reasons and comparing the words whole
would let a reference that resolves in neither pass.

**A historical tree's `[project] checker-version` is read, not compared.** Its absence stops the
tree in phase 1; its value is never compared with the tip checker's version, so moving the pin
fails no earlier commit, per `design@core@installed-binary-version-check`.

**`check` reads no range of history.** A message is not a file of the tree, and a check whose
verdict moved with the branch's history would be a check nobody could reproduce from a checkout:
`git stash` alone would move it. `check --staged` reads HEAD's tree and the index, one tree and no
range, per `design@core@staged-tree-source`. The range is always explicit, and the gates pass the range from the
remote `main` to `HEAD`, so what is judged is the branch's own commits and never `main`'s.

**An extension leaves out, over a commit's tree, a check whose subject is not the tree.** A
check of filesystem state has no subject there, and one that would read every archived input at
every step of the range decides nothing about whether a message's references resolve, which is
the question the per-commit model exists to answer; `check` is what judges a checkout against
it.

**The ignore rules a per-commit run asks are the working tree's.** `git check-ignore` reads the
`.gitignore` files on disk and has no form that asks a historical tree. A commit whose ignore
rules differed from today's is therefore judged against today's, which can cost a path reference
asserted where that commit's own rules exempted it — a finding rather than a silence. Since a
failing tree fails the range, a branch that changes its ignore rules orders its commits for it
as a branch that makes the checker stricter does: the change first, or a squash.

The designs this decision did not keep: `rejected@core@check-refusing-a-pipe`,
`rejected@core@commit-subcommand` and `rejected@core@commit-msg-hook`.

### `commits` refuses a citation of a commit of its range by SHA, where the manifest turns it on `##branch-shas-are-refused`

Under `[commits] refuse-branch-shas = true`, `commits` reports every run of 7 to 64 lowercase hex
digits, bounded by bytes that are not ASCII letters, digits or `_`, that prefixes the SHA of a
commit of the range it judges: in each commit's message, and in the whole text of each document of
that commit's tree, a Rust file's code and literals included. A commit whose tree stops at phase 2
or 3 is scanned too, since the scan needs no entity table and no verdict. One whose tree stops at
phase 1 is not: its manifest did not resolve, so the option is not read from it, and no document of
it was read. The range a branch is judged over is
its own commits, so these are exactly the commits a merge that rebases gives new SHAs; a citation of
one would name nothing once the branch merges. A SHA on the main branch, or of another project, is
not in the range and passes, unless its first 7 digits collide with a SHA of the range. **A citation
is seen only when the cited commit is in the range judged**, so where the option is on, the check
after a commit is run over the branch, `<main>..HEAD`: over `HEAD~1..HEAD`, a citation of an earlier
commit of the branch passes, and fails first at the merge gate, after the push, per
`issue@core@branch-sha-citations-are-judged-within-the-range-only`. Only lowercase counts, since git
prints a SHA in lowercase, and `_` bounds a word as a letter or a digit does, since it joins the
parts of an identifier. A project whose branch commits keep their SHAs until they reach the main
branch has nothing to refuse; a rebase onto main before a fast-forward rewrites them as surely as a
rebase merge does, so the check is off unless the manifest turns it on. It serves
`goal@core@document-rules-are-checked`: a rule against such citations is kept by running
`commits`, not by a reviewer remembering it.

The check lives in `commits` because it already holds every input: the range's commits, each
message byte for byte, and each commit's tree assembled into a model. A gate of its own would
list the range and read the messages again. A SHA in a file the walk does not read is not seen,
which is the checker's scope everywhere else. Sixty-four digits is the length of a SHA in a
repository under SHA-256. A citation of a commit the branch rewrote before `commits` ran is not in
the range, and is not seen: no clone can tell reliably which commits a branch once held, since a
fresh clone, such as CI's, never fetched them.
