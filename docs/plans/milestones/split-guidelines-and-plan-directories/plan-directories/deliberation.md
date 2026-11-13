# The threads and arguments of the slice on plan documents as directories

## Threads

### Every plan document is a directory: a spec, a milestone, and each slice inside its milestone `##plan-document-directory`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-9` and
  `argument@split-guidelines-and-plan-directories@arg-11`; shaped by the session in round 1,
  `argument@split-guidelines-and-plan-directories@arg-27`, on the measurement of
  `argument@split-guidelines-and-plan-directories@arg-20`.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-45`: adopting projects migrate their
  plan documents, and `init` writes the new layout; `new-component` writes no plan document.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, a spec as one file,
  has no place for the artifacts. The owner's word on the checkpoint table, attributed in the
  milestone's Threads, left it standing.
- Its shape: this spec's "Decided design", and the milestone document's "The plan document's
  directory". Harvest: this slice's, into `design@model@spec-file-is-an-anchor`,
  `design@model@milestone-entries-are-directories`, `design@agent-skills@milestone-is-a-directory`
  and the other heads of Group A.

### Plan documents get model types of their own in `model`, rather than riding on the generic Directory shape `##plan-model-first-class`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-14`; shaped by
  the session in round 1, `argument@split-guidelines-and-plan-directories@arg-31`.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-45`.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, extending the
  Directory shape, collides with the owner's ruling that a declared Directory register carries one
  register, `design@model@declarable-directory-register`. The owner's word on the checkpoint table
  left it standing.
- Its shape: this spec's "Decided design". Harvest: this slice's, into `design@model@plan-register`,
  `design@model@plan-document-kinds`, `design@model@entry-anchor-carries` and the other model heads
  of Group A, with `rejected@model@subdirectory-anchors-location` read again.

### The first slice migrates the existing plan documents: `spec@plans@path-quickfixes` minimally, this milestone properly `##plan-migration-in-first-slice`

- Proposed by the owner in round 2, `argument@split-guidelines-and-plan-directories@arg-36`, answering
  Q3, whose default the session had put in round 1, `argument@split-guidelines-and-plan-directories@arg-33`.
- Final state: `approved`.
- The owner's answer, round 2: Q3's default found right for now, with the migration of
  `argument@split-guidelines-and-plan-directories@arg-36`. Closed by the owner's word on the
  checkpoint table.
- Its shape: this spec's "Builds", "This repository", and its mapping table. Harvest: none of its
  own; the migration is work, and its decision is recorded by the heads of Group A.

## Arguments

### Plan documents as directories hold artifacts of their own `##arg-11`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@plan-document-directory`.
The owner thought a spec should be a directory, not one file, since standalone specs show the same
defect and become very large documents, and since a directory could hold separate data as
artifacts too, such as the transcript.

### Specs and milestones as first-class models, leaving the Directory shape as it is `##arg-14`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@plan-model-first-class`.
On the implementation, the owner thought the project should consider giving plan documents, specs
and milestones, and their directory structure, an arm of the model of their own, not reusable,
since they diverge from the other registers, with many additional items, and are being changed
again. The owner liked the Directory shape available to a project's own registers, and judged that,
rather than changing it, giving specs and milestones first-class models of their own in the model
crate is a better direction.

### A directory reopens the rival of `design@model@spec-file-is-an-anchor`, on artifacts with no other home `##arg-27`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@plan-document-directory`.
"Every plan document becomes a directory. This reopens the rival recorded in
`design@model@spec-file-is-an-anchor` ("Making every spec a directory was the rival"). The new
argument is that a plan document now has artifacts with no other home."

### The milestone already departs from the declared Directory shape in three ways `##arg-31`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@plan-model-first-class`.
"The investigator mapped about 15 code sites across five crates that already special-case plans. The
milestone already departs from the declared Directory shape in three ways: it carries five registers,
where a declared Directory register carries one, "on the owner's ruling"; it has no `path` kind; its
sections are compiled into the checker. Extending the generic shape instead would collide with that
ruling. One consequence: `rejected@model@subdirectory-anchors-location` partly rests its argument on
the milestone needing the Directory shape, so that argument has to be read again." The investigator
was a subagent of round 1 whose report no file keeps; the milestone document's "What the work is"
re-counts, with a command, the files that name the plan model's constants: 10 files of four crates.

### One milestone, plans first, so the later slices are planned in the new shape `##arg-33`

Round 1, the session, putting Q3. Bears on
`thread@split-guidelines-and-plan-directories@plan-migration-in-first-slice`. "the default is one
milestone, slices in this order: 1. plan documents: model, checker, planning skill; 2. split
guidelines and the two scouts; 3. pre-harvest conformance; 4. the Component argument, and the goal.
Plans go first so the later slices are planned in the new shape. The catch: this milestone's own
README would be written in the old shape, so it would be about as large as the ones it fixes."

### A proper migration of this milestone is the first real test of the new structure `##arg-36`

Round 2, the owner. Bears on
`thread@split-guidelines-and-plan-directories@plan-migration-in-first-slice`. The owner said that the
first slice would also need to migrate the existing plans, `spec@plans@path-quickfixes` and the
milestone written in that session: the migration of `spec@plans@path-quickfixes` kept minimal, that
of the milestone perhaps not, since doing it properly would also be a first real test of the new
structure.

### Adopting projects migrate their plan documents too `##arg-45`

Round 3, the session. Bears on `thread@split-guidelines-and-plan-directories@plan-document-directory`
and `thread@split-guidelines-and-plan-directories@plan-model-first-class`. "**Adopting projects must
migrate their plan documents too.** Every pin crossing the release that makes plan documents
directories gets a Migration entry, and `init` and `create` read the new plan model."
