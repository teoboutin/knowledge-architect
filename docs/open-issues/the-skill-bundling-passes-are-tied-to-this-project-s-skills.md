---
kind: todo
---
# The passes that build and bundle installable agent text are tied to this project's own skills, so no other project can reuse them

## Summary

The agent-skills crate holds two things: the build passes that turn a source tree of agent text
into the files a binary embeds and installs, and this project's own skills. The owner is
interested in reusing the first in other projects, and thinks the crate could later be split into
a skill-bundling utility and the skills themselves.

## Details

### What

This project delivers its agent text through a binary of a pinned version, which writes the text
into a project and checks the installed copies byte for byte, per
`design@knowledge-architect@binary-bundles-workflow`. The owner observes that the usual harness
plugin marketplaces do not offer this. It is the delivery that text telling an agent how to use a
tool needs, since that text must match the version the project pins. Any project that ships a tool
together with the skills that describe it has the same need.

The parts that would form a reusable utility, and where each sits now:

- the build passes of `path@agent-skills@build.rs` and `path@agent-skills@src/render.rs`: the
  `%%` comment pass, `strip_comments`, per `design@agent-skills@shipped-text-line-comments`; the
  section slug pass, `render_slugs`; the fragment pass, `render_fragments`, per
  `design@agent-skills@shared-text-fragment`; the snippet pass, `Snippets::render`; the
  substitution pass, `substitute`; the workflow name check, `check_workflow_name`; the
  placeholder readers `fragment_placeholder` and `snippet_placeholder`; and the source view,
  `source_view`, which `cargo x skill` prints, per `design@xtask@source-view-in-the-tool`;
- the mapping from the source layout to the install layout, `install_path` in
  `path@agent-skills@build.rs`, per `design@agent-skills@content-mirrors-the-install-layout`;
- the generated lists `FILES` and `SNIPPETS` of `path@agent-skills@src/lib.rs`;
- the install, the repair of the installed copies and the deletion of unshipped owned files, in
  `path@check@src/agents.rs` (`install_into`, `repairs`, `apply`, `owned_on_disk`), which the
  owned-namespace check of `design@check@owned-namespace-check` uses.

What ties them to this project now:

- the installer's prefix, in three constants: `PREFIX` of `path@agent-skills@build.rs`,
  `OWNED_PREFIX` of `path@model@src/manifest.rs` and `PREFIX` of `path@xtask@src/skill.rs`; and the
  paths `PRIMER` and `IMPORT_LINE` of `path@model@src/shipped.rs`; all are this project's values;
- the rows of `SUBSTITUTIONS` are this project's own literals;
- the build script and `path@xtask@src/skill.rs` include `path@agent-skills@src/render.rs` with
  `#[path]`, per `design@xtask@view-included-not-exported`, and its functions are `pub(crate)`,
  so the passes are no library a crate outside this repository can call, and a move of the file
  breaks both includes;
- the install lives in `check`, and it is not public API.

The owner has not designed the split. What it must decide includes: whether a consuming project
calls the passes from its own `build.rs`, and how the prefix, the layout and the substitutions
become parameters. Which crate holds the passes and the install, and that the checker keeps its
check of the installed copies, are leads of the Component split, below.

### Why it matters

A project that wants to ship its own skills with a pinned version has to copy these passes, and the
copy then drifts from this one. The owner is interested in reusing this capability in other
projects. No
decision of this project is strained by the split, and no goal of this project asks for it.

### What would close it

The owner plans to state the goal of the reuse when the work on the split starts, under
`skill@knowledge-architect-goal-setting`. Then a design discussion under
`skill@knowledge-architect-design` that settles the shape of the split, then the split built: a
utility crate that another crate's `build.rs` can call with its own prefix, layout and
substitutions, and this project's skills built with that crate.

### Leads from the design of the Component split

The design discussion that planned `milestone@plans@component-split` placed this work in the split.
On the owner's word, it planned the placement alone, and nothing of how the work is built, so
these are leads for this work's own design session:

- **A Component of its own, `bundle`**, a published crate in crates/bundle, created by this work,
  after the milestone has landed. It holds the build passes of `path@agent-skills@build.rs` and
  `path@agent-skills@src/render.rs`, and the generic part of the install: rendering a text for a
  project, writing it under a prefix, and deleting the owned files a version does not ship.
- **agent-skills keeps the content** (content/, fragments/, snippets/, `FILES`, `SNIPPETS`) and
  depends on `bundle` as a build-dependency.
- **The checker keeps the comparison** of the installed copies, as a check family of the last
  phase, which the milestone's slice `spec@component-split@check-renamed` builds, and the command
  `install-agent-skills`, which would call the install of `bundle`. The owner's lead, at the
  milestone's D3: the logic that verifies every shipped text is installed may belong to `bundle`
  rather than to the checker.
- On the owner's word in that discussion, the goal of the Component is written when it is created.
