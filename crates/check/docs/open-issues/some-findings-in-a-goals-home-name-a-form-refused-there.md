---
kind: defect
---
# Some findings of the reference check in a goals home still name a reference form refused there

## Summary

`design@check@goals-home-cites-upward-only` says that every finding of the reference check that
would name a reference form in a goals home names the goals-home repair instead,
`GOALS_HOME_REPAIR` in `path@check@src/check/references.rs`. The code does so for five findings
only. Four other findings still name a form the citation check refuses in a goals home, a fifth
case treats every file of a location as its goals home when the goals home is declared at the
location's own path, and the id lint's plain-word repair asks to rename a goal.

## Details

### What

`judge_part` in `path@check@src/check/references.rs` computes `goals_of`, the anchor whose goals
home holds the document, with `goals_home_owner`. It passes `goals_of` to these five findings,
which then name `GOALS_HOME_REPAIR`:

- an id written outside its reference form, in `id_lint`, where no admitted form exists;
- an unanchored path;
- a retired slug form;
- a reference written without backticks, in `id_lint`, where its form is refused;
- an anchor in the kind position.

These findings of the same check ignore `goals_of`, and still name a refused form in a goals home:

1. **A markdown link.** `link` reports a link from a file that is no navigation home, such as the
   root's goals home in its single-file shape, `register@knowledge-architect@goal`, with the repair
   "in prose, point with a backticked `path@<anchor>@<path>` reference". A `path` reference is
   refused in a goals home.
2. **A malformed candidate.** `Candidate::Malformed` carries its repair from `candidate` and
   `harness_candidate` in `path@model@src/entity.rs`, and `judge_part` pushes it unchanged. The
   repair is `GENERIC_REPAIR`, "write `<kind>@<anchor>@<id>`, and for a path
   `path@<anchor>@<path>`", or, for an empty id under `register`, "write the register's name, as
   `register@<anchor>@design`", or, for a harness kind, the forms `skill@<name>`,
   `agent@<name>`, `primer@<slug>` or `instructions@<slug>`. Each names a form refused there,
   or a kind the goals home may hold with an anchor it may not.
3. **A section-shaped register.** `admitted_in_goals_home` admits any `register` reference whose
   anchor is admitted, so a reference to the `register` kind naming an item register of a plan,
   such as `thread`, at the home's own anchor gets no citation finding. `register` then reports
   that the register's home is a plan's own documents, with the repair "cite the plan document
   whole, as `spec@plans@<id>` or `milestone@plans@<id>`", both refused in a goals home.
4. **An item of a plan cited from outside it.** `table`, on `Resolution::OutsidePlan`, names the
   repair "cite the plan whole, as `{form}`", the form being a `spec` or `milestone` reference,
   refused in a goals home. The citation finding also fires on such a reference, so the document
   receives two repairs that disagree.

Two related cases:

5. **A goals home declared at its carrier's path.** For a register declared with `dir = "."`,
   `Anchor::home_of` in `path@model@src/entity.rs` gives the home's `file` and `dir` both as the
   anchor's own directory. `goals_home_owner` tests
   `rel == home.file || rel.starts_with(&home.dir)`, so every document the location owns is judged as part of its goals home, its issue entries
   included. Established by reading the code; not established by a run, and no mock project
   declares a `goal` register at `dir = "."`.
6. **The id lint's plain-word repair for a goal.** `id_finding` names, for an id written as a plain
   word that no design entry defines, "rename what it names so that its slug is not an ordinary
   word". In a goals home the admitted form is a goal, and a goal is renamed only through the
   goal-setting skill, which the repair does not say.

The test `no_lint_in_a_goals_home_names_a_form_the_citation_check_refuses` covers the five
findings listed first, and no other. Its name claims all of them.

Reproduction, in a copy of the mock project `path@check@tests/projects/dirhome/` with `git init`
and `git add -A` run in it: add to a subdocument of its goals home, under
`path@check@tests/projects/dirhome/docs/goals/`, a line holding three things, then run the checker
built from this repository over the copy:

- a markdown link to `README.md`;
- a backticked span of the `goal` kind with two segments, the project's name `dirhome` after the
  `@`;
- a backticked `register` reference whose anchor is `dirhome` and whose id is `thread`.

The spans are described rather than written here, since this entry is itself checked. Expected,
from the code read above: the link's finding names a `path` form, the two-segment span's names
`GENERIC_REPAIR`, and the register reference's names the `spec` and `milestone` forms. Not
established by a run; that `dirhome` carries the `thread` item register is an assumption.

Ruled out: the citation check itself, `goals_home_citation`, names `GOALS_HOME_REPAIR` on every
refused kind and anchor, as its unit tests show. The five listed findings name it, as
`no_lint_in_a_goals_home_names_a_form_the_citation_check_refuses` shows by its five findings.

### Why it matters

A writer in a goals home who follows one of these repairs writes a form the citation check then
refuses, or, for a goal, renames it outside goal-setting. The code is the defect against
`design@check@goals-home-cites-upward-only`, whose argument stands, per `primer@intent-and-claims`:
the design home prevails. It threatens `goal@knowledge-architect@the-owner-decides`, since a repair
that rewrites a goal sends an agent to change a goal on its own.

### What would close it

Each of the four findings above names `GOALS_HOME_REPAIR` in a goals home, or a form the citation
check admits; `goals_home_owner` judges a home declared at its carrier's path by the documents the
home holds, not by every file under the anchor; the id lint's repair for a goal names its
rewording through goal-setting; a unit test per case, each shown to discriminate with
`cargo x mutate`; and the head's sentence on which findings name the repair rewritten to the code.
