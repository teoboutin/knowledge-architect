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
- one installed agent, `knowledge-architect-record-searcher`, replaces
  `agent@knowledge-architect-standing-entry-searcher` and
  `agent@knowledge-architect-design-entry-searcher`;
- the design skill's step 1, the planning skill's design audit and the retrospective intake's
  grounding dispatch that one search;
- the decision-recording skill states when a subject of a design home is split, and the
  decision-record reviewer and the design-record auditor judge against it.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits: the defaults D1 to D4 are built as written, and the branch merges once the owner has ruled
  on each.
- **The work lands as this one spec and a single pull request, on the owner's word**, round 3:
  "This work should land as one spec and a single PR." No commit of the work changes what the
  per-commit gate checks: `worklist` is a command, not a check, so the spec needs no pull request of
  its own before its work, per `skill@knowledge-architect-planning@plan-reviews`.
- It was assembled from one transcript. The discussion begins at the owner's message that opens "I
  want to refactor the way the grounding steps at the beginning of design skill, design audit of
  slices, and other triggers." and ends at the owner's message that ends "keep all 4 proposed AC".
  It holds 4 owner messages, called rounds 0 to 3 below, and the agent's replies to rounds 0, 1 and
  2, which the transcript heads "Round 1", "Round 2" and "Round 3": each is called here "the reply
  to round 0", "the reply to round 1" and "the reply to round 2". Find the file by that opening
  message, not by its name.

## How the work is done

By `skill@knowledge-architect-planning@working-a-slice`, as its last paragraph says for the work of
a spec.

## Names

| name | what it names |
| --- | --- |
| the record | every entry of the registers the graph holds: `goal`, `design`, `rejected`, `issue`, `tripwire`, and every register a project declares, in every anchor. Plan documents and their items are not in it |
| entry | one entry of one of those registers, as `path@core@src/records.rs` reads it into a `Record`: a heading entry's section, a file entry's file |
| edge | a reference from inside one entry's body to another entry, read with the candidate rule `inbound` and `cited_within` of `path@core@src/records.rs` use |
| unit register | every register of the record but `goal`, `rejected`, `issue` and `tripwire`: `design` and every declared register |
| unit | the entries of a unit register grouped as #subject-unit-both-shapes and #declared-register-units say: a level-two subject of a heading register's home, or a group of a file register |
| cluster | a unit, with the `rejected` entries its entries cite, the issues and tripwires that cite any of its entries, and the goals its entries cite with every goal those refine |
| residual group | the entries of one anchor that no cluster holds |
| agent | one dispatch of the searcher, given whole clusters and residual groups up to the bound |
| the bound | the number of words one agent is given at most, 25,000 by default |
| words | the count of whitespace-separated tokens in an entry's body, `Record.body` |
| the two searchers | `agent@knowledge-architect-design-entry-searcher` and `agent@knowledge-architect-standing-entry-searcher` |
| the searcher | `knowledge-architect-record-searcher`, the agent the work creates |
| the design audit | point 2 of `skill@knowledge-architect-planning@working-a-slice` |
| this repository's graph scripts | the throwaway scripts of the discussion: `build.py` through `build4.py`, `analyze.py`, `analyze2.py`, `subjects.py`, `cluster.py`, `cluster2.py`, `split.py`, `residual.py`, `residual4.py` and `pack.py`, kept in the session's scratch directory and in no tree; their results are the measurements this spec quotes, and the work's own tests replace them |

## What the work is

What exists today at each site the work touches:

- **The core's commands** are the variants of `Command` in `path@core@src/cli/mod.rs`, dispatched
  in the same file: `show`, `issues`, `tripwires`, `model`, `anchors` and the rest. None computes a
  work list of entries: `show` prints one entry and every site that references it, as file and line
  (`show` in `path@core@src/cli/mod.rs`); `issues` and `tripwires` print one row per entry.
- **The core already reads entries and edges.** `path@core@src/records.rs` holds `Record`, with its
  `body`, its `site` and its `group`, the group subdirectory of a file entry; `records` lists every
  entry of one kind; `inbound` lists every site that references one entity; `cited_within` maps a
  reference to the entry whose section holds it, for the orphan check `orphans` of
  `path@core@src/check/references.rs`, per `design@core@orphan-rejected-entry`.
- **The two searchers** are `path@agent-skills@content/agents/standing-entry-searcher.md` and
  `path@agent-skills@content/agents/design-entry-searcher.md`. The first takes consecutive positions
  of the combined, sorted rows of `issues` and `tripwires`, in groups of 60 or fewer, and derives
  its group itself by a sort its body states. The second takes one Component, or a group of its
  level-two subjects that the session cuts by judgement. Both write their report by hand; nothing
  checks it.
- **The dispatching texts**: the design skill's loop step 1,
  `skill@knowledge-architect-design@discussion-loop`; the design audit,
  `skill@knowledge-architect-planning@working-a-slice` point 2, which sends the standing-entry
  searcher alone; the retrospective intake's grounding,
  `skill@knowledge-architect-retrospective-intake@ground-before-judging` item 3, with its `%%`
  comment lines naming three heads.
- **No installed text states when a subject of a design home is split.**
  `skill@knowledge-architect-decision-recording@current-design` says only that a decision goes in
  the subdocument owning its subject. The design-entry searcher's
  `agent@knowledge-architect-design-entry-searcher@grade-each-entry` asks it to report a subject
  whose size strained its reading.

