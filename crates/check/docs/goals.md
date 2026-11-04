# Goals — check

Every goal here serves one purpose: the checker reports each breach of the rules it holds for a project's documents, and keeps what it
generates current, so that keeping them correct is cheap. These goals
are this Component's share of the project's goals, `register@knowledge-architect@goal`.

## In a project whose gates run the checker on each commit, its references, its goals and the entries of its record, and the texts the checker generates stay correct at every commit `##documents-stay-current`

A reference is a pointer by which one text names a recorded item, a file, a directory or a section of a text. A generated text is one the checker writes into the project's tree
from its documents, such as a listing of the entries of one kind. In a project whose gates
run the checker on each commit, these hold on the main branch and on every commit of a branch that
merges:

- every reference in the texts the checker reads resolves, with one reading;
- every goal has the shape a goal requires, and every entry of the record the shape its kind of
  entry requires;
- every text the checker generates matches what generating it again writes.

It is met while the checker's verdict fails on a tree where any of these fails, and on a range for
each commit where any fails.

It refines `goal@knowledge-architect@documentation-stays-consistent`.

- Requires, for instance: a commit of a merging branch that deletes an entry another text still
  names fails the checker's verdict.
- Admits, for instance: a pointer that resolves to nothing in a text the checker does not read.
- Rules out, for instance: a generated listing edited by hand so that it differs from what
  generating it writes, at a commit of a branch that merges.

## The rules a project's documents must meet, those the checker holds for every project and those the project declares, are enforced by the checker `##document-rules-are-checked`

A project declares to the checker what it records, where each kind of entry lives, and the kinds
and the places by which one entry points at another, in the reference grammar every project
shares. These stand beside the rules the checker holds for every project.

A tree that passes the checker conforms to what the project declares, to the rules the checker
holds for every project and to the reference grammar, in the texts the checker reads. No project
can hold back any part of these three, beyond:

- the paths it declares outside the check;
- the rules a declaration of the project switches off.

Conformance is established by running the checker, not by a reviewer remembering the rules.

It is met while a tree, and each commit of a range, that the checker passes conform, in the texts
the checker reads, to every rule the checker holds for every project that no declaration switches
off, to every rule the project declares, and to the reference grammar, save what a declaration
switches off.

It refines `goal@knowledge-architect@documentation-stays-consistent`.

- Requires, for instance: an entry of a kind the project declares, missing a part that kind
  requires, fails the check.
- Admits, for instance: a document under a path the project declares outside the check, which
  breaks every rule.
- Rules out, for instance: a project holding back a rule the checker holds for every project with
  no declaration that switches it off.

## A relocation of part of a project's documents needs no manual work beyond the change of goals its change of purpose calls for, and a few mechanical edits `##relocation-needs-only-goal-changes`

A project relocates part of its documents when it moves a Component, or another directory that
holds its entries, moves the place where a kind of entry lives, or turns that place from one file
into a directory. A project relocates only on a change of purpose of the part it moves, which the
goals carry and which justifies the move, or where the relocation, with mechanical edits alone,
breaks no check. Either way, changing the goals that the change of purpose calls for is the only
manual work it needs, beside a few mechanical edits, such as a new name replaced wherever the old
one is cited. An existing project that brings its documents into conformance when it adopts the
workflow makes no relocation under this goal.

It is met while:

- each such relocation, once the goals carry its change of purpose, passes the check after no
  other edit than that change of the goals and a few mechanical edits;
- the installed text has a session relocate part of a project's documents only on those two
  occasions.

It refines `goal@knowledge-architect@agents-work-without-drift`.

- Requires, for instance: a Component moved to another directory, with no change of purpose,
  passes the check after the project's declaration of where it lives changes.
- Admits, for instance: a Component renamed, its old name replaced wherever it is cited by one
  search and replace.
- Admits, for instance: an existing project that moves its decision records into the home the
  workflow gives them, when it adopts the workflow.
- Rules out, for instance: a Component moved for tidiness, with no change of purpose, after which a
  session must rewrite references one by one, by judgement, for the check to pass.

## A project adds checks of its own through public APIs `##projects-add-their-own-checks`

A project whose conventions the built-in checks do not cover writes its own checks as an extension,
against the public library APIs of knowledge-architect's crates, with no change to them, so it can build
it against published versions of them, which it pins. The checker the project runs with its extension runs
them in the same commands, with the same report, as the built-in ones.

It is met while an extension needs no change to any crate of knowledge-architect and can be built against
published versions of them, and while the checker run with an extension runs its checks in the same commands,
with the same report, as the built-in ones.

It refines `goal@knowledge-architect@agents-work-without-drift`, a project's own checks guarding its own
design intent, and `goal@knowledge-architect@every-target-project-can-adopt-it`, a project pinning the version
it uses.

- Requires, for instance: a project's check of its own naming convention reports in the same
  report as a built-in finding.
- Admits, for instance: an extension that only one project uses, one that depends on several crates of
  knowledge-architect, or one built against a patched copy while its project develops it.
- Rules out, for instance: an extension that needs a change to a crate of knowledge-architect to run.
- Rules out, for instance: an extension that can only be built against an unpublished copy of a crate.

## The checker judges a commit's tree from the git pre-commit hook `##checks-a-commit-from-its-hook`

A project can run the checker from its git pre-commit hook, the hook git runs before it records a
commit, to judge the tree that commit records, before the commit exists. The commits this covers are
those of `git commit`, `git commit -a` and `git commit <paths>`, the paths with or without `--`
before them, given its message by `-m`, by `-F` or in the editor, with or without `-q`, and with no
other option of `git commit`. Each is covered in the repository's main worktree and in a linked one.
A target project is covered here when a `.git` is at or above its directory.

It is met while the checker's tests show, for each of the three forms, `git commit <paths>` both with
and without `--` before its paths, with each of the three ways of giving its message, with and without `-q`, in a main worktree
and in a linked one, a commit whose tree fails, which the hook refuses, and one whose tree
conforms, which git records.

It refines `goal@knowledge-architect@documentation-stays-consistent`.

- Requires, for instance: `git commit -a -m` in a linked worktree, on a tree that fails, is refused
  by the hook.
- Admits, for instance: a hook that also judges a commit made with `--amend`.
- Rules out, for instance: a hook that refuses a covered commit whose tree conforms.
