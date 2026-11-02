# Slice 3: the installed skill for creating or splitting a Component, and its fragment "when to split"

## Builds

1. **The installed skill for creating or splitting a Component**, named
   `knowledge-architect-component-creation`, source
   `planned@agent-skills@content/skills/component-creation/`. The name meets
   `design@agent-skills@naming-rule`: an activity noun in two words, "creation" covering a split,
   which creates the Components it cuts out; the rival "component-splitting" names half the
   activity. It holds, in prose, with no `Cargo.toml` block, since a block in a skill alone is
   checked by nothing, per `design@agent-skills@rust-templates-one-home`:
   - the default criteria `goal@knowledge-architect@projects-are-split-into-components` asks for and
     does not list: one purpose, an interface with a reason to change on its own, its own tests, and
     decisions of its own with a design of them, from the owner's R3 notes and the owner's answer
     during slice 0, with "one reason to change" as the test for a Component as for a unit;
   - the intermediate Component, which holds no Rust code of its own, groups nested Components, and
     holds the design and the documents several of them share but the rest of the project does not.
     Which Components exist and how they depend on each other stays in the root's design home, as
     the owning-Component rule the decision-recording skill delivers says;
   - the split signals of `thread@development-and-components@component-split-signals`;
   - the crate mapping by publication status of `thread@development-and-components@component-crate-mapping`.
     The skill reads the project's publication status from its record; where the record states
     none, it is a plan only the owner holds, and goes to the owner, per `primer@question-authority`.
     The mapping weighs the project's versioning and publishing policy, which stays the project's,
     per `goal@knowledge-architect@agents-get-a-complete-workflow`, and never sets it;
   - the shapes: one crate; a module-directory Component, whose directory carries the Component's
     documents, so whether a published crate ships them is the project's packaging choice; nested
     Components for layers; the shared test Component of
     `thread@development-and-components@shared-test-component`, with its condition; the pure and
     output layers of `thread@development-and-components@pure-output-layer`, stated as the default
     code shape of the development skill at the scale of a Component, pointing to it and
     restating none of it;
   - the naming rule of `thread@development-and-components@component-naming`, as a recommendation,
     per the owner's R6 "can also be recommended" and
     `goal@knowledge-architect@serves-one-kind-of-project`: a directory never named after the
     project, since the checker refuses it, and a package prefix the project chooses for its own
     packages, apart from its skill prefix;
   - the basename rule of `thread@development-and-components@component-basename-collision`, one
     sentence stating the checker's refusal, beside the other names the checker refuses for an
     anchor (the project's name, a kind's name and the reserved words), at the moment a Component is
     named, before the manifest is written. Its ground is the owner's R6, "forbidding it and
     documenting that": the rarer checker rules are otherwise delivered by their finding, per
     `design@agent-skills@checker-model-in-primer`, and this one is stated because the name is
     chosen before any finding can show;
   - the procedure, each step pointing to its skill and restating none of it:
     - the decision to create or split goes through `skill@knowledge-architect-design` and its
       search of the record, per `design@agent-skills@record-searched-before-the-work`, and who rules
       it is `primer@question-authority`: a split the criteria and the record settle is argued and
       reported, a fork between two surviving splits goes to the owner;
     - `{{command}} new-component`, and `skill@knowledge-architect-setup` for declaring the
       Component, whose section on Components and locations points to this skill for the criteria
       and the naming;
     - the goals home's purpose line and the goals, a split's moved goals included, through
       `skill@knowledge-architect-goal-setting`, ruled by the owner;
     - a split moves each head, `rejected` entry, issue and tripwire whose subject the new Component
       holds, which Component owns each by the owning-Component rule; the move adds and removes no
       decision, per `primer@design-heads`; `{{command}} check` reports every reference into a moved
       entry or file, since a reference names its anchor, and each is re-pointed;
   - where a Component's purpose is written: the head of its goals home, as the purpose its goals
     serve, which `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`
     finds missing in four homes of this repository;
   - a Component kept against the default criteria is a decision about the layout, which the
     owning-Component rule places in the root's design home, through the design skill. Slice 0 took
     it out of the goal, on the owner's word that the rules of splitting go to the design record and
     the skill; the skill states no rule of its own on when a head is written, per
     `design@agent-skills@one-home-for-head-rules`, and the default criteria are guidelines a
     Component does not fail alone, per `goal@knowledge-architect@projects-are-split-into-components`.

   Its description triggers at a design discussion, in development, at setup and at a project
   audit, per `thread@development-and-components@component-skill-delivery`. At an audit, the moment
   is a finding of the design-record axis on the Component a head sits in, the one axis that reads
   which Component owns what; the axes that would read a Component's purposes are unbuilt, per
   `issue@agent-skills@audit-axes-beyond-the-design-record`, which stays open.
