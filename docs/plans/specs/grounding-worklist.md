# One search of the whole record, over clusters the checker computes, with report skeletons it checks

## Status and audience

This spec is the plan of the work that replaces the two grounding searches of the workflow, the
design-entry search and the standing-entry search, with one search of the whole record. The
checker computes the work list: it groups every entry of the record into clusters, packs the
clusters into agents under a bound in words, writes one report skeleton per agent, and checks the
filled reports for completeness. One installed searcher agent reads its clusters whole and fills
its skeleton. The work does four things:

- a new command of the core, `worklist`, writes the clusters, the agents' entries and their report
  skeletons into a directory, and `worklist --verify` checks the filled reports against them;
- one installed agent, `agent@knowledge-architect-record-searcher`, replaces
  `knowledge-architect-standing-entry-searcher` and
  `knowledge-architect-design-entry-searcher`;
- the design skill's step 1, the planning skill's design audit and the retrospective intake's
  grounding dispatch that one search;
- a new shared text of the installed skills, the split standard, says when a subject of a design
  home is split; the decision-recording skill, the decision-record reviewer and the design-record
  auditor host it.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. D1 to D6 are ruled.
- **The work lands as this one spec and a single pull request, on the owner's word**, round 3:
  "This work should land as one spec and a single PR." No commit of the work changes what the
  per-commit gate checks: `worklist` is a command, not a check, and `commits` does not compare a
  commit's installed files with the shipped set (`path@core@src/cli/history.rs`), so the spec needs
  no pull request of its own before its work, per `skill@knowledge-architect-planning@plan-reviews`.
- **The work starts in the session that converged**, on a main branch with no commit since the
  spec's, so it owes no design audit, per `skill@knowledge-architect-planning@working-a-slice`. It
  owes one reading first, under "What is already decided": the `may-bear` entries the grounding did
  not read whole.
- It was assembled from one transcript. The discussion begins at the owner's message that opens "I
  want to refactor the way the grounding steps at the beginning of design skill, design audit of
  slices, and other triggers." and converges at the owner's message that ends "keep all 4 proposed
  AC". The owner's messages are called rounds 0 to 8:
  - rounds 0 to 3, the discussion; the agent's replies to rounds 0, 1 and 2 carry the headings
    "Round 1", "Round 2" and "Round 3" in the transcript, and are called here "the reply to round
    0", "the reply to round 1" and "the reply to round 2";
  - round 4, the owner's question on the packing, which opens "Oh right, I got a question about
    packing clusters", sent while the spec was assembled;
  - round 5, the owner's rulings on D1 to D4 and on the bound, which opens "D1 agreed, plan doc sit
    outside the search";
  - round 6, the owner's rulings on two review findings, which opens
    "goal@agent-skills@installed-text-leaves-room-to-judge is about cases";
  - round 7, the owner's ruling on D5, "D5 accepted, proceed".
  - round 8, the owner's ruling on D6, "d6: it closes".

  Find the file by the opening message, not by its name.

## How the work is done

By `skill@knowledge-architect-planning@working-a-slice`, as its last two paragraphs say for the
work of a spec.

## Names

| name | what it names |
| --- | --- |
| the record | every entry of the registers the graph holds: `goal`, `design`, `rejected`, `issue`, `tripwire`, and every register a project declares, in every anchor, a location's included. Plan documents and their items are not in it |
| entry | one entry of one of those registers, as `path@core@src/records.rs` reads it into a `Record`: a heading entry's section, a file entry's file |
| edge | a reference written inside one entry's section, its heading and the body under it, to another entry. Prose of a home outside every entry yields no edge. Read with the candidate rule `inbound` and `cited_within` of `path@core@src/records.rs` use |
| unit register | every register of the record but `goal`, `rejected`, `issue` and `tripwire`: `design` and every declared register |
| unit | the entries of a unit register grouped as Decided design, "The units", says |
| cluster | a unit, with the `rejected` entries its entries cite, the issues and tripwires that cite any of its entries, and the goals reached from it upward |
| upward | from a goal to the goals it cites, which are the goals it refines, transitively: from a Component's goal to the root's goal it refines |
| residual group | the entries of one anchor that no cluster holds |
| agent | one dispatch of the searcher, given whole clusters and residual groups up to the bound |
| the bound | the number of words one agent is given at most, 25,000 by default |
| words | the count of whitespace-separated tokens in an entry's body, `Record.body` |
| the two searchers | `knowledge-architect-design-entry-searcher` and `knowledge-architect-standing-entry-searcher` |
| the searcher | `agent@knowledge-architect-record-searcher`, the agent the work creates |
| the split standard | the shared text the work creates at `path@agent-skills@fragments/subject-split.md` |
| graded as bearing | graded `bears` or `may-bear` by the searcher: what an older text of the workflow calls an entry a search "returned" |
| the design audit | point 2 of `skill@knowledge-architect-planning@working-a-slice` |
| Q1, Q2 | the two questions put to the owner in the discussion. Q1, the reply to round 0: "Do you intend every search to dispatch the whole record (every cluster plus the residual group), or only the clusters the work touches?" Q2, the reply to round 1: "The worklist notice (default) or a check finding: which incentive do you want?" |
| C1 to C6 | the discussion's names of the six criteria, in the order of the Criteria section: C1 `criterion@grounding-worklist@every-bearing-entry-read`, C2 `criterion@grounding-worklist@tool-computes-the-list`, C3 `criterion@grounding-worklist@any-project-from-its-manifest`, C4 `criterion@grounding-worklist@head-read-with-its-citers`, C5 `criterion@grounding-worklist@bounded-agent-load`, C6 `criterion@grounding-worklist@overlap-cost` |
| F1, F2 | the two findings that opened the reply to round 0: F1, that the owner's citation hierarchy contradicts recorded decisions, `argument@grounding-worklist@arg-8`; F2, that transitive membership merges the clusters, `argument@grounding-worklist@arg-10` |
| the discussion's scripts | throwaway scripts of the discussion, kept in the session's scratch directory and in no tree. Their figures are quoted with their names; step 5 takes the figures that matter again with `worklist` itself |
| thaum | the owner's other project that uses the workflow, a checkout at `../games/thaum` from this repository's root on the owner's machine, pinned at the same version as this workspace, 0.8.0, on the day this spec was written |

## What the work is

What exists today at each site the work touches:

- **The core's commands** are the variants of `Command` in `path@core@src/cli/mod.rs`, dispatched
  in `run` in the same file. None of them computes a work list of entries: `show` prints one entry
  and every site that references it, as file and line; `issues` and `tripwires` print one row per
  entry. None of the three gates on the phases. The writers do: `index` calls
  `complete_working_tree` of the same file before it writes.
- **The core already reads entries.** `path@core@src/records.rs` holds `Record`, with its `body`,
  its `site` and its `group`, the group subdirectory of a file entry, `None` for a Directory entry;
  `records` lists every entry of one kind, keeping its first definition site; `inbound` lists every
  site that references one entity; `cited_within` returns the set of entries of one kind that some
  entry of another kind cites inside its own section, for the orphan check `orphans` of
  `path@core@src/check/references.rs`, per `design@core@orphan-rejected-entry`. No function returns
  an edge, a citing entry with the entry it cites: `cited_within` finds the citing section with
  `section` and returns the cited side only.
- **Two saved workflows already compute a work list**, of the project audit:
  `path@agent-skills@content/workflows/design-record-audit.js` and
  `path@agent-skills@content/workflows/tripwire-audit.js` list every entry and cut groups, per
  `design@agent-skills@design-record-axis-dispatch`, packing small Components first-fit and logging
  every listed entry a return omits. They are outside the work.
- **The two searchers** are `path@elsewhere@crates/agent-skills/content/agents/standing-entry-searcher.md` and
  `path@elsewhere@crates/agent-skills/content/agents/design-entry-searcher.md`, each with the tools `Read`, `Grep`,
  `Glob` and `Bash`. The first takes consecutive positions of the combined, sorted rows of `issues`
  and `tripwires`, in groups of 60 or fewer, and derives its group itself by a sort its body
  states. The second takes one Component, or a group of its level-two subjects that the session cuts
  by judgement. Both return their report as text, by hand; nothing checks it.
- **The installed texts that name the old search**: the design skill's loop step 1,
  `skill@knowledge-architect-design@discussion-loop`; the design audit, which sends the
  standing-entry searcher alone; the retrospective intake's grounding,
  `skill@knowledge-architect-retrospective-intake@ground-before-judging` item 3, with its three `%%`
  comment lines, at lines 59 to 61, naming three heads; and the row "What is already decided" of
  `skill@knowledge-architect-planning@spec-contents`, which names "its design-entry search".
- **No installed text states when a subject of a design home is split.**
  `skill@knowledge-architect-decision-recording@current-design` says when a design home becomes a
  directory, for "a Component whose design has outgrown one file", that a decision goes in the
  subdocument owning its subject, and how the README indexes the subdocuments.
  `knowledge-architect-design-entry-searcher` asks the searcher to report a
  subject whose size strained its reading.
- **The core's README**, `path@core@README.md`, has one section per command and an exit-code
  table, "Exit codes", listing every command's outcome under each code.
- **Released changelog sections cite both searchers**: CHANGELOG.md's section 0.8.0 cites
  `knowledge-architect-design-entry-searcher`, and its section 0.4.0 cites
  `knowledge-architect-standing-entry-searcher`.

**What the record holds today**, measured by the discussion's scripts over `cargo klarch model`, in
this repository at the commit this spec was first written on, and in thaum's working tree on the
same day:

| measurement | here | thaum |
| --- | --- | --- |
| entries: goal / design / rejected / issue / tripwire / interpretation | 25 / 346 / 163 / 100 / 43 / none | 8 / 297 / 189 / 177 / 130 / 69 |
| heads citing a goal directly | 133 of 346 | 47 of 297 |
| largest set of heads joined by design → design citations, a strongly connected component | 159 | 58 |
| subject-rooted clusters, goals as context: clusters; entries reached; reads ÷ distinct | 23; 663 of 677; 839 ÷ 663 | 28, interpretations not counted; 706 of 801; 802 ÷ 706 |
| residual entries, every register counted | 14 | 55 |
| agents at the bound of 25,000 words, spread packing | 9, the lower bound, each at 22,907 to 23,409 words | 11: 8, the lower bound for the clusters within the bound, at 23,952 to 24,195 words, and 3 clusters above it sent alone, of 55,103, 27,217 and 26,424 words |

**Outside the work**, each with what owns it:

- **The search at convergence** of
  `the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`:
  this work closes that issue on other grounds, per #search-delta's withdrawal.
- **A lighter pass for Components a piece of work does not touch**,
  `thread@grounding-worklist@title-tier-for-untouched-components`: parked as a new issue, written at
  the harvest.
- **A rule that every head cites a goal**, or a check of citation direction: ruled out or withdrawn
  in the discussion, under Losing alternatives.
- **The standing-state review before every merge**,
  `design@agent-skills@standing-state-review-before-every-merge`: unchanged. It reads every tripwire
  and deferred trigger by rule, where the search reads by relevance.
- **The dispatch of the project audit's axes**, `design@agent-skills@design-record-axis-dispatch`,
  which `design@agent-skills@tripwire-audit-axis` follows, and its saved workflows: unchanged,
  except the sentence of `design@agent-skills@design-record-axis-dispatch` that borrows the bound of
  60 from `standing-entry-search-groups`, which the harvest rewrites to stand on
  the passes of the axis it ran with.
- **The record check of an audit repair**, `design@agent-skills@repairs-read-against-the-record`:
  unchanged; it is no grounding search.
- **The other work lists** of `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
  a `show` over a changed range and the enclosing entry of each citing site: outside the work. The
  issue is rewritten at the harvest to drop the searcher's use, which this work removes.

## What is already decided

**Decisions the design rests on and does not argue again:**

- `goal@knowledge-architect@agents-work-without-drift`, `goal@knowledge-architect@structure-and-workflow-work-together`,
  `goal@core@records-reach-their-reader`, `goal@core@declared-instructions-are-checked`,
  `goal@agent-skills@installed-text-works-anywhere`, `goal@agent-skills@no-contradiction-in-workflow`.
- `goal@agent-skills@installed-text-leaves-room-to-judge`: the split trigger, a subject whose cluster
  is above the bound, is no fixed mapping it refuses, on the owner's reading of round 6, which
  `argument@grounding-worklist@arg-63` records: the goal covers choices where either shape leaves the
  workflow working, and the split is what keeps it working.
- `design@agent-skills@rejected-citation-direction` and `design@core@orphan-rejected-entry`: a
  head cites its `rejected` entries; the owner chose not to reopen it, round 1.
- `design@agent-skills@a-reference-claims-a-revisit`: the edges are the references this rule has
  sessions write.
- `design@core@one-entity-table`: the graph reads the entity table, and resolves no name of its own.
- `design@core@phases-gate-the-report`: `worklist` writes files, so it refuses over an incomplete
  model, with exit 2, as `index` and `new-component` do.
- `design@core@exit-code-ladder`, `design@core@arguments-parse-through-clap`,
  `design@core@the-core-cli-is-a-library-module`, `design@core@one-question-one-command`,
  `design@core@a-failed-parse-is-loud`, `design@core@nothing-of-a-project-is-compiled-in`,
  `design@core@groups-are-subdirectories`, `design@core@heading-register-two-shapes`.
- `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@capabilities-not-structure`,
  `design@agent-skills@standing-state-review-before-every-merge`.
- `design@agent-skills@shared-text-fragment` and `design@agent-skills@one-home-for-head-rules`: a
  standard several installed texts apply whole is a fragment each hosts, so the split standard is
  one.
- `design@agent-skills@bounded-path-in-design`: the owner's argument that scope is known only after
  grounding, which defeats #scope-by-path.
- `design@agent-skills@behaviour-additions-need-real-use`: the reason
  `design-entry-search` gives for the design audit sending no design-entry
  search. #search-occasions does not reverse it: it adds no instruction on a predicted behaviour.
  The audit's search reads clusters, a cluster holds the heads its standing entries cite, and
  reading those standing entries without those heads would cut the unit,
  `argument@grounding-worklist@arg-41`. The owner approved the occasion, round 2.
- `design@knowledge-architect@changelog-section-fixed-at-release`: a released section's reference
  to a deleted agent is written back as its bare name.

**Decisions the work reverses or rewrites, with every text `cargo klarch show` lists as citing
each, and the step that updates it:**

| decision | citing texts | updated at |
| --- | --- | --- |
| `design-entry-search` | the intake's `%%` line 61; `standing-entries-searched-before-the-work`, now `design@agent-skills@record-searched-before-the-work`, twice; `the-design-entry-search-does-not-run-at-a-design-audit`; `design-entry-search-misses`, now `tripwire@agent-skills@record-search-misses`, twice; `searcher-reports-are-written-and-checked-by-hand`, twice | step 4 |
| `design-entry-search-groups` | `design-entry-search`; `rejected@agent-skills@design-listing-cut-in-groups-of-sixty`; `rejected@agent-skills@subject-groups-bounded-at-sixty` | step 4 |
| `standing-entry-search-agent` | the intake's `%%` line 59; `design-entry-search`; `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`; `searcher-reports-are-written-and-checked-by-hand` | step 4 |
| `standing-entry-search-groups` | the intake's `%%` line 60; `design-entry-search-groups`; `standing-entry-search-agent`; `design@agent-skills@design-record-axis-dispatch`; `rejected@agent-skills@subject-groups-bounded-at-sixty` | step 4 |
| `standing-entries-searched-before-the-work`, now `design@agent-skills@record-searched-before-the-work` | `design@agent-skills@standing-state-review-before-every-merge`; `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`, twice; `the-design-entry-search-does-not-run-at-a-design-audit`; `the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`, four times; `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` | step 4 |
| `design@agent-skills@search-relevance-grade` | `design-entry-search`; `the-standing-entry-searcher-has-no-list-for-a-doubt` | step 4 |
| `design@agent-skills@searcher-grades-goals` | `design@agent-skills@search-relevance-grade`; `tripwire@agent-skills@goal-missed-though-graded`, twice | step 4 |
| `design@agent-skills@a-scratch-directory-per-subagent` | none; its sentence naming the standing-entry searcher's description and its rival's task, "sorts a listing and runs one command per entry", is rewritten | step 4 |
| `design@agent-skills@design-record-axis` | its rule list gains the split standard, beside `primer@design-heads` and `skill@knowledge-architect-decision-recording@head-shape` | step 4 |
| `design@agent-skills@design-record-axis-dispatch` | the sentence borrowing the bound of 60; its citers stay true | step 4 |
| `rejected@agent-skills@one-search-agent-per-anchor` | brought back in part by #residual-per-anchor; its reason is amended to say what defeats it for a residual group | step 4 |

**The texts that cite a removed searcher by its reference**, each updated at step 4:
`design-entry-search`, `design@agent-skills@searcher-grades-goals` and
`standing-entry-search-agent` in `register@agent-skills@design`;
`issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`,
`the-design-entry-search-does-not-run-at-a-design-audit`,
`the-standing-entry-searcher-has-no-list-for-a-doubt`,
`issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` and
`searcher-reports-are-written-and-checked-by-hand`; CHANGELOG.md's sections 0.8.0 and
0.4.0, written back as bare names, then `cargo x changelog`; and this spec, which names them, until
it leaves. A search over the tree at step 4, `git grep -n "agent@knowledge-architect-\(standing\|design\)-entry-searcher"`,
finds every other.

**The `may-bear` entries the grounding did not read whole.** The grounding judged about 50 entries
graded `may-bear` from the searchers' reasons and their titles only, against step 1 of
`skill@knowledge-architect-design@discussion-loop`, which asks each to be read whole. Before step 1
of the work, the session reads each whole and records its outcome, a line each, in the message of
step 1's first commit. They are the `may-bear` entries of the six design-entry searchers' reports
of round 0, and the doubts of the three standing-entry searchers' reports, in the transcript.

**What the discussion's searches returned and graded as bearing, with each outcome**:

- The heads and goals above: bear, as listed.
- `rejected@agent-skills@one-agent-reads-every-standing-entry`, `rejected@agent-skills@design-listing-cut-in-groups-of-sixty`,
  `rejected@agent-skills@subject-groups-bounded-at-sixty`: stay `live`. The packing bound is no
  bound on a subject, so none is brought back.
- `rejected@agent-skills@entry-names-the-decision-it-lost-to`, `rejected@agent-skills@both-directions-written-by-hand`,
  `rejected@agent-skills@both-directions-checked-for-symmetry`, `rejected@core@entry-reachable-by-citing-a-goal`,
  `rejected@core@no-orphan-check`, `rejected@core@goal-citing-a-rejected-entry`: not brought back;
  #citation-direction-rule was withdrawn.
- `searcher-reports-are-written-and-checked-by-hand`: closed by steps 2 and 3.
- `the-standing-entry-searcher-has-no-list-for-a-doubt`: closed by the grades
  `worklist --verify` checks, step 3, and the searcher states, step 4.
- `the-design-entry-search-does-not-run-at-a-design-audit`: closed by
  #search-occasions, as argued under `design@agent-skills@behaviour-additions-need-real-use` above.
- `the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`:
  closed at step 4, its mechanism corrected, per #search-delta.
- `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`: narrowed at step 4.
- `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access`: bears on who writes the
  report. The searcher is an installed agent, which no recorded instance refuses; its fallback is
  under Decided design, "The searcher".
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` and
  `tripwire@core@candidate-rule-silence`: bear as premortem cause P5; no change.
- `issue@agent-skills@audit-axes-beyond-the-design-record`: its "goal coverage" lead gains, at step
  4, the discussion's sample of heads citing no goal, `argument@grounding-worklist@arg-12`.
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: re-read at step 4;
  the search still reads no pending file.
- `issue@core@a-component-states-at-least-one-goal`: a Component with no goal still has its heads
  in clusters, since a unit is a subject, not a goal.
- `issue@core@core-heads-bundle-decisions-with-many-citing-sites`, `issue@core@references-are-read-in-markdown-and-rust-only`,
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`: bear on nothing the work
  builds. The skeleton's format is read back only by `worklist --verify`, so no consumer depends on
  it.
- `design-entry-search-misses`, now `tripwire@agent-skills@record-search-misses` and `tripwire@agent-skills@goal-missed-though-graded`:
  rewritten at step 4, premortem P1.
- `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head`: unchanged.
- `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`: re-read at step 4; its premise,
  no search before work that bypasses the design skill, stays true.
- `tripwire@core@a-second-producer-chain`: not met; `worklist` is a command and no check reads its
  output.

## Criteria

### Every entry the work bears on reaches a reader who reads it whole `##every-bearing-entry-read`

