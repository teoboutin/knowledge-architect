# The standard, and the rewrite of this repository's goals

The first slice of `milestone@plans@goals-at-the-root`. It leaves when it lands. Its threads, its
arguments and its decided design are in the milestone document, which every slice cites.

## Builds

In this order, which `thread@goals-at-the-root@goals-are-edited-in-one-pass-under-the-whole-set-review`
fixes:

1. **The installed text.**
   - `path@agent-skills@fragments/goal-standard.md`: the three kinds; one direction; no measure; no
     refinement; the scope kind with rule 8's condition; no rule on a Component's goal. Each rule
     keeps a `%%` comment with its case and its reason. Every citation of a rule of the standard by
     its number, in any installed text, is found by a search and rewritten, the numbers changing.
   - `path@agent-skills@content/skills/goal-setting/SKILL.md`: the entry shape, a statement with no
     measure and no fixed block of cases; the reach by layer shown to the owner in the exchange;
     where each goal sits, the root's goals home; no step for a Component's goals.
   - `path@agent-skills@content/agents/goal-reviewer.md`: it judges the kind, the direction,
     fidelity to the owner's statements, a contradiction between goals, and a clause a design head
     could carry; it blocks on no question of when, how much or by what means; its rows that grade
     a finding on a measure leave.
   - `path@agent-skills@content/PRIMER.md`: the knowledge table's row for a goal, where it says
     what a goal's entry holds; `primer@question-authority`; `primer@design-heads` and its table of
     what the owner brings.
   - The component-creation skill and the setup skill: a Component is created on a design argument,
     its purpose stated by a head of the root's design home; no occasion for an intermediate
     Component. The setup skill's two template goals for the maintenance crate leave: what they
     stated becomes a recommendation of the setup skill, which a target project records as a design
     head.
   - `path@agent-skills@fragments/head-ground.md` and the decision-recording skill: a head states,
     beside its reference to a goal, the constraint it derives. The comparison of the readings of
     one goal by the decision-record axis is not built, on D3 of the milestone document: the slice
     opens an issue of kind `design` in `register@agent-skills@issue` that holds it, with the cause
     of the premortem it answered and the session's two doubts.
   - Each other installed text that a row of the milestone document's table makes false.
   - The installed copies under `path@agent-config@skills/` and its siblings, written by
     `cargo klarch install-agent-skills`.

   **Left to slice 2**: every sentence that says what the checker requires or that writes the form
   `register@*@goal`, among them `primer@checker-model` where it says what every Component carries.
   Until slice 2 the checker still requires each Component's goals home, so those sentences stay
   true, and the installed text says in the goal-setting skill that such a home holds no entry.
   With the pass over the standard goes the pass over the section of 24 heads on goals that the
   milestone document names for the closed entry on those heads.
2. **The analysis**, a file of its own in `path@knowledge-architect@docs/goal-reviews/`, beside
   the record of the whole-set review, written per theme and committed per theme. It proposes the
   themes. For each of the 54 goals: its kind, or that it is no goal; kept, merged, turned into a
   head, or removed; and, where it is not kept whole, a table of its sentences, each with a
   destination of the milestone document's second mapping table, and the argument of each head it
   proposes. It edits no goals home. **The owner rules the analysis before the one pass**: the
   themes, the fate of each goal, and every sentence sent nowhere.
3. **The one pass.** Its first commit edits every goal as the analysis was ruled: the kept goals
   rewritten under the new standard in the root's goals home,
   `planned@knowledge-architect@docs/goals/`, a README and one file per theme; each Component's
   goals home left with no entry; every reference to a goal that moved or left re-pointed, in every
   text `cargo klarch show` lists; and each head the analysis argued, written. That commit passes
   the checker, as every commit does, per `primer@branch-commits`. The whole-set review then runs
   in rounds, one reviewer per theme and one over the whole set, the owner answering between
   rounds, the record and the round's drafts committed each round, as
   `design@agent-skills@whole-set-rounds` says. A round that changes which goals exist re-points
   in its own commit.
4. **The record.** Each entry the milestone document's table gives slice 1, rewritten, reversed or
   removed; a head that argues the existence of `check` and one for `xtask`, each under the design
   skill, and `design@gates@gates-crate` moved to the root's design home; the issues named in the
   milestone document, closed or edited; the changelog's entries, written as
   `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`
   say, with the Migration entry `design@agent-skills@audit-triggers` owes a version that changes
   what recorded content must meet, the two slices shipping in one version. The analysis and the
   review's record leave in the commit that writes the last ruled goals, or a later one of the
   branch.

