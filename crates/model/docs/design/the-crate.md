# Model design: the crate and its interface

## The crate and its interface

### The public API is one file of role modules, each naming its items `##model-facade`

`path@model@src/lib.rs` names every public item, in the root or in a role module whose body is in that
file, and every other module is private. So the surface is read in one file, and moving an item between
files changes no public path, which `rejected@check@module-tree-as-public-api` refuses to give up.
`unreachable_pub` refuses a `pub` item no role module names, and a unit test refuses a `pub mod <name>;`
and a glob, on which the lint is silent. A method of a named type is public through its type, and the
facade does not list it. The entity table and the anchors are public here, since the tools over the
documents and the checks each need them across a crate boundary, on the session's choice over a second set
of types no less nameable, which the owner approved; the narrow view an extension is to get is `check`'s,
per `issue@check@an-extension-cannot-see-which-register-a-document-belongs-to`. Nothing is public for a test: a test that needs a private item is a unit test
of this crate.

### The resolver says what a reference names or why it names nothing, with no finding attached `##resolver-outcome`

The function of each kind, in `model::resolve`, returns a `Target`, or a `Cause` holding what its
texts need. A site renderer needs the resolution and no finding, per
`goal@check@documents-render-as-a-linked-site`, and the lints of `check` need a finding at a site, so they write
one from `Cause::texts`, and this crate writes none, per `goal@model@a-tool-reads-the-documents-with-no-check`. `reference` takes a
candidate of an anchored kind to the function of its kind, and `harness` takes one of a harness kind.

### The words of a cause are the resolver's, so a tool that links no check prints them `##cause-words-in-the-resolver`

A cause's two texts, what is wrong and what repairs it, per `design@check@finding-names-the-repair`, are
`Cause::texts`. `show` prints them, and a tool over the documents needs them and no check, per
`design@knowledge-architect@model-crate`. The nearest rival is
`rejected@model@cause-words-in-the-lints`.

### What a tool reads of the working tree for phases 1 to 3 and the resolver is gathered in `model`, the ignore batch included `##tree-inputs-gathered-here`

`Gathered` reads it from the working tree: one survey, the two git batches and the shipped text. The
resolver reads the ignore answers, so a tool that links no check needs the batch here, or it reads a
target the rules cover as absent, against `design@model@ignored-targets-are-not-asserted`. A run over a
tree git holds gathers its own in `check`, per `design@check@staged-tree-source`, since no working tree
holds what it reads. The nearest rival is `rejected@model@ignore-batch-in-the-checks`.

### An extension's phase-1 contribution reaches the model as data `##contribution-is-data`

`model::extension::Contribution` holds the tables a binary's extensions claim, their complaints, their
declared paths and their generated files, and `contribute` records them. `check` computes the value
from each extension's `tables` and `resolve`, per
`design@check@an-extension-plugs-in-through-phased-hooks`. Phase 1 is then a pure function a test drives
with a value, and a tool with no extension has no trait to name. The nearest rival is
`rejected@model@contribution-as-a-trait`.

### This crate depends on no code that judges a document's content `##no-check-below-the-model`

A tool over the documents needs the model and no check, per `design@knowledge-architect@model-crate`, so
such a dependency would put every check into the build of each of those tools. `cargo tree -e normal -p
knowledge-architect-model` lists no such crate; a dev-dependency builds into no consumer. The check that
stays in phase 2, the installed-file check, is this crate's own code, which is why it depends on
`agent-skills`, a crate of text.
