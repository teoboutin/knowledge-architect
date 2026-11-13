# Check design: Commit messages

## Commit messages

### A commit message is a document under the regime, judged against the tree its commit carries `##a-commit-message-is-a-document`

A commit message is parsed as one markdown document — its subject line, its blank line and its
body — and every rule of the regime runs over it: every reference is judged against the entity
table of `design@model@one-entity-table`, and each registered extension judges the message
against the tree it prepared for that commit.

**A message is history, and it cannot be edited.** `main`'s history is never rewritten, so a
message's claims are fixed the moment it lands: a pointer that stops resolving in a document is
repaired where it stands, and the same pointer in a message can only be read wrong. That is why
the regime matters most on messages.

**Everything a commit is judged against is read from that commit's tree.** The manifest, the
`[walk]` rules, the documents, the generated indexes, the per-instance options and every file an
extension reads all come through git objects, and the model is assembled in memory. Reading them from the
working tree instead would judge a message written a hundred commits ago against decisions that
did not exist then, and the finding list would be a list of things nobody could have known.

**The ignore rules a per-commit run asks are the working tree's.** `git check-ignore` reads the
`.gitignore` files on disk and has no form that asks a historical tree. A commit whose ignore
rules differed from today's is therefore judged against today's, which can cost a path reference
asserted where that commit's own rules exempted it — a finding rather than a silence. Since a
failing tree fails the range, a branch that changes its ignore rules orders its commits for it
as a branch that makes the checker stricter does, per
`design@check@every-commit-judged-by-the-tip-checker`: the change first, or a squash.

### A message is judged byte for byte, as the commit holds it `##message-judged-byte-for-byte`

`commits` reads the message with `%B` and cleans nothing: git applied its own cleanup before the
commit existed, and a second pass would take bytes of a commit out of the regime of
`design@check@a-commit-message-is-a-document` — under `-m` a `#` line is committed verbatim.

### One command judges messages, after the commit exists `##one-command-judges-messages-after-the-commit`

`commits <range>` judges every commit of the range against its own tree, with its first parent. It
is run over `HEAD~1..HEAD` after each commit, or over the branch where `design@check@branch-shas-are-refused` is turned on, per
`issue@check@branch-sha-citations-are-judged-within-the-range-only`, so a finding in the newest
commit's message is repaired by `git commit --amend`, and over the branch by the gates and by CI
before any merge, which is where the guarantee lies: `main` is never rewritten, so what must not
happen is a message with a finding reaching it, and the range check before the merge excludes that.
No hook judges a draft before the commit exists. A hook repeats the range check's message half, not
its tree half, at every commit; it is per-clone state a session has to install and check; and it
resolves a message against HEAD, which under an amend, a reword or a squash is the commit being
replaced, so it refuses history edits the range check accepts.

The designs this decision did not keep: `rejected@check@check-refusing-a-pipe`,
`rejected@check@commit-subcommand` and `rejected@check@commit-msg-hook`.

### Every commit of the range is judged by the tip checker, and a tree that fails is a finding `##every-commit-judged-by-the-tip-checker`

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
SHA. Its line names
each of those sources, with its count of findings, except a tree that does not load, which is one
finding. So the header agrees with the verdict line, and passed plus failed is
the number of commits. Separate counts of failed trees and
failed messages in the header lose to this: one commit can fail on both, so either such a count
overlaps the other, or one source is left out.

**A branch that makes the checker stricter orders its commits for it.** A new check, or a change
to the manifest format, makes every earlier commit of the same branch fail under the tip checker.
Such a branch puts that change in its first commit, with every fix the tree needs to pass it, or
is squashed to one commit before its review. Checker changes are rare and land on branches of
their own, so this costs a history edit on those branches alone; the alternative that avoids it,
building each commit's own checker, costs one release build per commit on every branch.

### A historical tree's `[project] checker-version` is read, not compared `##historical-pin-read-not-compared`

Its absence stops the tree in phase 1; its value is never compared with the tip checker's version,
so moving the pin fails no earlier commit, per `design@check@installed-binary-version-check`.

### An extension leaves out, over a commit's tree, a check whose subject is not the tree `##extension-leaves-out-checks-not-about-the-tree`

A check of filesystem state has no subject there, and one that would read every archived input at
every step of the range decides nothing about whether a message's references resolve, which is
the question the per-commit model exists to answer; `check` is what judges a checkout against
it.

