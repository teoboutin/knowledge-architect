# Design

How query is built, and why.

## What the commands answer

### This crate depends on no crate of the checks `##query-depends-on-no-check`

A tool over the record needs the model and at most phases 1 to 3, per `design@knowledge-architect@model-crate` and
`goal@model@a-tool-reads-the-documents-with-no-check`, so nothing here runs or names a check, and
`cargo tree -e normal -p knowledge-architect-query` lists no `knowledge-architect-check`. So a program that
consults a record pays for none of the checks, and each of them can change with no change here. A dependency added here passes
every gate, so a reader of this head is what catches it. A command that needs a check's output goes to `check`, as
`design@knowledge-architect@query-crate` says of `model`.

### Each function takes a manifest its caller configures, and a program that links no check reads a project with an extension as it stands `##a-function-takes-a-configured-manifest`

`cli::run` configures the manifest with each extension the binary registers before it dispatches, so a binary's commands of this crate read a project as that binary's `check` does. A program that links this crate and no check holds no extension: it walks what the extension generates, and `id` and `worklist --out` refuse the extension's table. No pin is refused here either, since a binary refuses it before dispatching. On the owner's reading of its reach, `goal@model@a-tool-knows-the-read-is-right` covers what makes the read wrong, and neither a pin nor an extension's table is that: a program that links no check reads the project as it stands. The session chose this over a type only `extension::configure` returns, placed in `model`, which each function would take, and which no goal ranks; the owner approved it. The choice costs a program that links no check a read that differs from a binary's; the rival's cost on `Manifest` as the facades publish it is not established. `issue@knowledge-architect@a-typed-command-trusts-its-caller-to-have-configured-the-manifest` holds the owner's view that the precondition is ultimately to be enforced.

### `anchors` lists every anchor of a project, computed as a run computes them, with the home of each register it carries but a plan's `##anchors-listing-command`

`klarch anchors` prints every anchor, the constructed ones included, built by `Anchors::of` over the
manifest and the tree's listing as every check builds them: its name, its kind and its path, and,
under every anchor but a plan's, each register it carries with the home the tree holds,
the file, the directory, both, or `missing`. An entry's kind is the Directory register it is an
entry of. A milestone and a spec list no register: a plan
anchor's registers keep their homes inside the plan itself, its sections, or a milestone's own
directory for its slice specs. A home is judged held by the rule phase 2 of a run judges it by, over the
same listing, so the two agree on what a project carries. It exits 1 when a home is missing, a negative answer
about the project, per `design@cli@exit-code-ladder`. Its output is for a reader and is not a
contract, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

It answers a question no other command answers: `model` prints observations, and `show` one
register's homes. The rival, each project's root `CLAUDE.md` listing its anchors by hand, goes stale
when the manifest changes, and no check reports the stale list; the command is computed from what
it describes. The primer names it, so every installing project's sessions learn its anchors from
it. It runs no phase, as `show`, `issues` and `tripwires` run none.

### `id` answers whether an id is free with every entity the table defines under it, of every kind and every anchor, and leaves to its reader which collision matters `##id-lookup-command`

`klarch id <id>...` prints, for each argument in order, `free` where the entity table defines
nothing under it, every entity it defines under it, as the reference `show` takes, or that the
argument is outside the grammar of `design@model@two-segment-ids`. It reads the table through
`Entities::by_id`, unfiltered, the set the lints on ids filter per citing document. It exits 0 when
every argument is free and 1 otherwise, a malformed argument being a negative answer about the
candidate, per `design@cli@exit-code-ladder`, since the run answers every other argument; and 2
over an incomplete model, since a `free` there is the finding computed over an incomplete model that
`design@check@phases-gate-the-report` refuses.

It answers a question no other command answers, per `design@check@one-question-one-command`: `show`
takes a whole reference, and the lints compute the set only to report a span. A design thread's slug
is checked against the design and `rejected` entries of the Component that will own its decision,
per `design@agent-skills@thread-slug-is-entry-id`, and without `id` a session checks it by a loop
over two `show` calls per candidate, the case `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`
gives the checker. An anchor's name, a Component's, a location's or a plan's, is no entity, so `id`
lists none: a plan named like an anchor is refused by `check`, per
`design@model@anchors-are-components-and-locations`. The rule on which collision refuses an id stays with the reader, since another
reader, such as a label or a new entry of another register, has another rule. The nearest rival, a
test of one named anchor's design and `rejected` entries, needs the owning anchor before the thread
can name it: `rejected@query@slug-test-of-one-anchor`.

