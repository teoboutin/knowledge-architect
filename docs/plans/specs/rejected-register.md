# Rejected designs are entries of a register, `rejected`, cited by the design heads that did not keep them, and a design discussion is grounded by a search of the design heads and those entries

## Status and audience

This spec is the plan of the work that turns each Component's rejected-alternatives document into
the home of a built-in heading register, `rejected`, and that sends a search of the design heads
and of the `rejected` entries to the grounding of every design discussion. The work does five
things:

- the checker gains a built-in component register `rejected`, home `planned@<anchor>@docs/rejected.md` or a
  `planned@<anchor>@docs/rejected/` directory, entries at level three; the rejected-alternatives document leaves the
  set of documents every Component carries;
- the checker reports a `rejected` entry that no design entry cites;
- this repository's 132 rejected alternatives become `rejected` entries, each cited by the design
  heads that did not keep it, and no entry names a winner it "lost to";
- the installed text records and cites a rejected design that way, and a new installed agent, the
  design-entry searcher, reads the design home and the `rejected` home of one Component whole at
  the grounding of a design discussion and grades each entry;
- the changelog's Migration entry tells an adopting project how to convert.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. The owner has ruled on D1 to D5, each marked ruled, with the ruling, under "Defaults
  awaiting the owner"; a default a later review raises is built as written, and the branch that
  carries it merges once the owner has ruled on it.
- **The spec lands before its work, in a pull request of its own**, per D2. The work's first commit
  changes what `cargo klarch commits` requires of every tree, so on one branch the spec's own
  commit would fail that gate, per `primer@branch-commits` and
  `skill@knowledge-architect-planning@plan-reviews`.
- **This spec names the rejected-alternatives documents in words, or by a `path` span whose anchor
  is a placeholder, never by a span that resolves.** The work removes them while this spec is still
  in the tree, and a resolving span would dangle in every commit of the work.
- It was assembled from one transcript. The discussion begins at the owner's message that opens "I
  would like to make rejected alternatives into a proper register with named slugs", and its last
  message of the owner is "You can keep both tripwires and both AC". It holds 6 owner messages,
  called rounds 1 to 6 below, and the agent's replies after rounds 1 to 5, called "the reply to
  round 1" and so on; round 6 arrived while the agent wrote its reply to round 5. Find the file by
  that opening message, not by its name. Two of the agent's replies were notes that the discussion
  waited on the owner; each answered an automatic message the session had scheduled for itself,
  "Continue the rejected-register design discussion once the standing-entry searchers report.",
  which is not the owner's and is not counted as a round. The reviews of this spec raised D5, a
  material finding against `thread@rejected-register@design-entry-search`, which the agent put to
  the owner after round 6.

## How the work is done

Under `skill@knowledge-architect-planning@working-a-slice`, as its paragraph on the work of a spec
says.

## Names

- **a `rejected` entry**: a level-three heading of a `rejected` home, its statement first and its
  slug last, and the text under it to the next heading of level three or less. Cited
  `rejected@<anchor>@<id>`.
- **the rejected-alternatives document**: the file `path@<anchor>@docs/rejected-alternatives.md` of a Component,
  which the checker requires today through `COMPONENT_DOCUMENTS` in `path@core@src/manifest.rs`.
  The work removes it.
- **an old entry**: one alternative of a rejected-alternatives document as it stands before the
  work. In the root's, agent-skills' and core's documents it is a paragraph that opens with the
  alternative's name in bold, then "— lost to" and one or more references, then a marker, `live`;
  in gates' document it is a level-two heading, then a paragraph that opens with "Lost to
  `<reference>`. `live`." and goes on with the reason.
- **the lost-to clause**: the part of an old entry from "lost to" to its marker, the marker
  excluded.
