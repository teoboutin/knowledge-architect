# A doc gate in the recommended list, the documentation lints in this repository, and a split signal the work does not need recorded as an issue

## Status and audience

This spec is the plan of one branch, named as the spec. It closes the issue
`the-documentation-lints-on-one-branch` and its two members, `no-gate-builds-the-documentation` and
`this-repository-warns-on-none-of-the-documentation-lints`, which the commit that adds this spec
deletes, per `skill@knowledge-architect-planning@spec-or-milestone`. The
work does four things:

- the gates library's recommended list, `rust_project`, gains a `doc` gate that builds the
  workspace's documentation with rustdoc's warnings denied, so a broken intra-doc link fails a gate
  in every project that runs the list, this repository included;
- every published crate of this repository warns on `missing_docs`, `clippy::missing_errors_doc`
  and `clippy::missing_panics_doc`, and every warning is answered by the documentation it asks for;
- `design@agent-skills@splits-planned-before-work` and the installed text that states it are
  rewritten: a split signal of code that a work's discussion decides the work does not need becomes
  an issue of kind `design`, whose re-entry point is the planning of the next work that changes that
  code;
- the files this work's documentation pass writes into that are already past one read get such an
  issue each.

It is written for a session that did not witness the design discussion that produced it. It leaves
the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits: D1 is built as written, and the branch merges once the owner has ruled on it.

The discussion spans one session, which begins at the owner's message "I'd like to solve
docs/open-issues/the-documentation-lints-on-one-branch.md and the two issues it bundles." Its rounds,
named by the opening words of each of the owner's messages:

| round | the owner's message opens with |
| --- | --- |
| 1 | "I'd like to solve docs/open-issues/the-documentation-lints-on-one-branch.md" |
| 2 | "start the design discussion" |
| 3 | "nothing to contest, I agree with your proposals" |
| 4 | "Q1 a is right." |

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
| `Distiller::WithoutProgress` | the distiller that drops cargo's build-progress lines, which the `clippy` gate uses |
| the gates | `cargo x gates`, this repository's run of the recommended list, `path@xtask@src/gates.rs` |
| the doc gate | the gate this work adds, named `doc` |
| the three lints | `missing_docs` in `[lints.rust]`, and `missing_errors_doc` and `missing_panics_doc` in `[lints.clippy]`, each at `"warn"` |
| the lint table | `[workspace.lints.rust]` and `[workspace.lints.clippy]` in the root `path@knowledge-architect@Cargo.toml`, which every crate takes with `[lints] workspace = true` |
| the documentation pass | step 2: answering every warning the three lints give |
| one read | the 2000 lines a session's Read tool returns by default, the size signal of `design@agent-skills@module-split-guidelines` |
| a split signal | a signal of the module guidelines, `path@agent-skills@fragments/module-split.md`, for a split of a module |
| the hand mutation | `cargo x mutate run --file <F> --replace <OLD> --with <NEW> -- <command>`, per `path@mutate@README.md` |

## What the work is

**What exists today**, at each site the work touches:

- The recommended list is `rebased`, `fmt`, `check`, `commits`, `clippy`, `test`. Three tests pin
  its names: `the_rust_list_is_in_cost_order_and_holds_the_range_check` in `path@gates@src/gates.rs`,
  `this_repository_runs_the_recommended_list_over_its_own_checker` in `path@xtask@src/gates.rs`, and
  the two loops of `ci_reaches_every_gate` in `path@xtask@tests/gates_bin.rs`, beside
  `locked_reaches_every_gate_that_resolves_dependencies` there, which checks `--locked` per gate.
- `path@agent-skills@content/skills/setup/SKILL.md` lists the recommended list's gates in the
  paragraph that opens "**The gates.** `rust_project` gives, in cost order".
- No gate runs `cargo doc`. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` exits 0 on
  the tree this spec is written on.
- The lint table holds `unsafe_code = "forbid"` alone.
- With the three lints at warn, `cargo clippy --workspace --all-targets` gives 279 distinct warnings
  over the published crates, counted by source location: 206 `missing_docs`, 69 `# Errors`, 4
  `# Panics`. Per crate: `model` 177 / 17 / 2, `check` 12 / 13 / 0, `cli` 11 / 3 / 0, `mutate` 6 /
  6 / 0, `git` 0 / 17 / 0, `query` 0 / 6 / 1, `create` 0 / 3 / 0, `transcript` 0 / 3 / 0, `gates`
  0 / 1 / 1, `agent-skills` and `testing` 0. `xtask` has 0. In `path@model@src/resolver.rs`, 114 are
  the 75 fields and 39 variants of `enum Cause`, and 8 are `# Errors`. No test or binary target lacks
  crate documentation. Re-take it with
  `RUSTFLAGS="-W missing_docs" cargo clippy -p <crate> --all-targets --message-format=short -- -W clippy::missing_errors_doc -W clippy::missing_panics_doc`
  per crate, then deduplicate the warning lines by file, line and column.
