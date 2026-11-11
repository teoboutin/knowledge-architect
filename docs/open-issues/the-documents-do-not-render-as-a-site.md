---
kind: todo
---
# The documents do not yet render as one linked site

## Summary

`goal@documents-render-as-a-linked-site` asks that the documents of each target
project render as one site of that project, each reference rendered as a link to what it names, and
that a project can host that site when it releases. So the tool that builds the site is one that
knowledge-architect provides to every target project, not a build of this repository's documents
alone, on the owner's word. Nothing builds it yet.

## Details

### What

A preprocessing pass that rewrites each reference into a link to its entry, and a build of the result,
which a target project runs over its own documents and can host when it releases. The owner's lead is mdbook, a design lead the goal does not name; nothing about the
shape is decided.

Two consequences of the goal for that work. A resolving `path` reference to a source file must render as a link, where the site
holds Markdown documents, so whether the site holds source files or links out of itself is that work's to decide. And the documents of a project
take in its plan documents, its skills and its open issues, so as written those are rendered; a narrower set is a design decision that
work argues against that rival, or a change of the goal that the owner rules.

### Why it matters

It is the work that fulfils `goal@documents-render-as-a-linked-site`. Until it exists, a
reader follows a reference by searching for its entry, not by a link.

### What would close it

The pass and the build, provided to every target project, and run over this repository's own
documents as a target project's, producing a site in which every reference is a working link.

### Leads from the design of the Component split

These are leads for this work's own design session:

- **A Component of its own, `render`**, a published crate in crates/render, created by this work,
  after the milestone has landed. It depends on the milestone's `model` Component alone, plus
  mdbook: the milestone separates the resolution of a reference from the lints that report a
  broken one, so the pass that rewrites each reference into a link reads the resolution with no
  check attached.
- **It is a Component of its own because it decides a layout**: the page tree, each link's target,
  and the book's summary. Its mdbook dependency also stays out of every crate an extension builds.
- The premise that the renderer needs no check is this work's to keep: its design session judges
  it, as the milestone's acceptance criterion judges it for the commands over the model, and
  `cargo tree -p knowledge-architect-render` listing no `knowledge-architect-check` would check it.
