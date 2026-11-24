# Rejected designs — knowledge-architect

The designs this project did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@knowledge-architect@design` whose argument covers it. An entry stays `live` while its
reason holds, and is marked `void` when it no longer does.

### The checker's crate in crates/knowledge-architect `##crate-directory-named-after-project`

`live`. Refuted by a run, not by argument: in a scratch project named knowledge-architect with a
component at that path, the check stopped in phase 1, because the directory's basename and the
project's name are the same anchor. A crate directory is named by its role.

### One published package holding both the checker and the installed text `##single-published-package`

`live`. The owner asked for a single package. cargo packages only the files under a package
directory, so a single package holding both components must be rooted at crates/. That makes crates/
a package holding two components, which mixes a crate with the directories around it, and a third
crate could not live under it. Users name one crate either way, because cargo fetches a dependency
without being asked.

### The agent workflow as a plugin published on a marketplace `##workflow-as-marketplace-plugin`

`live`. A Claude Code plugin is how the harness distributes skills, and it was designed in full: a
marketplace in this repository, the plugin pinned to each release's tag and sha, a hook printing the
primer and warning on a version mismatch. It loses on three facts. Two artifacts, the crate and the
plugin, had to be released in step, and a release had to edit the marketplace after tagging. The
harness's plugin cache is per user, so two projects on one machine needing two versions was never
shown to work. And a version mismatch could only be warned about, where an installed file can be
checked. A custom marketplace gave no discoverability in exchange.

### Tying the build to `CARGO_MANIFEST_DIR`, which every crate already reads `##build-tied-to-cargo-manifest-dir`

`live`. Refuted by a run on cargo 1.98.0: in two copies of a one-crate workspace sharing one
`CARGO_TARGET_DIR`, a binary printing `env!("CARGO_MANIFEST_DIR")` was not rebuilt in the second
copy and printed the first copy's directory. Cargo does not track a variable it sets itself.

### The crates' changelog copies written at packaging time and kept out of the tree `##changelog-copies-written-at-packaging`

`live`. Refuted by a run on cargo 1.98.0: a git-ignored copy that `include` lists makes
`cargo package --list` exit 101 as an uncommitted change, and an absent one is left out of the
package with no error.

### The gates library as the host of the hand mutation `##gates-library-hosts-the-mutation`

`live`. The gates library would have carried the mutation's run and restore, which a maintenance
crate already depends on. Every project that runs its gates through the library would then build
the mutation code, which no gate uses, and the library would hold a tool that judges nothing about
a branch's merge, against the purpose `design@knowledge-architect@gates-crate` gives it.

### The checker's git module made public, for the other tools that need git `##git-module-made-public`

`live`. A tool that needs git would depend on the whole checker to reach it, and build the
checker's parsers, tree-sitter, pulldown-cmark and toml_edit, which no question to git needs; the
module's interface would change with every release of the checker.

### The model left inside the checker `##model-inside-the-checker`

`live`. A tool over the record would then link every check, and a renderer of the site would read the
resolution through the lints, with a finding attached. It would change an interface others consume.

### The library and the binary as one crate `##single-crate`

`live`. The binary and the library as two targets of one package save a second package name to publish and to pin. It
holds while one crate contributes every command. Once the commands span crates, the binary's crate depends on them all,
so a commands module inside one of them forms a cycle with the others, and a binary crate that depends on them saves its
users no dependency it would otherwise be spared. It would change an interface others consume.

### The implementation's module tree as the public API, with only its visibility narrowed `##module-tree-as-public-api`

`live`. A consumer would learn the internal layout: thaum's rules extension at thaum's e98e296 named
13 of the 16 modules public at 465ff35: the distinct names `git grep -o "documentation::[a-z_]*"`
prints over its tools/rules-corpus, the function `component_dir` aside. Moving an item between files
would be a library break, and the public surface would be computed from the whole module tree rather
than read in one file.

### The harness kept a role module of `check`, re-exported by `cli` at `knowledge_architect::testing`, with the shared test crate unpublished `##harness-kept-in-check`

`live`. It spares an extension's tests one change of import and one more dependency, and the project one first publish. It is the import path extension writers use. The owner chose the published crate, believing it more natural that an extension's tests pull it separately.

### A `gates` command of the checker's binary, with the gate list declared in the manifest `##gates-command-of-the-checker`

`live`. It would have changed the manifest's format and the checker's command line, and needed no
code in a project. It would make `klarch` run cargo, the linter and the tests.

### A published gates binary configured by a file `##gates-binary-configured-by-file`

`live`. It would have added a configuration file format, which a gate list in Rust does not need,
and it leaves a project no binary of its own for its other repeated tasks.