- `path@model@src/resolver.rs` is 2511 lines, its tests from line 1583; `path@model@src/entity.rs`
  3904, its tests from line 2323; `path@model@src/manifest.rs` 3486, its tests from line 2021;
  `path@git@src/git.rs` 1948, with no `# Errors` section, which takes 17. Counted with `wc -l` and
  the line of `#[cfg(test)]`.
- `design@agent-skills@splits-planned-before-work` says of a split a work bears on: "Neither is left
  to the implementer, nor parked as an issue." `path@agent-skills@content/skills/planning/SKILL.md`
  says "never left to the implementer, and never an issue";
  `path@agent-skills@content/skills/design/SKILL.md`, loop step 1, has the discussion write "the
  split's shape, or that none is made", and says nothing of the signal when none is made.
  `issue@testing@binary-rs-is-past-one-read` and
  `issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read` are split signals already
  recorded as `todo` issues.

**Outside the work:**

- What the added documentation says, beyond its presence: the code-claims axis judges it, per
  `design@agent-skills@code-claims-scope`.
- Whether a public item should stay public: each crate's facade decision, per
  `design@check@the-facade-publishes-what-a-consumer-needs`; the documentation pass narrows no item.
- What becomes an ask of every target project, the rule and its statements:
  `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`, whose re-entry point this work
  meets, put off on the owner's word, per `thread@doc-gate-and-lints@doc-gate-ask-status`.
- The split of the model files past one read: the `design` issues step 4 opens, per
  `thread@doc-gate-and-lints@doc-pass-split`.
- A split of a subject of a design home: unchanged, per `design@agent-skills@subject-split-standard`.
- The stale quotation in `issue@git@rev-parse-answers-none-for-every-git-failure`, met outside the
  task: a commit of its own on this branch.

## What is already decided

**The decisions the design rests on and does not argue again:**

- `design@gates@distillers-are-a-closed-set`: the doc gate uses an existing distiller.
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
- `design@model@resolver-outcome` and `design@model@cause-words-in-the-resolver`: a refusal's variant
  of `enum Cause`, its arm and its words are read together.
- `design@agent-skills@completeness-check-is-a-step-claim`: the hand mutation that breaks a link is
  a claim of step 1, not an acceptance criterion.
- `goal@generic-tools-and-procedures-are-shipped`: a generic tool reaches every target project once
  knowledge-architect's owner takes it in; the owner took the doc gate in, round 2.
- `goal@adoption-is-easy`: the crate documentation docs.rs renders reaches its reader.

**The decision the work rewrites**: `design@agent-skills@splits-planned-before-work`, per
`thread@doc-gate-and-lints@split-not-needed-is-an-issue` and D1. Every text that cites it, as
`cargo klarch show design@agent-skills@splits-planned-before-work` lists them, is read at step 3 and
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
- the heads of `register@agent-skills@design` that cite it: in
  `path@agent-skills@docs/design/components.md`, `path@agent-skills@docs/design/development.md`
  (four sites, `design@agent-skills@code-shape-axis` among them),
  `path@agent-skills@docs/design/record-search.md`, `path@agent-skills@docs/design/reviews.md` and
  `path@agent-skills@docs/design/what-earns-a-record.md`.

A grep for "parked as an issue" and "never an issue" over the installed text's source and the
design homes finds the planning skill's sentence and the head's alone, besides an unrelated use in
`path@agent-skills@docs/design/how-installed-text-changes.md`; step 3 runs it again.

**The heads the harvest touches without reversing or rewriting them:**

- `design@gates@a-project-holds-its-gate-list`: its list of the recommended gates gains the
  documentation build;
- `design@gates@distill-over-full-logs`: its list of each recommended gate's distiller gains the doc
  gate's.

**The verdicts of the discussion's searches whose outcome changed something:**

