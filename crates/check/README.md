# The checker

How a project's knowledge is checked: its documents, the references between them and the registers
they carry. This Component is the generic part. A subject that belongs to one project is an
extension of it, per `design@check@an-extension-plugs-in-through-phased-hooks`.

The package is `knowledge-architect-check`, and a project that runs the checker or writes an extension does not name it: it pins
`knowledge-architect`, whose binary `klarch` and library are built on this one. Installing, the usage of each command but `check` and `commits`,
and the exit codes are `path@cli@README.md`. This file holds the rules a project's documents are
checked by. Its commands are written `cargo klarch <command>`, the command this repository declares.

## Who it serves

knowledge-architect serves one kind of project, a target project, which holds all of these, per
`goal@serves-one-kind-of-project`:

- the code it builds and ships is Rust, built by Cargo, apart from the code in another language that a platform it is built for requires;
- AI agents drive its development: its code, its documents, its entries and its reviews;
- its main branch has a linear history, with no merge commit;
- its owner has decided to use knowledge-architect.

It asks more of a target project than these conditions: among what it asks, the project is a git
repository, and its agents run under the `claude` harness. Nothing is guaranteed outside these
conditions, or where an ask is not followed.

**It needs `git` 2.36 or newer on the path, and a project inside a git worktree.** What the tool reads is what
`git ls-files` reports from the project root, so every pattern git honours decides the walk,
nested `.gitignore` files included, and a file git tracks is read whatever the ignore rules say.
A file whose name holds a line break, or a name Windows cannot create, is read by nothing and
reported once with the reason, any line break escaped: name it in `[walk] skip-files` or in an
ignore rule to keep it.
Neither failure is silent: no binary and no worktree are both exit 2 naming the reason, never an
empty walk. `commits` reads a commit's tree through `cat-file --batch -z`, which is where the
version floor comes from. `check --staged` reads the tree git's index would commit the same way:
HEAD's tree with the staged changes, a file added with `git add -N` left out, as `git commit`
leaves it out. The decisions are `design@model@git-supplies-the-walk`,
`design@git@failed-git-is-an-error`, `design@git@git-floor-for-commit-trees` and
`design@check@staged-tree-source`.
Git is asked about the repository the project's directory lies in, whatever `GIT_DIR`,
`GIT_WORK_TREE` or a variable like them names in the environment, and through the index
`GIT_INDEX_FILE` names where it is set. So `check --staged` runs as a pre-commit hook and judges
the tree the commit records, `git commit -a` included, per
`design@git@repository-variables-dropped` and `design@git@index-file-variable-kept`.

## `check`

**A run prints the summary first, its findings under it, and its verdict on the last line** —
`PASSED: no findings`, or `FAILED: n findings above`. The verdict is derived from the finding list
rather than tracked beside it, so it cannot disagree with the exit code. **The order is a
contract**: the summary block prints on a failing run as well as a passing one, so a reader taking
the tail of the output has to reach the verdict rather than the counts. When a finding sits in a
file git does not track, one note naming each such file follows the findings, above the verdict:
the walk reads untracked files, and a scratch file left in the tree fails the run like any other.

**The `checker source:` line names the binary's own source directories**, compiled into it, and
counts the walked files under them whose string literals are read as data rather than prose, per
`design@check@checker-source-literals-are-data`. A directory inside the tree is printed relative to
the root, and one outside it as an absolute path. A binary installed with `cargo install` names its
source in cargo's registry, outside the tree, so its count is 0 and nothing of the project is read
differently. A maintenance crate that runs the checker names its own directory, as the setup
skill's `main` does, so the literals of that crate's Rust files are data, and its comments stay
prose.

