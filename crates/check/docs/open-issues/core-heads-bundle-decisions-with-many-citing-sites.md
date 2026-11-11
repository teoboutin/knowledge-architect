---
kind: design
---
# `check` heads bundle decisions that lose to different rivals, and each has many citing sites

## Summary

Five heads, four of `register@check@design` and one of `register@model@design`, each hold two or more decisions whose nearest rivals
differ, against `design@agent-skills@one-decision-per-head`. The first run of the design-record axis on this repository found them, and a branch's review two more, and one more has since been split with every citing site re-pointed; each listed one was left unsplit because
a split must first read every site that cites the head, 7 to 33 sites per head, to re-point each
one to the part it cites.

## Details

### What

**The instances**, each with the decisions it bundles and the count of its citing sites. A count
is the number of lines `cargo klarch show design@check@<slug>` lists under "referenced at",
less the lines of this entry, taken again on the working tree of the branch of the first
agentic-workflow audit of this repository, at its review. The counts grow as text cites the heads:
the first run of the design-record axis counted 20, 23, 26 and 31 for the four heads it found.

- `design@check@every-path-names-its-anchor`, 25 citing sites, 8 of them in
  `spec@plans@path-quickfixes`. Its title is joined by "and" over two decisions, and its body holds
  a third:
  - the path grammar, `path@<anchor>@<path>`, against the rejected designs "A bare
    root-relative path form, with a targeted check on the collision cases", `rejected@check@bare-root-relative-path`, and "Paths without the kind prefix", `rejected@check@path-without-kind-prefix`;
  - the unanchored path lint and its scope, the paragraphs from "A span whose first segment names
    nothing here is another tool's notation, and is silent" through "Four shapes are outside the
    lint on purpose", against reporting every path-shaped span, with a census of its own;
  - the paragraph "The deepest anchor wins, and inside means a proper descendant", against naming
    a target from any ancestor anchor. Its argument, that relocating an anchor edits one manifest
    line, is not the grammar's.
- `design@check@a-commit-message-is-a-document`, 28 citing sites, 8 of them in
  `path@check@src/commands/history.rs`. Its title states the first of five decisions:
  - a message is judged against its own commit's tree, against the working tree;
  - "One command judges messages, after the commit exists", against the rejected alternatives "A
    `commit-msg` hook judging each message before the commit exists" and "A `commit` subcommand
    that runs the checks and then `git commit`", `rejected@check@commit-msg-hook` and `rejected@check@commit-subcommand`, both of which this head
    cites;
  - "Every commit of the range is judged by the tip checker, and a tree that fails is a finding",
    with "A branch that makes the checker stricter orders its commits for it", against skipping a
    commit whose tree fails and against building each commit's own checker;
  - "A message's references resolve against its commit's tree or its first parent's", against its
    own tree alone;
  - "`check` reads no range of history", against `check` judging the branch's messages.

  `issue@check@a-message-lint-is-deduplicated-in-one-tree-only` cites the head for the fourth
  decision.

`design@check@phases-gate-the-report`, found by the review of the branch that renamed the Component, holds beside its
phases "A writer refuses over an incomplete model", against writing over the model anyway, and "An extension prepares
its tree when the last phase is reached", against preparing before the gate, and `cargo klarch show` lists 26 sites citing it outside the plan documents and this entry. That branch split out the selection of checks, and left these.

The second pass of that run reworded a sentence of `design@check@a-commit-message-is-a-document`
into the present tense and left its bundle, so that head is a change's touched head left short of
`design@agent-skills@existing-heads-on-touch`. A head the run found bundled has since been split, with every site that cited it re-pointed.

Two more, found by the decision-record reviewer of the review before the merge of the branch
`pre-release-fixes`, which touched both and left both bundled; counts taken on that branch:

- `design@model@reserved-anchors`, 10 citing sites. Its title is joined by "and" over the escape
  anchor and the generic anchor `*`, whose arguments share no premise, and its body holds a third
  statement with a rival of its own: a file of another project written with that project's name
  as its first segment, against a repair that names the local anchor alone.
- `design@create@init-runs-git-init`, 7 citing sites. Its title states where `init` creates a
  repository; its body also holds the restriction of `--vcs` to `git` and `none`, against cargo's
  five values, and the rule that git is asked about PATH before the first write, each with a rival
  of its own.

**Suspected mechanism**: each head grew by appending a bold paragraph for a decision argued later
in the same subject, and a citing site then cited the head for whichever paragraph it needed.

**The re-entry point**: the next change that touches one of the five heads, in a branch other than
the one that opened this entry, which `design@agent-skills@existing-heads-on-touch` brings to the
rules on heads; or the next run of the design-record axis of
`skill@knowledge-architect-project-audit`.

Not established: whether every citing site cites one part alone. The sites were not all read.

### Why it matters

`skill@knowledge-architect-decision-recording@head-shape` holds that a head holds one decision. A session that reverses or extends one
of the bundled decisions cannot cite the decision it changes, since the slug names several, and
`cargo klarch show` on the slug lists the sites of every bundled decision at once, so the list of
texts to revisit is wider than the change. A `rejected` entry that the head cites cannot be
checked against the one decision whose argument covers it. This strains `design@agent-skills@one-decision-per-head`,
against `goal@design-is-recorded-with-its-arguments`.

### What would close it

Each of the splits applied: one head per decision, its text moved unchanged except the minimum
to read alone, and every site `cargo klarch show` lists for the head read and re-pointed to the head
of the part it cites, the sites outside `path@check@docs/` included. A head whose split the owner
rules out instead closes its part of this entry with that ruling, recorded in the closing commit.
