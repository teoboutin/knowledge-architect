---
kind: design
---
# No head argues the rule that every tool of knowledge-architect is a command of one program

## Summary

Four heads place a tool inside or outside `klarch` by one rule: every tool of knowledge-architect is a command of one
program, unless its result does not exist without running the project's build or its tests. No head states the rule.
No fact found defeats its nearest rival, one program per tool, and the exception has no argued reason.

## Details

### What

The statements nobody has argued, each with what it bound:

- "A project gets those tools by naming one package and pinning its version, whichever tools those are, a later one
  included." Its first half is held by `design@knowledge-architect@the-binary-crate-keeps-the-package-name`. Its
  second half bound every tool, a later one included, to be a command of that one program.
- "It is met while each other tool is a command of that program, and no command of it picks and runs the project's
  build or its tests, which a test of the program's commands shows of those it names." It bound each tool whose result
  exists without the project's build or tests to be a command of the program, and named a test. A search of
  `path@testing@tests/binary.rs` and of `path@cli@src/` for "project's tests", "the project's build" and "picks"
  finds no such test.
- "Requires, for instance: a new tool that searches the record is a command of that program; so is one that reads the
  agent harness's session transcripts." It bound the placement of two new tools.
  `design@knowledge-architect@transcript-crate` and `design@knowledge-architect@query-crate` state the placements
  built, and neither states the rule for a later tool.
- "Admits, for instance: a tool also offered as a library function; a library with no command of its own that the
  program links; a project's own command, in a program of its own; a task only this repository's own tooling performs,
  which is published to nobody; a command rewriting history that runs a command its caller names at each commit." It
  bound five limits of the rule.

The instances seen: `design@knowledge-architect@cli-crate`, `design@knowledge-architect@transcript-crate`,
`design@knowledge-architect@mutation-command-host` and `design@gates@gates-crate` each place a tool by the rule.

The suspected mechanism: the rule was held as a statement no argument had to defend, so no head gave it a rival and a
fact.

What an argument for it has to answer:

- The nearest rival is one program per tool, such as a binary of its own for the transcripts. A fact against it would
  be that a second program needs a second pin and a second substitution in every installed instruction that names it.
  The tree holds a counter-instance. The installed text's source holds two placeholders, `{{command}}` and
  `{{primer-import}}`, counted with `grep -rhoE '\{\{[a-z-]+\}\}' crates/agent-skills/content | sort | uniq -c`, and
  the development skill's source names a second program literally, `cargo x mutate run`, with no substitution.
  `design@agent-skills@xtask-pins-checker` pins the checker, the gates library and the mutation library by one
  declared version.
- The exception has no argued reason. Its rival is a `gates` command of `klarch`.
  `rejected@gates@gates-command-of-the-checker` states the consequence, "It would make `klarch` run cargo, the linter
  and the tests", and no reason why that is refused.

### Why it matters

A session that places a new tool finds four heads that apply the rule and none that argues it, so it can neither
weigh the rule against one program per tool nor tell whether a new case falls under the exception. It strains
`design@knowledge-architect@cli-crate`, whose crate exists to offer every command in one binary.

### What would close it

A design discussion under `skill@knowledge-architect-design`, raised when a new tool is placed or on the owner's word.
It argues the rule against one program per tool, and the exception against a `gates` command of `klarch`. Where no
fact ranks the shapes, the choice is a fork: the commit "[cli] The goals the owner ruled on what the command line is
for: one program for the tools, one home for what it accepts" records the owner's pick of the rule and the owner's
doubt on the exception's criterion. Its outcome is a head of `register@knowledge-architect@design` that the four
heads cite.