| entry | verdict | outcome |
| --- | --- | --- |
| `design@gates@a-project-holds-its-gate-list`, `design@gates@distill-over-full-logs` | conflicts | each lists the recommended gates by name; the harvest adds the doc gate, named "the documentation build" so that it stays apart from the head's "the document check", which is `check` |
| `design@check@the-facade-publishes-what-a-consumer-needs`, `design@git@an-operation-in-git-s-terms-is-the-library-s` | conflicts | the narrowing exception of `thread@doc-gate-and-lints@lint-warning-answer` dropped: an item is public because a consumer needs it, extensions outside the workspace included |
| `design@agent-skills@splits-planned-before-work` | conflicts | the head rewritten, per D1 |
| `design@agent-skills@code-shape-axis` | conflicts | the rewrite states that a split signal a reviewer meets is reported and recorded as the same `design` issue, never weighed at the review; this head's routing sentence defers to the rewritten head and keeps its text |
| `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` | trigger-met | the re-entry point is met; the owner put the discussion off, round 4, and the entry gains the doc gate as an instance at step 4 |
| `design@agent-skills@subject-split-standard` | constrains | the rewrite covers splits of code alone |
| the issue kinds of `skill@knowledge-architect-issue-tracking@issue-entry` | read by the searchers | the split issue is of kind `design`, the one kind with a re-entry point; a `todo` has none, and a `deferred` trigger is read by the standing-state reviewer at every merge, which would bring the split back to review time |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` | touches | the record search reads a `design` issue's re-entry point before designed or planned work, which is where the rewrite wants it read |
| the three issues the spec closes | closes | deleted by the commit that adds this spec |

## Criteria

### A link that names nothing, in the documentation docs.rs renders, fails a gate `##broken-link-fails-a-gate`

Binding, from `goal@adoption-is-easy`. Met by `thread@doc-gate-and-lints@doc-gate-home` and
`thread@doc-gate-and-lints@doc-gate-denial`.

### A project writes no gate logic beyond its list, and the recommended list runs in cost order `##no-gate-logic-beyond-the-list`

Binding, from `design@gates@a-project-holds-its-gate-list`. Met by
`thread@doc-gate-and-lints@doc-gate-home`: the gate is a member of the list, built by the library.

### Every published crate declares the three lints, and the clippy gate passes `##published-crates-declare-the-lints`

Binding, from `design@agent-skills@documentation-presence-is-linted`. Met by
`thread@doc-gate-and-lints@doc-lints-table` and `thread@doc-gate-and-lints@lint-warning-answer`.

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
1.98.0, each in a target directory of its own.

### A split is weighed where a work is designed or planned, never where it is implemented or reviewed `##split-weighed-at-planning`

Binding, stated by the owner, round 4, as the purpose of `design@agent-skills@splits-planned-before-work`.
Met by `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.

## New names, in one place

```text
doc                                   the gate's name, in rust_project          crates/gates/src/gates.rs
RUSTDOCFLAGS="-D warnings"            the doc gate's one variable, in its envs  crates/gates/src/gates.rs
missing_docs = "warn"                 [workspace.lints.rust]                    Cargo.toml
missing_errors_doc = "warn"           [workspace.lints.clippy]                  Cargo.toml
missing_panics_doc = "warn"           [workspace.lints.clippy]                  Cargo.toml
```

`Gate`, `Distiller::WithoutProgress`, `rust_project` and `gate_envs` exist.

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
  shows, and clippy's extract is the fuller report on a missing doc.
- `path@xtask@src/gates.rs` keeps calling `rust_project` unchanged.
- The setup skill's list of the recommended gates names it.
- The changelog says that every project's list gains it, with a Migration line: a project whose
  public documentation gives a rustdoc warning fails the gate at its pin move, and fixes the warning
  or drops the gate from its own list.

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
| that config, with `RUSTDOCFLAGS="--cfg foo"` exported | none | 0 |

Accepted costs:

- in the gate's child, the variable replaces a `RUSTDOCFLAGS` the caller exported and a
  `build.rustdocflags` the project configured; a project whose documentation needs a flag adds it to
  the doc gate's `envs` in its own list;
- alternating a plain `cargo doc` with the gate re-documents the workspace's crates: 0.77 s here;
- rustdoc also reports `missing_docs`, so one missing doc fails both `clippy` and `doc`.

The nearest rival, a plain `cargo doc` with an ask for `[lints.rustdoc]` levels, is defeated by the
first row: a project that has not yet followed the ask gets a gate that passes silently.

### doc gate item scope

The gate documents public items alone, the text docs.rs renders: no `--document-private-items`. With
private items, this tree gives 2 more warnings, both ambiguous links in `mutate`. No goal ranks the
two scopes; the session chose public items, and the owner approved it, round 3.

### doc lints table

The three lints go in the lint table, beside `unsafe_code`, so every crate takes them, `xtask`
included, which has no warning under them. One table per published crate is refused by cargo,
which rejects `lints.workspace = true` beside a crate's own lint keys, so each crate would repeat
`unsafe_code`, in 11 copies. The table's comment cites
`design@agent-skills@documentation-presence-is-linted` beside `design@agent-skills@no-security-axis`.

### lint warning answer

Every warning is answered by the documentation it asks for: a `///` line on each item, a `# Errors`
section saying when the function returns an error, a `# Panics` section saying when it panics. No
item is narrowed to avoid a warning.

