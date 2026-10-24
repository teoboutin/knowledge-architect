# Tripwires — the core checker

Evidence that would reopen a decision about the core checker itself. **A tripwire is not something
to do**: it is a hypothesis about a future failure plus the response, and it leaves this file by an
exit of `skill@knowledge-architect-issue-tracking@movement-instruction`. What is outstanding about the tool is `register@core@issue` beside it.

Entry shape and the movement instruction between the two files are
`skill@knowledge-architect-issue-tracking`. The standing re-entry point is the standing-state
review, which re-reads every tracker file.

## Guarding `design@core@planned-path-form`'s scope: a planned file named outside the plans directory `##planned-path-outside-plans`

**Fires when:** `check` reports a `path@elsewhere@<path>` reference outside the plans directory
whose target now resolves, in a sentence that said the file was to be created.
**Response:** reopen the scope of `design@core@planned-path-form`, with every document as the
candidate.
**Re-entry:** the standing-state review of every dispatched review, and the commit that creates
the file, which fails `check` on that reference.
**Watches until:** the next two release tags.

## Guarding `design@core@at-span-candidate-rule`' silence on an unknown head `##candidate-rule-silence`

**Fires when:** a review, or a session reading a document, finds a backticked span that was
meant as a reference and for which `cargo klarch check` reported nothing — a typo inside the
kind, an unmigrated span whose head is neither a kind nor an anchor, or a shape the tokenizer
does not record. `cargo klarch model` filtered on the file shows whether the span was recorded as
a `span` observation at all.
**Response:** reopen `design@core@at-span-candidate-rule` on the shape found, with the rule widened
to it, under a false-positive census over the tree for the widened class, among the candidates; or,
where the decision is not reopened in the same change, open a `defect` naming the span.
**Re-entry:** the review of the migration that follows a grammar change.
**Watches until:** standing: a span the scanner leaves silent is reported by no later run, and the
class stays after each shape found is repaired, so its failure stays silent and permanent.

## Guarding `design@core@checker-source-literals-are-data`' whole-source scope `##checker-source-literals-are-data-whole-source-scope`

**Fires when:** a review finds a string literal in the tool's non-test source that cites a rule:
a `CR:` marker followed by a rule number written out, with a sentence around it that says
something about the rule. A message that interpolates the number it names, and the marker
pattern itself in the scanner, do not fire it. A review is where it surfaces, such as
the data check of thaum's rules-reviewer: nothing mechanical reads that literal any more.
**Response:** reopen the scope of `design@core@checker-source-literals-are-data`, with the rule
narrowed to test modules among the candidates, at the cost of an attribute lookup in the extractor;
moving the citation into the comment above the literal repairs the instance.
**Re-entry:** the intake of a retrospective file from a project whose binary registers an
extension.
**Watches until:** the next three commits on the main branch that add an analysis under
`path@knowledge-architect@docs/retrospective-reports/`.

## Guarding `design@core@checker-source-literals-are-data`' purpose `##checker-source-literals-are-data-purpose`

**Fires when:** a test module of the tool gains a named constant whose only use is to be
interpolated into a fixture so that its rule-shaped, slug-shaped or path-shaped bytes are not
spelled at the site. A constant of the module under test, and a constant the tests also read as
a value or reuse across several sites, are not this.
**Response:** open an `observation` naming the constant and the fixture.
**Re-entry:** a review of a change that adds a test module under the checker's own directories.
**Watches until:** the next five commits on the main branch that change a file under
`path@core@src/`.

## Guarding `design@core@issue-kind-list-closed`: a kind is added for a single situation `##issue-kind-list-grows`

The kind list was closed so that an unknown kind is a finding naming the list, and so that the
kind can decide which subsections an entry owes. The failure that would make the closing wrong is
the list growing instead of being chosen from: each new situation gets a kind of its own, the list
stops partitioning anything, and the finding stops carrying information.

**Fires when:** a kind outside the six compiled defaults, declared in an issue register of this
project or of a consuming project as a retrospective file the intake receives reports, ends up
with a single member across that project.

**Response:** reopen `design@core@issue-kind-list-closed`, with the open set,
`rejected@core@open-issue-kind-set`, among the candidates.

**Re-entry:** every manifest diff that touches `[registers.issue] kinds`, and
the standing-state review on every dispatched review.

**Watches until:** the next three release tags.

## Guarding `design@core@phases-gate-the-report`: a way around the whole-report stop `##phases-gate-the-report-two`

The report stops at the first phase that finds anything, and the whole report stops: the
per-document form lost on cost, and is `rejected@core@per-document-phase-gate`.

