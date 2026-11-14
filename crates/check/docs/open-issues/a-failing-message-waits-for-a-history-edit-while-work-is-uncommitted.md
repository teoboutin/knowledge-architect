---
kind: design
---
# A message that fails `commits` while work is uncommitted waits for a history edit

## Summary

The primer has `commits` run after each commit, "where an amend still repairs a failing message".
This repository's root bars `commit --amend` while the tree holds uncommitted changes, so a message
that fails the check with other work in progress waits for a history edit of the whole branch once
the tree is clean. Three shapes would repair it at once, each changing a recorded decision.

## Details

### What

- `primer@checker-model`: "Run `cargo klarch commits <main>..HEAD` after each commit, where an
  amend still repairs a failing message."
- The root `CLAUDE.md`, `## Git` point 2, lists `commit --amend` among the operations that may not
  run while "uncommitted changes exist in the tree", per
  `design@knowledge-architect@no-operation-loses-content`.
- `commits --help` takes a range and no message file.

Instances: the session of quick fixes in this repository, 2026-11-08, C2 of its workflow file. Two
of its messages failed the check after they were committed, one for a bare slug of an entry the
parent tree holds, one for a path written where a register reference was due, with other work still
uncommitted; each waited for a history edit of the whole branch once the tree was clean.

The suspected mechanism, in one sentence: the checker's repair by an amend, assumed by
`design@check@one-command-judges-messages-after-the-commit`, is barred by a project's rule on
uncommitted work, and neither text is wrong alone.

The shapes:

- **An amend of the message alone**, at the root's rule: `git commit --amend --only -F <file>`,
  which git's manual says "can be used to amend the last commit without committing changes that
  have already been staged". It removes no content from the working tree, and the new commit keeps
  the replaced one's tree, so only the old message leaves. Admitting it changes the root's list of
  operations, a decision of this repository.
- **A draft check**: `commits` takes a draft message, such as `--message <file>`, judged against
  the staged tree with HEAD as its first parent, the file's proposal. It brings back
  `rejected@check@commit-msg-hook`, whose reason it must answer: a draft check "resolves a message
  against HEAD, which under an amend, a reword or a squash is the commit being replaced, so it
  refuses history edits the range check accepts". A first parent named explicitly may answer it;
  that is an assumption the discussion checks. It must also name the bytes it judges, since
  `design@check@message-judged-byte-for-byte` reads a message after git's own cleanup, which a draft
  has not had.
- **No change**: the message waits for the clean tree, at the cost of the history edit the root
  already describes.

The tree half of the same question, judging a commit before it exists, is
`issue@check@no-head-argues-that-the-checker-runs-from-a-pre-commit-hook`.

Re-entry point: a design discussion under `skill@knowledge-architect-design` when work next changes
what a session runs after a commit, or earlier on the owner's word.

### Why it matters

The premise of `design@check@one-command-judges-messages-after-the-commit`, that "a finding in the
newest commit's message is repaired by `git commit --amend`", does not hold in a project whose
rules bar the amend while work is uncommitted, as `design@knowledge-architect@no-operation-loses-content`
does here. Each such message costs a history edit of the whole branch, and stays false on the
branch until then, against `goal@documentation-stays-consistent`.

### What would close it

The discussion picks a shape, or rules that the history edit is the accepted cost; the shape built,
in the root's rule or in `commits` with a test over a scratch repository, on a branch of its own.