### doc pass split

The documentation pass splits no module. `path@model@src/resolver.rs`, `path@model@src/entity.rs`
and `path@model@src/manifest.rs` are already past one read, and the pass changes no code of theirs
but its documentation; in `resolver.rs` the growth falls inside `enum Cause`, one item no move of
code divides. Each gets an issue of kind `design`, as the rewritten head says, opened at step 4. So
does `path@git@src/git.rs` if it is past 2000 lines once step 2 has landed. The owner ruled for this
shape, round 4.

### doc gate ask status

The doc gate takes the status every gate of the recommended list holds: a project edits the list it
receives. The work adds no kind of ask. `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`
stays open, and its section "What" gains the doc gate as an instance of a quality tool taken in. The
owner put that discussion off, round 4, as larger than this work.

### split not needed is an issue

`design@agent-skills@splits-planned-before-work` keeps its slug and is rewritten, under
`skill@knowledge-architect-decision-recording`, to state:

- a split of code is decided where a work is designed or planned: the design discussion, through
  the code-split scout, or the planning; never where the work is implemented or reviewed;
- a split the work needs is a step before the work, as today, never left to the implementer;
- a split signal that the work's discussion weighs and decides the work does not need, as where the
  work changes no code of the module but its documentation, is opened as an issue of kind `design`,
  whose re-entry point is the design discussion or the planning of the next work that changes that
  module's code;
- a split signal met where the work is implemented or reviewed, by the code-shape reviewer among
  others, is reported and recorded as the same issue, never weighed or carried out in that work;
- a split of a subject of a design home is unchanged, per `design@agent-skills@subject-split-standard`.

Its argument, beside the head's current premises: the owner reported, round 4, that before the head
split signals were found and weighed at the review before the merge, which added unplanned work at
an inopportune time, and that the head's purpose is when a split is considered, at the planning
steps. The owner's earlier report, that an issue on a split sits untouched, is answered by the
re-entry point: the record search reads a `design` issue whole at the grounding of the next designed
or planned work, and grades a met re-entry point `trigger-met`, with the action `rule-before-work`.

The installed text follows: the planning skill's sentence drops "and never an issue" and says what
becomes of a signal the work does not need; the design skill's loop step 1 says the same of a signal
for which none is made. `issue@testing@binary-rs-is-past-one-read` and
`issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read` become `design` issues with that
re-entry point, each keeping its instances and its closing condition.

## Mapping tables

The work needs none.

## Losing alternatives