### A tripwire guards no `rejected` entry `##tripwire-guards-no-rejected-entry`

The `guarding` column of `cargo klarch tripwires`, and its `--guarding` filter, read every
reference of a tripwire to an entry of a register but `issue`, `tripwire` and `rejected`. A
tripwire guards a decision: a `rejected` entry records a design not kept, which is no decision, so
a tripwire that names one names the candidate its response would reopen, not what it guards. The
rival, counting it like a declared register's entry, would list a design not kept as guarded.

## The record search

### The record `klarch worklist` reads is every entry of every register but the plan registers, and its edges are the references written inside an entry `##record-is-every-register-entry`

The record holds every entry of `goal`, `design`, `rejected`, `issue`, `tripwire` and of every
register a project declares, in every anchor, a location's included. An edge is a reference written
inside an entry's own text, the section of a heading entry or the file of a file entry, to another
entry; a home's prose outside every entry yields none, and membership reads an edge either way. Plan
documents and their items are no part of it: they are deliberation, and leave when their work lands.
Agent-facing text is no part of it either: a decision stated by one text of it earns no head under
entry test 2 of `primer@design-heads`, its reader is the session that edits that text and reads it
whole, and the installed text cites no entry, so in an adopting project it would add nodes with no
edges. The command relies on the checker working as intended: the edges are the references the
checker resolves. It serves `goal@knowledge-architect@records-reach-their-reader`, a listing computed from the tree.

### Every Component and every declared location is a search unit, packed into an agent whole unless its load exceeds the bound `##packed-by-search-unit`

The search units are the Components and the locations the manifest declares. An entry belongs to the
search unit of its anchor, an anchor the tool constructs included, per
`design@query@constructed-anchor-in-its-home-s-unit`. A search unit's clusters, per `design@query@cluster-rooted-at-a-subject`, and its remainder, per
`design@query@residual-grouped-by-search-unit`, are built from its own entries alone. A search unit
whose load fits the bound goes to one agent whole; one above it is split into its clusters and its
remainder, packed per `design@query@clusters-packed-under-a-word-bound`, an entry its clusters share
kept in one of them, per `design@query@shared-entry-in-its-most-tied-cluster`.

It serves `goal@knowledge-architect@agents-work-without-drift` and
`goal@agent-skills@installed-text-works-in-every-target-project`: an issue is judged beside the design
of its own Component, and a project of one Component or with a large location, as thaum's `rules`, is
packed by what its manifest declares. The owner judged an agent that grades an issue of a Component
whose design it saw none of a major defect, and narrowed it to an issue that cites a head, which the
rule of `design@query@cluster-rooted-at-a-subject` covers inside a search unit and
`design@query@context-entries-given-whole` across them. The nearest rival, clusters packed under the
bound whatever their Component: `rejected@query@clusters-packed-across-search-units`. Small search units share agents rather than taking one
each, which answers `rejected@query@one-search-agent-per-anchor`: the split above the bound
bounds what an anchor does not. Where a search unit stands above or below half the bound is
`design@query@search-unit-above-half-the-bound-alone`.

### An entry of an anchor the tool constructs belongs to the search unit that holds its home `##constructed-anchor-in-its-home-s-unit`

An anchor the tool constructs, the entry anchor of a Directory register or `plans`, belongs to the
search unit that holds its home, so it is never a search unit of its own. Its nearest rival, a
constructed anchor as a search unit of its own, would pack every entry directory of a Directory
register as an item of its own, apart from the register's home and its Component's design, against
`goal@knowledge-architect@agents-work-without-drift`, under which an entry is judged beside the
design of its own Component.

### An entry that the clusters of a split search unit share is graded in the cluster it is most tied to `##shared-entry-in-its-most-tied-cluster`

Before a split search unit is packed, every entry its clusters share stays in one of them, the one it
is most tied to, the first in the unit's order on a tie: an issue or a tripwire in the cluster holding
most of the heads it cites, a `rejected` entry in the cluster holding most of the heads that cite it,
the same rule read from the other end of the citation. So every entry is graded in exactly one agent.
A cluster that lost a shared `rejected` entry gets it as a context entry
where the two clusters go to different agents, since its heads cite it, per
`design@query@context-entries-given-whole`; a cluster that lost an issue or a tripwire does not,
since that entry cites the cluster's heads and no head the agent grades cites it. The nearest rival, a
shared entry graded in every cluster that holds it, repeats its verdict in every agent: in the intake
of seventeen files of 2026-10-25 to 2026-10-31, 200 entries took 365 grade lines, 6 entries 11 lines
each, as the issue on those repeated lines recorded until the harvest of this decision closed it, and
the session reads each repeated line.

