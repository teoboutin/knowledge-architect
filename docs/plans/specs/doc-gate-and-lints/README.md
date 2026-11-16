# A doc gate in the recommended list, the documentation lints in this repository, and a split signal the work does not need recorded as an issue

## Status and audience

This spec is the plan of one branch, named as the spec. It closes the issue
`the-documentation-lints-on-one-branch` and its two members, `no-gate-builds-the-documentation` and
`this-repository-warns-on-none-of-the-documentation-lints`, which the commit that adds this spec
deletes, per `skill@knowledge-architect-planning@spec-or-milestone`. The work does five things:

- the gates library's recommended list, `rust_project`, gains a `doc` gate that builds the
  workspace's documentation with rustdoc's warnings denied, so a broken intra-doc link fails a gate
  in every project that runs the list, this repository included;
- every published crate of this repository warns on `missing_docs`, `clippy::missing_errors_doc`
  and `clippy::missing_panics_doc`, and every warning is answered by the documentation it asks for;
- the two gates heads the doc gate is written into are brought to the rules on design heads first,
  one of them split in two;
- a new head of `register@agent-skills@design` states that a split signal over code a work does not
  write, which the work's discussion weighs and finds the work does not need, becomes an issue of
  kind `design` whose re-entry point is the next design or planning of work that changes that code;
  `design@agent-skills@splits-planned-before-work` and the installed text follow it;
- each file the documentation pass writes into that a session cannot read whole gets such an issue.

It is written for a session that did not witness the design discussion that produced it. It leaves
the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. D1 and D2, the two defaults the owner was asked to rule on, are ruled, and so is Q3, put at
  the review before the merge.

The discussion spans one session, which begins at the owner's message "I'd like to solve
docs/open-issues/the-documentation-lints-on-one-branch.md and the two issues it bundles." Its rounds,
named by the opening words of each of the owner's messages:

| round | the owner's message opens with |
| --- | --- |
| 1 | "I'd like to solve docs/open-issues/the-documentation-lints-on-one-branch.md" |
| 2 | "start the design discussion" |
| 3 | "nothing to contest, I agree with your proposals" |
| 4 | "Q1 a is right." |
| 5 | "D1 D2 agreed, proceed with the work" |
| 6 | "q3 default" |

The threads and the arguments are in [the deliberation](deliberation.md). The generated lines of
the discussion's two record searches are in [the discussion's search](search/discussion.txt).

## How the work is done

As `skill@knowledge-architect-planning@working-a-slice` says for the work of a spec, read whole
before the work starts.

## Names

| name | what it is |
| --- | --- |
| the recommended list | the gate list `rust_project` returns, in `path@gates@src/gates.rs` |
| `Gate` | the struct of one gate in `path@gates@src/gates.rs`, with the fields `name`, `program`, `args`, `envs`, `resolves`, `rebased_only` and `distiller` |
| `Distiller::WithoutProgress` | the distiller that drops cargo's build-progress lines, which the `clippy` gate uses; the lines it drops are those `is_progress` in `path@gates@src/gates.rs` names |
| the gates | `cargo x gates`, this repository's run of the recommended list, `path@xtask@src/gates.rs` |
| the doc gate | the gate this work adds, named `doc` |
| the three lints | `missing_docs` in `[lints.rust]`, and `missing_errors_doc` and `missing_panics_doc` in `[lints.clippy]`, each at `"warn"` |
| the lint table | `[workspace.lints.rust]` and `[workspace.lints.clippy]` in the root `path@knowledge-architect@Cargo.toml`, which every crate takes with `[lints] workspace = true` |
| the documentation pass | step 2: answering every warning the three lints give |
| past one read | a file a session cannot read whole in one pass, the size signal of `design@agent-skills@module-split-guidelines`: a Read with no range returns it cut short. It is tested by that Read, never by a count of lines |
| a split signal | a signal of the module guidelines, `path@agent-skills@fragments/module-split.md`, for a split of a module |
| the hand mutation | `cargo x mutate run --file <F> --replace <OLD> --with <NEW> -- <command>`, per `path@mutate@README.md` |
| the line-joining search | `perl -0ne 'print "$ARGV\n" if /never\s+an\s+issue|parked\s+as\s+an\s+issue/' <files>`, which finds a phrase wrapped across a line break; a plain `grep` misses both target sentences, which wrap |

## What the work is

**What exists today**, at each site the work touches:

- The recommended list is `rebased`, `fmt`, `check`, `commits`, `clippy`, `test`. Three tests pin
  its names: `the_rust_list_is_in_cost_order_and_holds_the_range_check` in `path@gates@src/gates.rs`,
  `this_repository_runs_the_recommended_list_over_its_own_checker` in `path@xtask@src/gates.rs`, and
  the two loops of `ci_reaches_every_gate` in `path@xtask@tests/gates_bin.rs`, beside
  `locked_reaches_every_gate_that_resolves_dependencies` there, which checks `--locked` per gate.
- Six texts restate the list, each false once `doc` lands:
  - `path@agent-skills@content/skills/setup/SKILL.md`, the paragraph that opens "**The gates.**
    `rust_project` gives, in cost order";
  - the root `path@knowledge-architect@CLAUDE.md`, `instructions@verify-mechanically`: "It runs five
    gates in cost order", the list after it, and "adds a sixth gate";
  - `path@xtask@README.md`: "run the five gates" in its usage block, and its table "The gates, in
    cost order";
  - the root `path@knowledge-architect@README.md`: "`cargo x gates` runs every gate: formatting, the
    check, the commit messages, clippy and the tests";
  - the doc comment of `rust_project` in `path@gates@src/gates.rs`: "in cost order: the rebase check
    behind its flag, formatting, the document check, the commit messages of the branch, clippy, and
    the tests";
  - `design@gates@a-project-holds-its-gate-list` and `design@gates@distill-over-full-logs`, which
    the harvest rewrites.