| alternative | lost to | the deciding fact |
| --- | --- | --- |
| the doc gate in `path@xtask@src/gates.rs` alone | `thread@doc-gate-and-lints@doc-gate-home` | the owner took the gate into the recommended list, round 2 |
| a plain `cargo doc` gate with an ask for `[lints.rustdoc]` levels | `thread@doc-gate-and-lints@doc-gate-denial` | a plain `cargo doc` exits 0 on an unresolved link, so a project that has not followed the ask gets a gate that passes silently |
| the deny passed as `--config build.rustdocflags` | `thread@doc-gate-and-lints@doc-gate-denial` | an exported `RUSTDOCFLAGS` overrides the configuration, measured: the deny is lost silently |
| `-A missing_docs` in the doc gate | `thread@doc-gate-and-lints@doc-gate-denial` | gate logic only to avoid a duplicate report |
| `--document-private-items` | `thread@doc-gate-and-lints@doc-gate-item-scope` | no goal ranks the scopes; the session's choice of public items, which the owner approved |
| one lint table per published crate | `thread@doc-gate-and-lints@doc-lints-table` | cargo refuses `lints.workspace = true` beside a crate's own lint keys |
| narrowing an item no crate of the workspace uses, instead of documenting it | `thread@doc-gate-and-lints@lint-warning-answer` | `design@check@the-facade-publishes-what-a-consumer-needs`: a consumer outside the workspace decides what is public |
| no split and no issue | `thread@doc-gate-and-lints@doc-pass-split` | the signal would leave with this spec, and nothing would schedule the split; the owner ruled, round 4 |
| a split of the model files as steps before the documentation pass | `thread@doc-gate-and-lints@doc-pass-split` | the pass changes no code of theirs but its documentation, and no cut of `entity.rs` or `manifest.rs` is argued; the owner ruled, round 4 |
| holding the discussion of what becomes an ask before this work | `thread@doc-gate-and-lints@doc-gate-ask-status` | the owner ruled its scope too large for this discussion, round 4 |
| the head kept as written | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | the owner's ruling on `thread@doc-gate-and-lints@doc-pass-split` acts against its "nor parked as an issue" |
| the split issue as a `todo` or a `deferred` entry | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | a `todo` has no re-entry point for the search to grade; a `deferred` trigger is read at every merge, which brings the split back to review time |

## Readings

The work implements no external specification. The behaviour of cargo and rustdoc it rests on is
measured, in "Decided design".

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| an adopting project's documentation needs a rustdoc flag, such as `--cfg docsrs`, which the gate's variable replaces | `thread@doc-gate-and-lints@doc-gate-denial` | the gate fails, which is no silent failure; the Migration line names the repair. No tripwire |
| a toolchain update makes a new rustdoc lint warn by default, and every project's doc gate fails | `thread@doc-gate-and-lints@doc-gate-home` | loud, and `design@knowledge-architect@toolchain-is-pinned` makes the update an explicit edit here. Dropped |
| broken links accumulate in private documentation | `thread@doc-gate-and-lints@doc-gate-item-scope` | what the decision accepts, so it fails condition 3. Dropped |
| the added documentation is empty text written to satisfy a lint | `thread@doc-gate-and-lints@lint-warning-answer` | the quality of this work, which the code-claims axis judges, not a failure of the decision. Dropped as a cause |
| one missing doc fails two gates and misleads the reader of the run | `thread@doc-gate-and-lints@doc-gate-denial` | loud, and accepted. Dropped |
| a work changes the code of a module that holds an open split issue without going through the design skill, so no search reads the issue's re-entry point, and the split is never weighed | `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | named as a cause for the harvest to judge as a tripwire, by `skill@knowledge-architect-decision-recording@ruled-tripwires`; `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` holds the general question |

## Acceptance criteria

None. Every cause the premortem kept fails loudly, or is what its decision accepts; the check that
the doc gate fails on a broken link is a claim of step 1.

## Implementation sequence

Steps 1 and 2 follow `skill@knowledge-architect-development` and `skill@klarch-development`. Step 3
edits the installed text and a design head: it follows `skill@knowledge-architect-agent-configuration`,
`skill@knowledge-architect-decision-recording`, and the section "Editing an installed skill or
agent" of `path@agent-skills@CLAUDE.md`, each read whole. Each step writes its own entries in
`path@knowledge-architect@CHANGELOG.md`, `Next release`, then runs `cargo x changelog`, as
`instructions@git-workflow` point 1 says.

1. **The doc gate, passing on today's tree.** The gate in `rust_project`; the three list tests and
   `locked_reaches_every_gate_that_resolves_dependencies` updated; the setup skill's list; the
   changelog entry with its Migration line. Claims:
   - the recommended list's names are `rebased`, `fmt`, `check`, `commits`, `clippy`, `doc`, `test`,
     shown by the list test of `path@gates@src/gates.rs` failing when the gate is dropped or moved
     before `clippy`;
   - the doc gate runs `cargo doc --workspace --no-deps`, resolves, uses
     `Distiller::WithoutProgress`, and its `envs` hold `RUSTDOCFLAGS` set to `-D warnings`, shown by
     a unit test failing when the variable is removed;
   - `--locked` and `--ci` reach the doc gate, shown by the two tests of
     `path@xtask@tests/gates_bin.rs`;
   - the doc gate's command fails on a broken intra-doc link: a hand mutation of one link in a
     crate's documentation, with the gate's command after `--`, reported `caught` in the commit
     message;
   - `cargo x gates` passes.

   Fails alone on: a list test, or the doc gate failing on today's tree.
