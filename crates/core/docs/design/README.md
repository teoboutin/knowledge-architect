# The core checker: design

Recorded intent for this tool: how it is built internally, and why. Present tense, each decision
carrying a slug anchor, cited from elsewhere with `core` as its component. The designs this
Component did not keep are `register@core@rejected`, each cited by the heads whose argument covers
it, and `skill@knowledge-architect-decision-recording@head-shape` owns the shape.

**What belongs here:** a decision that does **not** survive deleting this tool. How it works and
how the pieces inside divide the work belongs here.

What otherwise shapes it is the module documentation at the top of each file under
`path@core@src/`.

The heads are grouped by subject, one subdocument per subject, listed below. A subject heading
carries no decision; each decision is one level-three head below it.

- [The model, the phases and the findings](run-model.md): how a run is built from the manifest and a model, how its findings are classified and its exit code read
- [Extensions and the library API](extensions-and-api.md): how an extension plugs into the checker, and the crate's public surface and command line
- [Git and the walk](git-and-the-walk.md): how git supplies the tree a run reads, the staged tree included
- [Anchors and Components](anchors.md): what an anchor is, what a Component carries, and how anchors stay apart
- [Registers and their homes](registers.md): which registers exist, how each shape of home holds its entries, and the built-in registers' own rules
- [Plan documents](plan-documents.md): the plans directory, its specs and milestones, and their items
- [Generated files and fixes](generated-files-and-fixes.md): the generated indexes, the commands that write them or list the anchors, and what `check --fix` repairs
- [The record worklist](worklist.md): how `klarch worklist` cuts the record into clusters and packs them into agents
- [Entities, ids and headings](entities-and-headings.md): what an entity is, where a slug defines one, and how an unresolved reference is reported
- [Reference spans and their lints](reference-lints.md): which spans are references, and the lints on reference shapes and bare ids
- [Paths, homes and links](paths-and-links.md): the `path`, `register` and `planned` references, and markdown links
- [What is prose](prose.md): which bytes of each kind of file are prose, and how the checker's version is pinned
- [Commit messages](commit-messages.md): a commit message judged as a document, against its commit's tree
- [The agent configuration](agent-configuration.md): the harnesses a project serves, its declared command, and the installed files as entities
- [Creating a project and a Component](creating.md): `init` and `new-component`
- [Session transcripts](transcripts.md): the `transcript` command, and which transcripts it reads
