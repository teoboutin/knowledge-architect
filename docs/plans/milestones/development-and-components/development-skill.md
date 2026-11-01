# Slice 2: the installed development skill, its review axes, and the local skill cut down

## Builds

1. **The installed skill `knowledge-architect-development`**, source
   `planned@agent-skills@content/skills/development/`, ported from `skill@klarch-development` with
   thaum's refinements, cutting this repository's particulars. Its parts:
   - **ground before editing**: the record of the code being changed, as the primer's "Read the record
     of code before changing it" says, and no search per fix (#fix-grounding); it names no scoped
     `CLAUDE.md` (`argument@development-and-components@arg-64`);
   - **claims with tests that discriminate**: every added test names its claim, and a test of logic
     carries a recorded mutation (#test-economy); the mutation is run with `cargo x mutate`, built by
     `spec@development-and-components@mutation-crate`;
   - **implement**: a design decision met while building is routed by the backstop of
     `primer@design-heads`;
   - **gate**: the project's one gates command;
   - **commit**: the claim check restated in one sentence with its pointer to
     `primer@verify-before-relying` (#commit-claim-check);
   - **review at checkpoints and before the merge**, per #review-cadence, with #adversarial-scoping;
   - **incompleteness is recorded, never encoded**;
   - **comment species**, pointing to the primer for the guard-reference rule and restating none of it
     (`argument@development-and-components@arg-64`);
   - **the conditional performance step** of #performance-opt-in;
   - **the optional diff mutation sweep** of #diff-mutation-sweep, with the caveat on
     the cargo configuration file;
   - **the wording of #upstream-tool-loop**: a project may build an improvement of a shipped tool as a
     tool of its own beside the shipped one, and reports it in the retrospective's workflow file.

   It states no pull request and no continuous integration (criterion #no-pr-or-ci-assumed). Each new
   instruction names its real-use evidence in its `%%` lines (`argument@development-and-components@arg-64`).
2. **Two new installed agents**: `knowledge-architect-code-shape-reviewer`, source
   `planned@agent-skills@content/agents/code-shape-reviewer.md`, judging #code-shape-axis with
   #test-economy; and `knowledge-architect-spec-first-reviewer`, source
   `planned@agent-skills@content/agents/spec-first-reviewer.md`, judging #spec-first-axis.
3. **`agent@knowledge-architect-code-claims-reviewer` widened** to every statement a diff writes about
   code, and to the completeness of the documentation of a public interface others consume
   (#code-claims-scope, #api-doc-completeness).
4. **The review skill's axis table** gains the rows the development skill adds, with their conditions
   and their cadence, in `skill@knowledge-architect-review@review-axes`, or in the development skill's
   own table that the review skill's sentence on an activity's added axes already reaches; the audit
   chooses the one that keeps each row in one home.
5. **The setup's linter ask** gains `missing_docs`, `clippy::missing_errors_doc` and
   `clippy::missing_panics_doc` for a published crate, in `skill@knowledge-architect-setup`, and in a
   snippet where one carries the workspace's lints, per `design@agent-skills@rust-templates-one-home`.
6. **The retrospective's standing question** of #retrospective-code-rules-question, in
   `skill@knowledge-architect-retrospective@what-it-examines`, watching the goal slice 0 rewords.
7. **The primer's list of installed skills** gains the development skill, at its moment, per
   `design@agent-skills@installed-skill-takes-precedence`.
8. **`skill@klarch-development` cut** to what is particular to this repository, an overlay that only
   adds (`argument@development-and-components@arg-64`): among them the cargo-configuration nesting trap,
   "No test may run the gates over this repository", and the core's first-commit placement; its
   worktree procedure replaced by `cargo x mutate`. `design@knowledge-architect@scratch-worktree-outside-git-rules`
   and the texts citing it rewritten where they name that procedure.
9. **Issues**, under `skill@knowledge-architect-issue-tracking`:
   - closed: the developing part of
     `issue@knowledge-architect@the-workflow-has-no-skill-for-developing-branch-work-or-releasing`, which
     stays open for branch work and releasing; `issue@agent-skills@a-fix-is-grounded-by-no-record-search`;
     `issue@agent-skills@the-code-claims-axis-reads-plan-documents-only`;
     `issue@agent-skills@a-reviewer-s-running-time-is-unbounded`;
   - opened: a `todo` for improving performance as an activity of its own (#performance-review-axis); a
     `todo` for #use-feedback-collection; a `deferred` for #upstream-co-development;
   - kept open, its other options intact:
     `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`.
10. **`cargo klarch install-agent-skills`**, the CHANGELOG.md entries in `Next release`, and
    `cargo x changelog`.

## Claims

No run is built to observe an agent following the new text, per
`design@agent-skills@synthetic-evidence-not-built`. The claims are of the text and of the install.

- **The installed skill, the two agents and the widened code-claims reviewer install, and each
  level-two heading carries a slug.** Test: `cargo klarch install-agent-skills`, then
  `cargo klarch check`, and the core's test that holds the shipped set to
  `design@core@section-homes-carry-slugs`. Shown to fail: a heading of the new skill without its
  slug, as a recorded mutation of the source.
- **The installed skill states no pull request, no continuous integration and no scoped
  `CLAUDE.md`, and restates no rule of the primer beyond one sentence with its pointer.** Test: a
  reading of the installed skill whole, recorded in the commit; a phrase search is a lead only.
  Shown to fail: the same reading of `skill@klarch-development` today finds each of
  `instructions@git-workflow`, a scoped `CLAUDE.md` and the guard-reference rule restated.
- **The installed skill and the new agents cite no entry of this repository**, per
  `design@agent-skills@shipped-text-cites-no-entry`. Test: a search of their sources for a reference
  whose anchor is a Component of this repository, every hit read; no check catches one, as
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` records. Shown to fail: the same
  search over `skill@klarch-development` today finds such references.
- **Every row the review owes for code names its agent, its condition and its moment, in one home.**
  Test: a reading of the review skill and the development skill together. Shown to fail: the review
  skill today names no axis for code.
- **The setup's lints are named where the setup skill asks for a linter, and a snippet that carries
  them builds.** Test: the snippet test of `design@agent-skills@rust-templates-one-home`, if a snippet
  carries them. Shown to fail: a lint name misspelt in the snippet, as a recorded mutation.
- **`skill@klarch-development` only adds to the installed skill.** Test: a reading of both, each
  instruction of the local one checked against the installed one for a contradiction. Shown to fail:
  the local skill today holds the manual worktree procedure, which the installed skill replaces.

## Audit subjects

- `skill@klarch-development` whole, and thaum's skills thaum-development and thaum-review, in
  `path@elsewhere@~/dev/games/thaum/.claude/skills/`, whole.
- `skill@knowledge-architect-review` whole, `skill@knowledge-architect-setup@rust-project` and
  `skill@knowledge-architect-setup@setup-gates`, `skill@knowledge-architect-retrospective@what-it-examines`,
  `agent@knowledge-architect-code-claims-reviewer`.
- The adversarial axis: the local skill dispatches it as a fresh subagent briefed by its own row.
  Whether it becomes an installed agent too is read against `design@agent-skills@additions-review-agent`
  and `rejected@agent-skills@one-generic-auditor-agent`; the discussion did not decide it.
- The heads and issues "What is already decided" sends to slice 2's audit, among them
  `design@agent-skills@plan-read-against-the-record`, `design@agent-skills@code-change-reads-its-record`,
  `design@knowledge-architect@scratch-worktree-outside-git-rules`,
  `design@agent-skills@premortem-as-watch-points`, `design@agent-skills@installed-fix-goes-upstream`,
  `issue@agent-skills@a-head-s-measurement-says-nothing-of-its-magnitude`,
  `issue@agent-skills@patching-an-installed-skill`,
  `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`,
  `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`,
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`.
- `design@agent-skills@naming-rule` for the skill's and the agents' names.
- The reworded goal of slice 0, and `goal@knowledge-architect@agents-get-a-complete-workflow`.
- cargo-mutants' book, chapters "Testing code changed in a diff" and "Limitations", for the diff
  sweep and its caveat on the cargo configuration file, read at the audit since the discussion read them once.

## Fails alone on

- A missing or contradicting instruction of the installed development skill, which the agent-facing
  text axis or a reading of the local and installed skills together reports.
- An agent that does not install, or an axis whose row names no agent, condition or moment.

## Premises that expire

- **The local skill keeps the manual worktree procedure** until this slice; this slice ends it.
- **The development skill names no component split** until slice 3 adds the fragment "when to split"
  to it.
- **The adversarial reviewers' running time**: the fix is the scoping, per the owner's R6; if
  scoped reviewers still run unbounded in real use, the closed issue's mechanism returns, which a
  retrospective reports.

## Decided design

### The code-shape axis

Thread: `thread@development-and-components@code-shape-axis`.

**Shape.** An installed read-only agent that judges each unit the diff writes or edits against the
default shape the skill states, which `goal@knowledge-architect@the-workflow-leads-to-better-code`
asks for and does not list: one responsibility, read as one reason to change; the logic pure,
the side effects in a thin layer that decides nothing a logic test should cover; the logic tested
without the side effects where the change adds or changes logic; and a departure argued, where the
review reports it: in code the change writes, the writer restructures it or argues it at the code;
in an existing unit the change edits, only the lines the change writes are weighed, and the rest of
the unit is a lead, never an obligation, on the owner's approval of Q6 during slice 0. A finding of two reasons to change names two concrete changes someone could
plausibly ask for, shows that each would touch the unit independently, and shows that they are
entangled. Fragmentation is a finding too: a split that buys nothing, such as a function with one
caller, no test of its own and a name that says no more than its body.

**Argument.** The code-quality goal asks for a review that judges the shape
(`argument@development-and-components@arg-1`); "one reason to change" is "much more testable and
lenient" (R3), and the reading "does one thing" leads to code "where everything is a function" (R2,
`argument@development-and-components@arg-14`, `argument@development-and-components@arg-19`).

**Nearest rival and the fact that defeated it.** A general quality reviewer: it has no standard to
judge against (`argument@development-and-components@arg-3`). A general-purpose reviewer briefed with
the standard: `rejected@agent-skills@one-generic-auditor-agent` (`argument@development-and-components@arg-57`).

### Test economy, in the code-shape reviewer

Thread: `thread@development-and-components@test-economy`.

**Shape.** Every test the diff adds names its claim, for any kind of test. Mutation evidence is
required for tests of logic only; for a golden test, a test with heavy mocks or a nondeterministic
test, the session picks the form of evidence. The code-shape reviewer reports a test with no claim,
or a test of logic with no recorded discrimination check:

| test kind | discrimination evidence | economy question |
| --- | --- | --- |
| pure logic | a named mutation | does it map to a claim |
| golden or snapshot | mutate the code that produces the output; it fails, but that tells little | does another golden test already cover the same claim |
| heavy mocks | mutate the logic under test, never the mock; a test that only checks a pass-through to the mock tests the side-effect layer | that is usually the finding: the goal says the side-effect layer needs no tests |
| nondeterministic or external | none is reliable | the session's judgement |

**Argument.** Agents "add lots of tests on a whim" (R1); whether a mutation applies depends on
whether a plausible wrong implementation exists in the logic a test exercises
(`argument@development-and-components@arg-20`); the owner's R2 doubt on mocks and golden tests
(`argument@development-and-components@arg-15`).

**Nearest rival and the fact that defeated it.** A separate axis: one more dispatch to read the same
diff (`argument@development-and-components@arg-4`).

### Adversarial reviewers scoped

Thread: `thread@development-and-components@adversarial-scoping`.

**Shape.** Several adversarial reviewers may be dispatched, each scoped to one subsystem, claim
family or boundary; scopes may overlap, and the dispatcher justifies a gap. Each reviewer chooses its
own mutations, picks the tests that matter for each, runs them, and runs the wider suite before
reporting a survivor. No cap on their running time.

**Argument.** Thaum's refinement (`argument@development-and-components@arg-2`); the owner's R6: it
"has proven to be enough of a fix, by sufficiently reducing the time adversarial reviewers take to
report, without adding constraints or a cap that might prevent them from finding more"
(`argument@development-and-components@arg-70`, `argument@development-and-components@arg-78`).

**Nearest rival and the fact that defeated it.** A cap on a reviewer's running time: the owner's R6
words above.

### The spec-first axis

Thread: `thread@development-and-components@spec-first-axis`.

**Shape.** An installed read-only agent that works out what an external specification the project
implements requires, before reading the code, then compares. It runs at development checkpoints
where the project implements such a specification and the diff changes how.

**Argument.** The generic form of thaum's rules-first axis (`argument@development-and-components@arg-2`);
a project with no specification never triggers it, which meets criterion
#works-without-performance-or-spec with no opt-in (`argument@development-and-components@arg-6`).

**Nearest rival and the fact that defeated it.** A briefed general-purpose reviewer:
`rejected@agent-skills@one-generic-auditor-agent`.

### The code-claims reviewer widened, with the completeness of public documentation

Threads: `thread@development-and-components@code-claims-scope` and
`thread@development-and-components@api-doc-completeness`.

**Shape.** The reviewer reads every statement a diff writes about code, doc comments, READMEs, issue
entries and heads, against the tree, as well as plan documents. Where the diff changes a public
interface others consume, it also asks of each public item the diff adds or changes whether its
documentation states what a caller must respect: invariants, argument expectations, errors, panics,
what it does not do. The presence of documentation is the lints `missing_docs`,
`clippy::missing_errors_doc` and `clippy::missing_panics_doc`, in the setup's linter ask for a
published crate.

**Argument.** It closes `issue@agent-skills@the-code-claims-axis-reads-plan-documents-only`, which
records a thaum case where an improvised reviewer for this question produced "the review's most
consequential findings" (`argument@development-and-components@arg-8`); completeness "matters mostly
for documentation of public facing code/API" (R2); the reading of content uses the same method, at
no extra dispatch (`argument@development-and-components@arg-24`).

**Nearest rival and the fact that defeated it.** A separate documentation axis: it would duplicate
the code-claims agent's method.

### Performance, a conditional step on declared paths

Thread: `thread@development-and-components@performance-opt-in`.

**Shape.** A target project declares a performance-critical path in one of its design heads, with the
paths it covers and the project's own command that measures it. A change that touches a declared
path measures before and after, and its commit message records both figures. A review checks that
the figures are recorded and does not judge speed by reading. A project that declares no path pays
nothing. The declaration is a design head (Q2); a manifest key comes only if sessions miss the step
in real use. An illustration of the shape, not a text to copy:

```text
### The engine's step loop is performance-critical `##step-loop-is-hot`
Paths: crates/engine/src/runtime/step/
Measure: cargo x bench measure step   (the project's own command)
```

**Argument.** A performance claim without a measurement cannot be reproduced, so a reviewer would
drop its own findings under `skill@knowledge-architect-review@review-invariants`
(`argument@development-and-components@arg-9`); the owner's R2: "for performance non-regression,
review is not needed, a benchmark procedure is better".

**Nearest rival and the fact that defeated it.** A performance review axis: withdrawn, see the
milestone document's losing alternatives.

### The review cadence

Thread: `thread@development-and-components@review-cadence`.

**Shape.** The adversarial and spec-first axes run at development checkpoints, as the local skill's
adversarial axis does today. The code-shape axis, with test economy, and the code-claims axis are
read-only and run once per branch, in the review before the merge, before the transcript reviewer,
per `design@agent-skills@transcript-review-last-before-merge`.

**Argument.** At most two more dispatches per branch, with no build; the instrument that would show
whether this is too many is `issue@agent-skills@the-retrospective-counts-no-review-cost`
(`argument@development-and-components@arg-10`).

**Nearest rival and the fact that defeated it.** Every axis at every checkpoint: the review cost the
owner asked to keep low, criterion #review-cost.

### The optional diff mutation sweep

Thread: `thread@development-and-components@diff-mutation-sweep`.

**Shape.** A project may opt in to a step that runs `cargo mutants --in-diff` over the branch's
changed code before the adversarial review, and gives its survivors to the adversarial reviewers as
leads. The skill states two caveats from cargo-mutants' book: a diff touching only test code produces
no mutants, and cargo-mutants ignores rust flags set in the cargo configuration file, `path@knowledge-architect@.cargo/config.toml` in this repository, so a project that sets
flags there gets mutants built differently from its gates.

**Argument.** cargo-mutants and the mutation command do different jobs: a sweep for survivors in
changed code, against a mutation the session writes (`argument@development-and-components@arg-21`).
It costs one build and one test run per mutant, so it is opt-in (`argument@development-and-components@arg-22`,
`argument@development-and-components@arg-26`).

**Nearest rival and the fact that defeated it.** cargo-mutants as a replacement for the mutation
command: it offers no mutation the session writes itself.

### The retrospective's standing question on rules about code

Thread: `thread@development-and-components@retrospective-code-rules-question`.

**Shape.** The retrospective always asks, worded as the agent proposed in the reply to R3: "Did an
installed rule about code or tests prove too hard or too strict to follow in this session's Rust
work, or get followed against the change's interest?" It names the reworded goal it watches, per
`design@agent-skills@premortem-as-watch-points`.

**Argument.** It is the "learn from use" half of the reworded goal's mechanism, and fits
`goal@knowledge-architect@the-workflow-improves-through-real-use` (`argument@development-and-components@arg-28`,
`argument@development-and-components@arg-32`). No rival was offered.

### A shipped tool improved locally, as a tool of the project's own

Threads: `thread@development-and-components@upstream-tool-loop` and
`thread@development-and-components@upstream-co-development`.

**Shape.** A session that finds a shipped tool insufficient may build the improvement as a tool of the
project's own beside the shipped one, never a patch of the shipped one, and reports it in the
retrospective's workflow file; the project's own tool stays until the improvement ships. Co-development
through a Cargo `[patch]` on a knowledge-architect branch waits for a first real need: a `deferred`
issue whose trigger is "a consumer needs a change to a shipped tool before the next release", with
the unverified premise that a patched build's version must match the consumer's `checker-version`
pin.

**Argument.** A consumer that no longer owns a tool may stop improving it, and thaum's mutation tool
was built because outside tools were not up to the task (`argument@development-and-components@arg-30`);
`design@agent-skills@installed-fix-goes-upstream` gives "what the project adds" to the project
(`argument@development-and-components@arg-57`).

**Nearest rival and the fact that defeated it.** A fix of the shipped tool inside the project: the
same head sends such a fix to the workflow's file only.

### A fix runs no record search

Thread: `thread@development-and-components@fix-grounding`.

**Shape.** The development skill's grounding step is the reading of
`design@agent-skills@code-change-reads-its-record`: the record of the code being changed. A fix
dispatches no record search. New features and additions go through the design skill's grounding and
its search, per `design@agent-skills@settled-means-recorded`.

**Argument.** The issue records the owner's statement that grounding in the design record concerns
additions and new features, not bug fixes; the issue's search took 10 agents, and this discussion's
18 (`argument@development-and-components@arg-58`).

**Nearest rival and the fact that defeated it.** A search per fix: its cost, and the owner's statement
above.

### The claim check restated at the commit

Thread: `thread@development-and-components@commit-claim-check`.

**Shape.** The commit step restates the primer's claim check for a commit message in one sentence,
with its pointer to `primer@verify-before-relying`, which `primer@where-knowledge-goes` allows.

**Argument.** The check binds at each commit and is read once at the start
(`argument@development-and-components@arg-59`); the restatement is no longer than a pointer.

**Nearest rival and the fact that defeated it.** A line printed by `check --staged` or by `commits`:
not decided here; the issue stays open for it.

## Mapping tables

The review axes the development skill dispatches, over every axis it names:

| axis | dispatched as | moment | condition |
| --- | --- | --- | --- |
| adversarial | as the audit decides, per "Audit subjects" | development checkpoints | any change to the code |
| spec-first | `knowledge-architect-spec-first-reviewer` | development checkpoints | the project implements an external specification and the diff changes how |
| code shape, with test economy | `knowledge-architect-code-shape-reviewer` | the review before the merge | the diff writes or edits code |
| code claims | `agent@knowledge-architect-code-claims-reviewer` | the review before the merge, and the plan-document moments as today | the diff writes statements about code; its completeness reading where the diff changes a public interface others consume |
| diff mutation sweep | cargo-mutants, no reviewer | before the adversarial review | the project opted in |

## Losing alternatives

The general quality axis, the security axis, the separate documentation and test economy axes, the
performance review axis and the briefed general-purpose reviewers are in the milestone document's
"Losing alternatives", since they bear on more than this slice's threads.

## Harvest

- **Heads** in `register@agent-skills@design`, under `skill@knowledge-architect-decision-recording`:
  one per decision the entry tests admit among #code-shape-axis, #test-economy, #adversarial-scoping,
  #spec-first-axis, #code-claims-scope with #api-doc-completeness, #performance-opt-in,
  #review-cadence, #diff-mutation-sweep, #upstream-tool-loop; rewritten in place:
  `design@agent-skills@code-change-reads-its-record` (#fix-grounding),
  `design@agent-skills@plan-read-against-the-record` where it places the code-claims reviewer,
  `design@agent-skills@premortem-as-watch-points` (#retrospective-code-rules-question); in
  `register@knowledge-architect@design`, `design@knowledge-architect@scratch-worktree-outside-git-rules`.
- **Rejected** in `register@agent-skills@rejected`, where `design@agent-skills@losing-alternatives-filter`
  admits it: the performance review axis, the general quality axis, the security axis, the separate
  documentation and test economy axes.
- **Tripwires:** premortem causes 1, 2, 3 and 7 of the milestone document, judged by the tripwire
  standard.
- **Issues:** as item 9 of "Builds" says.
- **Plan document:** each `planned` reference of this milestone's documents to a file this slice
  creates becomes a `path` reference in the commit that creates it.