- `is_progress` in `path@gates@src/gates.rs` lists cargo's progress verbs and holds neither
  `Documenting ` nor `Generated `: a failing `cargo doc --workspace` keeps one `Documenting` line per
  crate in a `WithoutProgress` extract, measured on a scratch crate.
- The doc comment of `Gate::envs` in `path@gates@src/gates.rs`, and that of `Spec` in
  `path@gates@src/process.rs`, say the variables are "added to the inherited environment, never
  replacing it"; a variable of the same name as an inherited one replaces its value, which the doc
  gate relies on.
- No gate runs `cargo doc`. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` exits 0 on
  the tree this spec is written on.
- The lint table holds `unsafe_code = "forbid"` alone.
- With the three lints at warn, `cargo clippy --workspace --all-targets` gives 279 distinct warnings
  over the published crates, distinct by file, line, column and message: four locations carry two
  lints each. 206 `missing_docs`, 69 `# Errors`, 4 `# Panics`. Per crate: `model` 177 / 17 / 2,
  `check` 12 / 13 / 0, `cli` 11 / 3 / 0, `mutate` 6 / 6 / 0, `git` 0 / 17 / 0, `query` 0 / 6 / 1,
  `create` 0 / 3 / 0, `transcript` 0 / 3 / 0, `gates` 0 / 1 / 1, `agent-skills` and `testing` 0.
  `xtask` has 0. In `path@model@src/resolver.rs`, 114 are the 75 fields and 39 variants of
  `enum Cause`, and 8 are `# Errors`. No test or binary target lacks crate documentation. Re-take it
  with
  `RUSTFLAGS="-W missing_docs" cargo clippy -p <crate> --all-targets --message-format=short -- -W clippy::missing_errors_doc -W clippy::missing_panics_doc`
  per crate, then deduplicate the warning lines by file, line, column and message.
- Files the documentation pass writes into: `path@model@src/resolver.rs`,
  `path@model@src/entity.rs` and `path@model@src/manifest.rs` are past one read: a Read with no
  range returns each cut short, at a cap of 25000 tokens, as the issue step 5 writes for each
  records. `path@git@src/git.rs` (no `# Errors` section today, 17 owed) is past one read too: a Read
  with no range returned lines 1 to 1293 and named a cap of 25000 tokens, measured by the
  code-claims review of this spec.
  `path@check@src/commands/history.rs`, `path@model@src/scan.rs` and
  `path@check@src/commands/mod.rs` are not measured.
- `design@agent-skills@splits-planned-before-work` says of a split a work bears on: "Neither is left
  to the implementer, nor parked as an issue." `path@agent-skills@content/skills/planning/SKILL.md`
  says "never left to the implementer, and never an issue";
  `path@agent-skills@content/skills/design/SKILL.md`, loop step 1, has the discussion write "the
  split's shape, or that none is made", and says nothing of the signal when none is made.
  `path@agent-skills@content/skills/project-audit/SKILL.md` sends "a split or a move the owner
  approved that is too large for the branch" to an issue of kind `design`.
  `issue@testing@binary-rs-is-past-one-read` and
  `issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read` are split signals recorded as
  `todo` issues.

**Outside the work:**

- What the added documentation says, beyond its presence: the code-claims axis judges it, per
  `design@agent-skills@code-claims-scope`.
- Whether a public item should stay public: each crate's facade decision, per
  `design@check@the-facade-publishes-what-a-consumer-needs`; the documentation pass narrows no item.
- What becomes an ask of every target project, the rule and its statements:
  `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`, whose re-entry point this work
  meets, put off on the owner's word, per `thread@doc-gate-and-lints@doc-gate-ask-status`.
- The split of the files past one read: the `design` issues step 5 opens, per
  `thread@doc-gate-and-lints@doc-pass-split`.
- A split of a subject of a design home: unchanged, per `design@agent-skills@subject-split-standard`.
- The stale quotation in `issue@git@rev-parse-answers-none-for-every-git-failure`, met outside the
  task: a commit of its own on this branch.

## What is already decided

**The decisions the design rests on and does not argue again:**

- `design@gates@distillers-are-a-closed-set`: the doc gate uses an existing distiller, whose list of
  progress verbs the work extends.
- `design@gates@the-library-owns-the-flags`: `--locked` and `--ci` reach the doc gate as they reach
  every gate.
- `design@xtask@gates-list-primary-home`: this repository runs the recommended list unchanged, and
  CI runs the same tool, so `ci.yml` does not change.
- `design@agent-skills@gates-convention`: an adopting project runs the library's gates.
- `design@agent-skills@documentation-presence-is-linted`: the setup asks every published crate of a
  target project for the three lints; this repository follows the ask.
- `design@agent-skills@module-split-guidelines`: size is a signal only as a file a session cannot
  read whole.
- `design@agent-skills@code-split-scout`: the scout reports split signals at the grounding.
- `design@agent-skills@code-shape-axis` and `design@agent-skills@default-code-shape`: a split signal
  over code the range writes is a departure the review weighs and the writer restructures or argues
  at the code; the new split head covers code the work does not write.
- `design@agent-skills@met-outside-the-task-outcomes`: a split signal met outside the work's area
  takes the first row of `primer@met-outside-the-task` that applies; the new split head gives the
  shape of the issue where that row is row 3.
- `design@model@resolver-outcome` and `design@model@cause-words-in-the-resolver`: a refusal's variant
  of `enum Cause`, its arm and its words are read together.
- `design@agent-skills@completeness-check-is-a-step-claim`: the hand mutation that breaks a link is
  a claim of step 1, not an acceptance criterion.
- `goal@generic-tools-and-procedures-are-shipped`: a generic tool reaches each target project that
  carries out its activity once knowledge-architect's owner takes it in; the owner took the doc gate
  in, round 2.
- `goal@adoption-is-easy`: the crate documentation docs.rs renders reaches its reader.

