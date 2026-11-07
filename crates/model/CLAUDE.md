# model

What knowledge-architect reads of a project. What it is for is `register@model@goal`; what it does for
a user is `path@model@README.md`; why it is shaped this way is `register@model@design`.

**An item is public only through the facade**, `path@model@src/lib.rs`, per
`design@model@model-facade`: a re-export in the root or in one of its role modules. An item another
crate's code calls is `pub` and named there; an internal item is `pub(crate)`.

**The walk and the ignore batch are this crate's questions to git, through `crate::git`**: the walk
is `git ls-files` from the project root, and the ignore question a path reference asks is one `git
check-ignore` batch per run. A phase may spawn nothing, so each is taken by the gathering and handed
in. A tree with no `git`, or a project outside a worktree, is an `Err` carrying the reason; an empty
walk is never an answer. The decisions are `design@model@git-supplies-the-walk`,
`design@git@failed-git-is-an-error` and `design@knowledge-architect@checker-asks-git-through-the-crate`:
read them whole before changing how this crate asks git.

**A new finding about what the model could not read — a home, a file, a declaration — goes in
`Manifest::parse`, `tree` or `Entities::build`, never in a module of the last phase, or nothing gates
it.** A new check whose findings another check would read goes before its consumers, in `foundation`,
and stops the run when it finds anything, per `design@check@phases-gate-the-report`.

**A test over a mock project is `testing`'s**, through this crate's facade, per
`design@knowledge-architect@testing-crate`. What each mock is for is `path@testing@CLAUDE.md`.