**What the record holds today**, measured by this repository's graph scripts over `cargo klarch
model`, at the commit this spec is written on (this repository) and at thaum's working tree on the
same day:

| measurement | here | thaum |
| --- | --- | --- |
| entries: goal / design / rejected / issue / tripwire / interpretation | 25 / 346 / 163 / 100 / 43 / none | 8 / 297 / 189 / 177 / 130 / 69 |
| heads citing a goal directly | 133 of 346 | 47 of 297 |
| largest set of heads joined by design → design citations, a strongly connected component | 159 | 58 |
| subject-rooted clusters, goals as context: clusters; entries reached of all; reads ÷ distinct | 23; 663 of 677; 839 ÷ 663 | 28 (interpretations not counted); 706 of 801; 802 ÷ 706 |
| residual entries, every register counted | 14 | 55 |
| agents at a bound of 25,000 words, spread packing: agents; loads | 9, the lower bound; 22,907 to 23,409 words | 11: 8, the lower bound for the clusters within the bound, at 23,952 to 24,195 words, and 3 clusters above it sent alone, of 55,103, 27,217 and 26,424 words |

**Outside the work**, each with what owns it:

- **The search at convergence** of
  `issue@agent-skills@the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`:
  this work closes that issue on other grounds, per #search-delta's withdrawal below.
- **A lighter pass for Components a piece of work does not touch**, #title-tier-for-untouched-components:
  parked as a new issue, written at the harvest.
- **A rule that every head cites a goal**, or a check of citation direction: ruled out or withdrawn
  in the discussion, under Losing alternatives.
- **The standing-state review before every merge**, `design@agent-skills@standing-state-review-before-every-merge`:
  unchanged. It reads every tripwire and deferred trigger by rule, where the search reads by
  relevance.
- **The dispatch of the design-record axis and of the tripwire axis of the project audit**,
  `design@agent-skills@design-record-axis-dispatch`: unchanged, except the sentence that borrows the
  bound of 60 from `design@agent-skills@standing-entry-search-groups`, which the harvest rewrites
  to stand on its own measurement.
- **The record check of an audit repair**, `design@agent-skills@repairs-read-against-the-record`:
  unchanged; it is no grounding search.
- **The other work lists** of `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
  a `show` over a changed range and the enclosing entry of each citing site: outside the work. The
  issue is rewritten at the harvest to drop the searcher's use, which this work removes.

## What is already decided

**Decisions the design rests on and does not argue again:**

- `goal@knowledge-architect@agents-work-without-drift`, `goal@knowledge-architect@structure-and-workflow-work-together`,
  `goal@core@records-reach-their-reader`, `goal@core@declared-instructions-are-checked`,
  `goal@agent-skills@installed-text-works-anywhere`, `goal@agent-skills@no-contradiction-in-workflow`,
  `goal@agent-skills@installed-text-leaves-room-to-judge`.
- `design@agent-skills@rejected-citation-direction` and `design@core@orphan-rejected-entry`: a
  head cites its `rejected` entries; the owner chose not to reopen it, round 1.
- `design@agent-skills@a-reference-claims-a-revisit`: the edges are the references this rule has
  sessions write.
- `design@core@one-entity-table`: the graph reads the entity table, and resolves no name of its own.
- `design@core@phases-gate-the-report`: `worklist` writes files, so it is a writer, and refuses
  over an incomplete model.
- `design@core@exit-code-ladder`, `design@core@arguments-parse-through-clap`,
  `design@core@the-core-cli-is-a-library-module`, `design@core@one-question-one-command`,
  `design@core@a-failed-parse-is-loud`, `design@core@nothing-of-a-project-is-compiled-in`.
- `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@a-scratch-directory-per-subagent`,
  `design@agent-skills@capabilities-not-structure`, `design@agent-skills@standing-state-review-before-every-merge`.
- `design@agent-skills@bounded-path-in-design`: the owner's argument that scope is known only after
  grounding, which defeats #scope-by-path.

**Decisions the work reverses or rewrites, with every text `cargo klarch show` lists as citing
each, and the step or harvest row that updates it:**

| decision | citing texts | updated by |
| --- | --- | --- |
| `design@agent-skills@design-entry-search` | the intake skill's `%%` line, `path@agent-skills@content/skills/retrospective-intake/SKILL.md` line 61; heads at `register@agent-skills@design` lines 1347 and 1383; `issue@agent-skills@the-design-entry-search-does-not-run-at-a-design-audit`; `tripwire@agent-skills@design-entry-search-misses` (lines 440, 450); `issue@core@searcher-reports-are-written-and-checked-by-hand` (lines 25, 46) | step 4 (the `%%` line), the harvest (heads, issues, tripwire) |
| `design@agent-skills@design-entry-search-groups` | heads at line 973; `rejected@agent-skills@design-listing-cut-in-groups-of-sixty` and `rejected@agent-skills@subject-groups-bounded-at-sixty` (rejected home lines 490, 498) | the harvest |
| `design@agent-skills@standing-entry-search-agent` | the intake's `%%` line 59; head at line 980; `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` line 28; `issue@core@searcher-reports-are-written-and-checked-by-hand` line 43 | step 4, the harvest |
| `design@agent-skills@standing-entry-search-groups` | the intake's `%%` line 60; heads at lines 1019, 1292 and 2487 (`design@agent-skills@design-record-axis-dispatch`); `rejected@agent-skills@one-search-agent-per-anchor`'s neighbour at rejected line 496 | step 4, the harvest |
| `design@agent-skills@standing-entries-searched-before-the-work` | head at line 1996; `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` (lines 79, 96); `issue@agent-skills@the-design-entry-search-does-not-run-at-a-design-audit` line 17; `issue@agent-skills@the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work` (lines 10, 57, 64, 72); `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` | the harvest |
| `design@agent-skills@search-relevance-grade` | head at line 972; `issue@agent-skills@the-standing-entry-searcher-has-no-list-for-a-doubt` line 24 | the harvest |
| `design@agent-skills@searcher-grades-goals` | head at line 1030; `tripwire@agent-skills@goal-missed-though-graded` (lines 458, 466) | the harvest |
| `design@agent-skills@a-scratch-directory-per-subagent` | none | the harvest: its sentence naming the standing-entry searcher's description and its rival's task, "sorts a listing and runs one command per entry" |

**What the discussion's searches returned, with each outcome.** The grounding file of round 0
listed them; each bearing entry is above or below.

- The heads and goals above: bear, as listed.
- `rejected@agent-skills@one-search-agent-per-anchor`: brought back in part by
  #residual-per-anchor. Its reason, "An anchor bounds nothing", is defeated by the packing and the
  notice of #agent-packing-bound and #split-standard; argued under Decided design.
- `rejected@agent-skills@one-agent-reads-every-standing-entry`, `rejected@agent-skills@design-listing-cut-in-groups-of-sixty`,
  `rejected@agent-skills@subject-groups-bounded-at-sixty`: stay `live`. The packing bound is no
  bound on a subject, so none is brought back.
