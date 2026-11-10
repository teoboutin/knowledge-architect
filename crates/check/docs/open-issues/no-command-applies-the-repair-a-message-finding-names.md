---
kind: design
---
# No command applies the repair a finding on a commit message names

## Summary

`commits` reports each reference in a commit message written without backticks or without its kind,
and names its repair, a respelling with one candidate. Nothing applies it: sessions apply the
repairs by scripts of their own, then amend and run `commits` again, in a loop. `check --fix`
writes generated and installed files only, and `commits` takes no `--fix`.

## Details

### What

What the commands do today: `commits --help` takes a range and no option;
`path@check@docs/design/generated-files-and-fixes.md` states that "`commits` judges history and
takes no `--fix`"; `design@check@fix-scope` admits two kinds of fix, the generated files and the
installed agent files.

Instances seen, from retrospective files:

- a pass of the design-record axis in thaum, a project that installs the workflow, 2026-11-01:
  `commits` reported 111, then 35, then 4 such references in the pass's commit message, each naming
  its repair; a script applied them;
- the development-and-components milestone of this repository, 2026-11-02: 21 findings on one
  audit's message, applied by a script, then an amend;
- the revision of the rules on Components, 2026-11-03: 41 findings on a message and 49 on its
  rewording, applied in a loop.

Suspected mechanism, in one sentence: the repairs that are respellings follow from the finding, but
no fix reaches a message, since a message is history and `--fix` writes the working tree.

Three classes of finding differ, and only the first follows from the finding:

- a reference written without backticks, and a backticked bare id with one candidate: the span
  already names its target, the respelling `spec@plans@path-quickfixes` defines;
- a plain word naming an entry: `design@check@plain-word-lint` names two repairs, the reference or
  a rename where the word is meant as an ordinary term, so a fix would choose, which
  `design@check@safe-fix-definition` refuses;
- a reference to an item of a deleted plan document: the session chooses between the bare name and
  a harvested head, so a fix would choose too.

The nearest rival, from one of the files: `commits` writing the repaired message of a commit it
judged to a file, after the commit exists, for `git commit --amend -F`, "never amending by itself".
It edits no history, keeps the amend the session's, and does not judge a draft before the commit,
which `rejected@check@commit-subcommand`, a subcommand judging the tree and the message at the
moment of committing, records as lost. A `--fix` that rewrites a message file is the other shape a
file proposed. The same respellings in documents are the work of
`spec@plans@path-quickfixes`, on hold on the owner's word, whose instances
`issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids` holds.

### Why it matters

The respellings follow from the finding with no judgement, a task
`goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` gives the published checker, and each loop
of script, amend and run repeats per message. A repair mode for a message rewrites the sentence of
`design@check@check-fix-flag` that "`commits` judges history and takes no `--fix`".

### What would close it

The re-entry point: a discussion under `skill@knowledge-architect-design` that admits or refuses a
`commits` output of the repaired message of the newest commit it judged, the respellings alone,
rewriting the sentence of `design@check@check-fix-flag`; held at the design audit of
`spec@plans@path-quickfixes` when that spec restarts, or earlier on the owner's word. If admitted,
with a test through the binary over a mock project whose commit message holds one finding of each
class above, only the first repaired.