### A search unit whose load exceeds half the bound gets agents of its own, and the others are packed together whole `##search-unit-above-half-the-bound-alone`

A search unit's load is the load of one group holding all of it, per
`design@query@clusters-packed-under-a-word-bound`. One whose load exceeds half the bound left gets
agents of its own: one where it fits the bound, else its groups spread by the packing. The search
units whose load is at most half the bound left are packed together, each whole, by the same packing.
Goals follow the same rule in the goal agents, per `design@query@goal-context-once`.

Two search units each above half the bound can never share an agent, since together they exceed it,
so at half the rule only keeps a small search unit out of a large one's agent. The owner chose it, on
the owner's judgement that a large Component is likely more critical to the project, so that an
agent's attention is better not divided over it, and that its entries probably interact, a judgement
the owner held to be somewhat arbitrary, with no measurement behind it. The owner chose the rule over small
search units packed into the room large ones leave, which no goal ranks, at a cost of more agents
than the rival packs: the loads `klarch worklist --out` prints for each item re-take it, packed both
ways. The rival is `rejected@query@small-units-in-large-units-room`.

### Every entry of a unit register is in one unit, cut by the headings or the groups of its home, and a cluster is rooted at a unit, never at a goal `##cluster-rooted-at-a-subject`

A unit register is `design` or any register a project declares. Its entries are grouped into units:
a heading register whose entries sit at level 3 or deeper by the nearest heading above each entry
that is shallower than the entries, in a single file or in each subdocument of a directory home,
each such heading a unit of its own, so a design home's units are its level-two subjects and the
entries under its title; a heading register at level 2 by file; a file register by its group
subdirectories, per `design@check@groups-are-subdirectories`, and the top of its home; a Directory
register by the anchor that carries it. A cluster is a unit with the `rejected` entries of its search
unit that its entries cite, and the issues and tripwires of its search unit that cite them; it holds
no goal, per `design@query@goal-context-once`.

It serves `goal@agent-skills@installed-text-works-in-every-target-project`, since every unit comes from what the
manifest declares and the tree holds. A subject groups what the work reads together: the owner
chose that grouping for design entries, since a subject's kept and rejected entries have to be read
together, and a standing entry joins the cluster of the heads it cites, since it is judged beside
them, and `design@query@context-entries-given-whole` carries that across search units and across the
agents of a split one. The nearest rival of the rooting, a cluster rooted at a goal, reached 342 of the 677 entries of this repository
at ed9dd60 and 208 of the 801 of thaum at its commit edd2cb5d, since 213 heads here cite no goal, and
the reference rule of `design@agent-skills@a-reference-claims-a-revisit` has a head name a goal only
where it derives a constraint from it; rooted at a subject, the clusters reached 663 and 706.
Followed transitively through design to design citations, a goal-rooted cluster held 466 to 480 of
the 677 entries, since 159 heads cite one another in one cycle. The counts were taken by scripts over
`klarch model`, which the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds. The design this decision did not keep:
`rejected@query@goal-rooted-clusters`.

### An entry that an entry an agent grades cites, and that another agent grades, is given to that agent whole, read only `##context-entries-given-whole`

Every head, entry of a declared register or `rejected` entry that an entry an agent grades cites,
and that the agent does not grade, is that agent's context entry: given whole, read only, and given
no verdict there. It carries the grouping of `design@query@cluster-rooted-at-a-subject` across search
units and across the agents of a split one: an issue that cites a head another agent grades is
judged beside the head's text, which serves `goal@knowledge-architect@agents-work-without-drift`. Its
nearest rival, a context entry given by its title alone, packs fewer agents and judges such an issue
without the head's text, which the rule exists to give it. A context entry counts in its agent's load,
per `design@query@clusters-packed-under-a-word-bound`.

### Goals reach a cluster agent as their title lines, and goal agents grade every goal whole, once `##goal-context-once`

A cluster agent is given, read only, the title line of each goal of the root, of each Component
enclosing a search unit it holds, and of those search units: the goal's heading statement without its
slug, with its reference. A goal brings in nothing: an issue or a tripwire that cites only goals is no
member of a cluster through them. The goals are graded apart, grouped by goals file inside each
search unit, each group with its file's text above its first goal, and packed into goal agents by the
rules of the cluster agents, per `design@query@search-unit-above-half-the-bound-alone`; so each goal
is graded once, by an agent that reads it whole.

