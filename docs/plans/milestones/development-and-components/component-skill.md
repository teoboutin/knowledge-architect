# Slice 3: the installed skill for creating or splitting a Component, and its fragment "when to split"

## Builds

1. **The installed skill for creating or splitting a Component**, default name
   `knowledge-architect-component-creation`, source
   `planned@agent-skills@content/skills/component-creation/`; the audit checks the name against
   `design@agent-skills@naming-rule`. It holds:
   - the default criteria `goal@knowledge-architect@projects-are-split-into-components` asks for and
     does not list: one purpose, an interface with a reason to change on its own, its own tests, and
     decisions of its own with a design of them, from the owner's R3 notes and the owner's answer
     during slice 0, with "one reason to change" as the test for a Component as for a unit;
   - the intermediate Component, which holds no Rust code of its own, groups nested Components, and
     holds the design and the documents several of them share but the rest of the project does not;
   - the split signals of #component-split-signals;
   - the crate mapping by publication status of #component-crate-mapping;
   - the shapes: one crate; a module-directory Component; nested Components for layers; the shared
     test Component of #shared-test-component, with its condition; the pure and output layers of
     #pure-output-layer;
   - the naming rule of #component-naming;
   - the basename rule of #component-basename-collision, documenting the checker's refusal;
   - the procedure: `klarch new-component`, the goals through `skill@knowledge-architect-goal-setting`,
     and a pointer to `skill@knowledge-architect-setup` for declaring the Component, restating none of
     it;
   - the guideline that a Component departing from the default criteria states why in a head of its
     own design home. Slice 0 took it out of the goal, on the owner's word that the rules of splitting
     go to the design record and the skill.

   Its description triggers at a design discussion, in development, at setup and at a project audit,
   per #component-skill-delivery.
2. **The shared fragment "when to split"**, `planned@agent-skills@fragments/when-to-split.md`: the
   signals and a pointer to the component skill, hosted by the design skill and the development
   skill, per `design@agent-skills@shared-text-fragment`. It joins `FRAGMENT_HOSTS` in
   `path@agent-skills@src/lib.rs`, with both hosts, whose test checks the list against the
   placeholders.
3. **The primer's list of installed skills** gains the component skill, at its moments.
   The retrospective's clause on splitting a project into Components, which the second "met while"
   item of `goal@knowledge-architect@projects-are-split-into-components` asks for, is written by
   slice 2, per #retrospective-code-rules-question; this slice writes none.
4. **`issue@agent-skills@a-skill-for-creating-a-component`** closed.
5. **`cargo klarch install-agent-skills`**, the CHANGELOG.md entries in `Next release`, and
   `cargo x changelog`.

## Claims

No run is built to observe an agent following the new text, per
`design@agent-skills@synthetic-evidence-not-built`.

- **The skill and the fragment install, the fragment appears whole in both hosts, and each
  level-two heading carries a slug.** Test: `cargo klarch install-agent-skills`, `cargo klarch check`,
  and a reading of the installed design and development skills. Shown to fail: a host that does not
  include the fragment, as a recorded mutation of the host's source.
- **The skill restates nothing of the setup skill's declaration of a Component.** Test: a reading of
  both, per `goal@agent-skills@no-contradiction-in-workflow`. Shown to fail: no earlier state exists;
  the reading is recorded with its result.
- **The basename rule the skill states matches the checker.** Test: the checker's test
  `two_anchors_sharing_a_name_keep_the_first_declared`, which already exists; the skill's sentence is
  read against `resolve_anchors` in `path@core@src/manifest.rs`.