**The decision the work rewrites**: `design@agent-skills@splits-planned-before-work`, per
`thread@doc-gate-and-lints@split-not-needed-is-an-issue` and D1. Its title stays; its body loses
"nor parked as an issue" and points to the new head. Every text that cites it, as
`cargo klarch show design@agent-skills@splits-planned-before-work` lists them, is read at step 4 and
repaired where it states what the rewrite changes:

- `path@agent-skills@content/agents/code-shape-reviewer.md`, its `%%` comment and the paragraph
  that routes a split signal to the dispatching session;
- `path@agent-skills@content/skills/design/SKILL.md`, its `%%` comment and loop step 1's sentence on
  a signal for which none is made;
- `path@agent-skills@content/skills/development/SKILL.md`, its `%%` comment and the paragraph that
  routes a signal over code the change does not write "usually as an issue";
- `path@agent-skills@content/skills/planning/SKILL.md`, its `%%` comment and the sentence "never
  left to the implementer, and never an issue";
- `path@agent-skills@fragments/module-split.md`, its `%%` comment;
- the heads of `register@agent-skills@design` that cite it: four in
  `path@agent-skills@docs/design/development.md` (`design@agent-skills@default-code-shape`,
  `design@agent-skills@module-split-guidelines`, `design@agent-skills@record-split-found-by-a-command`
  and `design@agent-skills@code-split-scout`), and one each in
  `path@agent-skills@docs/design/components.md`, `path@agent-skills@docs/design/record-search.md`,
  `path@agent-skills@docs/design/reviews.md` (`design@agent-skills@code-shape-axis`) and
  `path@agent-skills@docs/design/what-earns-a-record.md`.

The line-joining search over `path@agent-skills@content/`, `path@agent-skills@fragments/` and
`path@agent-skills@docs/` finds the planning skill's sentence, the head's, and an unrelated use in
`path@agent-skills@docs/design/how-installed-text-changes.md`; step 4 runs it again.

**The heads the harvest touches without reversing or rewriting their decision**, each brought to the
rules by step 3 first, as `agent@knowledge-architect-design-record-auditor` found them non-conformant
when briefed on them at the commit this spec was written on:

- `design@gates@a-project-holds-its-gate-list`: its title joins two decisions with two rivals,
  "a project holds its gate list" against a list the library fixes, and "the library offers a
  recommended one" against no recommended list, and nothing argues the second. Step 3 splits it:
  the head keeps its slug for the first, and a new head, the head of `thread@doc-gate-and-lints@recommended-gate-list`, takes the second,
  argued from `goal@generic-tools-and-procedures-are-shipped`, with the list's cost order and the
  distiller of each recommended gate. Five texts cite only the recommended list and are re-pointed:
  `design@agent-skills@the-workflow-teaches-no-rust-idiom` and
  `design@agent-skills@diff-mutation-sweep` in `path@agent-skills@docs/design/development.md`, the
  head of `path@check@docs/design/prose.md` that cites it, the head of `register@create@design`
  that cites it, and `design@xtask@gates-list-primary-home`. The three that cite a project's own
  list keep it: `design@agent-skills@gates-convention`, `design@knowledge-architect@gates-crate`, and
  `design@gates@the-library-owns-the-flags`.
- `design@gates@distill-over-full-logs`: its list of each recommended gate's distiller answers the
  recommended list's question, and moves to the new head, where the rebase check's
  `Distiller::Ancestry`, which the list omits, is added; it gains the fact that defeats its nearest
  rival, a failed gate's full output on stdout: a test run's log holds one line per passing test,
  which `grep -c ' \.\.\. ok$' target/gates/test.log` counts, 1254 of 2226 lines on the owner's
  workstation after a passing run.
- `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`: its "Why it matters" names
  the new split head as a decision its answer strains, since a review that read every re-entry point
  would bring a split issue back to review time.

The record side of a split: `cargo klarch worklist --out <dir>` over this spec printed no notice on a
subject the harvest writes into, run by the decision-record review of this spec.

**The verdicts of the discussion's searches whose outcome changed something:**

| entry | verdict | outcome |
| --- | --- | --- |
| `design@gates@a-project-holds-its-gate-list`, `design@gates@distill-over-full-logs` | conflicts | each lists the recommended gates by name; step 3 moves the list to the new head the head of `thread@doc-gate-and-lints@recommended-gate-list`, and the harvest adds the doc gate there, "the documentation build", kept apart from "the document check", which is `check` |
| `design@check@the-facade-publishes-what-a-consumer-needs`, `design@git@an-operation-in-git-s-terms-is-the-library-s` | conflicts | the narrowing exception of `thread@doc-gate-and-lints@lint-warning-answer` dropped: an item is public because a consumer needs it, extensions outside the workspace included |
| `design@agent-skills@splits-planned-before-work` | conflicts | the head rewritten and a new head added, per D1; the head's second paragraph, which routes a signal met outside the work's area by `primer@met-outside-the-task`, is kept, and the new head gives the issue's shape where that routing opens one |
| `design@agent-skills@code-shape-axis` | conflicts | the new head covers only code the work does not write, a doc comment added to a module not counting as writing its code; a signal over code the range writes stays a departure the review weighs; the code-shape reviewer's paragraph is repaired at step 4 to say both |
| `design@agent-skills@undesigned-work-is-an-issue` | constrains | answered by D2: a split signal that no discussion has weighed for a work that changes the code is an open design question, whether and where to split, not known undesigned work, as the project-audit skill already records a split too large for its branch as a `design` issue |
| `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` | trigger-met | the re-entry point is met; the owner put the discussion off, round 4, and the entry gains the doc gate as an instance at step 5 |
| `design@agent-skills@subject-split-standard` | constrains | the new head covers splits of code alone |
| the issue kinds of `skill@knowledge-architect-issue-tracking@issue-entry` | read by the searchers | the split issue is of kind `design`, the one kind with a re-entry point; a `todo` has none, so the search grades it `touches` at most, and a `deferred` trigger is read by the standing-state reviewer at every merge, which would bring the split back to review time; per D2 |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` | touches | the record search reads a `design` issue's re-entry point before designed or planned work, which is where the new head wants it read; the new head cites the issue as a premise, and the issue names the head |
| the three issues the spec closes | closes | deleted by the commit that adds this spec |

## Criteria

### A link that names nothing, in the documentation docs.rs renders, fails a gate `##broken-link-fails-a-gate`