- **a citing head**: a design entry whose section holds a reference to a `rejected` entry.
- **an orphan entry**: a `rejected` entry that no citing head cites.
- **a head of absence**: a design entry that records a decision not to build something ("we do not
  do X"), which entry test 2 of `primer@design-heads` admits as a decision about an absence.
- **the design-entry searcher**: the installed agent this work adds, `knowledge-architect-design-entry-searcher`,
  whose source is `planned@agent-skills@content/agents/design-entry-searcher.md`.
- **the standing-entry searcher**: `agent@knowledge-architect-standing-entry-searcher`, which
  exists.
- **a grade**: what the design-entry searcher returns for one entry it read: `bears`, with one of
  the relations `constrains`, `may change` or `may bring back`; `may-bear`; or unrelated.
- **the delegated-state entry**: the old entry of agent-skills that opens "A delegated state, under
  which the owner hands a decision to the agent within a stated boundary", whose lost-to clause
  names only `goal@knowledge-architect@the-owner-decides`.

- **C1 to C6**: the criteria as the reply to round 1 numbered them, in the order of the Criteria
  section: C1 `criterion@rejected-register@records-the-reason`, C2
  `criterion@rejected-register@reaches-the-reader`, C3 `criterion@rejected-register@no-inbound-list`,
  C4 `criterion@rejected-register@rules-are-checked`, C5
  `criterion@rejected-register@filter-unchanged`, C6 `criterion@rejected-register@adopter-cost`.
- **the recorded case**: the case of the issue `a-proposal-is-read-against-the-heads-only-after-the-plan-is-written`,
  which this spec's branch closes, in the commit of its review repairs: in the discussion that produced
  `spec@plans@path-quickfixes`, the owner approved a proposal that contradicted a core design head
  the session had not read, and the spec's reviewers found it.
- **the shapes A, B and C** of `thread@rejected-register@rejected-citation-direction`: A, both
  directions of the link written by hand; B, the head cites, and `show` computes the link back; C,
  both directions, with a check that they agree.
- **the shapes a1, a2, b and c** of `thread@rejected-register@orphan-rejected-entry`: a1, only a
  citing design entry makes an entry reachable; a2, a goal may cite it; b, a citing head or the
  entry's own citation of a goal; c, no orphan check. They are no arguments: an argument is
  `argument@rejected-register@a<n>`.
- **the shapes p1 and p2** of `thread@rejected-register@design-entry-search`: p1, one searcher per
  Component; p2, a listing command and groups of 60 rows. D5, as the owner ruled it, amends p1.
- **the shapes r1 and r2** of `thread@rejected-register@checkpoint-reader`: r1, the session's own
  reversal search over the checkpoint's rows; r2, the design-conformance reviewer dispatched on them.

## What the work is

### The checker today

- `COMPONENT_DOCUMENTS` in `path@core@src/manifest.rs` holds `README.md`, `CLAUDE.md` and
  `path@<anchor>@docs/rejected-alternatives.md`; `Manifest::required_documents` filters it, and `check` in
  `path@core@src/check/tree.rs` reports a Component that lacks one, "the component `<name>` carries
  no <document>".
- `Registers::built_in` in `path@core@src/manifest.rs` declares ten registers: `design` (level 3),
  `goal` and `tripwire` (level 2), `issue`, `spec`, `milestone`, and the four item registers. A
  heading register declared there gets its homes, its entity definitions, the refusal of its name
  as an anchor, `show` and the `anchors` listing with no case of its own: `Anchor::home_of`,
  `Entities::heading_definitions` and `is_kind_name` in `path@core@src/entity.rs`, and
  `records::records` in `path@core@src/records.rs`, read the register list.
- `resolve_registers` in `path@core@src/manifest.rs` refuses a component register whose
  `docs/<dir>.md` is in `COMPONENT_DOCUMENTS`; once the set holds no file under `docs/`, the branch
  can match nothing. Its test is
  `a_register_directory_wearing_a_compiled_document_s_name_is_reported` in
  `path@core@src/check/registers.rs`.
- `guarded` in `path@core@src/records.rs` counts as guarded every register kind but `issue` and
  `tripwire`, so a `rejected` reference in a tripwire would show in the `guarding` column of
  `cargo klarch tripwires`.
- No check of the last phase walks references by their target. `records::inbound` in
  `path@core@src/records.rs` finds the sites citing one entry, for `show` only. `CHECKS` in
  `path@core@src/check/mod.rs` names the three modules of the last phase: `generated`, `registers`,
  `references`; `PLANTED` in `path@core@src/mock_projects.rs` holds one count per module.
- Seven fixture files hold a rejected-alternatives document: the mock projects `core`, `dirhome`,
  `minimal`, `planted` and its Component `parts/widget`, `unsound` and its Component
  `parts/widget`, under `path@core@tests/projects/`. Tests that list the document or the four
  component registers: in `path@core@src/manifest.rs`, `path@core@src/check/mod.rs`,
  `path@core@src/check/registers.rs`, `path@core@src/check/references.rs`,
  `path@core@src/entity.rs`, `path@core@src/mock_projects.rs`, and `path@core@tests/binary.rs`,
  which also builds projects by hand that carry the document.
- `the_primer_s_model_section_names_every_kind_and_reserved_word` in `path@core@src/agents.rs`
  fails until the primer's section `checker-model` names every register, so it names `rejected`.

### This repository's record today

Measured with `grep -c '^\*\*'` on the root's, agent-skills' and core's documents, and
`grep -c '^## '` on gates':

| Component | old entries | shape |
| --- | --- | --- |
| knowledge-architect | 5 | bold paragraph |
| agent-skills | 51 | bold paragraph |
| core | 72 | bold paragraph |
| gates | 4 | level-two heading |
| xtask | 0 | "No entry yet." |

Every old entry has a lost-to clause. Two name two design heads, one names only a goal (the
delegated-state entry), none is marked `void`. `cargo klarch show` on a head lists an old entry
only as a file and a line. Design heads, tripwires, issues and the two other specs cite the
documents by `path` spans, for example 16 heads of `register@core@design` (an awk pass mapping
each line of `grep -n rejected-alternatives crates/core/docs/design.md` to the level-three heading
above it), and other texts name an old entry in words; the inventory of every such site is taken
again at step 2 by the searches that step names.

### The installed text today

The rule of `primer@where-knowledge-goes` says "a rejected alternative names the decision it lost
to", and its knowledge table routes "the engineering alternative that lost, and why" to the
rejected-alternatives document. `skill@knowledge-architect-decision-recording@losing-alternatives`
gives the entry's shape, "**<The alternative, named in a few words>** — lost to
`design@<component>@<slug>`. `live`.", and routes "a shape that lost, when the question produced no
decision" to the deliberation only, since "An entry names the decision it lost to, and there is
none." The design skill's grounding, `skill@knowledge-architect-design@discussion-loop` step 1,
reads the goals, design homes, rejected alternatives and README files itself, and sends only the
issues and tripwires to subagents. Its thread-naming rule checks a new slug against
`design@<anchor>@<slug>` only. The texts that name the rejected alternatives, 85 lines in 17
files under `path@agent-skills@content/` by
`git grep -il -E 'rejected.alternative|lost to|losing.alternative' -- crates/agent-skills/content`
for the files and the same with `-i -E -c` summed for the lines, are listed at step 3.

### Outside the work

- **A listing command for `rejected` entries**, as `cargo klarch tripwires` lists tripwires: no
  thread asked for one, and `show` and the design-entry searcher read the entries.
- **Sending the design-entry searcher anywhere but the grounding of a design discussion**, such as
  the design audit of a slice or of a spec, or a retrospective intake: the owner's proposal of round
  3 named design sessions. See Later consequences.
- **A tool that checks a searcher's report covers every entry, and a report skeleton**: the owner,
  round 5, "It is out of scope for now". An issue entry records it at the harvest.
- **A grade of its own for doubt in the standing-entry searcher's return**: per D4. Its text asks
  for an entry it cannot rule out, "with the doubt as its reason"; its return has two lists only.

## What is already decided

### Decisions the design rests on and does not argue again

- `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `goal@core@records-reach-their-reader`,
  `goal@core@declared-instructions-are-checked`.
- `design@agent-skills@losing-alternatives-filter`: which alternatives earn an entry stays as it
  is. Its wording of a winner and a loser is rewritten; see "Decisions the work rewrites".
- `design@agent-skills@standing-entry-search-groups` and `design@agent-skills@design-record-axis-dispatch`
  bound a standing-entry searcher and a design-record auditor at 60 entries. The design-entry
  searcher takes no numeric bound, on the owner's ruling of D5; those two heads are unchanged.
- `design@core@heading-register-two-shapes`: every heading register accepts the file and the
  directory shape, which gives `rejected` its `planned@<anchor>@docs/rejected/` directory.
- `design@core@an-entry-is-a-heading-at-the-register-level`, `design@core@statement-precedes-the-slug`,
  `design@core@a-slug-elsewhere-defines-nothing`: the shape every heading register's entries take.
- `design@core@kind-names-refused-as-anchors`: `rejected` joins the refused names as a member.
- `design@core@path-to-a-home-is-reported`: a `path` span to `planned@<anchor>@docs/rejected.md` is a finding whose
  repair is `register@<anchor>@rejected`.
- `design@agent-skills@capabilities-not-structure`: the design-entry searcher is a capability, and
  the design skill leaves when to dispatch it again to the session's judgement.
- `design@agent-skills@standing-entry-search-agent`: the shape the design-entry searcher copies:
  an agent of its own, the dispatch rule in its description, a return listing every entry read, the
  session reading whole every entry returned.
- `design@agent-skills@plan-read-against-the-record`: the plan-document review stays the backstop.
- `design@agent-skills@questions-go-to-their-authority`: per D1, it cites the delegated-state
  entry.

### Decisions the work rewrites

Each with the texts `cargo klarch show` lists as referencing it, and what judges or updates each.

| decision | what changes | referencing texts, and the step that judges or updates each |
| --- | --- | --- |
| `design@core@components-carry-the-same-documents` | the document list loses the rejected-alternatives document; the register list gains `rejected`; its sentence that `path@core@src/check/registers.rs` asserts both directions is brought to the code, where `path@core@src/check/tree.rs` raises the missing-document finding | heads `design@agent-skills@shipped-text-cites-no-entry`, `design@core@nothing-of-a-project-is-compiled-in` and `design@knowledge-architect@no-component-directory-named-after-the-project`, read at step 2; `path@agent-skills@content/agents/standing-state-reviewer.md`, a `%%` comment, step 1; `issue@core@a-checker-only-project-carries-the-workflow-skeleton` and `issue@core@a-missing-root-document-finding-names-a-repair-that-does-not-apply`, step 2; the doc comment of `is_live` in `path@core@src/walk.rs`, read at step 1 |
| `design@core@registers-are-declared` | "Ten registers" becomes eleven, `rejected` among those every component carries | heads `design@core@item-registers-built-in`, `design@core@register-home-reference` and `design@core@the-regime-has-no-opt-out`, read at step 2; `path@core@README.md`, step 4; the module comment of `path@core@src/manifest.rs` and the comment in its test `a_register_named_register_is_refused_as_a_reserved_kind`, step 1 |
| `design@core@an-entry-is-a-heading-at-the-register-level` | names `rejected` at level three | `path@agent-skills@content/agents/routing-reviewer.md`, a `%%` comment, step 1; heads `design@agent-skills@goal-entry-shape`, `design@core@headings-open-with-hash-marks`, `design@core@one-entity-table` and `design@core@section-homes-carry-slugs`, read at step 2; `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`, read at step 2; one old entry of core, which moves at step 2; `path@core@README.md`, step 4; `spec@plans@defined-ids-outside-references`, three sites, read at step 2 |
| `design@core@anchor-collisions-refused` | its clause on a register directory that spells a compiled document's name leaves with the dead branch | the doc comment of `resolve_anchors`, `path@core@src/manifest.rs`, read at step 1 |
| `design@core@reserved-anchors` | its example of `*` names another document | heads `design@agent-skills@plain-text-is-no-repair` and `design@core@register-home-reference`, read at step 2; three old entries of core, which move at step 2; the comments in `path` and in the test `a_path_only_a_location_carries_resolves_in_no_component` of `path@core@src/check/references.rs`, and the doc comment of `Anchors::required_kind` in `path@core@src/entity.rs`, step 1 |
| `design@agent-skills@a-reference-claims-a-revisit` | the row "a rejected alternative / the decision it lost to" becomes "a design head / the `rejected` entries its argument covers" | the primer's sentence of `primer@where-knowledge-goes` that restates it, step 3; `path@agent-skills@content/skills/goal-setting/SKILL.md`, a `%%` comment, step 3; `skill@klarch-development`, step 3; heads `design@agent-skills@plain-text-is-no-repair` and `design@agent-skills@primer-limit`, read at step 5; `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`, read at step 5 |
| `design@agent-skills@losing-alternatives-filter` | its words of a losing alternative and of a winner become a design not kept and the heads that cite it; which designs earn an entry does not change | head `design@agent-skills@losing-shape-test`, rewritten at step 5; `issue@agent-skills@the-void-marker-s-reason-reads-two-ways`, read at step 5; one old entry of agent-skills, which moves at step 2 |
| `design@agent-skills@losing-shape-test` | a design rejected with nothing built in its place is cited by a head of absence, not left to the deliberation | no text references it; `skill@knowledge-architect-decision-recording@losing-alternatives` restates it in words, step 3 |
| `design@agent-skills@standing-entries-searched-before-the-work` | the design skill's step 1 dispatches the design-entry searcher too | head `design@agent-skills@standing-state-review-before-every-merge`, `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`, and two tripwires, `tripwire@agent-skills@search-missed-before-the-work` and `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`, read at step 5; the design skill restates it in words, step 3 |
| `design@agent-skills@thread-slug-is-entry-id` | the collision check covers `rejected` entries | `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`, read at step 5; the design skill restates it in words, step 3 |
| `design@agent-skills@design-record-axis-dispatch` | its reason "a rejected alternative is judged against the head it lost to" reads "the head that cites it" | head `design@agent-skills@audit-method` and `tripwire@agent-skills@rule-misread-across-groups`, read at step 5; one old entry of agent-skills, which moves at step 2; the project-audit skill and the design-record auditor restate it in words, step 3 |

### Standing entries the discussion's search returned

The standing-entry search of the reply to round 1 returned 41 entries. Each, with its outcome:

| entry | outcome |
| --- | --- |
| `a-proposal-is-read-against-the-heads-only-after-the-plan-is-written`, an issue of agent-skills | the failure this work addresses, for heads; closed on this spec's branch, in the commit of its review repairs, since this spec schedules its work, per `skill@knowledge-architect-planning@spec-or-milestone`: its question is answered by `thread@rejected-register@design-entry-search`, which reads the record before the first ruling, and by the plan-document review that stays the backstop |
| `issue@agent-skills@the-void-marker-s-reason-reads-two-ways` | step 3 keeps the sentence it asks about with its meaning unchanged; the issue stays open |
| `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions` | its sentence of the primer is next to the one step 3 rewrites; read again at the harvest, not closed |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | the installed text writes `rejected@<anchor>@<id>` and `register@<anchor>@rejected` only as placeholders and the generic form, step 3 |
| `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it` | a `rejected` entry's slug from a thread inherits its case 4; no change |
| `issue@agent-skills@a-skill-for-creating-a-component` | a Component carries one more home; no change |
| `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` | the Migration entry names the design-record axis; no change |
| `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` | cites `design@agent-skills@standing-entries-searched-before-the-work`, which the work rewrites; read at step 5 |
| `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it` | bears on nothing in the work |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | an adopting project's pin move goes through it; no change |
| `issue@agent-skills@the-primer-s-design-head-section-sends-a-project-s-purpose-to-a-head` | bears on nothing in the work |
| `issue@agent-skills@the-audit-assumes-every-project-merges-by-rebase` | bears on nothing in the work |
| `issue@agent-skills@the-material-finding-duty-has-no-head` | bears on nothing in the work |
| `issue@core@a-missing-root-document-finding-names-a-repair-that-does-not-apply` | its instance of the rejected-alternatives document disappears; its text is updated at step 2, the defect stays for `README.md` and `CLAUDE.md` |
| `issue@core@four-heads-bundle-decisions-with-many-citing-sites` | its wording of "a rejected alternative that names the head" is updated at step 2 |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | bears on nothing in the work |
| `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` | its wording is updated at step 2 |
| `issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` | `rejected` is one more register the defect reaches; no change |
| `issue@core@the-rust-parser-choice-has-no-head` | its closing becomes "the new head cites the two entries"; updated at step 2 |
| `issue@core@a-checker-only-project-carries-the-workflow-skeleton` | its list of what a Component owes is updated at step 2 |
| `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` | `rejected` joins the refused names; no change |
| `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry` | reaches the new homes; no file of them holds such a block, so no change |
| `issue@core@a-message-lint-is-deduplicated-in-one-tree-only` | its reproduction names the root's document; updated at step 2 |
| `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | an old entry cites it; the citation moves with the entry |
| `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes` | its row of the knowledge table changes words only; no change |
| `tripwire@agent-skills@record-check-misses-a-head` | its wording "a rejected alternative" is kept; not fired |
| `tripwire@agent-skills@head-created-without-deliberation` | the new heads are argued in this discussion; not fired |
| `tripwire@agent-skills@member-beyond-the-argument` | not fired |
| `tripwire@agent-skills@design-conformance-findings-needless` | the reviewer's scope does not change; not fired |
| `tripwire@agent-skills@structure-rejection-overruled` | not fired |
| `tripwire@agent-skills@real-use-refusal-overruled` | the addition rests on the owner's account of real sessions, round 1 and round 3, and the measurement of `argument@rejected-register@a42`; not fired |
| `tripwire@agent-skills@axis-not-run-at-a-pin-move` | the Migration entry cites the design-record axis, so it watches the pin moves that cross this version |
| `tripwire@agent-skills@rarer-rule-in-the-model` | the primer's model section names `rejected` because a test requires every register named; not fired |
| `tripwire@agent-skills@agent-text-written-without-the-standard` | step 3 is written under `skill@knowledge-architect-agent-configuration`; not fired |
| `tripwire@core@issue-kind-list-grows` | its `path` span to core's document becomes a `rejected` citation, step 2 |
| `tripwire@core@item-of-another-plan-named` | the same |
| `tripwire@core@plans-directory-split-asked` | the same |
| `tripwire@core@phases-gate-the-report-two` | the same |
| `tripwire@core@extension-defines-a-kind` | `rejected` is a core kind; not fired |
| `tripwire@core@candidate-rule-silence` | a span `rejected@<anchor>@<id>` becomes a candidate; AC2 tests that it resolves or is reported; not fired |
| `tripwire@core@fix-makes-a-choice` | no `--fix` converts an old entry, by `thread@rejected-register@rejected-migration`; not fired |

Three entries the search did not return, found by the standing-state review of this spec, bear on
the work: each names an old entry in words, which step 2's search by `git grep -n 'rejected-alternatives'` misses.

| entry | outcome |
| --- | --- |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing` | quotes two old entries of core with their lost-to clauses; its wording updated at step 2 |
| `tripwire@agent-skills@ruling-lost-in-assembly` | names one old entry of agent-skills by its words; gains its `rejected` reference at step 2 |
| `tripwire@agent-skills@attribution-loses-the-scope` | the same |

## Criteria

### Every rejected design that earns an entry is recorded with the reason it was not kept `##records-the-reason`

Binding, from `goal@knowledge-architect@design-is-recorded-with-its-arguments`. Met by
`thread@rejected-register@rejected-register` and `thread@rejected-register@rejected-entry-shape`.

### A rejected design reaches the session that reads the decision it bears on, in the text that session reads `##reaches-the-reader`

Binding, from `goal@core@records-reach-their-reader`: the entry is delivered in the text the
session already reads, not only through a command it must think to run. Met by
`thread@rejected-register@rejected-citation-direction` and
`thread@rejected-register@design-entry-search`.

### No entry lists by hand what references it `##no-inbound-list`

Binding as a presumption, from `design@agent-skills@a-reference-claims-a-revisit`. Met by
`thread@rejected-register@rejected-citation-direction`, B.

### What the new rules require is checked where it can be `##rules-are-checked`

Binding, from `goal@core@declared-instructions-are-checked`. Met by
`thread@rejected-register@rejected-register` and `thread@rejected-register@orphan-rejected-entry`.

### Which rejected designs earn an entry does not change `##filter-unchanged`

Binding as a presumption, from `design@agent-skills@losing-alternatives-filter`. Met by
`thread@rejected-register@rejected-scope`.

### The cost to an adopting project `##adopter-cost`

Weighed, from `design@knowledge-architect@capability-cost-before-migration`. Met by
`thread@rejected-register@rejected-migration`: a migration, and no capability lost.

## Threads

### Rejected designs are entries of a built-in heading register, each a level-three heading with a slug `##rejected-register`

Proposed by the owner, round 1; minted by the agent in the reply to round 1. Approved. Arguments:
`argument@rejected-register@a1`, `argument@rejected-register@a2`, `argument@rejected-register@a6`,
`argument@rejected-register@a7`, `argument@rejected-register@a8`, `argument@rejected-register@a11`,
`argument@rejected-register@a25`. Shape: Decided design, "The register". Harvest: a new head of
`register@core@design`. The owner's words, round 2: "rejected-register: approved, we align here."

### A design head cites the rejected entries its argument covers, and an entry names no winner `##rejected-citation-direction`

The owner proposed in round 1 that heads cite their rejected alternatives; the agent proposed
shape B in the reply to round 1, against shape A, the agent's reading of the owner's request: the
heads citing, added to the lost-to link the entries already held. Approved. Arguments:
`argument@rejected-register@a3`, `argument@rejected-register@a9`, `argument@rejected-register@a12`,
`argument@rejected-register@a13`, `argument@rejected-register@a15`, `argument@rejected-register@a23`.
Shape: Decided design, "Citations". Harvest: a new head of `register@agent-skills@design`, and
`design@agent-skills@a-reference-claims-a-revisit` rewritten. The owner's words, round 2:
"rejected-citation-direction: B looks right. Also, it solves part of the problem I had with the
current status: the predicate is that there is always a losing and a winning side, and I think that
is a wrong way to put it." In the reply to round 1, B included the orphan check; the check moved to
`thread@rejected-register@orphan-rejected-entry` in the reply to round 2, and is ruled there.

### The register's kind is `rejected` `##rejected-register-name`

Proposed by the owner, round 1. Approved. Arguments: `argument@rejected-register@a4`,
`argument@rejected-register@a5`, `argument@rejected-register@a16`. Shape: Decided design, "The
register's name". Harvest: the head of `thread@rejected-register@rejected-register`, as its parameter. The
owner's words, round 2: "rejected-register-name: approved".

### Which rejected designs enter is unchanged; a design rejected with nothing built in its place is cited by a head of absence `##rejected-scope`

Opened by the agent in the reply to round 1, on the owner's remark of round 1 that "A design could
be rejected with being an alternative to something that is implemented" (the owner's words; the
agent read "with" as "without"). Approved. Arguments: `argument@rejected-register@a5`,
`argument@rejected-register@a10`, `argument@rejected-register@a17`, `argument@rejected-register@a24`,
`argument@rejected-register@a25`, `argument@rejected-register@a34`. Shape: Decided design,
"Which rejected designs enter". Harvest: `design@agent-skills@losing-shape-test` rewritten. The owner's words, round
2: "rejected-scope: I agree that the design "we do not do X" already covers it. It feels a bit
weird to me, but it is probably the best way to put it. It is weird because the winner is the
negation of the loser. TBH, this is making me question whether the rejected register is even
needed. The goal was to give a clear separation between what has been decided, and the
descriptions of the solutions we considered but did not keep. The separation is needed because an
agent reading a decision file partially (in particular with grep) might read the description of a
rejected design and take it for what is actually implemented. Separating the provenance at
filesystem level was the best way I found to avoid that. This argument still hold, so I would not
propose removing it with just that weird feeling that "we do not do X" give."