**A run is four phases, and it stops at the first that finds anything.** Phase 1 resolves the
manifest: a declaration the tool refuses is reported and acted on by nothing. Phase 2 reads the
tree against what the manifest declares: a file the walk could not read, a name it refuses, an
anchor or a register home that is not there, a home a walk row keeps out, a declared path that
does not exist, a file git both tracks and ignores, a symlink or a submodule, a heading markdown
reads and the checker does not, and, under the `claude` agent
harness, a root CLAUDE.md the walk keeps out. Phase 3 builds the entity table: a slug or an entry id where none may sit, or
defined twice. Each of these says the model is incomplete, and a finding computed from the model
afterwards would be unreliable in both directions, so the run prints that phase's findings, says
which phases were not judged, and exits 1. Phase 4 is every check, over the complete model:
`check`'s `agents`, `generated`, `registers` and `references`, each a module under
`path@check@src/check/`, then each registered extension's, per
`design@check@an-extension-plugs-in-through-phased-hooks`. `agents` judges, under the `claude`
harness, each installed agent file missing, differing or unshipped, a deletion of one not staged,
and a root CLAUDE.md that does not import a shipped primer. `references` judges
every `` `<kind>@<anchor>@<id>` `` reference against the entity table — a register kind against the
entries its home defines, the `path` kind against the tree, the `planned` kind against its absence
from the tree, the `register` kind against the anchors that carry the register, and, under the `claude` harness, a skill, an agent, the primer and the root
CLAUDE.md, cited with no anchor as `skill@<name>@<slug>` — and reports the unanchored path shape
where its first segment names something of this project, and a defined id written outside its reference form: a reference-shaped token outside backticks, per
`design@check@reference-shape-outside-span`, an id written in backticks alone, per
`design@check@backticked-bare-id`, and an id written as a plain word, per
`design@check@plain-word-lint`. An id several entries define gets one finding listing each
reference, and a quotation, between double quotes or on a blockquote line, is read by none of the
three, per `design@check@quotation-read-by-no-id-lint`.
In the goals home, which the root alone carries, its goals and its head alike, `references`
refuses every reference that is not a goal or a register's home of the root, per
`design@check@goals-home-cites-only-goals-and-root-homes`.
The directory of an anchor other than the root has no spelling under its own name, so a reference
names it from the anchor above it, as `path@<parent-anchor>@<dir>/`, per
`design@check@deepest-anchor-wins`. An illustration of a form writes its id as a placeholder
in angle brackets, as `path@*@<path>`, per `design@model@a-reference-is-live-in-every-prose`: a span
with an empty id is malformed.
`registers` judges the shape of what each anchor carries: a file
register's README, index, groups and entry shapes, and a directory home's links. A check the tree
gives no input to is printed as not run rather than counted. There is no way to select a subset:
the checks cross the phases, and a run over a passing tree costs under a second. The argument is
`design@check@phases-gate-the-report`.

`index`, `index --staged` over the staged tree, `check --fix`, `new-component` and a writing
command of an extension run the first three phases too, and
write no generated file while one of them holds anything; `init`, which writes where no project
exists yet, checks every destination before its first write instead, and writes the indexes only
once the three phases find nothing. `index` and an extension's command refuse
with exit 2; `check --fix` prints the stopped report and exits 1, having installed agent files
before that gate when they needed it, since their bytes do not depend on the model: an index generated over an incomplete model lists rows nobody asked for.

**`check --fix` applies the safe fixes `design@check@fix-scope` admits, then runs the check**, per
`design@check@check-fix-flag`. A fix is safe, per `design@check@safe-fix-definition`, when its
bytes are determined by the tree and the pinned version, and it writes or removes only files of
the installer's namespace or of the generated list. Two pass: the agent files the check reports missing,
differing or no longer shipped, judged from git's listing as the check judges them, so a file git
does not list, such as an ignored swap file, is never removed; and every stale or missing generated
file. Each file written or removed is listed on a line `fixed: wrote <path> (<kind>)` or
`fixed: removed <path> (installed)`, before the report. Every other finding's repair is a choice,
or touches git or a hand-written file, and stays the reader's.

- The order, per `design@check@fix-before-the-checks`: a manifest holding a refused declaration
  writes nothing; then the agent files; then
  the first three phases, over the tree as the install left it, which stop the run and write nothing
  more; then the refusal of a partial commit's mismatch, below; then the generated files; then the
  full check, whose report and exit code are the run's. With nothing to fix, the output is a plain
  `check`'s.
