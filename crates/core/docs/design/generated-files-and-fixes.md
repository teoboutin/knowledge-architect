# Core design: Generated files and fixes

## Generated files and fixes

### A file register's index is a banner, a count and one row per entry, and its bytes are the contract `##a-file-register-index-is-rows`

The generated `index.md` of a file-register instance, in order:

| line | content |
| --- | --- |
| 1 | `**Generated — do not edit.** \`<command> index\``, with the project's declared command, per `design@core@declared-command` |
| 2 | blank |
| 3 | `<n> entries`, over every entry of the instance, grouped or not, and `1 entry` where the instance holds one |
| then | the ungrouped entries' table, under no heading |
| then | per group, a blank line, `## <group>`, and that group's table |

A table's columns are the register's declared metadata keys, **ordered by name**, then `title`
holding the entry's level-one heading as a link to the file, relative to the index's own
directory. Rows sort by the first metadata key and then by id, or by id alone where the register
declares none. Groups sort by name, and the ungrouped entries come first.

**A `|` inside a cell is escaped as `\|`.** It is the one character of a title or of a metadata
value that would otherwise end the cell and shift every column right of it, turning a listing into
a table that says something else. Nothing else in a cell is rewritten.

**The bytes are a contract because `generated` compares bytes.** Any change here fails the gate on
every committed index at once, which is the cost of the comparison being exact; the gain is that a
listing cannot drift by a character without being named.

**Rows only: no summary and no date.** A row changes on create, delete, retitle, regroup and a
metadata change, and on nothing else, so an index is regenerated rarely. A summary column would
restale it on every wording edit, and a date would break
`design@core@generated-files-are-pure` outright. Both are `rejected@core@index-summary-column` and
`rejected@core@index-last-change-date`.

**A Directory register's index has the same bytes with one column before the title, `id`**: one
row per entry, its id, then its README's level-one title linking `<id>/README.md`, sorted by id. Its rows come from the
entries, which are anchors, so no file inside an entry is a row; the entry's own index lists those.

**The rows come from the walked entries, not from the instance's declarations.** A group is the
subdirectory an entry sits in, so an undeclared group still lists its entries and a declared group
holding none prints no heading; an entry whose id, title or metadata the shape check refuses is
still listed, under its basename and an empty cell. A listing that hid what the directory holds
because a declaration disagreed would be a listing that stops listing, and the declaration is
`check::registers`' to report.

The design this decision did not keep: `rejected@core@lines-flag-on-interpretation-index`.

### A generated file's content is a function of the walked tree `##generated-files-are-pure`

No generated file holds hand-written bytes: not a file-register `index.md`, and not a file an
extension generates. Each is what its generator returns over the model, which is what lets
`path@core@src/check/generated.rs` verify one by regenerating into a `String` and
comparing rather than by writing the file and reading it back.

**The purity is a property to preserve, not one to observe.** A generator that embedded a
timestamp, a hostname, or anything the walk does not see would break both consequences at once: the
check would report a file stale that nobody had changed, and `index` would rewrite on every run.

### `index` takes no dry-run flag, since writing a generated file destroys nothing `##index-takes-no-dry-run`

**Writing a generated file destroys nothing**, since its content is a function of the walked tree,
per `design@core@generated-files-are-pure`, so `cargo klarch index` takes no dry-run flag. Its one
flag, `--staged`, chooses the tree and the destination, per `design@core@index-staged-write`. The
dry run a write command normally owes exists because a write can lose something, and here it
cannot: the worst a run can do is replace a file with what the tree says that file is. It writes
only where the bytes differ, so running it to look moves not even an mtime, and it names each file
it rewrote.

**That last claim is held by two refusals, not by the paths being generated.** `fs::write` follows
a symlink and writes through it, so a generated path that is one would replace content this
command never produced; and a destination directory the manifest declares but the tree does not
have is a manifest defect rather than something to create. Both are checked over **every**
destination before any is written, so a run either refuses having written nothing or writes them
all. Remove either check and the paragraph above stops being true.

### The writer and the `generated` check take the generated files from one list `##one-list-of-generated-files`

**One list names the file-register indexes, and the writer and the gate both read it.**
`cargo klarch index` and the `generated` check take the same pairs of destination and expected
bytes from one function, so neither can generate one the other does not know about, nor disagree
about what is in it. Each is derived from the file-register instances, one per instance whose
directory is there. **An extension's generated files are read the same way**: the extension
names their paths before the walk and renders their bytes when prepared, and the writer and the
gate both take them from those two calls, per
`design@core@an-extension-plugs-in-through-phased-hooks`.

**An instance with no directory contributes no index**: generating into it would create a register home as a side effect of a listing, and the
missing home is what `check::tree` reports.

### One question has one command that answers it `##one-question-one-command`

Verifying is not `index`'s question. The `generated` check of `cargo klarch check` is the gate,
and it names the first line at which the committed file and the regenerated one disagree. A second
command answering the same question in its own format is what drifts, and there would be nothing to
say which of the two was right.

