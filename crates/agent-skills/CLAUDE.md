# Agent skills

This crate carries text, not behaviour; the core reads it to install and to check.
`path@agent-skills@content/` holds the shipped files in the install layout, and
`path@agent-skills@build.rs` generates the list that `path@agent-skills@src/lib.rs` exposes, each
entry the install path and the text, per `design@agent-skills@content-mirrors-the-install-layout`.
A file under content/ that the layout does not map fails the build.

**A saved workflow under `path@agent-skills@content/workflows/` is JavaScript, which the checker
does not read**: a reference in it is checked by nobody, so it cites none. Its `meta` declares the
installed file's stem as its `name`, which the build asserts. The harness runs the file's body as
an async function, so a top-level `return` is legal there and `node --check` refuses it on the
bare file: check it parses by wrapping the body in `async function body() {…}`, with `export`
removed. The build's passes run over it as over any shipped file: a `%%` line is removed, even
inside a template literal, so a line of a prompt never opens with `%%`.

This repository installs its own skills, and its check compares each installed file with the shipped
text byte for byte. After editing content/ or snippets/, run `cargo klarch install-agent-skills` and
commit the installed copies under .claude with the change, or the check fails.

**content/ is in the walk, and every reference in it is checked against this repository.** So a
reference to an entry of this repository passes here and would dangle in every installing project:
the text cites no entry, per `design@agent-skills@shipped-text-cites-no-entry`, and nothing checks
that yet, per `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`. It names no
Component and no convention of this repository; it writes only the forms
`design@agent-skills@shipped-text-cites-no-entry` admits, read whole, and the project's command as the install placeholder. It may cite the shipped set's own skills, agents, primer and their sections,
`skill@<name>@<slug>`, which every project serving `claude` holds. A level-two heading under
content/ ends with its section slug as a placeholder, `{{slug:<id>}}`, which the build renders. A skill names
another installed skill by its installed name, which carries the installer's prefix.

**A line whose first two characters are `%%` is a comment for this repository**, per
`design@agent-skills@shipped-text-line-comments`: it may cite design heads and issues, and the build
removes it. Indented, or inside a fenced block, it fails the build. **A literal the checker would
misread** goes in `SUBSTITUTIONS` of `path@agent-skills@build.rs`, with the reason in its comment,
and the text writes its placeholder; a row no text uses fails the build.

**The blocks the setup skill shows for a Rust project are files under
`path@agent-skills@snippets/`**, which the skill names by placeholder lines,
`{{snippet:<file>}}`, and the build script inlines; `klarch init --rust` writes all of them but
`ci.yml` from the crate's `SNIPPETS`, per `design@agent-skills@rust-templates-one-home`. Edit the
file, not the skill. The example `path@xtask@examples/setup_snippet.rs` compiles the main, and
`path@xtask@tests/scaffold.rs` builds a project written from the other three and parses the CI
workflow's gates command. rustfmt does not reach an included file, so
`rustfmt --edition 2021 --check` the main by hand after an edit. The checker reads the main as
Rust source: its comments are prose, a reference in them that resolves here passes the check and
ships, and the text it ships cites no entry, as content/ does, per
`design@agent-skills@shipped-text-cites-no-entry`. The release's hand check reads the installed
copies, which hold the snippet inlined.

**A text several installed texts apply whole is a file under `path@agent-skills@fragments/`**,
which each host names by a placeholder line, `{{fragment:<file>}}`, per
`design@agent-skills@shared-text-fragment`. Edit the fragment, never a host's installed copy. A
fragment cites no entry, as content/ does, and the build refuses one holding a heading slug,
another fragment, or a build placeholder but `{{command}}`. A review of a change to a fragment
names each host's installed copy in the brief of the agent-facing-text axis, since a sentence true
in one host's context can be false in another's.

**The primer is the part of every installing project's root `CLAUDE.md` that is the same in every
project**, per `design@agent-skills@primer-limit`.

## Editing an installed skill or agent

The text under `path@agent-skills@content/` is the source the install copies, not an installed
file, and it is edited here by hand. It is written under
`skill@knowledge-architect-agent-configuration`, read whole, as a project's own skill or agent is.
Three parts of that skill concern a project's own names and the installed copies, and do not apply
to this source: the first sentence of its scope, the prefix rule of
`skill@knowledge-architect-agent-configuration@shaping-a-skill`, since a shipped skill or agent is
named with the installer's prefix, and
`skill@knowledge-architect-agent-configuration@installed-files-never-edited`.

For the design skill, a finding about an intermediate table rather than the outcome is
not worth text (`design@agent-skills@outcome-over-display`). A rewording needs no design entry
(`design@agent-skills@instruction-record-is-minimal`).

The decisions about the skills are `register@agent-skills@design`.