2. **The shared fragment "when to split"**, `planned@agent-skills@fragments/when-to-split.md`: the
   signals, which a session judges, and a pointer to the component skill, hosted by the design
   skill and the development skill, per `design@agent-skills@shared-text-fragment`. It joins
   `FRAGMENT_HOSTS` in `path@agent-skills@src/lib.rs`, with both hosts, whose test checks the list
   against the placeholders. In the design skill it stands in a section of its own, outside step 1
   of the loop, whose order of grounding `design@agent-skills@search-after-first-challenge` fixes;
   in the development skill, beside the default code shape. The fragment proposes no split: a
   request of the owner to split still loads the design skill first.
3. **The primer's list of installed skills** gains the component skill, at its moments.
   The retrospective's clause on splitting a project into Components, which the second "met while"
   item of `goal@knowledge-architect@projects-are-split-into-components` asks for, is written by
   slice 2, per `thread@development-and-components@retrospective-code-rules-question`; this slice
   writes none.
4. **The setup skill's section on Components and locations** keeps the declaration and the command,
   and points to the component skill for the criterion and the naming it states today, so each rule
   keeps one home.
5. **The agentic-workflow audit's L2 lens** gains the activity in one of its groups, in
   `path@agent-skills@content/workflows/agentic-workflow-audit.js`: "design, component creation,
   planning, decision recording and issue tracking".
6. **`design@core@component-command`** is re-pointed: what a Component owes beyond its files is the
   component skill's, named by its reference, in the commit that creates the skill.
7. **No issue closed here:** `a-skill-for-creating-a-component` was closed when this milestone scheduled it.
8. **`cargo klarch install-agent-skills`**, the CHANGELOG.md entries in `Next release`, and
   `cargo x changelog`. The Workflow entry names the activity, creating or splitting a Component,
   so a consumer with a skill of its own for it sees the overlap, per
   `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, which stays open; no Migration
   entry, since no consumer is known to hold such a skill. Each source is staged with its installed
   copy, per `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`.
9. **The plan documents' names** of the new skill and fragment become references in the commit that
   creates them: each planned path becomes a path reference, per `design@core@planned-path-form`,
   and the bare skill name `knowledge-architect-component-creation` becomes a skill reference.

## Claims

No run is built to observe an agent following the new text, per
`design@agent-skills@synthetic-evidence-not-built`.

- **The skill and the fragment install, the fragment appears whole in both hosts, and each
  level-two heading carries a slug.** Test: `cargo klarch install-agent-skills`, `cargo klarch check`,
  and a reading of the installed design and development skills. Shown to fail: a host that does not
  include the fragment, as a recorded mutation of the host's source.
- **The skill restates nothing of the setup skill's declaration of a Component, and the setup skill
  states no criterion or naming rule of its own.** Test: a reading of both, per
  `goal@agent-skills@no-contradiction-in-workflow`. Shown to fail: the setup skill's criterion and
  naming sentence as they stand before this slice, which the reading finds.
- **The basename rule the skill states matches the checker.** Test: the checker's test
  `two_anchors_sharing_a_name_keep_the_first_declared`, which already exists; the skill's sentence is
  read against `resolve_anchors` in `path@core@src/manifest.rs`, for the names it refuses.
- **Each Cargo fact the skill states is checked.** Test: Cargo's documentation, read at the audit
  and cited in the skill's `%%` lines:
  - the Cargo Book, "Specifying Dependencies", "Local paths in published crates": "Crates that use
    dependencies specified with only a path are not permitted on crates.io", dev-dependencies
    excepted;
  - the Cargo Book, "Dependency Resolution", "dev-dependency cycles": "Usually the resolver does not
    allow cycles in the graph, but it does allow them for dev-dependencies";
  - the same section: in a library's unit tests, "there are actually two copies of the library
    linked into the final test binary", whose types "are not compatible".

  Shown to fail: a fact the documentation does not state is removed or reworded. The three were
  also measured with cargo 1.98.0 at the audit.

## Audit subjects

- `goal@knowledge-architect@projects-are-split-into-components`, `goal@agent-skills@one-skill-per-activity`,
  `goal@agent-skills@no-contradiction-in-workflow`.
- `skill@knowledge-architect-setup@components-and-locations` and
  `skill@knowledge-architect-setup@component-documents`; `design@core@component-command`,
  `design@core@components-carry-the-same-documents`, `design@core@anchor-collisions-refused`,
  `design@core@agents-table`, `design@agent-skills@goals-required`.
- `design@agent-skills@shared-text-fragment`, `design@agent-skills@naming-rule`,
  `design@agent-skills@adopting-existing-docs` and `design@agent-skills@audit-triggers`, for the
  triggers at setup and at an audit.
- `a-skill-for-creating-a-component` whole, closed when this milestone scheduled it, read in the parent of the commit that deleted it, which
  `git log --diff-filter=D -- crates/agent-skills/docs/open-issues/a-skill-for-creating-a-component.md` finds; and
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`, the issue the owner recalled
  for `thread@development-and-components@pure-output-layer`.