It serves `goal@knowledge-architect@agents-get-a-complete-workflow`: an agent's reading grows with
the distinct entries of the record, not with how many clusters cite a goal. A title states the goal,
and the searcher reads a goal whole before a verdict rests on it, per
`design@agent-skills@searcher-grades-goals`. The nearest rival, every goal a cluster's entries cite
copied into that cluster with the goals they refine: `rejected@query@goals-copied-into-every-cluster`.
One goal agent for every goals home loses to a goals home that can exceed the bound, as the owner
judged. Goals that bring in their citers: `rejected@query@goals-pull-their-citers`.

### Every entry no cluster holds, goals apart, is in the remainder of its search unit `##residual-grouped-by-search-unit`

One remainder per search unit that has any, packed with the unit's clusters, as one group where the
unit is split, and named by a notice when its own entries exceed the bound. On the owner's argument,
the standing entries were already cut into groups before, and the entries of one search unit share
the Component or the location they apply to. A remainder per anchor would make the entry anchor of a
Directory register a group of its own, read apart from its home, against
`design@query@packed-by-search-unit`. A single remainder for the project grows with every entry no
head reaches: issues about code with no head, issues citing only goals, and tripwires guarding a
single-site instruction, each a shape the reference rule admits, which made 14 entries here at
ed9dd60 and 55 in thaum at edd2cb5d, counted while the remainder still held the goals no head
serves. The design this decision did not keep:
`rejected@query@one-residual-group`.

### `worklist --out` writes the agents' entries, the report skeletons and the work into a directory outside the project, and `worklist --verify` judges the filled reports against the list it wrote `##worklist-writes-and-verifies-the-reports`

`klarch worklist --out <dir>` writes `agents.txt`, one line per agent and entry with its role, its
seed mark and its `changed-since` mark; `work.txt`, the files of the work, the subjects of the work,
and the plan document and the commit where they are given; per agent `entries.md`, the entries it
grades whole, then its context entries and its goal title lines, and `report.md`, one line per entry
to give a verdict, naming the verdicts its register admits; and `shown.md`, the snapshot of
`design@query@occasion-shaped-output`. It takes `--work <path>` for each file of the work, whose
words leave the bound, `--subject <id>` for each subject of the work, `whole-work` where none is
given, `--seed <ref>`, `--bound`, `--since`, per `design@query@since-marks-changed-entries`, and `--seeds-from`, per
`design@query@seeds-from-the-plan-s-decided-section`.
It is a writer, so it refuses over an incomplete model, per `design@check@phases-gate-the-report`,
and it refuses a directory inside the project, whose files the walk would read, a directory that
holds anything, a seed that names no entry, a file of the work it cannot read, and a path whose line
break would split a line of `work.txt`, writing nothing. `klarch worklist --verify <dir>` reads no
model: it judges each report against `agents.txt`, `work.txt` and the snapshot, and reports an entry
given a verdict nowhere or twice, a reference the report's own agent was not given, a verdict outside its register's
set, an action outside its verdict's, an `against` naming no subject of `work.txt`, a quotation it
does not find as a run of whole words of the entry's own text, or shorter than the length the code
asks, a missing reason, an entry that is or cites a seed given `unrelated`, and a report missing its
closing sections; a line it cannot parse stops it. The verdicts and their actions are
`design@agent-skills@verdict-report`'s. When every report passes, it writes the files of
`design@query@occasion-shaped-output`.

It serves `goal@knowledge-architect@records-reach-their-reader` and `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`:
an instruction to give every entry one verdict, with a quotation of its text, is kept by a command,
not by a reader. Each skeleton line names the verdicts its register admits, since a searcher reads
the table once and applies it from memory: over three searches whose skeleton named none, 13 of 45
searchers wrote a relation their entry's role did not admit, as the issue on that skeleton recorded
until the harvest of this decision closed it. The nearest rival, each searcher deriving its group from the listings and writing
its report by hand, listed one entry under two lists in the discussion that produced the `rejected`
register, and only the searcher's own correction, appended to its report, showed it. The command's
output is read by its own `--verify`, the searcher and the session, and is no contract with another
reader, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

### `worklist --since` marks the entries whose file changed since a commit, and filters none `##since-marks-changed-entries`