### The register's home is `planned@<anchor>@docs/rejected.md` `##rejected-home-path`

Proposed by the agent in the reply to round 1. Approved. Arguments: `argument@rejected-register@a18`,
`argument@rejected-register@a26`. Shape: Decided design, "The register's home". Harvest: the head of
`thread@rejected-register@rejected-register`. The owner's words, round 2: "rejected-home-path:
agreed, docs/rejected.md reads better and avoids a confusion point." The directory shape,
`planned@<anchor>@docs/rejected/`, is no part of the ruling: `design@core@heading-register-two-shapes` gives it to
every heading register.

### An entry's heading names the design not kept, its slug is the thread's name, and its body opens with the marker `##rejected-entry-shape`

Proposed by the agent in the reply to round 1. Closed `argued` in the reply to round 2; the owner
did not address it. Arguments: `argument@rejected-register@a2`, `argument@rejected-register@a19`,
`argument@rejected-register@a31`. Shape: Decided design, "An entry". Harvest:
`skill@knowledge-architect-decision-recording@losing-alternatives`, step 3; a head only where
`primer@design-heads` admits one. The argument that closed it: its rival, a marker the checker
checks, needs a field a heading register lacks, per `design@core@registers-are-declared`, and would
protect no entry, since none is marked `void`; the body states "the reason it was not kept", per
the owner's remark of round 2 on B.

### Old entries are converted by hand and by the checker's findings, with a Migration entry `##rejected-migration`

Proposed by the agent in the reply to round 1. Approved, with the owner's addition. Arguments:
`argument@rejected-register@a20`, `argument@rejected-register@a27`. Shape: Decided design, "The
conversion". Harvest: the changelog's Migration entry, step 4. The owner's words, round 2:
"rejected-migration: agreed on process. Maybe hint at dispatching subagents to propose slugs, then
to apply." The hint is hedged, and the Migration entry keeps the hedge: it suggests the two
dispatches, it does not require them.

### The reading of a design discussion's proposals against the record is bundled into this work `##proposals-read-against-record`

Proposed by the agent in the reply to round 1, as a fork on scope with the default to bundle.
Approved. Arguments: `argument@rejected-register@a21`. Shape: Decided design, "The reading of
proposals against the record", carried by `thread@rejected-register@design-entry-search`, which
decides how. Harvest: through that thread.
The owner's words, round 2: "proposals-read-against-record: agreed. I'd bundle it too."

### A `rejected` entry is reachable only when a design entry cites it `##orphan-rejected-entry`

Opened by the agent in the reply to round 2, on the finding of `argument@rejected-register@a28`,
with the default, shape b. Approved, shape a1, on the owner's word. Arguments: `argument@rejected-register@a14`,
`argument@rejected-register@a23`, `argument@rejected-register@a28`, `argument@rejected-register@a29`,
`argument@rejected-register@a33`, `argument@rejected-register@a34`. Shape: Decided design, "The
orphan check". Harvest: a new head of `register@core@design`. The owner's words, round 3:
"orphan-rejected-entry: the entry you found should just be removed IMO. It is an old artifact from
the past of the design skill, it is no longer needed IMO. I now think that a1 is the right
criteria, and that manufacturing a "we do not do X" design head is the right solution when the
design is not implemented." The removal of the delegated-state entry is D1: the agent found the
premise defeated.

