---
name: knowledge-architect-development
description: MUST use before starting or iterating on any change to the project's Rust code — grounding in the record of the code being changed, specifying the unit of work as claims with tests that discriminate and showing each with `cargo x mutate`, the default shape of the code, the gate, the commit, the review axes for code and when to dispatch them, the conditional performance step and the optional diff mutation sweep, and which comment species a sentence belongs in. Not for agent-facing text, a decision's record, or parking an item, which their own skills cover.
---

%% The activity is the developing of `goal@knowledge-architect@agents-get-a-complete-workflow`; the
%% skill is ported from this repository's own development skill and from thaum's, per
%% `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`. Its record is
%% `design@agent-skills@development-skill`.
# Developing

Scope: writing the project's Rust code, from grounding to the commit and the reviews it owes. One
activity, and the facets below are its parts rather than separate scopes.

Not covered here: **agent-facing text**, `skill@knowledge-architect-agent-configuration`;
**dispatching a review**, `skill@knowledge-architect-review`; **recording a decision**,
`skill@knowledge-architect-decision-recording`; **parking anything**,
`skill@knowledge-architect-issue-tracking`; **a change the owner asks for or proposes**, which goes
first to `skill@knowledge-architect-design`, as the primer's list of installed skills says.

## Ground before editing {{slug:ground-before-editing}}

%% A fix runs no search of the whole record, per `design@agent-skills@code-change-reads-its-record`:
%% the owner's statement recorded with it is that grounding in the record concerns additions and
%% new features, not fixes, and the search costs a dispatch of many agents.
Before changing code, read what the project decided about it, as the primer's "Read the record of
code before changing it" says: the entries its comments cite, every entry that cites its file, and
the heads on its subject in the design home of the Component that holds it, with its `rejected`
entries. Read the root's design home too: its heads bind every Component. A fix reads this record
and dispatches no search of the whole record. A behaviour that looks like a new defect is checked
as the primer's "Check whether it is already known" says.

## The loop {{slug:development-loop}}

1. **Claims.** Take the unit of work's claims, each with the test that could refute it.
2. **Write the tests, and show that they discriminate** (`skill@knowledge-architect-development@discriminating-tests`).
3. **Implement.** A design decision met while building is routed by the backstop of
   `primer@design-heads`, at the moment it is met.
4. **Gate** (`skill@knowledge-architect-development@development-gate`).
5. **Commit.** Before each commit, check every factual claim of its message against the tree the
   commit holds, as `primer@verify-before-relying` says in its subsection "Verify a claim before
   writing it".
6. **Review** (`skill@knowledge-architect-development@code-review-axes`), at any checkpoint where a
   coherent piece works, not only at the end. It follows the commit because a reviewer working on
   its own copy of the tree sees committed content only.

## Claims, and tests that discriminate {{slug:discriminating-tests}}

**A unit of work is a list of claims, each with the test that could refute it.** A part with no
claim is cost with no information. Where the work is a slice of a milestone or a step of a spec, the
Claims its plan document gives it present them; otherwise the session writes them.

**Every claim's test is shown to fail against a plausible wrong implementation.** A test written
first can still be written to the implementation its author has in mind, and then it passes on its
first run with nothing at risk. Two ways to show it:

- **Observed failing before its code exists.** Available when the answer is not yet known.
- **A recorded mutation check.** When the trap is known before the test or the code is written,
  both come out right on the first run; introducing the wrong implementation on purpose to watch a
  red bar is theatre. Changing the code afterwards, running the test, and recording whether it
  caught the change is the same evidence, obtained honestly. **Name the mutation, not just the
  outcome.**

%% The tool and its guarantees are the mutation library's own record; thaum measured three of five
%% hand-made checks invalid in one unit of work, which is why the step is a command.
**Run the mutation with `cargo x mutate run`, never by hand.** A mutation applied by hand cannot
show that it reached the code the command compiled, nor that removing it removed nothing else. The
command shows both, and runs safely over uncommitted work:

```sh
cargo x mutate run --file <F> --replace <OLD> --with <NEW> \
  --build "cargo test -p <package> --no-run" -- cargo test -p <package> <test> --exact
cargo x mutate run --file <F> --patch <P.diff> --build "<probe>" -- <command>
cargo x mutate restore        # after a run that was killed
```

- **The verdict is the exit code of the command after `--`**, printed as `caught` or `SURVIVED`,
  never the exit code of `cargo x mutate run`, which is 0 for any completed run.
- **The line `mutation applied to <file>, first changed line <n>` shows the mutation present.** A
  verdict without it is about code nobody mutated.
- **Pass `--build`** whenever the mutant might not compile: without a probe, a mutant that does not
  build reads as `caught`.
- **The baseline runs by default** and refuses a command that already fails, since over a red base
  every mutation reads as `caught`; `--no-baseline` skips it only for a failure that is expected.
- **The command names the test under check, with its module path and `--exact`.** A filter that
  matches no test runs nothing, cargo exits 0, and the mutation reads as `SURVIVED`.
- **A wrong implementation that spans several files** is beyond one run; the claim then records why
  its test cannot be shown to discriminate.

Where the project's maintenance crate carries no `mutate` command, add it as the setup skill's
section on a Rust project shows, before the first check.

**A recorded check quotes** the `-` and `+` lines of the mutation, the file and the function or
type that holds them, and the verdict, naming the failing test for a catch. A claim already settled
by a probe beforehand enters as a regression guard, and is recorded as such.

{{fragment:default-code-shape.md}}

### Unreachable states, and which of them earn a test

A guard, a refusal or an `expect` often stands against a state nothing can currently produce.

- **The language or a format makes it unreachable**, for the life of the project: a type that cannot
  hold the value, a format the parser guarantees. No test: the state would have to be constructed,
  and the result asserts something about the construction. **Name the guarantee.** A justification
  phrased as a property of the current code makes it one of the next two kinds.