C1. Binding, derived from `goal@knowledge-architect@agents-work-without-drift`, and from the
occasions of `standing-entries-searched-before-the-work`, now `design@agent-skills@record-searched-before-the-work`, which "should miss
nothing". Met by `thread@grounding-worklist@search-scope`,
`thread@grounding-worklist@residual-per-anchor`, `thread@grounding-worklist@subject-unit-both-shapes`
and `thread@grounding-worklist@declared-register-units`.

### The work list and the coverage of the reports are computed by the tool, never by the agent `##tool-computes-the-list`

C2. Binding, derived from `goal@core@records-reach-their-reader` and
`goal@knowledge-architect@structure-and-workflow-work-together`. Met by
`thread@grounding-worklist@report-skeleton-command` and `thread@grounding-worklist@entry-graph`.

### It works in any adopting project from its manifest alone `##any-project-from-its-manifest`

C3. Binding, derived from `goal@agent-skills@installed-text-works-anywhere` and
`design@core@nothing-of-a-project-is-compiled-in`: a project with few goal citations, Components
with no goal (8 of 12 in thaum, per `issue@core@a-component-states-at-least-one-goal`), both shapes
of a heading register's home, a declared register at any level and of any shape. Met by
`thread@grounding-worklist@subject-unit-both-shapes` and
`thread@grounding-worklist@declared-register-units`.

### A head is read together with the entries that cite it `##head-read-with-its-citers`

C4. Binding as a presumption. Its half on kept and rejected entries derives from the owner's reason
recorded in `design-entry-search-groups`, that a subject's kept and rejected
entries have to be read together. Its half on issues and tripwires stands on `argument@grounding-worklist@arg-36`. Met by `thread@grounding-worklist@one-searcher-agent` and
`thread@grounding-worklist@goals-walked-upward`.

### Each agent's load is bounded whatever the project's size `##bounded-agent-load`

C5. Binding as a presumption, derived from `standing-entry-search-groups`. Met
by `thread@grounding-worklist@agent-packing-bound`, except a cluster above the bound, which goes
alone with the notice of `thread@grounding-worklist@split-standard`.

### The overlap, the entries read more than once `##overlap-cost`

C6. Weighed. 1.27× here and 1.14× in thaum, per `argument@grounding-worklist@arg-31`.

## Threads

### The graph's edges are every reference between entries, and plan documents are no nodes `##entry-graph`

Proposed by the owner, round 0; shape by the agent, the reply to round 0. State: `approved`, the
owner, round 5, on D1: "D1 agreed, plan doc sit outside the search". Arguments:
`argument@grounding-worklist@arg-1`, `argument@grounding-worklist@arg-13`. Shape: Decided design,
"The graph". Harvest: the core's head on `worklist`.

### A cluster is rooted at a subject of a unit register, not at a goal `##cluster-unit`

Proposed by the owner as goal-rooted, round 0; the subject-rooted rival by the agent, the reply to
round 0; the leaf-goal shape by the owner, round 1. State: `argued`. Every goal-rooted shape was
ruled out on measured coverage, `argument@grounding-worklist@arg-30`: "The leaf shape loses for the
same reason the first goal-rooted shape did: 213 heads here cite no goal." The owner, round 1:
"your subject rooted approach does look better than my goal rooted one." Arguments:
`argument@grounding-worklist@arg-3`, `argument@grounding-worklist@arg-5`,
`argument@grounding-worklist@arg-10` to `argument@grounding-worklist@arg-12`,
`argument@grounding-worklist@arg-14` to `argument@grounding-worklist@arg-18`,
`argument@grounding-worklist@arg-25`, `argument@grounding-worklist@arg-30`. Shape: Decided design,
"The units". Harvest: the agent-skills head on the search's clusters; the losing shapes, `rejected`
entries if the tests admit them.

### The unit is a subject of a heading register's home, in a file or in each subdocument, or a group of a file register `##subject-unit-both-shapes`

Raised by the owner, round 1: "it has to account for both shapes of the design homes, which can be
either one document, or a directory with multiple documents." Shape by the agent, the reply to
round 1. State: `approved`, the owner, round 2: "subject-unit-both-shapes: Agreed on that shape".
Arguments: `argument@grounding-worklist@arg-26`, `argument@grounding-worklist@arg-34`. Shape:
Decided design, "The units", where the rule for a declared heading register at another level is a
member its argument covers. Harvest: the core's head on `worklist`.

### Goals enter a cluster as context, walked upward, and pull in no standing entry `##goals-walked-upward`

Raised by the owner, round 1: "Also, goals have to be included in clusters by walking upward." The
limit by the agent, the reply to round 1. State: `approved`, the owner, round 2:
"goals-walked-upward: agreed, do not pull in standing entries". Arguments:
`argument@grounding-worklist@arg-24`, `argument@grounding-worklist@arg-26`,
`argument@grounding-worklist@arg-31`, `argument@grounding-worklist@arg-35`. Shape: Decided design,
"The clusters". Harvest: the core's head on `worklist`.

### The residual is grouped by anchor, and packed like a cluster `##residual-per-anchor`

Proposed by the agent, the reply to round 2, replacing #residual-group, the single residual group
the agent proposed in the reply to round 1, which it absorbs. It brings back
`rejected@agent-skills@one-search-agent-per-anchor` in part, which the reply to round 2 did not say;
the design-conformance review of this spec reported it. State: `approved`, the owner, round 3:
"Agreed on residual-per-anchor and declared-register-units." and, on the bring-back, round 6:
"about the other finding, this is not a conflict, since we were already performing a split until
now. in an anchor, the issues do have something in commom, the anchor they apply to." Arguments:
`argument@grounding-worklist@arg-35`, `argument@grounding-worklist@arg-44`,
`argument@grounding-worklist@arg-51` to `argument@grounding-worklist@arg-53`,
`argument@grounding-worklist@arg-64`. Shape: Decided design, "The residual". Harvest: the core's
head on `worklist`; `rejected@agent-skills@one-search-agent-per-anchor`'s reason amended.

### Every declared register is a unit register, grouped by its subjects or its groups `##declared-register-units`

Proposed by the agent, the reply to round 2. State: `approved`, the owner, round 3, as above.
Arguments: `argument@grounding-worklist@arg-51`, `argument@grounding-worklist@arg-54`. Shape:
Decided design, "The units". Harvest: the core's head on `worklist`.

### One installed searcher replaces the two `##one-searcher-agent`

Proposed by the agent, the reply to round 1. State: `approved`, the owner, round 2:
"one-searcher-agent: agreed on this, the separation no longer makes sense with the new process".
Its name, D3, the owner, round 5: "D3 D4 accepted." Arguments: `argument@grounding-worklist@arg-36`,
`argument@grounding-worklist@arg-45`. Shape: Decided design, "The searcher". Harvest: the
agent-skills heads on the search, rewritten.

### `worklist` writes the agents' entries and report skeletons, and `worklist --verify` checks the filled reports `##report-skeleton-command`

Proposed by the owner, round 0; shape by the agent, the reply to round 0. State: `approved`, the
owner, round 1: "report-skeleton-command: agreed on shape, with the searcher agent editing the
report if possible." Its `--seed`, D4, the owner, round 5: "D3 D4 accepted." Arguments:
`argument@grounding-worklist@arg-1`, `argument@grounding-worklist@arg-7`,
`argument@grounding-worklist@arg-21`, `argument@grounding-worklist@arg-43`,
`argument@grounding-worklist@arg-57`. Shape: Decided design, "`worklist`" and
"`worklist --verify`". Harvest: the core's head on `worklist`; the agent-skills head on the grades.

### Clusters are packed into the fewest agents, spread evenly, under a bound of 25,000 words `##agent-packing-bound`

Proposed by the owner as a bound on total entries, round 0; in bytes, then in words, by the agent
and the owner, the replies to rounds 1 and 2. State: `approved`. The owner, round 3:
"agent-packing-bound: put the bound at 25000, keep the measurements and arguments in the design
record for this. My argument is that since we pack clusters, the actual load of a single agent will
rarely reach the bound, instead sitting a little lower. I'm even considering raising it a bit more,
maybe 30000. but that can wait for a few iterations." The premise was measured at the assembly,
`argument@grounding-worklist@arg-61`: it holds under spread packing, not under first-fit. The owner,
round 5: "For the packing: agreed, even spread with fewest agents is right shape. About my premise:
even with it being partially wrong, I stand on 25000, because our run today showed that 22000 does
not put much of a strain, and that is the main reason why I consider increasing it." Arguments:
`argument@grounding-worklist@arg-2`, `argument@grounding-worklist@arg-22`,
`argument@grounding-worklist@arg-32`, `argument@grounding-worklist@arg-38`,
`argument@grounding-worklist@arg-46`, `argument@grounding-worklist@arg-55`,
`argument@grounding-worklist@arg-60` to `argument@grounding-worklist@arg-62`. Shape: Decided design,
"The packing". Harvest: the agent-skills head on the search's clusters, with the measurements and
the arguments, on the owner's word.

### A subject above the bound is named by `worklist`, split as a fix of its own, by a standard every text that writes or judges a head hosts `##split-standard`

Raised by the owner, round 1: "we should provide a standard of how to split meaningfully and strong
incentive to split early." Shape by the agent, the reply to round 1. State: `approved`, the owner,
round 2: "split-standard: agreed on that." and, on Q2: "I think the incentive is right, with the
action carried as a fix in a commit of its own. I think the design record skill and reviewer and
design audits should also be looking for those, so the splits happen early." The reading of "design
audits" as the design-record axis of the project audit, D2, the owner, round 5: "D2 correct
reading". On `goal@agent-skills@installed-text-leaves-room-to-judge`, which the design-conformance
review of this spec raised, the owner, round 6: "goal@agent-skills@installed-text-leaves-room-to-judge
is about cases where either shape lets the agent workflow fully functional. here, the split is
needed to have the workflow function nominally". Arguments: `argument@grounding-worklist@arg-26`,
`argument@grounding-worklist@arg-33`, `argument@grounding-worklist@arg-37`,
`argument@grounding-worklist@arg-47`, `argument@grounding-worklist@arg-58`,
`argument@grounding-worklist@arg-63`. Shape: Decided design, "The split standard". Harvest: a new
agent-skills head.

### The whole record is searched every time `##search-scope`

Proposed by the agent as Q1's default, the reply to round 0. State: `approved`, the owner, round 1:
"overall I believe the whole record is safer. Filtering based on the subject title of a cluster is
bad practice here IMO. The filtering can only be properly done based on the results from the search
I believe." Arguments: `argument@grounding-worklist@arg-23`, `argument@grounding-worklist@arg-27`.
Shape: Decided design, "The scope". Harvest: the agent-skills head on the search.

