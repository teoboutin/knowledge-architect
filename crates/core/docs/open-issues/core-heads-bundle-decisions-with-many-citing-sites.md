---
kind: design
---
# Three core heads bundle decisions that lose to different rivals, and each has many citing sites

## Summary

Three heads of `register@core@design` each hold two or more decisions whose nearest rivals
differ, against `design@agent-skills@one-decision-per-head`. The first run of the design-record
axis on this repository found them, and left them unsplit because each split must first read every
site that cites the head, 25 to 33 sites per head, to re-point each one to the part it cites.

## Details

### What

**The instances**, each with the decisions it bundles and the count of its citing sites. A count
is the number of lines `cargo klarch show design@core@<slug>` lists under "referenced at",
less the lines of this entry, taken again on the working tree of the branch of the first
agentic-workflow audit of this repository, at its review. The counts grow as text cites the heads:
the first run of the design-record axis counted 20, 23, 26 and 31 for the four heads it found.

- `design@core@every-path-names-its-anchor`, 25 citing sites, 8 of them in
  `spec@plans@path-quickfixes`. Its title is joined by "and" over two decisions, and its body holds
  a third:
  - the path grammar, `path@<anchor>@<path>`, against the rejected designs "A bare
    root-relative path form, with a targeted check on the collision cases", `rejected@core@bare-root-relative-path`, and "Paths without the kind prefix", `rejected@core@path-without-kind-prefix`;
  - the unanchored path lint and its scope, the paragraphs from "A span whose first segment names
    nothing here is another tool's notation, and is silent" through "Four shapes are outside the
    lint on purpose", against reporting every path-shaped span, with a census of its own;
  - the paragraph "The deepest anchor wins, and inside means a proper descendant", against naming
    a target from any ancestor anchor. Its argument, that relocating an anchor edits one manifest
    line, is not the grammar's.
- `design@core@a-commit-message-is-a-document`, 28 citing sites, 8 of them in
  `path@core@src/cli/history.rs`. Its title states the first of five decisions:
  - a message is judged against its own commit's tree, against the working tree;
  - "One command judges messages, after the commit exists", against the rejected alternatives "A
    `commit-msg` hook judging each message before the commit exists" and "A `commit` subcommand
    that runs the checks and then `git commit`", `rejected@core@commit-msg-hook` and `rejected@core@commit-subcommand`, both of which this head
    cites;
  - "Every commit of the range is judged by the tip checker, and a tree that fails is a finding",
    with "A branch that makes the checker stricter orders its commits for it", against skipping a
    commit whose tree fails and against building each commit's own checker;
  - "A message's references resolve against its commit's tree or its first parent's", against its
    own tree alone;
  - "`check` reads no range of history", against `check` judging the branch's messages.

  `issue@core@a-contract-change-fails-every-earlier-commit-unexplained` cites the head for the
  third decision, and `issue@core@a-message-lint-is-deduplicated-in-one-tree-only` for the fourth.
- `design@core@owned-namespace-check`, 33 citing sites. Its title is joined by "and" over two
  decisions whose arguments share no premise:
  - the installed agent files are committed, so that a session started from a fresh clone has
    them, which serves `goal@knowledge-architect@agents-get-a-complete-workflow`; its rival is
    installing on demand;
  - the installed files are checked byte for byte in a namespace the installer owns, ownership
    decided by a name; its rivals are a record of installed files and a digest.

The second pass of that run reworded a sentence of `design@core@a-commit-message-is-a-document`
into the present tense and left its bundle, so that head is a change's touched head left short of
`design@agent-skills@existing-heads-on-touch`. A fourth head the run found bundled has since been
split, with every site that cited it re-pointed.

**Suspected mechanism**: each head grew by appending a bold paragraph for a decision argued later
in the same subject, and a citing site then cited the head for whichever paragraph it needed.

**The re-entry point**: the next change that touches one of the three heads, in a branch other than
the one that opened this entry, which `design@agent-skills@existing-heads-on-touch` brings to the
rules on heads; or the next run of the design-record axis of
`skill@knowledge-architect-project-audit`.

Not established: whether every citing site cites one part alone. The sites were not all read.

### Why it matters

`primer@design-heads` holds that a head holds one decision. A session that reverses or extends one
of the bundled decisions cannot cite the decision it changes, since the slug names several, and
`cargo klarch show` on the slug lists the sites of every bundled decision at once, so the list of
texts to revisit is wider than the change. A `rejected` entry that the head cites cannot be
checked against the one decision whose argument covers it. This strains `design@agent-skills@one-decision-per-head`,
against `goal@knowledge-architect@design-is-recorded-with-its-arguments`.

### What would close it

Each of the three splits applied: one head per decision, its text moved unchanged except the minimum
to read alone, and every site `cargo klarch show` lists for the head read and re-pointed to the head
of the part it cites, the sites outside `path@core@docs/` included. A head whose split the owner
rules out instead closes its part of this entry with that ruling, recorded in the closing commit.