- **A partial commit's mismatch is refused**, per `design@check@fix-refusal-mixed-state`. Where the
  index differs from HEAD, and a generated
  file `--fix` would write differs from the one the tree git's index would commit needs, `--fix`
  writes no generated file and names each such file, with the two repairs: `index --staged`, then
  `check --staged`, to commit the staged changes; `index`, to fix the working tree as a whole.
  Staged with a partial commit, the files `--fix` writes would make a commit whose tree fails. A
  file already current on disk is not written, so it is not refused, and after `index` the run
  writes nothing and refuses nothing. It exits 2, or 1 when agent files were already installed.
  Where no staged tree is there to commit, an unmerged index, a staged tree holding no manifest or
  one stopped in phases 1 to 3, `--fix` compares nothing and runs; any other failure to compute the staged side,
  such as an extension that cannot prepare over a snapshot, refuses the same way, naming it.
- **`--fix` with `--staged` is refused** while parsing, exit 2: `--fix` repairs the working tree.
  For the staged tree, run `index --staged`, then `check --staged`.
- **An upgrade that removes a shipped file leaves its deletion to stage.** The install deletes the
  file, git still lists it, and the last phase reports the deletion as not staged, after the
  generated files are written. Stage it with `git add`: no fix touches git.
- A write that fails exits 2 when nothing was written yet, and 1 after any write, as `index` does.
- `--fix` belongs to `check` alone. `commits` judges history and takes no `--fix`, and the gates
  run `check` without it, so continuous integration judges the tree as committed.

## Registers

**Eleven registers are compiled in.** `design`, `tripwire`, `rejected` and `issue` are what the
word component means to this tool. `goal` is the root's alone: a goals home in any other Component,
and a location that declares it, is refused, and a goal is cited `goal@<id>`, its home
`register@goal`, with no anchor. `rejected` holds the designs a Component did not keep,
per `design@check@rejected-register`, each cited by the design entries whose argument covers it, and an entry no design entry cites is a
finding, per `design@check@orphan-rejected-entry`. `spec` and `milestone` are the plan documents, and `thread`,
`argument`, `criterion` and `acceptance` their items, described under "Plan documents" below. A project declares further ones in its
`knowledge-architect.toml`. The first two tables are from thaum, whose `rules` location carries
an `interpretation` register:

```toml
[locations.rules]              # a directory carrying a subset of the registers
path = "docs/rules"
registers = ["issue", "tripwire", "interpretation"]

[registers.interpretation]
scope = "opt-in"               # `component` (every component carries it) or `opt-in` (those naming it)
shape = "file"                 # `file` (one file per entry), `heading` (slugs in a home) or `directory`
dir = "interpretations"        # the basename of the home; defaults to the register's name
sections = ["Rules", "Reading", "Consequences"]

[registers.interpretation.metadata.status]
values = ["settled", "interpretation", "ambiguous", "cr-gap"]

[registers.note]
scope = "opt-in"
shape = "heading"
dir = "decisions"
level = 2                      # a heading register only: the heading level its entries sit at
```

The next tables are a game project's: its design split by subject, one directory per subject under
game-design/, each entry cited `game-design@<subject>@<id>`:

```toml
[locations.gdd]
path = "game-design"
registers = ["subject"]

[registers.subject]
scope = "opt-in"
shape = "directory"            # one directory per entry, each an anchor of its own
dir = "."                      # the home is the carrier's own directory: game-design/<subject>/
carries = ["game-design"]      # what each entry carries, at the entry's own directory

[registers.game-design]
scope = "opt-in"
shape = "heading"
level = 3
```

A **heading register** keeps its entries as slugged headings, in `<dir>.md` or in `<dir>/` behind
a `README.md` that links every subdocument. It declares `level`, from 2 to 6: every heading at that
level in its home is an entry and carries a slug, per
`design@model@an-entry-is-a-heading-at-the-register-level`, and a slug at any other level defines
nothing, per `design@model@a-slug-elsewhere-defines-nothing`. The built-in levels are 3 for
`design` and `rejected`, and 2 for `goal` and `tripwire`. A heading is a line that opens with `#` marks, at most
three spaces in, followed by a space or a tab and text. A setext heading, a heading after a list or
block-quote marker on its line, and a heading with no text are reported in any markdown document,
per `design@model@headings-open-with-hash-marks`.

A **file register** takes no `level`. It keeps one file per entry under `<dir>/`, beside a
hand-written `README.md`, a generated `index.md` and an optional `register.toml` declaring the
group subdirectories. An entry opens with frontmatter carrying each declared metadata key, then a
level-one title, then the declared sections.

