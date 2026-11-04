# model

What knowledge-architect reads of a project. What it is for is `register@model@goal`; what it does for
a user is `path@model@README.md`; why it is shaped this way is `register@model@design`.

**An item is public only through the facade**, `path@model@src/lib.rs`: a re-export in one of its role
modules, each with its body in that file. Every other module is private, and its unit test refuses a
`pub mod <name>;` and a glob. An item the core or another crate calls is `pub` and named there; an
internal item is `pub(crate)`. A method of `Model` or `Manifest` made `pub` is public on the core's
re-export too, so it is made public for a caller's code and never for a test.

**`git` 2.36 or newer is a hard dependency, and every invocation lives in the `git` crate,
`path@git@src/git.rs`, which this crate names `crate::git`.** The floor is `cat-file --batch -z`,
which `commits` reads a commit's tree with. The walk is `git ls-files` from the project root, the
ignore question a path reference asks is one `git check-ignore` batch per run, and a listing's
last-change column is one `git log`. A check may spawn nothing, so each of those is taken by the
caller and handed in. A tree with no `git`, or a project outside a worktree, is exit 2 with the
reason; an empty walk is never an answer. The decisions are `design@model@git-supplies-the-walk`,
`design@git@git-floor-for-commit-trees`, `design@git@failed-git-is-an-error` and
`design@knowledge-architect@checker-asks-git-through-the-crate`: read them whole before changing how this crate
asks git.

**A new finding about what the model could not read — a home, a file, a declaration — goes in `tree` or
in the resolver, never in a module of the last phase, or nothing gates it.** A new check whose findings
another check would read goes before its consumers, in `foundation`, and stops the run when it finds
anything, per `design@core@phases-gate-the-report`.

**A mock project stays under `path@core@tests/projects/`**, the one crate reaching another's fixtures,
until the shared test Component holds them. What each mock is for is `path@core@CLAUDE.md`.