- `rejected@agent-skills@entry-names-the-decision-it-lost-to`, `rejected@agent-skills@both-directions-written-by-hand`,
  `rejected@agent-skills@both-directions-checked-for-symmetry`, `rejected@core@entry-reachable-by-citing-a-goal`,
  `rejected@core@no-orphan-check`, `rejected@core@goal-citing-a-rejected-entry`: not brought back;
  #citation-direction-rule was withdrawn.
- `issue@core@searcher-reports-are-written-and-checked-by-hand`: closed by steps 2 and 3.
- `issue@agent-skills@the-standing-entry-searcher-has-no-list-for-a-doubt`: closed by the uniform
  grades of step 4.
- `issue@agent-skills@the-design-entry-search-does-not-run-at-a-design-audit`: closed by
  #search-occasions.
- `issue@agent-skills@the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`:
  closed at the harvest, its mechanism corrected, per #search-delta.
- `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`: narrowed at the
  harvest, as under What the work is.
- `issue@agent-skills@a-general-purpose-subagent-is-refused-a-file-access`: bears on who writes the
  report. The searcher is an installed agent, which no recorded instance refuses.
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` and
  `tripwire@core@candidate-rule-silence`: bear as premortem cause P5; no change.
- `issue@agent-skills@audit-axes-beyond-the-design-record`: its "goal coverage" lead may later
  read the graph; no change now.
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: its citation of
  `design@agent-skills@standing-entries-searched-before-the-work` is re-read at the harvest; the
  search still reads no pending file.
- `issue@core@a-component-states-at-least-one-goal`: a Component with no goal still has its heads
  in clusters, since a unit is a subject, not a goal.
- `issue@core@core-heads-bundle-decisions-with-many-citing-sites`, `issue@core@references-are-read-in-markdown-and-rust-only`,
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`: bear on nothing the work
  builds. The skeleton's format is read back only by `worklist --verify`, so no consumer depends on
  it.
- `tripwire@agent-skills@design-entry-search-misses` and `tripwire@agent-skills@goal-missed-though-graded`:
  rewritten at the harvest, premortem P1.
- `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head`: unchanged.
- `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`: re-read at the harvest; its
  premise, no search before work that bypasses the design skill, stays true.
- `tripwire@core@a-second-producer-chain`: not met; `worklist` is a command and no check reads its
  output.
- Every other entry the searches returned bears on nothing the work builds.

## Criteria

### Every entry the work bears on reaches a reader who reads it whole `##every-bearing-entry-read`

Binding, derived from `goal@knowledge-architect@agents-work-without-drift`, and from the occasions
of `design@agent-skills@standing-entries-searched-before-the-work`, which "should miss nothing".
Met by `thread@grounding-worklist@search-scope`, `thread@grounding-worklist@residual-per-anchor`,
`thread@grounding-worklist@subject-unit-both-shapes` and
`thread@grounding-worklist@declared-register-units`.

### The work list and the coverage of the reports are computed by the tool, never by the agent `##tool-computes-the-list`

Binding, derived from `goal@core@records-reach-their-reader` and
`goal@knowledge-architect@structure-and-workflow-work-together`. Met by
`thread@grounding-worklist@report-skeleton-command` and `thread@grounding-worklist@entry-graph`.

### It works in any adopting project from its manifest alone `##any-project-from-its-manifest`

Binding, derived from `goal@agent-skills@installed-text-works-anywhere` and
`design@core@nothing-of-a-project-is-compiled-in`: a project with few goal citations, Components
with no goal (8 of 12 in thaum, per `issue@core@a-component-states-at-least-one-goal`), both shapes
of a design home, and declared registers. Met by `thread@grounding-worklist@subject-unit-both-shapes`
and `thread@grounding-worklist@declared-register-units`.

### A head is read together with the entries that cite it `##head-read-with-its-citers`

Binding as a presumption, derived from the owner's reason recorded in
`design@agent-skills@design-entry-search-groups`, that a subject's kept and rejected entries have
to be read together. Met by `thread@grounding-worklist@one-searcher-agent` and
`thread@grounding-worklist@goals-walked-upward`.

### Each agent's load is bounded whatever the project's size `##bounded-agent-load`

Binding as a presumption, derived from `design@agent-skills@standing-entry-search-groups`. Met by
`thread@grounding-worklist@agent-packing-bound`, except a cluster above the bound, which goes alone
with the notice of `thread@grounding-worklist@split-standard`.

### The overlap, the entries read more than once `##overlap-cost`

Weighed. 1.27× here and 1.14× in thaum, per `argument@grounding-worklist@arg-31`.

## Threads

### The graph's edges are every reference between entries, and plan documents are no nodes `##entry-graph`

Proposed by the owner, round 0; shape by the agent, the reply to round 0. State:
`presumed-settled`, awaiting the owner's word as D1. Arguments: `argument@grounding-worklist@arg-1`,
`argument@grounding-worklist@arg-13`. Shape: Decided design, "The graph". Harvest: the core's head
on `worklist`.

### A cluster is rooted at a subject of a unit register, not at a goal `##cluster-unit`

Proposed by the owner as goal-rooted, round 0; the subject-rooted rival by the agent, the reply to
round 0; the leaf-goal shape by the owner, round 1. State: `argued`. Every goal-rooted shape was
ruled out on measured coverage, `argument@grounding-worklist@arg-30`: "The leaf shape loses for the
same reason the first goal-rooted shape did: 213 heads here cite no goal." The owner, round 1:
"your subject rooted approach does look better than my goal rooted one." Arguments: `argument@grounding-worklist@arg-3`, `argument@grounding-worklist@arg-5`,
`argument@grounding-worklist@arg-10` to `argument@grounding-worklist@arg-12`, `argument@grounding-worklist@arg-14` to `argument@grounding-worklist@arg-18`, `argument@grounding-worklist@arg-25`, `argument@grounding-worklist@arg-30`. Shape: Decided design, "The units". Harvest:
the agent-skills head on the search's clusters; the losing shapes, `rejected` entries if the tests
admit them.

### The unit is a level-two subject of a file home or of each subdocument of a directory home `##subject-unit-both-shapes`

