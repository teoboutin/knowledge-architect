# The register `goal` is the root's alone

The second and last slice of `milestone@plans@goals-at-the-root`. It leaves when it lands. Its
threads, its arguments and its decided design are in the milestone document.

## Builds

- **`model`**: the register `goal` leaves the registers every Component carries, and is carried by
  the project's root alone. A goals home under a Component or a location is no home of a register.
- **`check`**: a finding on a file docs/goals.md, or a directory docs/goals/, of a Component other
  than the root, which names the repair: the goal moved to the root's goals home under the
  goal-setting skill, or turned into a design head. The rule on what a goals home cites, with one
  home left.
- **`create`**: `klarch init` and `klarch new-component` write no goals home for a Component;
  `home_text` in `path@create@src/command.rs` loses its `goal` arm for a Component.
- **`query`**: the goal context of `klarch worklist` is the root's goals alone.
- **The shipped text**: each of the sites that write the form `register@*@goal` names the root's
  goals home in a form that resolves in every installing project.
- **This repository**: each Component's empty docs/goals.md is deleted, in the first commit, with
  the change of the checker that refuses it, per `primer@branch-commits`.
- **The record**: the entries the milestone document's table gives slice 2;
  `design@check@every-path-names-its-anchor` split, per
  `issue@check@core-heads-bundle-decisions-with-many-citing-sites`; the changelog's entries, among
  them a Migration entry that names the whole-set review.

## Claims

- **A project with a goals home in a Component other than the root is refused, with a finding that
  names the file.** Test: a mock project of `path@testing@tests/` holding one. Shown to fail:
  `cargo x mutate` on the condition of the new check, the test then passing a tree it must refuse.
- **A project whose only goals home is the root's passes, the home being one file or a directory
  of files behind a README.** Test: two mock projects. Shown to fail: `cargo x mutate` on the
  branch that admits the directory form.
- **`klarch init` and `klarch new-component` write no goals home for a Component, and the tree
  they write passes the check.** Test: the tests of `create` over a scaffolded project. Shown to
  fail: `cargo x mutate` on the list of homes written.
- **A reference to a goal names the root, whatever theme file holds the goal.** Test: a mock
  project citing a goal of each of two files. Shown to fail: `cargo x mutate` on the resolution of
  a heading register's directory home.
- **The work list gives each agent the root's goals and no other.** Test: the tests of `query`
  over a mock project. Shown to fail: `cargo x mutate` on the gathering of the goal context.
- **Every form the shipped text writes for the goals home resolves in a project other than this
  one.** Test: the install over a mock project, then its check.

Each is shown with `cargo x mutate`, as `skill@knowledge-architect-development` asks.

## Fixtures

The mock projects of `path@testing@tests/` hold goals homes in their Components today. Each is
edited to hold its goals at its root, and one new mock project holds a goals home in a Component,
for the refusal. Both are in the vocabulary that exists.

## Audit subjects

- `path@model@src/manifest.rs`: `GOAL_REGISTER` and the list of the registers every Component
  carries.
- `path@create@src/command.rs`: `home_text`, and the list `design@create@init-command` derives.
- The worklist code of `path@query@src/`, and `design@query@goal-context-once`.
- `design@model@registers-are-declared`, whose reason is that the storage of the component
  registers "is what the word component means": the audit says how the head reads once one of
  them is the root's alone.
- The form the shipped text writes for the root's goals home. The discussion did not settle it.
  The anchor word `*` names every Component, and the root's anchor is each project's own name, so
  no form of today names the root in every project. This is a choice between shapes, which the
  audit puts to the owner: for instance `*` read as every anchor that carries the register, or a
  new reserved word for the root.
- Whether a location may still declare the register `goal`: the milestone document says none
  carries a goals home, and `design@check@goals-home-cites-upward-only` admits one today.
- The mock projects, and each test that counts goals homes.

## Fails alone on

- A tree that holds a goals home outside the root and passes, or the root's goals home as a
  directory refused.

## Premises that expire

None: this is the milestone's last slice.

## Implementation sequence

1. `model` and `check`: the register is the root's, and the refusal, with this repository's empty
   goals homes deleted and the mock projects moved, in the branch's first commit. Fails alone on
   the gates.
2. `create` and `query`. Fails alone on their tests.
3. The shipped text's form for the goals home, and the install. Fails alone on the install over a
   mock project.
4. The record, the split head and the changelog. Fails alone on `cargo klarch check`.
5. The harvest, and the report on the milestone's acceptance criterion as the milestone document
   leaves.

## Harvest

| what | where | when |
| --- | --- | --- |
| the register `goal` is the root's alone | `design@model@registers-are-declared` and `design@model@components-carry-the-same-documents`, rewritten in place | at this slice's landing |
| the refusal of a goals home outside the root, and what a goals home cites | `design@check@goals-home-cites-upward-only`, rewritten; `design@check@nothing-of-a-project-is-compiled-in` | at this slice's landing |
| what `init` and `new-component` write | `design@create@component-command` | at this slice's landing |
| the goal context of the work list | `design@query@goal-context-once` | at this slice's landing |
| the form the shipped text writes for the goals home | `design@agent-skills@shipped-text-cites-no-entry` | at this slice's landing |
