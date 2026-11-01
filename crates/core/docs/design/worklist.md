# Core design: The record worklist

## The record worklist

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
checker resolves. It serves `goal@core@records-reach-their-reader`, a listing computed from the tree.

### Every entry of a unit register is in one unit, cut by the headings or the groups of its home, and a cluster is rooted at a unit, never at a goal `##cluster-rooted-at-a-subject`

A unit register is `design` or any register a project declares. Its entries are grouped into units:
a heading register whose entries sit at level 3 or deeper by the nearest heading above each entry
that is shallower than the entries, in a single file or in each subdocument of a directory home,
each such heading a unit of its own, so a design home's units are its level-two subjects and the
entries under its title; a heading register at level 2 by file; a file register by its group
subdirectories, per `design@core@groups-are-subdirectories`, and the top of its home; a Directory
register by the anchor that carries it. A cluster is a unit with the `rejected` entries its entries
cite, the issues and tripwires that cite them, and the goals of `design@core@goals-join-a-cluster-as-context`.

It serves `goal@agent-skills@installed-text-works-in-every-target-project`, since every unit comes from what the
manifest declares and the tree holds. A subject groups what the work reads together: the owner
chose that grouping for design entries, since a subject's kept and rejected entries have to be read
together, and a standing entry joins the cluster of the heads it cites, since it is judged beside
them. The nearest rival, a cluster rooted at a goal, reached 342 of the 677 entries of this repository
at ed9dd60 and 208 of the 801 of thaum at its commit edd2cb5d, since 213 heads here cite no goal, and
the reference rule of `design@agent-skills@a-reference-claims-a-revisit` has a head name a goal only
where it derives a constraint from it; rooted at a subject, the clusters reached 663 and 706.
Followed transitively through design to design citations, a goal-rooted cluster held 466 to 480 of
the 677 entries, since 159 heads cite one another in one cycle. The counts were taken by scripts over
`klarch model`, which the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds. The design this decision did not keep:
`rejected@core@goal-rooted-clusters`.

### A goal joins a cluster as context, with every goal it refines, and brings in no issue or tripwire `##goals-join-a-cluster-as-context`

A cluster holds the goals its unit's entries cite, and every goal those cite in turn, which are the
goals they refine. A goal brings in nothing: an issue or a tripwire that cites only goals is no
member of a cluster through them. The rival, goals bringing in their citers, made every citer of a
root goal join every cluster whose heads cite that goal: the reads rose from 839 to 1323 for 663
entries, over this repository at ed9dd60. The design this decision did not keep:
`rejected@core@goals-pull-their-citers`.

### Every entry no cluster holds is in the residual group of its anchor `##residual-grouped-by-anchor`

One residual group per anchor that has any, packed with the clusters under the bound of
`design@core@clusters-packed-under-a-word-bound`, and named when it exceeds it. On the owner's
argument, the standing entries were already cut into groups before, and the entries of one anchor
share the anchor they apply to. A single residual group for the project grows with every entry no
head reaches: issues about code with no head, issues citing only goals, goals no head serves, and
tripwires guarding a single-site instruction, each a shape the reference rule admits, which made
14 entries here at ed9dd60 and 55 in thaum at edd2cb5d. The packing answers
`rejected@agent-skills@one-search-agent-per-anchor`, whose reason was that an anchor bounds nothing.
The design this decision did not keep: `rejected@core@one-residual-group`.

### No rule on the direction of a citation is checked `##no-citation-direction-rule`

Membership of a cluster reads an edge either way, so a rule that citations run from goals down to
issues would buy acyclicity alone, which serves a transitive membership that merges the clusters.
The design this decision did not keep: `rejected@core@citation-direction-checked`.

### `worklist --out` writes the agents' entries and the report skeletons into a directory outside the project, and `worklist --verify` judges the filled reports against the list it wrote `##worklist-writes-and-verifies-the-reports`

`klarch worklist --out <dir>` writes `agents.txt`, one line per agent and entry with its role and
its seed mark, and per agent `entries.md`, its entries whole, and `report.md`, one line per entry
to grade, and `shown.md`, the snapshot of `design@core@graded-entries-assembled`. It is a writer, so it refuses over an incomplete model, per
`design@core@phases-gate-the-report`, and it refuses a directory inside the project, whose files the
walk would read, a directory that holds anything, and a seed that names no entry, writing nothing.
`klarch worklist --verify <dir>` reads no model: it judges each report against `agents.txt`, and
reports an entry graded nowhere or twice, a reference no agent was given, a grade or a relation
outside its role's set, a missing reason, an entry that is or cites a seed graded `unrelated`, and
a report with no closing section on what was met outside the task. The grades and their relations
are `design@agent-skills@search-relevance-grade`'s. When every report passes, it assembles the graded
entries, per `design@core@graded-entries-assembled`.