The design this decision did not keep: `rejected@core@index-diff-with-write-flag`.

### `anchors` lists every anchor of a project, computed as `check` computes them, with the home of each register it carries but a plan's `##anchors-listing-command`

`klarch anchors` prints every anchor, the constructed ones included, built by `Anchors::of` over the
manifest and the tree's listing as every check builds them: its name, its kind and its path, and,
under every anchor but a plan's, each register it carries with the home the tree holds,
the file, the directory, both, or `missing`. An entry's kind is the Directory register it is an
entry of. A milestone and a spec list no register: a plan
anchor's registers keep their homes inside the plan itself, its sections, or a milestone's own
directory for its slice specs. A home is judged held by the rule `check` judges it by, over the
same listing, so the two agree on what a project carries. It exits 1 when a home is missing, a negative answer
about the project, per `design@core@exit-code-ladder`. Its output is for a reader and is not a
contract, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

It answers a question no other command answers: `model` prints observations, and `show` one
register's homes. The rival, each project's root `CLAUDE.md` listing its anchors by hand, goes stale
when the manifest changes, and no check reports the stale list; the command is computed from what
it describes. The primer names it, so every installing project's sessions learn its anchors from
it.

### `index --staged` writes the staged tree's generated files into git's index, and no working-tree file `##index-staged-write`

`cargo klarch index --staged` generates each file from the tree git's index would commit, per
`design@core@staged-tree-source`, and sets the staged entry of each whose staged bytes differ, or
that the index lacks, leaving the working-tree file as it is. After it, `check --staged` passes on
what will be committed, and plain `check` judges the working tree as before. It serves a commit
of part of the working tree when an unstaged change touches the same register: the `index.md` that
commit needs then differs from what `index` writes, and without this command the repairs are a
hand edit of a generated file or moving the unstaged file away.

- It gates the staged tree on phases 1 to 3, per `design@core@phases-gate-the-report`, and refuses,
  having staged nothing, a destination the index holds as a symlink, a gitlink or a directory, asked
  of the index itself so that no walk row hides it, or whose directory the staged tree does not
  hold. A symlink's or a gitlink's mode would carry the generated text, and `--index-info` would
  replace the entries under a directory.
- It stages every entry in one locked write, so the index changes for all of them or for none: a
  held lock is exit 2 with the index as it was. How, and what a refused run can leave in the object
  store, is the doc comment of `stage_generated` in `path@core@src/git.rs`.

It writes into git, where `design@core@safe-fix-definition` refuses a fix that touches git. That
test is the one `--fix` applies before a fix, and `--fix` never applies this one, per
`design@core@fix-refusal-mixed-state`. What it writes is the staged entry of a generated file,
whose bytes the staged tree determines, so no bytes a writer meant are lost. The rival, writing
the staged rows into the working-tree file, lost: after it, plain `check` fails, since that file no
longer matches the working tree, and `check --staged` fails until the file is staged.

The design this decision did not keep: `rejected@core@index-staged-writes-working-file`.

### `check --fix` applies the safe fixes `design@core@fix-scope` admits, then runs the full check `##check-fix-flag`

`cargo klarch check --fix` applies the safe fixes `design@core@fix-scope` admits, lists each file it
wrote or removed, then runs the check, whose report and exit code are the run's. It makes an edit cycle
one command: updating the generated files is needed after nearly every edit of a register, and
without the option it takes `index` then `check`. The option came from the owner: "it's quite
common for tools similar as this one to provide a "quick fix" option, that performs the fixes it
can safely do automattically", as `cargo clippy --fix` and `eslint --fix` do, and the same words
lean "toward using a generic name for the cli option". Without `--fix`, `check`
is unchanged, and `index` and `install-agent-skills` stay as commands, each for one fix alone. The
option takes a generic name because a later safe fix needs no new option. The rival, a separate
`fix` command that writes and then runs the check, lost: with the option, the check stays one
command rather than two that end in one report. `commits` judges history and takes no `--fix`,
and the gates run `check` without it, so continuous integration judges the tree as committed. It
refuses a partial commit's mismatch, per `design@core@fix-refusal-mixed-state`, and `--staged`,
per `design@core@fix-with-staged`.

### `check --fix` refuses where a generated file it would write differs from the staged tree's, not wherever the tree holds staged and unstaged changes `##fix-refusal-mixed-state`

