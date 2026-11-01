# Goals — the core checker

What the core checker is for, as its share of the project's goals,
`register@knowledge-architect@goal`. The checker exists mostly for
`goal@knowledge-architect@agents-work-without-drift`: it reports where the documents and the code
drift from the recorded design, and it makes keeping that design current cheap. A goal is met or
unmet, where a decision about how the checker is built is won or lost and lives in
`register@core@design`. A goal stays here while it is met, and leaves only when the owner abandons
it.

## In a project whose gates run the checker on each commit, references, register entries and generated listings stay correct at every commit `##documents-stay-current`

In a project whose gates run the checker on each commit, these hold on the main branch and on every
commit of a branch that merges:

- every reference in the texts the checker reads resolves;
- every register entry has the shape its register declares;
- every generated listing matches what regenerating it writes.

It is met while the checker's verdict fails on a tree where any of these fails, and on a range for
each commit where any fails.

It refines `goal@knowledge-architect@documentation-stays-consistent`.

## The rules a project's documents must meet, compiled in or declared in its manifest, are enforced by the checker `##document-rules-are-checked`

A project states in its manifest what it records, where each kind of record lives, and the kinds and
the anchors by which one record points at another, in the reference grammar every project shares.
These stand beside the rules the core compiles in.

A tree that passes the checker conforms to what its manifest states, to the rules the core compiles
in and to the reference grammar, in the texts the checker reads. No project can hold back any part
of these three, beyond:

- the paths its manifest leaves out of the walk;
- the rules a declaration of its manifest switches off, as an empty list of harnesses switches off
  the checks of the installed text.

Conformance is established by running the checker, not by a reviewer remembering the rules.

It is met while a tree, and each commit of a range, that the checker passes conform, in the texts
the checker reads, to every rule the core compiles in that no declaration switches off, to every
rule the manifest declares, and to the reference grammar, save what a declaration switches off.

It refines `goal@knowledge-architect@documentation-stays-consistent`.

## Moving a Component or a location, moving a declared register's home within the anchors that carry it, or turning a register's home from one file into a directory, costs at most one edit of the checker's manifest, and no edit to a reference to what moved `##relocation-is-one-manifest-edit`

Every reference to what the manifest places names its anchor, and each anchor's directory is
declared once, in the checker's manifest. So:

- moving a Component or a location changes the manifest;
- moving a declared register's home changes its declaration, and moves it in every anchor that
  carries the register;
- refactoring a register's home from one file to a directory changes no file outside the home and no
  manifest line;
- none of these changes a reference to what moved, as the measure names those references.

Renaming a Component's directory renames its anchor, which its basename names, and is no move.

It is met while such a move or change of shape leaves no reference to repair among those that name
the moved Component or location, or a register, its home or one of its entries, the anchor-relative
`path` references under a moved Component or location included. A `path` reference to a file inside
a register's home is outside this goal.

It refines `goal@knowledge-architect@documentation-stays-consistent`.

## A recorded decision or an outstanding issue is delivered to whoever needs it, through checked references `##records-reach-their-reader`

The checker gives two listings, so a session reading a decision meets what stands against it, and a
reversal comes with the list of what it touches:

- for any entry, every text of the tree the checker reads that references the entry;
- every standing issue and tripwire of the project.

It is met while the first listing holds every reference in the texts of the tree the checker reads,
and the second every standing issue and tripwire, both computed from the tree, never kept by hand.

It refines `goal@knowledge-architect@structure-and-workflow-work-together`.

## The documents render as one linked site `##documents-render-as-a-linked-site`

A preprocessing pass rewrites every reference into a link and an mdbook build renders the result,
hosted with the project once it is released. Dense cross-referencing is what makes the rendered site
worth reading, so every citeable thing having one reference form serves this goal directly.

It is met while a build renders knowledge-architect's Markdown documents that the checker reads into
one hosted site, each reference that resolves rendered as a link to its target.

## A project adds checks of its own through a documented extension API `##projects-add-their-own-checks`

A project whose conventions the built-in checks do not cover writes its own checks as an extension,
against a library API documented with its contract and its examples. The project's checker binary
runs them in the same subcommands, with the same report, as the built-in ones.

It is met while an extension needs no change to the core and uses only its public, documented API.

It refines `goal@knowledge-architect@agents-work-without-drift`: a project's own checks guard its
own design intent.

## The checker judges a commit's tree from the git pre-commit hook `##checks-a-commit-from-its-hook`

It refines `goal@knowledge-architect@documentation-stays-consistent`.

A project can run the checker from its git pre-commit hook, the hook git runs before it records a
commit, to judge the tree that commit records, before the commit exists. The commits this covers are
those of `git commit`, `git commit -a` and `git commit <paths>`, the paths with or without `--`
before them, given its message by `-m`, by `-F` or in the editor, with or without `-q`, and with no
other option of `git commit`. Each is covered in the repository's main worktree and in a linked one.
A target project is covered here when a `.git` is at or above its directory.

It is met while the core's tests show, for each of the three forms, in a main worktree and in a
linked one, a commit whose tree fails, which the hook refuses, and one whose tree conforms, which
git records.