### A message's references resolve against its commit's tree or its first parent's, and its lints stand where either tree holds what they name `##message-references-resolve-in-either-tree`

A commit that closes an issue deletes the entry and names it in the message, and against its own tree alone every such message would dangle. A
reference finding therefore survives where both trees refuse the reference. The unanchored path,
per `design@check@unanchored-path-first-segment-here`, fires where its span names something of the
project, so its finding stands where either tree holds what it names: a message naming,
by a bare path, the file its commit deletes still points at it. The parent's model is the previous commit's wherever
the walk followed the parent chain — a failed commit's tree still serves as the next commit's
parent, since its entities are read and not its verdict — so an extra model is built only at
the range's start and where the chain was not followed. **A tree whose manifest fails phase 1 is
the exception**: nothing of it is read past its manifest, it is judged no further, and it serves
as no parent, so the next commit's message resolves against its own tree alone. Such a commit
has already failed the range, and reading its whole tree for its successor's sake dominates the
time `commits` takes over a range of them. A parent tree is assembled for its
entity table and the facts a path reference and the unanchored path lint ask about, and nothing else: no extension is
prepared for it and no check runs over it, because nobody reads its verdict. The two arms are compared by the site each names — the line and the span — rather than by the words each writes,
because two trees can refuse one reference for different reasons and comparing the words whole
would let a reference that resolves in neither pass.

### `check` reads no range of history `##check-reads-no-range-of-history`

A message is not a file of the tree, and a check whose verdict moved with the branch's history would be a check nobody could reproduce from a checkout:
`git stash` alone would move it. `check --staged` reads HEAD's tree and the index, one tree and no
range, per `design@check@staged-tree-source`. The range is always explicit, and the gates pass the range from the
remote `main` to `HEAD`, so what is judged is the branch's own commits and never `main`'s.

### No check judges a change of a frozen file: the review before a merge does `##a-frozen-file-is-guarded-by-the-review`

`commits` judges no change of a file under a milestone's harvested/, as `check` judges none: a
commit that modifies, adds or deletes one passes or fails on its tree and its message alone. The
rule that no commit modifies a frozen file, or adds one other than by the move that freezes its
slice, is an instruction of the installed text, and the review
before a merge judges each commit of the branch against it. A frozen file is out of the walk and
read for its definitions alone, per `design@model@a-harvested-slice-is-frozen`, so after its slice
was live no rule of the checker judges it.

The owner chose the review over a clause of `commits`, which no goal ranks, on the argument that a
check cannot reliably cover every sequence of actions a session may take on the frozen files, and
that a review handles those sequences more flexibly. The record bears it out: the clause built for
it admitted, in its first wording, three sequences that an adversarial reviewer of its code
reached, a freezing move that also edits the slice, a rename inside harvested/ that edits the file,
and a frozen file deleted in one commit and added again with new content in the next. It strains no
goal: `goal@documentation-stays-consistent` binds in the texts the checker reads, and a frozen file,
parsed for definitions only, is none, a reading the owner confirmed. Its cost is an edit of a
frozen file that the review misses, which reaches the main branch with nothing reporting it. The
design not kept: `rejected@check@commits-refuses-a-change-under-harvested`.

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
`issue@check@branch-sha-citations-are-judged-within-the-range-only`. Only lowercase counts, since git
prints a SHA in lowercase, and `_` bounds a word as a letter or a digit does, since it joins the
parts of an identifier. A project whose branch commits keep their SHAs until they reach the main
branch has nothing to refuse; a rebase onto main before a fast-forward rewrites them as surely as a
rebase merge does, so the check is off unless the manifest turns it on. It serves
`goal@documentation-stays-consistent`: a rule against such citations is kept by running
`commits`, not by a reviewer remembering it.

The check lives in `commits` because it already holds every input: the range's commits, each
message byte for byte, and each commit's tree assembled into a model. A gate of its own would
list the range and read the messages again. A SHA in a file the walk does not read is not seen,
which is the checker's scope everywhere else. Sixty-four digits is the length of a SHA in a
repository under SHA-256. A citation of a commit the branch rewrote before `commits` ran is not in
the range, and is not seen: no clone can tell reliably which commits a branch once held, since a
fresh clone, such as CI's, never fetched them.