`--since <commit>` resolves the revision to one commit and marks `changed-since`, in `agents.txt` and
on each entry's line in the entries files, every entry whose file differs between that commit and
the work tree, an untracked or renamed file included, through `git`'s `paths_changed_since`; it marks
no code, and every entry is still graded. A failure of git stops the run with git's reason, per
`design@git@failed-git-is-an-error`.

It serves `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: what changed since the design
audit's spec is read from the tree by a command. A mark, not a filter: a search of what changed alone
would miss the entries recorded before the work, `rejected@agent-skills@delta-search-at-convergence`,
and `--since` draws attention where the record moved under a spec written earlier. Code is not
marked, since the record is the registers' entries, per `design@query@record-is-every-register-entry`.

### `worklist --seeds-from` seeds from a plan document's section "What is already decided", its decisions only `##seeds-from-the-plan-s-decided-section`

`--seeds-from <plan>` takes a spec, a slice's spec or a milestone document, and adds as seeds the
design entries and the declared-register entries its section "What is already decided" cites, no
goal and no `rejected` entry; it combines with `--seed`, is refused where the path is no plan
document, and writes the plan document into `work.txt`, which makes the run an audit for `--verify`.

It serves `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: the design audit's seeds are
read from the tree by a command, per `design@agent-skills@seeds-name-decisions`, which decides that a
seed is a decision and never a goal. The nearest rival, the session listing the seeds by hand from
the plan document, is a script each audit writes again: the slice of the integrated tests, on
2026-11-07, extracted its 54 seeds by `grep` from the decided sections of its milestone document and
its spec, as the issue on that
seeding recorded until the harvest of this decision closed it.

### A passing `worklist --verify` writes, from a snapshot `--out` takes of every entry's show text, one verdict file per subject of the work and the per-entry lines a commit message owes `##occasion-shaped-output`

`klarch worklist --out` writes `shown.md`, the snapshot: every entry of the record with its own text
and its inbound list, as `show` prints them, in two fenced blocks, computed by `show`'s own code. No
agent is given it, so an agent's load is its entries file alone, and `--verify` compares a quotation
with the entry's own text alone. When every report passes, `--verify` writes, from it:

- one verdict file per subject of `work.txt`, `verdicts-<subject>.md`: every verdict other than
  `unrelated` whose `against` names that subject, with its agent's verdict line and its entry's text
  and inbound list, fenced as the snapshot fences them so that a heading of the text reads as no
  section of the file, in the order of `agents.txt`;
- outside an audit, `generated-lines.md`: one line per entry given a verdict other than `unrelated`,
  its reference, its verdict and its action, with no quotation and no reason, for the commit message
  the occasion owes;
- at an audit, a run whose `work.txt` names a plan document, `gaps.md`, every verdict other than
  `constrains` and `unrelated` with its entry's text, and `fit-lines.md`, the plan document cited
  whole, the commit named by its subject, and the line of each entry given a verdict other than
  `unrelated`, in place of `generated-lines.md`.

A run removes these files of a previous run before it judges anything, and a write that fails
removes those written before it, so a run that fails, or stops on a file it cannot read, leaves none
of them, per `design@cli@exit-code-ladder`, but where a removal itself fails: the removal stops at the
first file it cannot remove, and a failed removal of a file this run wrote is not reported.
`--verify` reads no model, so the session reads what the searchers judged, whatever the tree has
become since.

It serves `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
`goal@knowledge-architect@records-reach-their-reader` and
`goal@agent-skills@session-orchestrates-the-work`: a session that hands one subject of the work to an
agent hands it that subject's file, and the lines a commit message owes per entry are written by the
command in forms `cargo klarch commits` judges. Written by each session, they cost a script per
occasion: twelve sessions wrote them so, nine of this repository and three of thaum, and four met a
cycle of findings on the reasons they copied, of 13, 41 and 49, 11, and 45 findings, as the issue
on those lines recorded until the harvest of this decision closed it. A generated line carries no quotation, since a commit
message reads a quoted reference as live and a quotation's own marks can end its span early. The
nearest rival, the session running `show` once per entry it acts on, is a script each session writes
again: the grounding of the discussion that first assembled the entries did so over 202 entries, 576
KB of `show` output, as the message of the commit "[harvest] The graded entries' assembly, `klarch id`,
the narrower grounding and the retrospective's new question recorded" states. Its other rival, an
assembly that reads the model when `--verify` runs, either makes `--verify` read a model, which a tree
that gained a finding of the first three phases since `--out` would then block, or adds a third
command step to every grounding, for an inbound list current to the minute:
`rejected@query@gather-reads-the-model`. The files are read by `--verify`, the searcher and the
session, and are no contract with another reader, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`.

