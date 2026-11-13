# Check design: Plan documents

## Plan documents

### A plan document's README.md and deliberation.md owe the sections of their kind, and a slice's README.md owes its own `##plan-document-sections-by-kind`

The `README.md` of a spec of specs/ and of a milestone owes the plan sections, in order, matched as
the documents write them: Status and audience, How the work is done, Names, What the work is, What
is already decided, Criteria, "New names, in one place", Decided design, Mapping tables, Losing
alternatives, Readings, Premortem, Acceptance criteria, Implementation sequence, Order rationale,
Defaults awaiting the owner, Harvest, Later consequences. The `README.md` of a slice owes Builds,
Claims, Audit subjects, Fails alone on and Premises that expire, in order; the plan sections a
slice's `README.md` holds, for the design only its slice builds, are not ordered against them. A
`deliberation.md`, a plan document's or a slice's, owes Threads, then Arguments, and a section
Threads or Arguments in any plan's `README.md` is a finding: the deliberation sits apart from the
documents the search reads, per `design@model@plan-items-by-section`. One `spec` plan type owes two
lists for its `README.md`, chosen by where the spec sits, under specs/ or in a milestone.

Three things are owed under a condition no check can read, so none is checked: Fixtures, only where
a Component drives its tests with authored content; a slice's "What is already decided", only where
it has entries to seed; and a `deliberation.md`, only where the document has a thread or an
argument. The files of a harvested slice owe nothing: they were judged while live, and are read for
their definitions alone, per `design@model@a-harvested-slice-is-frozen`.

The lists are the planning skill's, which owns the shape of a plan document, per
`design@agent-skills@design-hands-off-to-planning`; a change to them changes this head.
