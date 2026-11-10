# testing

What an extension's tests need, and this repository's mock projects with the tests over them. What it
is for is `design@knowledge-architect@testing-crate`; what it does for a user is `path@testing@README.md`; why it is shaped
this way is `register@testing@design`.

**Nothing of the repository depends on this crate**, and it dev-depends on `cli` by path alone, per
`design@knowledge-architect@testing-crate`, which `path@testing@tests/dependency_rules.rs` holds for each crate of the workspace.

**A test of the binary asks for it with `klarch()`**, in `path@testing@tests/binary.rs`, which builds it, per
`design@testing@nested-cargo-builds-the-binary`. A mutation of the binary's code owes a `--build` probe that
builds it, such as `cargo build -p knowledge-architect --bin klarch`, or a mutant that does not compile
reads as caught.

**The test files here are outside the walk**, per `design@testing@test-files-outside-the-walk`: a
reference in one of their comments is checked by nobody, so a rename is found here by a search.

**A test states a tree's state in a copy.** `path@testing@tests/checks_over_mocks.rs` makes an absent, a current or an
edited generated file in a copy of `planted`, since no caller assembles what a run reads, per
`design@check@inputs-are-gathered`. Such a state changes every check that reads the tree, so a count in
`PLANTED` is of the state its test makes.

**Five mock projects, each for one thing, under `path@testing@tests/projects/`.** They declare no table
`check` does not own. The head comment of each `knowledge-architect.toml` says what its project
exercises, and that comment is where a session decides which one to touch. In short: `planted` is detection
in the last phase, one defect per built-in check, and it is clean through the phases before it so a
run reaches them, the family `agents` apart, which needs a served harness; `unsound` is the phase gate, one defect per assertion of phase 2 that a
committed file can hold and, behind them, the definition-site defects of phase 3, so a run over
it stops at phase 2. The phase-2 states no committed fixture can hold — a refused name, a symlink,
a file deleted or unreadable on disk, a nested repository — are built by the tests of
`path@testing@tests/binary.rs` in a copy; `dirhome` is
conformance, every built-in check running and finding nothing; `minimal` is the walk, the
exclusions, a location and a declared register; `core` is the smallest conformant project, and
the tests of `klarch`'s refusal of a table it does not own. **Plant a phase-4 defect in
`planted`, the family `agents` apart, a phase-2 or phase-3 one in `unsound`, and nowhere else.** `klarch` over a pinned copy
of `dirhome`, `minimal` or `core` reports no finding; anything else it reports there is a defect in
the tool or in the fixture. In place it refuses the pin, per `design@testing@binary-tests-in-testing`.

**A generated file of `planted` is written by hand.** Its stale and missing generated files are planted
defects, which `index` and `check --fix` over a copy would rewrite to the current listing, and which
`klarch` refuses to touch in place.

**Every mock but `unsound` declares `harness = []`.** The shipped set changes with every version
of the skills, so a mock serving the `claude` harness would need the installed copy of that set,
and would change with it. `unsound` keeps the default harness, which it never reaches the last phase
to judge, since a run over it stops at phase 2. A test about the installed set
declares the harness in its own copy, and imports the primer from its root CLAUDE.md, as `Sandbox::serve_claude` in `path@testing@tests/binary.rs` does.

**A test that copies a mock project runs `git init` and `git add -A` in the copy**, or the walk
is empty and the test proves nothing; `Sandbox` in `path@testing@tests/binary.rs` does it. Two
consequences a test about ignore behaviour has to choose between, because the ignore rules act on
untracked files alone:

- for a file that must be **ignored**, write the `.gitignore` before the first `git add -A`, so
  the add never tracks it. `Sandbox::seeded` is that order.
- for a file that must be **tracked and ignored**, stage it first and add the `.gitignore`
  after, so the add finds the file already in the index.

**A test that runs git through `Sandbox` never depends on the machine's git identity**:
`Sandbox::git_command`, which `Sandbox::git` runs, gives every invocation the sandbox's identity on
its command line and removes the environment variables that would override it. Other machine configuration still
reaches some helpers, per `issue@check@the-tests-read-the-machine-s-git-configuration`.

A test that deletes or rewrites a fixture file after the first add stages again, or git's listing
still names what the working tree no longer holds. A test in process over a mock in place needs none
of this: it builds under this repository's own worktree, so a mock file is walked because this
repository's listing holds it. The listing is `--cached --others --exclude-standard`, so a new fixture
file is walked the moment it exists, tracked or not; what removes one is an ignore rule, and what a
deletion needs is staging, or the path stays in the listing with no bytes behind it.

**A test about commit messages builds its own project.** `commits` fails every commit whose tree
carries a finding, the last one included, so each commit a test makes has a tree with none,
except the one it plants. `dirhome` is the one mock over which every family runs
and finds nothing, so a copy of it could serve as a base; `History` in
`path@testing@tests/binary.rs` writes a project out
anyway, because each test states the exact findings the commits it makes carry and a mock's
contents are shared with every other test over it. `History` then configures `user.name` and
`user.email` in that project's own configuration, and commits. Two orders it has to keep:

- **stage, regenerate the indexes, stage again.** The walk is git's listing, so a deleted entry
  the index still holds is still counted and `cargo klarch index` writes the listing the tree
  no longer has.
- **`--allow-empty`** on a commit whose subject is the message rather than a tree edit.