2. **The documentation pass, then the lint table.** One commit per crate or per group of crates,
   each answering that crate's warnings; the lint table last. Claims:
   - with the lint table, `cargo clippy --workspace --all-targets -- -D warnings` passes, and so does
     the doc gate;
   - the lints are live: a hand mutation deleting the `///` line of one public item, with the clippy
     gate's command after `--`, reported `caught`.

   Fails alone on: a warning left, or a lint absent from the table.
3. **The split rule.** `design@agent-skills@splits-planned-before-work` rewritten as "Decided
   design, split not needed is an issue" says; each text of "What is already decided" that cites it
   read and repaired; the planning and design skills' sentences; `cargo klarch install-agent-skills`;
   the two existing split issues turned into `design` issues; the changelog entry. Claims:
   `cargo klarch check` passes; the installed copies equal their source, as the tests of
   `agent-skills` check; the grep of "What is already decided" finds no "never an issue" left on
   code splits.

   Fails alone on: an installed text that still refuses an issue for a split of code.
4. **The issues this work's signals owe.** One `design` issue each for `path@model@src/resolver.rs`,
   `path@model@src/entity.rs` and `path@model@src/manifest.rs`, and for `path@git@src/git.rs` if
   `wc -l` gives more than 2000 lines; the doc gate added to the instances of
   `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`. Claim: `cargo klarch check`
   passes.

   Fails alone on: an issue missing for a file past one read.
5. **The harvest**, per the Harvest section, and the deletion of this spec in a commit of its own
   after the review before the merge.

## Order rationale

- 1 before 2: the doc gate lands empty, passing on today's tree, so a failure in step 2 is a
  failure of the documentation, not of the gate, which then also checks every link step 2 writes.
- 2 before 3: they share no site, and step 2 is the bulk of the work, so its review starts first.
- 3 before 4: the issues of step 4 follow the rule step 3 writes.
- 4 before 5: the harvest reads the record the earlier steps leave.

## Defaults awaiting the owner

- **D1**, on `thread@doc-gate-and-lints@split-not-needed-is-an-issue`: the rewrite of
  `design@agent-skills@splits-planned-before-work` and of the installed text lands in this branch,
  as "Decided design, split not needed is an issue" states it. The owner proposed the clarification
  in round 4 and did not rule on its shape. Built as written; the branch merges once the owner rules.

## Harvest

At step 5, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's commit,
with the test it fails.

| item | home |
| --- | --- |
| `thread@doc-gate-and-lints@doc-gate-home` | `design@gates@a-project-holds-its-gate-list` and `design@gates@distill-over-full-logs`, their lists gaining the documentation build and its distiller |
| `thread@doc-gate-and-lints@doc-gate-denial` | a head of `register@gates@design`, since its argument turns on how cargo and rustdoc behave, test 3 of `primer@design-heads`, with the measurements of "Decided design, doc gate denial"; or, if the tests exclude it, the comment at the gate and the commit |
| `thread@doc-gate-and-lints@doc-gate-item-scope` | the same head as the denial, in the weak form, or the comment at the gate |
| `thread@doc-gate-and-lints@doc-lints-table` | the comment of the lint table and the commit |
| `thread@doc-gate-and-lints@lint-warning-answer` | the commits of step 2 |
| `thread@doc-gate-and-lints@doc-pass-split` | the issues of step 4 |
| `thread@doc-gate-and-lints@doc-gate-ask-status` | `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`, at step 4 |
| `thread@doc-gate-and-lints@split-not-needed-is-an-issue` | `design@agent-skills@splits-planned-before-work`, rewritten at step 3, per D1 |
| every item of "Losing alternatives" | `register@gates@rejected` or `register@agent-skills@rejected`, each as the recording tests admit, cited by the head whose argument covers it |
| the last cause of "Premortem" | `register@agent-skills@tripwire`, guarding `design@agent-skills@splits-planned-before-work`, if the standard of `skill@knowledge-architect-decision-recording@ruled-tripwires` admits it; otherwise dropped, listed under "Decided" |
| this spec | deleted in a commit of its own after the review before the merge, cited as `spec@plans@doc-gate-and-lints` |

## Later consequences

- The discussion of `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` is still owed,
  and the next quality tool taken in meets its re-entry point again.
- The model files' split issues come back at the planning of the next work that changes their code.
- A project that wants broken links in private documentation caught adds
  `--document-private-items` to the doc gate in its own list.