- **Each Cargo fact the skill states is checked.** Test: Cargo's documentation, read at the audit
  and cited in the skill's `%%` lines: that a published crate depends only on published crates, that
  Cargo rejects cyclic package dependencies, and that a dev-dependency back onto a testing crate
  builds the crate under test twice with types that do not match. Shown to fail: a fact the
  documentation does not state is removed or reworded.

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
- `issue@agent-skills@a-skill-for-creating-a-component` whole, and
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`, the issue the owner recalled
  for #pure-output-layer.
- `issue@knowledge-architect@four-goals-homes-state-no-purpose-their-goals-serve`,
  `issue@agent-skills@a-lighter-pass-for-untouched-components` and
  `rejected@core@split-enforced-by-a-check`, for premortem cause 6.
- thaum's testing crate, `path@elsewhere@~/dev/games/thaum/tools/thaum-testing/`, and its
  `Cargo.toml` files, for the shape of #shared-test-component.
- `design@knowledge-architect@no-component-directory-named-after-the-project`.

## Fails alone on

- A shape or a signal the skill states that contradicts the goal or the setup skill.
- The fragment missing from a host, or differing between the two.

## Premises that expire

- **The development skill exists** to host the fragment: slice 2 builds it, and the milestone's order
  is the guard.
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

**Nearest rival and the fact that defeated it.** Always one crate: a published crate can depend only
on published crates, and Cargo rejects cyclic package dependencies, two facts the audit checks.

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
dev-dependency back onto the testing crate: it builds the crate under test twice, with types that do
not match across the two copies, a Cargo behaviour the audit checks.

### Delivery: a new skill, a fragment in design and development, nothing in planning

Thread: `thread@development-and-components@component-skill-delivery`.

**Shape.** A Component is created or split at four moments: new work that may belong to no existing
Component, in a design discussion; a second reason to change found while adding code, in development;
an adopting project's crates mapped to Components, at setup; a Component with two purposes found by
an audit. The fragment "when to split", a few lines of signals and a pointer, is hosted by the design
and development skills; the component skill holds the rest, and its description triggers at setup
and at an audit too. Planning holds nothing of it.

**Argument.** The owner's R4 proposal (`argument@development-and-components@arg-42`), extended to the
four moments (`argument@development-and-components@arg-48`). The skill points to the setup skill and
restates none of it, per `goal@agent-skills@no-contradiction-in-workflow`.

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

**Shape.** A Component's directory is named by what it does, never after the project; the package name
in `Cargo.toml` carries the project's prefix.

**Argument.** The first half is forced: a directory named after the project collides with the root's
anchor, which the checker refuses, per
`design@knowledge-architect@no-component-directory-named-after-the-project`. The second half is a
convention whose benefit is short anchors in references (`argument@development-and-components@arg-73`,
`argument@development-and-components@arg-76`).

**Nearest rival and the fact that defeated it.** A directory named after its package: the collision
above, for the project's own name.

## Harvest

- **Heads** in `register@agent-skills@design`, under `skill@knowledge-architect-decision-recording`:
  #component-skill-delivery; #component-crate-mapping with #component-split-signals,
  #pure-output-layer and #shared-test-component, as heads or as one head where the test of one
  decision per head allows; #component-naming, citing
  `design@knowledge-architect@no-component-directory-named-after-the-project`. In
  `register@core@design`, under its anchors subject, a head for #component-basename-collision
  recording that two anchors never share a name, on the owner's R6 argument "Two components with the
  same basename are either the same one, or they do not have a specific enough name." and the
  reference rule that a reference names its anchor by one word; entry test 1 of
  `primer@design-heads`, what the manifest accepts, admits it. `design@core@anchor-collisions-refused`
  refuses two anchors at one path, not two of one name, and no head states the refusal by name:
  only `resolve_anchors` in `path@core@src/manifest.rs` and its test hold it.
- **Rejected:** planning as the place of the component instructions, where
  `design@agent-skills@losing-alternatives-filter` admits it.
- **Tripwires:** premortem cause 6 of the milestone document, judged by the tripwire standard, and
  folded into `issue@agent-skills@a-lighter-pass-for-untouched-components` or dropped.
- **Issues:** `issue@agent-skills@a-skill-for-creating-a-component` closed.
