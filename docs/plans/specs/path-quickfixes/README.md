# `check --fix` respells a reference whose target its span already names, a path citation of a skill or an agent is refused, and an unanchored path's repair asks which candidate was meant

## Status and audience

**On hold, for an undefined duration, on the owner's word.** No work of this spec starts, and no
release waits for it. The owner: "the path-quickfixes spec will be clearly marked as on hold, so
this does not happen again." and "And the quickfix spec should not blcok a release." The
register-references work, ordered after this spec, was built without it on the owner's word, and
its harvest added the respelling of a register's home to this spec. Its line numbers name the tree
at the commit each was written on, and later work has moved some; the design audit that starts its
work takes each one again.

This spec is the plan of the work that lets `cargo klarch check --fix` edit hand-written documents,
within one limit: a fix rewrites the spelling of a reference whose target its span already names,
and never chooses a target the span does not name. The work does four things:

- `check --fix` gains respellings: five kinds of reference finding whose repair is determined by
  the span and the tree are rewritten in place, and each rewrite is printed; a sixth, a `path`
  reference to a register's home, was added while this spec was on hold, per "The respelling of a
  register's home";
- a `path` citation of a skill's directory, of its `SKILL.md`, or of an agent's file is a new
  finding, and `--fix` respells it as `skill@<name>` or `agent@<name>`;
- the finding on a backticked path that names no anchor asks which candidate of the tree was
  meant, listing every candidate, and writes nothing;
- the design heads on what `--fix` may do are rewritten to state the principle, "a fix never
  rewrites what a writer meant", with the respelling as its test.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree, the design home wins, and the disagreement is a
  defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as the owner's and the owner is absent, the work does not proceed on that
  point.
- **The spec lands before its work, in a pull request of its own.** The owner, round 3: "we won't
  be implementing it right now, we'll just merge the branch with the standing spec after the
  reviews." Step 3 changes what `cargo klarch commits` judges in a commit's tree, and every commit
  of a branch is judged under the branch tip's checker, per the root CLAUDE.md,
  `instructions@git-workflow` point 1. So a span step 3 refuses must not stand in any commit of the
  work's branch, this spec included; see step 3.
- **The work starts with the design audit** of `skill@knowledge-architect-planning@working-a-slice`
  point 2, since it does not start in the session where the discussion converged. The owner, round
  3, on the parked thread `thread@path-quickfixes@same-anchor-inference`: "The narrower variant you
  identified could be interesting to rediscuss, when I restart working on this and the spec goes
  through a design audit." It applies an inference, so adopting it reopens
  `thread@path-quickfixes@fix-admission`: a design session under
  `skill@knowledge-architect-design`, before step 1, and the reviews of a plan document after it.
- It was assembled from one transcript. The discussion begins at the owner's message that opens "I would like to provide quickfix (under
  --fix) for paths", and ends at the owner's message "Keep all tripwires and AC, proceed." It holds
  4 owner messages, called rounds 1 to 4 below, and 3 agent replies, one after each of rounds 1 to
  3. The agent's replies carry the headings "Round 1", "Round 2" and "Round 3" in the transcript;
  each is called here "the reply to round 1", "the reply to round 2" and "the reply to round 3".
  Find the file by that opening message, not by its name. The reviews of this spec raised six
  defaults, D1 to D6; the owner approved all six in a fifth message, called round 5: "All defaults
  approved, proceed." The second reading raised D7 and D8, which the owner approved in a sixth message, called
  round 6: "Agreed, those are small details."

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **a span**: a backticked span of one line, as the scanner of `path@model@src/scan.rs` records it.
- **a respelling**: a rewrite of one span into another span that names the same entity: the
  target the original span already resolves to under the checker's own rules, or the entity a
  recorded head declares the span to mean. The bytes outside the span do not change. Defined by
  `thread@path-quickfixes@fix-admission`.
- **an inference**: a rewrite that chooses an entity the span does not name, from what the tree
  holds. `--fix` never applies one.
- **a raw path**: a span with two or more path segments and no `@`, whose first segment names a
  file or a directory of the tree. The check reports it with the statement "is shaped like a path
  and names no anchor", raised at `Observation::UnanchoredPath` in `path@check@src/check/references.rs`,
  line 157.
- **a candidate**: for a raw path, a target the tree's listing holds. The owner proposed in round 1
  "look at the path, under each component (including root), and at paths relative to the
  containing file"; the agent widened "each component" to each anchor in the reply to round 2. As
  decided, the places `names_this_tree` reads the first segment from: the span, with any line
  suffix or fragment dropped, joined to the root, to the directory of every anchor that is a
  directory (every Component, every location, the plans directory and every milestone; a spec is a
  file and is no place), and to the directory of the file that holds it, with `..` and `.` read
  lexically; a span with a leading `/` is joined to the root alone. A target is a candidate when
  the listing holds it; an ignored target is not. Distinct targets only, each spelled in the one
  form the check accepts for it: a plan document by its kind; a skill's or an agent's site, or a
  skill's directory, as `skill@<name>` or `agent@<name>`; an anchor's own directory from the
  anchor above it; anything else with its deepest anchor, and a trailing slash when it is a
  directory.
- **the lint**: the bare-name lint, `Observation::BareName` at line 199 of
  `path@check@src/check/references.rs`, per `design@check@backticked-bare-id`.
- **the census**: the measurement the agent's subagent took during the discussion, over the
  transcripts of this repository (14 sessions) and of thaum (25 sessions), of every reference
  finding and of how each session repaired it. Its figures are the ones quoted under Arguments.
  Its method, so that it can be taken again over another project:
  - extract every finding line from the output of `check` and `commits` in the session logs, and
    drop those of mock projects, probe worktrees, copies under a build directory and test output;
  - deduplicate by project, session, file, span and finding kind, a subagent counted under its
    parent session;
  - take the tree at each finding from the commit the session's checkout held at that moment (its
    HEAD reflog, else the last commit before the finding's timestamp), list its files with
    `git ls-tree -r --name-only`, and read its manifest; a file not yet committed is approximated
    by the first commit after the finding;
  - find the repair in the session's later edits of that file, else in the next commit of the
    file, else, for a commit message, in a later commit of the same subject;
  - apply the rule under test to the tree at the finding, and compare its rewrite with the repair;
  - read every disagreement in its context before classing it, as the reply to round 2 did: a
    misbinding is a rewrite that resolves while the text meant another target.
  The scripts were in the session's scratch directory and are not kept, per D6.
- **thaum**: a consumer of the checker, a separate repository whose checkout sits at
  `path@elsewhere@thaum/` beside this one on the owner's machine.
- **a misbinding**: an inference that rewrites a span into a reference that resolves, while the
  writer meant something else. The check passes on it, and nothing reports it later.
- **the generated list**: the files `generated_list` in `path@check@src/commands/mod.rs`, line 750,
  returns.
- **T1, T2, AC1 to AC4**: the labels the premortem put to the owner, kept in the items below.
  **AC1 of the register-references work** is the label another discussion put to the owner,
  carried here with its item.
- **D1 to D8**: the defaults the reviews raised, under "Defaults awaiting the owner": D1 to D6
  ruled by the owner in round 5, D7 and D8 in round 6.