### No gates code shipped: each project writes its own runner `##each-project-writes-its-runner`

`live`. Each project would keep the runner's logic and its tests, `path@gates@src/`, and refine it
alone. It is kept because a doubt remains: the library's interface may still carry assumptions of
the two projects it came from, and a project it does not fit falls back to writing its own.

### The changelog read by every check, each finding in a released section ruled by the owner case by case `##changelog-read-by-every-check`

`live`. Every span of the changelog that names an entry is checked, so a renamed or deleted entry
in the `Next release` section is reported. It loses on the owner's argument that the changelog
faces outward and should hold no reference to real content, a span shaped like one being read as a
name or a syntax example, and on its cost to a released section: each later change of the grammar
gives a finding there, whose repair would change the section's content.

### A new crate's real first version, pinning a new version of an existing crate, published by hand before the tag `##first-versions-by-hand-before-the-tag`

`live`. Refuted by a run: the new version of an existing crate reaches crates.io only through the
release workflow, after the tag, since the existing crates accept trusted publishing alone, and
cargo refuses to publish a crate that pins a version crates.io does not hold. Under cargo 1.98.0, a
crate whose manifest holds `knowledge-architect-agent-skills = "=0.99.0"`, packaged with
`cargo publish --dry-run --no-verify` in a directory outside every repository, stops with "failed to
select a version for the requirement `knowledge-architect-agent-skills = "=0.99.0"`", exit 101.

### The session pushing a release's tag, and deleting a pushed one `##session-pushes-the-release-tag`

`live`. Refuted by an observation of an outside tool: the harness's auto mode refused the tag push
to the session in the release of 0.10.0, before `git tag` ran. The deletion pushes the same ref,
and its refusal is assumed from the push's, not seen.

### The session reading a new crate's trusted publisher itself `##session-reads-the-trusted-publisher`

`live`. Refuted by a measurement of an outside tool: crates.io answers 403 without login on the
endpoint of a crate's trusted-publisher configuration, so a session cannot read which publisher a
new crate names.

### A table of the documentation lints in each published crate's manifest `##lint-table-per-published-crate`

`live`. cargo refuses a crate manifest that sets `[lints] workspace = true` beside lints of its own,
with "cannot override `workspace.lints` in `lints`", observed on cargo 1.98.0, so each published
crate would repeat every lint of the workspace's table, `unsafe_code` included, in its own manifest.
The run that shows it is the one `design@knowledge-architect@doc-lints-table` names.

### Narrowing a public item to `pub(crate)` instead of documenting it, where no crate of the workspace uses it `##narrowing-instead-of-documenting`

`live`. Narrowing a public item of a published crate removes it from the crate's public API, an
interface its consumers use, to answer a documentation lint. Its test, that no crate of the
workspace uses the item, is the wrong one: an item is public because a consumer needs it,
extensions outside the workspace included, which no search of the workspace sees. Whether an item
stays public is the facade's decision, made for a consumer, not a lint pass's. It holds for a
crate whose public items serve consumers outside the workspace, as
`design@check@the-facade-publishes-what-a-consumer-needs` and
`design@git@an-operation-in-git-s-terms-is-the-library-s` argue; a crate whose public items serve
the workspace's own tools, as `design@model@model-facade` says of `model`, is outside it.

### The crate mod-tempdir in place of `tempfile`, for its sweep of a killed run's files `##mod-tempdir-for-its-sweep`

`live`. It loses to a ranking, which `design@knowledge-architect@nothing-sweeps-a-killed-run` states:
adopting it ties the tests to a crate that is young and little used, a cost that
`curl -s -A curl https://crates.io/api/v1/crates/mod-tempdir | jq '.crate | .created_at, .downloads'`
shows, and that falls as the crate ages and its use grows. No fact defeats it. The names it gives by
default carry the process id and need no prefix of ours, so its sweep is available to the project; a
directory under a caller's prefix is outside the sweep. That sweep is not complete: it decides by
process id, so an orphan whose id a live process has taken is kept until that process exits. Both are
read in its source,
`path@knowledge-architect@docs/grounding/temporary-directory-crates.md`.

### The crate testdir in place of `tempfile`, which removes by count `##testdir-for-test-directories`

`live`. Refuted by a survey of crates that read their sources,
`path@knowledge-architect@docs/grounding/temporary-directory-crates.md`. It keeps the directories of
the last runs, eight by default and at least one by the type of its count, and has no removal at the
drop, so a run that passes leaves its files, which
`design@knowledge-architect@scratch-removed-by-a-guard` excludes. Its default place is the target
directory, inside this worktree, where a test whose project must lie outside every worktree cannot
work. It brings `cargo_metadata` and `sysinfo` into each package's tests.