Raised by the owner, round 1: "it has to account for both shapes of the design homes, which can be
either one document, or a directory with multiple documents." Shape by the agent, the reply to
round 1. State: `approved`, the owner, round 2: "subject-unit-both-shapes: Agreed on that shape".
Arguments: `argument@grounding-worklist@arg-26`, `argument@grounding-worklist@arg-34`. Shape: Decided design, "The units". Harvest: the core's head on
`worklist`.

### Goals enter a cluster as context, walked upward, and pull in no standing entry `##goals-walked-upward`

Raised by the owner, round 1: "Also, goals have to be included in clusters by walking upward." The
limit by the agent, the reply to round 1. State: `approved`, the owner, round 2:
"goals-walked-upward: agreed, do not pull in standing entries". Arguments: `argument@grounding-worklist@arg-24`, `argument@grounding-worklist@arg-26`, `argument@grounding-worklist@arg-31`,
`argument@grounding-worklist@arg-35`. Shape: Decided design, "The clusters". Harvest: the core's head on `worklist`.

### The residual is grouped by anchor, and packed like a cluster `##residual-per-anchor`

Proposed by the agent, the reply to round 2, replacing #residual-group, the single residual group of
the reply to round 1, which it absorbs. State: `approved`, the owner, round 3: "Agreed on
residual-per-anchor and declared-register-units." Arguments: `argument@grounding-worklist@arg-35`, `argument@grounding-worklist@arg-44`, `argument@grounding-worklist@arg-51` to `argument@grounding-worklist@arg-53`.
Shape: Decided design, "The residual". Harvest: the core's head on `worklist`.

### Every declared register is a unit register, grouped by its subjects or its groups `##declared-register-units`

Proposed by the agent, the reply to round 2. State: `approved`, the owner, round 3, as above.
Arguments: `argument@grounding-worklist@arg-51`, `argument@grounding-worklist@arg-54`. Shape: Decided design, "The units". Harvest: the core's head on
`worklist`.

### One installed searcher replaces the two `##one-searcher-agent`

Proposed by the agent, the reply to round 1. State: `approved`, the owner, round 2:
"one-searcher-agent: agreed on this, the separation no longer makes sense with the new process".
Arguments: `argument@grounding-worklist@arg-36`. Shape: Decided design, "The searcher". Harvest: the agent-skills heads on the
search, rewritten.

### `worklist` writes the agents' entries and report skeletons, and `worklist --verify` checks the filled reports `##report-skeleton-command`

Proposed by the owner, round 0; shape by the agent, the reply to round 0. State: `approved`, the
owner, round 1: "report-skeleton-command: agreed on shape, with the searcher agent editing the
report if possible." Arguments: `argument@grounding-worklist@arg-1`, `argument@grounding-worklist@arg-7`, `argument@grounding-worklist@arg-21`, `argument@grounding-worklist@arg-43`, `argument@grounding-worklist@arg-57`. Shape: Decided design,
"`worklist`" and "`worklist --verify`". Harvest: the core's head on `worklist`; the agent-skills
head on the grades.

### Clusters are packed into agents under a bound of 25,000 words `##agent-packing-bound`

Proposed by the owner as a bound on total entries, round 0; in bytes, then in words, by the agent
and the owner, the replies to rounds 1 and 2. State: `approved`, the owner, round 3:
"agent-packing-bound: put the bound at 25000, keep the measurements and arguments in the design
record for this. My argument is that since we pack clusters, the actual load of a single agent will
rarely reach the bound, instead sitting a little lower. I'm even considering raising it a bit more,
maybe 30000. but that can wait for a few iterations." The owner's premise was measured at the
assembly, `argument@grounding-worklist@arg-61`: it holds under the balanced packing of Decided
design, not under first-fit. Arguments: `argument@grounding-worklist@arg-2`, `argument@grounding-worklist@arg-22`, `argument@grounding-worklist@arg-32`, `argument@grounding-worklist@arg-38`, `argument@grounding-worklist@arg-46`, `argument@grounding-worklist@arg-55`, `argument@grounding-worklist@arg-60`,
`argument@grounding-worklist@arg-61`. Shape: Decided design, "The packing". Harvest: the agent-skills head on the search's
clusters, with the measurements and the owner's argument, on the owner's word.

### A subject above the bound is named by `worklist`, split as a fix of its own, and the split standard lives in the decision-recording skill `##split-standard`

Raised by the owner, round 1: "we should provide a standard of how to split meaningfully and strong
incentive to split early." Shape by the agent, the reply to round 1. State: `approved`, the owner,
round 2: "split-standard: agreed on that." and, on Q2: "I think the incentive is right, with the
action carried as a fix in a commit of its own. I think the design record skill and reviewer and
design audits should also be looking for those, so the splits happen early." The agent's reading of
"design audits" is D2. Arguments: `argument@grounding-worklist@arg-26`, `argument@grounding-worklist@arg-33`, `argument@grounding-worklist@arg-37`, `argument@grounding-worklist@arg-47`, `argument@grounding-worklist@arg-58`. Shape: Decided design,
"The split standard". Harvest: a new agent-skills head.

### The whole record is searched every time `##search-scope`

Proposed by the agent as Q1's default, the reply to round 0. State: `approved`, the owner, round 1:
"overall I believe the whole record is safer. Filtering based on the subject title of a cluster is
bad practice here IMO. The filtering can only be properly done based on the results from the search
I believe." Arguments: `argument@grounding-worklist@arg-23`, `argument@grounding-worklist@arg-27`. Shape: Decided design, "The occasions". Harvest: the
agent-skills head on the search.

### The search runs at design step 1, at the intake and at the design audit of a slice or a spec `##search-occasions`

Proposed by the agent, the reply to round 1. State: `approved`, the owner, round 2:
"search-occasions: agreed, this unifies the procedures". Arguments: `argument@grounding-worklist@arg-41`, `argument@grounding-worklist@arg-49`. Shape: Decided
design, "The occasions". Harvest: `design@agent-skills@standing-entries-searched-before-the-work`
and `design@agent-skills@design-entry-search`, rewritten.

### A lighter, title-only pass for Components the work does not touch `##title-tier-for-untouched-components`