- **A validated boundary makes it unreachable**: whatever checks untrusted input refuses every state
  that would reach the code. What is tested is **the refusal**, and the mutation is deleting it.
- **Only the current implementation makes it unreachable.** It stops being so at whichever unit of
  work populates that region. **Test it**, constructing the state where the boundary accepts one.

**The operational test is to ask the boundary.** If it refuses every state that reaches the code,
the guard is the second kind. If it accepts such a state, the guard is the third kind, whatever an
argument about normal use says.

**A mutation set the author of the tests chose is not enough.** The author mutates the thing the
claim is about, and the survivors are adjacent to it, where an independent reviewer looks: the
adversarial review chooses its own mutations.

## The review axes for code {{slug:code-review-axes}}

%% The cadence is `design@agent-skills@review-cadence`: the checkpoint axes here, the read-only code
%% axes in the review skill's table, once per branch.
**Dispatch when a coherent piece of code compiles, passes its tests, and does what its claims say**,
and a defect found after the next piece is built on it would mean unbuilding both. How to dispatch is
`skill@knowledge-architect-review`: read it before sending anything. The review before a merge owes
the axes of its table, the code-shape and code-claims axes among them; this activity adds:

| axis | dispatched as | when |
| --- | --- | --- |
| adversarial | `agent@knowledge-architect-adversarial-reviewer` | at each checkpoint, for any change to the code |
| spec-first | `agent@knowledge-architect-spec-first-reviewer` | at each checkpoint, where the project implements an external specification and the change touches how |

%% Scoping is `design@agent-skills@adversarial-scoping`, thaum's refinement; the owner reported
%% that it reduced the time a reviewer takes enough, with no cap that might stop it finding more.
**Scope each adversarial reviewer.** When a change spans more than one subsystem or boundary,
dispatch several, each scoped to one subsystem, one family of claims, or one boundary such as a
crate's public surface. Each brief names its scope and its own worktree, and says nothing of what
to find. The scopes may overlap; a gap between them is the dispatcher's to justify.

%% The step is `design@agent-skills@diff-mutation-sweep`, opt-in for its cost; the caveats are from
%% cargo-mutants' book, chapters "Testing code changed in a diff" and "Limitations".
**An optional diff mutation sweep**, where the project runs it: before the adversarial review,
`cargo mutants --in-diff <diff>` over the branch's changed code, its survivors given to the
adversarial reviewers as leads, never as findings. It costs one build and one test run per mutant. A
diff that touches only test code yields no mutant, and cargo-mutants ignores the rust flags set in
a cargo configuration file, so a project that sets flags there runs the sweep with builds that
differ from its gates. It replaces no recorded mutation check: it chooses its own mutants.

## Performance on a declared path {{slug:declared-performance-paths}}

%% The step is `design@agent-skills@performance-opt-in`: a performance claim without a measurement
%% cannot be reproduced, and the owner held that a benchmark procedure, not a review, guards it.
A project may declare a code path performance-critical in one of its design heads, naming the paths
it covers and the project's own command that measures it, for example:

```text
### The engine's step loop is performance-critical `##step-loop-is-hot`
Paths: crates/engine/src/runtime/step/
Measure: cargo x bench measure step
```

A change that touches a declared path measures before and after with that command, and its commit
message records both figures. No review judges speed by reading code. A project that declares no
path pays nothing.

## The gate {{slug:development-gate}}

Run the project's one gates command, which the setup skill recommends, before each commit that
could fail it and before the review: its exit code says whether every gate passes. Read its output
in the terminal or from a file; never filter it through a pipe, as the primer's section on the
checker says of a verdict.

## Incompleteness is recorded, never encoded {{slug:incompleteness-recorded}}

Every unit of work is wrong about everything later work builds. A guard, a refusal or a special case
added to mark work as unbuilt singles out one incompleteness among many and gives it a mechanism.
Within a single session that mechanism is indistinguishable from a decision, and it is harder to
remove than a note: deleting a refusal looks like relaxing a guarantee, so the session that finishes
the work needs an argument it does not have.

Unbuilt work belongs in a plan document or in the issue registers, whose entries leave. Code says
what is true. A statement in code that is false, such as a doc comment claiming a capability the
crate lacks, is corrected, which differs from enforcing the lack.

## Comments: two species, two audiences {{slug:comment-species}}

Neither carries the history of a change: that lives in the commit message.

- **A doc comment addresses the caller**: what someone using the type or function must respect, the
  invariants it keeps, what it expects of its arguments and of the object's state, what it does not
  do, its errors and its panics. It does not explain the implementation: a reader who never opens
  the body gets everything they need from it.
- **An inline comment addresses the next developer of this code**: what the code is meant to do and
  why it is shaped that way, so the intent can be recovered when it turns out wrong. **Give the
  domain of validity explicitly**: the cases the code handles, and why it holds for them. Write the
  current intent, not the episode. A guard, a workaround, a stub or a test that exists because of an
  issue names it, as `primer@where-knowledge-goes` says.

**Where a comment ends and a document begins** is `primer@design-heads`: if changing the code would
force a change to the design home, it is design; otherwise it is a comment.

## A shipped tool found insufficient {{slug:local-tool-improvement}}

%% The loop is `design@agent-skills@upstream-tool-loop`: a project that no longer owns a tool may
%% stop improving it, and thaum's mutation tool was built because outside tools fell short.
Where a tool knowledge-architect ships falls short of the work, the project may build the
improvement as a tool of its own beside the shipped one, never a patch of the shipped one, and
reports it in the workflow file of `skill@knowledge-architect-retrospective`. The project's own tool
stays until the improvement ships.