### The search runs at design step 1, at the intake and at the design audit of a slice or a spec `##search-occasions`

Proposed by the agent, the reply to round 1. State: `approved`, the owner, round 2:
"search-occasions: agreed, this unifies the procedures". Arguments:
`argument@grounding-worklist@arg-41`, `argument@grounding-worklist@arg-49`. Shape: Decided design,
"The occasions". Harvest: `standing-entries-searched-before-the-work`, now `design@agent-skills@record-searched-before-the-work` and
`design-entry-search`, rewritten.

### A lighter pass for the Components the work does not touch `##title-tier-for-untouched-components`

Proposed by the owner, round 1. State: `parked`, the owner, round 2: "title-tier-for-untouched-components:
agreed to park. given your arguments, I would now argue for title reading of design + full read of
standing state. We can keep the leads on this thread in an open issue." Cause: a whole-record
worklist in a real project needs more agents than the owner accepts for one grounding. Re-entry:
the first design discussion in the owner's planned project of about 30 Components, or a
retrospective that reports the search's cost. Its leads are `argument@grounding-worklist@arg-28`,
`argument@grounding-worklist@arg-40`, `argument@grounding-worklist@arg-48` and
`argument@grounding-worklist@arg-65`. Whether they are every lead the owner wants kept is D5, which the owner accepted, round 7.
Harvest: a new issue of kind `design` in `register@agent-skills@issue`, holding the owner's current
position, every lead, the cause and the re-entry.

### The citations follow one direction, checked by the checker `##citation-direction-rule`

Proposed by the owner, round 0. State: `withdrawn`, the owner, round 1: "citation-direction-rule:
you convinced me, I withdraw." Defeating reason: clustering needs no direction, and the hierarchy
would reverse `design@agent-skills@rejected-citation-direction` and forbid goal refinement and
issue dependencies, `argument@grounding-worklist@arg-8`, `argument@grounding-worklist@arg-9`,
`argument@grounding-worklist@arg-19`. Arguments: `argument@grounding-worklist@arg-4`,
`argument@grounding-worklist@arg-8`, `argument@grounding-worklist@arg-9`,
`argument@grounding-worklist@arg-19`, `argument@grounding-worklist@arg-24`. Harvest: a `rejected`
entry if the tests admit it.

### Agent-facing text stays outside the graph `##agent-text-in-graph`

Raised by the owner, round 0. State: `approved`, the owner, round 1: "agent-text-in-graph: agreed
to exclude". Arguments: `argument@grounding-worklist@arg-6`, `argument@grounding-worklist@arg-20`.
Shape: Decided design, "The graph". Harvest: the core's head on `worklist`, which names the
exclusion and its reason.

### The in-change path searches less `##scope-by-path`

Proposed by the owner, round 1, with its doubt: "But I do not think this is a very sound approach,
because scope of change is hard to determine upfront." State: `withdrawn` by its proposer.
Defeating reason: scope is known only after grounding, the owner's own argument in
`design@agent-skills@bounded-path-in-design`, `argument@grounding-worklist@arg-39`. Arguments:
`argument@grounding-worklist@arg-29`, `argument@grounding-worklist@arg-39`. Harvest: none, as the
tests of `skill@knowledge-architect-decision-recording@losing-alternatives` judge it.

### A second search at convergence lists only entries changed since the first `##search-delta`

Proposed by the agent, the reply to round 1. State: `withdrawn` by its proposer, the reply to round
2. Defeating reason: all 5 entries the issue reports as missed existed before their discussion,
`argument@grounding-worklist@arg-56`. The owner, round 2: "nothing is supposed to add an entry
between search and end of design discussion. The suspected mechanism is likely wrong. It was
probably a failure at worklist assembly IMO. I think our mechanical tooling is the correct fix".
Arguments: `argument@grounding-worklist@arg-42`, `argument@grounding-worklist@arg-50`,
`argument@grounding-worklist@arg-56`, `argument@grounding-worklist@arg-57`. Harvest:
`the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`
closes with its mechanism corrected.

## Arguments

### The owner's motive: mechanical tools for the work list `##arg-1`

Round 0, the owner. Threads: #report-skeleton-command, #entry-graph. "My idea is to simplify the
tasks agents have to perform with mechanical tools as much as possible. The main point where I
believe this is important is defining the work list. Right now, I believe this is left a bit to
judgement and requires manual steps which often miss items."

### No size bound splits a cluster; a total bound packs them `##arg-2`

Round 0, the owner. Threads: #agent-packing-bound, #cluster-unit. "I think that clusters should not
be getting a hard size limit (such as 60 entries), because reading a cluster whole is always gonna
be more efficient. [...] But the worklist can still have a bound on total entries when there is
multiple clusters, since this is not a bound that splits meaning."

### A cluster starts at a goal, and clusters may overlap `##arg-3`

Round 0, the owner. Thread: #cluster-unit. "IMO, a cluster has to start at a goal, and include every
design entry that serves it, and every issue/tripwire that affect this subset of the design. Also,
I believe it is okay for multiple clusters to overlap".

### Citations trickle down in one direction `##arg-4`

Round 0, the owner. Thread: #citation-direction-rule. "Overall, citations have to "trickle down"
in a single direction [...] And I am unsure about whether this single way of citation should be
mechanically enforced. Probably it should."

### Orphans are a tell `##arg-5`

Round 0, the owner. Thread: #cluster-unit. "Orphan and small clusters are not a hard problem though,
just a tell that something might not be properly recorded."

### Design shaped only by agent-facing text is a recording failure `##arg-6`

Round 0, the owner. Thread: #agent-text-in-graph. "A case where text that only sits in agent facing
text shapes the design is probably a failure to record the proper entry in the design homes."

### A report skeleton per agent, checked for completeness `##arg-7`

Round 0, the owner. Thread: #report-skeleton-command. "a command that generates one template report
for each agent to be dispatched, holding all the entries they have to judge, which they just fill
out with whether they bear or not, plus a slot for any additional remark, and that we can check for
completeness mechanically at the end."

### The hierarchy contradicts recorded decisions `##arg-8`

The reply to round 0, the agent. Thread: #citation-direction-rule. ""Rejected cites design"
reverses `design@agent-skills@rejected-citation-direction`. [...] Its ground is partly yours". And:
""Goals never cite anything" contradicts the refinement links that goals write today."

### 287 and 591 edges break the hierarchy `##arg-9`

The reply to round 0, the agent, measured with `analyze.py`. Thread: #citation-direction-rule. 287
edges here and 591 in thaum; design → rejected 173 / 203, issue → issue 18 / 255, design → issue 39
/ 32, goal → goal 12 / 0.

### Transitive service merges every goal's cluster `##arg-10`

The reply to round 0, the agent, measured with `analyze.py` and `analyze2.py`. Thread: #cluster-unit.
"159 heads in this repository form a single cycle-connected component [...] 215 of the 346 heads
"serve" all 20 goals," the 20 of the 25 goals that some head reaches, and every goal-rooted cluster holds 466 to 480 of the 677 entries. A design
→ design citation means "depends on", not "serves the same goal"."

### The record's shape, measured `##arg-11`

The reply to round 0, the agent. Threads: #cluster-unit, #agent-text-in-graph. Heads citing a goal
directly: 133 of 346 here, 47 of 297 in thaum; goal-rooted reach 342 of 677 and 208 of 801;
subject-rooted reach 668 of 677 and 707 of 801, a count that took every goal as reached, where
`argument@grounding-worklist@arg-30` counts only the goals a unit reaches, 663; "entries cited from agent-facing text | 111 (99
heads) | 55".

### An orphan is sometimes the reference rule working `##arg-12`

The reply to round 0, the agent. Thread: #cluster-unit. "of the 213 heads that cite no goal here, 44
cite no entry at all. I sampled 8 of them. Some derive from another head and correctly cite only
that head, for example `design@core@index-staged-write`. Others look like real gaps, for example
`design@core@phases-gate-the-report`. So an orphan is sometimes a recording failure and sometimes
the reference rule working as written." The 44 is the session's script's count; a reviewer's
heuristic over the tree gave 43.

### The graph needs no new syntax `##arg-13`

The reply to round 0, the agent. Thread: #entry-graph. "The checker already computes both
directions; `show` uses the inbound one. [...] No new syntax and no direction rule is needed to
build the graph." "**Proposed default:** plan documents are not nodes. They are deliberation, not
record, and they leave when their work lands."

### For goal-rooted: a goal is the owner's intent `##arg-14`

The reply to round 0, the agent. Thread: #cluster-unit. "a goal is your statement of intent, so
grouping by goal makes the grouping follow meaning that the owner wrote."

### Against goal-rooted: completing it needs a migration the reference test refuses `##arg-15`

The reply to round 0, the agent. Thread: #cluster-unit. "Making it complete needs a new rule that
every head cites a goal, migrated over 213 heads here and 250 in thaum. That rule contradicts the
reference test, under which a head cites a goal only when it derives a constraint from it."

### For subject-rooted: the owner's own grouping, with no migration `##arg-16`

The reply to round 0, the agent. Thread: #cluster-unit. "the level-two subjects are already the
meaning-based grouping you ruled for design entries in `design-entry-search-groups`.
Today it reaches 668 of 677 entries here with 1.1× overlap, and needs no migration."

### Against subject-rooted: a subject is layout `##arg-17`

The reply to round 0, the agent. Thread: #cluster-unit. "a subject is a choice of document layout,
not a fact of citation. A design home with no subjects (gates, xtask) becomes one cluster."

### The fact that would flip the agent `##arg-18`

The reply to round 0, the agent. Thread: #cluster-unit. "if you intend every head to cite a goal
anyway, as a goal of the record, then the migration is wanted and goal-rooting stops being a cost."

### Clustering needs no direction `##arg-19`

The reply to round 0, the agent. Thread: #citation-direction-rule. "Clustering does not need a
direction: membership is computed over edges in either direction." "Acyclicity is useful only for
transitive closure, and F2 shows that transitive closure is the wrong membership rule."

### Entry test 2 admits design that lives in one agent-facing text `##arg-20`

The reply to round 0, the agent. Thread: #agent-text-in-graph. "Entry test 2 in `primer@design-heads`
says a decision stated in one agent-facing text has one site and earns no head: its reason lives
beside the instruction." "Their reader is the session that edits that text, which reads it whole
anyway".

