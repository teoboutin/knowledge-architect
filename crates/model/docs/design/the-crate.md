# Model design: the crate and its interface

## The crate and its interface

### The public API is one file of role modules, each naming its items `##model-facade`

`path@model@src/lib.rs` names every public item, in the root or in a role module whose body is in that
file, and every other module is private. So the surface is read in one file, and moving an item between
files changes no public path, which `rejected@core@module-tree-as-public-api` refuses to give up.
`unreachable_pub` refuses a `pub` item no role module names, and a unit test refuses a `pub mod <name>;`
and a glob, on which the lint is silent. Nothing is public for a test: a test that needs a private item
is a unit test of this crate. An inherent method of a public type is not read here: a method is public
for a caller's code, and the review reads it.

### The resolver says what a reference names or why it names nothing, with no finding attached `##resolver-outcome`

Each function of `model::resolve` returns a `Target`, or a `Cause` holding what its texts need. A site
renderer needs the resolution and no finding, and the lints of the core need a finding at a site, so
`Cause::finding` writes one and a consumer that reports nothing never calls it. The words of a cause
are the resolver's, per `design@core@finding-names-the-repair`: `show` prints them, and it sits below
the checks, so they cannot sit in the lints. A dispatch takes a candidate to the function of its kind,
so no consumer repeats it. The resolver reads `TreeInputs`, and `model`'s gathering takes the check-ignore
batch, since a consumer with no check would otherwise read a target the rules cover as absent,
against `design@model@ignored-targets-are-not-asserted`.

### An extension's phase-1 contribution reaches the model as data `##contribution-is-data`

`model::extension::Contribution` holds the tables an extension claims, its complaints, its declared
paths and its generated files, and `contribute` records them. `check` computes the value from each
extension's `tables` and `resolve`, per `design@core@an-extension-plugs-in-through-phased-hooks`. Phase 1
is then a pure function a test drives with a value, and `query` and `render` run it with no extension
to name. The nearest rival is `rejected@model@contribution-as-a-trait`.

### This crate depends on no code that judges a document's content `##no-check-below-the-model`

A tool over the record needs the model and no check, per `design@knowledge-architect@model-crate`, so
such a dependency would put every check into the build of each of those tools. `agent-skills` is the one
exception while the installed-file check stays in phase 2. `cargo tree -e normal -p knowledge-architect-model`
lists no such crate; a dev-dependency builds into no consumer.
