# Knowledge checker

**Nothing about a project is compiled into the tool**, per
`design@check@nothing-of-a-project-is-compiled-in`. Every list a check reads comes from the
project's `knowledge-architect.toml`, which is both the manifest and the marker that makes a
directory a project root. So one binary checks any project and a mock project under
`path@testing@tests/projects/` with no special case anywhere. A path that should not be checked
says so there, in one place, with a reason beside it. What is compiled in is the directory of each
Component a binary's libraries belong to — `knowledge_architect_check::component_dir` for `check`,
which is its crate's `CARGO_MANIFEST_DIR`, and the same function in each extension's library —
so that the string literals of the tool's own source are read as data, per
`design@check@checker-source-literals-are-data`. Each crate's own
directory and package name are compiled in too, so a binary built from another checkout of the
tool is refused, per `design@check@a-foreign-build-is-refused`.

**`git` 2.36 or newer is a hard dependency, and every invocation is the `git` crate's,
`path@git@src/git.rs`, which `check` names `crate::git`.** The floor is `cat-file --batch -z`, which
`commits` reads a commit's tree with. A check may spawn nothing, so that read is taken by the caller and handed in. The walk and the ignore batch are
`path@model@CLAUDE.md`. The decisions are `design@git@git-floor-for-commit-trees`,
`design@git@failed-git-is-an-error` and `design@knowledge-architect@checker-asks-git-through-the-crate`:
read them whole before changing how `check` asks git. `check --staged`, `index --staged`, the
comparison of `check --fix` and `commits` read a snapshot rather than the working tree, from git
objects, through `git::snapshot_entries` and the same `cat-file` batch: HEAD's tree overlaid with the
index's changes, or one commit's tree, per `design@check@staged-tree-source`.

What each mock project exercises, and where a defect is planted, is `path@testing@CLAUDE.md`.

**A check that produces something another check reads is a phase, not a check.** A run is
four phases, per `design@check@phases-gate-the-report`, and every check of the last phase
consumes the entity table and produces nothing another check reads; that is what lets a
finding be classified by where it is produced. A new check whose findings another check would
read goes before its consumers, in `model`'s `foundation`, per `path@model@CLAUDE.md`.

**An item is public only through the facade**, per `design@knowledge-architect@api-facade` and `design@check@the-facade-publishes-what-a-consumer-needs`: a re-export in
`path@check@src/lib.rs`, or one of the role modules `commands`, `extension`, `document` and `run`.
Every other module is private. lib.rs's `#![warn(unreachable_pub)]` refuses a `pub` item no
consumer can reach. So a new item a consumer needs is re-exported where its use belongs, and an
internal item is `pub(crate)`. A type in a public signature is public too, or the compiler reports
it. An integrated test drives the crate from `testing`, through this facade; what it needs and cannot reach is the facade's to fix where a consumer could see its effect, and otherwise the claim is a unit test's, per `design@check@the-facade-publishes-what-a-consumer-needs`.
**`check` dev-depends on nothing above it**, per `skill@knowledge-architect-component-creation@component-shapes`, **and holds no clap type**, per `design@cli@command-parsing-home`: a declaration of an argument is `path@cli@CLAUDE.md`'s.

Read `register@check@design` before changing how it works.