A **directory register** keeps one directory per entry under `<dir>/`, beside a hand-written
`README.md` and a generated `index.md`. Each entry directory holds a `README.md`, the entry's
document, which owes a level-one title, beside the files of what it carries, and holds no
subdirectory and no `register.toml`. Each entry is an anchor named by its basename, cited as a
whole `<register>@<parent>@<id>`, the parent being the anchor that carries the home, and it carries
the one register `carries` names, an `opt-in` heading or file register, with its home at the
entry's own directory: a carried heading register's README is the entry's README, which links
every file and defines no slug. A `register` or a `path` reference to an entry's directory or to
its README is a finding naming the entry's reference. An entry's name is refused when it is a
kind's, a Component's, a location's or a reserved word, or when a milestone, a spec or an earlier
entry holds it: the milestones first, then the specs, then the declared registers by name, each
register's entries by path. The argument is `design@model@declarable-directory-register`.

**`dir = "."` puts a register's home at its carrier's own directory**, for an `opt-in` register
only: the location carrying it carries no other register, since every other home would sit inside
that one, per `design@model@home-at-the-carrier-path`.

**`components = [...]` attaches an `opt-in` register to the Components it names**, the root's name
included, each keeping its home at `<component>/docs/<dir>/`, per
`design@model@a-component-carries-what-a-project-attaches`. The game design above could sit at
`<root>/docs/game-design/` with no location: `dir = "game-design"` and
`components = ["<root>"]`, the root being named by `[project] name`, in place of the location and
`dir = "."`, each subject then cited `subject@<root>@<subject>`. A name that is no Component is a finding, and so is a home at the
Component's `docs/` itself, at the root's plans directory, or at the directory of another register
the Component carries.

`[registers.issue]` accepts `kinds` and nothing else; the design, goal, tripwire and rejected
registers accept nothing, `level`, `carries` and `components` included, and a table for a plan register or an
item register is refused whole. The
arguments are `design@model@registers-are-declared` and
`design@check@a-file-register-is-a-directory-of-entries`.

## Plan documents

**Every project carries a plans directory, docs/plans/ at its root**, and the tool constructs an
anchor named `plans` there. No manifest row declares it, and a declared anchor named `plans` is
refused. It holds three things and nothing else:

| path | what it is |
| --- | --- |
| docs/plans/README.md | hand-written: what the directory holds |
| docs/plans/specs/ | the `spec` plan type: one directory `<id>/` per spec, beside a hand-written `README.md` and a generated `index.md`; a subdirectory holding no `README.md` is a declared group of specs |
| docs/plans/milestones/ | the `milestone` plan type: one directory `<id>/` per milestone, beside a hand-written `README.md` and a generated `index.md` |

**Each spec's directory and each milestone's is an anchor of its own**, named by its basename. A
plan document's directory holds:

| file | what it is |
| --- | --- |
| `README.md` | the document the search reads: the decided design, the steps, the criteria, and "What is already decided" |
| `deliberation.md` | the Threads and Arguments, where the document has a thread or an argument |
| `search/` | the generated lines of a search run for the document, each a `.txt` file, `discussion.txt` and `design-audit.txt` |
| `transcript.txt` | the trimmed transcript of the discussion, where the document was assembled from one |

A milestone's directory holds besides its generated `index.md`, which lists its slices, one
directory per live slice, holding the same files as a spec's, and `harvested/`, holding the
directory of each slice that has landed. Any other file or directory is a finding, and so is a spec
written as one file. **A slice's directory is no anchor**: the milestone is the anchor of every file
under its directory. A `.txt` file is never parsed, and every file under `harvested/` is frozen: out
of the walk, read for its definitions alone, and cited from inside its milestone alone.
`cargo klarch commits` reports a commit that modifies a frozen file, or adds one other than by the
move that freezes a slice.

**A plan document holds fixed sections.** The `README.md` of a spec and of a milestone owes, in
order: Status and audience, How the work is done, Names, What the work is, What is already decided,
Criteria, "New names, in one place", Decided design, Mapping tables, Losing alternatives, Readings,
Premortem, Acceptance criteria, Implementation sequence, Order rationale, Defaults awaiting the
owner, Harvest, Later consequences. The `README.md` of a slice owes Builds, Claims, Audit subjects,
Fails alone on, Premises that expire. A `deliberation.md` owes Threads, then Arguments, which no
`README.md` holds.