### One grade set for every kind, and the report's writer `##arg-21`

The reply to round 0, the agent. Thread: #report-skeleton-command. "one set for every kind of entry,
`bears` (with a relation) / `may-bear` / `unrelated`." "a general-purpose subagent is refused report
writes, per `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access`."

### Packing is no split of a subject `##arg-22`

The reply to round 0, the agent. Thread: #agent-packing-bound. "This is not
`rejected@agent-skills@subject-groups-bounded-at-sixty`, which split subjects." "60 rests on
thaum's mean entry size, 1.6 KB, and on no measurement of recall."

### Choosing clusters is the judgement step to remove `##arg-23`

The reply to round 0, the agent. Thread: #search-scope. "Choosing clusters is the judgement step you
want removed, and a choice made from seeds rests on the reference graph being complete".

### The owner does not reopen the rejected direction `##arg-24`

Round 1, the owner. Threads: #citation-direction-rule, #goals-walked-upward. "I had forgotten the
direction for rejected entries, I don't want to reopen that [...] I overlooked that goals cite other
goals, which is normal and should make the finer goals part of the same cluster."

### The leaf-goal shape `##arg-25`

Round 1, the owner. Thread: #cluster-unit. "a cluster seed is a goal that is at the end of a goal
refinement tree, meaning a goal that is not refined again. But I think it will still loose to
subject rooted shape."

### Both home shapes, a split standard, goals walked upward `##arg-26`

Round 1, the owner. Threads: #subject-unit-both-shapes, #split-standard, #goals-walked-upward. "it
has to account for both shapes of the design homes [...] we should provide a standard of how to
split meaningfully and strong incentive to split early. [...] Also, goals have to be included in
clusters by walking upward."

### Filtering is done from the search's results `##arg-27`

Round 1, the owner. Thread: #search-scope. "Filtering based on the subject title of a cluster is bad
practice here IMO. The filtering can only be properly done based on the results from the search I
believe."

### A lighter pass for untouched Components `##arg-28`

Round 1, the owner. Thread: #title-tier-for-untouched-components. "The components that are not
expected to be touched directly could have a smaller procedure, a searcher that is asked to look
just at entry titles, and go deeper only if a doubt arises from the title."

### A smaller search on the in-change path, doubted `##arg-29`

Round 1, the owner. Thread: #scope-by-path. "for example the in change path of the skill could
reduce the scope. But I do not think this is a very sound approach, because scope of change is hard
to determine upfront."

### The leaf-goal shape reaches 255 to 342 of 677 entries `##arg-30`

The reply to round 1, the agent, measured with `cluster.py`. Thread: #cluster-unit. Heads citing the
leaf: 255 of 677 (residual 422), 208 of 801; heads citing any goal of its chain: 342 of 677, 208 of
801; subject-rooted: 663 of 677, 706 of 801. "The leaf shape loses for the same reason the first
goal-rooted shape did: 213 heads here cite no goal."

### Goals that pull in standing entries double the reads `##arg-31`

The reply to round 1, the agent, measured with `cluster.py` and `cluster2.py`. Thread:
#goals-walked-upward. "Reads rise to 1323 entries for 669 distinct ones. With goals included as
context only, and standing entries attached only through heads, reads fall to 839 for 663 distinct
(1.27×) here, and 802 for 706 in thaum."

### Entry sizes are uneven `##arg-32`

The reply to round 1, the agent, measured with `build3.py`. Thread: #agent-packing-bound. "issue 2.9
KB, design 1.6 KB, tripwire 1.1 KB, goal 0.6 KB, rejected 0.5 KB. A bound counted in entries
therefore bounds context poorly."

### The graph gives no clean split points `##arg-33`

The reply to round 1, the agent, measured with `split.py`. Thread: #split-standard. "Most subjects
are one large connected group of heads plus isolated single heads." "Thaum's state-model subject is
one cluster of 160 entries and 355 KB."

### Heads under no subject form one subject per file `##arg-34`

The reply to round 1, the agent. Thread: #subject-unit-both-shapes. "gates and xtask here, and 9
homes in thaum, are in that case. [...] One rule covers both shapes, since a subdocument with no
level-two heading is the same case as a file home with none."

### The residual's first measure `##arg-35`

The reply to round 1, the agent. Threads: #goals-walked-upward, #residual-per-anchor. "A standing
entry that cites only goals goes to the residual group: 6 here, 13 in thaum." "Here it holds 14
entries [...] In thaum it holds 95".

### Two agents would each read half a cluster `##arg-36`

The reply to round 1, the agent. Thread: #one-searcher-agent. "With two agents, each would read
half of every cluster, against C4 [...] **Nearest rival:** keep two agents with their present
standards. It loses on C4, and on the fact that both standards lean to recall and grade alike."

### A split is a fix of its own; a gate on an unmeasured number is refused `##arg-37`

The reply to round 1, the agent. Thread: #split-standard. "splitting a subject moves headings and
decides nothing, so it is row 2 of `primer@met-outside-the-task`". "a gate that fails on a number
that has not been measured fails `primer@working-prose`'s requirement that a number has a basis."

### A bound in bytes, from this session's loads `##arg-38`

The reply to round 1, the agent. Thread: #agent-packing-bound. "this session's design-entry
searchers read 124, 127, 138 and 143 KB each, plus goals, and none reported that a subject strained
its reading. [...] There is no measurement of recall."

### Scope is known only after grounding `##arg-39`

The reply to round 1, the agent. Thread: #scope-by-path. "scope is known only after grounding. That
is your own argument, recorded in `design@agent-skills@bounded-path-in-design`."

### A title shows no firing clause `##arg-40`

The reply to round 1, the agent. Thread: #title-tier-for-untouched-components.
"`standing-entry-search-groups` records that "a title shows neither a
tripwire's firing clause nor a deferred trigger"." "titles state the rule, per
`design@agent-skills@title-states-the-rule`, so skimming heads by title is more defensible." "No
real use exists yet".

### The unit decides the audit's reading `##arg-41`

The reply to round 1, the agent. Thread: #search-occasions. "At the audit, this adds design entries to what the agents read. That changes
`design-entry-search`, which says the audit sends no design-entry search because
no miss has been recorded there. My argument is that this is no new instruction added on a predicted
behaviour. It follows from the unit: a standing entry is judged against the heads it cites, and at
the audit an agent would otherwise read the standing entries without those heads." "The recorded
ground for the audit sending no design search is weak, so this is a weighing you may overrule." The
recorded ground is `design@agent-skills@behaviour-additions-need-real-use`.

### A delta search at convergence `##arg-42`

The reply to round 1, the agent. Thread: #search-delta. "a cheap second search when convergence is
proposed. This is the open candidate of
`the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`".

### An installed agent can write its report `##arg-43`

The reply to round 1, the agent. Thread: #report-skeleton-command. "An installed agent can write in
its scratch directory; the refusals recorded are of general-purpose agents."

### Are residual entries justified? `##arg-44`

Round 2, the owner. Thread: #residual-per-anchor. "I am curious whether they are justified, because
if yes it means they can grow, which might become big as a single cluster. In particular, I am
surprised by residual tripwires."

### The separation no longer makes sense `##arg-45`

Round 2, the owner. Thread: #one-searcher-agent. "agreed on this, the separation no longer makes
sense with the new process"

### Words over bytes `##arg-46`

Round 2, the owner. Thread: #agent-packing-bound. "the bound unit being KB is not very clear. how
about using a word count instead ? [...] word count translates a bit better to agent tokens I
believe (though it is still an estimation)."

### Reviews and audits look for subjects to split `##arg-47`

Round 2, the owner. Thread: #split-standard. "I think the design record skill and reviewer and
design audits should also be looking for those, so the splits happen early."

### Titles for design, a whole read for standing entries `##arg-48`

Round 2, the owner. Thread: #title-tier-for-untouched-components. "I would now argue for title
reading of design + full read of standing state."

### One procedure everywhere `##arg-49`

Round 2, the owner. Thread: #search-occasions. "agreed, this unifies the procedures"

### Nothing adds an entry during a discussion `##arg-50`

Round 2, the owner. Thread: #search-delta. "nothing is supposed to add an entry between search and
end of design discussion. The suspected mechanism is likely wrong. It was probably a failure at
worklist assembly IMO."

### Thaum's residual tripwires guard a declared register `##arg-51`

The reply to round 2, the agent, measured with `residual.py` and `residual4.py`. Threads:
#residual-per-anchor, #declared-register-units. "Of thaum's 38 "residual" tripwires, 32 guard an
`interpretation@rules@…` entry. With the interpretations counted as nodes, thaum's residual falls
from 95 to 55 entries."

### The residual's shapes are mostly justified `##arg-52`

The reply to round 2, the agent. Thread: #residual-per-anchor. Goals no head serves, 5 / 1; issues
citing only goals, 6 / 14; issues about code with no head, 2 / 33; a tripwire guarding a skill
section, 1 / 0; tripwires guarding something named in words, 0 / 4, the only unjustified class. "So
most residual entries are justified, and they can grow."

### A Component's residual shares its code `##arg-53`

The reply to round 2, the agent. Thread: #residual-per-anchor. "Entries of one Component's residual
concern the same code, which is the only shared ground they have. Each group is packed like a
cluster, and a group above the bound gets the same notice."

### Without declared registers, C3 fails `##arg-54`

The reply to round 2, the agent. Thread: #declared-register-units. "Thaum's interpretations then
form 13 units of 1 to 16 entries. Without this, C3 fails for any project with a declared register."

### Words and bytes differ by about 15% `##arg-55`

The reply to round 2, the agent, measured with `wc -c` and `wc -w`. Thread: #agent-packing-bound.
Bytes per word: 6.3, 6.4, 6.2, 7.1, 6.6. "**Basis:** the two largest loads a searcher of this
session read whole with no strain reported. One was agent-skills' design lines 2135 to 3667 plus the
`rejected` entries they cite" (the file ends at line 3666), " about 19,000 + 3,700 words. The other was core's design lines 1491 to
2959 plus theirs, about 17,500 + 5,000 words."

### The missed entries existed before the search `##arg-56`

The reply to round 2, the agent, measured with `git log -S"##<slug>"` and `--diff-filter=A`. Thread:
#search-delta. The 5 entries were added on 2026-10-07, 2026-10-15, 2026-10-12, 2026-10-03 and
2026-10-13, before their discussion of 2026-10-19. "So a delta search would fix nothing".

### A judgement miss is not an assembly miss `##arg-57`