### The design-conformance reviewer is dispatched on a discussion's checkpoint `##checkpoint-reader`

Proposed by the agent in the reply to round 2. Withdrawn by the agent in the reply to round 3,
after the owner asked "Which one do you think is better, between survey at start of design, or
review of checkpoint after most of the design ? I lean toward the first one...". Arguments:
`argument@rejected-register@a30`, `argument@rejected-register@a35`, `argument@rejected-register@a36`.
The defeating reason: by the checkpoint the owner has ruled most threads one by one, 7 of 8 in round
2 of this discussion, so a reader there reads rulings already given. Shape: Losing alternatives.
Harvest: the `rejected` home of agent-skills, as the recording tests admit, cited by the head of
`thread@rejected-register@design-entry-search`, whose question it answered.

### A design-entry searcher reads one Component's design home and `rejected` home whole at the grounding of a design discussion `##design-entry-search`

Proposed by the owner, round 3; minted by the agent in the reply to round 3. Approved, shape p1;
reopened by the material finding of D5, which the plan-document review raised.
Arguments: `argument@rejected-register@a35`, `argument@rejected-register@a37`,
`argument@rejected-register@a38`, `argument@rejected-register@a39`, `argument@rejected-register@a40`.
Shape: Decided design, "The design-entry search". Harvest: a new head of
`register@agent-skills@design`, and `design@agent-skills@standing-entries-searched-before-the-work`
rewritten. It answers the question of the issue `a-proposal-is-read-against-the-heads-only-after-the-plan-is-written`,
which this spec's branch closes, in the commit of its review repairs.
The owner's words, round 4: "p1 approved. I believe grouping by quantity only would most likely be
a bad decision, because then the subagent might need to read completely unrelated entries, and get
a partial read of a section in a component's design. Also, the rejected and kept entries about
similar subjects have to be read together."

### The searcher grades each entry `bears`, with its relation, or `may-bear`, and lists every unrelated entry `##search-relevance-grade`

Proposed by the owner, round 4; minted by the agent in the reply to round 4. Approved, with the
owner's change that every unrelated entry is listed, not counted. Arguments:
`argument@rejected-register@a41`, `argument@rejected-register@a42`, `argument@rejected-register@a43`,
`argument@rejected-register@a44`, `argument@rejected-register@a45`, `argument@rejected-register@a47`.
Shape: Decided design, "The grades". Harvest: a new head of
`register@agent-skills@design`, or a clause of the head of
`thread@rejected-register@design-entry-search`, as `primer@design-heads` decides. The owner's
words, round 5: "For search-relevance-grade: I agree on the grades. For the unrelated entries, the
only important point I want to add is that all the entries should be listed."

## Arguments

### Rejected alternatives are missed at design and come back later `##a1`

Round 1, the owner. Bears on `thread@rejected-register@rejected-register`. "rejected alternatives
are often missed during design sessions, and come back later, often during plan review (and
sometimes later after implementation itself), which then requires rethinking the design, or
re-arguing." / "Having them be a register with citeable entries would make them much more
reachable, and solve most of this problem."

### The shape of the design register `##a2`

Round 1, the owner. Bears on `thread@rejected-register@rejected-register`,
`thread@rejected-register@rejected-entry-shape`. "a proper register with named slugs, just like the
design register (with the same shape)."

### Heads cite their rejected alternatives `##a3`

Round 1, the owner. Bears on `thread@rejected-register@rejected-citation-direction`. "notably the
decision recording skill, which needs to be instructed to cite related rejected alternatives in
design heads."

### A shorter citation `##a4`

Round 1, the owner. Bears on `thread@rejected-register@rejected-register-name`. "I'd consider
renaming it to just "rejected", to keep citations a bit shorter."

### Not every rejected design is an alternative `##a5`

Round 1, the owner. Bears on `thread@rejected-register@rejected-register-name`,
`thread@rejected-register@rejected-scope`. "having hold only "alternatives" is a bit weird to me. A
design could be rejected with being an alternative to something that is implemented."

### No old entry can be cited `##a6`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-register`. "No current
entry can be cited." The entries are bold-led paragraphs with no slug.

### The link back exists, and delivers a line number `##a7`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-register`,
`thread@rejected-register@rejected-citation-direction`. "`cargo klarch show` on a head already lists
its rejected alternatives, but only as a file and a line, line 7 of agent-skills' document, never
the entry's text." / "A tripwire's `guarding` column is computed from the `design`, at sign,
references in its section", here with the span's at sign spelled out so the span is no reference. Precisely, the
column holds every reference of a kind `guarded` in `path@core@src/records.rs` accepts.

### A checker change `##a8`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-register`. "This is a
checker change, not only a text change." The document is compiled in as a required document.

### Both directions by hand is an inbound list `##a9`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-citation-direction`.
"A head that also cites its rejected entries is a hand-kept reverse list, which is what that rule
forbids, because such a list goes stale."

### The name may change the scope `##a10`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-scope`. "The name
`rejected` may change the register's scope, not just its name." Shapes of a question that produced
no decision are kept out by `design@agent-skills@losing-shape-test`.

### Printing the paragraphs from `show` delivers to the wrong reader `##a11`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-register`. "It fails
C2: it delivers only to a session that runs `show` on the right head, and the recorded case above
is a session that did not read the right head. It also leaves nothing for a plan item or a review
finding to cite."

### A fails the inbound-list rule `##a12`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-citation-direction`.
"A fails C3 as written. The next entry written that cites a head without the head citing it back is
the stale case that head names."

### B puts the citation where the reader is `##a13`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-citation-direction`.
"B moves the hand-written direction to the side your failure needs: the head, which is what a design
session reads." / "If a rejected entry goes `void`, is reopened or is renamed, the head's argument
against that rival must be re-read."

### The orphan check `##a14`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-citation-direction`,
`thread@rejected-register@orphan-rejected-entry`. "a `rejected` entry that no design head cites is
a finding, an orphan check. It replaces today's unchecked "lost to" convention with a guarantee
that every loser is reachable from a head."

### C needs a field a heading register lacks `##a15`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-citation-direction`.
"C keeps both directions and adds a symmetry check." / "A heading register has no field for it
[...] So C needs a new syntax that B does not need."

### A shorter span, and no information lost `##a16`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-register-name`.
"`rejected@core@<slug>` is 8 characters shorter than `rejected-alternative@core@<slug>`. That
counts for a reference span, which never crosses a line break." The figure is wrong: the difference
is 12 characters, `-alternative`, as the agent told the owner after the plan-document review.

