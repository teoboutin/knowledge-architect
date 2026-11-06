# The checker: design

Recorded intent for this tool: how it is built internally, and why. Present tense, each decision
carrying a slug anchor, cited from elsewhere with `check` as its component. The designs this
Component did not keep are `register@check@rejected`, each cited by the heads whose argument covers
it, and `skill@knowledge-architect-decision-recording@head-shape` owns the shape.

**What belongs here:** a decision that does **not** survive deleting this tool. How it works and
how the pieces inside divide the work belongs here.

What otherwise shapes it is the module documentation at the top of each file under
`path@check@src/`.

The heads are grouped by subject, one subdocument per subject, listed below. A subject heading
carries no decision; each decision is one level-three head below it.

- [The model, the phases and the findings](run-model.md): how a run is built from the manifest and a model, how its findings are classified and its exit code read
- [Extensions and the library API](extensions-and-api.md): how an extension plugs into the checker, and the crate's public surface and command line
- [Git and the walk](git-and-the-walk.md): the staged tree a run reads, and how `check` differs from `commits`
- [Registers and their homes](registers.md): which registers exist, how each shape of home holds its entries, and the built-in registers' own rules
- [Plan documents](plan-documents.md): the sections a plan document owes
- [Generated files and fixes](generated-files-and-fixes.md): the generated indexes, the commands that write them, and what `check --fix` repairs
- [Entities, ids and headings](entities-and-headings.md): which goals a goals home may cite, and what a check says of a citation's direction
- [Reference spans and their lints](reference-lints.md): the lints on reference shapes and on an id written outside its reference
- [Paths, homes and links](paths-and-links.md): the anchor a path names, and markdown links
- [What is prose](prose.md): the checker's own directories, and how its version is pinned
- [Commit messages](commit-messages.md): a commit message judged as a document, against its commit's tree
- [The agent configuration](agent-configuration.md): the agent files, the declared command, and the installed set