The reply to round 2, the agent. Threads: #search-delta, #report-skeleton-command. "The issue
records of the second instance that each entry "named a rejected alternative in words". That
points to a judgement miss [...] The worklist removes the assembly failure [...] It does not remove
a judgement miss."

### The reviewers judge by the writer's standard `##arg-58`

The reply to round 2, the agent. Thread: #split-standard. "The decision-record reviewer and the
design-record auditor judge against it, by pointer. `goal@agent-skills@no-contradiction-in-workflow`
requires that a review judge only by the criteria the writer's skill states."

### The premortem's causes `##arg-59`

The reply to round 2, the agent. Threads: all decided ones; see Premortem.

### Packing keeps loads below the bound `##arg-60`

Round 3, the owner. Thread: #agent-packing-bound. "My argument is that since we pack clusters, the
actual load of a single agent will rarely reach the bound, instead sitting a little lower."

### The owner's premise holds under balanced packing, not first-fit `##arg-61`

The assembly, after round 3, the agent, measured with `pack.py` over this repository and thaum. Thread:
#agent-packing-bound. First-fit-decreasing at 25,000 words gives 9 agents here with a median load
of 24,737 words, and 11 in thaum with 24,969: it fills each agent to the bound. Spreading the same
clusters over the same number of agents gives loads of 22,907 to 23,409 words here, 92% to 94% of
the bound, and 23,952 to 24,195 in thaum's 8 agents for the clusters within the bound, beside 3
clusters above it sent alone, of 55,103, 27,217 and 26,424 words. In both projects the number of
agents first-fit-decreasing uses equals the lower bound, the clusters' total words divided by the
bound and rounded up: 9 here, 8 in thaum.

### The owner stands on 25,000 words for what this session's searchers read `##arg-62`

Round 5, the owner. Thread: #agent-packing-bound. "About my premise: even with it being partially
wrong, I stand on 25000, because our run today showed that 22000 does not put much of a strain, and
that is the main reason why I consider increasing it."

### The goal on room to judge does not cover a split the workflow needs `##arg-63`

Round 6, the owner. Thread: #split-standard. "goal@agent-skills@installed-text-leaves-room-to-judge
is about cases where either shape lets the agent workflow fully functional. here, the split is
needed to have the workflow function nominally"

### An anchor is what a residual group's entries share `##arg-64`

Round 6, the owner. Thread: #residual-per-anchor. "this is not a conflict, since we were already
performing a split until now. in an anchor, the issues do have something in commom, the anchor they
apply to."

### The leads of the lighter pass `##arg-65`

The reply to round 1, the agent, and round 1, the owner. Thread:
#title-tier-for-untouched-components. The agent: ""Untouched" can be computed: a cluster of an
untouched Component with an edge into a touched cluster is read whole." and "`worklist` can take a
Component filter later without changing the report format." The owner: "maybe in a project with
more components (such as thaum), filtering based on Components touched would make sense though."
and, of the planned project, "work is likely to be able to happen in a very isolated way between
the various components (~20 would be pretty much standalone, the others depending on those 20 in
various ways)".

## New names, in one place

```text
crates/core/src/records.rs                 new: edges(), every (citing entry, cited entry) pair, each read inside the citing entry's section
crates/core/src/graph.rs                   new module, pub(crate): Role, Entry, Group, Graph, graph(), Packing, pack()
crates/core/src/cli/mod.rs                 Command::Worklist(WorklistArgs); WorklistArgs is pub, as every *Args is
crates/core/src/cli/worklist.rs            new module: fn worklist, the writer and --verify
  WorklistArgs { out: Option<PathBuf>, verify: Option<PathBuf>, bound: usize, seeds: Vec<String> }
  flags: --out <DIR> | --verify <DIR>, exactly one; --bound <WORDS>, default 25000; --seed <REF>, repeatable
files under <DIR>:
  agents.txt                               one line per agent and entry: <agent> <role> <reference>, then `seed` or `cites-seed` where it applies
  agent-<n>/entries.md                     every entry of the agent, whole, each introduced by its reference
  agent-<n>/report.md                      the skeleton the searcher fills
crates/agent-skills/content/agents/record-searcher.md   new: the record searcher
crates/agent-skills/fragments/subject-split.md           new: the split standard
existing, removed: crates/agent-skills/content/agents/standing-entry-searcher.md,
                   crates/agent-skills/content/agents/design-entry-searcher.md
existing: records::{Record, records, inbound, cited_within, section}, Record.group, Record.body,
          cli::complete_working_tree
```

## Decided design

### The graph

The nodes are the entries of the record: every entry of `goal`, `design`, `rejected`, `issue`,
`tripwire` and every declared register, in every anchor, a location's included. Plan documents and
their items are no nodes: they are deliberation and leave when their work lands, D1. An edge is a
reference written inside the citing entry's section to another entry; a new function `edges` of
`path@core@src/records.rs` returns every pair, reading each entry's section as `cited_within` does
and keeping the citing entry. No direction rule is added: membership reads edges either way. The
nearest rival, transitive membership over design → design edges, lost on
`argument@grounding-worklist@arg-10`: it merges every goal's cluster into one.

Agent-facing text is no node, `thread@grounding-worklist@agent-text-in-graph`: a decision stated in
one agent-facing text earns no head by entry test 2 of `primer@design-heads`, and its reader is the
session that edits that text, which reads it whole; and the installed text cites no entry, so in an
adopting project it would add nodes with no edges, `argument@grounding-worklist@arg-20`.

### The units

A unit register is `design` and every declared register. Every entry of a unit register is in
exactly one unit:

- **a heading register whose entries sit at level L, L of 3 or more**: one unit per heading of its
  home shallower than L that stands nearest above an entry, in a file home or in each subdocument of
  a directory home, so two subjects of one text are two units. For `design`, L is 3, so the units
  are the level-two subjects, and the entries under a file's title form one unit of that file;
- **a heading register whose entries sit at level 2**: one unit per file of its home;
- **a file register**: one unit per group subdirectory, per `design@core@groups-are-subdirectories`,
  and one for the entries at the top of its home;
- **a Directory register**: one unit per anchor that carries it, holding every entry of it there.

A unit's key is its anchor, the path of its file or group, and the line of its heading, or nothing
for a unit with no heading. It is named by its heading's text where the heading sits one level
above the entries; by the path where the heading is the file's first, or where there is none; and
by the path and the heading's text under any other shallower heading.

### The clusters

A cluster is a unit and:

- every `rejected` entry an entry of the unit cites;
- every issue and every tripwire that cites an entry of the unit;
- every goal an entry of the unit cites, and every goal reached from those upward.

A goal pulls in nothing: an issue or a tripwire that cites only goals is no member of a cluster
through them, and a goal no unit reaches belongs to no cluster. The rival, goals pulling in their
citers, lost on `argument@grounding-worklist@arg-31`.

### The residual

Every entry no cluster holds is in the residual group of the anchor that holds it, one group per
anchor that has any, keyed by its anchor and `(residual)`. A residual group is packed with the
clusters, in the same pool. It brings back `rejected@agent-skills@one-search-agent-per-anchor` in
part, on the owner's word, round 6, `argument@grounding-worklist@arg-64`: the standing entries were
already cut into groups, and the entries of one anchor share the anchor they apply to. That entry's
reason, "An anchor bounds nothing", is answered by the bound: the group is packed under it, and
named when it exceeds it. A residual group above the bound gets its own notice: its issues are given
group subdirectories, or cite the heads they concern.

### The packing

- **Loads.** The load of a cluster or a residual group is the sum of the words of its entries. An
  entry in two clusters of one agent is given once: `entries.md` holds it once, and `report.md`
  lists it under the first of those clusters in the agent's order and names it under the others as
  graded above. The load an agent prints counts each entry once.
- **Above the bound.** A cluster or a residual group above the bound goes to an agent alone, unsplit,
  and the output prints its notice.
- **Fewest agents.** The others are packed first-fit-decreasing: largest load first, each into the
  first agent it fits in under the bound, a new agent where it fits in none. This fixes n, the
  number of agents.
- **Spread.** The same clusters are then dealt again over n agents, largest first, each to the
  least-loaded agent it fits in under the bound, the lowest-numbered among equals. Where one fits in
  none, the spreading is dropped and the first-fit-decreasing assignment is kept, so the number of
  agents is always n.
- **Order.** Ties of load break on the key, in byte order. Agents are numbered from 1, in the order
  their largest cluster was placed; an agent sent alone above the bound comes after them. Two runs
  over one tree give one packing.
- **Bin packing is NP-hard, and no optimum is sought.** First-fit-decreasing uses at most 11/9 of the
  optimal number of agents, plus 6/9, by the bound György Dósa proved tight in 2007; measured here
  and in thaum, it used the lower bound, the total words divided by the bound and rounded up,
  `argument@grounding-worklist@arg-61`. The spreading is argued from
  `standing-entry-search-groups`: equal groups keep one agent from carrying the
  full bound while another carries a remnant.
- **The bound** is 25,000 words by default, on the owner's word, rounds 3 and 5; `--bound` overrides
  it. Its measurements and arguments are `argument@grounding-worklist@arg-32`,
  `argument@grounding-worklist@arg-38`, `argument@grounding-worklist@arg-46`,
  `argument@grounding-worklist@arg-55` and `argument@grounding-worklist@arg-60` to
  `argument@grounding-worklist@arg-62`.

### `worklist`

`worklist --out <DIR>`:

- builds the model and refuses with exit 2 over a tree that stops before the last phase, through
  `complete_working_tree`, as `index` does, writing nothing;
- refuses with exit 2, writing nothing, a `<DIR>` inside the project's root, since the walk would
  read its files, and a `<DIR>` that exists and is not empty; the dispatcher gives a directory of its
  scratch space, and each agent's directory under it is that agent's scratch directory, per
  `design@agent-skills@a-scratch-directory-per-subagent`;
- refuses with exit 2 a `--seed` that is not a reference or names no entry of the record;
- writes, per agent, `entries.md` and `report.md`, and `agents.txt` for the whole run, and exits 2 on
  a write that fails, naming the path;
- prints one line per agent with its clusters and its load, and one notice per cluster or residual
  group above the bound, and exits 0, notices included.

Illustration of the output's shape, not authority:

```text
agent-3  23,410 words  core :: 3. Names and references; core :: 6. The agent configuration
notice: subject "4. State model — the decisions" (crates/thaum-engine/docs/design/state-model.md)
        holds 55,103 words, above the 25,000-word bound: split it into subjects read apart
notice: the residual entries of thaum-engine hold 26,000 words, above the 25,000-word bound: give
        its issues group subdirectories, or have them cite the heads they concern
```