### A design rejected outright is a decision of absence `##a17`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-scope`. "A design
rejected outright, with nothing built in its place, is a decision about an absence ("we do not do
X"). That earns a design head by entry test 2 of `primer@design-heads`." / "A design rejected while
the question stays open is an issue entry".

### Every pointer changes anyway `##a18`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-home-path`. "every
pointer to the file changes anyway. A `path` reference to a register's home is a finding whose
repair is `register@<anchor>@rejected` [...] So keeping the old name saves no edit in any project."

### The entry's title, slug and marker `##a19`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-entry-shape`. "The
title names the alternative, not a decision." / "when the loser was a thread of a design
discussion, its slug is the thread's name." / "a checked marker buys nothing measurable."

### A slug is a choice `##a20`

The reply to round 1, the agent. Bears on `thread@rejected-register@rejected-migration`.
"Converting a paragraph to a heading requires choosing a slug. That is a choice, so `check --fix`
cannot do it".

### The register alone does not close the failure `##a21`

The reply to round 1, the agent. Bears on `thread@rejected-register@proposals-read-against-record`.
"Citations from heads only reach a session that reads the right head. The recorded case is a
session that did not read the right head." / "The register is what makes the check's output
citeable, and the two serve one failure."

### The path `##a22`

The reply to round 1, the agent. Bears on no thread. "the full path, with a plan document at
convergence."

### Not always a loser and a winner `##a23`

Round 2, the owner. Bears on `thread@rejected-register@rejected-citation-direction`,
`thread@rejected-register@orphan-rejected-entry`. "the predicate is that there is always a losing
and a winning side, and I think that is a wrong way to put it."

### The winner as the negation of the loser `##a24`

Round 2, the owner. Bears on `thread@rejected-register@rejected-scope`. "It is weird because the
winner is the negation of the loser. TBH, this is making me question whether the rejected register
is even needed."

### Separation at the filesystem level `##a25`

Round 2, the owner. Bears on `thread@rejected-register@rejected-scope`,
`thread@rejected-register@rejected-register`. "The separation is needed because an agent reading a
decision file partially (in particular with grep) might read the description of a rejected design
and take it for what is actually implemented. Separating the provenance at filesystem level was the
best way I found to avoid that. This argument still hold".

### The new name reads better `##a26`

Round 2, the owner. Bears on `thread@rejected-register@rejected-home-path`. "docs/rejected.md reads
better and avoids a confusion point."

### Subagents for the slugs `##a27`

Round 2, the owner. Bears on `thread@rejected-register@rejected-migration`. "Maybe hint at
dispatching subagents to propose slugs, then to apply."

### One entry loses only to a goal `##a28`

The reply to round 2, the agent. Bears on `thread@rejected-register@orphan-rejected-entry`. "The
orphan check, part of B as I presented it, breaks on one existing entry." The delegated-state entry
"loses only to `goal@knowledge-architect@the-owner-decides`."

### The four shapes of reachability `##a29`

The reply to round 2, the agent. Bears on `thread@rejected-register@orphan-rejected-entry`. a1:
"needs a head of absence the owner ruled unneeded"; a2: "a goal does not change when an
alternative goes `void`, so the citation is decoration"; b: "The entry's citation of a goal passes
the revisit test"; c: "a loser that no head cites, the very case you want to end, goes unreported".

### The same reader repeats the same miss `##a30`

The reply to round 2, the agent. Bears on `thread@rejected-register@checkpoint-reader`. r1: "It
fails on the recorded case: that session "had read the general heads of the subject, not that
one". The same reader repeats the same miss." / r2: "This reviewer already reads the goals, the
heads and the rejected alternatives".

### A checked marker would protect nothing `##a31`

The reply to round 2, the agent. Bears on `thread@rejected-register@rejected-entry-shape`. "Its
rival, a marker the checker checks, needs a field that a heading register lacks [...] It would
protect 0 entries".

### The premortem runs `##a32`

The reply to round 2, the agent. Bears on no thread. "the change alters a document grammar that
adopting projects consume".

### The delegated-state entry is an old artifact `##a33`

Round 3, the owner. Bears on `thread@rejected-register@orphan-rejected-entry`. "the entry you found
should just be removed IMO. It is an old artifact from the past of the design skill, it is no
longer needed IMO."

### A head of absence where nothing is built `##a34`

Round 3, the owner. Bears on `thread@rejected-register@orphan-rejected-entry`,
`thread@rejected-register@rejected-scope`. "I now think that a1 is the right criteria, and that
manufacturing a "we do not do X" design head is the right solution when the design is not
implemented."

### Searchers at the start of design `##a35`

Round 3, the owner. Bears on `thread@rejected-register@checkpoint-reader`,
`thread@rejected-register@design-entry-search`. "having design sessions dispatch subagent searchers
to ground in design heads and rejected entries, the same way as done for open issues (and that is
arguably more important for design than it is for standing state). I've seen partial miss of
existing design entries happen quite a few time already." / "I lean toward the first one..."

### Rulings come before the checkpoint `##a36`

The reply to round 3, the agent. Bears on `thread@rejected-register@checkpoint-reader`. "In round 2
you approved 7 of the 8 threads one by one, before any checkpoint table was shown." / "A survey at
grounding puts the heads in view before the first proposal is shaped, so before any ruling."

### The survey's blind spot, and its cover `##a37`

The reply to round 3, the agent. Bears on `thread@rejected-register@design-entry-search`. "A survey
seeded with the opening question can miss a head that only a later thread touches." / "dispatch it
at grounding, and again for a thread whose subject the first survey was not given." / "The
plan-document design-conformance review stays the backstop".

### No listing of design heads exists `##a38`

The reply to round 3, the agent. Bears on `thread@rejected-register@design-entry-search`. "No
command lists design heads (checked: `cargo klarch --help`)." / p1 "needs no checker code" / "p2
buys even group sizes, which only matters once one Component's home outgrows one agent's reading."
Precisely, `cargo klarch model` prints every heading and slug definition with its file and line, but
no command lists design entries as entries. The premise that no Component had outgrown one agent's
reading is false by the bound of `design@agent-skills@standing-entry-search-groups`; see D5.

### A new agent `##a39`

The reply to round 3, the agent. Bears on `thread@rejected-register@design-entry-search`. "The word
"standing" means open state, which a design head is not."

### Grouping by quantity cuts the subject `##a40`

Round 4, the owner. Bears on `thread@rejected-register@design-entry-search`. "grouping by quantity
only would most likely be a bad decision, because then the subagent might need to read completely
unrelated entries, and get a partial read of a section in a component's design. Also, the rejected
and kept entries about similar subjects have to be read together."

### A grade per entry `##a41`

Round 4, the owner. Bears on `thread@rejected-register@search-relevance-grade`. "the searcher reads
the components design and rejected registers in full, and reports a list of related entries. Even
better, a ruling for each entry about how closely related it is (not sure if a yes/no is enough or
if more grades are useful)."

### The searchers grade anyway `##a42`

The reply to round 4, the agent. Bears on `thread@rejected-register@search-relevance-grade`. "They
returned 41 entries, and 12 of them carried a third grade that nobody asked for: "Doubt: …" (9) or
"included because I cannot rule it out" (3). So a yes/no is not enough." The searcher's text asks
for such an entry "with the doubt as its reason"; what it lacks is a list of its own for it.

### A grade is worth an act `##a43`

The reply to round 4, the agent. Bears on `thread@rejected-register@search-relevance-grade`. "A
grade is worth having only where it changes what the session does." / "constrains", "may change",
"may bring back": "Each one routes a different act".

### A numeric scale changes no act `##a44`

The reply to round 4, the agent. Bears on `thread@rejected-register@search-relevance-grade`. "No act
of the session differs between a 3 and a 4, and a scale without defined steps is graded differently
by each agent".

### Every entry listed `##a45`

Round 5, the owner. Bears on `thread@rejected-register@search-relevance-grade`. "For the unrelated
entries, the only important point I want to add is that all the entries should be listed."

### A coverage tool, later `##a46`

Round 5, the owner. Bears on no thread of this work. "It is out of scope for now, but later I want
to give a mechanical tool that would be able to verify that the report of those searcher did indeed
cover the whole design and rejected registers. And maybe help them by creating a report skeleton
that they only need to fill."

### The full list is what the tool would check `##a47`

The reply to round 5, the agent. Bears on `thread@rejected-register@search-relevance-grade`. "the
report lists every entry it read, the unrelated ones included, as the standing-entry searcher's
report already does under "Read and judged unrelated". That full list is also what your later
coverage tool would check."

## New names, in one place

```text
rejected                                   the register's name, in Registers::built_in, path@core@src/manifest.rs
planned@<anchor>@docs/rejected.md          the register's file home, from Anchor::home_of
planned@<anchor>@docs/rejected/            the register's directory home, from Anchor::home_of
an orphan finding                          the new check, in check_under of path@core@src/check/references.rs
knowledge-architect-design-entry-searcher  the agent, from planned@agent-skills@content/agents/design-entry-searcher.md
bears, may-bear                            the searcher's grades
constrains, may change, may bring back     the relations a `bears` grade states
```

`COMPONENT_DOCUMENTS`, `Registers::built_in`, `resolve_registers`, `Anchors::required_kind`,
`records::guarded`, `records::inbound`, `check_under`, `judge_part`, `CHECKS` and `PLANTED` exist.

## Decided design

### The register

`Registers::built_in` declares `rejected`: a heading register, scope component, entries at level
three, directory `rejected`, so its homes are `planned@<anchor>@docs/rejected.md` or
`planned@<anchor>@docs/rejected/` with a `README.md`. Every Component carries it, as it carries
`design`. `COMPONENT_DOCUMENTS` loses the rejected-alternatives document, and with it
`Anchors::required_kind` stops accepting a `path` span to every Component's copy of it. The branch of `resolve_registers`
that refuses a register directory spelling a compiled document's name, and its test, leave with it,
since no compiled document lies under `docs/`. `records::guarded` excludes `rejected`: a tripwire
guards a decision, and a `rejected` entry is none.

The argument: an entry needs a slug to be cited by a head, a plan item, a review finding or a
commit message, and `show` then prints it whole, per `argument@rejected-register@a6` and
`argument@rejected-register@a7`. The separate home keeps a grep hit on a rejected design from being
read as the design in place, per `argument@rejected-register@a25`. The nearest rival, the old
paragraphs with `show` printing each one that cites a head, delivers only to a session that runs
`show` on the right head, per `argument@rejected-register@a11`.

### The register's name

The kind is `rejected`, cited `rejected@<anchor>@<id>`. The argument: the owner asked for the
shorter citation and judged "alternatives" wrong for a design that was not an alternative, per
`argument@rejected-register@a4` and `argument@rejected-register@a5`; the span is 12 characters
shorter, and the slug, which names the design, says what "alternative" would add, per
`argument@rejected-register@a16`. The nearest rival, `rejected-alternative`, loses on both.

### The register's home

The home is `planned@<anchor>@docs/rejected.md`, or `planned@<anchor>@docs/rejected/` with a
`README.md`, the second shape given to every heading register by
`design@core@heading-register-two-shapes`. The argument: every pointer to the old file changes
anyway, since a `path` span to a register's home is a finding whose repair is the `register` form,
per `argument@rejected-register@a18`, and the owner judged the new name to read better and to avoid
a confusion point, per `argument@rejected-register@a26`. The nearest rival, keeping the old file
name as the home, saves no edit and leaves a name that differs from its kind.

### An entry

```markdown
### <the design not kept, named in a few words> `##<slug>`

`live`. <the reason it was not kept, checkable without leaving the entry>
```

This block illustrates the shape; the installed text of step 3 is its home. The title names the
design not kept; it states no decision. When the design was a thread of a design discussion, the
slug is the thread's name. The marker `live` or `void` is the body's first word, in prose. The
entry names no winner: the heads that did not keep it cite it. Its reason may name a head where the
reason needs one, as `design@agent-skills@a-reference-claims-a-revisit` decides any reference.

The argument: the shape is the design register's, as the owner asked, per
`argument@rejected-register@a2`, so every rule on a heading register's entries applies unchanged.
The title states no decision, because a design not kept decides nothing, per
`argument@rejected-register@a19`. The nearest rival, a marker the checker reads, needs a field a
heading register lacks, per `design@core@registers-are-declared`, and protects no entry today, per
`argument@rejected-register@a31`; the premise that no entry is marked `void` expires with the first
`void` entry, and nothing checks the marker's place meanwhile, which a review of that entry reads.

### Citations

A design head cites each `rejected` entry its argument covers: where its argument meets that
design, or, where the argument names none, in a closing sentence of its body.

A citation that the conversion adds to an existing head moves the link the old entry held into the
head, and changes no decision: it is a re-pointing of a reference, which `primer@design-heads` says
does not touch the head, so the head is not brought to the rules in that change.

**An entry leaves** when its design is reopened, chosen and built; when the owner rules that it
fails every recording test; or when no design head cites it any longer, which the orphan check
reports, and no head of absence is written for it. The last case replaces today's "the decision it
lost to leaves the design home".

The argument: the hand-written direction sits in the text a design session reads, the head, per
`argument@rejected-register@a13`; the link back is `show`'s to compute, per
`criterion@rejected-register@no-inbound-list`. Writing both directions by hand is an inbound list,
per `argument@rejected-register@a9` and `argument@rejected-register@a12`, and checking their
symmetry needs a field a heading register lacks, per `argument@rejected-register@a15`. The owner
judged the winner-and-loser framing wrong, per `argument@rejected-register@a23`, so the entry names
no winner.

### Which rejected designs enter

Which designs earn an entry is unchanged, per `criterion@rejected-register@filter-unchanged`. A
design rejected with nothing built in its place is cited by a head of absence, which records the
decision not to build it. A design rejected while its question stays open is no entry; the question
is an issue.

The argument: a design rejected outright is a decision about an absence, which entry test 2 of
`primer@design-heads` admits as a head, per `argument@rejected-register@a17`, and the owner chose to
write such a head where nothing is built, per `argument@rejected-register@a34`. The nearest rival,
widening the register to designs no decision covers, as the name `rejected` could suggest, per
`argument@rejected-register@a10`, would leave entries no head cites, which the orphan check reports;
the owner kept the register on the separation of the provenance, per
`argument@rejected-register@a25`, not on a wider scope.

### The orphan check

A `rejected` entry that no design entry cites is a finding of the last phase, raised in
`check_under` of `path@core@src/check/references.rs`, the path that judges a tree, and not in
`judge_part`, which `commits` also runs on each commit message. Its count in `PLANTED` is that
module's, grown by one. A reference counts when it sits inside the section of an entry of a
`design` register, in any anchor; a reference from any other text, the entry's own home, a goal, an
issue or a tripwire included, does not. The finding names the entry and its repair: "cite it from
the design head whose argument covers it, or from a head of absence that records the decision not
to build it".

The argument: a `rejected` entry that no head cites is the case the owner wants to end, per
`argument@rejected-register@a1` and `argument@rejected-register@a14`. Of the rivals of
`argument@rejected-register@a29`, a2, a goal citing the entry, fails the revisit test; c, no check,
leaves that case unreported; b, the entry made reachable by its own citation of a goal, passes the
revisit test, and lost only to the owner's choice of a1: "I now think that a1 is the right
criteria", per `argument@rejected-register@a34`.

### The design-entry search

The design-entry searcher is an installed agent of its own, sibling of the standing-entry searcher.
Its description carries its dispatch rule: one agent per Component the question touches, the
project's root always among them, all in parallel, each given the work, the seeds, its group and a
scratch directory of its own. A Component is one group, unless the dispatching session judges that
it holds too many entries for one agent to read whole; it is then cut into groups of whole
level-two subjects of its design home, in order. **The cut never splits a level-two subject**,
whatever its size, and no number bounds a group. Each group takes the `rejected` entries its heads
cite, an entry cited from two groups going to both, and a Component's `rejected` entries no head
cites go to its first group. An agent whose group holds a level-two subject too large to read
whole reports it under "Met outside the task", naming the subject and its count of heads, as a sign
that the Component's design home is not divided enough; the owner's example of such a subject, a
hedged one, is one of more than 60 entries; the session routes it by
`primer@met-outside-the-task`.

Each agent reads its group's design entries and `rejected` entries whole. A seed is a decision or a
goal the work names: a seed that is an entry of its group is graded `bears`, and an entry of its
group that cites a seed is graded `bears` or `may-bear`, never unrelated. It grades every entry it
read:

| grade | the entry | what the session does |
| --- | --- | --- |
| `bears`, `constrains` | states a rule the work's subject falls under, which a proposal must respect | reads it whole; names it in the first round |
| `bears`, `may change` | a design entry the work could contradict, widen or narrow | the same, and sends that part of the work to `skill@knowledge-architect-decision-recording@reversal-check` and to the backstop of `primer@design-heads` |
| `bears`, `may bring back` | a `rejected` entry whose design the work could propose again | the same; a proposal that brings it back needs a new argument, by the reopening rule of `skill@knowledge-architect-design@threads-and-states` |
| `may-bear` | shares the work's subject, with none of the three relations established | reads it whole; states in the first round whether it bears |
| unrelated | shares no subject with the work | nothing |

It returns:

```text
Component: <name>, group <n> of <total>, <h> design entries and <r> rejected entries read.

Bears on the work:
- <reference> (constrains | may change | may bring back): <one line: what in the work meets what in the entry>

May bear:
- <reference>: <one line: the shared subject, and why no bearing is established>

Read and judged unrelated:
- <reference>

Commands run:
- <each command, with its arguments>

Met outside the task:
- <each item, or none>
```

This block illustrates the return; the agent's file is its home. Every entry of the group appears
in exactly one of the three lists. Its sections follow the standing-entry searcher's: the brief,
the stance of recall, taking its group in place of positions, reading every entry, following the
seeds as above, judging each entry by the grades above, and the return.

The design skill's step 1 keeps its reading of the goals, design homes, `rejected` homes and README
files, and dispatches the design-entry searcher beside the standing-entry searcher. The session
reads whole, with `show`, every entry graded `bears` or `may-bear`; its first round names the
`bears` entries, and states for each `may-bear` entry whether it bears. The skill tells the session
to dispatch the search again, for one Component, when a thread opens on a subject the work given to
the first search did not cover. The thread-naming rule checks a new slug against
`rejected@<anchor>@<slug>` as well as `design@<anchor>@<slug>`.

The argument: the survey reads the record before the first proposal is shaped, so before any
ruling, per `argument@rejected-register@a36`; a group of whole subjects reads a subject's kept and
rejected entries together and cuts no section, per `argument@rejected-register@a40`; a Component
too large for one agent is cut along its subjects, and never by a number, on the owner's ruling of
D5, which judged a numeric limit not sensible for design entries and keeps the cut meaningful at
any cost; a new agent, since a design head is no standing entry, per `argument@rejected-register@a39`. The
session keeps reading the design homes, since the premises of
`design@agent-skills@a-head-is-owed-by-an-entry-test` rest on every grounding reading them whole; the
search adds a second reader that grades each entry. The grades are the ones that route an act, per
`argument@rejected-register@a43`; the rival scale routes none, per `argument@rejected-register@a44`.
The survey's blind spot, a later thread, is covered by the re-dispatch and by the plan-document
review, per `argument@rejected-register@a37`.

### The grades

The grades are the table of "The design-entry search": `bears` with one of three relations,
`may-bear`, and unrelated, every entry read listed in one of the return's three lists. The
argument: the searchers of this discussion graded a doubt anyway, unasked, in 12 of 41 entries, per
`argument@rejected-register@a42`; a grade is kept only where it routes an act, per
`argument@rejected-register@a43`; the owner asked that every entry be listed, per
`argument@rejected-register@a45`, which is also what a later coverage tool would check, per
`argument@rejected-register@a47`. The nearest rival, a numeric scale, routes no act, per
`argument@rejected-register@a44`.

### The reading of proposals against the record

The record is read against the work before the first proposal, by the design-entry search at
grounding and its re-dispatch for a later thread; the plan-document review of
`design@agent-skills@plan-read-against-the-record` stays the backstop. The argument: the register
alone reaches only a session that reads the right head, per `argument@rejected-register@a21`, and a
reading at the checkpoint comes after the owner has ruled most threads, per
`argument@rejected-register@a36`. The nearest rival, the design-conformance reviewer at the
checkpoint, is `thread@rejected-register@checkpoint-reader`, withdrawn on that fact.

### The conversion

No `--fix` converts an old entry, per `argument@rejected-register@a20`. In this repository, step 2
converts each old entry by the mapping of "Mapping tables". The Migration entry of step 4 tells an
adopting project:

- move each rejected-alternatives document to `planned@<anchor>@docs/rejected.md`, turn each entry
  into a heading with a slug, drop each lost-to clause, and cite each entry from the heads that did
  not keep it; it may dispatch subagents to propose the slugs, then to apply them;
- that a Component, a location or a plan named `rejected` is now refused, as every kind's name is,
  and that a `[registers.rejected]` table, or a declared register whose directory is `rejected`, is
  refused or ignored, as for any built-in register;
- that the new home is owed by every Component of a mock project too, as
  `design@knowledge-architect@changelog-entries` asks of an entry that adds a required home;
- to run `cargo klarch check`, whose findings name a missing home, an unslugged heading and an
  orphan entry, and then the design-record axis of `skill@knowledge-architect-project-audit` as the
  check after the move.

## Mapping tables

### Old entry to `rejected` entry

The domain is every old entry: 132, or 131 if the owner rules D1 against its default. For each:

| old entry, in the document of | `rejected` entry | citing heads |
| --- | --- | --- |
| a Component | an entry of that Component's `planned@<anchor>@docs/rejected.md`, in the old order | each design head the lost-to clause names; for the delegated-state entry, `design@agent-skills@questions-go-to-their-authority`, per D1 |

- **Title**: in a bold paragraph, the bold name, followed by the text between it and "— lost to"
  where there is any, as in "A hand-rolled gitignore matcher reading the root file alone, with the
  walk pruning what it covered" (18 entries: 1 of agent-skills, 17 of core, by a split on each bold
  paragraph); in gates, the level-two heading's text.
- **Slug**: proposed by one subagent per Component, then reviewed by the session; an id unique in
  the Component's `rejected` home, in the entry-id grammar, naming the design not kept.
- **The lost-to clause**: in a bold paragraph, from "— lost to" to the marker; in gates, from "Lost
  to" to the marker, at the head of the paragraph.
- **Body**: the marker, then the old text after the marker. Where the clause holds words beyond its
  references and the words "and" and "to" that join them, such as "for the re-check" or "which
  fixes `@`", those words are rewritten into a sentence of the body that keeps them, and the entry
  is listed in AC1's report as rewritten.
- **The documents' introductions**, and the four design-home introductions that say "What lost to
  a decision here is …": rewritten to state what the home holds, with no "lost to".
- **Each `path` span to a rejected-alternatives document** elsewhere in the tree: a `rejected`
  reference where the sentence names one entry, a `register@<anchor>@rejected` reference where it
  names the home. **Each text that names an old entry in words**, as a tripwire's response does:
  a `rejected` reference beside the words.

The values are produced at step 2, and the commit that lands them lists each old entry's first
words beside its new reference, as AC1's report.

## Losing alternatives

Each with the head that will cite it, if the recording tests admit it, per
`thread@rejected-register@orphan-rejected-entry`.

- **The old paragraphs, with `show` printing each one that cites a head**: lost to
  `thread@rejected-register@rejected-register`, on `argument@rejected-register@a11`. Cited by that
  thread's head.
- **A `rejected` entry names the decision it lost to**, the rule the work reverses, the row of
  `design@agent-skills@a-reference-claims-a-revisit` and the template of
  `skill@knowledge-architect-decision-recording@losing-alternatives`: lost to
  `thread@rejected-register@rejected-citation-direction`, on `argument@rejected-register@a13` and
  `argument@rejected-register@a23`. Cited by that thread's head.
- **Both directions of the link written by hand (A)**: lost to
  `thread@rejected-register@rejected-citation-direction`, on `argument@rejected-register@a9` and
  `argument@rejected-register@a12`. Cited by that thread's head.
- **Both directions, with a check of their symmetry (C)**: lost to
  `thread@rejected-register@rejected-citation-direction`, on `argument@rejected-register@a15`. Cited
  by that thread's head.
- **The kind named `rejected-alternative`**: lost to
  `thread@rejected-register@rejected-register-name`, on `argument@rejected-register@a16`. Cited by
  the head of `thread@rejected-register@rejected-register`.
- **Keeping the file name of the rejected-alternatives document as the home**: lost to
  `thread@rejected-register@rejected-home-path`, on `argument@rejected-register@a18`. Cited by the
  head of `thread@rejected-register@rejected-register`.
- **A marker the checker checks**: lost to `thread@rejected-register@rejected-entry-shape`, on
  `argument@rejected-register@a31`. Cited by the head of `thread@rejected-register@rejected-register`,
  which decides the register's fields.
- **A `--fix` that converts old entries**: lost to `thread@rejected-register@rejected-migration`,
  on `argument@rejected-register@a20`. Cited by `design@core@fix-scope`, which decides what `--fix`
  may do.
- **A goal citing a `rejected` entry (a2); an entry made reachable by its own citation of a goal
  (b); no orphan check (c)**: lost to `thread@rejected-register@orphan-rejected-entry`; a2 and c on
  `argument@rejected-register@a29`, b on the owner's choice of a1, `argument@rejected-register@a34`.
  Cited by that thread's head.
- **The design-conformance reviewer at the checkpoint (r2), and the session's own reversal search
  at the checkpoint (r1)**: the thread `thread@rejected-register@checkpoint-reader`, withdrawn on
  `argument@rejected-register@a36`; r1 on `argument@rejected-register@a30`. Cited by the head of
  `thread@rejected-register@design-entry-search`, whose question they answered.
- **A listing command, `cargo klarch decisions`, cut in groups of 60 rows (p2)**: lost to
  `thread@rejected-register@design-entry-search`, on `argument@rejected-register@a40`. Cited by that
  thread's head.
- **Groups of whole subjects bounded at 60 heads, a larger subject cut into consecutive heads**,
  the default of D5: lost to `thread@rejected-register@design-entry-search` on the owner's ruling of
  D5, which keeps the cut meaningful at any cost and puts no number on it. Cited by that thread's
  head.
- **Widening the standing-entry searcher to design entries**: lost to
  `thread@rejected-register@design-entry-search`, on `argument@rejected-register@a39`. Cited by that
  thread's head.
- **A numeric scale of relatedness**: lost to `thread@rejected-register@search-relevance-grade`, on
  `argument@rejected-register@a44`. Cited by the head that records that thread.
- **Unrelated entries counted, not listed**: lost to
  `thread@rejected-register@search-relevance-grade`, on the owner's words of round 5,
  `argument@rejected-register@a45`. Cited by the head that records that thread.

## Readings

The work reads no external specification.

## Premortem

Assume the work shipped and failed. The causes, as presented in the replies to rounds 2 and 3:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| 1. Sessions satisfy the orphan check by citing a new entry from the nearest head, not from the head that decides the entry's question, so the entry still misses the session that reads the right head | `thread@rejected-register@rejected-citation-direction`, `thread@rejected-register@orphan-rejected-entry` | the tripwire **T1**, on the owner's word, round 6 |
| 2. A design session still misses a head or a `rejected` entry, and the plan review finds it | `thread@rejected-register@rejected-register`, `thread@rejected-register@design-entry-search` | the tripwire **T2**, on the owner's word, round 6 |
| 3. The checkpoint reader costs more than it finds | `thread@rejected-register@checkpoint-reader` | dropped in the reply to round 3, with the thread it watched |
| 4. The conversion loses or changes an entry | `thread@rejected-register@rejected-migration` | the acceptance criterion **AC1**, on the owner's word, round 6 |
| 5. The checker's new rules do not hold | `thread@rejected-register@rejected-register`, `thread@rejected-register@orphan-rejected-entry` | the acceptance criterion **AC2**, on the owner's word, round 6 |

The owner's ruling, round 6: "You can keep both tripwires and both AC".

T1, as the harvest writes it: guards the head of `thread@rejected-register@rejected-citation-direction`.
Fires when a review or an audit finds a `rejected` entry whose only citing heads decide a question
other than the one the design it records answered. Response: reopen that head on how the citing
head is chosen. Re-entry: the decision-record review of every branch that adds a `rejected` entry,
and the design-record axis of `skill@knowledge-architect-project-audit`.

T2, as the harvest writes it: guards the head of `thread@rejected-register@design-entry-search`.
Fires when, after this work ships, the design-conformance review of a plan document reports a
contradicted head, or a `rejected` entry brought back without a recorded reopening, in a discussion
where the design-entry search ran. Response: open a `design` issue naming both, and reopen that head,
asking whether the miss came from a thread that opened after the search. Re-entry: the reviews of
every plan document, and each retrospective.

## Acceptance criteria

### AC1: the conversion maps every old entry to one `rejected` entry, its text intact but for the lost-to clause `##conversion-keeps-every-entry`

Guards `thread@rejected-register@rejected-migration`. Judged at step 2. The observable: a script,
kept in the scratch directory and named with its output in the step's commit message, reads every
old entry from the parent commit and every `rejected` entry from the step's tree, and shows a
one-to-one map over 132 entries, or 131 if the owner rules D1 against its default, the
delegated-state entry removed. For each entry, the new title followed by the new body equals the old
text with the lost-to clause removed, word for word after whitespace and markdown emphasis are
normalized, except the entries whose clause held words beyond its references, per "Mapping
tables": the script lists those, and the session reads each against its old text. Fires when an
entry is missing or doubled, when an unlisted entry differs beyond its clause, or when a listed one
loses the words of its clause. Response: repair the conversion in the same commit; a second firing
reopens `thread@rejected-register@rejected-migration` on how the conversion is checked.

### AC2: the checker's tests hold the register and the orphan rule `##checker-rules-are-tested`

Guards `thread@rejected-register@rejected-register` and
`thread@rejected-register@orphan-rejected-entry`. Judged at step 1. The observables, each a test
shown to fail against a wrong implementation:

- a `rejected@<anchor>@<id>` span resolves to an entry of `planned@<anchor>@docs/rejected.md`, and
  a dangling one is reported;
- a level-three heading of `planned@<anchor>@docs/rejected.md` with no slug is reported;
- a Component with no `rejected` home is reported;
- an entry that no design entry cites is reported, and so is one cited only from a goal, an issue or
  another `rejected` entry;
- an entry cited from a design entry of a directory-shaped design home is not reported: the test
  builds that project itself, in the shape of the mock `dirhome`.

Fires when one fails. Response: repair the checker in step 1; a test that cannot be written reopens
the thread it guards.

## Implementation sequence

Steps 1 and 2 land in one commit, with `cargo klarch install-agent-skills` run inside it, per
"Order rationale".

### Step 1: the checker

- `rejected` in `Registers::built_in`; the rejected-alternatives document out of
  `COMPONENT_DOCUMENTS`, which `Anchors::required_kind` follows; the dead branch of
  `resolve_registers` and its test removed; `rejected` excluded from `records::guarded`.
- The orphan check in `check_under` of `path@core@src/check/references.rs`, with that module's
  count in `PLANTED`.
- The tests that list the document or the four registers brought to five registers and two
  documents, the hand-built projects of `path@core@tests/binary.rs` included.
- Each mock project's Components given `planned@<anchor>@docs/rejected.md` in place of the
  rejected-alternatives document, the title copied from `dirhome` in the mock `unsound` corrected;
  the mock `core`'s home given one entry, `### Deciding the other way` with the slug `other-way`
  and the body "`live`. The fixture's rejected design.", cited from the body of the
  mock's design head `one-decision`; `planted` given one orphan entry as its planted defect; the
  head comment of `planted`'s manifest, which says its widget carries "every document but one",
  read again against the widget's files.
- The AC2 tests.
- The comments the table "Decisions the work rewrites" assigns to step 1, in `path@core@src/` and
  in the `%%` comments of `path@agent-skills@content/agents/standing-state-reviewer.md` and
  `path@agent-skills@content/agents/routing-reviewer.md`; the module doc of
  `path@core@src/check/registers.rs`, which names "its rejected-alternatives document".
- The primer's section `checker-model` naming `rejected` and the documents every Component carries,
  which the test of `path@core@src/agents.rs` requires.

Fails alone on: `cargo test -p knowledge-architect`.

### Step 2: this repository's record

- The five rejected-alternatives documents converted by the mapping table, each with `git mv` to
  `planned@<anchor>@docs/rejected.md`: one subagent per Component proposes the slugs, the session
  reviews them, and one subagent per Component applies them and adds each citation to its citing
  heads.
- The five core heads of "Decisions the work rewrites" rewritten as that table says, since their
  texts must pass in this commit; the heads that table marks "read at step 2" read again against
  the change.
- The four design-home introductions rewritten, per "Mapping tables".
- Every site found by `git grep -n 'rejected-alternatives'` and by
  `git grep -n -i -E 'rejected alternative|lost to'` over the tree re-pointed or reworded, the
  installed text under `path@agent-skills@content/` and the two other specs included; in
  `spec@plans@path-quickfixes`, its harvest row that relies on entries that "still lost to" a head
  reworded to the citing heads.
- The issues and tripwires of "Standing entries the discussion's search returned" marked "updated at
  step 2" or "gains its `rejected` reference at step 2" updated.

AC1 judged. Fails alone on: `cargo klarch check`.

### Step 3: the installed text

Under `skill@knowledge-architect-agent-configuration`, read whole, as `path@agent-skills@CLAUDE.md`
says of the installed source. The texts, by file:

- `path@agent-skills@content/PRIMER.md`: the knowledge table's row, "a design not kept, and why",
  home `register@<anchor>@rejected`, leaving as "Citations" says an entry leaves; the reference
  rule, "a design head names the `rejected` entries its argument covers"; "What a head holds", the
  head citing those entries.
- `path@agent-skills@content/skills/decision-recording/SKILL.md`: the reversal check moves the
  incumbent into `rejected` and the new head cites it; the three-homes table; the section
  `losing-alternatives`: the home, the template of "An entry", no lost-to clause, the head of
  absence for a design not built, the leaving rule of "Citations", the sentence on `void` kept with
  its meaning; "Before you finish": every entry written is cited by a head.
- `path@agent-skills@content/skills/design/SKILL.md`: step 1 dispatches the design-entry searcher
  and reads its grades, as "The design-entry search" says; the re-dispatch for a later thread; the
  slug collision check on both registers; the decision record and the reopening rule naming
  `rejected` entries.
- `planned@agent-skills@content/agents/design-entry-searcher.md`: the agent of "The design-entry
  search", its dispatch rule written as D5 rules.
- `path@agent-skills@content/agents/standing-entry-searcher.md`: one sentence of its description
  telling it apart from the new searcher.
- `path@agent-skills@content/agents/decision-record-reviewer.md`,
  `path@agent-skills@content/agents/design-conformance-reviewer.md`,
  `path@agent-skills@content/agents/design-record-auditor.md`,
  `path@agent-skills@content/workflows/design-record-audit.js`,
  `path@agent-skills@content/skills/project-audit/SKILL.md`: a `rejected` entry is named by its
  reference; an entry owes a citing head; the old paragraph shape leaves.
- `path@agent-skills@content/skills/planning/SKILL.md`, `path@agent-skills@content/skills/setup/SKILL.md`,
  `path@agent-skills@content/skills/retrospective-intake/SKILL.md`,
  `path@agent-skills@content/skills/retrospective/SKILL.md`,
  `path@agent-skills@content/skills/agent-configuration/SKILL.md`,
  `path@agent-skills@content/skills/review/SKILL.md`,
  `path@agent-skills@content/agents/cold-implementer-reviewer.md`,
  `path@agent-skills@content/agents/transcript-reviewer.md`,
  `path@agent-skills@content/agents/workflow-auditor.md`: the words "rejected alternatives" and
  "lost to" brought to the register; in the planning skill, per D3, the row "What is already
  decided" adds the entries the design-entry search graded `bears`.
- `path@agent-skills@content/skills/goal-setting/SKILL.md`: its `%%` comment citing
  `design@agent-skills@a-reference-claims-a-revisit`, read against the rewritten row.
- This repository's own `skill@klarch-development`, whose grounding line names
  `register@<anchor>@rejected`, and `agent@klarch-changelog-reviewer`, which reads "every design
  head, rejected alternative and commit message".
- This spec's three `planned` spans to the new agent, rewritten as `path` spans once the file
  exists.

Then `cargo klarch install-agent-skills`, and the installed copies committed. Fails alone on:
`cargo klarch check` and the agent-facing-text review.

### Step 4: the changelog and the READMEs

- The `Next release` section's entries, per `design@knowledge-architect@changelog-entries` and
  `design@knowledge-architect@versioning-policy`, read whole: a Migration entry as "The conversion"
  says, and entries for the check and the agent; then `cargo x changelog`.
- `path@core@README.md`: "Ten registers are compiled in", the built-in levels, the registers that
  "accept nothing", and its sentence "What a tripwire guards is an entry of any register but
  `issue` and `tripwire`", which `rejected` now joins.
- `path@knowledge-architect@README.md`: its row for docs/.
- `path@agent-skills@README.md`: its account of the design skill's grounding, and its words on a
  losing alternative.

Fails alone on: the changelog copy test of `cargo x gates`.

### Step 5: the reviews, the report and the harvest

The review of the work under `skill@knowledge-architect-review`; the report on AC1 and AC2; the
harvest by its table; the review before the merge, which reads the harvest; this spec's deletion in
the commit that completes the harvest.

## Order rationale

- Step 1 before step 2: the conversion's trees pass only under the checker that knows `rejected`.
  The two land in one commit, per `primer@branch-commits`, with the re-points of step 2 in the
  installed text and the install that brings its installed copies to the shipped text, which
  `cargo klarch check` compares byte for byte.
- Step 2 before step 3: the installed text describes the record the conversion produced, and its
  examples are checked against it.
- Step 3 before step 4: the Migration entry describes the installed text it ships with.
- Step 4 before step 5: the review reads the whole change.

## Defaults awaiting the owner

- **D1**, bears on `thread@rejected-register@orphan-rejected-entry`. The owner, round 3: "the entry
  you found should just be removed IMO. It is an old artifact from the past of the design skill, it
  is no longer needed IMO." The entry records "The state was decided and reversed in one revision
  of designing-together, after five of that revision's eight blocking review findings fell on it",
  which recording test 3 of `skill@knowledge-architect-decision-recording@losing-alternatives`
  admits, and `design@agent-skills@questions-go-to-their-authority` decides what it rejects.
  Default: the entry is kept, converted like the others, and cited by that head. **Ruled**: the
  owner, after the reviews: "Then you can proceed with the defaults D1 to D4."
- **D2**, bears on the owner's direction of round 2, "I want this work to be done as a spec with a
  single PR to the end, in this session." Default: the spec merges first in a pull request of its
  own, then the work in one pull request, for the reason "Status and audience" gives. **Ruled**, by
  the same words.
- **D3**, bears on `thread@rejected-register@design-entry-search`. Default: the planning skill's
  section "What is already decided" also lists each entry the design-entry search graded `bears`,
  with its outcome, as it lists each standing entry today, so a plan document's reader sees the
  record its discussion read. **Ruled**, by the same words.
- **D4**, bears on no thread: the standing-entry searcher's return has no list of its own for an
  entry kept on a doubt; in this discussion 12 of 41 returned entries carried one in their reason.
  Default: left out of this work; an issue entry, kind `todo`, in `register@agent-skills@issue`, at
  the harvest. **Ruled**, by the same words.
- **D5**, a material finding against `thread@rejected-register@design-entry-search`, which the
  owner closed on shape p1, round 4: "p1 approved. I believe grouping by quantity only would most
  likely be a bad decision, because then the subagent might need to read completely unrelated
  entries, and get a partial read of a section in a component's design. Also, the rejected and kept
  entries about similar subjects have to be read together. The point here is that the searcher
  reads the components design and rejected registers in full, and reports a list of related
  entries." The design-conformance
  review of this spec found the rejected entry of agent-skills "One search agent per anchor", whose
  reason is "An anchor bounds nothing", and the bound of 60 entries per agent that
  `design@agent-skills@standing-entry-search-groups` and
  `design@agent-skills@design-record-axis-dispatch` argue. The premise the agent supplied,
  `argument@rejected-register@a38`, that the bound "only matters once one Component's home
  outgrows one agent's reading", is false: agent-skills holds 152 design heads and 51 rejected
  entries, core 108 and 72, by `grep -c '^### '` and the counts of "This repository's record
  today". The largest level-two subject of a design home holds 40 heads, core's "What a tree
  carries: anchors and registers", by an awk count of level-three headings per level-two heading.
  Default: the cut along whole subjects that "The design-entry search" states. It changes the
  ruling: for a Component of more than 60 design entries, no single searcher reads its registers
  in full, as the owner's last sentence asks; each reads whole subjects, up to 60 heads, with the
  `rejected` entries those heads cite. It keeps the rest of the owner's words: no section is cut,
  kept and rejected entries of one subject are read together, and grouping is not by quantity
  alone. **Ruled**, otherwise: the owner, after the reviews: "For D5 though, I'm not really
  convinced the 60 entries limit is truly sensible. I would not put a numerical limit on that. But I
  agree with the split when a component holds too many entries. Just that the cut has to keep
  making sense at all costs. A component that has a level two subject with more than 60 entries to
  itself is probably not compartimenting the project sufficiently, and that is probably something an
  agent should report." "The design-entry search" states the shape: a split along whole subjects,
  no numeric bound, and an agent reporting a subject too large to read whole. The rejected entry
  "One search agent per anchor" and the bound of 60 stand for standing entries; the owner chose
  otherwise for design entries.

## Harvest

At step 5, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's
commit, with the test it fails.

| item | home |
| --- | --- |
| `thread@rejected-register@rejected-register`, with its name and home | a new head of `register@core@design`, written at step 2 with the five rewritten core heads, since the converted record cites it; `design@core@components-carry-the-same-documents`, `design@core@registers-are-declared`, `design@core@an-entry-is-a-heading-at-the-register-level`, `design@core@anchor-collisions-refused` and `design@core@reserved-anchors` rewritten as "Decisions the work rewrites" says, at step 2 |
| `thread@rejected-register@orphan-rejected-entry` | a new head of `register@core@design` |
| `thread@rejected-register@rejected-citation-direction` | a new head of `register@agent-skills@design`; `design@agent-skills@a-reference-claims-a-revisit` and `design@agent-skills@losing-alternatives-filter` rewritten |
| `thread@rejected-register@rejected-scope` | `design@agent-skills@losing-shape-test` rewritten |
| `thread@rejected-register@rejected-entry-shape` | the installed text of step 3; a head only where `primer@design-heads` admits one |
| `thread@rejected-register@rejected-migration` | the changelog's Migration entry, step 4 |
| `thread@rejected-register@design-entry-search` | a new head of `register@agent-skills@design`, with D5 as ruled; `design@agent-skills@standing-entries-searched-before-the-work`, `design@agent-skills@thread-slug-is-entry-id` and `design@agent-skills@design-record-axis-dispatch` rewritten |
| `thread@rejected-register@search-relevance-grade` | a new head of `register@agent-skills@design`, or a clause of the head of `thread@rejected-register@design-entry-search` |
| `thread@rejected-register@proposals-read-against-record` | through `thread@rejected-register@design-entry-search` |
| every item of "Losing alternatives" | the `rejected` home of the Component of the head named beside it, as the recording tests admit, cited by that head |
| T1, T2 | `register@agent-skills@tripwire`, as the Premortem section words them |
| AC1, AC2 | reported on at the landing; each spent, or proposed to the owner as a tripwire if it recurs |
| D4 | an issue entry, kind `todo`, in `register@agent-skills@issue` |
| the owner's later coverage tool, `argument@rejected-register@a46` | opened on the owner's word before the work: `issue@core@searcher-reports-are-written-and-checked-by-hand` |
| this spec | deleted in the harvest's commit, cited as `spec@plans@rejected-register` |

## Later consequences

- The design audit of a slice or of a spec, under
  `skill@knowledge-architect-planning@working-a-slice`, and the retrospective intake dispatch the
  standing-entry searcher and not the design-entry searcher. A later change that sends the new
  searcher there argues it under `skill@knowledge-architect-design`.
- The owner's coverage tool, `argument@rejected-register@a46`, is
  `issue@core@searcher-reports-are-written-and-checked-by-hand`, for both searchers.
- A design-entry searcher's report of a subject too large to read whole is the signal the owner
  named that a Component's design home is not divided enough; a later change that acts on such
  reports argues it under `skill@knowledge-architect-design`.