### The items are packed into the fewest agents first-fit-decreasing gives, spread evenly, under the bound left once the work is counted, an item above it alone `##clusters-packed-under-a-word-bound`

An item is a search unit packed whole or a group of a split one, per
`design@query@search-unit-above-half-the-bound-alone`. A group's load is the words of the entries it
grades, of its context entries and of its title lines, a goals file's group counting its text above
its first goal; a context entry or a title line that two groups of one agent share is counted in
each, so an agent's true load is at most the sum. The bound left is the bound less the words of the
work. An item above it goes to an agent alone, never split, and `worklist` prints a notice naming a
group above it only where its own entries exceed it, since the context entries and the title lines
are no part of the subject the notice asks to split. The others are packed first-fit-decreasing,
which fixes the number of agents, then dealt again over that many agents, each to the least-loaded
agent it fits in; where one fits in none, the first-fit assignment stands. It serves
`goal@knowledge-architect@agents-get-a-complete-workflow`: each agent's reading is bounded whatever
the project's size, and no agent is spent on a remnant.

What an agent reads is counted, not only what it grades: a bound on the graded entries alone would
let an agent's reading exceed it by every context entry and title line the agent is given. The work's words leave the
bound, since every agent reads the work whole. Words rather than entries or bytes: in the owner's
estimate, words translate a bit better to an agent's tokens; an entry's mean size ranged from 0.5
KB, a `rejected` entry, to 2.9 KB, an issue, over this repository at ed9dd60, so a count of entries
bounds a load poorly, while words and bytes differed by about 15% across kinds. First-fit-decreasing
uses at most 11/9 of the optimal number of agents plus 6/9, by the bound György Dósa proved tight in
2007; with the clusters and remainders as its items, it used the lower bound over this repository at
ed9dd60, 9 agents, and over thaum at edd2cb5d the lower bound for the groups within the bound, 8
agents, beside 3 groups above it sent alone. The spreading keeps an agent from carrying the full bound while another carries a remnant:
first-fit alone filled this repository's agents to a median of 24,737 words of a 25,000 bound at
ed9dd60, as the scripts the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds took it. Re-take the packing with `klarch worklist --out` and `--bound` over this repository, which prints each agent's load; a run whose
agents sit at the bound reopens the spreading. Splitting a cluster to meet a bound would cut a
subject, which the grouping exists to keep whole. The designs this decision did not keep:
`rejected@query@packing-to-the-bound`, `rejected@query@bound-in-entries`,
`rejected@agent-skills@one-agent-reads-every-standing-entry`,
`rejected@agent-skills@design-listing-cut-in-groups-of-sixty` and
`rejected@agent-skills@subject-groups-bounded-at-sixty`.

### The default bound is 40,000 words, so that a searcher's context stays below where the cost of its rounds ramps up `##worklist-bound-is-40000-words`

`DEFAULT_WORKLIST_BOUND`, in `path@cli@src/cli/mod.rs`, is 40000, and `--bound` sets another. The
owner reported that the tokens a searcher spends per round grow more steeply from about 250,000
tokens of context, while agents stay reliable past it, so the measure to weigh is cost, an agent's fixed start against the growing cost of each
round; the bound keeps a searcher's peak below that point. The owner chose 40,000 over the
session's 50,000 and over the owner's own first figure, 30,000, to raise the bound by steps while the
searchers are asked for fuller verdicts; no goal ranks the three. Both searches of the discussion
that chose it, real use per `design@agent-skills@synthetic-evidence-not-built`,
peaked under 250,000 tokens in every searcher at this bound. Re-take it, on the machine that ran the
search, over each searcher's transcript file, `subagents/agent-<id>.jsonl` under the session's
directory of the harness's transcripts, the input, cache and output tokens of its last turn:
`jq -s '[.[] | select(.message.usage) | .message.usage | .input_tokens + (.cache_creation_input_tokens//0) + (.cache_read_input_tokens//0) + .output_tokens] | last' <file>`.
A search whose searchers peak above 250,000 tokens reopens the bound, and one whose peaks stay well
under it is the ground of a next raise. No measurement of recall stands behind any value of the
bound; real sessions measure it, per `goal@knowledge-architect@the-workflow-improves-through-real-use`.
