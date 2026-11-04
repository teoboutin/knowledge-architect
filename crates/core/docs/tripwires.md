# Tripwires — the core checker

Evidence that would reopen a decision about the core checker itself. **A tripwire is not something
to do**: it is a hypothesis about a future failure plus the response, and it leaves this file by an
exit of `skill@knowledge-architect-issue-tracking@movement-instruction`. What is outstanding about the tool is `register@core@issue` beside it.

Entry shape and the movement instruction between the two files are
`skill@knowledge-architect-issue-tracking`. The standing re-entry point is the standing-state
review, which re-reads every tracker file.

## Guarding `design@core@checker-source-literals-are-data`: a literal of the tool's non-test source cites a rule `##checker-source-literals-are-data-whole-source-scope`

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

## Guarding `design@core@api-facade`: a consumer needs an item that is private behind the facade `##private-item-needed`

The decision rests on the premise that the role modules publish every item a consumer of the
library, a binary, an extension or the extension's tests, needs.

**Fires when:** a consumer of the library, thaum's rules extension first, needs an item that is
private behind the facade, and no public item replaces it. An example still unmet in this library's facade: the records
listing, for an extension command that lists its own entries, which `model` publishes. The entity table and the anchors
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