**Fires when:** a flag, an environment variable or a manifest row is added that lets a run report
a later phase while an earlier one holds findings, or lets a writer write over an incomplete
model: the gate felt as an obstacle and answered with a way around it, which is the check-family
selection under another name.
**Response:** reopen `design@core@phases-gate-the-report` with the per-document form,
`rejected@core@per-document-phase-gate`, among the candidates, rather than keeping the way around.
**Re-entry:** the standing-state review on every dispatched review, and any change to the
arguments of `check`, `index` or a writing command of an extension.
**Watches until:** the next three release tags.

## Guarding `design@core@phases-gate-the-report`: a second producer chain `##a-second-producer-chain`

The classification of a finding by the phase that produces it rests on one chain: the manifest
feeds the walk, the walk the model, the model the entity table, and every check consumes the table
and produces nothing another check reads.

**Fires when:** a check of the core, under `path@core@src/check/`, or a check of an extension
reads another check's findings, or a check's output is stored for another check to consume.
**Response:** open a `design` issue proposing the new phase and its place in the order; the
producing check is a phase, not a check of the last one, and it goes before its consumers in
`check::foundation`.
**Re-entry:** the standing-state review on every dispatched review; any new module under the
check directory; and any new check of an extension.
**Watches until:** the next ten commits on the main branch that change a file under
`path@core@src/check/`.

## Guarding `design@core@api-facade`'s membership `##private-item-needed`

**Fires when:** a consumer of the library, thaum's rules extension first, needs an item that is
private behind the facade, and no public item replaces it. An example still unmet: the records
listing, for an extension command that lists its own entries. The entity table and the anchors
fired it once, recorded as `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`.
**Response:** open a `todo` for the instance. Its fix gives the consumer what it needs under the
role module its use belongs to, as a 0.MINOR change: the item itself where its shape is fit to
publish, or a narrow read-only view of it where publishing it would expose the core's internals,
the direction the owner chose for the entity table. Add it to the facade's entry if it changes the
rule rather than the list.
**Re-entry:** thaum's migration onto the published crate, and each extension written after it.
**Watches until:** the next three release tags.

## Guarding `design@core@safe-fix-definition`: a fix that makes a choice `##fix-makes-a-choice`

**Fires when:** a fix is added to `check --fix` whose bytes are not fully
determined by the tree and the pinned version, or that writes or removes a file outside the
installer's namespace and the generated list, other than by the respellings
`spec@plans@path-quickfixes` defines, the respelling of a register's home among them, which reverse
this head at that spec's harvest. That harvest deletes this entry.
**Response:** reopen `design@core@safe-fix-definition`, rather than widen what `--fix` does under
its present argument.
**Re-entry:** the standing-state review before every merge, and any change that adds a fix.
**Watches until:** the commit on the main branch that deletes `spec@plans@path-quickfixes` at its
landing, which deletes this entry.

## Guarding `design@core@init-runs-git-init` and `design@core@init-asks-no-confirmation`: a mistaken run's cleanup `##init-cleanup-beyond-the-listed-files`

The decisions rest on the premise that the worst a mistaken run does is add files, which the
cleanup removes by deleting the files the run's output lists, under the refusal of a directory
holding files outside every worktree that the owner chose over following `cargo init` exactly.

**Fires when:** an issue or a retrospective finding reports an `init` run in an unintended
directory that needed a cleanup beyond deleting the files its output listed, or that lost a file.

**Response:** reopen `design@core@init-runs-git-init` and `design@core@init-asks-no-confirmation`
with the stronger guard, `rejected@core@init-refused-outside-a-repository-root` in its form that
refuses outside every worktree whatever the directory holds.

**Re-entry:** the standing-state review, and each intake of a retrospective file.

**Watches until:** the third release tag after the commit that writes this line, the first of them
the release that ships `init`.

## Guarding `design@core@git-supplies-the-walk`'s portability `##walked-count-differs-between-machines`

The walk is a `git` invocation, so what is checked depends on the git installed and on the
per-clone configuration it reads. The design answers the two loud cases: no binary and no
worktree are exit 2, per `design@core@failed-git-is-exit-two`. It pins the per-user ignore file
away, per `design@core@per-user-ignore-file-pinned`. This guards the quiet case that remains: two
machines running the same commit and reading a different set of files, with both runs green.

**Fires when:** the walked-file count CI prints for a commit differs from the count a local run
prints for the same commit. Both are the `walk: n file(s)` line of the summary block, which every
`cargo klarch check` prints on a stop as on a pass.
**Response:** open a `defect` naming both counts and the two git versions.
**Re-entry:** each time CI's output is read against a local run, and at any change to the walk.
**Watches until:** the commit that closes
`issue@core@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`, whose check then
carries it, and which deletes this entry.