- `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`,
  `issue@agent-skills@a-lighter-pass-for-untouched-components` and
  `rejected@core@split-enforced-by-a-check`, for premortem cause 6.
- thaum's testing crate, `path@elsewhere@~/dev/games/thaum/tools/thaum-testing/`, and its
  `Cargo.toml` files, for the shape of `thread@development-and-components@shared-test-component`.
- `design@knowledge-architect@no-component-directory-named-after-the-project`.

## Fails alone on

- A shape or a signal the skill states that contradicts the goal or the setup skill.
- The fragment missing from a host, or differing between the two.

## Premises that expire

- **The development skill exists** to host the fragment: slice 2 built it.
- **The checker refuses two anchors with one name.** If a change of the core admits it, the skill's
  basename rule is false; the checker's test above is the guard.

## Decided design

### The crate mapping by publication status

Thread: `thread@development-and-components@component-crate-mapping`.

**Shape.** One Component is one crate by default. A Component may be a module directory inside a
crate, where a crate would force publication or a dependency cycle.

| project | one Component per crate | crate count |
| --- | --- | --- |
| publishes, or plans to | default | a low cost, weighed against how usable each crate is alone for others |
| never publishes | a stronger default: the crate boundary is the stronger isolation | no cost |

**Argument.** The goal already holds the owner's points, so "one reason to change" is the test for a
function and for a Component alike (`argument@development-and-components@arg-33`); the owner's
weighting by publication (`argument@development-and-components@arg-41`,
`argument@development-and-components@arg-46`); Rust practice accepts many small published crates,
at a cost in release order and version bumps (`argument@development-and-components@arg-36`).

**Nearest rival and the fact that defeated it.** Always one crate: a published crate can depend on
no crate published only by path, dev-dependencies excepted, and Cargo's resolver refuses a cycle
outside dev-dependencies, two facts read in Cargo's documentation at the audit.

### The split signals

Thread: `thread@development-and-components@component-split-signals`.

**Shape.** Signals against a split: it needs many new dependencies, it would create a cycle, or every
expected change crosses the boundary. Nested Components carry the layering where there are many side
crates.

**Argument.** The owner's R3 notes (`argument@development-and-components@arg-29`,
`argument@development-and-components@arg-34`). No rival was offered.

### The pure and output layers

Thread: `thread@development-and-components@pure-output-layer`.

**Shape.** A Component's logic returns data, a struct, and a separate layer of code prints it or
serialises it in a format a machine reads. By default, both layers sit in the Component that defines
the struct.
A Component of its own holds the output side only where that side takes decisions of its own, as a
graphical interface that decides a layout does: a Component that takes no decision has no design of
its own, on the owner's word during slice 0.

**Argument.** The owner's R3 note: "Separating pure logic from non pure routines with side effects is
very good for long term health of a project, I have noticed." The owner's answer during slice 0,
verbatim in the milestone document's thread item `thread@development-and-components@pure-output-layer`:
the printing "should probably" be a function in the crate or Component that returns or defines the
struct, or a Component would take no decision and have no design record, which a Component
requires. No rival was offered.

### The shared test Component

Thread: `thread@development-and-components@shared-test-component`.

**Shape.** An offered shape, not a rule: a test Component that depends on the Components it tests,
never the reverse, holding the cross-Component tests with their fixtures and oracles. It applies
where several crates share testing tools and fixtures without depending on each other; the case of a
crate A whose tests need a crate B that already depends on A is its example.

**Argument.** Thaum's testing crate depends on thaum's engine and AI crates and is depended on only
by its bench tool (`argument@development-and-components@arg-47`); the owner: it centralises tooling
"thus improving the quality of tests, and it also avoids circular dependencies"
(`argument@development-and-components@arg-43`); the condition as the general case
(`argument@development-and-components@arg-51`, `argument@development-and-components@arg-53`).

**Nearest rival and the fact that defeated it.** Shared helpers behind a `test-utils` feature, or a
dev-dependency back onto the testing crate: in the library's unit tests it links two copies of the
crate under test, whose types do not match, as Cargo's documentation states.

### Delivery: a new skill, a fragment in design and development, nothing in planning

Thread: `thread@development-and-components@component-skill-delivery`.

**Shape.** A Component is created or split at four moments: new work that may belong to no existing
Component, in a design discussion; a second reason to change found while adding code, in development;
an adopting project's crates mapped to Components, at setup; a Component with two purposes found by
an audit, today through the design-record axis's finding on the Component a head sits in. The
fragment "when to split", a few lines of signals and a pointer, is hosted by the design and
development skills; the component skill holds the rest, and its description triggers at setup and at
an audit too. Planning holds nothing of it.