**A plan's items are headings of four of those sections.** A level-three heading ending with its
slug, `### <statement> ##<id>`, under Threads, Arguments, Criteria or Acceptance criteria, defines
an item of the kind `thread`, `argument`, `criterion` or `acceptance`, in the plan's anchor: the
spec, or the milestone, whose files and slices' files share one namespace, a harvested slice's
included. Every level-three heading of those sections carries a slug; a slug anywhere else in a
plan defines nothing. An item is cited `<kind>@<plan>@<id>`, from inside its own plan's directory
only: a citation from outside it, a commit message included, is refused, and the finding names the
whole-document form.

**A plan document is cited by its kind, never by its path**:

```text
spec@plans@<id>               a spec, docs/plans/specs/<id>/
milestone@plans@<id>          a milestone, docs/plans/milestones/<id>/
spec@<milestone>@<slice>      a live slice, docs/plans/milestones/<milestone>/<slice>/
```

A `path` citation of a plan document, or of a file inside its directory, is refused, and the
finding names the form above. A harvested slice is cited `spec@<milestone>@<slice>` from inside its
milestone's directory alone; from anywhere else, the milestone is cited whole. A plan's `README.md` reaches
the files beside it by relative links. The plans directory's README, and the README and index of
each of its two homes, are cited by path, as in `path@plans@README.md`.

**A path a plan's work will create is cited `planned@<anchor>@<path>`**, from a document of the
plans directory only. Its anchor and path follow the rules of a `path` reference, and the target
must not exist: once it does, the finding asks for `path@<anchor>@<path>`, which the change that
creates the file writes. `*` and `elsewhere` are refused for it, and a register named `planned` is
refused like one named `path`.

**A register's home is cited `register@<anchor>@<register>`**, as `register@check@design` or
`register@*@issue`, and names the home in whichever shape the tree holds, so a home that outgrows
one file breaks no reference. The anchor is a component, a declared location or `plans`, or `*`
for a component-scoped register. A `path` reference to a home, a heading register's file, its
directory or that directory's `README.md`, a file register's directory, or a directory register's
directory, is a finding whose repair names this form; a `path` reference to an entry's directory or
its `README.md` names the entry's own reference. A register named `register` is refused like one named `path`.

**A plan's name reads as nothing else.** A milestone's name or a spec's id that is the name of a
component, of a location or of `plans`, `elsewhere` or `*`, and one name used under both homes,
are findings. So is a milestone name outside the id grammar, `[a-z0-9]+(-[a-z0-9]+)+`, per
`design@model@two-segment-ids`. An entry of
a declared directory register gives way to every plan of the same name.

A tree that breaks this layout stops the run at phase 2, except a home's missing `README.md` or
`index.md`, which is a finding of the last phase as in a file register's home. A misplaced or
duplicated item stops it at phase 3, and a missing section is a finding of the last phase. The arguments are
`design@model@plan-register`, `design@model@plans-split-dirs`, `design@model@plan-document-kinds`,
`design@model@a-placed-name-reads-as-nothing-else`, `design@model@spec-directory-is-an-anchor`,
`design@check@plan-document-sections-by-kind`, `design@model@plan-items-by-section`,
`design@model@plan-item-scope`, `design@model@a-harvested-slice-is-frozen` and
`design@check@a-frozen-file-is-never-modified`.

## Commit messages

**A commit message is a document under the regime.** It is parsed as one markdown document —
subject line, blank line, body — and every rule runs over it: every
`` `<kind>@<anchor>@<id>` `` reference resolves, and each registered extension judges it against
its commit's tree. The argument is
`design@check@a-commit-message-is-a-document`.

```sh
cargo klarch commits HEAD~1..HEAD        # the commit just made, where branch SHAs are not refused
cargo klarch commits origin/main..HEAD   # the branch's own commits
```