`agents.txt` gives each entry's role: `unit`, `goal`, `rejected`, `issue` or `tripwire`, then
`seed` for a seed that is itself an entry of the agent, or `cites-seed` for an entry that cites
one. `entries.md` holds, under one heading per cluster, each
entry introduced by one line, `=== <reference>  <file>:<line>`, then its body whole. `report.md`
lists every entry of the agent, cluster by cluster, one line each, and ends with a section for what
the searcher met outside the task. An entry with an edge to a seed names the seed. Illustration of a
shape:

```text
## Cluster: core :: 3. Names and references
- design@<anchor>@<id> | grade: | relation: | reason:
- issue@<anchor>@<id> | cites seed: design@<anchor>@<seed> | grade: | relation: | reason:
- design@<anchor>@<other> | graded above

## Met outside the task
```

### `worklist --verify`

`worklist --verify <DIR>` reads `agents.txt` and every `report.md`, with no model. A line's fields
are split on the first three ` | ` separators after the reference, and its reason is the rest of the
line, so a reason may hold `|`. It exits 1, naming each, for:

- an entry of an agent's list missing from its report, or listed twice;
- a reference that is not in that agent's list;
- a grade other than `bears`, `may-bear` or `unrelated`, or empty;
- `bears` with no relation, or with one outside its role's set; several relations are written
  comma-separated, each in the set;
- `may-bear` or `unrelated` with a relation;
- an empty reason for `bears` or `may-bear`;
- `unrelated` on an entry that names a seed, or on a seed itself;
- a report with no section `## Met outside the task`.

It exits 2 on `agents.txt` or a `report.md` it cannot parse or cannot find, naming it, per
`design@core@a-failed-parse-is-loud`, and 0 on complete reports. The grades:

| role | `bears` relations | the entry bears when the work |
| --- | --- | --- |
| `unit`, `goal` | `constrains`, `may change` | must respect it; or could contradict, widen, narrow or strain it |
| `rejected` | `may bring back` | could propose its design again |
| `issue` | `touches`, `closes`, `worsens`, `depends on`, `trigger met` | touches its subject; would close it; would make it worse; depends on its answer; would meet its deferred trigger |
| `tripwire` | `fires` | would meet its firing clause |

`may-bear` is an entry the searcher cannot rule out, with the doubt as its reason. The nearest
rival, a check of the report in the installed agent's text alone, lost: an instruction to list every
entry once is the one `searcher-reports-are-written-and-checked-by-hand` records as
unchecked.

### The searcher

`agent@knowledge-architect-record-searcher` takes the work, the seeds, and its agent's directory, which is
its scratch directory. It reads `entries.md` whole, judges every entry against the work leaning to
recall, and edits `report.md` in place; its tools are those of the two searchers plus `Edit`. Where
the harness refuses the edit, it returns the filled report as text, and the dispatcher writes it into
`report.md` before `worklist --verify`. Its description carries the dispatch: run
`{{command}} worklist --out <DIR>` with the seeds, send one agent per directory in parallel, run
`{{command}} worklist --verify <DIR>` when they return, send again an agent whose report fails it,
then read whole every entry graded as bearing, but at a retrospective intake, as
`design@agent-skills@search-relevance-grade` says there: an issue or a tripwire graded `may-bear` is
read whole, and a goal, a design entry or a `rejected` entry graded `may-bear` only where a proposal
touches its subject. It replaces both searchers. The point of
`standing-entry-search-agent` that the search stays apart from the
standing-state reviewer stays. The nearest rival, the two searchers kept, lost on
`argument@grounding-worklist@arg-36`: each would read half of every cluster.

### The scope

Every search reads the whole record: every cluster and every residual group, whatever the work. The
rival, clusters chosen by the work, lost on `argument@grounding-worklist@arg-23` and the owner's
`argument@grounding-worklist@arg-27`: choosing is the judgement step the work removes, and a choice
by title is the bad practice the owner names. A lighter pass by Component is parked,
`thread@grounding-worklist@title-tier-for-untouched-components`.

### The occasions

The search runs at design step 1, at the retrospective intake's grounding and at the design audit
of a slice or a spec, and again for a later thread, or a proposed fix at an intake, on a subject the
first work did not cover, with the thread or the fix as the work: the first search reads the whole
record but grades it against its own work. The session still reads the goals homes itself, per
`design@agent-skills@searcher-grades-goals`. Where an installed text asks for one line per entry a
search returned, as the design audit and design step 1 do, it means each entry graded as bearing.
The rival, the audit keeping the standing-entry search alone, lost on
`argument@grounding-worklist@arg-41`.

### The split standard

The split standard, `path@agent-skills@fragments/subject-split.md`, states: a subject holds the
heads a piece of work reads together, one part of the Component, such as a command, a register or an
activity; a subject whose cluster `worklist` names above the bound is split, and so is one whose
heads answer questions about parts that change apart; a split moves headings and decides nothing, so
it is a fix in a commit of its own, per `primer@met-outside-the-task` row 2; and a reader who judges
a subject runs `{{command}} worklist --out` into its scratch directory and reads the notices. Three
texts host it, per `design@agent-skills@shared-text-fragment`:
`skill@knowledge-architect-decision-recording@current-design`, the writer's;
`agent@knowledge-architect-decision-record-reviewer@design-head-rules`; and the design-record
auditor's rules, which `design@agent-skills@design-record-axis` lists, D2. Each judge reports a
subject the standard says to split. The nearest rival, a check finding above the bound, lost on
`argument@grounding-worklist@arg-37`.

## Mapping tables

**The role of each register in the worklist**, over every register the manifest can hold:

| register | role |
| --- | --- |
| `design` | unit register, by level-two subject |
| a declared heading register, entries at level 3 to 6 | unit register, by heading one level above its entries |
| a declared heading register, entries at level 2 | unit register, by file |
| a declared file register | unit register, by group subdirectory, and the top of its home |
| a declared Directory register | unit register, one unit per carrying anchor |
| `goal` | context: joins every cluster one of whose unit entries cites it, or cites a goal from which it is reached upward; else residual |
| `rejected` | joins every cluster one of whose unit entries cites it; else residual |
| `issue` | joins every cluster one of whose unit entries it cites; else residual |
| `tripwire` | as `issue` |
| `spec`, `milestone`, `thread`, `argument`, `criterion`, `acceptance` | none: plan documents are no nodes |

**Which text dispatches or names what, before and after:**

| site | today | after |
| --- | --- | --- |
| `skill@knowledge-architect-design@discussion-loop` step 1 | both searchers, and a second dispatch for a new subject | `worklist`, the searcher, `worklist --verify` |
| `skill@knowledge-architect-planning@working-a-slice` point 2 | the standing-entry searcher | the same as step 1 |
| `skill@knowledge-architect-retrospective-intake@ground-before-judging` item 3, its `%%` lines included | both searchers, and again per Component | the same as step 1, once |
| `skill@knowledge-architect-planning@spec-contents`, row "What is already decided" | "each design or `rejected` entry its design-entry search graded `bears`" | each entry the search graded as bearing |

## Losing alternatives

- **Goal-rooted clusters**, the owner's, round 0, and **leaf-goal clusters**, the owner's, round 1:
  lost to #cluster-unit on measured coverage, `argument@grounding-worklist@arg-15` and
  `argument@grounding-worklist@arg-30`.
- **Transitive membership over design → design edges**: lost to #cluster-unit,
  `argument@grounding-worklist@arg-10`.
- **One direction of citation, checked**: #citation-direction-rule, withdrawn by the owner,
  `argument@grounding-worklist@arg-19`.
- **Agent-facing text as nodes**: lost to #agent-text-in-graph, `argument@grounding-worklist@arg-20`.
- **Goals that pull in their citers**: lost to #goals-walked-upward,
  `argument@grounding-worklist@arg-31`.
- **One residual group for the project**, #residual-group: superseded by #residual-per-anchor, on
  the growth `argument@grounding-worklist@arg-52` measured.
- **Two searchers kept**: lost to #one-searcher-agent, `argument@grounding-worklist@arg-36`.
- **A bound in entries, then in bytes**: lost to #agent-packing-bound,
  `argument@grounding-worklist@arg-32` and `argument@grounding-worklist@arg-46`.
- **First-fit packing without spreading**: lost to the spreading, `argument@grounding-worklist@arg-61`,
  the owner, round 5.
- **A check finding above the bound**: lost to #split-standard, `argument@grounding-worklist@arg-37`.
- **The split standard delivered to its judges by a pointer**: lost to the fragment, per
  `design@agent-skills@shared-text-fragment`, on the design-conformance review of this spec.
- **Clusters chosen by the work, or by their title**: lost to #search-scope,
  `argument@grounding-worklist@arg-23` and `argument@grounding-worklist@arg-27`. A choice by
  Component touched is no loser: it is parked, #title-tier-for-untouched-components.
- **A smaller search on the in-change path**: #scope-by-path, withdrawn,
  `argument@grounding-worklist@arg-39`.
- **A delta search at convergence**: #search-delta, withdrawn, `argument@grounding-worklist@arg-56`.

## Readings

The work reads no external specification.

## Premortem

- **P1**, stresses #report-skeleton-command and #one-searcher-agent: `worklist --verify` passes on a
  report graded without real reading, every entry "unrelated" with a boilerplate reason. Named as a
  cause for the harvest to judge as a tripwire: `design-entry-search-misses`, now `tripwire@agent-skills@record-search-misses`
  is rewritten to guard the unified search and to fire on standing entries too, and
  `tripwire@agent-skills@goal-missed-though-graded` to follow the reading of goals as context.
- **P2**, stresses #split-standard and #agent-packing-bound: notices are ignored, subjects stay
  large, and an agent reading 55,000 words misses entries. Named as a cause; the same tripwire as P1
  sees it.
- **P3**, stresses #search-scope: the record grows past what sessions will pay to read, and sessions
  skip the search. Named as a cause; the parked issue of #title-tier-for-untouched-components holds
  its answer.
- **P4**, stresses #goals-walked-upward and #residual-per-anchor: a standing entry citing only a goal
  is read apart from the heads serving that goal, and misjudged. Named as a cause.
