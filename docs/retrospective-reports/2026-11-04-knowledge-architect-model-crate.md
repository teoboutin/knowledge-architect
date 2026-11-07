# 2026-11-04-knowledge-architect-model-crate

- **Received file**: 2026-11-04-knowledge-architect-model-crate.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-109-g3f6074c8`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: none beyond those its sections cite.

## P1, with C2 of its workflow file: a Rust item named in prose has no checked form

- **a.** What it says: the design homes, the scoped `CLAUDE.md` files and the comments name Rust
  modules in plain backticks, such as a module path of `check`; a move left them stale with a passing
  check, unlike a `path` reference, which dangles. The search found most; one sat wrapped in a build
  script's comment, and the rewrite produced three names of items no facade publishes. "Each name is
  written bare, beside no issue entry." Proposal: a kind that names an item by its anchor and path,
  resolved against the Rust parse, "a question for a design discussion" (C2); on the project's side,
  none until that form exists (P1).
- **b.** Whether it still applies: yes. `primer@checker-model` lists the reference kinds at HEAD, and
  none names a Rust item.
- **c.** Whether it is already recorded: no issue. `design@model@names-collected-where-declared`
  collects a Rust name where it is declared, for a rule marker, "A name is the one thing prose
  cannot reach"; it holds names and no resolution of a module path, as C2 says. No `rejected` entry
  covers an item kind.
- **d.** Its kind: missing, a pointer no checked form expresses; the project's side is a lapse of the
  primer's rule that such a pointer is written beside an issue reference, which could not be met as
  no issue existed.
- **e.** Its scope: `goal@knowledge-architect@records-reach-their-reader`, "renaming or removing an
  item gives the list of what to repair". Owner `model`, which holds the parse and the resolver. A
  new kind is a decision under `skill@knowledge-architect-design`.
- **f.** A better fix: the nearest rival is a convention of naming a module by its file, as a `path`
  reference, which dangles at a move today. What decides: a module path names an item a facade
  publishes, which a file path does not, and the three stale names the code-claims reviewer found
  were of that kind. Both are for the discussion.
- **g.** Its route: an issue.
- **h.** The proposal: open an issue in `model`, kind `design`, id
  `issue@model@a-rust-item-named-in-prose-has-no-checked-form`.
  - What: a module or a function named in prose is plain text to the checker, so a move leaves it
    stale with a passing check; the instance of the model's split.
  - Why it matters: against `goal@knowledge-architect@records-reach-their-reader`; and the primer
    asks such a pointer to be written beside an issue reference, which this entry gives.
  - What would close it: a discussion that gives a kind resolved against the Rust parse, or rules
    that none is worth its cost, then the sites converted or left beside this entry.
- **Default**: open the issue.
- **Outcome**: decided by the session: open `issue@model@a-rust-item-named-in-prose-has-no-checked-form` in
  `model`, kind `design`. Ground: no record holds it, and the primer's rule on a pointer with no
  checked form needs an issue to cite.

## P2: the rule that decided which heads moved left with the spec that held it

Analysed with P2 of 2026-11-02-knowledge-architect-component-split-git-out, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-split-git-out.md`.

## P3: the milestone's list of what a new crate owes missed the root README's layout table

Analysed with P2 of 2026-11-02-knowledge-architect-component-split-git-out, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-split-git-out.md`.