Proposed by the owner, round 1. State: `parked`, the owner, round 2: "title-tier-for-untouched-components:
agreed to park. given your arguments, I would now argue for title reading of design + full read of
standing state. We can keep the leads on this thread in an open issue." Cause: a whole-record
worklist in a real project needs more agents than the owner accepts for one grounding. Re-entry:
the first design discussion in the owner's planned project of about 30 Components, or a
retrospective that reports the search's cost. Arguments: `argument@grounding-worklist@arg-28`, `argument@grounding-worklist@arg-40`, `argument@grounding-worklist@arg-48`. Harvest: a new
issue of kind `design` in `register@agent-skills@issue`, holding the owner's current position, the
cause and the re-entry.

### The citations follow one direction, checked by the checker `##citation-direction-rule`

Proposed by the owner, round 0. State: `withdrawn`, the owner, round 1: "citation-direction-rule:
you convinced me, I withdraw." Defeating reason: clustering needs no direction, and the hierarchy
would reverse `design@agent-skills@rejected-citation-direction` and forbid goal refinement and
issue dependencies, `argument@grounding-worklist@arg-8`, `argument@grounding-worklist@arg-9`, `argument@grounding-worklist@arg-19`. Arguments: `argument@grounding-worklist@arg-4`, `argument@grounding-worklist@arg-8`, `argument@grounding-worklist@arg-9`, `argument@grounding-worklist@arg-19`, `argument@grounding-worklist@arg-24`. Harvest:
a `rejected` entry if the tests admit it.

### Agent-facing text is a node of the graph `##agent-text-in-graph`

Raised by the owner, round 0. State: `approved` as excluded, the owner, round 1: "agent-text-in-graph:
agreed to exclude". Arguments: `argument@grounding-worklist@arg-6`, `argument@grounding-worklist@arg-20`. Shape: Decided design, "The graph".

### The in-change path searches less `##scope-by-path`

Proposed by the owner, round 1, with its doubt: "But I do not think this is a very sound approach,
because scope of change is hard to determine upfront." State: `withdrawn` by its proposer.
Defeating reason: scope is known only after grounding, the owner's own argument in
`design@agent-skills@bounded-path-in-design`, `argument@grounding-worklist@arg-39`. Arguments: `argument@grounding-worklist@arg-29`, `argument@grounding-worklist@arg-39`.

### A second search at convergence lists only entries changed since the first `##search-delta`

Proposed by the agent, the reply to round 1. State: `withdrawn` by its proposer, the reply to round
2. Defeating reason: all 5 entries the issue reports as missed existed before their discussion,
`argument@grounding-worklist@arg-56`. The owner, round 2: "nothing is supposed to add an entry between search and end of design
discussion. The suspected mechanism is likely wrong. It was probably a failure at worklist assembly
IMO. I think our mechanical tooling is the correct fix". Arguments: `argument@grounding-worklist@arg-42`, `argument@grounding-worklist@arg-50`, `argument@grounding-worklist@arg-56`, `argument@grounding-worklist@arg-57`.
Harvest: `issue@agent-skills@the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`
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

Round 0, the owner. Thread: #citation-direction-rule. "Overall, citations have to \"trickle down\"
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

The reply to round 0, the agent. Thread: #citation-direction-rule. "\"Rejected cites design\"
reverses `design@agent-skills@rejected-citation-direction`. [...] Its ground is partly yours". And:
"\"Goals never cite anything\" contradicts the refinement links that goals write today."

### 287 and 591 edges break the hierarchy `##arg-9`

The reply to round 0, the agent, measured with `analyze.py`. Thread: #citation-direction-rule. 287
edges here and 591 in thaum; design → rejected 173 / 203, issue → issue 18 / 255, design → issue 39
/ 32, goal → goal 12 / 0.

### Transitive service merges every goal's cluster `##arg-10`

The reply to round 0, the agent, measured with `analyze.py` and `analyze2.py`. Thread: #cluster-unit.
"159 heads in this repository form a single cycle-connected component [...] 215 of the 346 heads
\"serve\" all 20 goals, and every goal-rooted cluster holds 466 to 480 of the 677 entries. A design
→ design citation means \"depends on\", not \"serves the same goal\"."

### The record's shape, measured `##arg-11`

The reply to round 0, the agent. Threads: #cluster-unit, #agent-text-in-graph. Heads citing a goal
directly: 133 of 346 here, 47 of 297 in thaum; goal-rooted reach 342 of 677 and 208 of 801;
subject-rooted reach 668 of 677 and 707 of 801; "entries cited from agent-facing text | 111 (99
heads) | 55".

### An orphan is sometimes the reference rule working `##arg-12`

The reply to round 0, the agent. Thread: #cluster-unit. "of the 213 heads that cite no goal here, 44
cite no entry at all. I sampled 8 of them. [...] So an orphan is sometimes a recording failure and
sometimes the reference rule working as written."

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
meaning-based grouping you ruled for design entries in `design@agent-skills@design-entry-search-groups`.
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
"`design@agent-skills@standing-entry-search-groups` records that \"a title shows neither a
tripwire's firing clause nor a deferred trigger\"." "titles state the rule, per
`design@agent-skills@title-states-the-rule`, so skimming heads by title is more defensible." "No
real use exists yet".

### The unit decides the audit's reading `##arg-41`

The reply to round 1, the agent. Thread: #search-occasions. "It follows from the unit: a standing
entry is judged against the heads it cites, and at the audit an agent would otherwise read the
standing entries without those heads."

### A delta search at convergence `##arg-42`

The reply to round 1, the agent. Thread: #search-delta. "a cheap second search when convergence is
proposed. This is the open candidate of
`issue@agent-skills@the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work`".

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
#residual-per-anchor, #declared-register-units. "Of thaum's 38 \"residual\" tripwires, 32 guard an
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
`rejected` entries they cite, about 19,000 + 3,700 words. The other was core's design lines 1491 to
2959 plus theirs, about 17,500 + 5,000 words."

### The missed entries existed before the search `##arg-56`

The reply to round 2, the agent, measured with `git log -S"##<slug>"` and `--diff-filter=A`. Thread:
#search-delta. The 5 entries were added on 2026-10-07, 2026-10-15, 2026-10-12, 2026-10-03 and
2026-10-13, before their discussion of 2026-10-19. "So a delta search would fix nothing".

### A judgement miss is not an assembly miss `##arg-57`

The reply to round 2, the agent. Threads: #search-delta, #report-skeleton-command. "The issue
records of the second instance that each entry \"named a rejected alternative in words\". That
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