Binding, from `goal@adoption-is-easy`: the crate documentation is the home of a library's API
description, per the root `CLAUDE.md`'s table of where knowledge goes, and a link that names nothing
there is found by a reader, not by a gate. Met by `thread@doc-gate-and-lints@doc-gate-home` and
`thread@doc-gate-and-lints@doc-gate-denial`.

### A project writes no gate logic beyond its list, and the recommended list runs in cost order `##no-gate-logic-beyond-the-list`

Binding, from `design@gates@a-project-holds-its-gate-list`. Met by
`thread@doc-gate-and-lints@doc-gate-home`: the gate is a member of the list, built by the library;
the one measurement of cost cannot rank `doc` against `clippy`, so the order between them is decided
by another reason, given in "Decided design, doc gate home".

### Every published crate declares the three lints, and the clippy gate passes `##published-crates-declare-the-lints`

Binding, from `design@agent-skills@documentation-presence-is-linted`. This repository is held to an
ask it ships: a target project that does not yet follow an ask brings itself into conformance over
time, per `goal@serves-one-kind-of-project`, and meanwhile this repository's public items can lack
the documentation a consumer of its libraries needs, and the code-claims axis reads documents that
may not exist. Met by `thread@doc-gate-and-lints@doc-lints-table` and
`thread@doc-gate-and-lints@lint-warning-answer`.

### A tool reaches every target project only once knowledge-architect's owner takes it in `##taken-in-by-the-owner`

Binding, from `goal@generic-tools-and-procedures-are-shipped`. Met: the owner took the doc gate in,
round 2, per `thread@doc-gate-and-lints@doc-gate-home`.

### What an adopting project must fix when it moves its pin `##pin-move-cost`

Weighed. Met by `thread@doc-gate-and-lints@doc-gate-denial` and
`thread@doc-gate-and-lints@doc-gate-item-scope`: a project whose public documentation warns fails
the new gate at its move, which the changelog's Migration entry names; private items are not
documented by the gate.

### How long a gates run takes `##gate-run-time`

Weighed. Met by `thread@doc-gate-and-lints@doc-gate-home`: a cold doc build of this workspace took
3.72 s and a cold clippy 4.40 s, one run each, on the owner's workstation with 32 cores and cargo
1.98.0, each in a target directory of its own, re-taken by the code-claims review at 3.91 s and
4.46 s. The commands: `CARGO_TARGET_DIR=<new dir> RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps -q`
and `CARGO_TARGET_DIR=<another new dir> cargo clippy --workspace --all-targets -q -- -D warnings`,
each under `/usr/bin/time`.

### A split is weighed where a work is designed or planned, never where it is implemented or reviewed `##split-weighed-at-planning`

Binding as a presumption, derived from `design@agent-skills@splits-planned-before-work`, which has
a split found where the work is designed or planned and never left to its implementer. Its review
half rests on the owner's report of round 4, a premise: split signals weighed at the review before
the merge added unplanned work at an inopportune time. Met by
`thread@doc-gate-and-lints@split-not-needed-is-an-issue`.

## New names, in one place

```text
doc                                   the gate's name, in rust_project            crates/gates/src/gates.rs
RUSTDOCFLAGS="-D warnings"            the doc gate's one variable, in its envs    crates/gates/src/gates.rs
"Documenting ", "Generated "          two progress verbs added to is_progress     crates/gates/src/gates.rs
missing_docs = "warn"                 [workspace.lints.rust]                      Cargo.toml
missing_errors_doc = "warn"           [workspace.lints.clippy]                    Cargo.toml
missing_panics_doc = "warn"           [workspace.lints.clippy]                    Cargo.toml
the head of `thread@doc-gate-and-lints@recommended-gate-list`, split from `design@gates@a-project-holds-its-gate-list`   crates/gates/docs/design.md
the head of `thread@doc-gate-and-lints@split-not-needed-is-an-issue`, its slug the thread's   crates/agent-skills/docs/design/development.md
```

`Gate`, `Distiller::WithoutProgress`, `is_progress`, `rust_project` and `gate_envs` exist. The slugs
of the heads the harvest writes for the denial, the item scope and the lint table are the harvest's,
as `skill@knowledge-architect-decision-recording@head-shape` gives them.

## Decided design

### doc gate home

The recommended list gains a gate named `doc`, after `clippy` and before `test`. An illustration
of the shape, untested:

```rust
Gate {
    envs: vec![("RUSTDOCFLAGS".into(), "-D warnings".into())],
    ..gate("doc", "cargo", owned(&["doc", "--workspace", "--no-deps"]), true,
           Distiller::WithoutProgress)
}
```

- It resolves dependencies, so `--locked` reaches it as it reaches `clippy`.
- It goes after `clippy`: their cold costs, 3.72 s and 4.40 s, are the same within what one run
  shows, and clippy's extract is the fuller report on a missing doc, which both gates report; the
  reason is a comment at the gate.
- `is_progress` gains `Documenting ` and `Generated `, so the doc gate's extract holds rustdoc's
  diagnostics and not one line per crate documented.
- The doc comments of `Gate::envs` and of `Spec` say that a variable of the same name as an
  inherited one replaces its value.