`commits` reads everything from each commit's own tree through git objects — the manifest, the
documents, the generated indexes, every file an extension reads — so a message is judged against the tree it
was written against, byte for byte and cleaned of nothing, per
`design@check@message-judged-byte-for-byte`. **A commit whose tree does not load or carries findings
fails the run**, the range's last commit included, with its tree's findings named by the commit and
the file, and its message is still judged where its tree reached the last phase, per
`design@check@every-commit-judged-by-the-tip-checker`. The summary block counts the commits, how many passed and how many failed, and a commit
fails on any finding of its tree, of its message, of a citation by SHA, or of a change of a frozen
file under a milestone's harvested/, per `design@check@a-frozen-file-is-never-modified`:

```
commits in origin/main..HEAD: 2 commits, 1 passed, 1 failed
  <sha> passed
  <sha> failed: its tree fails 1 finding(s); its message carries 2 finding(s)
```

A generated file that two or more commits of the range fail on gets one more line in the block,
naming the file and the repair that reaches each of them, since `index` at the tip rewrites the
last commit's file alone:

```
  docs/open-issues/index.md: 3 commits fail on this generated file; regenerate it at each of them and fold each regeneration into its commit
```

The checker that judges every commit is the one built from the working tree, so a branch
that makes it stricter puts that change in its first commit or is squashed before review, per
`design@check@every-commit-judged-by-the-tip-checker`. A message's references resolve against its
commit's tree **or its first parent's**, which is what lets a commit that closes an issue name it; a
parent whose manifest fails phase 1 is not read, and serves as no parent, per
`design@check@message-references-resolve-in-either-tree`. No hook judges a message before the
commit exists: the range is run after each commit, and a finding in the newest commit is repaired
by an amend, per `design@check@one-command-judges-messages-after-the-commit`.

**A project whose merges rewrite SHAs refuses citations of its branch's own commits.** Under
`[commits] refuse-branch-shas = true`, `commits` reports every run of 7 to 64 lowercase hex digits
that prefixes the SHA of a commit of the range, in a message or in a document of a commit's tree: a
rebase merge gives those commits new SHAs, and the citation would then name nothing. Name such a
commit by its subject. A SHA already on the main branch may be cited. The option is off when absent,
and is read from each commit's own manifest. A citation is seen only when the cited commit is in the
range judged, so with the option on, run the check after a commit over the branch, `<base>..HEAD`:
over `HEAD~1..HEAD`, a citation of an earlier commit of the branch passes, per
`issue@check@branch-sha-citations-are-judged-within-the-range-only`. The argument is
`design@check@branch-shas-are-refused`.

`check` reads no range of history, and the range is always explicit, per
`design@check@check-reads-no-range-of-history`. `cargo x gates` runs
`commits origin/main..HEAD` as a gate.

**The range need not end at the checkout.** Any range `git rev-list` resolves is judged, commit by
commit, and where HEAD sits changes nothing. The uses a range serves:

| use | range |
| --- | --- |
| the commit just made, before it is amended or built on | `HEAD~1..HEAD`, or `<base>..HEAD` under `refuse-branch-shas` |
| a branch's own commits, before review and merge | `<base>..HEAD` |
| CI over a pull request, which checks out the branch's head | `<base>..HEAD` |
| a pre-push hook: git gives it, per pushed ref, the local and the remote sha, and with `git push origin other-branch` the local sha is not HEAD | `<remote sha>..<local sha>` |
| a branch that is not checked out, reviewed locally | `<base>..<branch>` |
| an audit of old history, such as the commits made before a manifest migration | `<first>^..<last>` |

`<base>` is the branch the work merges into, as the remote holds it.

Two things are still read from the working tree, whatever the range: the binary that judges, per
`design@check@every-commit-judged-by-the-tip-checker`, and git's ignore rules, which `git check-ignore` answers for the files on disk only, per
`design@check@a-commit-message-is-a-document`.

## What to respect

**Nothing about a project is compiled into the tool.** Every list a check reads comes from the
project's `knowledge-architect.toml`, which is both the manifest and the marker that makes a
directory a project root. So one binary checks any project, a copy of a mock project included, with no special case. A path that should not be checked says so
there, in one place, with a reason beside it. What is compiled in describes the tool, not a tree:
the directory of each Component a binary's libraries belong to, so that the string literals of
the tool's own source are read as data. The decision is
`design@check@nothing-of-a-project-is-compiled-in`.

Read `register@check@design` before changing how it works.
