---
name: knowledge-architect-development
description: MUST use before starting or iterating on any change to the project's Rust code — grounding in the record of the code being changed, specifying the unit of work as claims with tests that discriminate and showing each with `cargo x mutate`, the default shape of the code, the module guidelines and who splits a module, the gate, the commit, the review axes for code and when to dispatch them, the conditional performance step and the optional diff mutation sweep, and which comment species a sentence belongs in. Not for agent-facing text, a decision's record, or parking an item, which their own skills cover.
---

%% Its record is `design@agent-skills@development-skill`. It states no pull request and no continuous
%% integration, which the workflow ships only as offered shapes, and names no scoped CLAUDE.md, which
%% a target project may not have.
# Developing

Scope: writing the project's Rust code, from grounding to the commit and the reviews it owes. One
activity, and the facets below are its parts rather than separate scopes.

Not covered here: **agent-facing text**, `skill@knowledge-architect-agent-configuration`;
**dispatching a review**, `skill@knowledge-architect-review`; **recording a decision**,
`skill@knowledge-architect-decision-recording`; **parking anything**,
`skill@knowledge-architect-issue-tracking`; **a change the owner asks for or proposes**, which goes
first to `skill@knowledge-architect-design`, as the primer's list of installed skills says.

## Ground before editing {{slug:ground-before-editing}}

%% A fix runs no search of the whole record, per `design@agent-skills@code-change-reads-its-record`.
Before changing code, read what the project decided about it, as the primer's "Read the record of
code before changing it" says: the entries its comments cite, every entry that cites its file, and
the heads on its subject in the design home of the Component that holds it, with its `rejected`
entries.
%% The root's heads bind every Component or several of them, per the primer's section on the
%% Components a project declares, so a change inside one Component reads them too.
Read the root's design home too, since its heads bind every Component or several of them, and
those of the intermediate Components enclosing this one, per `primer@checker-model`. A fix reads this record
and dispatches no search of the whole record. A behaviour that looks like a new defect is checked
as the primer's "Check whether it is already known" says. How to read all of it is
`primer@intent-and-claims` and `primer@verify-before-relying`, read whole: intent is authority, and
a divergence between a design document and the code is a defect in one of them, never licence to
follow the code.

## The loop {{slug:development-loop}}

1. **Claims.** Take the unit of work's claims, each with the test that could refute it.
2. **Write the tests, and show that they discriminate** (`skill@knowledge-architect-development@discriminating-tests`).
3. **Implement.** A design decision met while building is routed by the backstop of
   `primer@design-heads`, at the moment it is met.
4. **Gate** (`skill@knowledge-architect-development@development-gate`).
%% The claim check restated in one sentence with its pointer: it binds at each commit and the primer
%% is read once at the start, per
%% `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`.
5. **Commit.** Before each commit, check every factual claim of its message against the tree the
   commit holds, as `primer@verify-before-relying` says in its subsection "Verify a claim before
   writing it".
6. **Review** (`skill@knowledge-architect-development@code-review-axes`), at any checkpoint where a
   coherent piece works, not only at the end. It follows the commit because a reviewer working on
   its own copy of the tree sees committed content only, so uncommitted work is reviewed by nobody.
   A repair the review asks for lands as `skill@knowledge-architect-review@what-review-leaves` says.

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

**Run the mutation with `cargo x mutate run`, never by hand.** A mutation applied by hand cannot
show that it reached the code the command compiled, nor that removing it removed nothing else. The
command shows both, and runs safely over uncommitted work:

```sh
cargo x mutate run --file <F> --replace <OLD> --with <NEW> \
  --build "cargo test -p <package> --no-run" -- cargo test -p <package> <module>::<test> -- --exact
cargo x mutate run --file <F> --patch <P.diff> --build "<probe>" -- <command>
cargo x mutate restore        # after a run that was killed
```

- **The verdict is the exit code of the command after `--`**, printed as `caught` or `SURVIVED`, or
  `unmutatable here` where the build probe failed and a different mutation is owed. It is never the
  exit code of `cargo x mutate run`, which is 0 for any completed run, 1 when something is owed on
  the tree, so look at it and run `cargo x mutate restore`, and 2 when the run never started.
- **The line `mutation applied to <file>, first changed line <n>` shows the mutation present.** A
  verdict without it is about code nobody mutated.
- **Pass `--build`**, one command split on whitespace and run with no shell, so a probe needing
  `&&`, a pipe or quoting runs as words: without a probe, a mutant that does not build reads as `caught`, and the sites
  a mutation check most often sends you to are the ones that do not compile. Leaving it off weakens
  the check rather than shortening it.
- **The baseline runs by default** and refuses a command that already fails, since over a red base
  every mutation reads as `caught`; `--no-baseline` skips it only for a failure that is expected.
- **The command names the test under check by its module path, with `-- --exact` after it.** A
  filter that matches no test runs nothing, cargo exits 0, and the mutation reads as `SURVIVED`;
  `--exact` before the second `--` is refused by cargo, so the baseline refuses the run.
- **A wrong implementation that spans several files** is beyond one run; the claim then records why
  its test cannot be shown to discriminate.

Where the project's maintenance crate carries no `mutate` command, add it as the setup skill's
section on a Rust project shows, before the first check.

**A recorded check quotes** the `-` and `+` lines of the mutation, the file and the function or
type that holds them, and the verdict, naming the failing test for a catch. A claim already settled
by a probe beforehand enters as a regression guard, and is recorded as such.

{{fragment:default-code-shape.md}}

### A module with a signal for a split

{{fragment:module-split.md}}