**Re-pointing a head's reference to a goal.** 62 design heads cite one of the 16 goals the first
mapping table sorts as a head or as merged, a count the decision-record reviewer of this document
took over every design home; the audit takes it again. Where such a goal
becomes a head, the citing head's reference is re-pointed to it, which does not touch the citing
head, per `skill@knowledge-architect-decision-recording@head-shape`. A citing head whose argument
leans on the goal binding outright, or quotes its measure or one of its cases, is rewritten, and is
then brought to the rules on design heads in that change.

## Claims

Each claim on a corpus is tested by reading the corpus, the search being a lead.

- **No goal carries a measure, a fixed block of cases or a refinement.** Test: summed over every
  goals home, `grep -c '^It is met while'`, `grep -c 'for instance:'` and `grep -c 'refines'` give
  0, 0, and the 2 lines of the root's home that use the word in another sense. Shown to fail: on
  the commit before the one pass the three sums are 54, 183 and 27.
- **No Component's goals home holds an entry.** Test: `grep -c '^## '` over each Component's
  docs/goals.md prints 0. Shown to fail: the same on main sums to 33.
- **Every sentence of a goal that was removed, merged or shortened has a row in the analysis.**
  Test: the reviewers of the whole-set review read each such goal, at the commit before the one
  pass, against its table. No command shows it, and no wrong implementation is planted: the
  reviewers' reading is the only check.
- **No head the slice writes stands unargued.** Test: the decision-record axis of the review
  before the merge, over every head the diff adds. The same holds: a review is its only check.
- **Every reference resolves.** Test: `cargo klarch check`, and `cargo klarch commits origin/main..HEAD`.
  Shown to fail: a goal removed with a reference to it left in place is a finding.
- **The installed copies are the shipped text.** Test: the installed-file check of
  `cargo klarch check`. Shown to fail: an edit of the source with no install is a finding.

No claim is tested by a run built to watch a session apply the new standard, per
`design@agent-skills@synthetic-evidence-not-built`. The one pass over this repository's own goals is
the standard's test.

## Audit subjects

- `register@agent-skills@design`, its subdocument on goals whole, 24 heads, and the heads the
  milestone document's table gives slice 1. The table is not the full list for that subdocument:
  the audit reads every head of it.
- `path@agent-skills@docs/grounding/goal-standard.md`, which several of those heads argue from.
- `FRAGMENT_HOSTS` in `path@agent-skills@src/lib.rs`: the hosts of each fragment the slice edits.
- The 13 goals homes, and for each goal the list `cargo klarch show` gives of what cites it.
- The tripwires that cite a head the table rewrites: `tripwire@agent-skills@expectation-set-closes-a-contradiction`,
  `tripwire@agent-skills@owner-fact-recorded-unattributed`,
  `tripwire@agent-skills@open-fork-classed-as-settled` and
  `tripwire@agent-skills@a-shortcut-decision-earns-a-head`.
- Questions the discussion left to the analysis, each put to the owner with the analysis: whether
  `goal@agent-skills@goals-state-the-owner-s-intent`,
  `goal@check@relocation-needs-only-goal-changes` and
  `goal@cli@a-refused-invocation-does-no-work` stay goals; the wording of
  `goal@agent-skills@only-needed-instructions` once measures leave; the reason the gates library
  depends on nothing of the checker; whether `goal@agent-skills@session-orchestrates-the-work` is
  a purpose or a means to two other outcomes; and the clause "through a goal" of
  `goal@knowledge-architect@the-owner-decides`, read as a goal of the three kinds.
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: the owner finds that its
  scope "might be a bit too large", and that "if taken literaly, it would forbid pretty much any
  change to the project". The whole-set review reads it with that statement.
- A scope goal under the new shape: the discussion tested two purposes and no scope;
  `goal@knowledge-architect@serves-one-kind-of-project` is the longest goal. Its rewording is the
  trigger of `issue@check@configuration-for-several-agent-providers` if it changes what the goal
  asks.
- `spec@plans@path-quickfixes`, another plan document, which cites heads this milestone rewrites.
- A stale quotation of a goal, fixed where its head is rewritten:
  `design@agent-skills@plan-read-against-the-record` quotes the drift goal's measure as "a rejected
  design brought back", where the goal reads "a design not kept brought back".