**Where the project's part of the index differs from HEAD, and a generated file `--fix` would
write differs from the one the tree git's index would commit needs, `--fix` writes no generated
file.** It names each such path, a path the staged tree does not need included, with the two
repairs: `index --staged` then `check --staged`, to commit the staged changes, and `index`,
to fix the working tree as a whole. `--fix` cannot lose unstaged content, since it writes only
generated and installed files; the hazard is a wrong commit, since the `index.md` it writes
reflects the working tree, and staged with a partial commit it makes a commit whose tree fails. A
file already current on disk is not written, so no refusal names it: the hazard is in what `--fix`
writes, and refusing a run that writes nothing would refuse one that `index`, the repair it names,
cannot clear. Where no staged tree is there to commit, an unmerged index, a staged tree holding no
manifest, as one where the manifest is written and not yet staged, or a staged tree stopped in
phases 1 to 3, nothing is compared and the fix runs. Any other failure to compute the staged
side refuses, naming it, such as an extension that cannot prepare over a snapshot: a comparison
skipped in silence lets the partial commit through, per `design@core@a-failed-parse-is-loud`. The
refusal is exit 2, or 1 where agent files were installed before it, since 2 promises an untouched
tree.

A refusal wherever the tree holds both staged and unstaged changes loses: it also fires where
the unstaged changes touch no input of a generated file and the fix is correct for both trees,
which blocks the edit-then-`--fix` loop for nothing. A printed note in place of the refusal loses: a
line in a long report is missed where an exit code is not.

### `--fix` with `--staged` is refused while parsing `##fix-with-staged`

The refusal is clap's `conflicts_with`, per `design@core@arguments-parse-through-clap`. `--fix`
repairs the working tree. Combining them would tie every fix to a write into git's index. The
owner's words, given with the ruling on `index --staged`, and hedged: "This is not definitive, but
I would not recommend to use this with --fix though. The reason  is that I'd like to provide more
types of quick fixes later, and this might not be compatible."

### A fix is safe when its bytes are determined by the tree and the pinned version, and it writes or removes only files of the installer's namespace or of the generated list `##safe-fix-definition`

This is the test every fix must pass before `--fix` applies it. The installer's namespace counts
whole, per `design@core@owned-namespace-check`: an unshipped file the check reports there is
removed, as the install removes it, whoever put it there. The rival, removing only the files the
install itself wrote, has nothing to tell them by: ownership is decided by a name, with no record
and no history, per that head. A fix that makes a choice, or touches
git or a hand-written file, would rewrite what a writer meant, so its repair stays the reader's.
The test is what keeps `--fix` safe to run after every edit, which serves
`goal@knowledge-architect@agents-work-without-drift`. `tripwire@core@fix-makes-a-choice` watches
it.

### `--fix` applies only the kinds of repair added to it on purpose, each passing the safe-fix test `##fix-scope`

`--fix` applies a repair only when it passes `design@core@safe-fix-definition`. Passing the test
makes a repair admissible, not owed: each kind of repair `--fix` applies is a member added on
purpose. The direction of the rule is the owner's, given against a title stating that `--fix`
repairs every finding whose repair passes the test: the owner put the rule the other way round,
`--fix` admitting a repair only when it passes the safe-fix test, since otherwise it becomes a
source of defects where the project may never have meant to cover some repairs.

`--fix` applies two kinds of repair today: an installed file missing, differing or no longer
shipped, and a stale or missing generated file. Every other repair touches git, such as staging a
deletion, or a hand-written file, such as the root CLAUDE.md's primer import, or is a choice. The
installed set is repaired exactly where the installed-file check reports it, judged from
git's listing after normalising line endings: a file git does not list, such as an ignored file in
the namespace, is never touched. The generated files alone, the rival, lost: repairing the installed set costs little, and it removes the
install-then-index sequence from an upgrade.

The design this decision did not keep: `rejected@core@fix-converting-old-entries`.

### A fix run repairs the installed files, gates the model, refuses a partial commit's mismatch, writes the generated files, then checks `##fix-before-the-checks`

The order of a `--fix` run: `--fix` with `--staged` is refused while parsing; a manifest holding a
refused declaration writes nothing; the installed files are repaired; phases 1 to 3 run over the
tree as the repairs left it, and a finding there stops the run, printed as `check` prints a stop,
with nothing more written; a partial commit's mismatch is refused, per
`design@core@fix-refusal-mixed-state`, after the installed repairs, whose bytes are the same for
both trees, and after the gate, whose model the comparison reads; the generated files are
written, from the list `index` reads, after every destination is checked; the full check runs. The
installed files' bytes do not depend on the model, so they are repaired before the gate, while a
generated file is never written over an incomplete model, per `design@core@phases-gate-the-report`.
An upgrade that removes a shipped file takes two runs: the deletion is unstaged, phase 2 reports
it, and staging it touches git. The rival, `--fix` treating the deletions its own install just made
as done, adds a special case to the phase gate. The cost of two runs is small by the owner's
weighing: the owner judged deleting or renaming a skill rare, and several invocations to handle it
not much of a problem. A failed
write exits 2 with nothing written, and 1 after any write, since 2 promises an untouched tree. The check stays the only verifier: `--fix` only writes
before it verifies, so the rejected design of an `index` that prints a diff and a `--write`
that applies it, a second command answering whether a file is current in its own format,
`rejected@core@index-diff-with-write-flag`, is not reopened.