## New names, in one place

```text
crates/core/src/graph.rs                   new module: Unit, Cluster, Agent (pub(crate)), clusters(), pack()
crates/core/src/cli/mod.rs                 Command::Worklist(WorklistArgs), fn worklist
  WorklistArgs { out: PathBuf, verify: Option<PathBuf>, bound: usize (default 25000), seeds: Vec<String> }
  flags: --out <DIR>, --verify <DIR>, --bound <WORDS>, --seed <REF> (repeatable; D4)
files under <DIR>:
  agents.txt                               one line per agent and entry: <agent> <reference>
  agent-<n>/entries.md                     every entry of the agent's clusters, whole, cluster by cluster
  agent-<n>/report.md                      the skeleton the searcher fills
crates/agent-skills/content/agents/record-searcher.md   new: knowledge-architect-record-searcher (D3)
existing, removed: crates/agent-skills/content/agents/standing-entry-searcher.md,
                   crates/agent-skills/content/agents/design-entry-searcher.md
existing: records::{Record, records, inbound, cited_within}, Record.group, Record.body
```

## Decided design

### The graph

The nodes are the entries of the record: every entry of `goal`, `design`, `rejected`, `issue`,
`tripwire` and every declared register, in every anchor, a location's included. Plan documents and
their items are no nodes: they are deliberation and leave when their work lands. Agent-facing text
is no node, `thread@grounding-worklist@agent-text-in-graph`. An edge is a reference from inside one
entry's body to another entry, read as `cited_within` reads a reference. No direction rule is added:
membership reads edges either way. The nearest rival, transitive membership over design → design
edges, lost on `argument@grounding-worklist@arg-10`: it merges every goal's cluster into one.

### The units

A unit register is `design` and every declared register. A heading register's units are its home's
level-two subjects: in a file home, each level-two heading of the file; in a directory home, each
level-two heading of each subdocument. The entries before any level-two heading of a file form one
unit of that file. A file register's units are its group subdirectories, per
`design@core@groups-are-subdirectories`, and the entries at the top of its home form one unit. A
Directory register, whose entries are directories, has no groups: its entries form one unit per
anchor. Every entry of a unit register is in exactly one unit. The nearest rival, a unit rooted at a
goal, lost on `argument@grounding-worklist@arg-15` and `argument@grounding-worklist@arg-30`.

### The clusters

A cluster is a unit and:

- every `rejected` entry an entry of the unit cites;
- every issue and every tripwire that cites an entry of the unit;
- every goal an entry of the unit cites, and every goal those cite, transitively.

A goal pulls in nothing: an issue or a tripwire that cites only goals is no member of a cluster
through them. The rival, goals pulling in their citers, lost on `argument@grounding-worklist@arg-31`.

### The residual

Every entry no cluster holds is in the residual group of the anchor that holds it, one group per
anchor that has any. A residual group is packed like a cluster, and gets the same notice above the
bound. It brings back `rejected@agent-skills@one-search-agent-per-anchor` in part; that entry's
reason, "An anchor bounds nothing", is defeated here, since the group is packed under the bound and
named when it exceeds it.

### The packing

The load of a cluster or a residual group is the sum of the words of its entries, an entry counted
once in it. Clusters are never split. A cluster above the bound goes to an agent alone. The others
are packed into the fewest agents first-fit-decreasing allows, then spread over that number of
agents: each cluster, largest first, goes to the least-loaded agent it fits in under the bound, and
to a new agent where it fits in none, so that the loads sit near the mean rather than at the bound.
Bin packing is NP-hard, and no optimum is sought: first-fit-decreasing uses at most 11/9 of the
optimal number of agents plus 6/9, and measured here and in thaum it used the lower bound, `argument@grounding-worklist@arg-61`. The spreading is the session's choice, argued from
`design@agent-skills@standing-entry-search-groups`: equal groups keep one agent from carrying the
full bound while another carries a remnant. It is what makes the owner's premise of `argument@grounding-worklist@arg-60` hold,
`argument@grounding-worklist@arg-61`. Ties break on the cluster's key, its home's path and its heading, so two runs over one tree
give one packing. An entry in two clusters of one agent is listed once for that agent. The bound
is 25,000 words by default, on the owner's word, round 3; `--bound` overrides it. Its measurements
are `argument@grounding-worklist@arg-55` and `argument@grounding-worklist@arg-61`, and the owner's argument `argument@grounding-worklist@arg-60`.

### `worklist`

`worklist --out <DIR>` builds the model, refuses with exit 2 over a tree that stops before the last
phase, as every writer does per `design@core@phases-gate-the-report`, and refuses a `<DIR>` that
exists and is not empty. It writes, per agent, `entries.md` and `report.md`, and `agents.txt` for
the whole run. It prints one line per agent with its clusters and its load, and one notice per
cluster or residual group above the bound. Illustration of a shape, not authority:

```text
agent-3  23,410 words  core :: 3. Names and references; core :: 6. The agent configuration
notice: subject "4. State model — the decisions" (crates/thaum-engine/docs/design/state-model.md)
        holds 55,103 words, above the 25,000-word bound: split it into subjects read apart
```

`report.md` lists every entry of the agent, cluster by cluster, one line each, and ends with a
section for what the searcher met outside the task. With `--seed`, D4, each line of an entry that
cites a seed names it. Illustration of a shape:

```text
## Cluster: core :: 3. Names and references
- design@<anchor>@<id> | grade: | relation: | reason:
- issue@<anchor>@<id> | cites seed: design@<anchor>@<seed> | grade: | relation: | reason:

## Met outside the task
```

### `worklist --verify`

`worklist --verify <DIR>` reads `agents.txt` and every `report.md`, and exits 1, naming each, for:
an entry of an agent's list missing from its report, an entry listed twice, a grade outside the set
or empty, a relation outside its kind's set, a reason left empty for `bears` or `may-bear`, and a
reference that is not in that agent's list. It exits 2 on a file it cannot parse, naming it, per
`design@core@a-failed-parse-is-loud`, and 0 on complete reports. The grades:

| kind of entry | `bears` relations |
| --- | --- |
| design, a declared register's entry, goal | `constrains`, `may change` |
| rejected | `may bring back` |
| issue | `touches`, `closes`, `worsens`, `depends on` |
| tripwire | `fires`; a deferred issue's trigger met is `touches` with the reason saying so |