- Items the searchers of the two searches met, which their generated lines do not carry:
  - eight goals homes of Components open with a sentence "Every goal here serves one purpose", and
    the root's defines its terms "for the goals of this home and of every Component": the one pass
    says what becomes of each text above a home's first goal;
  - `rejected@agent-skills@module-directory-component` and
    `design@agent-skills@component-crate-mapping` refuse a Component that is a module directory
    inside a crate, which the drafting of a wider goal on modular units reads first;
  - `design@agent-skills@ruled-items-labelled` lists no label prefix for a file of
    `path@knowledge-architect@docs/goal-reviews/`, and the analysis is such a file whose items the
    owner rules;
  - `issue@create@init-outside-a-worktree-deletes-an-owned-file-silently` says two heads of
    `create` conflict with a goal; if that goal becomes a head, the conflict is between heads,
    which `primer@goals-bind` no longer ranks;
  - `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`: the slice
    edits several installed texts, and meets that defect if it stages them one pair at a time;
  - `issue@agent-skills@a-comment-line-may-copy-the-argument-of-the-head-it-cites` names as an
    instance a comment above the standard in the goal-setting skill's source, which a searcher did
    not find there;
  - the words "rules out what the owner has not ruled out knowingly" of the planning skill read
    close to the name of the cases block that leaves.
- The measure of `goal@agent-skills@session-orchestrates-the-work` names questions of the
  retrospective that the retrospective skill does not hold: a search of its source for "spent its
  context" finds nothing. That sentence is no shape a text already holds, so it does not go
  nowhere by the first row of the second mapping table.

## Fails alone on

- A goal that the three kinds cannot sort, or a sentence that no destination of the second mapping
  table takes.
- A round of the whole-set review whose count of blocking findings does not fall.

## Premises that expire

- The checker accepts a goals home with no entry. Slice 2 ends it; until then each Component keeps
  its empty docs/goals.md, and nothing guards a session against writing a goal there but the
  installed text.
- The milestone document cites goals of Components, in its first mapping table and elsewhere. The
  first commit of the one pass re-points each citation of a goal that moved, and names by its bare
  slug a goal that left, per `primer@where-knowledge-goes`.
- `goal@agent-skills@only-needed-instructions` keeps an instruction where a goal's body or measure
  states its act. Until the whole-set review has ruled that goal, no installed instruction is
  removed on the need test.
- A plan document's sentence that a session drafts goals is no leave to skip step 1 of
  goal-setting. Two earlier slices read one so, as the closed entry on a discovery step recorded.
  The one pass drafts each goal from the owner's statements, those of this milestone's items
  first, and asks the owner where a goal has none.

## Implementation sequence

1. The fragment, the goal-setting skill and the goal reviewer, with the heads that argue them and
   the pass over the 24 heads on goals. Fails alone on the build or the installed-file check.
2. The primer, the component-creation skill, the setup skill, the head-ground fragment and the
   decision-recording skill, with their heads. Fails alone on a host in which an edited fragment's
   sentence is false.
3. The analysis, theme by theme, and the owner's ruling on it. Fails alone on a goal or a sentence
   it cannot sort.
4. The one pass, its first commit carrying the edits, the re-pointing and the argued heads, then
   the rounds of the whole-set review. Fails alone on a round that does not converge.
5. The rest of the record: the remaining rows of the table, the three existence heads, the issues,
   the changelog. Fails alone on `cargo klarch check`.
6. The report on `acceptance@goals-at-the-root@the-goals-home-is-not-larger`, and the harvest.

Steps 1 and 2 land before step 3 so that the analysis and the rounds run under the new rules. Step
3 lands before step 4 so that no goal is edited before every goal is sorted and the owner has ruled
the sorting.

## Harvest

| what | where | when |
| --- | --- | --- |
| the threads on the standard: the three kinds, one direction, no measure, the example, no refinement | `register@agent-skills@design`, its subdocument on goals: a head rewritten in place where the milestone document's table says rewritten; where it says reversed, a new head under the thread's slug, the incumbent judged for `register@agent-skills@rejected` | at this slice's landing |
| goals at the root only, as an instruction of the installed text | `register@agent-skills@design`, a new head under the thread's slug, with `design@agent-skills@goals-required` reversed | at this slice's landing |
| a goal's reading as a head's premise | `design@agent-skills@standing-argument-in-head`, rewritten; the comparison in the issue D3 opens, and no head | at this slice's landing |
| the installed text's silence on an intermediate Component | `design@agent-skills@intermediate-component-occasion`, rewritten and its slug renamed with every reference to it; `design@knowledge-architect@no-intermediate-component`, removed | at this slice's landing |
| each head the analysis argued, and the heads that argue the existence of `check`, `gates` and `xtask` | the design home `skill@knowledge-architect-decision-recording@owning-component` gives each: the root's for which Components exist | at this slice's landing |
| every cause of the premortem but the one on a consumer's pin, and the losing alternatives | as the milestone document's harvest says | at this slice's landing |
