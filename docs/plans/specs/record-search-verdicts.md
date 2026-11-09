# The record search reports checkable verdicts, packed by search unit, and reads the draft of the plan

## Status and audience

This spec is the plan of work done on one branch and one pull request: it changes the record
search, `klarch worklist` and the installed texts that dispatch it and act on its output. It leaves
when its work lands. Where it and a design home disagree on a point the section "What is already
decided" does not list as reversed or rewritten, the design home wins. Every name it uses is
defined in it or exists in the code. Where the owner's word is awaited, as its section "Defaults
awaiting the owner" lists, `skill@knowledge-architect-planning@working-a-slice` says what waits.

It is assembled from one session's transcript, the session whose discussion begins with the owner's
message "I want to discuss the design record search procedure and the design audit step." Its
rounds 2 to 10 hold a side task, the drafting of `goal@agent-skills@session-orchestrates-the-work`,
which landed on its own; the threads below cite that side task only where a ruling in it bears on
them.

## How the work is done

`skill@knowledge-architect-planning`, as the work of a spec.

## Names

- **the search**: the record search: `klarch worklist --out <dir>`, one agent per agent directory it
  writes, then `klarch worklist --verify <dir>`. Its code is `path@query@src/worklist.rs`
  (`worklist`, `write`, `render`, `snapshot`, `verify`, `assemble`, `judge`) and
  `path@query@src/graph.rs` (`pack`); its command line is `path@cli@src/cli/mod.rs`
  (`DEFAULT_WORKLIST_BOUND`).
- **the searcher**: `agent@knowledge-architect-record-searcher`, whose source is
  `path@agent-skills@content/agents/record-searcher.md`.
- **a search unit**: a Component or a location the manifest declares; the unit of packing.
- **a subject of the work**: a part of the work the dispatching session names in the brief: a
  thread at the design grounding, a finding cluster at a retrospective intake, the slice or the spec
  at a design audit.
- **a cluster**: a design subject with the `rejected` entries its heads cite and the standing entries
  that cite them, per `design@query@cluster-rooted-at-a-subject`, whose own word for a design
  subject is "unit".
- **a context head**: a head of another search unit that an entry of an agent cites, given to that
  agent read only.
- **a verdict**: one line of a searcher's report on one entry, shaped as the section "Decided design"
  says.
- **the verdict file**: what `--verify` writes for one subject of the work in place of `bears.md`
  and `may-bear.md`.
- **the draft**: the draft of the document that will carry a design's deliberation, the search's
  work at the design grounding.
- **the main session**: as `goal@agent-skills@session-orchestrates-the-work` defines it.

## What the work is

What exists today at each site:

- `klarch worklist --out` builds clusters rooted at a design subject, each with the goals its heads
  cite and every goal those refine (`design@query@goals-join-a-cluster-as-context`); the entries no
  cluster holds form a residual group per anchor (`design@query@residual-grouped-by-anchor`); the
  groups are packed first-fit decreasing and spread under 25,000 words
  (`design@query@clusters-packed-under-a-word-bound`; `DEFAULT_WORKLIST_BOUND` is 25000).
- The searcher grades each entry `bears` with a relation, `may-bear`, or `unrelated`
  (`design@agent-skills@search-relevance-grade`); `--verify` checks the reports against
  `agents.txt` and assembles `bears.md` and `may-bear.md` (`design@query@graded-entries-assembled`).
- The design skill's loop step 1 runs the search with the problem and the drafted shapes as the
  work, once the scope settles and before any thread closes, and reads `bears.md` whole and
  `may-bear.md` on touch (`design@agent-skills@may-bear-read-on-touch`). The design audit of a slice
  reads both files whole, seeded with every decision its milestone lists, and its commit message
  gives one line to each graded entry. The intake reads every issue and tripwire graded `bears` or
  `may-bear` whole.

The measurements the design rests on, each with its instrument, are in the arguments, most of them
in `argument@record-search-verdicts@arg-7`, `argument@record-search-verdicts@arg-22`,
`argument@record-search-verdicts@arg-36` and `argument@record-search-verdicts@arg-51`.

Outside the work:

- a lighter pass for the Components a piece of work does not touch,
  `issue@agent-skills@a-lighter-pass-for-untouched-components`, left open on the owner's planned
  project of about 30 Components, per `thread@record-search-verdicts@lighter-pass-untouched`;
- a fuller partitioned procedure of the intake, parked as
  `thread@record-search-verdicts@intake-partitioned-procedure`;
- where the judgement of a dispatch is stated, in
  `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`, per
  `thread@record-search-verdicts@dispatch-judgement-home`;
- the rewording of `goal@agent-skills@only-needed-instructions`, which the owner asked for in round
  14, a goal-setting item of its own.

## What is already decided

Rested on, not argued again: `goal@agent-skills@session-orchestrates-the-work`,
`goal@knowledge-architect@agents-work-without-drift`,
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
`goal@query@the-record-is-consulted-as-the-workflow-requires`,
`design@agent-skills@search-reads-the-whole-record`,
`design@agent-skills@no-thread-closes-before-the-search`,
`design@query@record-is-every-register-entry`, `design@agent-skills@synthetic-evidence-not-built`.

Reversed or rewritten, each judged or updated at step 5, the harvest, with every text that
`cargo klarch show` lists as citing it read there:

- `query`: `design@query@cluster-rooted-at-a-subject` (a standing entry judged beside a context
  head's full text), `design@query@goals-join-a-cluster-as-context`,
  `design@query@residual-grouped-by-anchor`, `design@query@clusters-packed-under-a-word-bound`,
  `design@query@worklist-writes-and-verifies-the-reports`, `design@query@graded-entries-assembled`;
  the rewritten heads keep citing `rejected@query@bound-in-entries`,
  `rejected@query@gather-reads-the-model`, `rejected@query@goals-pull-their-citers`,
  `rejected@query@one-residual-group` and `rejected@query@packing-to-the-bound`.
- `agent-skills`: `design@agent-skills@one-record-searcher`,
  `design@agent-skills@searcher-grades-goals`, `design@agent-skills@search-relevance-grade`,
  `design@agent-skills@may-bear-read-on-touch`,
  `design@agent-skills@unread-entries-reach-the-conformance-review`,
  `design@agent-skills@seeds-name-decisions`,
  `design@agent-skills@search-waits-until-the-scope-settles`,
  `design@agent-skills@record-searched-before-the-work`.
- the root: `design@knowledge-architect@query-crate` (a second use of `git` in `query`).
- brought back with a new argument: `rejected@agent-skills@one-search-agent-per-anchor`.
- a reason defeated, its entry rewritten: `rejected@agent-skills@conformance-reader-at-the-checkpoint`.
- reworded: `tripwire@agent-skills@owner-proposal-built-ungrounded`.

Closed by the work: `issue@agent-skills@the-search-s-graded-entries-exceed-what-one-session-reads-whole`,
`issue@agent-skills@a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone`,
`issue@agent-skills@the-lines-owed-per-graded-entry-are-written-by-a-script-at-every-occasion`,
`issue@agent-skills@the-root-goals-home-is-larger-than-its-design-home`,
`issue@agent-skills@the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`,
`issue@query@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`.

The discussion ran two searches, both in the verdict shape: the first with a work statement as its
work, the second with the draft of this spec. Each entry the second graded other than `unrelated`
follows, with its verdict, its action and its outcome; where the outcome is the searcher's reason, the
searcher's reason is given, and where the discussion answered it, the answer. The first search's
findings were answered in round 13 and are in `argument@record-search-verdicts@arg-38`.

- `design@agent-skills@verification-in-primer`: constrains, respect. the main session acts on verdicts "checked as its own claims are"; the quotation is the reported evidence and the session still re-verifies a load-bearing verdict before acting on it
- `design@agent-skills@code-change-reads-its-record`: constrains, respect. cites a rested-on decision; the draft's search on the in-change path and for bounded work stays within the design skill's occasions and adds no search to a fix
- `design@agent-skills@questions-go-to-their-authority`: conflicts, respect. Answered: `owner-question` takes every question `primer@question-authority` reserves to the owner.
- `design@agent-skills@owner-word-holds-as-far-as-its-premise`: constrains, respect. every thread is held on the owner's word until the search; a verdict whose entry defeats a premise of that word must still reach the design skill's routing, which the actions must not bypass
- `design@agent-skills@met-outside-the-task-outcomes`: constrains, respect. the verdict report keeps its "Met outside the task" section, which the main session routes
- `design@agent-skills@decisions-reported-to-the-owner`: constrains, respect. the searcher now resolves every doubt the record answers; a resolution the owner could have decided must reach the main session's "Decided" section, which the report shape names no place for
- `issue@agent-skills@the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`: closes, close-with-the-work. listed as closed; `may-bear` and its reading on touch are gone, and the search reading the draft answers the case of reasons graded against a work as first drafted
- `design@agent-skills@losing-alternatives-filter`: constrains, respect. the harvest writes `rejected` entries for the losing alternatives "that meet its tests"; the rest stay in the spec and the commit message
- `design@agent-skills@losing-shape-test`: constrains, respect. `thread@record-search-verdicts@search-work-carries-the-deliberation` is superseded; the harvest judges its shape by the filter if it lost, and keeps it in the spec only if `thread@record-search-verdicts@search-reads-the-draft` carries it whole
- `design@agent-skills@rejected-citation-direction`: constrains, respect. the rewritten packing head covers the brought-back `rejected@agent-skills@one-search-agent-per-anchor`, and the heads rewritten for the verdicts and the draft cover their new `rejected` entries and `rejected@agent-skills@conformance-reader-at-the-checkpoint`
- `design@agent-skills@subject-split-standard`: constrains, respect. cites a head the work rewrites; the rewrite must keep the notice naming a cluster above the bound, now 40,000 words with titles and the work counted, and this head is re-pointed at assembly
- `design@agent-skills@thread-slug-is-entry-id`: constrains, respect. the harvest rewrites heads in place; a thread whose decision earns a new entry, such as `thread@record-search-verdicts@bound-raise`, which names a change, takes a slug naming the decision with the pair stated in the harvest row
- `rejected@agent-skills@design-listing-cut-in-groups-of-sixty`: constrains, respect. a unit above the bound is split by whole clusters, never inside one, so no subject is cut
- `rejected@agent-skills@one-agent-reads-every-standing-entry`: constrains, respect. the 40,000-word bound and the split of a large unit keep each agent's load within its context; packing by unit does not put every standing entry in one agent
- `rejected@agent-skills@one-search-agent-per-anchor`: brings-back, new-argument-needed. Brought back with a new argument, per `thread@record-search-verdicts@component-packing`: the split above the bound answers its reason; it leaves the rejected home at the harvest.
- `rejected@agent-skills@subject-groups-bounded-at-sixty`: constrains, respect. the verdict report and the rewritten packing head keep the searcher's report of a strained subject and the notice of a cluster above the bound; no numeric limit cuts a subject
- `design@agent-skills@audit-method`: constrains, none. only cites `design@agent-skills@synthetic-evidence-not-built` for an audit's edits; the search's verdicts propose no edit, so 'no re-reading by rule' does not meet it
- `design@agent-skills@refused-report-returned-as-text`: constrains, respect. the rewritten searcher keeps its text fallback on a refused write; the head's own argument says a long text result can reach the session cut, and the verdict report with a quotation per line is longer than today's, so a refused write is the case where a cut text would fail --verify
- `design@agent-skills@run-counts-computed-by-the-workflow`: constrains, none. only cites `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`; it binds the audit workflows' counts, and the counts of verdicts are --verify's, by the same principle
- `issue@agent-skills@audit-axes-beyond-the-design-record`: touches, none. also cites `goal@knowledge-architect@agents-work-without-drift`; with residual groups gone and every goal in goal agents, this lead on goal coverage becomes false and is rewritten at the harvest
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: touches, respect. generated lines in checked forms remove the hand-copied reasons, one occasion of this mechanism; but a verbatim quotation copied into a commit message can carry spans the checker reports (a slug mid-line, a reference to an entry the branch renames), the narrowed-goals instance's case, so the generated lines must keep quotations in a form the checker accepts
- `issue@agent-skills@the-lines-owed-per-graded-entry-are-written-by-a-script-at-every-occasion`: closes, close-with-the-work. --verify writes the per-entry lines and the audit's fit lines in checked forms, and may-bear's 'not read' lines at the other three texts disappear with may-bear; the close also owes the shape recorded in a head the four texts cite
- `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule`: constrains, none. only cites `design@agent-skills@synthetic-evidence-not-built`; its subject is the audit's saved workflows, which no shape changes
- `design@agent-skills@milestone-split-by-lifetime`: constrains, respect. the generated gap and fit lines for the audit's commit message must leave room for each gap's answer and the decision it follows from; the head also makes a new slice found at an audit a scope change the owner rules on, which the gap class scope-change must keep
- `design@agent-skills@in-change-path`: constrains, respect. on the in-change path the draft the search reads is the commit message's draft, which this head names the deliberation's home; the shape keeps that home
- `design@agent-skills@design-hands-off-to-planning`: conflicts, check-reversal. Answered: the draft is written under the planning skill, as a new step of its "When this skill starts", and the design skill's loop step 1 points to it; the design skill writes no plan document.
- `design@agent-skills@ledger-from-transcript`: conflicts, check-reversal. Answered: the draft is the search's work only; the spec is assembled from the transcript after convergence.
- `design@agent-skills@completeness-check-is-a-step-claim`: constrains, none. only cites `goal@knowledge-architect@agents-work-without-drift`; it binds the acceptance criteria the draft leaves to assembly, nothing in the shapes meets it
- `design@agent-skills@plan-landing-is-not-tied-to-its-work`: constrains, respect. cites `goal@knowledge-architect@agents-work-without-drift`; the work changes query, cli and installed text, not the checker the per-commit gate runs, so D1 (spec and work on one branch) is outside the exception
- `design@agent-skills@plan-records-the-exchange`: constrains, respect. the draft written before the search holds shapes, not the exchange; the plan that follows it still owes every thread with its arguments and verbatim rulings
- `design@agent-skills@premortem-as-watch-points`: constrains, none. only cites `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`; its evidence includes half the searchers of one search filling their reports by script, which a longer verdict line could repeat; the standing question stays
- `design@agent-skills@tripwire-recording-authority`: constrains, respect. the draft's Harvest lists no outcome for its six premortem causes; each is judged by the standard at the harvest and listed with its outcome
- `design@agent-skills@standing-state-review-before-every-merge`: constrains, none. only cites `design@agent-skills@record-searched-before-the-work`, whose occasions the shapes keep; the standing-state review stays the reader of every tripwire and deferred trigger beside the searcher's verdicts
- `design@agent-skills@plan-read-against-the-record`: conflicts, respect. Answered: the design-conformance review still reads the plan against the goals, design and rejected homes, and reads the verdict file beside them.
- `design@agent-skills@unread-entries-reach-the-conformance-review`: conflicts, check-reversal. Rewritten at the harvest: the reviewer reads the verdict file beside the homes, and at the merge of an in-change or bounded branch the axis is owed wherever the grounding's verdict file holds any verdict but `unrelated`.
- `design@agent-skills@a-scratch-directory-per-subagent`: constrains, respect. the new goal agents and unit agents each get an agent directory of their own as scratch directory, as cluster agents do; the rewritten searcher description keeps the rule
- `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed`: touches, none. the shapes remove the list of entries left unread, one of the conditions this issue's instances name; the condition under which the conformance axis is owed at a merge changes with the verdict file
- `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`: touches, none. trigger-met covers a design issue's re-entry point and the searcher no longer passes a doubt up, which answers the thaum instance at the search's occasions; but the thaum entry was of kind question, so trigger-met should cover a re-entry point of any kind, and work outside the search's occasions still meets no reader, so the issue stays open
- `design@agent-skills@bounded-path-in-design`: constrains, respect. the draft the search reads is written before the search, when whether the work is bounded is not yet known; the draft must not presuppose its final form (plan document or commit message), and the bounded classification still follows the search
- `design@agent-skills@frontier-tier-only`: constrains, none. only cites `design@agent-skills@synthetic-evidence-not-built`, a rested-on decision; nothing in the shapes meets the design skill's model tier
- `design@agent-skills@extension-follows-the-ground`: constrains, respect. this head's guard rests on the conformance reviewer reading every plan against the heads; the shapes must keep that reading, not only the verdict file, and the plan reviews must not shrink to shape and structure
- `issue@agent-skills@a-lighter-pass-for-untouched-components`: trigger-met, rule-before-work. Re-entry point met. The discussion keeps the whole-record search, on the owner's agreement, and leaves the issue open on the owner's planned project of about 30 Components; the harvest rewrites its re-entry point, which names an issue this work closes.
- `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`: constrains, none. guards `design@agent-skills@record-searched-before-the-work`, a rested-on decision whose occasions the shapes keep; its firing clause is not met
- `tripwire@agent-skills@owner-proposal-built-ungrounded`: conflicts, check-reversal. Reworded at the harvest: its firing clause names the entries left unread, a record the verdict shape removes; it names the generated per-entry lines instead.
- `design@agent-skills@ruled-items-labelled`: constrains, respect. an owner-question the main session puts to the owner is such an item and takes a label, Q where it reaches no committed document
- `design@agent-skills@standing-argument-in-head`: constrains, respect. the heads harvested from these threads rest on measurements taken by 'a script over the agents' entries.md' and transcript peaks; each needs a command a reader can run, or the script kept in the message of a commit the head names by subject
- `design@agent-skills@heads-stand-on-argument`: constrains, respect. `thread@record-search-verdicts@bound-raise` rests on the owner's report that agents stay reliable to about 250,000 tokens, and `thread@record-search-verdicts@component-packing` on the owner's judgement that a large Component's entries interact; each lands by its kind, the report as an attributed premise
- `design@agent-skills@fork-choice-in-weak-form`: constrains, respect. the owner's choice of 40,000 over 50,000 and 30,000, and of the half-bound rule over packing into large units' room, are fork choices recorded in the weak form with their rivals and costs, reopening only on an argument their record does not hold
- `design@agent-skills@owner-rules-at-a-fork`: constrains, respect. the draft holds every thread presumed-settled until the search has run, as this head says; the draft-before-search order keeps it
- `design@agent-skills@default-ruled-before-merge`: constrains, respect. D1 of the draft, and any owner-question a verdict raises that the work proceeds on with a default, must be ruled before the branch merges
- `tripwire@agent-skills@owner-fact-recorded-unattributed`: constrains, respect. the `thread@record-search-verdicts@bound-raise` head will carry the owner's report on reliability to about 250,000 tokens; written unattributed, it would fire this tripwire
- `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`: touches, none. `thread@record-search-verdicts@verdict-report` has the conformance review judge the work against every verdict, the second question of goal@agent-skills@reviews-judge-what-no-tool-can, and changes the merge-time condition; the first question, cited where it bears, stays unnamed
- `design@agent-skills@one-record-searcher`: conflicts, check-reversal. listed as rewritten; goal agents now grade the goals apart from the cluster agents, and the reading of `bears.md`/`may-bear.md` the head names gives way to verdict files; the rewrite keeps one agent per cluster for heads, `rejected` entries, issues and tripwires, which the shapes keep
- `design@agent-skills@search-apart-from-the-standing-state-review`: constrains, respect. the verdicts `fires` and `trigger-met` and the audit's standing-entry gaps stay in the searcher's relevance judgement; the standing-state reviewer keeps its read-every-tripwire rule and no shape merges the two
- `design@agent-skills@search-reads-the-whole-record`: constrains, respect. rested on; every entry is still graded once by some agent; its title's "every residual group" and its cost pointer to the rewritten `design@query@clusters-packed-under-a-word-bound` become stale once residual groups go, so the harvest rewords them as texts citing a rewritten head
- `design@agent-skills@searcher-grades-goals`: conflicts, check-reversal. listed as rewritten; goals reach cluster agents as titles and goal agents grade each goal once; the owner's ruling that both readings stay is kept by the shape, and its cost sentence is re-argued on the 36% measurement
- `design@agent-skills@search-relevance-grade`: conflicts, check-reversal. listed as rewritten; `may-bear` is gone and relations become verdicts; the rewrite must also restate its seed rule ("where it meets no element of the change, it is `may-bear`"), which the draft does not say, and its routing of a goal to goal-setting
- `design@agent-skills@may-bear-read-on-touch`: conflicts, check-reversal. listed as rewritten; the main session acts on verdicts with no re-reading by rule; the owner's earlier ask is replaced by the owner's word that `goal@agent-skills@session-orchestrates-the-work` records, per `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`, so the record answers it
- `design@agent-skills@search-waits-until-the-scope-settles`: conflicts, check-reversal. listed as rewritten for the work's form: the work becomes the draft of the plan document or commit message; the moment, the one search, and the rule against a delta search stay as the shape keeps them
- `design@agent-skills@no-thread-closes-before-the-search`: constrains, respect. rested on; the draft keeps every thread held presumed-settled until the search, and "Findings revise the draft" must route a defeated word as this head says, not by amending the draft alone
- `design@agent-skills@seeds-name-decisions`: conflicts, check-reversal. listed as rewritten; its argument against a goal seed rests on goals joining as context and on the forced `may-bear` grade, both changed; `--seeds-from <plan>` must also say which plan documents of a slice it reads (the slice spec alone or with the milestone), which this head's "the decisions its plan documents cite" decides
- `design@agent-skills@record-searched-before-the-work`: conflicts, check-reversal. listed as rested on for its occasions only, but this sentence states the work's form, which `thread@record-search-verdicts@search-reads-the-draft` changes to the draft; and its "the slice's spec and the milestone document as the work and the decisions they cite as seeds" changes if `--seeds-from` reads the slice spec alone; both sentences go to the rewrite list
- `rejected@agent-skills@conformance-reader-at-the-checkpoint`: conflicts, check-reversal. Its reason, rulings already given, is defeated by `design@agent-skills@no-thread-closes-before-the-search`; the harvest rewrites its reason and the two heads citing its cost.
- `rejected@agent-skills@delta-search-at-convergence`: constrains, respect. `--since <commit>` stays a mark over a whole-record search; used as a filter it would bring this design back
- `rejected@agent-skills@design-grounding-reads-every-may-bear-entry-whole`: constrains, none. only cites the rewritten `design@agent-skills@may-bear-read-on-touch`; no shape reads every graded entry whole; the harvest keeps a head citing it or lets it leave
- `rejected@agent-skills@goal-seeds`: constrains, respect. `--seeds-from` takes no goal, as required; its reason rests on goals joining as context and on `may-bear`, both changed, so the harvest restates it on the new shape
- `rejected@agent-skills@intake-reads-every-may-bear-entry-whole`: constrains, none. cited by the rewritten `design@agent-skills@may-bear-read-on-touch`; the intake no longer reads graded entries whole by rule, so nothing brings it back; the harvest keeps a head citing it or lets it leave
- `rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`: constrains, respect. an `owner-question` must name the readings and what each changes, never a doubt naming nothing, as the shape states
- `rejected@agent-skills@search-after-the-first-challenge`: constrains, respect. the search still runs once the shapes settle, now over the draft
- `rejected@agent-skills@search-before-and-after-the-challenge`: constrains, respect. the draft keeps one search per discussion
- `rejected@agent-skills@search-work-is-the-question`: constrains, respect. the draft as the work holds the problem ("What the work is"), the shapes and the losing alternatives, never the question alone; its doubt stays watched by the tripwire
- `rejected@agent-skills@searches-split-by-kind`: constrains, respect. goal agents split off goals only, which are cluster context, not members; but packing by unit gives an issue that cites another unit's head that head by title alone, the half-cluster reading this reason weighs, traded on the 100-of-341 measurement
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: constrains, none. only cites the rested-on `design@agent-skills@record-searched-before-the-work`; no shape makes the search read pending files
- `issue@agent-skills@a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone`: closes, close-with-the-work. `--seeds-from` and the end of the forced `may-bear` grade close it, but its close also needs the rule chosen: the draft must state whether a slice's seeds are the milestone's whole list or the slice's own and rewrite the two heads; under `thread@record-search-verdicts@slice-audit-checks-fit` the main session reads gaps only, so the whole list no longer reaches it
- `issue@agent-skills@a-spec-worked-without-its-design-audit-missed-a-critical-gap`: constrains, none. only cites the rewritten `design@agent-skills@searcher-grades-goals`; the draft keeps the condition on which a spec's audit runs ("where one runs")
- `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`: touches, none. the work repairs the record-search departure; the entry stays open for the others, and its first bullet is removed when the work lands
- `issue@agent-skills@the-root-goals-home-is-larger-than-its-design-home`: closes, close-with-the-work. its measure was taken (goals 36% of words read), not small, so the decision went back to the owner, and goals as titles with goal agents answer it
- `issue@agent-skills@the-search-s-graded-entries-exceed-what-one-session-reads-whole`: closes, close-with-the-work. verdicts checked by `--verify` replace the whole reading; the owner's word it names as deciding is the one the goal records
- `tripwire@agent-skills@record-search-misses`: constrains, respect. not fired by the shapes; it guards the rewritten `design@agent-skills@one-record-searcher` and names `bears`, `may-bear` and the named doubt, so its premise and response are reworded with the rewrite; the premortem rests on it for `unrelated` misses and the raised bound
- `design@agent-skills@goals-change-through-goal-setting`: constrains, respect. Answered: a `conflicts` verdict on a goal routes to the goal-setting skill, never to `check-reversal`.
- `design@agent-skills@no-instruction-for-a-goal-read-two-ways`: conflicts, respect. Answered: `owner-question` carries no rule of its own for a goal read two ways; it takes what `primer@question-authority` reserves to the owner.
- `design@agent-skills@goal-edit-ruled-alone`: constrains, respect. the rewording of `goal@agent-skills@only-needed-instructions` stays its own goal-setting item, never among the spec's defaults, as the draft says
- `rejected@agent-skills@ledger-written-during-discussion`: constrains, respect. the draft is written once the shapes settle, as the search's work, and revised by its findings; kept current every round with thread states, it would be this ledger
- `design@agent-skills@instruction-record-is-minimal`: constrains, respect. the harvest records the decisions (report format, packing, occasions), not the wording of the searcher's or the skills' new text
- `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect. the searcher's new instructions (resolve doubts, check verdicts) rest on the sessions of the graded-entries issue and the owner's goal; the prototype searches' standing as real use is `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`
- `design@agent-skills@synthetic-evidence-not-built`: constrains, respect. rested on; the acceptance criteria, to be written, test the command over mock projects and observe no searcher's verdicts; the token peaks are a load measure, but "three verdicts turned on its wording" is an agent's decision, see the census issue
- `design@agent-skills@capabilities-not-structure`: constrains, respect. one verdict file per unit of the work is the command's output, and the draft leaves the division among agents to the session; the installed text must not map a file to one agent
- `design@agent-skills@statement-names-an-act`: constrains, respect. step 4 rewrites the searcher agent and four skills; each new statement passes the need test
- `design@agent-skills@term-defined-at-first-load`: constrains, respect. Answered: the packing unit is named a search unit, and a unit of the work a subject of the work.
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`: depends-on, none. Answered for this work by the owner's ruling, recorded in `design@agent-skills@synthetic-evidence-not-built`: the searches of this discussion are real use. The issue stays open on its census case.
- `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect. the generated per-entry lines put references in commit messages; each line references an entry the work argues from, such as a `fits` or a gap, and not one graded `unrelated`
- `design@agent-skills@a-past-sentence-is-rewritten`: constrains, respect. the harvest deletes six issues and rewrites heads that other entries cite, such as the dispatch `todo` entry's list of the search's own entries; each dangling sentence is rewritten to the present
- `design@agent-skills@plain-text-is-no-repair`: constrains, respect. the generated lines name the `--since` commit by its subject, cite the plan whole and never a bare item name, and put every reference outside the quotation, all forms this head admits
- `rejected@agent-skills@checked-commit-form-by-subject`: constrains, respect. the `--since` commit named by its subject in the generated lines stays prose; no shape adds a check that resolves it
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`: trigger-met, rule-before-work. Re-entry point met, not taken: its question is where the claim check is delivered at a commit, which this work does not change; the generated lines only remove one class of hand-written claims.
- `issue@check@the-self-pin-refusal-has-no-discriminating-test`: constrains, none. it only cites goal@knowledge-architect@agents-work-without-drift, which the draft rests on; nothing in the shapes meets the self-pin refusal
- `design@check@a-commit-message-is-a-document`: constrains, respect. the per-entry lines and fit lines --verify generates for a commit message are prose under the regime, the quotation included, so every span they carry, a quoted one too, must resolve against the commit's tree or its first parent's
- `design@check@branch-shas-are-refused`: constrains, respect. the --since commit in the audit's generated lines is named by its subject, as the shape says, since a branch commit's SHA is refused in a message under refuse-branch-shas
- `design@check@orphan-rejected-entry`: constrains, respect. the harvest writes rejected entries for the losing alternatives and brings back `rejected@agent-skills@one-search-agent-per-anchor`; each rejected entry kept must stay cited by a design head's section, or the check fails
- `design@check@goals-home-cites-upward-only`: constrains, none. it only cites `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, which the draft rests on; no shape writes into a goals home or changes the citation rule
- `design@check@no-citation-direction-rule`: constrains, none. it only cites `design@query@record-is-every-register-entry`, which the draft rests on; the clusters packed inside a unit keep reading an edge either way, so its argument stands
- `issue@check@a-home-for-developer-contracts-outside-agent-configuration`: constrains, none. it only cites goal@knowledge-architect@agents-work-without-drift, which the draft rests on; no shape routes developer contracts or scoped CLAUDE.md files
- `issue@check@no-checked-form-cites-a-point-inside-a-section`: touches, respect. step 4 edits the design skill's loop step 1 and its order with the draft, and the entry lists the loop step numbers cited in words across the design skill; a renumbering must re-check each such citation by hand
- `issue@check@install-agent-skills-exits-two-after-writing`: constrains, none. it only cites goal@knowledge-architect@agents-work-without-drift, which the draft rests on; step 4 runs the install, but no shape changes how it fails
- `design@check@plan-document-sections-by-kind`: constrains, respect. Respected: this spec holds the sections Arguments and New names, in one place.
- `design@check@retired-slug-reference-reported`: constrains, respect. The `against` field names threads as #<slug>; carried into a generated commit line it must stay plain, never backticked, or `commits` reports the retired form.
- `design@check@reference-shape-outside-span`: constrains, respect. The generated per-entry lines and gaps in "checked forms" go into a commit message that `commits` judges; a quote that cuts a backtick span leaves a reference-shaped token outside backticks unless it sits in a quotation.
- `design@check@backticked-bare-id`: constrains, respect. A generated commit line that writes an entry's or item's id bare in backticks, rather than its reference, is a finding of `commits`; the "checked forms" must be full references.
- `design@check@plain-word-lint`: constrains, respect. Entry text quoted in a generated commit line can hold a defined id as a plain word (a skill name, a slug), a finding unless the quote is a quotation; the reason and action fields written by the searcher too.
- `design@check@quotation-read-by-no-id-lint`: constrains, respect. The per-entry lines --verify generates for a commit message carry each quotation of entry text; only inside a quotation so delimited are the lints on ids silent on it, so the generator must delimit each quote as one, and an entry text holding a `"` of its own would end the quotation early.
- `issue@check@no-command-applies-the-repair-a-message-finding-names`: touches, none. Generated per-entry lines in checked forms remove the audit-message instances of the respelling loop, without giving `commits` a repair mode, so the issue stays open.
- `design@check@every-path-names-its-anchor`: constrains, respect. The audit's generated lines cite the plan whole: that citation must be `spec@plans@<id>` or `milestone@plans@<id>`, never a path to the plan file that `--seeds-from <plan>` reads.
- `design@check@declared-command`: constrains, respect. The read-only context heads of other units are read "with `show` for its text", and the verdict files name commands; what worklist writes into entries.md and the verdict files names the project's declared command, not a fixed `klarch`.
- `issue@check@a-plain-hash-item-name-is-read-by-nothing`: touches, none. The `against` field names units of the work as plain #<slug>; where the generated lines carry it into a commit message they add plain item names nothing checks, while `thread@record-search-verdicts@slice-audit-checks-fit` already cites the plan whole and names no bare item; the shapes do not change the retired-slug head, so the re-entry point is not met.
- `issue@check@the-tests-read-the-machine-s-git-configuration`: worsens, respect. Step 3 tests `--since <commit>` over mock projects in `testing`, which build git histories; through today's helpers each new test reads the machine's git configuration, adding to the failing set unless it uses an isolating helper.
- `rejected@check@split-enforced-by-a-check`: constrains, respect. The bound moves to 40,000 on the owner's report of reliability to about 250,000 tokens and a peak-token measurement, still no recall measurement; a unit above the bound is split among agents, and nothing in the shapes gates a tree on it, which keeps this design rejected.
- `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`: constrains, none. only cites the mechanical-tasks goal the draft rests on; its subject, the review's worktrees and scratch directories, is untouched by the shapes
- `design@knowledge-architect@checker-asks-git-through-the-crate`: constrains, respect. `worklist --since <commit>` asks git which entries changed since a commit; query must ask it through a function of the `git` crate
- `design@knowledge-architect@model-crate`: constrains, respect. the quotation check, --since and --seeds-from stay within phases 1 to 3 and run no check
- `design@knowledge-architect@git-crate`: constrains, respect. the git question behind `--since` (files changed since a commit) is added to the `git` crate as a new function, not written in query
- `design@knowledge-architect@query-crate`: conflicts, check-reversal. `--since <commit>` through `git` widens query's use of git beyond the last column of `issues`; the draft already lists this head as rewritten
- `design@knowledge-architect@cli-crate`: constrains, respect. DEFAULT_WORKLIST_BOUND, `--since` and `--seeds-from` are declared in cli and their logic stays in query
- `design@knowledge-architect@mock-projects-in-one-crate`: constrains, respect. the implementation sequence tests the packing, the verdict check and the new options over mock projects in `testing`, as this head places them
- `issue@knowledge-architect@the-checker-may-depend-on-every-crate-below-it`: constrains, none. only cites `design@knowledge-architect@query-crate`, which the draft rewrites; the shapes add no dependency of `check`
- `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`: constrains, none. only cites the `goal@knowledge-architect@agents-work-without-drift` goal the draft rests on; the shapes add no crate and change no publish order
- `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes`: constrains, none. only cites the `goal@knowledge-architect@agents-work-without-drift` goal the draft rests on; the draft's figures were measured in this repository, so the two-homes question does not arise for them
- `issue@knowledge-architect@the-mechanical-tasks-goal-omits-what-a-helper-does-better`: constrains, none. criterion mechanical-by-command rests on the goal as worded today; packing, the quotation check, --since, --seeds-from and the generated lines follow with no judgement, so the goal covers them without the rewording
- `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`: constrains, none. only cites the mechanical-tasks goal the draft rests on; the shapes rename and move nothing
- `design@knowledge-architect@versioning-policy`: constrains, respect. the work changes the `worklist` command (default bound, output files replaced, new options) and the installed skills; the harvest's changelog entries take their bump class from this table
- `design@knowledge-architect@changelog-entries`: constrains, respect. the harvest's "Migration where the report format changes" owes an entry only for a change to a consumer's own files; the report and verdict files are written in scratch space, and the new files and actions are Workflow entries
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect. the harvest's CHANGELOG entries are written on the spec's one branch, not at the release
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: touches, none. the shapes replace `worklist`'s output files (bears.md, may-bear.md by verdict files and generated lines); worklist declares its output no contract, so the change is classed under that declaration and the issue's policy question stays open
- `design@knowledge-architect@no-branch-sha-is-cited`: constrains, respect. the generated fit lines name the `--since` commit by its subject, as the shape already says; the command must never print its SHA into those lines
- `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`: constrains, respect. the audit's generated per-entry fit lines, where the audit applies nothing in place, go in the slice's first commit, as the planning skill already says, never in a commit of their own
- `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`: constrains, none. only cites the mechanical-tasks goal the draft rests on; `--since` reads history and rewrites none
- `design@model@model-facade`: constrains, respect. Whatever `query` newly needs of `model` (an anchor's kind as Component or declared location, the unit of an entry, the plan's resolved citations) is published in lib.rs's role modules because `query` consumes it, shown by an integrated test.
- `design@model@plan-document-kinds`: constrains, respect. The generated lines cite the plan by its kind, `spec@plans@<id>`, `milestone@plans@<id>` or a slice's `spec@<milestone>@<slice>`, never by path; `--seeds-from <plan>` names the plan the same way or by path only as a command argument.
- `design@model@plan-item-scope`: constrains, respect. The per-entry and fit lines `--verify` generates for a commit message may not cite a thread or other item by reference; the plan is cited whole with `#<id>` beside it, as the shape already states.
- `design@model@declarable-directory-register`: constrains, respect. An entry of a Directory register is named under the anchor carrying the home, and what it carries under the entry anchor; the shape's rule assigning both to the unit holding the register's home agrees and must stay so.
- `design@model@a-reference-is-live-in-every-prose`: constrains, respect. A quotation carried into the generated commit-message lines is prose, so every reference inside it is resolved by `commits`, and a quotation cut through a backticked span leaves an unpaired backtick; the generator must keep quoted text in a form that holds whole spans.
- `design@model@anchors-are-components-and-locations`: conflicts, respect. Answered: a search unit is each Component and each location the manifest declares; a constructed location belongs to the search unit holding its home, per `thread@record-search-verdicts@component-packing`.
- `design@model@anchor-collisions-refused`: constrains, respect. The packing unit of a location nested in a Component is the location, by the deepest-anchor rule; the half-bound rule then weighs the location and the Component separately.
- `design@model@one-entity-table`: constrains, respect. `--seeds-from <plan>` must take the plan's cited entries from the model's resolution of its references, not from a parse of its own.
- `design@model@an-entity-belongs-to-its-anchor`: constrains, respect. "An entry belongs to the unit that holds it" must take the deepest anchor as owner, so an entry of a location inside a Component packs with the location, not the Component.
- `issue@query@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`: closes, close-with-the-work. listed as closed by the work; with every entry in one unit's agent, every goal graded once by a goal agent and one verdict per entry, no entry carries a line per agent
- `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: touches, none. cites a head the work rewrites; `--since <commit>` marks the entries changed in a range, the issue's first work list asks the texts citing them, which the mark does not give; the cited sentence on seed marks is re-read when the head is rewritten, since the new report line carries no seed field
- `issue@query@the-report-skeleton-names-no-relation-its-entry-s-role-admits`: touches, none. the work replaces the grade and relation fields by a verdict and an action and rewrites the skeleton, but as drafted the line names no verdict the role admits (brings-back only for `rejected`, fires only for a tripwire), so the issue's defect carries over to verdicts unless the skeleton names them; it also cites `design@agent-skills@one-record-searcher` and `design@agent-skills@search-relevance-grade`, both rewritten
- `design@query@query-depends-on-no-check`: constrains, respect. the quotation check, the verdict files, `--since` and `--seeds-from` are built in `query`, so none of them may depend on or run a check
- `design@query@record-is-every-register-entry`: constrains, respect. rested on; `--seeds-from <plan>` reads a plan only to take seeds, the draft read as the work is never graded as an entry, and `--since` marks no code
- `design@query@cluster-rooted-at-a-subject`: conflicts, check-reversal. Answered: a standing entry that cites a head of another search unit gets that head's full text, counted in the bound, so it is judged beside the heads it cites; the head's word for a design subject keeps its sense, and the packing unit is named a search unit.
- `design@query@goals-join-a-cluster-as-context`: conflicts, check-reversal. listed as reversed; goals reach a cluster agent as title lines only and are graded by goal agents
- `design@query@residual-grouped-by-anchor`: conflicts, check-reversal. listed as reversed; the entries no cluster holds are packed with their unit, and its answer to `rejected@agent-skills@one-search-agent-per-anchor` moves to the split above the bound
- `design@query@worklist-writes-and-verifies-the-reports`: conflicts, check-reversal. listed as rewritten; `--verify` judges verdicts, actions and quotations instead of grades and relations, `entries.md` holds goal and context titles, and the brief names units of the work
- `design@query@graded-entries-assembled`: conflicts, check-reversal. listed as rewritten; one verdict file per unit of the work replaces both files; its rule that `--verify` reads no model stays and binds the quotation check and the gaps, which read `shown.md`
- `design@query@clusters-packed-under-a-word-bound`: conflicts, check-reversal. listed as rewritten; the bound becomes 40,000 and the load counts titles and the draft; its spreading over the agent count is not restated in `thread@record-search-verdicts@component-packing` and is kept unless argued away, per `rejected@query@packing-to-the-bound`
- `rejected@query@bound-in-entries`: constrains, respect. the bound stays in words; the rewritten packing head must still cite this entry, or it is left uncited
- `rejected@query@gather-reads-the-model`: constrains, respect. the quotation check, the verdict files and the audit's gaps are computed by `--verify` from the snapshot, never from a model read again; the rewritten assembly head must still cite this entry
- `rejected@query@goals-pull-their-citers`: constrains, respect. goal agents read goals alone and bring in no citer; the head citing this entry is reversed, so the rewritten head must still cite it or it is left uncited
- `rejected@query@one-residual-group`: constrains, respect. the entries of a unit pack with that unit, not into one project group; its citing head is reversed, so the rewritten head must still cite it or it is left uncited
- `rejected@query@packing-to-the-bound`: constrains, respect. Respected: the spreading over the agent count is kept for the search units below half the bound.
- `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`: touches, none. the work rewrites the report line, the form this issue asks to change, but as drafted it keeps a filled skeleton with more fields, so the issue stays open; its sentence giving the line shape `grade:
- `issue@query@no-command-lists-the-entries-two-commits-differ-by`: touches, none. `worklist --since <commit>` computes which entries changed between a commit and the tree, a mark inside the search, not the listing asked; its sentence that `commits` is the only command reading a given commit's tree turns false with the work
- `design@git@git-is-asked-through-one-module`: constrains, respect. `--since <commit>` reads history through `git`; any invocation it needs that the library lacks is added to that one module as an `Invocation`, not run from `query`
- `design@git@failed-git-is-an-error`: constrains, respect. a failed history read behind `--since` must stop the run with git's reason, not mark no entry as changed, since the audit reads the mark as the set of changed entries
- `design@cli@command-parsing-home`: constrains, respect. `--since <commit>` and `--seeds-from <plan>` are declared in `cli` and their logic takes typed values in `query`, as step 3 of the draft places them
- `design@cli@arguments-parse-through-clap`: constrains, respect. `--since` and `--seeds-from` belong to `--out`, so their combination with `--verify` is refused by the clap declaration, with a test
- `design@cli@exit-code-ladder`: constrains, respect. a quotation not found or a verdict outside its set is a negative answer, exit 1; an unreadable report, a `--since` commit git cannot resolve or a `--seeds-from` plan that cannot be read is exit 2
- `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation`: touches, none. `thread@record-search-verdicts@bound-raise` changes that constant to 40000 and the README's figure with it, the place where a test of the default bound would land; the worklist.rs mutations it lists sit in code the work rewrites
- `issue@cli@some-value-checks-are-made-after-parsing`: touches, none. the new values `--since <commit>` and `--seeds-from <plan>` name something in the project, so by this issue's own line they are checked by the tool, not while parsing, and do not widen the issue
- `issue@testing@a-test-run-leaves-directories-in-the-temporary-directory`: worsens, respect. the work's claims are tested over mock projects in `testing` with the worklist helpers, whose output directories are among those left behind; each new test built on them adds more unless the helper removes what it creates
- `design@testing@binary-tests-in-testing`: constrains, respect. the tests of the packing, the verdict check and `--since` over mock projects, `--since` needing a repository with history, sit in `testing`, as step 1 of the draft says
- `goal@agent-skills@no-contradiction-in-workflow`: constrains, respect. step 4 edits the searcher, the design, planning and intake skills and the conformance reviewer at once; each must name the verdict files and the new order, none still bears.md, may-bear.md or the reading on touch
- `goal@agent-skills@only-needed-instructions`: constrains, respect. one verdict file per unit of the work is a command output, not the record, and the division among agents stays the session's judgement; the per-entry fit lines of the audit's commit message are a fixed mapping into the record, kept only while no other instruction needs to vary it; agent counts come from the bound, not a fixed number
- `goal@knowledge-architect@agents-get-a-complete-workflow`: constrains, respect. the ten verdicts, the six actions and the audit's fit and gap classes must each have one reading in the searcher and in the skills that act on them
- `goal@knowledge-architect@agents-work-without-drift`: constrains, respect. rested on; every entry and every goal is still graded once against the draft, goals by goal agents reading them whole, and the conformance review judges the work against every verdict but unrelated
- `goal@knowledge-architect@design-with-agents-as-counterparts`: constrains, respect. the owner-question field carries a goal's reading or a fork to the main session, which puts it to the owner; the searcher resolving other doubts keeps those out of the exchange only where the record answers them
- `goal@knowledge-architect@documentation-stays-consistent`: constrains, respect. the verdict files --verify writes are the listing the conformance review reads; the quotation check stays mechanical and judges no verdict's meaning
- `goal@knowledge-architect@owner-attention-on-what-is-theirs`: constrains, respect. an owner-question states the readings and what each changes in the shapes, so the main session can put it with its consequences established
- `goal@knowledge-architect@projects-are-split-into-components`: constrains, respect. packing by unit must serve a project of one Component, whose entries are then split by cluster above the bound; the shape does so
- `goal@knowledge-architect@serves-one-kind-of-project`: constrains, respect. --since <commit> relies on the git ask, which every target project follows
- `goal@knowledge-architect@the-owner-decides`: constrains, respect. the searcher answers what the draft and the record answer, and sends to the owner only a goal's reading or a fork; the decisions it makes reach the owner through the session's Decided section
- `goal@knowledge-architect@the-owner-s-word-is-verified`: constrains, respect. the bound rests on the owner's report of reliability to about 250,000 tokens, checked against the prototype's peaks of 143,134 to 204,976; the owner's judgement that a large Component's entries interact is backed by the 100 of 341 placements measured
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: constrains, respect. the new searcher instructions rest on the searcher transcripts of 2026-11-07 and the prototype run inside this project's own design discussion, which is real use; no mock run is to be built to test them
- `goal@agent-skills@installed-text-works-in-every-target-project`: constrains, respect. the criterion any-project derives from it; units, the half-bound rule and the 40,000 bound must work with one Component and with a large location such as thaum's rules, and the searcher text names no path of this repository
- `goal@agent-skills@one-skill-per-activity`: constrains, respect. the units of the work and the draft as the work are given to the searcher by the design, planning and intake skills; each skill, or the shared agent definition, must state what its dispatch gives
- `goal@knowledge-architect@every-target-project-can-adopt-it`: constrains, respect. the 40,000 bound and the half-bound rule are measured on this repository and must stay generic defaults, naming none of its Components
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: constrains, respect. rested on; the quotation check, the verdict files, the per-entry commit lines, the --since mark and the --seeds-from seeds are written by worklist, not by a session's script
- `goal@testing@a-consumer-test-uses-what-a-consumer-can`: constrains, respect. the claims of steps 1 to 3, tested over mock projects in testing, reach worklist through the published command or public API, publishing nothing only for the test
- `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: constrains, respect. the conformance review judges the work against the entry each verdict names, which binds the writer; a verdict or an action is no criterion of its own
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: constrains, respect. the harvest rewrites the listed heads and must carry the losing alternatives and their reasons, in the head or a cited rejected entry
- `goal@knowledge-architect@records-reach-their-reader`: constrains, respect. the check-reversal action routes to that reading, and the review before a merge keeps both questions
- `goal@cli@one-home-for-the-command-line`: constrains, respect. --since, --seeds-from and DEFAULT_WORKLIST_BOUND are declared in cli; whether the commit or the plan exists is the tool's answer, not the parser's
- `goal@git@a-pure-wrapper-over-git`: constrains, respect. --since takes from git the paths changed since the commit; which entries changed is decided in query
- `goal@model@a-tool-reads-the-documents-with-no-check`: constrains, respect. the unit of each entry, a directory register entry's included, is read through model, with no check attached
- `goal@query@agents-query-the-project`: constrains, respect. --verify reads the reports of the work list it generated; --since reads the git history, which the goal admits
- `goal@query@the-record-is-consulted-as-the-workflow-requires`: constrains, respect. rested on; goals are graded once by goal agents and their titles in cluster agents are not graded; --verify still refuses a report that leaves an entry out
- `goal@testing@program-tests-over-whole-projects-have-one-home`: constrains, respect. the mock-project tests of steps 1 to 3 sit in testing with their projects
- `goal@cli@a-refused-invocation-does-no-work`: constrains, respect. a combination of --since or --seeds-from that the command refuses writes nothing in the --out directory
- `goal@agent-skills@reviews-judge-what-no-tool-can`: constrains, respect. a passing --verify checks quotations and coverage only; the conformance review still asks both questions against each verdict's entry
- `goal@agent-skills@session-orchestrates-the-work`: conflicts, check-reversal. Answered: every `unrelated` line carries a one-line reason, so every conclusion carries evidence.

## Criteria

### Every entry the work bears on is judged against it before the work `##c-bearing-reaches-a-judge`

Binding, from `goal@knowledge-architect@agents-work-without-drift`, whose measure admits that the
reading is done "by a step the installed text owes an agent or a reviewer". Met by
`thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@component-packing` and
`thread@record-search-verdicts@search-reads-the-draft`.

### A conclusion acted on is checked as the main session checks its own claims `##c-claims-verified`

Binding, from `primer@verify-before-relying` and `goal@agent-skills@session-orchestrates-the-work`.
Met by `thread@record-search-verdicts@verdict-report`: the command checks each quotation, and the
searcher checks its conclusions.

### What reaches the main session grows with what changes the work `##c-session-load-bounded`

Weighed when named in round 1; binding once `goal@agent-skills@session-orchestrates-the-work` was
written, per `argument@record-search-verdicts@arg-31`. Met by
`thread@record-search-verdicts@verdict-report` and `thread@record-search-verdicts@occasion-shaped-output`.

### The agent count grows with distinct entries, not with duplication `##c-agents-scale-with-record`

Weighed, from the owner's second flaw in round 1. Met by
`thread@record-search-verdicts@goal-context-once` and `thread@record-search-verdicts@component-packing`.

### Mechanical tasks are done by a command `##c-mechanical-by-command`

Binding, from `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`. Met by
`thread@record-search-verdicts@verdict-report` (the quotation check and the generated lines) and
`thread@record-search-verdicts@slice-audit-checks-fit` (`--since`, `--seeds-from`).

### It works in a project of one Component and in a project with a large location `##c-any-project`

Binding, from `goal@agent-skills@installed-text-works-in-every-target-project`. Met by
`thread@record-search-verdicts@component-packing`: a project of one Component is one search unit,
split by its clusters above the bound, and a location the manifest declares, such as thaum's
`rules`, is a search unit of its own.

## Threads

### The main session orchestrates the work, a goal `##orchestrating-session`

Proposed by the owner, round 1; framed as a goal by the session, round 1
(`argument@record-search-verdicts@arg-15`); closed `approved` by the owner in round 2: "agreed this
is a goal. I think your draft is fairly correct with my intent. The reviews will judge any problem
with it. I think we should draft it as a goal in this session, so that it can shape the direction of
the discussion more reliably." Drafted and ruled in rounds 2 to 9 under the goal-setting skill, and
written as `goal@agent-skills@session-orchestrates-the-work`. Carried by no section of this spec: it
landed. Harvest: done.

### Goals reach the cluster searchers as titles, and goal searchers grade them `##goal-context-once`

Proposed by the session, round 1 (`argument@record-search-verdicts@arg-9`), on the measurement of
`argument@record-search-verdicts@arg-7`. Owner, round 2: "goal-context-once: I agree with this."
Revised in round 13 from one goal searcher to goal agents packed under the bound
(`argument@record-search-verdicts@arg-37`); owner, round 14: "goal-context-once: agreed to pack under
the same bound, by component. This is clearly needed if goal size exceeds the bound". Its ground for
the revision is the bound, not `goal@agent-skills@only-needed-instructions`, which the owner showed
misapplied (`argument@record-search-verdicts@arg-40`, `argument@record-search-verdicts@arg-42`).
Closed `approved` by the owner's word against the checkpoint, round 20: "agreed on the table". Shape:
"Decided design". Harvest: `design@query@goals-join-a-cluster-as-context`,
`design@agent-skills@searcher-grades-goals`.

### The bound is 40,000 words, counting everything an agent reads `##bound-raise`

Proposed by the owner, round 2 (`argument@record-search-verdicts@arg-16`): "I would also consider
raising the word cap, say to 30k." The session proposed 50,000 (`argument@record-search-verdicts@arg-21`).
Owner, round 3: "about bound-raise: I think we can go with a middle ground of 40k first. Because we
are also demanding more detailed results of searchers, I think we can raise incrementally, rather
too much at once. This should leave plenty more ground." Measured at 40,000 by both searches
(`argument@record-search-verdicts@arg-36`, `argument@record-search-verdicts@arg-51`). Closed
`approved`, round 20. Shape: "Decided design". Harvest:
`design@query@clusters-packed-under-a-word-bound`.

### The packing unit is a search unit, a Component or a declared location, alone above half the bound `##component-packing`

Proposed by the owner, round 1 (`argument@record-search-verdicts@arg-5`); argued by the session
(`argument@record-search-verdicts@arg-10`, `argument@record-search-verdicts@arg-22`). Owner, round 2:
"component-packing agreed. In particular, an agent that ends up judging an issue of a component of
which it saw no design, would be de major defect IMO." Owner, round 3, proposing one agent per
Component, split when too large (`argument@record-search-verdicts@arg-26`). Owner, round 12, the
half-bound rule (`argument@record-search-verdicts@arg-33`): "if a component exceeds half of the bound,
always alone (and split if it  exceeds the bound). Only components below half of the bound are
packed together." Owner, round 14 (`argument@record-search-verdicts@arg-41`): "I think each location
should be treated as one small component (or not necessarily small) for that." Answered by the
second search: declared locations only, and the full text of context heads
(`argument@record-search-verdicts@arg-50`, `argument@record-search-verdicts@arg-51`). Closed
`approved`, round 20. The owner chose the half-bound rule over packing small units into the room of
large ones, which no goal ranks (`argument@record-search-verdicts@arg-34`). Shape: "Decided design".
Harvest: `design@query@clusters-packed-under-a-word-bound`,
`design@query@residual-grouped-by-anchor`, `design@query@cluster-rooted-at-a-subject`; the removal of
`rejected@agent-skills@one-search-agent-per-anchor`.

### The searcher returns a verdict per entry, its quotation checked by the command `##verdict-report`

Proposed by the owner as a direction, round 1 (`argument@record-search-verdicts@arg-4`); shaped by the
session (`argument@record-search-verdicts@arg-11`, `argument@record-search-verdicts@arg-12`). Owner,
round 2: "verdict-report: I think this is a good shape." Reopened by the session in round 15 on a
conditional verdict consumed as unconditional (`argument@record-search-verdicts@arg-45`), with an `if:`
field; the owner argued parity of context instead (`argument@record-search-verdicts@arg-46`) and the
session withdrew the field for `owner-question` (`argument@record-search-verdicts@arg-47`). Owner,
round 17: "verdict-report agreed." Its members from the second search
(`argument@record-search-verdicts@arg-50`). Closed `approved`, round 20. Shape: "Decided design".
Harvest: `design@agent-skills@search-relevance-grade`, `design@agent-skills@may-bear-read-on-touch`,
`design@agent-skills@one-record-searcher`, `design@agent-skills@unread-entries-reach-the-conformance-review`,
`design@query@worklist-writes-and-verifies-the-reports`, `design@query@graded-entries-assembled`.

### One verdict file per subject of the work, and the dispatch over them left to the main session `##occasion-shaped-output`

Proposed by the session, round 1 (`argument@record-search-verdicts@arg-13`). Owner, round 2: "I am not
convinced this fully solves the problem. For the intake, I believe a significantly different
procedure, with more steps to partition the work, might be neeeded. But today's scope shuld stay
relatively limited, so we can go with that at first". Reopened by the session in round 13 on a reading
of `goal@agent-skills@only-needed-instructions` (`argument@record-search-verdicts@arg-37`), which the
owner contested in round 14 (`argument@record-search-verdicts@arg-40`); the session read the goal
whole and withdrew its revision (`argument@record-search-verdicts@arg-42`). Closed `argued` in round
14: one verdict file per subject of the work, the owner's earlier word on the first shape standing,
minus the sentence sending each finding cluster to one subagent, which
`goal@agent-skills@session-orchestrates-the-work` rules out. Shape: "Decided design". Harvest:
`design@query@graded-entries-assembled`.

### The design audit of a slice checks that the spec still fits `##slice-audit-checks-fit`

Proposed by the owner, round 2 (`argument@record-search-verdicts@arg-19`): "The goal is to verify that
the spec fits in the curent design and code (and fix if it does not), not to redo the work the design
session did." Shaped by the session (`argument@record-search-verdicts@arg-23`). Owner, round 3:
"slice-audit-checks-fit: agreed". Members from both searches (`argument@record-search-verdicts@arg-38`,
`argument@record-search-verdicts@arg-51`). Closed `approved`, round 20. Shape: "Decided design".
Harvest: `design@agent-skills@seeds-name-decisions`, `design@agent-skills@record-searched-before-the-work`.

### The search's work at the design grounding is the draft of the plan `##search-reads-the-draft`

Proposed by the owner, round 17 (`argument@record-search-verdicts@arg-48`): "Maybe the best shape would
be to have a plan document already written". Argued by the session
(`argument@record-search-verdicts@arg-49`). Owner, round 18: "agreed, draft the plan and run the search
on it". Answered by the second search (`argument@record-search-verdicts@arg-50`): the draft is
written under the planning skill and is never the spec. Owner, round 19, on the prototype run: "For
the question, I would count that as real use." Closed `approved`, round 20. Shape: "Decided design".
Harvest: `design@agent-skills@search-waits-until-the-scope-settles`,
`design@agent-skills@record-searched-before-the-work`, the reason of
`rejected@agent-skills@conformance-reader-at-the-checkpoint`.

### The extracted transcript as the search's work `##search-work-carries-the-deliberation`

Proposed by the owner, round 16 (`argument@record-search-verdicts@arg-46`); costed by the session
(`argument@record-search-verdicts@arg-47`). Owner, round 17: "the cost does seem a bit too high here.
I am not convinced this is worth it." Closed `superseded` by
`thread@record-search-verdicts@search-reads-the-draft`. Section: "Losing alternatives".

### A lighter pass for untouched Components is not taken `##lighter-pass-untouched`

Raised by the owner, round 1 (`argument@record-search-verdicts@arg-5`); the session did not propose it
(`argument@record-search-verdicts@arg-14`, `argument@record-search-verdicts@arg-38`). Owner, round 14:
"Agreed on the other items". Closed `approved`: the whole-record search is kept, and the issue stays
open on its other re-entry point. Harvest: the re-entry point of
`issue@agent-skills@a-lighter-pass-for-untouched-components`.

### A partitioned procedure of the intake `##intake-partitioned-procedure`

Raised by the owner, round 1 (`argument@record-search-verdicts@arg-3`) and round 2
(`argument@record-search-verdicts@arg-18`); parked by the session in round 2
(`argument@record-search-verdicts@arg-24`). State: `parked`. Cause: the next intake the owner runs still
loads its session beyond one reading. Re-entry point: the retrospective of that intake.

### Where the judgement of a dispatch is stated `##dispatch-judgement-home`

Raised by the owner, round 8 (`argument@record-search-verdicts@arg-30`). State: `parked`, outside this
work. Cause and re-entry point: the work that closes
`issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`, which holds
the owner's leads.

## Arguments

### Too many entries graded bearing `##arg-1`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@occasion-shaped-output`
- "the search is often leading to far too many entries that are rated bear. Nearly all of the record is often rated bear or may bear. This results in loads that are far too much for one session."

### More searchers because goals grew `##arg-2`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@component-packing`
- "since the goals have started becoming fuller, especially with the component split that led to more goals overall, much more searchers now need to be dispatched."

### The search fits the design skill less than the audit and the intake `##arg-3`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@intake-partitioned-procedure`
- "I think that the design search is relatively well suited for the design skill, especially now that it runs later. But I think it is less adapted to design audit, and especially to the intake."
- "The intake may have a deeper problem in itself, mostly because its nature has it handle lots of unrelated small problems, hence it often ends up covering the full design record."

### Richer verdicts than bears or may-bear `##arg-4`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@verdict-report`
- "I think the search would be more valuable if the results were more detailed than bear or maybear. Since we are dispactching all of these subagents, we could ask them to give stronger opinions and conclucsions on what actions must be taken related to the read entries."

### Pack by Component, give only enclosing goals `##arg-5`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@lighter-pass-untouched`
- "packing entries in a way that reduce the count of spanned component for each searcher, and give only the goals of components that are in parents of the entries a searcher got."

### The main session as orchestrator `##arg-6`

Round 1, the owner, at L6.

- Threads: `thread@record-search-verdicts@orchestrating-session`, `thread@record-search-verdicts@verdict-report`
- "I would like to move the installed workfolw toward something were the main session acts more like an orchestrator, dispatching subagents to work or analyze or review, then collecting results and reports and acting one them (with double checking)."
- "we need to move away from the curretn shape that requires the main session to read the full bear entries. This probably goes with giving the searcher's words more weights."

### Measurement of the 25-agent work list `##arg-7`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@component-packing`
- Source: `cargo klarch worklist --out` over main at 350d4d59, a script over the agents' `entries.md`; `wc -w` on goals homes.
- "`cargo klarch worklist --out` over main now gives **25 agents**." "total words the 25 searchers read | 497,444"; "of which goal entries | 181,458 (36%)"; "all 25 root goals are in all 25 agents"; "Components spanned per agent | 2 to 7"; "goals homes, `wc -w` | 17,886 words; the root's alone 8,544".
- "**About a third of every searcher's load is the same goals, copied 25 times.** That comes from `design@query@goals-join-a-cluster-as-context`"
- "about 85,000 words of issues and `rejected` entries sit in more than one cluster."

### The whole reading stands on the owner's earlier ask only `##arg-8`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@orchestrating-session`
- "The goal does not require the session itself to read. Its text says the bearing parts "are read against the work by a step the installed text owes **an agent or a reviewer**". So the whole reading stands on your earlier ask alone, and your message today changes it."
- "`primer@verify-before-relying` treats a report as a lead, never as evidence. So "giving the searcher's words more weight" has to mean words the session can check cheaply, not words it trusts."

### Goal titles plus one goal searcher `##arg-9`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@goal-context-once`
- "Cluster searchers get each goal as its title line only, about 20 to 40 words. A title states the goal. One extra **goal searcher** reads every goals home whole (17,886 words, one agent under the bound) and grades every goal against the work."
- Estimate: "232,000 words of distinct entries plus shared duplicates, at 25,000 per agent, gives about 13 cluster agents plus one."
- Rival: "your idea of giving each searcher only the goals of the Components its entries sit in. It still copies the root's 8,544 words into every agent, because every Component goal refines a root goal."

### Packing by Component gains coherence `##arg-10`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@component-packing`
- "The gain is coherence more than words. A searcher holding one Component's whole design home grades each head against its neighbours, so a contradiction or a "may change" between two heads of the same Component is seen by one reader."
- Cost: "a cluster with edges into two Components, such as an issue citing heads in `check` and `model`, still goes to one agent."

### The quoted verdict with a closed action `##arg-11`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@verdict-report`
- "**`quote` is verbatim, and `klarch worklist --verify` checks it** against the snapshot `shown.md` it already holds. A paraphrase cannot pass as a quotation."
- "**The session reads whole only the entries whose action changes the work**: a conflict, a design brought back, a firing, a closing, a dependency."
- Measured precedent: "about 30 of 202 entries changed a proposal (commit "[harvest] The graded entries' assembly, `klarch id`, …")."

### May-bear removed `##arg-12`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@verdict-report`
- "**`may-bear` goes away as a grade.** A doubt becomes a `conflicts` verdict whose `against` names the element of the work."
- Record cited: "80 of the 137 entries graded `may-bear` alone gave as reason that no bearing was established" (`rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`).
- Cost: "The searcher's judgement becomes load-bearing for entries graded `constrains` and `unrelated`." "the five entries missed in `tripwire@agent-skills@record-search-misses` were all graded so."
- Rival: "Rival 1 of the open issue: fresh readers over parts, each writing one line per entry. It is the same trust, with a second set of agents and no quote check."

### Output partitioned by the unit of the work `##arg-13`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@slice-audit-checks-fit`
- "Its work is N unrelated findings, and today the union of their neighbourhoods lands on one session. Recorded instances: 333 `bears` plus 249 `may-bear` entries for the forty-file intake."
- "seed with the slice's decisions only, read from the plan documents by a `worklist --seeds-from <plan>` option. Recorded instance: "about 110 of 125 `may-bear` entries graded so for citing a seed" in thaum."

### The lighter pass not proposed `##arg-14`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@lighter-pass-untouched`
- "Taking out the goal duplication roughly halves the agent count without dropping any entry from a full reading, so the case for a lighter pass is weaker than when it was parked."

### Orchestration is an outcome, so a goal `##arg-15`

Round 1, the session, at L163.

- Threads: `thread@record-search-verdicts@orchestrating-session`
- "This states an outcome, not a shape. Per `primer@design-heads`, it lands as a **goal**, drafted under `skill@knowledge-architect-goal-setting`. Once written, it ranks shapes that no goal ranks today."

### Raise the bound; reliability and cost `##arg-16`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@goal-context-once`
- "I would also consider raising the word cap, say to 30k. My observation is that searchers right now go up to around 130k to 150k tokens."
- "So the main metric to consider is actually cost, which is split between flat upfront costs and agent need to start [...] versus increasing costs of each round, which are reduced due to prompt caching."
- "token usage starts to ramp up more noticeably around 250k tokens. So I think we can aim for more content per searcher, and more detailed conclusions, as long as these searchers end their tasks around 250k tokens."

### An issue judged without its Component's design is a defect `##arg-17`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@component-packing`
- "an agent that ends up judging an issue of a component of which it saw no design, would be de major defect IMO. The searcher would need to reason on very little info to judge, or to go read the related design. I think this was a major miss in the current design."

### The intake needs more than per-unit files `##arg-18`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@intake-partitioned-procedure`
- "For the intake, I believe a significantly different procedure, with more steps to partition the work, might be neeeded. But today's scope shuld stay relatively limited, so we can go with that at first, and see how the situation improved the next time I run an intake."

### The slice audit verifies fit, not redesign `##arg-19`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@slice-audit-checks-fit`
- "the audit is much less needed when the spec was recently designed. It is important when work has landed between the design and the satrt of the work."
- "it should not put a lot of weight on the main session, especially when the findings are low and easy to fix, or even require basicall no action. The goal is to verify that the spec fits in the curent design and code (and fix if it does not), not to redo the work the design session did."

### Draft the goal in this session `##arg-20`

Round 2, the owner, at L172.

- Threads: `thread@record-search-verdicts@orchestrating-session`
- "I think we should draft it as a goal in this session, so that it can shape the direction of the discussion more reliably."

### Token fit over 40 transcripts; bound of 50,000 proposed `##arg-21`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@bound-raise`
- Source: token usage of the 40 most recent `agent@knowledge-architect-record-searcher` transcripts, all from 2026-11-07; "Peak context" is input plus cache plus output on the last turn.
- "The fit is about **36,000 tokens of fixed start plus 3.3 tokens per word read**. These runs peak lower than the 130k to 150k you saw."
- "**I propose a bound of 50,000 words**, which predicts about 200k tokens plus the verdict output: under 250k, with margin. Your 30,000 is the conservative rival. It predicts about 135k tokens and keeps the agent count about 1.7 times higher."
- "With goals as titles (`thread@record-search-verdicts@goal-context-once`), 50,000 words gives about 6 to 8 agents over this repository, against 25 today."

### 100 of 341 issue placements without their design `##arg-22`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@component-packing`
- Source: the 25-agent work list.
- "of 341 placements of an issue or a tripwire, **100 sit in an agent that holds no design entry of their own Component**. That's 99 issues and 1 tripwire. The largest counts are the root (22), `check` (19) and `model` (15)."
- "**an agent that holds any entry of a Component holds that Component's whole design home** [...] Applied directly, this conflicts with the bound for `agent-skills` (115,616 words of entries)."

### The audit's narrower question and shape `##arg-23`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@slice-audit-checks-fit`
- "The audit's question is narrower than the design's: **does the spec still fit the record and the code as they stand now?**"
- "the whole record is still read, every cluster by some searcher, so this does not bring back `rejected@agent-skills@delta-search-at-convergence`. [...] Here `--since` is a mark that draws attention, not a filter."
- "An audit with no gap costs it one count line, so the case you described, a spec designed recently, is cheap without any rule about when to skip."
- "**I recommend the generated lines**: the reason stands, and once a command writes the lines, their cost is message length only." Recorded count: "365 lines and 45 checker findings for one slice".

### The intake procedure parked `##arg-24`

Round 2, the session, at L219.

- Threads: `thread@record-search-verdicts@intake-partitioned-procedure`, `thread@record-search-verdicts@occasion-shaped-output`
- "The fuller intake procedure is parked as `thread@record-search-verdicts@intake-partitioned-procedure`. Its cause: the next intake you run still loads its session beyond one reading. Its re-entry: that intake's retrospective."

### 40k as a middle step `##arg-25`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@bound-raise`
- "I think we can go with a middle ground of 40k first. Because we are also demanding more detailed results of searchers, I think we can raise incrementally, rather too much at once."

### One agent per Component as the stronger rival `##arg-26`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@component-packing`
- "with the solution you are proposing, we might as well just drop the packing, and simply dispatch one agent per component. I think most issues are actually fine to judge without the design. The hard case is one issue that cites a design head. But I believe that case is already packed with others actually ?"
- "Maybe with a split of the component once volume goes to high, since one component can actually end up very big even under the split rules. Such as agent skills, which has a single subject, but can end up packing lots of design"

### This session's search need not be read whole `##arg-27`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@verdict-report`
- "For the current session, I think we can try to apply a different procedure. Such as giving more isntructions to the search agents, maybe even trying out the verdic shape you proposed, even if we have to bypass the checked verification. I don't think it is worth it to have you read the whole bear entries."

### F3, the new word replaces the earlier ask `##arg-28`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@orchestrating-session`
- "F3: agreed, my new word replaces the previous one. I've had a change of mind of what I expect of the main session, the goal we are drafting carries my new intent."
- "F2 design only"

### Trust in the main session equals trust in an agent `##arg-29`

Round 3, the owner, at L273.

- Threads: `thread@record-search-verdicts@orchestrating-session`, `thread@record-search-verdicts@verdict-report`
- "my take is that the main session is just as reliable as any dispatched agent, so there is no need to give more trust to a subagent. [...] So the reauirements for the main session to verify the subagent work should not  be stronger (or weaker) than the requirements of the main session to verify its own claims."

### How to judge a dispatch `##arg-30`

Round 8, the owner, at L412.

- Threads: `thread@record-search-verdicts@dispatch-judgement-home`
- "Is the next coherent piece of work large enough that it would spend too much of this sessions context and make finishing the complete piece of work at hand (the slice) difficult ? Is there a need to investigate a subject were only the conclusions matter, or write a large code probe ? Is there a need for a fresh point of view ?"
- "Maybe we should add it to the primer, that would probably make the goal met in good part."

### The new goal makes the load criterion binding; reading only what it reasons over `##arg-31`

Round 11, the session, at L611.

- Threads: `thread@record-search-verdicts@orchestrating-session`, `thread@record-search-verdicts@verdict-report`
- "The weighed criterion #c-session-load-bounded becomes **binding**"
- "So the session reads an entry whole only where it must itself reason over the entry, for instance to reverse it. Reading an entry just because the work changes because of it is no longer required."

### An issue citing a head is packed with it; the Component as unit `##arg-32`

Round 11, the session, at L611.

- Threads: `thread@record-search-verdicts@component-packing`
- "**Yes.** `design@query@cluster-rooted-at-a-subject` puts an issue in the cluster of every head it cites. So the case you called hard is covered today, but only by the cited head, not by the rest of its Component's design."
- "This answers the reason behind `rejected@agent-skills@one-search-agent-per-anchor`, "an anchor bounds nothing", because the split above the bound restores the bound."
- Measurement (session's script over today's tree, bound 30,000): "9 agents"; "`agent-skills` in 4 agents, `check` in 2"; "read-only context heads per agent from 7 to 76"; "the largest agent at 56,000 words, context included."
- "With a fixed start of about 36,000 tokens per agent, eight agents for Components of 464 to 4,800 words each would spend more on starting than on reading."

### Half-bound rule `##arg-33`

Round 12, the owner, at L613.

- Threads: `thread@record-search-verdicts@component-packing`
- "if a component exceeds half of the bound, always alone (and split if it  exceeds the bound). Only components below half of the bound are packed together. Large components are likely to be more critical to the project, so splitting attention is not optimal IMO. And their entries probably have lots of interactions together."
- "This is a bit arbitrary though, I have no measurement to back that idea."

### Half the bound is the one point that does only what is intended; cost measured `##arg-34`

Round 12, the session, at L621.

- Threads: `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@bound-raise`
- "Two Components that each exceed half the bound can never share an agent anyway, since together they exceed the bound. So at exactly half, your rule adds a single constraint: no small Component joins a large one."
- Measurement over today's tree at 40,000 words: "agent-skills | 115,191"; "check | 37,185"; "model | 26,742"; "knowledge-architect (root) | 19,838"; "the 10 others | 463 to 7,571 each, 31,511 in all"; "**Your rule: 7 agents.**" against "Small Components packed into the free room of large ones: 6 agents."
- "Nothing ranks the two shapes beyond that, so this is a fork, and the record will say you chose it over the rival at a cost of one agent per search here."
- "**One refinement of mine: the read-only context heads count toward the bound.** In the prototype, `check` alone carried 39 context heads from other Components."

### Searchers' returns during the first search `##arg-35`

Round 13, the session, at L707 to L806.

- Threads: `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@occasion-shaped-output`
- L707: "`--since` would add a second use of `git` in `query`, which `design@knowledge-architect@query-crate` words as if it had only one."
- L724: "`thread@record-search-verdicts@component-packing` says nothing about where a location's entries go, or those of a declared register's entry anchors."
- L751: "**`conflicts`** on `rejected@agent-skills@one-agent-reads-every-standing-entry`: the single goal searcher's load grows with the goals homes."
- L765: "Under `design@agent-skills@synthetic-evidence-not-built`, a run built to observe how agents follow an instruction may not choose a shape. [...] I'll use only its mechanics: agent counts, words per agent, peak tokens, and whether the quotation check works."
- L780: "**`--since` should mark changed entries only, not code.** Marking code would read beyond the record that `design@query@record-is-every-register-entry` defines."
- L806: "Of 54 goals it graded 25 `constrains`, 3 `conflicts` and 26 `unrelated`."

### Mechanics and token fit of the first search `##arg-36`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@verdict-report`
- Source: the searchers' reports, the session's quotation-check script, the searchers' transcripts.
- "7 cluster searchers plus 1 goal searcher, against 25 under today's work list." "911 entries, each graded by exactly one searcher." "Verdicts: 794 `unrelated`, 74 `constrains`, 18 `conflicts`, 16 `touches`, 5 `closes`, 2 `worsens`, 1 `brings-back`, 1 `depends-on`." "0 findings over 911 lines."
- "That fits about **68,000 tokens of fixed start plus 3.4 per word**. [...] At 40,000 words the peak is about 200,000, under your 250,000. A step to 50,000 would predict about 240,000, near the limit, so 40,000 holds and the next raise should be small."

### A goal rules out the per-unit mapping (later withdrawn) `##arg-37`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@goal-context-once`
- "`goal@agent-skills@only-needed-instructions` refuses "a fixed sequence, count or mapping between a unit of the work and a unit of its record that another instruction needs to vary". One verdict file per finding cluster or per thread is such a mapping."
- "**3. `thread@record-search-verdicts@goal-context-once`: "one goal searcher" is a fixed count.**" "**Revised:** goals are packed under the same bound, by Component, in as many goal searchers as that takes."

### Locations, conformance replacement, trigger-met, routed actions, audit fixes, lighter pass `##arg-38`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@lighter-pass-untouched`
- "**Rule:** an anchor that is no Component counts in the nearest Component that encloses its path."
- "Once that list is gone, the reviewer reads the verdict file whole and judges the work against every verdict that is not `unrelated`."
- "**`trigger-met`** extends to the re-entry point of a `design` issue, not only a deferred trigger."
- "`check-reversal` goes to the reversal check, which reads what cites the decision. That keeps `goal@knowledge-architect@records-reach-their-reader`, whose "requires a session to read what cites a decision before it reverses it" names no main session."
- "`--seeds-from` takes the spec's design entries and declared-register entries, no goal and no `rejected` entry."
- "The cost argument is gone here: 8 agents read the whole record at about 200,000 tokens each. But a project of about 30 Components hasn't been measured."

### Premortem causes `##arg-39`

Round 13, the session, at L839.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@goal-context-once`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@bound-raise`
- "a searcher grades `unrelated` an entry that bears, and no reader reads `unrelated` entries"; "a quotation is verbatim but not the sentence that decides, so the check passes on a wrong verdict"; "a cluster searcher misjudges a head's constraint from goal titles alone"; "a Component split above the bound separates two subjects that interact"; "in a project with longer entries, 40,000 words exceeds 250,000 tokens"

### `goal@agent-skills@only-needed-instructions` misread `##arg-40`

Round 14, the owner, at L848.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@goal-context-once`
- "the refusal you mentionned is about what "the work does not need". If we judge the work needs it, that is allowed."
- "this clause was intended to apply mostly to the design procedure, were creativity is important, and structure could hurt it. But the semi-mechanical part of a record search should not be affected."
- "If the structure can be enforced by a mechanical check, that clause should not apply, because deviation will be corrected instantly."
- "It should not apply to a mechanical procedure. It applies to agentic instructions."

### Each location its own unit `##arg-41`

Round 14, the owner, at L848.

- Threads: `thread@record-search-verdicts@component-packing`
- "I think each location should be treated as one small component (or not necessarily small) for that. If you look at thaum (../games/thaum), it contains a rule interpretation location, zhich is quite big, and probably needs to get its own searcher(s)."
- "goal-context-once: agreed to pack under the same bound, by component. This is clearly needed if goal size exceeds the bound"

### The goal read whole; first shape wins minus one sentence `##arg-42`

Round 14, the session, at L873.

- Threads: `thread@record-search-verdicts@occasion-shaped-output`, `thread@record-search-verdicts@goal-context-once`
- "Its measure is "each installed instruction". So it constrains instructions in the installed text, and a structure that an instruction would need to vary."
- "The number of goal searchers is computed by the command from the bound. That is no instruction, and no instruction needs to vary it."
- "**So the first shape wins, minus that one sentence.** `--verify` writes one verdict file per unit the brief names: per thread, per finding cluster, or the slice."

### Locations as units; thaum measured `##arg-43`

Round 14, the session, at L873.

- Threads: `thread@record-search-verdicts@component-packing`
- Source: thaum's manifest; `wc -w` over `path@elsewhere@docs/rules`.
- "The packing unit becomes **every anchor that carries registers: a Component or a location**, each treated alike."
- "`rules`, at `path@elsewhere@docs/rules`, carrying `issue`, `tripwire` and `interpretation`. Its Markdown holds 83,376 words by `wc -w`. That is an upper bound on its entries"

### Can the searcher be trusted; context or model? `##arg-44`

Round 15, the owner, at L876.

- Threads: `thread@record-search-verdicts@verdict-report`
- "Right now, it looks like we cannot trust the record searcher as much as I thought. But maybe the difference lies in what instructions the searcher has in context. For example, you held the design skill in context. [...] Or maybe it is about the model. Which one did it use ?"

### A conditional verdict consumed as unconditional `##arg-45`

Round 15, the session, at L900.

- Threads: `thread@record-search-verdicts@verdict-report`
- Source: the `model` field of the searchers' transcripts; grep over the reports.
- "**Mostly mine, not the searchers'.** Within their conditions, the searchers were largely right"
- "**I dropped the conditions.** Three "under the reading that…" verdicts became one unconditional sentence" "**I moved the objection.**" "**I ruled without reading the entry.**"
- "**Model:** all eight searchers ran `claude-opus-5-5`, the same model as this session, by the `model` field of their transcripts."
- "By grep, 8 of the 18 `conflicts` verdicts from the cluster searchers mention a reading in their reason, and all 3 from the goal searcher do."
- "That premise holds only if the line between the two cases is visible, and a reason in prose doesn't make it visible." Proposal: "a conditional verdict as a field of its own" with action `resolve-reading` (withdrawn in round 16).
- "A shape worded vaguely produces conditional verdicts."

### Give searchers the design context; parity rather than conditional verdicts `##arg-46`

Round 16, the owner, at L910.

- Threads: `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@search-work-carries-the-deliberation`
- "the main design session holds detailed info on the design, which the searchers do not. Maybe feeding them with the extracted transcript of the design discussion could help, if the size is reasonnable"
- "I think the best shape would be to give the dispatched agents a level of reading that is on par with what the main session holds, so that they can reach conclusion that are similar to the ones of the main session, without needing a conditional. The only advantage the main session keeps is that it can ask me for a goal reading, or for a fork."

### Transcript cost; parity; `owner-question` `##arg-47`

Round 16, the session, at L939.

- Threads: `thread@record-search-verdicts@search-work-carries-the-deliberation`, `thread@record-search-verdicts@verdict-report`
- Source: `klarch transcript show` over this session.
- "owner | 3,087"; "session | 15,908"; "injected (skills, reminders, hand-backs) | 4,946"; "about 65,000 tokens per searcher at the 3.4 tokens per word measured in today's run."
- "40,000 words of entries plus 19,000 of transcript predicts about 270,000 tokens at peak, over your 250,000." "this repository needs about 12 agents instead of 8."
- "**A raw transcript holds superseded positions.** [...] So the brief gives **the latest delta or checkpoint as the authority on each thread's state**"
- "So I withdraw the `if` field." "**What remains is a question for you**, as an `owner-question` field"
- "So point 1 changes the design skill's grounding alone."

### Transcript cost too high; plan before the search? `##arg-48`

Round 17, the owner, at L942.

- Threads: `thread@record-search-verdicts@search-work-carries-the-deliberation`, `thread@record-search-verdicts@search-reads-the-draft`
- "the cost does seem a bit too high here. I am not convinced this is worth it. And you are right that the transcript holds information that is not necessarily useful."
- "Maybe the best shape would be to have a plan document already written, but this is a shape we rejected long ago because we wanted to have a search that runs before the plan writing. But maybe it is truly more valuable to have that search run as part of the plan review."
- "The search before the plan is starting to look like it has more flaws, for now, to me."

### The search reads the draft `##arg-49`

Round 17, the session, at L971.

- Threads: `thread@record-search-verdicts@search-reads-the-draft`, `thread@record-search-verdicts@search-work-carries-the-deliberation`
- "**the search reads the draft of the document that will carry the design**, written when convergence is near, and still before any thread closes."
- "**That reason no longer holds.** `design@agent-skills@no-thread-closes-before-the-search` came after it" (against `rejected@agent-skills@conformance-reader-at-the-checkpoint`).
- "a work statement written for the search (today) | about 1,500 words, written only for the search | it compresses: it said "one extra goal searcher" where the discussion meant a count the bound decides, and three verdicts turned on that wording" (this argument from the prototype run was removed in round 18, then reinstated by the owner's ruling in round 19, per L1282).
- "written anyway, since the main session writes the plan under `goal@agent-skills@session-orchestrates-the-work`"
- "**A draft written before the search can need revising after it.** It's a document, so a revision costs an edit, never rework of code."

### Returns of the search on the draft `##arg-50`

Round 18, the session, at L1075 to L1187.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@search-reads-the-draft`, `thread@record-search-verdicts@lighter-pass-untouched`
- L1075: "**The bound** is 37,400 words of entries per agent, leaving room for the draft's 2,508 words inside your 40,000."
- L1101: "the real brief must apply the rule to decisions only, never to goals" (`rejected@agent-skills@goal-seeds`).
- L1129: "So "each Component and each location" would make every one of those its own unit, against the draft's own rule. The thread will say "each location the manifest declares""
- L1140: "The verdict file is added to that reading, not put in its place." "today the list of unread entries is what triggers the conformance axis at the merge of an in-change or bounded branch. Once the list goes, that trigger needs a replacement."
- L1158: "The field takes every question that section reserves to you." "the draft's figures, such as "100 of 341" and "about 200,000 tokens", must not go into the heads as numbers."
- L1176: "A wrong `unrelated`, which is the premortem's first cause, then leaves nothing to check [...] The fix: every `unrelated` line carries a one-line reason." "about 800 `unrelated` lines per search here. At about 15 words a line that is about 1,500 words per agent."
- L1187: "So the draft must not "become the spec", as my Status section says. The draft is the search's work only. The spec is still assembled from the transcript afterwards." "the design skill writes no plan document. So the draft is written under the planning skill's sections"

### Result of the search on the draft `##arg-51`

Round 18, the session, at L1270.

- Threads: `thread@record-search-verdicts@bound-raise`, `thread@record-search-verdicts@component-packing`, `thread@record-search-verdicts@verdict-report`, `thread@record-search-verdicts@slice-audit-checks-fit`, `thread@record-search-verdicts@search-reads-the-draft`
- Source: the reports, the session's quotation-check script, the transcripts, the session's packing script.
- "912 entries, each graded once, and 54 goals." "Peaks ran from 139,098 to 210,556 tokens, all under your 250,000."
- "The packing unit becomes a **search unit**, and a unit of the work a **subject of the work**."
- "a standing entry that cites a head of another search unit gets that head's **full text**, counted in the bound. [...] This costs **12 agents instead of 9** over this repository, by my script."
- "the evenly spread packing of small search units stays, or `rejected@query@packing-to-the-bound` would come back."
- "A `conflicts` verdict on a goal goes to goal-setting, never to the reversal check."
- "`--seeds-from` reads the slice's own spec, not the milestone document."
- "The thread now rests on two arguments: the draft is written anyway, and a separate work statement would be a second copy of the design, which `primer@where-knowledge-goes` refuses."

### The prototype run is real use `##arg-52`

Round 19, the owner, at L1272.

- Threads: `thread@record-search-verdicts@search-reads-the-draft`, `thread@record-search-verdicts@bound-raise`
- "For the question, I would count that as real use."

### Scope of the real-use ruling; spec or milestone `##arg-53`

Round 19, the session, at L1282.

- Threads: `thread@record-search-verdicts@search-reads-the-draft`
- "a record search run in a real design discussion, under a planned instruction's shape, counts as real use. [...] Your word was about our search, not a census."
- "The prototype argument then comes back into `thread@record-search-verdicts@search-reads-the-draft`: the first search's compressed work statement is evidence again."
- "**D1, spec or milestone** [...] **Default: one spec on one branch.** Between the `query` steps and the text step, the installed text would name files the command no longer writes."

## New names, in one place

```text
crates/cli/src/cli/mod.rs        DEFAULT_WORKLIST_BOUND     existing; 25000 becomes 40000
                                 --since <commit>           new option of `worklist`
                                 --seeds-from <spec>        new option of `worklist`
crates/query/src/graph.rs        pack                       existing; packs search units
crates/query/src/worklist.rs     search unit                new term: a Component or a declared location
                                 context head               new term
                                 verdict, action            new report fields, replacing grade, relation
                                 owner-question             new report field
                                 against                    new report field: the subjects of the work
                                 verdict file               new output of --verify, one per subject
                                 bears.md, may-bear.md      existing outputs, removed
agent directory entries.md       goal titles, context heads new read-only sections
```

## Decided design

### Goals reach the cluster agents as titles, and goal agents grade them

A cluster agent's `entries.md` holds, read only, the title line of every goal of the root and of each
search unit it holds. The goals are packed by search unit under the same bound into goal agents, as
many as the bound takes, each reading its goals whole with the head of each goals home and grading
each goal once. The session still reads every goals home at the design grounding, so both readings of
`design@agent-skills@searcher-grades-goals` stay. The argument: goal entries were about a third of the
words read, the root goals home copied into every agent (`argument@record-search-verdicts@arg-7`).
Nearest rival: the goals of the enclosing Components only, which still copies the root goals home into
every agent (`argument@record-search-verdicts@arg-9`).

### The bound is 40,000 words of everything an agent reads

`DEFAULT_WORKLIST_BOUND` is 40000. An agent's load counts its entries, the full text of its context
heads, the title lines of its goals, and the draft or the plan it reads as the work, so the bound left
for entries is the bound minus the work. On the owner's report that agents stay reliable to about
250,000 tokens of context and that the cost rises from there, and the owner's choice to raise by
steps. Both searches measured peaks under 250,000 tokens at this bound
(`argument@record-search-verdicts@arg-36`, `argument@record-search-verdicts@arg-51`).

### Search units, packed alone above half the bound

Every Component and every location the manifest declares is a search unit. An entry belongs to the
search unit that holds it; a constructed location, a plan or an entry anchor of a directory register,
belongs to the search unit holding its home. A search unit whose load exceeds half the bound gets
agents of its own: one where it fits the bound, else its clusters spread among as many as the bound
needs. Search units below half the bound are packed together whole, first-fit decreasing, then spread
over that many agents, as `design@query@clusters-packed-under-a-word-bound` spreads them today. A
standing entry that cites a head of another search unit gets that head's full text as a context head,
counted in the bound, so that it is judged beside the heads it cites. The argument: an issue judged in
an agent holding no design of its own Component is the defect the owner named
(`argument@record-search-verdicts@arg-17`, `argument@record-search-verdicts@arg-22`); the half bound is
the one threshold at which the rule only forbids a small unit joining a large one
(`argument@record-search-verdicts@arg-34`). The cost of the full-text context heads is measured in
`argument@record-search-verdicts@arg-51`. Brought back:
`rejected@agent-skills@one-search-agent-per-anchor`, whose reason the split above the bound answers.

### The verdict line

```text
- <ref> | verdict: <v> | quote: <text> | against: <subjects> | action: <a> | owner-question: <text or empty> | reason: <one line>
```

(An illustration of the shape, not authority.)

- Verdicts: `constrains`, `conflicts`, `brings-back`, `closes`, `touches`, `worsens`, `depends-on`,
  `fires`, `trigger-met`, `unrelated`. `trigger-met` covers a deferred issue's trigger and a design
  issue's re-entry point. A goal takes `constrains`, `conflicts` or `unrelated`.
- Actions: `respect`, `check-reversal`, `new-argument-needed`, `rule-before-work`,
  `close-with-the-work`, `none`, each routed to the procedure the installed text already has. A
  `conflicts` verdict on a goal routes to the goal-setting skill.
- The quotation: at least eight words of the entry, verbatim; `--verify` refuses one not found in the
  entry's snapshot text in `shown.md`, whitespace normalised. An `unrelated` line takes none, and
  every line carries a one-line reason.
- The searcher resolves every doubt the work and the record answer, and states its conclusion, not a
  condition. `may-bear` is gone. What `primer@question-authority` reserves to the owner goes in
  `owner-question`, which the main session puts to the owner.
- An entry that is a seed, or cites one, is never `unrelated`; seeds are decisions, never goals.
- The main session acts on the verdicts, checked as its own claims are, with no re-reading by rule.
- The design-conformance review reads the plan against the goals, design and rejected homes, and the
  verdict file beside them; at the merge of an in-change or bounded branch, its axis is owed wherever
  the grounding's verdict file holds any verdict but `unrelated`.

The argument: the size of the reading the grades owe
(`issue@agent-skills@the-search-s-graded-entries-exceed-what-one-session-reads-whole`), and the goal's
check parity. Nearest rival: the conditional `if:` field, withdrawn for parity of context
(`argument@record-search-verdicts@arg-46`, `argument@record-search-verdicts@arg-47`).

### One verdict file per subject of the work

The brief names the subjects of the work. The searcher writes in `against` the subjects each verdict
bears on, and `--verify` writes one verdict file per subject, every verdict but `unrelated` with its
entry's reference, in checked forms. The main session divides the work among agents over those files
at its own judgement, per `goal@agent-skills@session-orchestrates-the-work`.

### The design audit checks fit

At the design audit of a slice or a spec, the work is the spec; the searcher's verdicts are read as
`fits` or as a gap with a class: `applied-in-place`, which amends plan documents only, never a head;
`scope-change`; `choice`; `firing`. Every verdict on a standing entry, `closes`, `touches`, `worsens`,
`depends-on`, `fires` or `trigger-met`, is a gap. `worklist --since <commit>` marks the entries changed
since that commit, a mark and not a filter, and marks no code
(`design@query@record-is-every-register-entry`). `worklist --seeds-from <spec>` takes as seeds the
design entries and declared-register entries the slice's own spec cites, no goal and no `rejected`
entry. `--verify` writes the gaps file and the per-entry fit lines for the audit's commit message in
checked forms: every reference resolves against the commit's tree or its first parent's, the plan is
cited whole, the `--since` commit is named by its subject, and every reference stands outside the
quotation. The main session reads the gaps.

### The search reads the draft

At the design skill's grounding, the search runs once the shapes settle and before any thread closes,
and its work is the draft of the document that will carry the deliberation: the plan document on the
full path, the commit message on the in-change path and for bounded work. The draft is written under
the planning skill, as a new step of its "When this skill starts"; the design skill's loop step 1
points to it and writes no plan document (`design@agent-skills@design-hands-off-to-planning`). The
findings revise the draft; the premortem and the convergence follow; the plan is then assembled from
the transcript (`design@agent-skills@ledger-from-transcript`) and reviewed. The argument: the draft is
written anyway; a work statement written apart from it is a second copy of the design, which
`primer@where-knowledge-goes` refuses, and the first search's work statement compressed the design
(`argument@record-search-verdicts@arg-49`), evidence admitted as real use on the owner's word; the
transcript costs more agents (`argument@record-search-verdicts@arg-47`). The reason of
`rejected@agent-skills@conformance-reader-at-the-checkpoint`, rulings already given, is defeated by
`design@agent-skills@no-thread-closes-before-the-search`.

## Mapping tables

| today | after |
| --- | --- |
| `bears` with a relation | the verdict of that relation |
| `may-bear` | resolved by the searcher, a `conflicts` verdict, or an `owner-question` |
| `bears.md`, `may-bear.md` | one verdict file per subject of the work |
| a residual group per anchor | the entries of a search unit, packed with it |
| the goals in every cluster | goal titles in cluster agents; goal agents |
| a seed (decision or goal) | a seed, decisions only |

## Losing alternatives

- Packing small units into the room of large ones: lost to the half-bound rule, the owner's choice,
  at a cost of one agent over this repository (`argument@record-search-verdicts@arg-34`).
- Context heads by title only: lost to `design@query@cluster-rooted-at-a-subject`, which judges a
  standing entry beside the heads it cites; it cost 3 agents fewer.
- One goal searcher: lost to goals homes that can exceed the bound
  (`argument@record-search-verdicts@arg-41`).
- One verdict file with a filter option: withdrawn, its ground a misreading
  (`argument@record-search-verdicts@arg-42`).
- A sentence sending each finding cluster to one subagent: ruled out by
  `goal@agent-skills@session-orchestrates-the-work`.
- The conditional `if:` field with a `resolve-reading` action: withdrawn by its proposer for parity of
  context (`argument@record-search-verdicts@arg-47`).
- The transcript as the search's work: superseded by
  `thread@record-search-verdicts@search-reads-the-draft`, on its cost
  (`argument@record-search-verdicts@arg-47`).
- A work statement written for the search: lost to the draft, a second copy of the design.
- `--since` marking code: lost to `design@query@record-is-every-register-entry`.

## Readings

None: the work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| a searcher gives `unrelated` to an entry that bears, and no reader reads `unrelated` entries | `thread@record-search-verdicts@verdict-report` | survives as a named clause: every `unrelated` line carries a reason; `tripwire@agent-skills@record-search-misses` watches the misses |
| a quotation is verbatim but not the sentence that decides, so the check passes a wrong verdict | `thread@record-search-verdicts@verdict-report` | named as a cause for the harvest to judge as a tripwire |
| a cluster searcher misjudges a head's constraint from goal titles alone | `thread@record-search-verdicts@goal-context-once` | named as a cause for the harvest to judge |
| a split above the bound separates two subjects that interact | `thread@record-search-verdicts@component-packing` | named as a cause for the harvest to judge |
| in a project with longer entries, 40,000 words exceeds 250,000 tokens | `thread@record-search-verdicts@bound-raise` | becomes the acceptance criterion `acceptance@record-search-verdicts@peaks-under-the-reliability-bound` |
| a draft written before the search anchors the discussion, so a finding is answered by amending rather than reconsidering | `thread@record-search-verdicts@search-reads-the-draft` | named as a cause for the harvest to judge |

## Acceptance criteria

### The searchers of a search at the bound peak under 250,000 tokens `##peaks-under-the-reliability-bound`

Guards `thread@record-search-verdicts@bound-raise`. Judged at step 4, by the first search the built
command and texts run over this repository. Fires when any searcher's peak context, input plus cache
plus output of its last turn in its transcript, exceeds 250,000 tokens, the owner's reliability bound.

### Packing by search unit costs no more agents than measured `##agent-count-as-measured`

Guards `thread@record-search-verdicts@component-packing`. Judged at step 1, by `klarch worklist --out`
over this repository at the bound of 40,000 with no work counted. Fires when the cluster agents number
more than 13, against the 12 the session's script measured for this shape with the draft counted.

## Implementation sequence

1. **`query`, `cli`: the packing by search unit.** Builds search units, the half-bound rule, the
   spreading of the small units, context heads in full text, goal titles in cluster agents and goal
   agents, the load counting the work, and `DEFAULT_WORKLIST_BOUND` at 40000. Claims: each entry is in
   exactly one agent; no agent's load exceeds the bound unless it holds one cluster alone; a search
   unit above half the bound shares no agent; a constructed location is packed with its home's search
   unit; tested over mock projects in `testing`. Fails alone on: the agent count and the loads.
2. **`query`: the verdict report.** Builds the report line, the verdict and action sets, the
   quotation check against `shown.md`, `owner-question`, the reason on every line, the verdict files
   per subject of the work, and the generated per-entry lines in checked forms. Claims: `--verify`
   refuses a quotation not in its entry, a verdict or an action outside its set, a missing reason, a
   seed or a seed's citer graded `unrelated`, and a goal given an issue verdict; tested over mock
   reports. Fails alone on: what `--verify` refuses.
3. **`query`, `cli`: the audit's options.** Builds `--since <commit>` through `git` and
   `--seeds-from <spec>`. Claims: `--since` marks exactly the entries whose text changed since the
   commit; `--seeds-from` takes only design and declared-register entries. Fails alone on: the marks
   and the seeds.
4. **`agent-skills`: the installed text.** Rewrites the searcher agent, the design skill's loop step 1,
   the planning skill's "When this skill starts" and design audit, the intake's grounding and the
   design-conformance reviewer, then runs `klarch install-agent-skills`. The loop-step citations a
   renumbering touches are checked by hand
   (`issue@check@no-checked-form-cites-a-point-inside-a-section`). Fails alone on: the installed
   copies against the source, and the first search the built text runs.
5. **The harvest**, as the section "Harvest" says.

## Order rationale

- 1 before 2: the report is judged over the agents the packing writes.
- 2 before 3: the audit's options write into the verdict files.
- 3 before 4: the text names what the command writes.
- 4 before 5: the harvest records what landed.

## Defaults awaiting the owner

### D1, one spec on one branch, not a milestone

Bears on the whole work. Between steps 2 and 4, the installed text would name files the command no
longer writes, so the work lands on one branch. The rival, a milestone of a `query` slice and a text
slice, would need `--verify` to keep writing `bears.md` and `may-bear.md` until the second slice.
Awaiting the owner's ruling.

## Harvest

- **Heads**: each head listed as reversed or rewritten in "What is already decided", rewritten in place
  under `skill@knowledge-architect-decision-recording`, with the claim, the command and the direction
  of each measurement and no magnitude, as the root `CLAUDE.md` asks.
- **`rejected` entries**: `rejected@agent-skills@one-search-agent-per-anchor` leaves; the losing
  alternatives above are judged by the recording tests, the packing of small units into large ones'
  room and the transcript as the work, both refuted by measurement, as candidates; each new entry is
  cited from a head.
- **Tripwire**: `tripwire@agent-skills@owner-proposal-built-ungrounded` reworded; the premortem's
  causes judged.
- **Issues**: the six listed as closed, deleted; the re-entry point of
  `issue@agent-skills@a-lighter-pass-for-untouched-components` and the sentence on residual groups of
  `issue@agent-skills@audit-axes-beyond-the-design-record` rewritten.
- **CHANGELOG.md**: entries under Workflow (the verdict report, the draft searched, the audit's fit),
  New features (`--since`, `--seeds-from`) and Migration where a consumer's tooling reads `bears.md`.

## Later consequences

- The rewording of `goal@agent-skills@only-needed-instructions`, a goal-setting item.
- The dispatch judgement, in
  `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`.
- The intake's partitioned procedure, parked as `thread@record-search-verdicts@intake-partitioned-procedure`.
- The next raise of the bound, on the peaks the criterion
  `acceptance@record-search-verdicts@peaks-under-the-reliability-bound` measures.