`may-bear` and `unrelated` take no relation. The nearest rival, a check of the report in the
installed agent's text alone, lost: an instruction to list every entry once is the one
`issue@core@searcher-reports-are-written-and-checked-by-hand` records as unchecked.

### The searcher

`knowledge-architect-record-searcher` (D3) takes the work, the seeds, its agent directory and a
scratch directory of its own. It reads `entries.md` whole, judges every entry against the work
leaning to recall, and edits `report.md` in place, its tools being those of the two searchers plus
`Edit`. Its description carries the dispatch: run `worklist --out`, send one agent per
directory in parallel, run `worklist --verify` when they return, send again an agent whose report
fails it, then read whole every entry graded `bears` or `may-bear`. It replaces both searchers; the
point of `design@agent-skills@standing-entry-search-agent` that the search stays apart from the
standing-state reviewer stays.

### The occasions

The search runs over the whole record at design step 1, at the retrospective intake's grounding and
at the design audit of a slice or a spec. At the intake, the reading rule of
`design@agent-skills@search-relevance-grade` for `may-bear` entries stays. A second dispatch for a
thread on a new subject is no longer needed: the whole record is read. The session still reads the
goals homes itself, per `design@agent-skills@searcher-grades-goals`.

### The split standard

`skill@knowledge-architect-decision-recording@current-design` gains the standard: a subject holds
the heads a piece of work reads together, one part of the Component, such as a command, a register
or an activity; a subject whose cluster `worklist` names above the bound is split, and so is one
whose heads answer questions about parts that change apart. A split moves headings and decides
nothing, so it is a fix in a commit of its own, per `primer@met-outside-the-task` row 2. The
decision-record reviewer's `agent@knowledge-architect-decision-record-reviewer@design-head-rules`
and the design-record auditor point to it, and report a subject the standard says to split. The
nearest rival, a check finding above the bound, lost on `argument@grounding-worklist@arg-37`.

## Mapping tables

**The role of each register in the worklist**, over every register the manifest can hold:

| register | role |
| --- | --- |
| `design` | unit register, by level-two subject |
| a declared heading register | unit register, by level-two subject |
| a declared file register | unit register, by group subdirectory |
| a declared Directory register | unit register, one unit per anchor |
| `goal` | context: joins every cluster whose entries cite it or a goal it refines; else residual |
| `rejected` | joins every cluster whose entries cite it; else residual |
| `issue` | joins every cluster one of whose unit entries it cites; else residual |
| `tripwire` | as `issue` |
| `spec`, `milestone`, `thread`, `argument`, `criterion`, `acceptance` | none: plan documents are no nodes |

**Which text dispatches what, before and after:**

| site | today | after |
| --- | --- | --- |
| `skill@knowledge-architect-design@discussion-loop` step 1 | both searchers | `worklist`, the searcher, `worklist --verify` |
| `skill@knowledge-architect-planning@working-a-slice` point 2 | the standing-entry searcher | the same as step 1 |
| `skill@knowledge-architect-retrospective-intake@ground-before-judging` item 3 | both searchers, and again per Component | the same as step 1, once |

## Losing alternatives

- **Goal-rooted clusters**, the owner's, round 0, and **leaf-goal clusters**, the owner's, round 1:
  lost to #cluster-unit on measured coverage, `argument@grounding-worklist@arg-15` and `argument@grounding-worklist@arg-30`.
- **Transitive membership over design → design edges**: lost to #cluster-unit, `argument@grounding-worklist@arg-10`.
- **One direction of citation, checked**: #citation-direction-rule, withdrawn by the owner, `argument@grounding-worklist@arg-19`.
- **Agent-facing text as nodes**: lost to #agent-text-in-graph, `argument@grounding-worklist@arg-20`.
- **Goals that pull in their citers**: lost to #goals-walked-upward, `argument@grounding-worklist@arg-31`.
- **One residual group for the project**, #residual-group: superseded by #residual-per-anchor, on
  the growth `argument@grounding-worklist@arg-52` measured.
- **Two searchers kept**: lost to #one-searcher-agent, `argument@grounding-worklist@arg-36`.
- **A bound in entries, then in bytes**: lost to #agent-packing-bound, `argument@grounding-worklist@arg-32` and `argument@grounding-worklist@arg-46`.
- **A check finding above the bound**: lost to #split-standard, `argument@grounding-worklist@arg-37`.
- **Clusters chosen by the work, by title or by Component**: lost to #search-scope, `argument@grounding-worklist@arg-23` and
  `argument@grounding-worklist@arg-27`; the lighter pass is parked, #title-tier-for-untouched-components.
- **A smaller search on the in-change path**: #scope-by-path, withdrawn, `argument@grounding-worklist@arg-39`.
- **A delta search at convergence**: #search-delta, withdrawn, `argument@grounding-worklist@arg-56`.
- **First-fit packing without spreading**: lost to the spreading of Decided design, `argument@grounding-worklist@arg-61`.

## Readings

The work reads no external specification.

## Premortem

- **P1**, stresses #report-skeleton-command and #one-searcher-agent: `worklist --verify` passes on a
  report graded without real reading, every entry "unrelated" with a boilerplate reason. Named as a
  cause for the harvest to judge as a tripwire: `tripwire@agent-skills@design-entry-search-misses`
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

### AC2. The agents' lists together hold every entry of the project `##union-is-every-entry`

Guards `thread@grounding-worklist@subject-unit-both-shapes`, `thread@grounding-worklist@declared-register-units`
and `thread@grounding-worklist@residual-per-anchor`. Judged at step 2. Fires when, over a project a
test writes that holds a file home and a directory home, heads under no level-two subject, a
declared file register with groups, a goal refined twice and reached by walking upward, and an
issue citing only a goal, the union of the lists in `agents.txt` is not every entry of the project,
or that issue is not in its anchor's residual group. Response: reopen the three threads.

### AC3. A cluster above the bound goes alone, and is named `##oversized-cluster-alone`

Guards `thread@grounding-worklist@agent-packing-bound`. Judged at step 2. Fires when a test's
cluster above `--bound` shares its agent with another cluster, or the output does not print its
notice. Response: reopen #agent-packing-bound.

