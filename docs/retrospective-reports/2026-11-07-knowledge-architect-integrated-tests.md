# 2026-11-07-knowledge-architect-integrated-tests

- **Received file**: 2026-11-07-knowledge-architect-integrated-tests.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-172-g5a956d02`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: none beyond those its sections cite.

## P1: the list of places that name every published crate lives in several texts

Analysed with P1 of 2026-11-01-knowledge-architect-release-0-9-0, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-release-0-9-0.md`.

## P2: the plan's load-bearing point carried a premise as a constraint without the measurement that would test it

- **a.** What it says: the spec listed among its constraints "the binary's tests stay in `cli`, the
  one package where `CARGO_BIN_EXE_klarch` is set", and the session put the fork with it standing; the
  owner's question led to a measurement, a test can build the binary with the cargo that runs it, and
  the constraint fell, with the shape the owner had chosen. The file calls it no defect of an
  instruction: `instructions@verify-before-relying` and the primer cover it.
- **b.** The spec left with its milestone (`00533a5c`). The instruction stands:
  `primer@verify-before-relying`, "Establish it from the implementation, or from a run you observe",
  and `primer@owner-word-premise`, "A premise the session supplied is the session's to check before
  the owner rules on it".
- **d.** Its kind: a lapse of a clear instruction, one instance; the file proposes nothing.
- **Default**: no change, a lapse.
- **Outcome**: decided by the session: no change. Ground: the instructions exist and are clear; a
  lapse seen once is not repaired by adding text, per
  `skill@knowledge-architect-agent-configuration@admission-standard`.

## P3: the test helpers leak directories into the temporary directory

Analysed with C2 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-transcript-issues-klarch-workflow.md`.

## P4: no scratch location named, and the session's work put in the temporary directory

Analysed with C2 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-transcript-issues-klarch-workflow.md`.