- `path@xtask@src/gates.rs` keeps calling `rust_project` unchanged.
- The six texts of "What the work is" that restate the list are repaired: the setup skill, the
  `path@xtask@README.md` usage line and table, the root `path@knowledge-architect@README.md`, the
  doc comment of `rust_project`, and the root `CLAUDE.md`, whose restatement of
  `design@xtask@gates-list-primary-home` is longer than one sentence and is converted, as
  `primer@where-knowledge-goes` asks of one a change edits: a pointer to that head, with at most one
  sentence restated. The two gates heads take it at step 6.
- The changelog says that every project's list gains the gate, with a Migration line: a project
  whose public documentation gives a rustdoc warning fails the gate at its pin move, and fixes the
  warning or drops the gate from its own list.

The nearest rival, the gate in this repository's list alone, is out on the owner's word, round 2.

### doc gate denial

The doc gate sets `RUSTDOCFLAGS="-D warnings"` in its `envs`, so every rustdoc warning fails it,
with nothing the project must declare. Measured on a scratch crate, cargo and rustdoc 1.98.0:

| how the deny is set | broken link | exit |
| --- | --- | --- |
| none, `cargo doc --no-deps` | warning | 0 |
| `RUSTDOCFLAGS="-D warnings"` | error | 101 |
| `[lints.rustdoc] broken_intra_doc_links = "deny"` | error | 101 |
| `build.rustdocflags = ["-D", "warnings"]` in the crate's cargo configuration file | error | 101 |
| that configuration, with `RUSTDOCFLAGS="--cfg foo"` exported | warning | 0 |
| `cargo test --doc`, with or without `RUSTDOCFLAGS="-D warnings"` | not reported | 0 |

Re-taken by this script, run in a new empty directory; each line after the crate's creation prints
the exit code of one row:

```sh
cargo new -q --lib probe && cd probe
printf '//! Crate doc linking [`Nowhere`].\n' > src/lib.rs
cargo doc --no-deps -q; echo "row 1: $?"
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -q; echo "row 2: $?"
printf '[lints.rustdoc]\nbroken_intra_doc_links = "deny"\n' >> Cargo.toml
cargo doc --no-deps -q; echo "row 3: $?"
sed -i '/lints.rustdoc/,$d' Cargo.toml
mkdir -p .cargo && printf '[build]\nrustdocflags = ["-D", "warnings"]\n' > .cargo/config.toml
cargo doc --no-deps -q; echo "row 4: $?"
RUSTDOCFLAGS="--cfg foo" cargo doc --no-deps -q; echo "row 5: $?"
rm -r .cargo; RUSTDOCFLAGS="-D warnings" cargo test --doc -q; echo "row 6: $?"
```

Accepted costs:

- in the gate's child, the variable replaces a `RUSTDOCFLAGS` the caller exported and a
  `build.rustdocflags` the project configured; a project whose documentation needs a flag adds it to
  the doc gate's `envs` in its own list;
- alternating a plain `cargo doc` with the gate re-documents the workspace's crates: 0.77 s here;
- rustdoc also reports `missing_docs`, so one missing doc fails both `clippy` and `doc`.

The nearest rival, a plain `cargo doc` with an ask for `[lints.rustdoc]` levels, is defeated by
row 1: a project that has not yet followed the ask gets a gate that passes silently. The `test`
gate cannot carry the check either: row 6.

### doc gate item scope

The gate documents public items alone, the text docs.rs renders: no `--document-private-items`.
With private items, this tree gives 2 more warnings, both ambiguous links in `mutate`, counted with
`cargo doc --workspace --no-deps --document-private-items --message-format=short` against the same
command without the flag. No goal ranks the two scopes; the session chose public items, and the
owner approved it, round 3. It is a decision of its own, apart from the denial.

### doc lints table

The three lints go in the lint table, beside `unsafe_code`, so every crate takes them, `xtask`
included, which has no warning under them. One table per published crate is refused by cargo,
which rejects `lints.workspace = true` beside a crate's own lint keys, so each crate would repeat
`unsafe_code`, in 11 copies. Re-taken by a workspace whose member declares `[lints] workspace = true`
and `[lints.rust] missing_docs = "warn"`: `cargo check` fails to load the member's manifest. The
table's comment cites `design@agent-skills@documentation-presence-is-linted` beside
`design@agent-skills@no-security-axis`.

### lint warning answer

Every warning is answered by the documentation it asks for: a `///` line on each item, a `# Errors`
section saying when the function returns an error, a `# Panics` section saying when it panics. No
item is narrowed to avoid a warning.

### doc pass split

The documentation pass splits no module. Each file it writes into that is past one read gets an
issue of the shape of "Decided design, split not needed is an issue", opened at step 5:
`path@model@src/resolver.rs`, `path@model@src/entity.rs`, `path@model@src/manifest.rs` and
`path@git@src/git.rs`, measured; every other file the pass writes into, among them
`path@check@src/commands/history.rs`, `path@model@src/scan.rs` and
`path@check@src/commands/mod.rs`, is tested by a Read with no range once step 2 has landed.
`path@query@src/worklist.rs` has its issue already. The pass changes no code of these files but its
documentation; in `resolver.rs` the growth falls inside `enum Cause`, one item no move of code
divides.

The scout's signal on `path@gates@src/gates.rs`, two parts that change apart, held weakly and was
outweighed by three signals against a split that the scout named: the list names the module's own
`Distiller` variants, two tests build their fixtures from `rust_project`, and the doc gate's `envs`
is read by `gate_envs` in the same file. No split stands there, so it owes no issue.

The owner ruled for an issue per file and no split, round 4, shape (a) of Q1. The kind is D2.

### doc gate ask status

The doc gate takes the status every gate of the recommended list holds: a project edits the list it
receives. The work adds no kind of ask. `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`
stays open, and its section "What" gains the doc gate as an instance of a quality tool taken in. The
owner put that discussion off, round 4, as larger than this discussion.

### recommended gate list

