---
kind: todo
---
# The documents do not yet render as one linked site

## Summary

The core's goal is that knowledge-architect's own Markdown documents that the checker reads render
as one hosted site, each reference that resolves rendered as a link to its target, built by mdbook.
Nothing builds it yet.

## Details

### What

A preprocessing pass that rewrites each reference into a link to its entry, and an mdbook build of
the result, hosted with the project once it is released. Beyond the mdbook build the goal names,
nothing about the shape is decided.

### Why it matters

It is the work that fulfils `goal@check@documents-render-as-a-linked-site`, and the rendered site
is one of the places `goal@knowledge-architect@adoption-is-easy` names. Until it exists, a
reader follows a reference by searching for its entry, not by a link.

### What would close it

The pass and the build, run over this repository's own documents, producing a site in which every
reference is a working link.

### Leads from the design of the Component split

The design discussion that planned `milestone@plans@component-split` placed this work. These are
leads for this work's own design session:

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
