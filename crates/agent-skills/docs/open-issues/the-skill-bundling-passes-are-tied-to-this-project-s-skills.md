---
kind: todo
---
# The passes that build and bundle installable agent text are tied to this project's own skills, so no other project can reuse them

## Summary

The agent-skills crate holds two things: the build passes that turn a source tree of agent text
into the files a binary embeds and installs, and this project's own skills. The owner wants to
reuse the first in other projects. Doing it means splitting the crate into a skill-bundling
utility and the skills themselves.

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
  substitution pass, `substitute`; and the workflow name check, `check_workflow_name`;
- the mapping from the source layout to the install layout, `install_path` in
  `path@agent-skills@build.rs`, per `design@agent-skills@content-mirrors-the-install-layout`;
- the generated lists `FILES` and `SNIPPETS` of `path@agent-skills@src/lib.rs`;
- the install, the repair of the installed copies and the deletion of unshipped owned files, in
  `path@core@src/agents.rs` (`install_into`, `repairs`, `apply`, `owned_on_disk`), which the
  owned-namespace check of `design@core@owned-namespace-check` uses.

What ties them to this project now:

- the installer's prefix, the constant `PREFIX` of `path@agent-skills@build.rs`, and the paths
  `PRIMER` and `IMPORT_LINE` of `path@core@src/agents.rs`, are this project's values;
- the rows of `SUBSTITUTIONS` are this project's own literals;
- the build script includes `path@agent-skills@src/render.rs` with `#[path]`, and its functions
  are `pub(crate)`, so the passes are not a library any other crate can call;
- the install lives in the core, and it is not public API.

The owner has not designed the split. What it must decide includes: which crate holds the passes
and which holds the install, whether a consuming project calls the passes from its own `build.rs`,
how the prefix, the layout and the substitutions become parameters, and whether the checker's
check of the installed copies becomes part of the utility.

### Why it matters

A project that wants to ship its own skills with a pinned version has to copy these passes, and the
copy then drifts from this one. The owner wants to reuse this capability in other projects. No
decision of this project is strained by the split, and no goal of this project asks for it.

### What would close it

A design discussion under `skill@knowledge-architect-design` that settles the shape of the split,
then the split built: a utility crate that another crate's `build.rs` can call with its own prefix,
layout and substitutions, and this project's skills built with that crate.