`design@gates@a-project-holds-its-gate-list` is split, as "What is already decided" says. The head of
`thread@doc-gate-and-lints@recommended-gate-list` states that the library offers the recommended
gate list of a Rust project that uses the checker, in cost order, with each gate's distiller. Its argument: the generic
part of a gate reaches every target project once knowledge-architect's owner takes it in, per
`goal@generic-tools-and-procedures-are-shipped`, and what is particular to one project stays in it;
the nearest rival, no recommended list, leaves each project to write its own, which ships nothing.
A gate enters the list on the owner's word, as the doc gate did, round 2.

### split not needed is an issue

Two heads of `register@agent-skills@design`, per D1:

- `design@agent-skills@splits-planned-before-work` keeps its title and its decision, a split the
  work needs is found where the work is designed or planned and is a step before the work, never
  left to its implementer, and its second paragraph, which routes a signal met outside the work's
  area by `primer@met-outside-the-task`. Its body loses "nor parked as an issue", and points to the
  new head for a signal the work does not need.
- A new head, the head of `thread@doc-gate-and-lints@split-not-needed-is-an-issue`, whose title states its rule: a split signal over code
  a work does not write, weighed at the work's design or planning and found not needed by that work,
  is an issue of kind `design` whose re-entry point is the next design or planning of work that
  changes that code. Its statements:
  - a doc comment added to a module does not count as writing its code, for this rule;
  - a signal the discussion weighs and finds outweighed by the signals against a split, as for
    `path@gates@src/gates.rs` here, makes no split stand and owes no issue;
  - a signal met where the work is implemented or reviewed is routed by `primer@met-outside-the-task`,
    as the incumbent's second paragraph says; where that routing opens an issue, it takes this shape,
    and the signal is not weighed or carried out in that work;
  - a signal over code the range writes is a departure, per `design@agent-skills@code-shape-axis`,
    outside this rule;
  - a split of a subject of a design home is outside it, per `design@agent-skills@subject-split-standard`.

  Its argument: the owner reported, round 4, that before the incumbent head, split signals were
  found and weighed at the review before the merge, which added unplanned work at an inopportune
  time, a premise attributed as the owner's report; so the rule's purpose is when a split is
  weighed, and an issue that comes back where a work is next designed or planned keeps it there. The
  owner's earlier report, that an issue on a split sits untouched, is answered by the re-entry
  point: the record search reads a `design` issue whole at the grounding of the next designed or
  planned work, and grades a met re-entry point `trigger-met`, with the action `rule-before-work`.
  Its premise: no review reads a `design` issue's re-entry point, the question
  `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` holds open; a review that read
  every re-entry point would bring the split back to review time, and the head would be argued again.

The installed text follows: the planning skill's sentence drops "and never an issue" and says what
becomes of a signal the work does not need; the design skill's loop step 1 says the same of a signal
for which none is made; the development skill's paragraph and the code-shape reviewer's say that a
signal over code the change does not write, a doc comment not counting as writing it, takes that
shape when its routing opens an issue. `issue@testing@binary-rs-is-past-one-read` and
`issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read` become `design` issues of that
shape, each keeping its instances and its closing condition, gaining a mechanism sentence and the
re-entry point, and testing size by a Read rather than by its line count.

## Mapping tables

The work needs none.

## Losing alternatives

| alternative | lost to | the deciding fact |
| --- | --- | --- |
| the doc gate in `path@xtask@src/gates.rs` alone | `thread@doc-gate-and-lints@doc-gate-home` | the owner took the gate into the recommended list, round 2 |
| the check carried by the `test` gate, the "equivalent" the closed issue named | `thread@doc-gate-and-lints@doc-gate-home` | `cargo test --doc` reports no unresolved link and exits 0, with or without `RUSTDOCFLAGS="-D warnings"`, measured: row 6 of the denial table |
| a plain `cargo doc` gate with an ask for `[lints.rustdoc]` levels | `thread@doc-gate-and-lints@doc-gate-denial` | a plain `cargo doc` exits 0 on an unresolved link, so a project that has not followed the ask gets a gate that passes silently |
| the deny passed through `build.rustdocflags`, in a configuration file or with `--config` | `thread@doc-gate-and-lints@doc-gate-denial` | an exported `RUSTDOCFLAGS` overrides it: measured for the file, row 5, and for `--config` by the code-claims review of this spec; the deny is lost silently |
| `-A missing_docs` in the doc gate | `thread@doc-gate-and-lints@doc-gate-denial` | gate logic only to avoid a duplicate report |
| `--document-private-items` | `thread@doc-gate-and-lints@doc-gate-item-scope` | no goal ranks the scopes; the session's choice of public items, which the owner approved |
| one lint table per published crate | `thread@doc-gate-and-lints@doc-lints-table` | cargo refuses `lints.workspace = true` beside a crate's own lint keys |
| narrowing an item no crate of the workspace uses, instead of documenting it | `thread@doc-gate-and-lints@lint-warning-answer` | `design@check@the-facade-publishes-what-a-consumer-needs`: a consumer outside the workspace decides what is public |
| no split and no issue | `thread@doc-gate-and-lints@doc-pass-split` | the signal would leave with this spec, and nothing would schedule the split; the owner ruled, round 4 |
| a split of the files past one read as steps before the documentation pass | `thread@doc-gate-and-lints@doc-pass-split` | the pass changes no code of theirs but its documentation, and no cut of `entity.rs` or `manifest.rs` is argued; the owner ruled, round 4 |
| holding the discussion of what becomes an ask before this work | `thread@doc-gate-and-lints@doc-gate-ask-status` | the owner ruled its scope too large for this discussion, round 4 |
| no recommended list | `thread@doc-gate-and-lints@recommended-gate-list` | each project would write its own list, and the generic part of the gates would ship to none, against `goal@generic-tools-and-procedures-are-shipped` |
| the incumbent head kept as written | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | the owner's ruling on `thread@doc-gate-and-lints@doc-pass-split` acts against its "nor parked as an issue" |
| one rewritten head holding both rules | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | a split the work needs and a split the work does not need lose to different rivals, so they are two decisions |
| every split signal met at implementation or review sent to an issue | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | `design@agent-skills@met-outside-the-task-outcomes` routes such a signal by the first row that applies, and a signal over code the range writes is a departure, per `design@agent-skills@code-shape-axis` |
| the split issue as a `todo` or a `deferred` entry | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | a `todo` has no re-entry point for the search to grade `trigger-met`; a `deferred` trigger is read at every merge, which brings the split back to review time; per D2 |