- **P5**, stresses #entry-graph: a pointer written as plain text drops an edge, and its entry lands in
  a residual group, read without its neighbours. Named as a cause; already watched by
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`.
- **AC1 to AC4**: became acceptance criteria on the owner's word, round 3: "keep all 4 proposed AC".

## Acceptance criteria

### AC1. `worklist --verify` names each gap of a report `##verify-names-each-gap`

Guards `thread@grounding-worklist@report-skeleton-command`. Judged at step 3. Fires when a test over a
filled directory shows `worklist --verify` exiting 0, or not naming the entry, for any of: an entry
missing from a report, an entry listed twice, a grade outside the set, an entry not in that agent's
list; or not exiting 2 on a report it cannot parse; or not exiting 0 on a complete one. Response:
reopen #report-skeleton-command.

### AC2. The agents' lists together hold every entry of the record `##union-is-every-entry`

Guards `thread@grounding-worklist@subject-unit-both-shapes`, `thread@grounding-worklist@declared-register-units`
and `thread@grounding-worklist@residual-per-anchor`. Judged at step 2. Fires when, over a project a
test writes that holds a file home and a directory home, heads under no level-two subject, a
declared file register with groups, a chain of goals, one refining another that refines a third,
reached from a head that cites the first, and an issue citing only a goal, the union of the lists in
`agents.txt` is not every entry of the record, or the chain's three goals are not in the head's
cluster, or that issue is not in its anchor's residual group. Response: reopen the three threads.

### AC3. A cluster above the bound goes alone, and is named `##oversized-cluster-alone`

Guards `thread@grounding-worklist@agent-packing-bound`. Judged at step 2. Fires when a test's
cluster above `--bound` shares its agent with another cluster, or the output does not print its
notice. Response: reopen #agent-packing-bound.

### AC4. `worklist` refuses over an incomplete model `##refused-over-an-incomplete-model`

Guards `thread@grounding-worklist@report-skeleton-command`. Judged at step 2. Fires when
`worklist --out` over a tree that stops before the last phase exits other than 2, or writes any
file. The test plants a phase-2 finding. Response: reopen #report-skeleton-command.

## Implementation sequence

0. **The reading owed.** Read whole the `may-bear` entries the grounding judged by title, as "What
   is already decided" says, and record each outcome in the message of step 1's first commit.
1. **The graph, with no command.** Builds `records::edges` and `path@core@src/graph.rs`: units,
   clusters, residual groups and the packing, as pure functions of the model and the entity table,
   with unit tests over models written as text, as the tests of `path@core@src/records.rs` are.
   Fails alone on: a membership or a packing the unit tests name. Claims:
   - `edges` returns each pair once per citing section, and no pair from a home's prose outside an
     entry; refuted by a test whose reference sits under a subject before its first head;
   - every entry of a unit register is in exactly one unit, for each row of the role table; refuted
     by a test whose entry lands in none or two;
   - a goal joins a cluster through a citation and upward through refinement, and pulls in no issue;
     refuted by a test whose issue cites only a goal and joins the cluster;
   - every entry of the record is in a cluster or in its anchor's residual group;
   - the packing gives the number of agents first-fit-decreasing gives, sends a cluster above the
     bound alone, spreads the rest under the bound, and gives one result over one input; refuted by a
     test with clusters of known sizes and their expected agents and loads.

   Each test is shown to fail by mutating its rule in a scratch worktree, a goal walked downward
   instead of upward, for instance.
2. **`worklist --out`.** Builds the variant, its arguments and the files. Fails alone on: the files
   written or a refusal. Claims: AC2, AC3 and AC4, by tests in `path@core@tests/binary.rs` that
   write their own project, as `History` does; the refusals of a `<DIR>` inside the project or not
   empty, and of a seed that names nothing; `path@core@README.md` gains a section on `worklist` and
   its outcomes in the three rows of "Exit codes".
3. **`worklist --verify`.** Fails alone on: the verdicts. Claims: AC1 and every case of the list
   under Decided design, by tests over directories step 2 wrote and then edited.
4. **The installed text and the record, in one commit**, since removing the two searchers dangles
   every reference to them until the record that cites them is rewritten, and a tree that fails the
   tip's checker lands in the commit that makes it pass, per `primer@branch-commits`:
   - creates `path@agent-skills@content/agents/record-searcher.md` and
     `path@agent-skills@fragments/subject-split.md`, and removes the two searchers' files;
   - rewrites the four texts of the second mapping table and hosts the fragment in the three texts
     of Decided design, "The split standard";
   - writes the harvest, every row;
   - writes the released sections' references to the two searchers back as bare names, and runs
     `cargo x changelog`;
   - writes the `Next release` entries, judged under `design@knowledge-architect@changelog-entries`
     and `design@knowledge-architect@versioning-policy`, read whole: at least New features, `cli`,
     for `worklist`; Workflow, `agent-skills`, for the one searcher and the split notice; and
     Migration, `agent-skills`, for a project whose own text cites a removed searcher by its
     reference;
   - runs `cargo klarch install-agent-skills`.

   Written under `skill@knowledge-architect-agent-configuration`, as `path@agent-skills@CLAUDE.md`,
   "Editing an installed skill or agent", says. Fails alone on: the check, the build, or a text
   naming an agent that no longer exists.
5. **Measure.** Runs this checkout's `klarch worklist --out` over this repository and over thaum,
   from thaum's root with this checkout's release binary, which thaum's pin accepts while both are
   at one version; where they differ, the run in thaum is skipped and the commit says so. Records the
   agents, their loads and the notices in the message of a commit that changes a file, such as the
   report on the acceptance criteria.

## Order rationale

1. Step 0 before step 1: the reading may change the design, and nothing is built yet.
2. Step 1 before step 2: the command writes what the graph computes, so the graph is proved alone.
3. Step 2 before step 3: verify reads the files step 2 writes.
4. Step 3 before step 4: the searcher's text names both commands, which exist then.
5. Step 4 before step 5: the measurement is of what is built and installed.

## Defaults awaiting the owner

- **D1**, #entry-graph: plan documents are no nodes, and every reference between entries is an edge.
  Ruled, round 5: "D1 agreed, plan doc sit outside the search".
- **D2**, #split-standard: "design audits" in the owner's word of round 2 is read as the
  design-record axis of the project audit, which the design-record auditor carries; the design audit
  of a slice gains no check. Ruled, round 5: "D2 correct reading".
- **D3**, #one-searcher-agent: the searcher is named `agent@knowledge-architect-record-searcher`, its file
  `record-searcher.md`. Ruled, round 5: "D3 D4 accepted."
- **D4**, #report-skeleton-command: `--seed <REF>`, repeatable, makes each skeleton line of an entry
  that cites a seed name it, so the searcher does not map seeds through `show`. Ruled, round 5: "D3
  D4 accepted."
- **D5**, #title-tier-for-untouched-components: the issue the harvest opens keeps the leads
  `argument@grounding-worklist@arg-28`, `argument@grounding-worklist@arg-40`,
  `argument@grounding-worklist@arg-48` and `argument@grounding-worklist@arg-65`, and no other.
  Ruled, round 7: "D5 accepted, proceed".
- **D6**, #search-delta: `the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`
  waits, on the owner's word, for the discussion of
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`. The work
  corrected its mechanism and argues both instances in
  `design@agent-skills@record-searched-before-the-work`, which its closing condition names. Whether
  the owner's word of round 2, "I think our mechanical tooling is the correct fix", closes it now is
  the owner's: the entry stays open until the owner rules. Ruled, round 8: "d6: it closes".

## Harvest

All in the commit of step 4, then repaired by the review before the merge. What landed:

| what | where |
| --- | --- |
| the record and its edges, the units and the clusters, the goals as context, the residual, the absence of a direction rule, `worklist` with `--verify`, and the packing under the bound | `register@core@design`: `design@core@record-is-every-register-entry`, `design@core@cluster-rooted-at-a-subject` (#cluster-unit), `design@core@goals-join-a-cluster-as-context` (#goals-walked-upward), `design@core@residual-grouped-by-anchor` (#residual-per-anchor), `design@core@no-citation-direction-rule` (#citation-direction-rule), `design@core@worklist-writes-and-verifies-the-reports` (#report-skeleton-command), `design@core@clusters-packed-under-a-word-bound` (#agent-packing-bound). The two incumbent heads on groups, `design-entry-search-groups` and `standing-entry-search-groups`, leave; their decision is the command's output, an interface of the core, by test 3 of `skill@knowledge-architect-decision-recording@owning-component` |
| the one searcher, apart from the standing-state review, the whole record, the occasions | `design@agent-skills@one-record-searcher` (#one-searcher-agent), `design@agent-skills@search-apart-from-the-standing-state-review`, `design@agent-skills@search-reads-the-whole-record` (#search-scope), `design@agent-skills@record-searched-before-the-work` (#search-occasions), which replace `design-entry-search`, `standing-entry-search-agent` and `standing-entries-searched-before-the-work` |
| the grades for every role, and goals as context | `design@agent-skills@search-relevance-grade`, `design@agent-skills@searcher-grades-goals`, rewritten |
| the split standard and its three hosts | `design@agent-skills@subject-split-standard` (#split-standard); `design@agent-skills@design-record-axis` gains it in its rules |
| the losing alternatives | `register@core@rejected` and `register@agent-skills@rejected`, as the tests of `skill@knowledge-architect-decision-recording@losing-alternatives` admit |
| the bound of 60 the design-record axis borrows | `design@agent-skills@design-record-axis-dispatch` |
| the rival's task named in `design@agent-skills@a-scratch-directory-per-subagent` | rewritten |
| P1 to P5 | judged as `skill@knowledge-architect-decision-recording@ruled-tripwires` says; `tripwire@agent-skills@record-search-misses` guards the record searcher |
| the issues | `searcher-reports-are-written-and-checked-by-hand`, `the-standing-entry-searcher-has-no-list-for-a-doubt` and `the-design-entry-search-does-not-run-at-a-design-audit` closed; `the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work` closed on the owner's word, D6, its mechanism corrected first; `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` rewritten; `issue@agent-skills@audit-axes-beyond-the-design-record` gains the sample of heads citing no goal; `issue@agent-skills@a-lighter-pass-for-untouched-components` opened, per D5 |

## Later consequences

- The bound may rise to 30,000 words, the owner's word of round 3: "I'm even considering raising it
  a bit more, maybe 30000. but that can wait for a few iterations." The head on the bound carries
  that.
- The "goal coverage" axis of `issue@agent-skills@audit-axes-beyond-the-design-record` may read the
  graph `worklist` builds; nothing is built for it now.
- The lighter pass for untouched Components waits in its new issue.
