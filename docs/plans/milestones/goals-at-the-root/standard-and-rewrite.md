# The standard, and the rewrite of this repository's goals

The first slice of `milestone@plans@goals-at-the-root`. It leaves when it lands. Its threads, its
arguments and its decided design are in the milestone document, which every slice cites.

## Builds

In this order, which `thread@goals-at-the-root@goals-are-edited-in-one-pass-under-the-whole-set-review`
fixes:

1. **The installed text.**
   - `path@agent-skills@fragments/goal-standard.md`: the three kinds; one direction; no measure; no
     refinement; the scope kind with rule 8's condition; no rule on a Component's goal. Each rule
     keeps a `%%` comment with its case and its reason.
   - `path@agent-skills@content/skills/goal-setting/SKILL.md`: the entry shape, a statement with no
     measure and no fixed block of cases; the reach by layer shown to the owner in the exchange;
     where each goal sits, the root's goals home; no step for a Component's goals.
   - The goal reviewer's source under `path@agent-skills@content/agents/`: it judges the kind, the
     direction, fidelity to the owner's statements, a contradiction between goals, and a clause a
     design head could carry; it blocks on no question of when, how much or by what means; its
     grades on a measure leave.
   - The primer's source: the knowledge table's row for a goal, `primer@question-authority`,
     `primer@design-heads` and its table of what the owner brings, `primer@checker-model` where it
     says what every Component carries, and the sentence of `primer@where-knowledge-goes` on what a
     goals home cites.
   - The component-creation skill and the setup skill: a Component is created on a design argument,
     its purpose stated by a head of the root's design home; no occasion for an intermediate
     Component.
   - The head-ground fragment and the decision-recording skill: a head states, beside its reference
     to a goal, the constraint it derives; the decision-record reviewer compares the readings of
     one goal.
   - The installed copies under `path@agent-config@skills/` and its siblings, written by
     `cargo klarch install-agent-skills`.
2. **The analysis**, one file in `path@knowledge-architect@docs/goal-reviews/`, written per theme
   and committed per theme. For each of the 54 goals: its kind, or that it is no goal; kept,
   merged, turned into a head, or removed; and, where it is not kept whole, a table of its
   sentences, each with a destination of the milestone document's second mapping table. It edits no
   goals home.
3. **The one pass.** Every goal is edited in the working tree: the kept goals rewritten under the
   new standard in the root's goals home, `planned@knowledge-architect@docs/goals/`, a README and
   one file per theme; each Component's goals home left with no entry. The whole-set review then
   runs in rounds, one reviewer per theme and one over the whole set, the owner answering between
   rounds, the record committed each round.
4. **The record.** The heads the analysis argued, in the home each belongs to; each entry the
   milestone document's table gives slice 1, rewritten, reversed or removed; every reference to a
   goal that moved or left, re-pointed, in every text `cargo klarch show` lists; the issues named
   in the milestone document, closed or edited; the changelog's entries.

## Claims

Each claim on a corpus is tested by reading the corpus, the search being a lead.

- **No goal carries a measure, a fixed block of cases or a refinement.** Test: over every goals
  home, `grep -c '^It is met while'`, `grep -c 'for instance:'` and `grep -c '^It refines'` each
  print 0. Shown to fail: the three commands on the commit before the one pass print 54 or fewer
  and more than 0.
- **No Component's goals home holds an entry.** Test: `grep -c '^## '` over each Component's
  docs/goals.md prints 0. Shown to fail: the same on main prints 33 in total.
- **Every sentence of a goal that was removed, merged or shortened has a row in the analysis.**
  Test: the reviewers of the whole-set review read each such goal, at the commit before the one
  pass, against its table. No command shows it.
- **No head the slice writes stands unargued.** Test: the decision-record axis of the review
  before the merge, over every head the diff adds.
- **Every reference resolves.** Test: `cargo klarch check`, and `cargo klarch commits origin/main..HEAD`.
  Shown to fail: a goal removed with a reference to it left in place is a finding.
- **The installed copies are the shipped text.** Test: the installed-file check of
  `cargo klarch check`. Shown to fail: an edit of the source with no install is a finding.

No claim is tested by a run built to watch a session apply the new standard, per
`design@agent-skills@synthetic-evidence-not-built`. The one pass over this repository's own goals is
the standard's test.

