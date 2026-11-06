# Knowledge checker

**Nothing about a project is compiled into the tool**, per
`design@check@nothing-of-a-project-is-compiled-in`. Every list a check reads comes from the
project's `knowledge-architect.toml`, which is both the manifest and the marker that makes a
directory a project root. So one binary checks any project and a mock project under
`path@check@tests/projects/` with no special case anywhere. A path that should not be checked
says so there, in one place, with a reason beside it. What is compiled in is the directory of each
Component a binary's libraries belong to — `knowledge_architect_check::component_dir` for `check`,
which is its crate's `CARGO_MANIFEST_DIR`, and the same function in each extension's library —
so that the string literals of the tool's own source are read as data, per
`design@check@checker-source-literals-are-data`. Each crate's own
directory and package name are compiled in too, so a binary built from another checkout of the
tool is refused, per `design@check@a-foreign-build-is-refused`.

**`git` 2.36 or newer is a hard dependency, and every invocation is the `git` crate's,
`path@git@src/git.rs`, which `check` names `crate::git`.** The floor is `cat-file --batch -z`, which
`commits` reads a commit's tree with, and a listing's last-change column is one `git log`. A check may
spawn nothing, so each of those is taken by the caller and handed in. The walk and the ignore batch are
`path@model@CLAUDE.md`. The decisions are `design@git@git-floor-for-commit-trees`,
`design@git@failed-git-is-an-error` and `design@knowledge-architect@checker-asks-git-through-the-crate`:
read them whole before changing how `check` asks git. `check --staged`, `index --staged`, the
comparison of `check --fix` and `commits` read a snapshot rather than the working tree, from git
objects, through `git::snapshot_entries` and the same `cat-file` batch: HEAD's tree overlaid with the
index's changes, or one commit's tree, per `design@check@staged-tree-source`.

**Five mock projects for the binary's tests, each for one thing, under `path@check@tests/projects/`.** They declare no table
`check` does not own. The head comment of each `knowledge-architect.toml` says what its project
exercises, and that comment is where a session decides which one to touch. In short: `planted` is detection
in the last phase, one defect per built-in check, and it is clean through the phases before it so a
run reaches them, the family `agents` apart, which needs a served harness; `unsound` is the phase gate, one defect per assertion of phase 2 that a
committed file can hold and, behind them, the definition-site defects of phase 3, so a run over
it stops at phase 2. The phase-2 states no committed fixture can hold — a refused name, a symlink,
a file deleted or unreadable on disk, a nested repository — are built by the tests of
`path@cli@tests/binary.rs` in a copy; `dirhome` is
conformance, every built-in check running and finding nothing; `minimal` is the walk, the
exclusions, a location and a declared register; `core` is the smallest conformant project, and
the tests of `klarch`'s refusal of a table it does not own. **Plant a phase-4 defect in
`planted`, the family `agents` apart, a phase-2 or phase-3 one in `unsound`, and nowhere else.** `klarch` run from
`dirhome`, `minimal` or `core` reports no finding; anything else it reports there is a defect in
the tool or in the fixture.

**Neither `cargo klarch index` nor `cargo klarch check --fix` is run inside `planted`.** Its stale
and missing generated files are planted defects, and both commands rewrite them to the current
listing. A generated file of `planted` is written by hand.

**Every mock but `unsound` declares `harness = []`.** The shipped set changes with every version
of the skills, so a mock serving the `claude` harness would need the installed copy of that set,
and would change with it. `unsound` keeps the default harness, which it never reaches the last phase
to judge, since a run over it stops at phase 2. A test about the installed set
declares the harness in its own copy, and imports the primer from its root CLAUDE.md, as `Sandbox::serve_claude` in `path@cli@tests/binary.rs` does.

**A check that produces something another check reads is a phase, not a check.** A run is
four phases, per `design@check@phases-gate-the-report`, and every check of the last phase
consumes the entity table and produces nothing another check reads; that is what lets a
finding be classified by where it is produced. A new check whose findings another check would
read goes before its consumers, in `model`'s `foundation`, per `path@model@CLAUDE.md`.

**An item is public only through the facade**, per `design@check@api-facade`: a re-export in
`path@check@src/lib.rs`, or one of the role modules `commands`, `extension`, `document` and `testing`.
Every other module is private. lib.rs's `#![warn(unreachable_pub)]` refuses a `pub` item no
consumer can reach. So a new item a consumer needs is re-exported where its use belongs, and an
internal item is `pub(crate)`. A type in a public signature is public too, or the compiler reports
it. A test that needs a private item is a unit test inside the crate, never a reason to publish it.
**`check` dev-depends on nothing above it, and holds no clap type**, per the claims of the Component split: a declaration of an argument is `path@cli@CLAUDE.md`'s.

Read `register@check@design` before changing how it works.