**Argument.** The owner's R4 proposal (`argument@development-and-components@arg-42`), extended to the
four moments (`argument@development-and-components@arg-48`). The skill points to the setup skill and
restates none of it, per `goal@agent-skills@no-contradiction-in-workflow`. The activity is new, its
methodology apart from the design discussion's and the setup's, and the owner took it in at R4 and
R7, so `goal@agent-skills@one-skill-per-activity` gives it one skill; the goal's list of activities
"grows with the workflow", and no goal edit is owed.

**Nearest rival and the fact that defeated it.** Planning as the place: whether a Component exists is
a decision about how the project is built, which `primer@design-heads` places in design.

### The basename rule

Thread: `thread@development-and-components@component-basename-collision`.

**Shape.** Two Components never share a basename; the checker refuses two anchors with one name, and
the skill states it.

**Argument.** Closed `argued` (`argument@development-and-components@arg-75`): the refusal stands in
`resolve_anchors` of `path@core@src/manifest.rs`, with its test. The owner: "Two components with the
same basename are either the same one, or they do not have a specific enough name." (R6)

**Nearest rival and the fact that defeated it.** Supporting two Components with one basename: no
value, on the owner's R6.

### The naming rule

Thread: `thread@development-and-components@component-naming`.

**Shape.** A recommendation: a Component's directory is named by what it does, never after the
project; the package name in `Cargo.toml` carries a prefix the project chooses for its packages.

**Argument.** The first half is forced: a directory named after the project collides with the root's
anchor, which the checker refuses, per
`design@knowledge-architect@no-component-directory-named-after-the-project`. The second half is a
convention whose benefit is short anchors in references (`argument@development-and-components@arg-73`,
`argument@development-and-components@arg-76`).

**Nearest rival and the fact that defeated it.** A directory named after its package: the collision
above, for the project's own name, as `rejected@knowledge-architect@crate-directory-named-after-project`
records.

## Harvest

- **Heads** in `register@agent-skills@design`, under `skill@knowledge-architect-decision-recording`,
  each recording the owner's words by attribution, never verbatim:
  - `thread@development-and-components@component-skill-delivery`;
  - `thread@development-and-components@component-crate-mapping`, with the module-directory
    Component, its exception, kept in the same head, per
    `tripwire@agent-skills@split-rule-from-exception`;
  - `thread@development-and-components@component-split-signals`,
    `thread@development-and-components@pure-output-layer` and
    `thread@development-and-components@shared-test-component`, each a head where the entry tests of
    `primer@design-heads` admit it and its nearest rival differs, otherwise `%%` lines at the
    skill's text. The split signals and the pure and output layers were approved in a batch with no
    rival offered, so a head for either first runs
    `skill@knowledge-architect-decision-recording@unargued-approval`;
  - `thread@development-and-components@component-naming`, its title stating the forced rule, no
    Component directory takes the project's name, and the rest as a recommendation, per
    `tripwire@agent-skills@rule-title-wider-than-its-argument`, citing
    `design@knowledge-architect@no-component-directory-named-after-the-project` and
    `rejected@knowledge-architect@crate-directory-named-after-project`;
  - the list of fragments in `design@agent-skills@shared-text-fragment` gains "when to split", a
    small block the owner's R4 proposal placed in two hosts; the lists of parts in
    `design@agent-skills@development-skill` and the design skill's head gain it where they list
    parts.
- **A head in `register@core@design`**, under its anchors subject, beside
  `design@core@reserved-anchors` and not folded into it, which
  `issue@core@core-heads-bundle-decisions-with-many-citing-sites` lists as bundled: two anchors the
  manifest resolves never share a name, citing `goal@core@documents-stay-current`, on the owner's R6
  argument that two Components with one basename are one Component or badly named, and the reference
  rule that a reference names its anchor by one word; entry test 1 of `primer@design-heads`, what
  the manifest accepts, admits it. It states nothing of the entry anchors of a directory register.
  `design@core@anchor-collisions-refused` refuses two anchors at one path, not two of one name, and
  no head states the refusal by name: only `resolve_anchors` in `path@core@src/manifest.rs` and its
  test hold it. Supporting two Components of one name earns no `rejected` entry: its refutation is
  derived again in one round, so it fails the third test of
  `design@agent-skills@losing-alternatives-filter`, which the harvest's commit names.
- **Rejected:** planning as the place of the component instructions, where
  `design@agent-skills@losing-alternatives-filter` admits it.
- **Tripwires:** premortem cause 6 of the milestone document, judged by the tripwire standard, and
  folded into `issue@agent-skills@a-lighter-pass-for-untouched-components` or dropped, with a line
  under "Decided".
- **Issues:** none closed; `a-skill-for-creating-a-component` was closed when this milestone scheduled it.
