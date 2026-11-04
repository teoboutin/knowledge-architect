---
kind: todo
---
# The documents do not yet render as one linked site

## Summary

`goal@knowledge-architect@documents-render-as-a-linked-site` asks that knowledge-architect's own Markdown
documents that the checker reads render as one hosted site, each reference that resolves rendered as a link
to its target. Nothing builds it yet.

## Details

### What

A preprocessing pass that rewrites each reference into a link to its entry, and a build of the result,
hosted with the project. The owner's lead is mdbook, a design lead the goal does not name; nothing about the
shape is decided.

Two cases of the goal for that work to decide, which the goal's words leave open: a resolving `path` reference to a source file
must render as a link, where the site holds Markdown documents, so the site holds source files or links out of itself; and the
documents the checker reads take in plan documents, skills and open issues, so working documents would be published.

### Why it matters

It is the work that fulfils `goal@knowledge-architect@documents-render-as-a-linked-site`. Until it exists, a
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
