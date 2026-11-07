# 2026-11-02-knowledge-architect-component-split-git-out

- **Received file**: 2026-11-02-knowledge-architect-component-split-git-out.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-59-g5c7bc807`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: none beyond those its sections cite.

## P1: the release skill counted the published crates and the version sites

Analysed with P1 of 2026-11-01-knowledge-architect-release-0-9-0, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-release-0-9-0.md`.

## P2, with P2 and P3 of 2026-11-04-knowledge-architect-model-crate: the milestone's obligations of a new crate, and its rule on which heads move

- **a.** What it says. P2 here: the milestone's "How the work is done" named the publish order in
  the release workflow and not its second list, the verify job's. P2 of model crate: the rule "A head
  that a slice's cut divides stays in the core" was in a spec that left, and not in the milestone;
  proposal, one bullet in the milestone. P3 of model crate: the milestone's list missed the root
  README's layout table; proposal, name it.
- **b.** Whether it still applies. No: the milestone has left, with the spec of its last slice, in
  the commit "[plans] The milestone that split the Components leaves, with the spec of its last
  slice: their work and their harvest have landed" (`00533a5c`); `register@plans@milestone` holds only
  its README and index. The general rule on which Component a moved head goes to is installed:
  `path@agent-skills@content/skills/component-creation/SKILL.md`, step 4 of a split, "each design
  head, `rejected` entry, issue and tripwire whose subject it holds, the Component that owns each
  decided by `skill@knowledge-architect-decision-recording@owning-component`", whose rule 1 sends a
  decision constraining more than one Component to the root. The release workflow's two lists stand
  at HEAD, and the cluster of P1 of 2026-11-01-knowledge-architect-release-0-9-0 takes them.
- **d.** Its kind: defects of a plan document's text, now gone.
- **Default**: no change, with the reason: the text left with the milestone; the lists are taken by
  the cluster of the release skill's lists.
- **Outcome**: decided by the session: no change. Ground: the blamed text no longer exists
  (`00533a5c`); the head-moving rule's durable home is
  `skill@knowledge-architect-decision-recording@owning-component`, which step 4 of a split names; the
  crate lists are analysed with P1 of 2026-11-01-knowledge-architect-release-0-9-0.

## P3: the design audit judged most graded entries from their grade lines

Analysed with W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## P4, with W4 of 2026-11-02-knowledge-architect-component-split-git-out-klarch-workflow: the rules on Components disagreed with each other and with the owner's rulings

- **a.** What it says: `skill@knowledge-architect-component-creation@split-signals`, "Where many
  Components would sit beside one another, nested Components under an intermediate Component carry
  the layering", beside `skill@knowledge-architect-component-creation@component-criteria`, "An
  intermediate Component holds no Rust code in its own files"; and point 1 of
  `skill@knowledge-architect-decision-recording@owning-component`, "how they depend on each other"
  to the root. The owner ruled the second prevails, and that point 1 holds of dependencies spanning
  several Components. Proposal: the revision the issue on the two rulings holds.
- **b.** Whether it still applies: repaired. 634b19e3, on main, "closes both" the issue on the two
  rulings of the split's first audit and the issue on the reversal rule, per its message; it removed
  "the sentence on layering by a count of Components" and gave point 1 "the dependency decisions
  that constrain more than one Component". At HEAD a search of
  `path@agent-skills@content/skills/component-creation/SKILL.md` for "carry the layering" finds
  nothing; at `5c7bc807` it was line 74. Lines 56 to 64 now state the intermediate Component by the
  purpose its nested Components serve, and `path@agent-skills@fragments/owning-component.md`, line 8,
  reads "constraining more than one of them". The repairs of its reviews, 0fe89dc5, 1bc7ced1, b37c91a7
  and 64041b42, are on main.
- **Default**: no change: already repaired.
- **Outcome**: decided by the session: no change. Ground: repaired by 634b19e3 and its review
  repairs, which closed the issue that held the owner's two rulings.