## Audit subjects

- `register@agent-skills@design`, its subdocument on goals whole, 24 heads, and the heads the
  milestone document's table gives slice 1.
- `path@agent-skills@docs/grounding/goal-standard.md`, which several of those heads argue from.
- `FRAGMENT_HOSTS` in `path@agent-skills@src/lib.rs`: the hosts of each fragment the slice edits.
- The 13 goals homes, and for each goal the list `cargo klarch show` gives of what cites it.
- Five questions the discussion left to the analysis: whether
  `goal@agent-skills@goals-state-the-owner-s-intent`,
  `goal@check@relocation-needs-only-goal-changes` and
  `goal@cli@a-refused-invocation-does-no-work` stay goals; the wording of
  `goal@agent-skills@only-needed-instructions` once measures leave; and the reason the gates
  library depends on nothing of the checker.
- Whether `goal@agent-skills@session-orchestrates-the-work` is a purpose or a means to two other
  outcomes: the session raised it in round 2, and the owner answered that the goal is orthogonal to
  the goal it names, not which of the two it is.
- A scope goal under the new shape: the discussion tested two purposes and no scope;
  `goal@knowledge-architect@serves-one-kind-of-project` is the long one.
- The stale quotation in `design@agent-skills@audit-outcomes`: it quotes the complete-workflow
  goal as "says what it asks plainly enough", where the goal reads "says what it requires plainly
  enough". Fixed in a commit of its own.

## Fails alone on

- A goal that the three kinds cannot sort, or a sentence that none of the five destinations takes.
- A round of the whole-set review whose count of blocking findings does not fall.

## Premises that expire

- The checker accepts a goals home with no entry. Slice 2 ends it; until then each Component keeps
  its empty docs/goals.md, and nothing guards a session against writing a goal there but the
  installed text.
- The milestone document cites goals of Components, in its first mapping table and elsewhere. The
  commit of the one pass re-points each citation of a goal that moved, and names by its bare slug a
  goal that left, per `primer@where-knowledge-goes`.
- `goal@agent-skills@only-needed-instructions` keeps an instruction where a goal's body or measure
  states its act. Until the whole-set review has ruled that goal, no installed instruction is
  removed on the need test.

## Implementation sequence

1. The fragment, the goal-setting skill and the goal reviewer, with the heads that argue them.
   Fails alone on the build or the installed-file check.
2. The primer, the component-creation skill, the setup skill, the head-ground fragment and the
   decision-recording skill, with their heads. Fails alone on a host in which an edited fragment's
   sentence is false.
3. The analysis, theme by theme. Fails alone on a goal or a sentence it cannot sort.
4. The one pass and the rounds of the whole-set review. Fails alone on a round that does not
   converge.
5. The record: the argued heads, the re-pointed references, the issues, the changelog. Fails alone
   on `cargo klarch check`.
6. The report on `acceptance@goals-at-the-root@the-goals-home-is-not-larger`, and the harvest.

Steps 1 and 2 land before step 3 so that the analysis and the rounds run under the new rules. Step
3 lands before step 4 so that no goal is edited before every goal is sorted. Step 5 follows step 4
in the same commit where a reference would dangle between them.

## Harvest

| what | where | when |
| --- | --- | --- |
| the threads on the standard: the three kinds, one direction, no measure, the example, no refinement | `register@agent-skills@design`, its subdocument on goals, each head rewritten in place or reversed as the milestone document's table says | at this slice's landing |
| goals at the root only, as an instruction of the installed text | `register@agent-skills@design`, with `design@agent-skills@goals-required` reversed | at this slice's landing |
| a goal's reading as a head's premise | `design@agent-skills@standing-argument-in-head` and the head on the decision-record axis | at this slice's landing |
| the installed text's silence on an intermediate Component | `design@agent-skills@intermediate-component-occasion`, rewritten; `design@knowledge-architect@no-intermediate-component`, removed | at this slice's landing |
| each head the analysis argued | the design home of the Component its decision constrains, per `skill@knowledge-architect-decision-recording@owning-component` | at this slice's landing |
| the premortem's first cause, and the losing alternatives | as the milestone document's harvest says | at this slice's landing |