## What the work is

What exists today at each site the work touches:

- **`check --fix`** is `fix_then_check` in `path@check@src/commands/mod.rs`, line 466. Its order, per
  `design@check@fix-before-the-checks`: a manifest holding a refused declaration writes nothing;
  the installed files are repaired and listed, `fixed: wrote <path> (installed)` or
  `fixed: removed <path> (installed)`; the gate of phases 1 to 3 runs over a model rebuilt from
  disk; the generated list is computed; the refusal of a partial commit's mismatch compares it with
  the staged tree's, per `design@check@fix-refusal-mixed-state`; the generated files are written,
  `fixed: wrote <path> (regenerated)`; the full check runs. No hand-written file is written: the
  head `design@check@safe-fix-definition` admits only "files of the installer's namespace or of the
  generated list".
- **The resolution of a `path` reference** is `path` and `anchored_target` in
  `path@check@src/check/references.rs`, lines 477 and 610. In order, `anchored_target` refuses: an
  unknown anchor; a milestone or a spec anchor; a refused shape, by `refused` at line 462 (a
  leading `/`, a `..` segment, a `.` segment); a plan document cited by its path, by
  `plan_document` at line 846, whose repair carries the respelled span, "cite it as `{form}`"; a
  target inside a deeper anchor, whose repair names no anchor ("anchor at the deepest anchor
  holding the target"). Then `path` hands the target to `assert_target`, which asserts that it
  exists, then the trailing slash against its kind. The anchors `*` and `elsewhere` take other
  branches of `path`, which do not go through `anchored_target`; `planned` goes through it, by
  `planned_target`.
- **A `path` citation of a skill or an agent** is resolved as any path and accepted. None stands in
  the tree since this spec's branch: the three that stood on the main branch, in
  `register@knowledge-architect@design` and in two issue entries of agent-skills, were written
  as `skill@<name>` references by its commit "Three citations of a project skill by its file's
  path are written as the skill's reference". The entity table records each skill and each agent
  with one site, its file and line 1, in `section_home` of `path@model@src/entity.rs`, reached from
  `harness_definitions` for the walked documents and the installed ones, under a harness only.
- **A raw path's finding** is raised at line 157 of `path@check@src/check/references.rs`. Its
  repair is fixed text: "write `path@<anchor>@<path>`, `path@elsewhere@<path>` for a path outside
  this tree, or `path@*@<path>` for every component's own copy", plus the `planned` form in a plan
  document where nothing exists, plus a sentence on dropping a line suffix or a fragment. Two tests
  pin the whole repair line: `an_unanchored_path_shape_s_repair_offers_no_unchecked_form`, line
  2068, and `an_unanchored_path_in_a_plan_document_is_offered_the_planned_form`, line 2279. Others
  match parts of it: `an_unanchored_path_shape_is_a_finding_naming_the_grammar` and
  `a_located_path_is_told_to_drop_its_suffix`, `path@check@src/check/references.rs` lines 2058 and
  2408. Others match only the statement, which the work keeps: `path@testing@tests/checks_over_mocks.rs`
  and `path@testing@tests/binary.rs`. Two sites read a
  raw path today: `names_this_tree`, line 267, which asks only whether the first segment exists and
  so decides whether the finding is raised; and the `exists` computation of the raw-path arm,
  lines 168 to 176, which joins the span unnormalized to every anchor and decides whether the
  `planned` form is offered.
- **A bare skill or agent name** is reported at `Observation::BareName`, line 199, with the repair
  `write `skill@<name>`` or `write `agent@<name>``, per `design@check@backticked-bare-id`.
- **The retired form `<anchor>@<path>`** is reported at `Candidate::AnchorInKindPosition`, line 146,
  with the repair "prefix the kind".
- **The documentation of `--fix`**: `path@check@README.md`, its command listing and the paragraphs
  on `check --fix` in its section `check`; the doc comment of `CheckArgs::fix`,
  `path@cli@src/cli/mod.rs`, in `CheckArgs`, which is the `--help` text; the shipped setup skill,
  `path@agent-skills@content/skills/setup/SKILL.md` line 213, "it writes the installed files and
  the generated `index.md` files the check would report, then checks"; the shipped issue-tracking
  skill, `path@agent-skills@content/skills/issue-tracking/SKILL.md` lines 153 and 167; the shipped
  planning skill, `path@agent-skills@content/skills/planning/SKILL.md` line 129.

**Outside the work:**

- **Applying an inference** in any form, the owner's proposal (a) of round 1 included. Ruled out by
  `thread@path-quickfixes@fix-admission`; the narrower inference is
  `thread@path-quickfixes@same-anchor-inference`, parked.
- **Commit messages.** `commits` judges history and takes no `--fix`, per
  `design@check@check-fix-flag`. The census found 70 of the 179 raw-path findings there.
- **`--fix` with `--staged`.** It stays refused while parsing, per
  `design@check@fix-with-staged`.
- **The shapes "The respellings" lists as not respelled**, per
  `thread@path-quickfixes@respelling-fixes`.
- **A span read from a Rust string literal.** Outside the checker's own source, an unbound string
  literal is read as prose, per `path@model@src/source/mod.rs` line 45; rewriting it would change a
  program's bytes. Only Markdown and Rust comments are respelled.
- **The retrospective finding on heads that state the built instance** is decided by
  `design@agent-skills@title-states-the-rule`, per `thread@path-quickfixes@head-states-the-principle`.

## What is already decided

The design rests on these decisions, and does not argue them again:

- `design@check@check-fix-flag`: `--fix` applies the safe fixes `design@check@fix-scope` admits, then runs the full check; the
  option's generic name was chosen because "a later safe fix needs no new option".
- `design@check@phases-gate-the-report`: no writer writes over an incomplete model. A respelling
  reads the entity table, so it runs after the gate. One sentence of the head is rewritten, per
  D1, in the table below.
- `design@model@trailing-slash-claims-directory`: "The kind claim sits in the span itself —
  greppable, visible to the reader, checkable — rather than inferred from what happens to exist."
  Per D3, no respelling changes a span's kind claim, so the work keeps this head.
- `design@check@every-path-names-its-anchor`, `design@check@deepest-anchor-wins` and `design@model@plan-document-kinds`: the deepest
  anchor wins, a plan document is cited by its kind, and the path is plain. The respellings write
  the form these heads require.
- `design@model@harness-kinds-cited-without-anchor`: a skill or an agent is cited `skill@<name>` or
  `agent@<name>`.
- `design@check@backticked-bare-id`: "A bare name is a pointer written with no kind." This is
  the recorded meaning the bare-name respelling relies on. The same head records that a match may be
  a span not meant as a pointer, and that its repair is then to rename the skill; that sentence is
  rewritten, in the table below.
- `design@agent-skills@plain-text-is-no-repair`: a repair names checked forms only.
- `design@check@finding-names-the-repair`: the new repair text of a raw path follows it.

The work reverses or rewrites these decisions. Every text that `cargo klarch show` lists as
referencing each, on the main branch before this spec:

| decision | texts referencing it | judged or updated at |
| --- | --- | --- |
| `design@check@safe-fix-definition`: reversed in its second clause, "it writes or removes only files of the installer's namespace or of the generated list"; restated as the principle with the respelling as its test | `path@check@README.md`, its section `check`; `design@check@finding-names-the-repair`, `design@check@index-staged-write` and `design@check@fix-scope`; `register@check@tripwire`, the entry `tripwire@check@fix-makes-a-choice`; `path@check@src/commands/mod.rs` line 445 | step 1 for the README and the source; the harvest for the design home and the tripwires |
| `design@check@fix-scope`: gains the respellings; its title states the test that `design@check@safe-fix-definition` defines, so whether the two stay two heads or merge into one is put to the owner at this harvest | `path@check@src/agents.rs` line 181 | the harvest; the comment stays true where it names the installed files only |
| `design@check@fix-refusal-mixed-state`: its sentence "`--fix` cannot lose unstaged content, since it writes only generated and installed files" is rewritten, and, per D1, its clause "The refusal is exit 2, or 1 where agent files were installed before it" becomes "exit 2, or 1 where any file was written before it" | `path@check@README.md`, its section `check`; `design@check@index-staged-write`, `design@check@check-fix-flag` and `design@check@fix-before-the-checks`; `register@check@tripwire`, the entry `fix-refusal-routed-around`, since deleted when it fired | step 1 for the README; the harvest for the design home and the tripwires |
| `design@check@fix-before-the-checks`, and the sentence of `design@check@phases-gate-the-report`, "having repaired before that gate only the installed files": the respellings join the order, per D1; the same sentence's "exits 1" stays, per D7. The other texts referencing `design@check@phases-gate-the-report` cite it for the gate itself, which the work keeps: `primer@checker-model`, `path@check@CLAUDE.md` line 90, `design@check@model-then-checks`, `design@check@an-extension-plugs-in-through-phased-hooks`, `design@model@tracked-and-ignored-is-a-finding`, `design@model@a-wrong-declaration-is-a-finding`, `design@check@index-staged-write`, `design@check@fix-before-the-checks`, `design@model@headings-open-with-hash-marks` and `design@check@owned-namespace-check`, four entries of `register@check@rejected`, at lines 74, 84, 90 and 96 of the document it replaced when this spec was written, the issue `installed-file-findings-belong-in-phase-four` line 20, since closed, its move of the installed-file findings, since landed, per `design@check@phases-gate-the-report`, and `tripwire@check@phases-gate-the-report-two`; `path@check@README.md`, its section `check`, restates the sentence and is updated | `path@check@README.md`, its section `check`; `design@check@model-then-checks` and `design@check@phases-gate-the-report`; the issue `installed-file-findings-belong-in-phase-four` line 27, since closed, its sentence on the order now in `design@check@fix-before-the-checks`; `path@check@src/commands/mod.rs` lines 445 and 542; `path@testing@tests/binary.rs` line 3746 | step 1 for the README, the source and the test; the harvest for the design home, reading `design@check@fix-before-the-checks` for the closed issue's sentence |
| `tripwire@check@fix-makes-a-choice`: it fired on the proposal this spec plans, per D4 | `design@check@safe-fix-definition` | per D4: its firing clause narrowed in this spec's branch, on the owner's word; deleted at the harvest |
| `design@check@every-path-names-its-anchor`: gains the refusal of a `path` citation of a skill or an agent; `design@check@unanchored-path-first-segment-here`: gains the question form of a raw path's repair | `primer@where-knowledge-goes`; `path@check@README.md`, its section `check`; `design@model@anchors-are-components-and-locations`, `design@model@plan-document-kinds`, `design@model@at-span-candidate-rule` and `design@check@message-references-resolve-in-either-tree`; six entries of `register@check@rejected`, at lines 117, 146, 180, 186, 362 and 371 of the document it replaced when this spec was written; `path@check@src/check/references.rs` line 14; `path@model@src/manifest.rs` line 971 | steps 3 and 5 for the source; step 6 for the README and the primer; the harvest for the design home and the rejected alternatives |
| `design@model@harness-kinds-cited-without-anchor`: a skill or an agent has no `path` spelling | `primer@checker-model`; `design@model@an-entity-belongs-to-its-anchor` and `design@model@at-span-candidate-rule`; three entries of `register@check@rejected`, at lines 156, 161 and 167 of the document it replaced when this spec was written | step 6 for the primer; the harvest for the design home and the rejected alternatives |
| `design@check@backticked-bare-id`: its repair is applied by `--fix`, and its sentence "A project skill named by an ordinary word makes every backticked use of that word a finding, and the repair is to rename the skill with the project's prefix" is rewritten: `--fix` respells such a span before a session reads the finding, which T1 watches | `primer@where-knowledge-goes`; `design@agent-skills@plain-text-is-no-repair`; `path@check@README.md`, its section `check`; `design@model@at-span-candidate-rule` and `design@check@unanchored-path-first-segment-here`; the issue `a-bare-mention-of-a-deleted-entry-is-never-reported`, since closed; `issue@check@tooling-for-project-skills`, line 42; `design@knowledge-architect@changelog-section-fixed-at-release` | step 4 for the source; step 6 for the README and the primer; the harvest for the design home; `issue@check@tooling-for-project-skills` read again at the harvest |

Open entries that bear on the work, read again at the audit:

- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: a fix that wrote a
  project-specific anchor into shipped text would make a reference that dangles in every
  installing project. No respelling writes a new anchor, per
  `criterion@path-quickfixes@shipped-text-stays-portable`.
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: the new repair text of a
  raw path and the new `fixed:` lines change a command's printed output.
- `tripwire@model@candidate-rule-silence`: a span the candidate rule leaves silent gets no
  respelling, and a widened rule widens what `--fix` rewrites. Judged: a widening is a change to
  `design@model@at-span-candidate-rule`, which reopens this work's mapping table.
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: it reopens
  `design@agent-skills@plain-text-is-no-repair`, with a mechanical check for plain-text pointers
  among the candidates. Judged: the respellings and the question form are a
  checked way out of the findings that sessions have cleared by writing plain text, which the
  reopening reads.
- A staged pass that a clean checkout of the same tree fails, which
  `design@check@staged-check-semantics` rules out: a respelling rewrites the working-tree file only.
  Judged: `--fix --staged` stays refused, so no respelling is computed over the staged tree; the
  staged copy keeps the old span and `check --staged` reports it, as "The order of a run" says.
- `issue@check@configuration-for-several-agent-providers`: the citation check assumes the `claude`
  harness's layout. Judged: it reads the entity table's sites, not a layout, so a second harness
  that defines skills and agents in the table is judged the same way.

## Criteria

### A fix never writes a reference that passes the check while naming something the writer did not mean `##no-silent-misbinding`

Binding. Derived by the agent in the reply to round 1 from
`goal@agents-work-without-drift` and
`goal@documentation-stays-consistent`. Met by
`thread@path-quickfixes@fix-admission`, `thread@path-quickfixes@respelling-fixes` and
`thread@path-quickfixes@raw-path-inference`.

### A fix's bytes are determined by the tree and the pinned version `##determined-bytes`

Binding as a presumption. Derived from the first clause of `design@check@safe-fix-definition`,
which the work keeps. Met by every thread.

### No unstaged content is lost `##no-content-loss`

Binding as a presumption. Derived from `design@check@fix-refusal-mixed-state` and from the root
CLAUDE.md, `instructions@git-workflow`, point 2. Met by `thread@path-quickfixes@fix-run-order`,
given `acceptance@path-quickfixes@a-changed-file-is-not-written`.

### No fix writes an unchecked form `##no-unchecked-form`

Binding as a presumption. Derived from `design@agent-skills@plain-text-is-no-repair`. Met by every
thread; the question form offers checked forms only.

### The share of raw-path findings repaired with no hand edit `##covers-the-common-finding`

Weighed. Stated by the owner in round 1: "It is by far the most common finding, because agents
always perfer the raw backticked form by habit." Unmet, and accepted by the owner in round 3: "I'm
not fully satisfied, because this design is quite a lot of work (especially the rewriting part),
and yet it only addresses a small portion of the findings that happen." The reply to round 3
stated this reading of the words, and the owner's round 4 did not contest it.

### No fix writes a project-specific anchor into shipped text `##shipped-text-stays-portable`

Weighed. Derived by the agent in the reply to round 1 from
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. Met: a respelling names the
target its span already names, so it writes no anchor the span did not already depend on, and the
question form writes nothing.

### The checker is not designed from this repository's own needs where a consumer's differ `##no-self-catering`

Binding, as the owner stated it in round 2: "I want to avoid catering to this project's own needs
when designing the checker." Listed as binding by the agent in the reply to round 2, until the
owner says otherwise; not contested. Met: the reply to round 2 re-argued
`thread@path-quickfixes@fix-admission` without this repository's cases,
`argument@path-quickfixes@arg-19` and `argument@path-quickfixes@arg-20`.

## New names, in one place

An illustration of the shapes, not authority; the audit and the implementing session decide the
final names.

```text
crates/check/src/fix.rs                                  new module, pub(crate)
  struct Respelling { file, line, old, new }            one rewrite: the span's text without its
                                                        backticks, before and after
  fn respellings(model, manifest, entities, inputs)     -> Vec<Respelling>, computed after the first
                                                        gate; no check calls it
  fn apply(root, respellings, baseline)                 baseline: each file's text as the model
                                                        read it; writes a file only if its bytes on
                                                        disk equal its baseline, and returns the
                                                        files written and the files skipped

crates/model/src/scan.rs
  Located::origin                                       new: whether a span of Rust source was read
                                                        from a comment or a string literal

crates/check/src/check/references.rs
  fn candidates(span, rel, anchors, inputs)             new: the candidates of a raw path, as Names
                                                        defines them, each spelled as a reference
  fn harness_citation(target, entities)                 new: the skill or agent whose site, or
                                                        whose skill directory, a path target is
  fn path, fn anchored_target, fn refused,              existing; each refusal that has a
  fn plan_document, fn assert_target                    respelling exposes the respelled span

output of check --fix
  fixed: respelled <file>:<line> `<old>` → `<new>`      one line per respelling, after the installed
                                                        files' lines and before the generated ones'
  not fixed: <file> changed since it was read           one line per skipped file
```

The new module is `planned@check@src/fix.rs`.

## Decided design

### What a fix may write

**A fix never rewrites what a writer meant.** That is the principle, and the head
`design@check@safe-fix-definition` states it at the harvest. Its tests:

- **the bytes are determined by the tree and the pinned version**, the clause kept;
- **a hand-written document is edited only by a respelling**: the span already names exactly one
  entity, under the checker's own resolution or under a recorded head that declares what the span
  means; the rewrite names the same entity in the form the check requires; no byte outside the
  span changes;
- **an inference is never applied**: a rewrite that chooses an entity the span does not name stays
  the writer's, however the tree constrains it;
- **git is never touched**: staging and the index stay outside `--fix`, as today.

The installed files and the generated files still pass, as their bytes are the installer's and the
generator's, not a writer's. The respellings join `design@check@fix-scope`'s members as a kind
added on purpose, each passing the safe-fix test.

**Where a respelling may write**: a Markdown document of the walk, and the comments of a Rust
source file of the walk. Never a Rust string literal, never a file of the generated list, which
the generator rewrites, and never a commit message. The scanner drops the origin of a Rust span
today: `Located` holds a line and the observation, and the comment-or-literal flag of
`CommentLine` in `path@model@src/source/rs.rs` is not carried. Step 1 carries it, as `Located::origin`
under New names, and a span whose origin is a literal is not respelled.

**How a span is found in its line**: the scanner records a span's line and text, not its column.
`respellings` emits one `Respelling` per observation, so two observations of one span on one line
give two. `apply` finds, in the line as stored on disk, each occurrence of the span's text delimited
by one backtick on each side, where neither delimiter touches a further backtick. When the number
of such occurrences differs from the number of `Respelling` values with that file, line and text,
the line is not rewritten. Every occurrence is replaced, from the end of the line to its start, so
the offsets of the earlier ones stay valid.

**What a run prints**: one line per respelling, `fixed: respelled <file>:<line> `<old>` → `<new>``,
after the installed files' lines and before the generated files'; and one line per skipped file,
`not fixed: <file> changed since it was read`.

Nearest rival: the owner's unique-candidate rule as the admission test. The fact that defeated it:
the census found 4 misbindings among the one-candidate findings in files, and the tree cannot tell
apart two meanings of one span, `argument@path-quickfixes@arg-20`. The owner weighed the corrected
premise, `argument@path-quickfixes@arg-18`, and kept the ruling.

### The respellings

A respelling computes the one form the check accepts for the target the span already names, in one
step: the path is normalized lexically, then spelled as Names spells a candidate, the kind claim
excepted: the writer's trailing slash, or its absence, is kept as written. A respelling applies to
a `path` reference whose anchor is one the manifest declares or one the tool constructs, the plans
directory or a milestone: never to `*` or `elsewhere`, and never to a `planned` reference. A
target the ignore rules cover is accepted by the check without existing, and is not respelled. A respelling whose result would itself be a finding is not applied;
`acceptance@path-quickfixes@respellings-leave-no-finding` judges it.

| finding | its respelling | condition |
| --- | --- | --- |
| a target inside a deeper anchor | `path@<deepest>@<rest>` | the target exists |
| a `..` or a `.` segment | the normalized path, spelled as above | the normalized target stays inside the root and exists |
| a plan document cited by its path | the form `plan_document` already names: `spec@plans@<id>`, `milestone@plans@<id>` or `spec@<milestone>@<slice>` | the finding is raised |
| the retired `<anchor>@<path>`, the anchor not a reserved word | the target's one form, as above | the target exists under that anchor |
| a bare skill or agent name, on one line | `skill@<name>` or `agent@<name>` | the lint reports it, and the name is a skill's or an agent's, not both |
| a `path` citation of a skill or an agent | `skill@<name>` or `agent@<name>` | the finding of "The skill and agent citation" is raised |

**Not respelled**, each a finding as today:

- a trailing slash that disagrees with the target's kind, per D3, `argument@path-quickfixes@arg-27`;
- a leading `/`: its intended base is ambiguous;
- a path that climbs above the root;
- the retired `<anchor>@<path>` whose target does not exist, or whose anchor is a reserved word;
- a `planned` reference whose target exists;
- a span with a line suffix or a fragment;
- a span wrapped onto two lines, a bare name wrapped at a hyphen included;
- a name both a skill and an agent hold, `argument@path-quickfixes@arg-28`.

The reasons not given here are `argument@path-quickfixes@arg-8`.

Nearest rival: no respelling, the head as it stands. It lost on `argument@path-quickfixes@arg-3`: each
row's output is already computed or named by the check, and applying it changes no meaning.

### The respelling of a register's home, added by the register-references work

Added to this spec by the harvest of the register-references spec, on the owner's word, given
while this spec was on hold: "We will build the spec we just designed, without anything related to
quickfixes. Then, harvest it into either the project record or into the path quickfix spec." That
work built the `register` kind, `design@model@register-home-reference`, and the finding on a `path`
reference to a register's home, `design@model@path-to-a-home-is-reported`, and left this respelling
to this spec.

`--fix` respells each finding on a home target with no line number or fragment into the form its
repair names: `register@<anchor>@<register>`, the anchor being the one the finding names, `*` kept
as `*`; the trailing slash is dropped. Where a `..` or a `.` segment is normalized by the respellings
above and the normalized target is a home target, the result is the new form, written in one step.
Every home target respells; which paths are home targets is the head
`design@model@path-to-a-home-is-reported`.

The owner ruled on three defaults of that spec, which this subsection carries: "All three agreed,
proceed".

- **A span is respelled alone.** A file-or-directory pair in one sentence becomes two identical
  spans, which `--fix` leaves: collapsing the pair edits bytes outside the spans, which a respelling
  never does. A span with a line number or a fragment is not respelled. Both are repaired by hand.
- **Two exclusions of "The respellings" give way for a home target alone**: the rule's "never to
  `*`", since a home target under `*` names one entity, every component's own home of one register,
  and the trailing slash that disagrees with the target's kind, since the new form claims no kind
  and names the home in both shapes. Both stand for every other finding.
- **The clause of `tripwire@check@fix-makes-a-choice` that exempts this spec's respellings** covers
  this one, as one of them.

### The question form

The finding on a raw path keeps its statement. Its repair lists the candidates as a question, and
keeps the forms for a path the tree does not hold. The repair, with one candidate and with two or
more, the candidates in byte order of their spelling, per D8, approved in round 6:

```text
→ did you mean `<c1>`? Otherwise write `path@elsewhere@<project>/<path>` for another project's
  file, `path@*@<path>` for every component's own copy, or a placeholder for a pattern
→ did you mean one of `<c1>`, `<c2>`? Otherwise write `path@elsewhere@<project>/<path>` for
  another project's file, `path@*@<path>` for every component's own copy, or a placeholder for a
  pattern
```

The `planned` clause of a plan document and the suffix sentence follow the "Otherwise" sentence, as
each follows today's repair. The exact words are the implementing session's, within these
constraints, and the pinned tests pin them.

- With no candidate, the repair is today's.
- With a line suffix or a fragment, the candidates are computed from the span with the suffix
  dropped, and today's sentence on dropping the suffix stays.
- In a plan document, where nothing exists, today's `planned` form stays.
- `--fix` writes nothing for this finding.
- The candidates are computed by one function, which replaces the `exists` computation of the
  raw-path arm; `names_this_tree` keeps its role of deciding whether the finding is raised. A
  finding raised with no candidate is possible, where the first segment exists and the rest does
  not.

Nearest rival: B, a repair that names the one candidate as an instruction, "write
`path@check@<path>`". It lost to the owner's question in round 2, `argument@path-quickfixes@arg-14`.

### The skill and agent citation

Under a harness, a `path` reference whose target is a skill's site, its `SKILL.md`, the directory
holding that site, or an agent's site, is a finding, judged against the entity table's sites. The
finding names `skill@<name>` or `agent@<name>`, and `--fix` respells it. It is judged by where the
target sits, as a plan document citation is.

- A path to any other file inside a skill's directory stays a `path` reference.
- The skills directory and the agents directory themselves stay `path` references.
- Under `harness = []`, the table defines no skill and no agent, so nothing is reported.

Nearest rival: no check, the citation accepted as a path. It lost on `argument@path-quickfixes@arg-15`.

### The order of a run

Per D1, which the owner approved in round 5:

1. a manifest holding a refused declaration writes nothing;
2. the installed files are repaired;
3. the gate of phases 1 to 3;
4. the respellings, each printed;
5. the model rebuilt from disk, and the gate run again; a stop is printed as `check` prints it;
6. the refusal of a partial commit's mismatch, over the rebuilt model;
7. the generated files;
8. the full check.

Exit codes, per D7, approved in round 6: a stop at either gate exits 1, as today, whatever was written: it reports
findings. A failed write and the refusal each exit 2 when nothing was written, and 1 once any file,
installed or respelled, was written; 2 promises an untouched tree. A file skipped by `apply` because it changed since it was read is printed as a `not fixed:` line and
is not a failed write: the run continues, and the check of point 8 reports the findings left in
it.

`--fix --staged` stays refused while parsing. A respelling rewrites the working-tree file only; a
file whose staged copy differs keeps the old span in git's index, and `check --staged` reports it
until the file is staged again.

Nearest rival: the approved order of the checkpoint table, the refusal before the respellings.
D1 replaced it, `argument@path-quickfixes@arg-29`, on the owner's word of round 5.

## Mapping tables

Every finding of `check::references` that concerns a `path`, `planned` or harness-kind reference,
or a span with no `@`, and what `--fix` does with it after the work:

| finding | `--fix` |
| --- | --- |
| unknown kind, unknown anchor, the anchor carries no register of that kind, an undefined id | nothing: a choice |
| a malformed span, the empty-id case included | nothing |
| the retired `<anchor>@<path>` | respelled, when the anchor is declared and the target exists |
| a raw path | nothing; the repair asks |
| a bare skill or agent name | respelled, under the conditions of "The respellings" |
| a wrapped span | nothing |
| `path` under a milestone or a spec anchor | nothing |
| a refused shape: a leading `/` | nothing |
| a refused shape: `..` or `.` | respelled, when the normalized target exists inside the root |
| a plan document cited by its path | respelled |
| a target inside a deeper anchor | respelled |
| a trailing slash against the target's kind | nothing, per D3 |
| a `path` that does not resolve | nothing |
| a `path` reference under the anchor `*`: any of its findings | nothing |
| a `path@elsewhere@<path>` that resolves | nothing |
| any `planned` finding | nothing |
| a harness-kind reference that resolves to nothing | nothing |
| a plan item cited from outside its plan | nothing |
| a link finding | nothing |
| a `path` citation of a skill or an agent (new) | respelled |

A finding that `check::references` raises and this table misses is not respelled. The audit reads
the table against the findings the code raises, and adds the rows it misses.

## Losing alternatives

- **The unique candidate applied by `--fix`**, the owner's proposal (a) of round 1 and option A of
  the reply to round 1. Lost to `thread@path-quickfixes@fix-admission` on
  `argument@path-quickfixes@arg-5`, `argument@path-quickfixes@arg-18` and `argument@path-quickfixes@arg-20`.
- **The unique candidate applied and printed as `inferred`**, the alternative of the reply to round
  2. Lost to `thread@path-quickfixes@fix-admission` on `argument@path-quickfixes@arg-21`.
- **A separate opt-in flag that applies inferences**, option C. Lost to
  `thread@path-quickfixes@raw-path-inference` on `argument@path-quickfixes@arg-12`.
- **A repair that names the one candidate as an instruction**, option B, the agent's default of
  round 1. Superseded by the owner's question form, `thread@path-quickfixes@raw-path-inference`, on
  `argument@path-quickfixes@arg-14`.
- **A precedence rule among two or more candidates**, the file's own anchor first. Lost to
  `thread@path-quickfixes@raw-path-inference` on `argument@path-quickfixes@arg-6`.
- **A respelling of a `planned` reference whose target exists, of a retired `#<id>`, of a span with
  a line suffix or a fragment, and of a leading `/`.** Lost to
  `thread@path-quickfixes@respelling-fixes` on `argument@path-quickfixes@arg-8`.
- **A respelling of the trailing slash**, per D3. Lost to `thread@path-quickfixes@respelling-fixes`
  on `argument@path-quickfixes@arg-27`.
- **A name both a skill and an agent hold respelled as the skill.** Lost to
  `thread@path-quickfixes@respelling-fixes` on `argument@path-quickfixes@arg-28`.
- **The incumbent: `--fix` writes only installed and generated files, never a hand-written one.**
  Lost to `thread@path-quickfixes@fix-admission` on `argument@path-quickfixes@arg-3`.
- **The same-anchor inference** is not lost: it is parked,
  `thread@path-quickfixes@same-anchor-inference`.

## Readings

None. The work reads no external specification. The layout of a skill and an agent is the one
`design@model@harness-kinds` already records.

## Premortem

Assume the work shipped and failed. The causes, as presented in the reply to round 3:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| 1. A project names a skill by an ordinary word, such as `check`, and `--fix` turns every backticked `check` into a reference to a skill named `check` | `thread@path-quickfixes@respelling-fixes`, the bare-name row | the tripwire **T1**, on the owner's word, round 4 |
| 2. Agents take the first listed candidate without reading the sentence, and the misbinding comes back through the agent | `thread@path-quickfixes@raw-path-inference` | the tripwire **T2**, on the owner's word, round 4 |
| 3. A file edited between the moment `--fix` reads it and the moment it writes it loses that edit | `thread@path-quickfixes@fix-run-order`, `criterion@path-quickfixes@no-content-loss` | the acceptance criterion **AC1**, on the owner's word, round 4 |
| 4. A respelling produces a form the check refuses, such as the deepest anchor applied before the plan-document rule | `thread@path-quickfixes@respelling-fixes` | the acceptance criterion **AC2**, on the owner's word, round 4 |
| 5. A rewrite touches bytes outside the span: line endings, a wrapped span, a multibyte neighbour | `thread@path-quickfixes@fix-admission` | the acceptance criterion **AC3**, on the owner's word, round 4 |
| 6. The new check reports a path into a skill's other files, which has no harness-kind form | `thread@path-quickfixes@skill-cited-by-path` | the acceptance criterion **AC4**, on the owner's word, round 4 |

The owner's ruling, round 4: "Keep all tripwires and AC, proceed."

T1, as the harvest writes it: guards `design@check@fix-scope`. Fires when a review or a
retrospective reports a `--fix` rewrite of a bare name the writer did not mean as a skill or an
agent. Response: reopen the bare-name respelling; the candidate fix is to respell only names that
carry a prefix, which `issue@check@tooling-for-project-skills` would check. Re-entry: the
standing-state review of every dispatched review, `agent@knowledge-architect-standing-state-reviewer`, and each retrospective intake.

T2, as the harvest writes it: guards the head that records the question form, per the harvest row
of `thread@path-quickfixes@raw-path-inference`. Fires when a review finds a raw-path repair that took a listed candidate where
the text meant another project's file or every project's layout. Response: reopen the question
form; one candidate fix is to list the candidates in a fixed order with no first-choice position.
Re-entry: the standing-state review of every dispatched review, `agent@knowledge-architect-standing-state-reviewer`. It fires only in sessions the owner
sees, as `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`
records of every tripwire on agent behaviour.


## Acceptance criteria

### AC1: `--fix` writes a document only if its bytes are still the bytes the model read `##a-changed-file-is-not-written`

Guards `thread@path-quickfixes@fix-run-order`, and `criterion@path-quickfixes@no-content-loss`.
Judged at step 1. The baseline is the document's text as the model read it, from which the
respellings were computed. A unit test calls `apply` with a baseline that differs from the bytes
on disk, and asserts that the file keeps the bytes on disk and that `apply` returns it as skipped;
the `not fixed:` line is asserted by a unit test of the function that prints it, since no binary test can change a file inside one run. Fires when the file is written. Response: reopen
`thread@path-quickfixes@fix-run-order` on how a document is written.

### AC2: every respelling leaves no finding on its span, and a second run writes nothing `##respellings-leave-no-finding`

Guards `thread@path-quickfixes@respelling-fixes`. Judged at steps 2 to 4. A test builds a project in
a copy, with `Sandbox` of `path@testing@tests/binary.rs` serving the `claude` harness, holding every
respellable shape of "The respellings" and every shape listed there as not respelled. It runs
`check --fix`, then asserts that no finding is reported on any rewritten span, that every shape not
respelled is still reported, and that a second `check --fix` prints no `fixed:` line. Fires on a
finding on a rewritten span, on a shape not respelled that was rewritten, or on a second write.
Response: reopen `thread@path-quickfixes@respelling-fixes` on the row that produced it.

### AC3: a respelling changes exactly the bytes of its span `##respelling-is-byte-exact`

Guards `thread@path-quickfixes@fix-admission`. Judged at step 1. A unit test over a file with CRLF
line endings, multibyte characters beside the span, the same span twice on one line, the span
inside a double-backtick run, and a span wrapped onto two lines asserts that the byte difference
after the rewrite is exactly each recorded one-line span, and that the double-backtick run and the
wrapped span are untouched. Fires when any other byte changes. Response: reopen `thread@path-quickfixes@fix-admission` on
what a respelling may touch.

### AC4: a path into a skill's other files, and the skills directory itself, stay silent `##other-skill-files-stay-silent`

Guards `thread@path-quickfixes@skill-cited-by-path`. Judged at step 3. A test over a project
serving the `claude` harness, built with `Sandbox`, asserts that a `path` reference to a file under
a skill's directory other than its `SKILL.md`, and one to the skills directory itself, raise no
finding of the new check. Fires when either is reported. Response: reopen `thread@path-quickfixes@skill-cited-by-path` on
what the check judges.

### AC1 of the register-references work: `--fix` respells every home target written as one span with no suffix, and a second run writes nothing `##register-home-respelled`

Kept by the owner in the design discussion of the register-references spec, under the label AC1
there, and carried here with its label: "Keep none of the three tripwires, but keep the three AC.
Proceed". Guards the respelling of "The respelling of a register's home". The observable that fires
it: a test, on a `Sandbox` copy, plants each home target under a literal anchor, under `*`, inside a
deeper anchor, with a wrong trailing slash, and as a pair; after `check --fix`, a planted home
target remains, or a span is not the form its finding's repair names, or a second run respells
anything. The pair is asserted to become two identical spans. Response: the respelling is repaired
before the merge; if a home target cannot be respelled without choosing, the respelling reopens.

## Implementation sequence

Steps 1 to 5 follow the project's development procedure, `skill@klarch-development`, and pass the
gates they owe. Step 6 edits the root CLAUDE.md and the shipped text, which that skill excludes:
it follows `skill@knowledge-architect-agent-configuration`, and for the shipped text the section on
editing an installed skill or agent of `path@agent-skills@CLAUDE.md`. Each default under "Defaults
awaiting the owner" gates the point it names. The mock project `planted` declares `harness = []`,
so a test that needs a skill or an agent builds its project in a copy with `Sandbox` serving the
harness, as `Sandbox::serve_claude` does; `check --fix` is never run inside `planted`, per
`path@check@CLAUDE.md`.

1. **The respelling machinery, empty.** `planned@check@src/fix.rs` with `Respelling`, `respellings`
   returning none, and `apply` with its baseline; `Located::origin`; the order and the exit codes
   of "The order of a run" in `fix_then_check`; the `fixed: respelled` and `not fixed:` lines. The README, the doc
   comment of `CheckArgs::fix`, and the source comments that cite
   `design@check@safe-fix-definition`, `design@check@fix-before-the-checks` and
   `design@check@fix-refusal-mixed-state` updated. Claims: AC1; AC3; `check --fix` over `dirhome`
   writes no document and prints what it prints today; every existing test of `--fix` passes.
   Fails alone on: a byte written outside a span, or a write over a changed file.
2. **The path respellings**: the deeper anchor, `..` and `.`, the plan document by its path, the
   retired `<anchor>@<path>`, and a register's home, per "The respelling of a register's home".
   Claims: AC2 for these rows, and AC1 of the register-references work for the last; the shapes
   listed as not respelled are not. Fails alone on: a respelled span still reported.
3. **The skill and agent citation.** The finding, judged from the entity table's sites, and its
   respelling. Before the commit that adds the finding, any `path` citation of a skill or an agent
   that stands in the tree is written as its reference in the branch's first commit, so that no
   commit of the branch carries it under the tip's checker. Claims: AC4; AC2 for the row; nothing is
   reported under `harness = []`. Fails alone on: a citation of a skill's `SKILL.md` accepted, or
   another file of its directory refused.
4. **The bare-name respelling.** Claims: AC2 for the row; a name both kinds hold, and a bare name
   wrapped at a hyphen, are not respelled. Fails alone on: a span the conditions exclude respelled.
5. **The question form.** `candidates`, the new repair text, and the two pinned tests,
   `an_unanchored_path_shape_s_repair_offers_no_unchecked_form` and
   `an_unanchored_path_in_a_plan_document_is_offered_the_planned_form`, updated to the new line.
   Claims: zero, one and two candidates each give the repair of "The question form"; a line suffix
   and a plan document keep their sentences; a plan document candidate is spelled by its kind;
   `--fix` writes nothing for the finding. Fails alone on: a candidate that does not exist listed,
   or an existing one missed.
6. **The documentation and the changelog.** `path@check@README.md` on `--fix`, on the new finding,
   and on the question form; the primer, `primer@checker-model` and `primer@where-knowledge-goes`,
   where it states the rules of `design@check@every-path-names-its-anchor`, `design@check@unanchored-path-first-segment-here`,
   `design@model@harness-kinds-cited-without-anchor` and
   `design@check@backticked-bare-id`; the shipped setup, issue-tracking and planning skills
   where they say what `--fix` writes, then `cargo klarch install-agent-skills`. In this
   repository the root CHANGELOG.md is walked and its copies in the crates are not, so a `--fix`
   that respells the root file leaves the copies different until `cargo x changelog` runs. Entries in the
   `Next release` section of CHANGELOG.md, then `cargo x changelog`:
   - New features: `cli`, minor: `check --fix` respells a reference whose target its span names,
     and prints each rewrite.
   - Migration: `checks`, minor: a `path` citation of a skill or an agent is a finding; rewrite it
     as `skill@<name>` or `agent@<name>`, or run `check --fix`.
   - New features: `checks`, patch: the finding on a backticked path that names no anchor lists
     the candidates the tree holds, per `argument@path-quickfixes@arg-13`.
   Claims: `cargo klarch check` passes with the installed copies committed;
   `cargo test -p knowledge-architect-agent-skills` passes; the shipped edits cite no entry and
   write `{{command}}`, by a reading. Fails alone on: a changelog copy that differs.
7. **The harvest**, below, reviewed before the merge. The spec is deleted in a commit of its own
   after that review.

## Order rationale

Step 1 before step 2: the respellings need the machinery that step 1 proves empty. Step 2 before
step 3: the citation's respelling goes through the path resolution that step 2 changes. Step 3
before step 4: step 3 is the change to what the check judges, and lands while the respelling
machinery is fresh; step 4 changes no check. Step 4 before step 5: the question form is
independent of `--fix`, and comes last among the code so the `--fix` steps 1 to 4 are read together by the review of point 4 of `skill@knowledge-architect-planning@working-a-slice`, which then
judges the run order they build as a whole. Step 5 before step 6: the documentation describes what was built. Step 6 before step 7:
the harvest records the decisions the built work implements.

## Defaults awaiting the owner

None. D1 to D8 are under the subsection below.

### The defaults the owner ruled on

Six defaults stood after the reviews of this spec. The owner ruled on all six in one message,
called round 5: "All defaults approved, proceed." Each is applied in the sections it names, and is
kept here with its reason. D4's narrowing of `tripwire@check@fix-makes-a-choice` is applied in the
commit that records the ruling. The second reading of this spec raised D7 and D8, which the owner
approved in a sixth message, called round 6: "Agreed, those are small details."

- **D7**, on `thread@path-quickfixes@fix-run-order`, from the design-conformance and code-claims
  reviews, `argument@path-quickfixes@arg-29`. D1, as the owner approved it in round 5, made a stop at
  either gate exit 2 when nothing was written. That contradicts `design@cli@exit-code-ladder` and
  the sentence of `design@check@phases-gate-the-report` that says the stop "exits 1", and the
  existing test `check_fix_writes_no_generated_file_over_an_incomplete_model` of
  `path@testing@tests/binary.rs` asserts exit 1. The default: a stop exits 1 whatever was written, as
  today; only the refusal and a failed write take 2 or 1 by what was written. This narrows the
  approved D1. The alternative: D1 as approved, with both heads and the test changed.
- **D8**, on `thread@path-quickfixes@raw-path-inference`, from the cold-implementer review. The
  first revision left the order of the candidates and a bound on their number to the audit, a
  choice the document did not rule. T2's response names "a fixed order with no first-choice
  position" as a candidate fix. The default: every candidate is listed, in byte order of its
  spelling, so no position is chosen by the tree's meaning, and the question reads "did you mean
  one of" when there are two or more. The alternative: a bound, with the remaining count printed.

- **D1**, on `thread@path-quickfixes@fix-run-order`, from the author at assembly. The order the owner approved puts the
  refusal of a partial commit's mismatch before the respellings. The refusal compares the generated
  files of the working tree with the staged tree's; a respelling inside an issue title changes the
  working tree's `index.md` after that comparison, so the comparison would be made on bytes the run
  then changes. The default: the order of "The order of a run", with its exit codes,
  `argument@path-quickfixes@arg-29`, and the sentence of `design@check@phases-gate-the-report` and the
  exit clause of `design@check@fix-refusal-mixed-state` rewritten at the harvest. The alternative:
  the approved order, with the refusal made before the respellings.
- **D2**, on `thread@path-quickfixes@same-anchor-inference`, from the transcript's coverage at assembly. The owner's word of
  round 4, "Keep all tripwires and AC", stands after a premortem that labelled T1 and T2, while the
  parked thread's tripwire stood in the checkpoint table with no label. The default: if the thread
  is still parked at the harvest, its tripwire is recorded in `register@check@tripwire`,
  guarding `design@check@safe-fix-definition`, as rewritten. The
  alternative: it leaves with this spec.
- **D3**, on `thread@path-quickfixes@respelling-fixes`, a material finding of the decision-record and design-conformance
  reviews, `argument@path-quickfixes@arg-27`. The owner approved the trailing-slash row without the
  premise that the tripwire it fires was recorded from a premortem cause naming that very fix, and
  that `design@model@trailing-slash-claims-directory` places the kind claim in the span. The default:
  the row is dropped, and the head is kept. The alternative: the row stays, and the harvest
  reverses that head.
- **D4**, on `thread@path-quickfixes@fix-admission`, from the standing-state, routing and decision-record reviews.
  `tripwire@check@fix-makes-a-choice` fired on the proposal this spec plans, and stays listed with
  its clause until the harvest, so every standing-state review until then finds it fired. The
  owner's word of round 4 did not rule on its wording. The default, applied: on the owner's word, this
  spec's branch narrows its firing clause to a fix other than the respellings
  `spec@plans@path-quickfixes` defines, a reference that dangles when the spec leaves and so forces
  the harvest to judge it again; at the harvest, the tripwire is deleted, since its decision is
  reversed; the rewritten head is guarded by the parked thread's tripwire, per D2, while T1
  guards `design@check@fix-scope` and T2 the head that records the question form.
  The alternative: the tripwire stays as written until the harvest, which rewrites it.
- **D5**, from the transcript review. The owner's direction of round 2, "I want to avoid catering to
  this project's own needs when designing the checker", is a standing direction about the
  checker's design, not about this work alone, and it has no home outside this spec. A goal is the
  owner's to word. The default: at the harvest, the direction is put to the owner as a draft goal
  of the core, under `skill@knowledge-architect-goal-setting`. The alternative: put to the owner
  now, in this spec's branch.
- **D6**, from the decision-record, transcript and cold-implementer reviews. The census scripts
  were in the session's scratch directory, which no table of the knowledge homes covers. The
  default: the scripts are not kept; the census method is described under Names; the head
  `design@check@safe-fix-definition` rests on `argument@path-quickfixes@arg-20`, a mechanism, and the
  census figures go in the harvest's commit message, where a measurement that serves no head is
  routed. The alternative: the owner names a home for the scripts.

## Harvest

At step 7, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's
commit, with the test it fails.

| item | home |
| --- | --- |
| `thread@path-quickfixes@fix-admission` | `design@check@safe-fix-definition` rewritten in place: the principle in its title, its tests, the respelling and the inference; `argument@path-quickfixes@arg-20` as its argument, and the census figures in the harvest's commit message, per D6 |
| `thread@path-quickfixes@respelling-fixes` | `design@check@fix-scope` rewritten: the three kinds of fix, the respellings listed as the table of "The respellings", the shapes not respelled; `design@check@backticked-bare-id` rewritten, its title and its sentence that name renaming the skill as the repair of an ordinary-word match, and its census clause, per `argument@path-quickfixes@arg-30` |
| `thread@path-quickfixes@fix-run-order` | `design@check@fix-before-the-checks` rewritten with the order of "The order of a run"; the sentence and the exit clause of `design@check@fix-refusal-mixed-state` named in "What is already decided" rewritten; the sentence of `design@check@phases-gate-the-report` named there rewritten; `design@check@fix-before-the-checks` read again for the order of a `--fix` run |
| `thread@path-quickfixes@skill-cited-by-path` | a head whose title states that a skill or an agent has no `path` spelling: a new head, or `design@model@harness-kinds-cited-without-anchor` with its title rewritten to state it, as `primer@design-heads` and `skill@knowledge-architect-decision-recording@head-shape` decide; with `argument@path-quickfixes@arg-15` |
| `thread@path-quickfixes@raw-path-inference` | a head whose title states the question form: a new head, or `design@check@unanchored-path-first-segment-here` with its title rewritten to state it, as `primer@design-heads` and `skill@knowledge-architect-decision-recording@head-shape` decide |
| `thread@path-quickfixes@head-states-the-principle` | nothing: its question is decided by `design@agent-skills@title-states-the-rule` |
| `thread@path-quickfixes@same-anchor-inference` | per D2 |
| `tripwire@check@fix-makes-a-choice` | per D4 |
| the owner's direction of round 2 on this repository's own needs | per D5 |
| every item of "Losing alternatives", the incumbent included | `register@check@rejected`, each as the recording tests admit, cited by the head whose argument covers it |
| the existing entries of `register@check@rejected` that a rewritten head cites | each read again; each still covered by its head's argument as rewritten, or the citation moves to the head that covers it |
| T1 | `register@check@tripwire`, guarding `design@check@fix-scope`, judged at the harvest by the standard of `skill@knowledge-architect-decision-recording@ruled-tripwires` alone, on the wording it has then, a drop listed under "Decided"; the owner's ruling to keep it admits it no more than the standard does |
| T2 | `register@check@tripwire`, guarding the head that records the question form, judged as T1 is |
| AC1 to AC4, and AC1 of the register-references work | reported on at the landing; each spent, or, if it recurs, judged as a cause by `skill@knowledge-architect-decision-recording@ruled-tripwires` |
| the respelling of a register's home | `design@check@fix-scope` with the other respellings; the two exclusions that give way for a home target, in the head that states them |
| this spec | deleted in a commit of its own after the review before the merge, cited as `spec@plans@path-quickfixes` |

## Later consequences

- A later fix of `--fix` is admitted by the tests of "What a fix may write", not by a list. A fix
  that would need an inference reopens `thread@path-quickfixes@fix-admission`.
- The raw-path finding stays the most frequent one this work does not repair. The owner accepted
  that in round 3; `thread@path-quickfixes@same-anchor-inference` is where it is argued again.
- `issue@check@tooling-for-project-skills`, once closed with a prefix check, is the narrowing T1's
  response names for the bare-name respelling.