## Readings

The work implements no external specification. The behaviour of cargo and rustdoc it rests on is
measured, in "Decided design".

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| an adopting project's documentation needs a rustdoc flag, such as `--cfg docsrs`, which the gate's variable replaces | `thread@doc-gate-and-lints@doc-gate-denial` | the gate fails, which is no silent failure; the Migration line names the repair. No tripwire |
| a toolchain update makes a new rustdoc lint warn by default, and every project's doc gate fails | `thread@doc-gate-and-lints@doc-gate-home` | loud, and `design@knowledge-architect@toolchain-is-pinned` makes the update an explicit edit here. Dropped |
| broken links accumulate in private documentation | `thread@doc-gate-and-lints@doc-gate-item-scope` | what the decision accepts, so it fails condition 3 of the tripwire standard of `skill@knowledge-architect-planning@acceptance-criteria`. Dropped |
| the added documentation is empty text written to satisfy a lint | `thread@doc-gate-and-lints@lint-warning-answer` | the quality of this work, which the code-claims axis judges, not a failure of the decision. Dropped as a cause |
| one missing doc fails two gates and misleads the reader of the run | `thread@doc-gate-and-lints@doc-gate-denial` | loud, and accepted. Dropped |
| a work changes the code of a module that holds an open split issue without going through the design skill, so no search reads the issue's re-entry point, and the split is never weighed | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | silent: named as a cause for the harvest to judge as a tripwire, by `skill@knowledge-architect-decision-recording@ruled-tripwires`; `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` holds the general question |

## Acceptance criteria

None. Of the causes the premortem kept, the first five fail loudly or are what their decision
accepts; the last is silent and goes to the harvest's judgement as a tripwire, not to a criterion,
since the work's built code cannot apply it. The check that the doc gate fails on a broken link is a
claim of step 1.

## Implementation sequence

Steps 1 and 2 follow `skill@knowledge-architect-development` and `skill@klarch-development`. Steps
1, 3 and 4 edit agent-facing text or design heads: those edits follow
`skill@knowledge-architect-agent-configuration`, `skill@knowledge-architect-decision-recording`, and,
for the installed text, the section "Editing an installed skill or agent" of
`path@agent-skills@CLAUDE.md`, each read whole. A step that edits the installed text's source runs
`cargo klarch install-agent-skills` in the same commit, since the check compares each installed copy
with its source. Each step writes the entries in `path@knowledge-architect@CHANGELOG.md`,
`Next release`, that `design@knowledge-architect@changelog-entries` owes it, then runs
`cargo x changelog`, as `instructions@git-workflow` point 1 says.

1. **The doc gate, passing on today's tree.** The gate in `rust_project`; `is_progress` gains its
   two verbs; the doc comments of `Gate::envs` and `Spec`; the three list tests and
   `locked_reaches_every_gate_that_resolves_dependencies` updated; the restatements of "Decided
   design, doc gate home" repaired, the setup skill installed; the changelog entry with its
   Migration line. Claims:
   - the recommended list's names are `rebased`, `fmt`, `check`, `commits`, `clippy`, `doc`, `test`,
     shown by the list test of `path@gates@src/gates.rs` failing when the gate is dropped or moved
     before `clippy`;
   - the doc gate runs `cargo doc --workspace --no-deps`, resolves, uses
     `Distiller::WithoutProgress`, and its `envs` hold `RUSTDOCFLAGS` set to `-D warnings`, shown by
     a unit test failing when the variable is removed;
   - a `WithoutProgress` extract drops a `Documenting` and a `Generated` line, shown by a unit test
     failing when either verb is removed from `is_progress`;
   - `--locked` and `--ci` reach the doc gate, shown by the two tests of
     `path@xtask@tests/gates_bin.rs`;
   - the doc gate's command fails on a broken intra-doc link: a hand mutation of one link in a
     crate's documentation, run as
     `cargo x mutate run --file <F> --replace <link> --with <broken link> -- env RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`,
     reported `caught` in the commit message, with the mutant's log showing the unresolved link, since
     `issue@mutate@an-environment-failure-reads-as-caught` makes any failure read as `caught`;
   - `cargo x gates` passes.

   Fails alone on: a list test, or the doc gate failing on today's tree.
2. **The documentation pass, then the lint table.** One commit per crate or per group of crates,
   each answering that crate's warnings; the lint table last, with its comment. Claims:
   - with the lint table, `cargo clippy --workspace --all-targets -- -D warnings` passes, and so does
     the doc gate;
   - the lints are live: a hand mutation deleting the `///` line of one public item, with the clippy
     gate's command after `--`, reported `caught`, the mutant's log showing `missing documentation`.

   Fails alone on: a warning left, or a lint absent from the table.
3. **The two gates heads brought to the rules.** `design@gates@a-project-holds-its-gate-list` split
   and the head of `thread@doc-gate-and-lints@recommended-gate-list` written, as "Decided design, recommended gate list" says; the five
   citing texts of "What is already decided" re-pointed; `design@gates@distill-over-full-logs`
   edited as listed there; the per-gate list written as it stands before the doc gate. Claim:
   `cargo klarch check` passes, and `cargo klarch show design@gates@a-project-holds-its-gate-list`
   lists only the three texts that cite a project's own list.

   Fails alone on: a text that still cites the old head for the recommended list.