It serves `goal@core@records-reach-their-reader` and `goal@core@document-rules-are-checked`:
an instruction to grade every entry once is kept by a command, not by a reader. The nearest rival,
each searcher deriving its group from the listings and writing its report by hand, listed one entry
under two lists in the discussion that produced the `rejected` register, and only the searcher's own
correction, appended to its report, showed it. The command's output is read by its own `--verify`
and the searcher, and is no contract with another reader, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`.

### A passing `worklist --verify` assembles the graded entries, fenced, from a snapshot `--out` takes of every entry's show text `##graded-entries-assembled`

`klarch worklist --out` writes `shown.md`, the snapshot: every entry of the record with the text
`show` prints for it, its inbound list included, computed by `show`'s own code. No agent is given
it, so an agent's load is its entries file alone. When every report passes, `--verify` writes from
it `bears.md`, every entry whose highest grade over every report is `bears`, and `may-bear.md`,
every entry whose highest grade is `may-bear`, after a table of contents of their reasons; an entry
graded `unrelated` by every report is in neither. Above each entry stand every agent's bearing grade
lines, identical lines merged, then its text, fenced as the snapshot fences it so that a heading of
the text reads as no section of the file; a goal is its grade lines and its site alone, since the
design grounding reads every goals home whole, per `design@agent-skills@searcher-grades-goals`, and
the other readers read a goal with `show`. The entries keep the order of `agents.txt`. A run that
fails, or stops on a file it cannot read, leaves no assembly, a previous run's included; a snapshot
missing, cut short or lacking an entry of the list, and a write that fails, stop it with exit 2 and
no assembly, per `design@core@exit-code-ladder`. `--verify` still reads no model, so the session
reads what the searchers graded, whatever the tree has become since.

It serves `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` and
`goal@core@records-reach-their-reader`. The nearest rival, the session running `show` once per
graded entry, is a script each session writes again: the grounding of the discussion that made this
decision did so over 202 entries, 576 KB of `show` output, as the message of the commit "[harvest]
The graded entries' assembly, `klarch id`, the narrower grounding and the retrospective's new
question recorded" states. Its other rival, an assembly that reads the model when `--verify` runs,
either makes `--verify` read a model, which a tree that gained a finding of the first three phases
since `--out` would then block, or adds a third command step to every grounding, for an inbound list
current to the minute: `rejected@core@gather-reads-the-model`. The files are read by `--verify`, the
searcher and the session, and are no contract with another reader, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`.

### The groups are packed into the fewest agents first-fit-decreasing gives, spread evenly, under a bound of 25,000 words, a group above it alone `##clusters-packed-under-a-word-bound`

A group's load is the words of its entries. A group above the bound goes to an agent alone, never
split, and `worklist` prints a notice naming it. The others are packed first-fit-decreasing, which
fixes the number of agents, then dealt again over that many agents, each to the least-loaded agent
it fits in; where one fits in none, the first-fit assignment stands. `--bound` sets another bound.
It serves `goal@knowledge-architect@agents-get-a-complete-workflow`: each agent's reading is bounded
whatever the project's size, and no agent is spent on a remnant.

The owner set the bound at 25,000 words, on the owner's judgement that loads of about 22,000 words
put not much of a strain on this repository's searchers, and holds raising it to 30,000 open for
later; no measurement of recall stands behind any value, and real sessions measure it, per
`goal@knowledge-architect@the-workflow-improves-through-real-use`. Words rather than entries or
bytes: in the owner's estimate, words translate a bit better to an agent's tokens; an entry's mean
size ranged from 0.5 KB, a `rejected` entry, to 2.9 KB, an issue, over this repository at ed9dd60,
so a count of entries bounds a load poorly, while words and bytes differed by about 15% across
kinds. First-fit-decreasing uses at most 11/9 of the optimal number of agents plus 6/9, by the bound
György Dósa proved tight in 2007; over this repository at ed9dd60 it used the lower bound, 9 agents,
and over thaum at edd2cb5d it used the lower bound for the groups within the bound, 8 agents, beside
3 groups above it sent alone. The spreading keeps an agent from carrying the full bound while
another carries a remnant: first-fit alone filled this repository's agents to a median of 24,737
words of a 25,000 bound, as the scripts the message of the commit "[review] Repairs of the transcript review of one search of the whole record, with the scripts of its measurements" holds took it. Re-take the packing with `klarch worklist --out` and `--bound` over this repository; a run whose
agents sit at the bound reopens the spreading. Splitting a cluster to meet a bound would cut a
subject, which the grouping exists to keep whole. The designs this decision did not keep:
`rejected@core@packing-to-the-bound`, `rejected@core@bound-in-entries`,
`rejected@agent-skills@one-agent-reads-every-standing-entry`,
`rejected@agent-skills@design-listing-cut-in-groups-of-sixty` and
`rejected@agent-skills@subject-groups-bounded-at-sixty`.