### AC4. `worklist` refuses over an incomplete model `##refused-over-an-incomplete-model`

Guards `thread@grounding-worklist@report-skeleton-command`. Judged at step 2. Fires when `worklist
--out` over a tree with a phase-2 finding exits other than 2, or writes any file. Response: reopen
#report-skeleton-command.

## Implementation sequence

1. **The graph, with no command.** Builds `planned@core@src/graph.rs`: units, clusters, residual
   groups and the packing, as pure functions of the model and the entity table, with unit tests over
   models written as text. Fails alone on: a membership or a packing the unit tests name. Claims:
   - every entry of a unit register is in exactly one unit, in both home shapes and for a declared
     file register's groups; refuted by a test whose entry lands in none or two;
   - a goal joins a cluster through a citation and through refinement, and pulls in no issue;
     refuted by a test whose issue cites only a goal and joins the cluster;
   - every entry is in a cluster or in its anchor's residual group;
   - the packing uses the fewest agents first-fit-decreasing gives, sends a cluster above the bound
     alone, spreads the rest, and gives one result over one input; refuted by a test with clusters
     of known sizes and their expected agents.

   Each test is shown to fail by mutating its rule, a goal walked downward instead of upward, for
   instance, in a scratch worktree.
2. **`worklist --out`.** Builds the variant, its arguments and the files. Fails alone on: the files
   written or the refusal. Claims: AC2, AC3 and AC4, by tests in `path@core@tests/binary.rs` that
   write their own project, as `History` does; the core's README gains the command's row.
3. **`worklist --verify`.** Fails alone on: the verdicts. Claims: AC1, by tests over directories
   step 2 wrote and then edited.
4. **The installed text.** Creates `planned@agent-skills@content/agents/record-searcher.md`,
   removes the two searchers' files, rewrites the three dispatching texts of the mapping table and
   the intake's `%%` lines, writes the split standard into
   `skill@knowledge-architect-decision-recording@current-design`, points
   `agent@knowledge-architect-decision-record-reviewer@design-head-rules` and the design-record
   auditor to it, runs `cargo klarch install-agent-skills`, and writes the changelog entries:
   New features (`cli`, `worklist`) and Workflow (`agent-skills`, one searcher; the split notice).
   Written under `skill@knowledge-architect-agent-configuration`, as
   `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", says. Fails alone on: the
   check, a skill naming an agent that no longer exists, or the build.
5. **Measure and harvest.** Runs `worklist --out` over this repository and over thaum, and records
   the agents, the loads and the notices in the commit's message; then the harvest.

## Order rationale

1. Step 1 before step 2: the command writes what the graph computes, so the graph is proved alone.
2. Step 2 before step 3: verify reads the files step 2 writes.
3. Step 3 before step 4: the searcher's text names both commands, which exist then.
4. Step 4 before step 5: the measurement and the harvest record what is built.

## Defaults awaiting the owner

- **D1**, #entry-graph: plan documents are no nodes, and every reference between entries is an edge.
  It stayed `presumed-settled` through two checkpoints with no word naming it. Built as written.
- **D2**, #split-standard: "design audits" in the owner's word of round 2 is read as the
  design-record axis of the project audit, which the design-record auditor carries; the design audit
  of a slice gains no check. Built as written.
- **D3**, #one-searcher-agent: the searcher is named `knowledge-architect-record-searcher`, its file
  `record-searcher.md`. Built as written.
- **D4**, #report-skeleton-command: `--seed <REF>`, repeatable, makes each skeleton line of an entry
  that cites a seed name it, so the searcher does not map seeds through `show`. Built as written.

## Harvest

| what | where | when |
| --- | --- | --- |
| the graph, the units, the clusters, the residual, the packing and `worklist` with `--verify`: an interface others consume, test 1 | `register@core@design`, one head or more as `skill@knowledge-architect-decision-recording@head-shape` cuts them | the commits of steps 2 and 3, or step 5 |
| the unified search, its occasions and the whole record | `design@agent-skills@design-entry-search` and `design@agent-skills@standing-entries-searched-before-the-work`, rewritten; `design@agent-skills@standing-entry-search-agent` folded into the first | step 5 |
| the clusters and the bound of 25,000 words with `argument@grounding-worklist@arg-55`, `argument@grounding-worklist@arg-60` and `argument@grounding-worklist@arg-61`, the owner's argument attributed | `design@agent-skills@design-entry-search-groups` and `design@agent-skills@standing-entry-search-groups`, rewritten as one or two heads | step 5 |
| the grades for every kind | `design@agent-skills@search-relevance-grade`, rewritten | step 5 |
| goals as context | `design@agent-skills@searcher-grades-goals`, rewritten | step 5 |
| the split standard | a new head in `register@agent-skills@design` | step 5 |
| the losing alternatives | `register@agent-skills@rejected` and `register@core@rejected`, as the tests of `skill@knowledge-architect-decision-recording@losing-alternatives` admit | step 5 |
| the bound of 60 the design-record axis borrows | `design@agent-skills@design-record-axis-dispatch`, its sentence standing on its own measurement | step 5 |
| the rival's task named in `design@agent-skills@a-scratch-directory-per-subagent` | rewritten | step 5 |
| P1 to P5 | judged as `skill@knowledge-architect-decision-recording@ruled-tripwires` says; the two tripwires of P1 rewritten | step 5 |
| the issues | `issue@core@searcher-reports-are-written-and-checked-by-hand`, `issue@agent-skills@the-standing-entry-searcher-has-no-list-for-a-doubt`, `issue@agent-skills@the-design-entry-search-does-not-run-at-a-design-audit` and `issue@agent-skills@the-search-before-the-work-missed-standing-entries-in-two-pieces-of-work` closed; `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` rewritten; the issue of #title-tier-for-untouched-components opened | step 5 |

## Later consequences

- The bound may rise to 30,000 words, the owner's word of round 3: "I'm even considering raising it
  a bit more, maybe 30000. but that can wait for a few iterations." The head on the bound carries
  that.
- The "goal coverage" axis of `issue@agent-skills@audit-axes-beyond-the-design-record` may read the
  graph `worklist` builds; nothing is built for it now.
- The lighter pass for untouched Components waits in its new issue.