4. **The split rule.** `design@agent-skills@splits-planned-before-work` edited and
   the head of `thread@doc-gate-and-lints@split-not-needed-is-an-issue` written, as "Decided design, split not needed is an issue" says;
   each text of "What is already decided" that cites the incumbent read and repaired; the installed
   text and its install; the two existing split issues turned into `design` issues;
   `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` naming the new head; the
   changelog entry. Claims: `cargo klarch check` passes; the installed copies equal their source, as
   the tests of `agent-skills` check; the line-joining search finds no "never an issue" and no
   "parked as an issue" on a split of code.

   Fails alone on: an installed text that still refuses an issue for a split of code.
5. **The issues this work's signals owe.** One `design` issue, of the new head's shape, for each
   file of "Decided design, doc pass split" past one read: its instances, the size and the Read that
   measured it; its suspected mechanism in one sentence, such as "items were added to the file by
   works that weighed no split"; its re-entry point. The doc gate added to the instances of
   `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`. Claim: `cargo klarch check`
   passes.

   Fails alone on: an issue missing for a file past one read.
6. **The harvest**, per the Harvest section.
7. **The transcript and the deletion.** After the review before the merge: the discussion's
   transcript, `transcript.txt` beside this README, committed once the owner has approved it, as
   `skill@knowledge-architect-planning@spec-contents` says; then the deletion of this spec in a
   commit of its own, cited as `spec@plans@doc-gate-and-lints`.

## Order rationale

- 1 before 2: the doc gate lands empty, passing on today's tree, so a failure in step 2 is a
  failure of the documentation, not of the gate, which then also checks every link step 2 writes.
- 2 before 3: they share no file but `CHANGELOG.md`, whose entries are independent, and step 2 is
  the bulk of the work, so its review starts first.
- 3 before 4: they touch different Components' heads; the gates heads conform before the
  agent-skills rewrite so that step 6 writes into conforming heads in both.
- 4 before 5: the issues of step 5 take the shape step 4 writes.
- 5 before 6: the harvest reads the record the earlier steps leave.
- 6 before 7: the transcript holds the rulings given at the review before the merge, which follows
  the harvest.

## Defaults awaiting the owner

- **D1**, on `thread@doc-gate-and-lints@split-not-needed-is-an-issue`: the rule lands in this branch
  as "Decided design, split not needed is an issue" states it, a new head beside the incumbent, whose
  title and decision stay. The owner proposed the clarification in round 4 and did not rule on its
  shape. Ruled: the session chose this shape over the rivals of "Losing alternatives", which no
  goal ranks; the owner approved it, round 5.
- **D2**, on `thread@doc-gate-and-lints@doc-pass-split` and
  `thread@doc-gate-and-lints@split-not-needed-is-an-issue`: the split issues are of kind `design`,
  not `todo` as the shape the owner ruled on in round 4 said. The session chose `design` after that
  ruling, as the one kind whose re-entry point the record search reads at the next design or
  planning, where the owner's argument of round 4 has the split reconsidered; a split no discussion
  has weighed for a work that changes the code is an open design question, as the project-audit
  skill already records a split too large for its branch. Ruled: the session chose `design` over
  `todo`; the owner approved it, round 5.
- **Q3**, put at the review before the merge, on
  `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps`, whose
  re-entry point this spec's steps 3 and 4 met: the session's default (a), this spec a second
  instance in the issue, which stays open with its re-entry point as written, against (b), that
  discussion held before the merge. Ruled: the owner chose the default, round 6.

## Harvest

At step 6, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's commit,
with the test it fails.

| item | home |
| --- | --- |
| `thread@doc-gate-and-lints@doc-gate-home` | the head of `thread@doc-gate-and-lints@recommended-gate-list`, written at step 3, its list and its distillers gaining the documentation build, with the owner's taking-in of round 2 attributed; the comment at the gate for its place after `clippy`. The setup skill and the other restatements are repaired at step 1 |
| `thread@doc-gate-and-lints@recommended-gate-list` | the head of `thread@doc-gate-and-lints@recommended-gate-list`, at step 3 |
| `thread@doc-gate-and-lints@doc-gate-denial` | a head of `register@gates@design`, since its argument turns on how cargo and rustdoc behave, test 3 of `primer@design-heads`, with the table and the script of "Decided design, doc gate denial" |
| `thread@doc-gate-and-lints@doc-gate-item-scope` | a head of its own in `register@gates@design`, test 1, since its flag reaches every adopting project's list, in the weak form; or the comment at the gate, if the tests exclude it |
| `thread@doc-gate-and-lints@doc-lints-table` | a head of `register@knowledge-architect@design`, test 3, with the measurement of "Decided design, doc lints table"; the comment of the lint table cites it |
| `thread@doc-gate-and-lints@lint-warning-answer` | the commits of step 2 |
| `thread@doc-gate-and-lints@doc-pass-split` | the issues of step 5 |
| `thread@doc-gate-and-lints@doc-gate-ask-status` | `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`, at step 5 |
| `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | `design@agent-skills@splits-planned-before-work` and the head of `thread@doc-gate-and-lints@split-not-needed-is-an-issue`, at step 4, per D1 and D2 |
| every item of "Losing alternatives" | `register@gates@rejected`, `register@agent-skills@rejected` or `register@knowledge-architect@rejected`, each as the recording tests admit, cited by the head whose argument covers it |
| the last cause of "Premortem" | `register@agent-skills@tripwire`, guarding the head of `thread@doc-gate-and-lints@split-not-needed-is-an-issue`, if the standard of `skill@knowledge-architect-decision-recording@ruled-tripwires` admits it; otherwise dropped, listed under "Decided" |
| this spec | deleted at step 7, after the review before the merge |

## Later consequences

- The discussion of `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` is still owed,
  and the next quality tool taken in meets its re-entry point again.
- The split issues of step 5 come back at the design or planning of the next work that changes their
  files' code.
- An answer to `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` that has a review
  read every re-entry point reopens the head of `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- A project that wants broken links in private documentation caught adds
  `--document-private-items` to the doc gate in its own list.