%% Who splits a module and when: a split planned before the work, per
%% `design@agent-skills@default-code-shape`; its record, a comment and the move's commit message
%% unless a test of the primer's design-head rules holds.
**Shaping the code the change writes stays yours under the default shape**, a new module for it
included. **Moving code the change does not write between modules, or out of one, is a split**,
the move of the guidelines above: a step of the plan document where one carries the work, and
otherwise commits of its own before the work, on the same branch. State the split's reason in a
comment at the module and in the message of the move's commit. A signal for a split over code the
work does not write, which no such step took, is not acted on inside that work: route it as
`primer@met-outside-the-task` says.

### A Component with a second reason to change

{{fragment:when-to-split.md}}

### Unreachable states, and which of them earn a test

A guard, a refusal or an `expect` often stands against a state nothing can currently produce.

- **The language or a format makes it unreachable**, for the life of the project: a type that cannot
  hold the value, a format the parser guarantees. No test: the state would have to be constructed,
  and the result asserts something about the construction. **Name the guarantee.** A justification
  phrased as a property of the current code makes it one of the next two kinds.
- **A validated boundary makes it unreachable**: whatever checks untrusted input refuses every state
  that would reach the code. There is no state to construct and run, so what is tested is **the
  refusal**, and the mutation is deleting it.
- **Only the current implementation makes it unreachable.** It stops being so at whichever unit of
  work populates that region, and anything shipped under it has no coverage at the moment it starts
  mattering. **Test it**, constructing the state where the boundary accepts one.

**The operational test is to ask the boundary.** If it refuses every state that reaches the code,
the guard is the second kind, and the refusal is what carries the claim. If it accepts such a state, the guard is the third kind, whatever an
argument about normal use says.

**A mutation set the author of the tests chose is not enough.** The author mutates the thing the
claim is about, and the survivors are adjacent to it, where an independent reviewer looks: the
adversarial review chooses its own mutations.

## The review axes for code {{slug:code-review-axes}}

%% The cadence is `design@agent-skills@review-cadence`, and where each row sits
%% `design@agent-skills@code-axes-rows-home`.
**Dispatch when a coherent piece of code compiles, passes its tests, and does what its claims say**,
and a defect found after the next piece is built on it would mean unbuilding both. How to dispatch is
`skill@knowledge-architect-review`: read it before sending anything. The review before a merge owes
the axes of its table, the code-shape and code-claims axes among them; this activity adds:

| axis | dispatched as | when |
| --- | --- | --- |
| adversarial | `agent@knowledge-architect-adversarial-reviewer` | at each checkpoint, for any change to the code |
| spec-first | `agent@knowledge-architect-spec-first-reviewer` | at each checkpoint, where the project implements an external specification and the change touches how |

%% Scoping is `design@agent-skills@adversarial-scoping`.
**Scope each adversarial reviewer.** When a change spans more than one subsystem or boundary,
dispatch several, each scoped to one subsystem, one family of claims, or one boundary such as a
crate's public surface. Each brief names its scope and its own worktree, and says nothing of what
to find. The scopes may overlap; a gap between them is the dispatcher's to justify.

%% The step is `design@agent-skills@diff-mutation-sweep`, opt-in for its cost; the caveats are from
%% cargo-mutants' book, chapters "Testing code changed in a diff" and "Limitations".
**An optional diff mutation sweep**, where the project runs it: before the adversarial review,
`cargo mutants --in-diff <diff>` over the branch's changed code, its survivors given to the
adversarial reviewers as leads, never as findings. It costs one build and one test run per mutant. A
diff that touches only test code yields no mutant; a diff whose new side differs from the tree
swept is refused, so it is taken from committed work, `git diff <base> HEAD`; and cargo-mutants
ignores the rust flags set in a cargo configuration file, so a project that sets flags there runs the sweep with builds that
differ from its gates. It replaces no recorded mutation check: it chooses its own mutants.

## Performance on a declared path {{slug:declared-performance-paths}}

%% The step is `design@agent-skills@performance-opt-in`.
A project may declare a code path performance-critical in one of its design heads, naming the paths
it covers and the project's own command that measures it, for example:

```text
### The engine's step loop is performance-critical `##step-loop-is-hot`
Paths: crates/engine/src/runtime/step/
Measure: cargo x bench measure step
```

A change that touches a declared path measures before and after with that command, and its commit
message records both figures. The review checks that the figures are recorded, and judges no speed
by reading code.

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
  do, its errors and its panics, and the pitfalls of using it. It does not explain the
  implementation: a reader who never opens the body must get everything they need from it.
- **An inline comment addresses the next developer of this code**: what the code is meant to do and
  why it is shaped that way, so the intent can be recovered when it turns out wrong. **Give the
  domain of validity explicitly**: the cases the code handles, and why it holds for them. Write the
  current intent, not the episode. A past-tense reference earns its place only where a reader would
  otherwise re-introduce the defect, and then it names the mechanism, not the fix. What a comment
  must reference is `primer@where-knowledge-goes`.

**Where a comment ends and a document begins** is `primer@design-heads`: if changing the code would
force a change to the design home, it is design; otherwise it is a comment.

## A shipped tool found insufficient {{slug:local-tool-improvement}}

%% The loop is `design@agent-skills@upstream-tool-loop`.
Where a tool knowledge-architect ships, which the project does not maintain, falls short of the
work, the project may build the
improvement as a tool of its own beside the shipped one, never a patch of the shipped one, and
reports it in the workflow file of `skill@knowledge-architect-retrospective`. The project's own tool
stays until the improvement ships.
