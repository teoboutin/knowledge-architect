# `worklist` assembles the graded entries, `klarch id` tells whether an id is free, the record search reads less, and the retrospective asks for hand-carried mechanical tasks

## Status and audience

This spec is the plan of one branch and one pull request. It leaves the repository in a commit of
its own, after its harvest and the review before the merge. The work does four things:

- `worklist --out` also writes the `show` text of every entry of the record, and a passing
  `worklist --verify` assembles from it the entries graded `bears` into one file and those graded
  `may-bear` into another;
- a new command, `klarch id <id>...`, says for each argument whether it is a well-formed id and
  which entities, if any, the entity table defines under it;
- the installed text reads the search's results with less context: a `may-bear` grade needs a named
  doubt about an element the change makes, seeds are decisions, a goal set through goal-setting is
  not part of a design search's work, and a `may-bear` entry is read whole where a proposal touches
  it;
- the installed retrospective skill gains a standing question on mechanical tasks a session wrote a
  script for or carried by hand.

The spec is written for a session that did not witness the design discussion that produced it.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. D1, D2 and D3, under "Defaults awaiting the owner", are ruled.

It was assembled from one session's transcript. The discussion begins at the owner's message that
opens "Two more things I would like to mechanically automate for the design skill:". Find the
session by that message, with `cargo klarch transcript list --containing`, not by a file name. The
session holds 9 owner messages, the eighth and ninth after this spec was first written. Here **round n** is the owner's n-th message and the session's
reply to it:

- round 1: the owner's request; the reply is the grounding and the record search, interrupted by the
  owner during the slug check;
- round 2: the owner's message on the grounding's reading cost; the reply, headed "Why the grounding
  read so much", measures it and proposes five threads;
- round 3: the owner's agreement to the five threads, and a question on the goal; the reply measures
  where each proposal reached;
- round 4: "I agree with that"; the reply is headed "Round 1: the three proposals" in the
  transcript, and is called here **the reply to round 4**;
- round 5: the owner's rulings on the threads and the goal; the reply proposes convergence, the
  premortem and AC1 to AC4;
- round 6: the owner's objection to AC1 and AC2; the reply corrects its premise;
- round 7: the owner keeps AC1 to AC4 and says "Proceed";
- round 8: the owner rules D1, "Agreed on the spec shape, to land in a single PR with the work";
- round 9: after the review of the spec, the owner rules D2, D3 and five questions on its findings,
  "Agreed on all defaults here, Q1-5 and D2 D3, proceed".

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

## Names

- **the work list directory**, `<dir>`: the directory `cargo klarch worklist --out <dir>` writes,
  outside the project, per `design@core@worklist-writes-and-verifies-the-reports`.
- **the entries file** of an agent: `agent-<n>/entries.md` under `<dir>`, every entry the agent is
  given, whole, written by `render` in `path@core@src/cli/worklist.rs`.
- **the report** of an agent: `agent-<n>/report.md` under `<dir>`, the skeleton the agent fills.
- **the show text** of an entry: what `cargo klarch show <reference>` prints for it: the line
  `<reference>  <file>:<line>`, a blank line, the entry's body, a blank line, then `referenced at:`
  with one indented site per line, or `referenced by nothing`. It is printed by `show` in
  `path@core@src/cli/mod.rs`, the inbound sites from `records::inbound`.
- **the snapshot**: the file `<dir>/shown.md`, new, the show text of every entry of the record, as
  `--out` computes it. Defined by `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`.
- **the assembly**: the files `<dir>/bears.md` and `<dir>/may-bear.md`, new, written by a passing
  `--verify`. Defined by `thread@graded-entries-and-id-lookup@graded-entries-assembled`.
- **a grade line**: one graded line of a report, its fields after the reference, `grade:`,
  `relation:` and `reason:`, as `parse_fields` in `path@core@src/cli/worklist.rs` reads them.
- **an entry's highest grade**: over every report that grades it, `bears` above `may-bear` above
  `unrelated`. Goals are the common case, since every agent whose clusters reach a goal grades it.
- **the entity table**: `Entities` in `path@core@src/entity.rs`, keyed by kind, anchor and id;
  `Entities::by_id` lists, per id, every kind and anchor that defines it.
- **the id grammar**: `[a-z0-9]+(-[a-z0-9]+)+`, `is_entry_id` in `path@core@src/entity.rs`, per
  `design@core@two-segment-ids`.
- **the first three phases**: `check::foundation` in `path@core@src/check/mod.rs`, which
  `complete_working_tree` in `path@core@src/cli/mod.rs` asks before a writer writes, per
  `design@core@phases-gate-the-report`.
- **a named doubt**: the reason of a `may-bear` grade that names an element the change makes and the
  sentence of the entry that element might conflict with. Defined by
  `thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt`.
- **the not-read list**: the `may-bear` entries a session did not read whole under
  `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`, each with the searcher's reason.
- **the installed text**: the sources under `path@agent-skills@content/`, with the fragments of
  `path@agent-skills@fragments/` and the snippets of `path@agent-skills@snippets/` the build copies
  into them, which `cargo klarch install-agent-skills` writes into `.claude`; edited per
  `path@agent-skills@CLAUDE.md`, its section "Editing an installed skill or agent". No passage this
  work edits lives in a fragment or a snippet.
- **the design audit**: point 2 of `skill@knowledge-architect-planning@working-a-slice`, the reading
  of a slice's spec or of a spec against the tree and the record before its work, which dispatches
  the record search.
- **AC1 to AC4**: the labels the acceptance criteria were put to the owner under, in the reply to
  round 5, kept in their items.
- **D1, D2, D3**: the defaults put to the owner, under "Defaults awaiting the owner", all ruled.

## What the work is

What exists today at each site the work touches:

- **`worklist --out`** is `write` in `path@core@src/cli/worklist.rs`. It builds the model, refuses
  over an incomplete one through `complete_working_tree`, builds `Anchors`, `Entities` and the
  `graph::graph` of the record, resolves the seeds, packs the groups, and writes the files `render`
  returns: `agents.txt`, and per agent its entries file and its report. Every refusal comes before
  the first write. The entries file holds each entry's body only, in a fence, after a line
  `=== <reference>  <site>`; it holds no inbound list.
- **`worklist --verify`** is `verify` in the same file. Its doc comment states: "It reads no model:
  what each agent was given is what `agents.txt` says, so the check holds whatever the tree has
  become since." It reads `agents.txt` and every report, collects findings, and prints
  `PASSED: every report grades every entry it was given` with exit 0, or
  `FAILED: <n> findings above` with exit 1. It stops with an error, exit 2, on a line of
  `agents.txt` it cannot parse, a report it cannot read, and the first report line `parse_fields`
  refuses. It keeps no grade after judging a line, and writes nothing. It holds the agents in a
  `BTreeMap`, so it does not keep the order of `agents.txt`.
- **`show`** is `show` in `path@core@src/cli/mod.rs`. It builds the model with no phase gate, finds
  a register entry through `records::records`, prints it line by line, then the inbound sites from
  `records::inbound`, which scans every observation of every document for the one entry. No function
  returns the show text; `records::records` builds the entity table again on each call. Its
  malformed argument exits 2, a reference that names nothing exits 1, per
  `design@core@exit-code-ladder`. `show`, `issues` and `tripwires` answer with no phase gate.
- **No command answers "what defines this id"**. The bare-id lint builds the answer internally:
  `IdTable::new` in `path@core@src/check/references.rs` holds `Entities::by_id`. A session checks a
  slug today with two `show` calls per candidate, as the design skill's naming paragraph says, or
  with a loop of its own; the owner reported in round 1 that "every session writes a small bash
  script".
- **The commands** are the variants of `Command` in `path@core@src/cli/mod.rs`, dispatched by the
  match of `run` there, their arguments declared through clap, per
  `design@core@arguments-parse-through-clap`, in the library module every extension binary offers,
  per `design@core@the-core-cli-is-a-library-module`, whose head lists the commands one by one. The
  declaration is tested by `cli_declaration_is_consistent` in `path@core@src/main.rs`. `Command` is
  a public, non-exhaustive enum, so a variant is a `library` addition for the changelog. Under
  `harness = []` the entity table holds no skill, agent or section.
- **The record searcher**, `path@agent-skills@content/agents/record-searcher.md`: its description
  carries the dispatch and ends with "Then read whole with `{{command}} show` every entry graded
  `bears` or `may-bear`, never from its reason alone, or, at a retrospective intake, as
  `skill@knowledge-architect-retrospective-intake@ground-before-judging` says." Its body tells the
  searcher "The session that sent you reads whole the entries you grade `bears` or `may-bear`, as
  its skill says", to lean to recall, that "An entry you grade `may-bear` that does not bear costs
  the session at most one read", and to "Grade `may-bear` an entry you cannot rule out, with the
  doubt as its reason"; its brief, `agent@knowledge-architect-record-searcher@your-brief`, names
  "the decisions and goals the dispatching skill names" as seeds; its grade table, `agent@knowledge-architect-record-searcher@grade-each-entry`, gives
  `may-bear` as "an entry that shares the work's subject, with no relation established"; and
  `agent@knowledge-architect-record-searcher@fill-the-report` asks, for `may-bear`, "the subject it
  shares and why no bearing is established".
- **The design skill**, `path@agent-skills@content/skills/design/SKILL.md`: step 1 of
  `skill@knowledge-architect-design@discussion-loop` dispatches the search "with the question as the
  work and, as seeds, the decisions and goals read so far that bear on it", then runs `show` on every
  entry graded `bears` or `may-bear` and reads each whole, and the first round states, "for each
  entry graded `may-bear`, whether it bears". The naming paragraph of
  `skill@knowledge-architect-design@threads-and-states` checks a slug with
  `{{command}} show design@<anchor>@<slug>` and `{{command}} show rejected@<anchor>@<slug>`.
- **The planning skill**, `path@agent-skills@content/skills/planning/SKILL.md`: point 2 of
  `skill@knowledge-architect-planning@working-a-slice` reads whole with `show` every entry graded
  `bears` or `may-bear`; the row "What is already decided" of
  `skill@knowledge-architect-planning@spec-contents` asks for each such entry "its outcome: how it
  bears on the work, or that it bears on nothing". The design audit seeds the search, for a slice,
  with the decisions its milestone document lists under "What is already decided" and "the
  decisions and goals the slice's spec cites", and for a spec with its own "What is already
  decided", which can hold goals.
- **The intake skill**, `path@agent-skills@content/skills/retrospective-intake/SKILL.md`: point 3
  of `skill@knowledge-architect-retrospective-intake@ground-before-judging` reads with `show` every
  issue and tripwire graded `bears` or `may-bear`, every other entry graded `bears`, and a `may-bear`
  entry where a proposal touches its subject, and lists the other `may-bear` entries as not read.
- **The design-conformance reviewer**,
  `path@agent-skills@content/agents/design-conformance-reviewer.md`: its section
  `agent@knowledge-architect-design-conformance-reviewer@read-against-record` reads "What is already
  decided" first, and its section `agent@knowledge-architect-design-conformance-reviewer@find-the-record`
  reads whole every goals home, design home and `rejected` home of each Component the document
  touches. No text of it mentions a list of entries not read, and it reads no issue, no tripwire and
  no entry of a declared register. Its description, and the row of
  `skill@knowledge-architect-review@review-axes` that names it, scope it to a plan document, a spec
  or a milestone document. The design-conformance reviewer is installed text of its own, not part of
  the planning skill. The installed head behind its reading is
  `design@agent-skills@plan-read-against-the-record`.
- **The design skill's in-change path**, `skill@knowledge-architect-design@decision-authority`: the
  deliberation goes in a commit message, with no plan document; no text gives that message a list of
  entries not read.
- **The retrospective skill**, `path@agent-skills@content/skills/retrospective/SKILL.md`: four
  standing questions in `skill@knowledge-architect-retrospective@what-it-examines`, after "**Always
  ask these standing questions**".
- **The documentation**: `path@core@README.md`, its command listing, its exit-code table, which
  lists per command which outcome takes which code, and its section `worklist`; the `Next release`
  section of CHANGELOG.md at the root, whose `cli` entry for `worklist` is not released yet.

**Already done on this branch**, before this spec, so outside the work:

- the goal `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, the edit of
  `goal@xtask@repeated-tasks-are-automated` that points to it, and the `todo` issue for an unmet
  instance of the goal this discussion found, `issue@core@a-searcher-fills-its-report-by-a-script-of-its-own`;
- the sentence of the root CLAUDE.md under `instructions@verify-before-relying` that restates
  `design@agent-skills@synthetic-evidence-not-built`, in the commit "[agent-config] The root
  CLAUDE.md restates that no run is built to observe an agent following the workflow".

**Outside the work:**

- **The searchers' fill scripts.** `issue@core@a-searcher-fills-its-report-by-a-script-of-its-own`
  stays open; this work does not close it.
- **The other instances of the new goal**: `issue@core@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`
  and `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`. Adding the goal
  does not oblige meeting it now, per the goal-setting skill, as the reply to round 3 says.
- **The intake's seeds.** `thread@graded-entries-and-id-lookup@seeds-name-decisions` was the
  session's proposal of round 2, "One word removed from the design skill's step 1", which the owner
  approved in round 3. The intake keeps its seeds, "the design heads and goals the findings name".
  The design audit's seeds are D3's. The searcher's brief, which names the seeds "the dispatching
  skill names", follows whatever each skill names. `worklist --seed` keeps accepting a goal: the
  rule is the skill's, not the command's.
- **The reading at the design audit.** `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`
  applies at the design grounding. Point 2 of `skill@knowledge-architect-planning@working-a-slice`
  keeps reading every `bears` and `may-bear` entry whole, from the assembly instead of `show`.
- **The intake's reading rule**, which `design@agent-skills@search-relevance-grade` records for an
  intake. It is unchanged; the intake reads from the assembly instead of `show`.
- **The design skill's rule on which collision refuses a slug**, per
  `design@agent-skills@thread-slug-is-entry-id`: only a design or `rejected` entry of the owning
  Component. `klarch id` shows every collision; the skill keeps the judgement.

## What is already decided

The design rests on these decisions, and does not argue them again:

- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, approved by the owner in round 5 and
  written on this branch: the commands of this work are two of its instances.
- `goal@core@records-reach-their-reader`: a listing is computed from the tree.
- `design@core@one-question-one-command`: `klarch id` is the one command that answers "what defines
  this id". The assembly holds the show text byte for byte, a goal's aside, not a second format of
  `show`'s answer.
- `design@core@phases-gate-the-report`: the head gates the writers and the report, and `show`,
  `issues` and `tripwires` answer ungated. `klarch id` is the first reader to take the gate, on the
  head's own reason: a `free` over a table missing an unread file's entries is a finding computed
  over an incomplete model, unreliable in the direction the command exists to rule out, where `show`
  prints an entry that exists whatever else the model misses. The cost: after an edit of the
  installed text and before `install-agent-skills`, the installed copies differ, phase 2 reports
  them, and `klarch id` exits 2 until the install runs.
- `design@core@exit-code-ladder`: 0, 1 and 2 for `klarch id`. An argument outside the id grammar
  exits 1, a negative answer about the command's subject, the candidate id, as an argument that
  names no defined id does; the ladder's 2 is kept for the command that cannot run, a malformed
  invocation that clap refuses, no project, or an incomplete model. `show`'s malformed argument is
  a reference it cannot look up, so it cannot run; `klarch id` answers every candidate, malformed
  ones included, in one run.
- `design@core@arguments-parse-through-clap` and `design@core@the-core-cli-is-a-library-module`:
  `klarch id` is a variant of `Command`, its arguments declared through clap.
- `design@core@two-segment-ids` and `design@core@an-entity-belongs-to-its-anchor`: the grammar
  `klarch id` checks, and the reason it lists every anchor.
- `design@agent-skills@thread-slug-is-entry-id`: kept; the naming paragraph of the design skill
  calls `klarch id` instead of `show`.
- `design@agent-skills@searcher-grades-goals`: kept;
  `thread@graded-entries-and-id-lookup@goals-listed-not-reshown` relies on the session's own reading
  of every goals home that this head keeps.
- `design@agent-skills@synthetic-evidence-not-built`: its "Admitted" bullet admits the checker's
  behaviour tested over mock projects, which is what AC1 to AC4 test.
- `goal@agent-skills@installed-text-works-anywhere`, `design@agent-skills@shipped-text-cites-no-entry`
  and `design@core@declared-command`: the installed edits cite no entry and write `{{command}}`.
- `design@agent-skills@behaviour-additions-need-real-use`: the evidence for each installed
  addition is listed in its thread.

The work rewrites these decisions. Every text that `cargo klarch show` lists as referencing each,
on this branch before the work:

| decision | what changes | texts referencing it | judged or updated at |
| --- | --- | --- | --- |
| `design@core@worklist-writes-and-verifies-the-reports` | `--out` also writes the snapshot; a passing `--verify` writes the assembly, and still reads no model | `path@agent-skills@docs/design/record-search.md` twice, in `design@agent-skills@one-record-searcher` and in `design@agent-skills@search-relevance-grade`; `rejected@agent-skills@searches-split-by-kind`; `issue@core@a-searcher-fills-its-report-by-a-script-of-its-own`, line 30; `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`, line 28; the section comment of the `worklist` tests in `path@core@tests/binary.rs` | the harvest for the head and for each text, each read again; the two issues are read again at the harvest for whether the assembly changes what they state; the test comment stays true |
| `design@agent-skills@search-relevance-grade` | the `may-bear` grade needs a named doubt on an element the change makes; at the design grounding a `may-bear` entry is read whole where a proposal touches it, and the not-read list goes to the review; goals appear in the assembly as grade lines only | `path@agent-skills@content/skills/retrospective-intake/SKILL.md`, a `%%` comment of point 3 of its grounding; `design@agent-skills@one-record-searcher`; `design@core@worklist-writes-and-verifies-the-reports` | step 4 for the intake skill's comment, read again; the harvest for the head and the two heads |
| `design@agent-skills@record-searched-before-the-work` | its sentence "with the question as the work and the decisions and goals read so far as seeds" no longer states the design skill's step 1, nor, under D3's default, the design audit's seeds; `thread@graded-entries-and-id-lookup@one-search-per-question`, where the decision-recording skill places it | `design@agent-skills@standing-state-review-before-every-merge`, in `path@agent-skills@docs/design/reviews.md`; `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`, lines 79 and 96; `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`; `tripwire@agent-skills@record-search-misses` | the harvest rewrites the head, since its seeds sentence turns false; `skill@knowledge-architect-decision-recording` judges only where the one-search rule goes; each text read again |
| `design@agent-skills@one-record-searcher` | its dispatch, carried by the searcher's description, ends with the reading of the assembly; it is rewritten only if the decision-recording skill judges that its text no longer holds | `path@agent-skills@content/skills/retrospective-intake/SKILL.md`, the `%%` comment of point 3 of its grounding; `tripwire@agent-skills@record-search-misses`, twice | the harvest; the comment and the tripwire read again |
| `design@agent-skills@premortem-as-watch-points` | a fifth standing question, which watches a goal; the head's member set widens from "a decision" to "a decision or a goal" | `design@agent-skills@installed-fix-goes-upstream`, in `path@agent-skills@docs/design/retrospective.md`; `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, line 34; `issue@agent-skills@patching-an-installed-skill`, line 63 | step 4 adds the question; the harvest rewrites the head, and reads each text again |
| `design@agent-skills@plan-read-against-the-record`, under D2's default | the design-conformance reviewer reads the entries a document lists as not read, issues and tripwires included, and is dispatched at the review before the merge of an in-change deliberation | a `%%` comment of `path@agent-skills@content/skills/planning/SKILL.md`, in its section on the reviews of a plan document; `design@agent-skills@record-searched-before-the-work`; `issue@agent-skills@audit-axes-beyond-the-design-record`; `issue@agent-skills@the-code-claims-axis-reads-plan-documents-only` | D2's ruling; step 4 for the reviewer's text; the harvest rewrites the head, and reads each text again |
| `design@core@the-core-cli-is-a-library-module` | its list of the core's commands gains `id` | `design@agent-skills@xtask-pins-checker`; `path@core@README.md`; four heads of `register@core@design`, `design@core@single-crate`, `design@core@arguments-parse-through-clap`, `design@core@exit-code-ladder` and `design@core@transcript-command-in-core`; comments in `path@core@src/cli/create.rs`, `path@core@src/cli/mod.rs`, `path@core@src/main.rs` and `path@core@tests/binary.rs` | the harvest adds the command to the list; each text cites the head for the module, not for its list, and stays true |

The line numbers name this branch's tree when the spec was written.

**Each entry the discussion's search graded `bears`**, 65, with how it bears on the work. The
search ran in round 1, over the three proposals of the owner's request as the work, before the
narrowing threads existed; the reply to round 2 counts about 30 entries that changed a proposal,
by the session's own judgement.

- `design@agent-skills@a-head-is-owed-by-an-entry-test`: `klarch id` is a command line, entry test
  1, so it earns a head at the harvest.
- `design@agent-skills@a-reference-claims-a-revisit`: the snapshot carries each entry's inbound
  list as `show` prints it; constrains nothing the decided shapes do not already meet.
- `design@agent-skills@admission-rules-home`: the installed edits of step 4 are written and
  reviewed under the standard it locates.
- `design@agent-skills@behaviour-additions-need-real-use`: a criterion,
  `criterion@graded-entries-and-id-lookup@behaviour-needs-real-use`.
- `design@agent-skills@capabilities-not-structure`: the two commands are capabilities; constrains
  nothing the decided shapes do not already meet.
- `design@agent-skills@checker-model-in-primer`: its argument, that a command reaches a session
  only where a text names it at the moment of use, is premortem cause 5; step 4 names `id` in the
  design skill's naming paragraph.
- `design@agent-skills@goal-entry-shape`: a criterion, `criterion@graded-entries-and-id-lookup@goal-states-outcome`;
  the goal was drafted as an outcome, and is written.
- `design@agent-skills@goals-bind`: the new goal binds outright; it was put to the owner, and is
  written.
- `design@agent-skills@goals-change-through-goal-setting`: the goal was written under the
  goal-setting skill, before this spec.
- `design@agent-skills@installed-fix-goes-upstream`: the new retrospective question sends each
  answer to the workflow's file, as a finding on the checker.
- `design@agent-skills@one-record-searcher`: its dispatch changes in step 4; listed above.
- `design@agent-skills@premortem-as-watch-points`: rewritten; listed above.
- `design@agent-skills@questions-go-to-their-authority`: the goal went to the owner; the two
  commands and the narrowing threads were argued from the record.
- `design@agent-skills@record-searched-before-the-work`: listed above. Its argument against moving
  the `may-bear` reading to the plan review is
  `argument@graded-entries-and-id-lookup@arg-7`.
- `design@agent-skills@review-judges-additions`: the agent-facing-text axis reviews step 4.
- `design@agent-skills@search-relevance-grade`: rewritten; listed above. Its intake rule is the
  model of `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`, which differs from it on
  one point: at the design grounding, a `may-bear` issue or tripwire is read whole only where a
  proposal touches it, where the intake reads every one.
- `design@agent-skills@searcher-grades-goals`: kept; listed among the decisions the design rests on.
- `design@agent-skills@shipped-text-cites-no-entry`: step 4's installed edits cite no entry.
- `design@agent-skills@statement-names-an-act`: each installed addition names an act: the reading
  of the assembly, the `id` call, the retrospective question.
- `design@agent-skills@thread-slug-is-entry-id`: kept; `klarch id` serves its check, and its rule on
  which collision refuses a slug is unchanged.
- `design@agent-skills@transcript-extraction-is-the-checker-s`: the precedent of a task the
  installed text asks of every project moved into a checker command; the new goal generalises it.
  Constrains nothing the decided shapes do not already meet.
- `design@agent-skills@unrecognised-entries-reach-upstream`: the owner's choice of a route over a
  standing question on cost; the new question answers that cost argument,
  `argument@graded-entries-and-id-lookup@arg-19`.
- `design@core@an-entity-belongs-to-its-anchor`: ids are unique per register and anchor, so
  `klarch id` lists every anchor and the design skill keeps the judgement,
  `argument@graded-entries-and-id-lookup@arg-16`.
- `design@core@arguments-parse-through-clap`: `klarch id` declares its arguments through clap.
- `design@core@backticked-bare-id`: its defined-id set spans every register, plan item, skill,
  agent and section slug; `klarch id` lists the same set from `Entities::by_id`, unfiltered, where
  the lint keeps for each citing document only the candidates it can resolve from there, such as a
  plan item from inside its own plan alone.
- `design@core@declared-command`: every installed mention of a command writes `{{command}}`.
- `design@core@exit-code-ladder`: the exit codes of `klarch id`.
- `design@core@nothing-of-a-project-is-compiled-in`: both commands read the manifest and the
  tree; constrains nothing the decided shapes do not already meet.
- `design@core@one-entity-table`: `klarch id` reads the one table; the snapshot uses the resolver
  `show` uses.
- `design@core@one-question-one-command`: a criterion, `criterion@graded-entries-and-id-lookup@one-command-per-question`.
- `design@core@owned-namespace-check`: the installed edits ship through the install; step 4 runs it.
- `design@core@phases-gate-the-report`: a criterion, `criterion@graded-entries-and-id-lookup@no-answer-over-incomplete-model`.
- `design@core@plan-items-by-section`: plan items are entities of the table, so `klarch id` lists
  them with their plan.
- `design@core@the-core-cli-is-a-library-module`: `klarch id` is added to the library module.
- `design@core@transcript-command-in-core`: the precedent of a task every project's sessions
  scripted, moved into the published checker; constrains nothing the decided shapes do not already
  meet.
- `design@core@two-segment-ids`: the grammar `klarch id` checks.
- `design@core@worklist-writes-and-verifies-the-reports`: rewritten; listed above.
- `design@gates@gates-crate`: the new goal does not bring running cargo or the tests into the
  checker; read whole, constrains nothing in this work.
- `design@knowledge-architect@binary-bundles-workflow`: the commands ship in the binary that
  carries the workflow; constrains nothing the decided shapes do not already meet.
- `design@knowledge-architect@changelog-entries`: the entries of step 4.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: this branch writes them.
- `design@knowledge-architect@versioning-policy`: classes the entries of step 4.
- `goal@agent-skills@installed-text-leaves-room-to-judge`: the installed edits add no structure the
  work does not need; the assembly replaces a sequence of `show` calls, and the session keeps the
  choice of which `may-bear` entries a proposal touches.
- `goal@agent-skills@installed-text-works-anywhere`: a criterion, `criterion@graded-entries-and-id-lookup@installed-text-portable`.
- `goal@agent-skills@no-contradiction-in-workflow`: step 4 edits every installed text that states
  the reading of the search, the searcher, the design, planning and intake skills, so they agree.
- `goal@core@declared-instructions-are-checked`: an instruction to read every graded entry is
  served by a computed file, as the worklist head serves it; constrains nothing more.
- `goal@core@records-reach-their-reader`: a criterion, `criterion@graded-entries-and-id-lookup@listing-from-the-tree`.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: the seed of the search, and the goal
  the new goal refines; the reply to round 2 names this seed as the main cause of the inflated list,
  `argument@graded-entries-and-id-lookup@arg-4`.
- `goal@knowledge-architect@agents-work-without-drift`: the assembly must drop no graded entry and
  no inbound site; AC1 and AC2 test it.
- `goal@knowledge-architect@any-project-can-adopt-it`: both commands read the manifest; the
  question is asked in every project; constrains nothing the decided shapes do not already meet.
- `goal@knowledge-architect@documentation-stays-consistent`: "its commands list and display the
  entries a review must read": the assembly is such a display.
- `goal@knowledge-architect@setup-brings-quality-tools`: read whole; the new goal is about the
  checker's commands, not the tools setup brings; bears on nothing in this work.
- `goal@knowledge-architect@structure-and-workflow-work-together`: "the tool computes the
  workflow's work lists"; the assembly and `klarch id` are such computations.
- `goal@knowledge-architect@the-owner-decides`: the goal was the owner's to set, and was put to the
  owner.
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: the retrospective question is a
  real-use channel; the owner's objection of round 6 rested on it, and the reply to round 6 read it
  as refusing a mock project built for an agent to apply the workflow to.
- `goal@xtask@repeated-tasks-are-automated`: edited on this branch, on the owner's word of round 5.
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: names the
  retrospective's standing questions as the channel from consumers; the new question adds to that
  channel. Read again at the harvest, listed above.
- `issue@agent-skills@patching-an-installed-skill`: cites the head the new question widens; read
  again at the harvest.
- `issue@agent-skills@the-retrospective-counts-no-review-cost`: another proposed change to the
  retrospective's questions; the new question neither closes nor depends on it.
- `issue@core@a-contract-change-fails-every-earlier-commit-unexplained`: an instance of the class
  the new question asks about, a mandated repair scripted by hand; this work does not close it.
- `issue@core@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`: the sibling of
  `klarch id` for plan numbers; it stays open, under "Later consequences".
- `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`: the assembly prints
  the inbound sites of a set of entries, the graded ones, but not of any set; it stays open.
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: the snapshot, the assembly
  and the output of `klarch id` are read by sessions; the module comment of `worklist` already
  declares its files no contract, and `klarch id` takes the same status.
- `rejected@agent-skills@intake-reads-every-may-bear-entry-whole`: the assembly offers
  `may-bear.md` whole; the intake still reads a `may-bear` entry other than an issue or a tripwire
  only where a proposal touches it, so the rejected reading is not brought back.
- `rejected@gates@gates-command-of-the-checker`: the new goal could bring a gates command back;
  this work proposes none.

**Each entry the search graded `may-bear` only**, 137. Under
`thread@graded-entries-and-id-lookup@may-bear-read-on-touch`, which was approved after this
search, the session had already read all 137 whole. The reply to round 2 names six that changed a
proposal:

- `rejected@agent-skills@retrospective-asks-for-unrecognised-lines`: the cost argument against a
  standing question; the new question meets it, `argument@graded-entries-and-id-lookup@arg-19`.
- `rejected@core@commit-subcommand`: a command that is a bad trade; it is why the goal draft says
  the owner decides whether a proposed command is built.
- `rejected@core@verdict-only-output`, `rejected@agent-skills@checker-model-printed-by-binary`,
  `design@agent-skills@staged-check-before-each-commit` and `design@core@anchors-listing-command`:
  named in the reply to round 2 as changing a proposal; the transcript does not say how. Read against
  the decided shapes: the second gives the argument of premortem cause 5, a command reaches a session
  only where a text names it; the fourth is a precedent of a listing command computed as `check`
  computes it, as `klarch id` is; the first and the third constrain nothing the decided shapes do
  not already meet.

Three more were used later in the discussion:

- `tripwire@agent-skills@record-search-misses` and `tripwire@agent-skills@goal-missed-though-graded`:
  premortem causes 2 and 3 fold into them.
- `design@agent-skills@synthetic-evidence-not-built`: the owner's objection of round 6 and its
  correction; listed among the decisions the design rests on.

Six more bear on no decided shape, but cite a head this work rewrites, and are read again at the
harvest, as the table of rewritten decisions above lists:
`design@agent-skills@standing-state-review-before-every-merge`,
`issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`,
`tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`,
`rejected@agent-skills@searches-split-by-kind`, `design@agent-skills@xtask-pins-checker` and
`issue@agent-skills@audit-axes-beyond-the-design-record`.

The other 122 were read whole at the grounding and bear on nothing in this work, by the session's
judgement in the reply to round 2: "The other 131 `may-bear` entries changed nothing. That is my
judgement, not a measurement." Four of them are the discussion's examples of what inflated the list:
`design@agent-skills@content-mirrors-the-install-layout`, the reply to round 2's example of a reason
that "shares the subject"; and `design@core@fix-scope`,
`issue@knowledge-architect@a-mechanical-changelog-check` and
`issue@agent-skills@no-check-compares-an-audit-pass-s-owner-list-count-with-the-previous-pass-s`,
three of the reply to round 3's four examples of entries graded for what the goal might bring later;
its fourth, `rejected@gates@gates-command-of-the-checker`, was graded `bears` and is listed above.
The other 118:
`design@agent-skills@a-scratch-directory-per-subagent`,
`design@agent-skills@audit-axis-runs-a-workflow`, `design@agent-skills@audit-net-growth`,
`design@agent-skills@audit-outcomes`, `design@agent-skills@bounded-path-in-design`,
`design@agent-skills@context-cost-of-agent-text`,
`design@agent-skills@design-hands-off-to-planning`,
`design@agent-skills@design-record-axis-dispatch`, `design@agent-skills@existing-heads-on-touch`,
`design@agent-skills@expectation-set-bounds-scope`, `design@agent-skills@goal-lifecycle`,
`design@agent-skills@harness-shell-fails-on-a-pipe`, `design@agent-skills@in-change-path`,
`design@agent-skills@instruction-record-is-minimal`,
`design@agent-skills@intake-shares-the-admission-standard`,
`design@agent-skills@knowledge-table-home`, `design@agent-skills@ledger-from-transcript`,
`design@agent-skills@losing-alternatives-filter`, `design@agent-skills@no-external-handoff`,
`design@agent-skills@one-run-pass`, `design@agent-skills@outcome-over-display`,
`design@agent-skills@plain-text-is-no-repair`,
`design@agent-skills@plan-document-leaves-at-landing`, `design@agent-skills@primer-limit`,
`design@agent-skills@rejected-citation-direction`,
`design@agent-skills@repairs-read-against-the-record`,
`design@agent-skills@retrospective-destination`,
`design@agent-skills@search-apart-from-the-standing-state-review`,
`design@agent-skills@search-reads-the-whole-record`, `design@agent-skills@shared-text-fragment`,
`design@agent-skills@staged-forms-for-a-partial-commit`,
`design@agent-skills@subject-split-standard`, `design@agent-skills@title-states-the-rule`,
`design@agent-skills@tripwire-admission-test`, `design@agent-skills@tripwire-recording-authority`,
`design@agent-skills@tripwires-read-by-the-audit`, `design@agent-skills@verification-in-primer`,
`design@agent-skills@workflow-axis-finding-standard`, `design@agent-skills@workflow-axis-lenses`,
`design@core@a-foreign-build-is-refused`, `design@core@a-reference-is-live-in-every-prose`,
`design@core@a-slug-elsewhere-defines-nothing`,
`design@core@an-entry-is-a-heading-at-the-register-level`,
`design@core@anchors-are-components-and-locations`, `design@core@api-facade`,
`design@core@check-fix-flag`, `design@core@check-judges-no-given-commit`,
`design@core@clusters-packed-under-a-word-bound`,
`design@core@components-carry-the-same-documents`, `design@core@declarable-directory-register`,
`design@core@finding-names-the-repair`, `design@core@generated-files-are-pure`,
`design@core@heading-register-two-shapes`, `design@core@inputs-are-gathered`,
`design@core@installed-binary-version-check`, `design@core@model-then-checks`,
`design@core@ne-minimal`, `design@core@one-list-of-generated-files`, `design@core@plain-word-lint`,
`design@core@plan-item-scope`, `design@core@plan-register`,
`design@core@record-is-every-register-entry`, `design@core@registers-are-declared`,
`design@core@rejected-register`, `design@core@safe-fix-definition`,
`design@core@section-homes-carry-slugs`, `design@core@staged-tree-source`,
`design@core@transcript-format-tolerated-and-reported`, `design@core@transcript-query-verbs`,
`design@knowledge-architect@committed-findings-analysis`,
`design@knowledge-architect@no-future-breaking-cost-argument`,
`design@knowledge-architect@workflow-findings-stay-here`,
`goal@agent-skills@one-skill-per-activity`, `goal@knowledge-architect@adoption-is-easy`,
`goal@knowledge-architect@design-is-recorded-with-its-arguments`,
`goal@knowledge-architect@rust-projects-come-first`,
`issue@agent-skills@a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise`,
`issue@agent-skills@a-handled-retrospective-file-is-marked-where-it-lies`,
`issue@agent-skills@a-lighter-pass-for-untouched-components`,
`issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`,
`issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own`,
`issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`,
`issue@agent-skills@expectation-sets-for-the-installed-skills`,
`issue@agent-skills@no-check-reports-a-duplicated-workflow-lens-whose-other-agents-found-nothing`,
`issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`,
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`,
`issue@agent-skills@the-agentic-workflow-audit-s-repair-agents-read-the-admission-standard-by-a-pointer`,
`issue@agent-skills@the-cold-reader-standard-s-scope-reads-two-ways`,
`issue@agent-skills@the-design-record-auditor-s-return-names-two-verdict-vocabularies`,
`issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`,
`issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`,
`issue@agent-skills@the-void-marker-s-reason-reads-two-ways`,
`issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`,
`issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`,
`issue@core@a-plain-hash-item-name-is-read-by-nothing`,
`issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`,
`issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`,
`issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`,
`issue@core@core-heads-bundle-decisions-with-many-citing-sites`,
`issue@core@installed-file-findings-belong-in-phase-four`, `issue@core@tooling-for-project-skills`,
`issue@knowledge-architect@ci-does-not-run-on-a-draft-and-no-record-says-why`,
`issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids`,
`rejected@agent-skills@admission-standard-in-primer`,
`rejected@agent-skills@milestone-holds-every-slice-design`,
`rejected@agent-skills@three-runs-per-pass`,
`rejected@agent-skills@two-readings-left-to-judgement`, `rejected@core@fix-converting-old-entries`,
`rejected@core@index-diff-with-write-flag`, `rejected@core@per-document-phase-gate`,
`rejected@core@project-wide-unique-slug`, `rejected@gates@gates-binary-configured-by-file`,
`tripwire@agent-skills@a-shortcut-decision-earns-a-head`,
`tripwire@agent-skills@agent-text-written-without-the-standard`,
`tripwire@agent-skills@rarer-rule-in-the-model`, `tripwire@core@fix-makes-a-choice`,
`tripwire@core@phases-gate-the-report-two`, `tripwire@core@private-item-needed`.

## Criteria

### A listing is computed from the tree, never kept by hand `##listing-from-the-tree`

Binding, from `goal@core@records-reach-their-reader`. Stated in the reply to round 4. Met by
`thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out` and
`thread@graded-entries-and-id-lookup@id-lookup-command`, as the reply to round 5 gives it.

### One question, one command `##one-command-per-question`

Binding as a presumption, from `design@core@one-question-one-command`. Met by
`thread@graded-entries-and-id-lookup@id-lookup-command`, "the only command answering "what defines
this id""; `thread@graded-entries-and-id-lookup@graded-entries-assembled` "adds no command".

### No answer or write over an incomplete model `##no-answer-over-incomplete-model`

Binding as a presumption, from `design@core@phases-gate-the-report`. Met: "`klarch id` exits 2
over one; `--out` already refuses; `--verify` reads no model".

### Installed text names no project's paths and writes the command as `{{command}}` `##installed-text-portable`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met: "every new mention written as
`{{command}}`".

### An instruction about behaviour is added only on real-use evidence `##behaviour-needs-real-use`

Binding as a presumption, from `design@agent-skills@behaviour-additions-need-real-use`. Met by
"this session's measurement; your report of slug loops; five searchers' scripts".

### A goal states an outcome, not a mechanism `##goal-states-outcome`

Binding as a presumption, from `design@agent-skills@goal-entry-shape`. Met by "the approved
draft", now `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

### Context the grounding session spends reading `##grounding-context-spent`

Weighed. From this session's measurement, `argument@graded-entries-and-id-lookup@arg-3`. Met by
`thread@graded-entries-and-id-lookup@may-bear-read-on-touch` and the three narrowing threads,
`thread@graded-entries-and-id-lookup@seeds-name-decisions`,
`thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt` and
`thread@graded-entries-and-id-lookup@one-search-per-question`.

## Threads

### Seeds are only the decisions the work rests on or may change, never a goal `##seeds-name-decisions`

Proposed by the session in the reply to round 2. Approved. Arguments:
`argument@graded-entries-and-id-lookup@arg-4`, `argument@graded-entries-and-id-lookup@arg-6`.
Shape: Decided design, "The search reads less". Harvest: the head that states what the design
skill seeds the search with, `design@agent-skills@record-searched-before-the-work`, as the
decision-recording skill judges. The owner's words, round 3: "seeds-name-decisions,
may-bear-needs-a-named-doubt, goals-listed-not-reshown, may-bear-read-on-touch,
one-search-per-question: I agree with your positions on all five here. I believe this should be
enough to prevent this from happening again."

### A `may-bear` grade names the element this change makes and the sentence of the entry it might conflict with `##may-bear-needs-a-named-doubt`

Proposed by the session in the reply to round 2; widened by the session in the reply to round 3:
the element named is one this change makes, not one it could lead to later. Approved, and the
widening approved. Arguments: `argument@graded-entries-and-id-lookup@arg-5`,
`argument@graded-entries-and-id-lookup@arg-11`. Shape: Decided design, "The search reads less".
Harvest: `design@agent-skills@search-relevance-grade` rewritten. The owner's words, round 3, as
above; on the widening, round 4: "I agree with that".

### Goals appear in the assembly as their grade lines and their site, without their text `##goals-listed-not-reshown`

Proposed by the session in the reply to round 2. Approved. Arguments:
`argument@graded-entries-and-id-lookup@arg-6`. Shape: Decided design, "The assembly". Harvest:
`design@core@worklist-writes-and-verifies-the-reports` and
`design@agent-skills@search-relevance-grade` rewritten. The owner's words, round 3, as above.

### At the design grounding, a `may-bear` entry is read whole where a proposal touches it, and the rest are listed for the review `##may-bear-read-on-touch`

Proposed by the owner in round 2 in a wider form, every `may-bear` entry left to the plan review:
"Instead, the may bear entries can be left to a reviewer at the time of plan review, after using
the planning skill." The session proposed in the reply to round 2 the middle shape, the intake's
rule at the design grounding, and named the part the owner had to rule: narrowing the reading of
issues and tripwires, which `design@agent-skills@search-relevance-grade` records the owner asked to
be read whole. Approved, with that part. Arguments: `argument@graded-entries-and-id-lookup@arg-2`,
`argument@graded-entries-and-id-lookup@arg-7`, `argument@graded-entries-and-id-lookup@arg-8`,
`argument@graded-entries-and-id-lookup@arg-9`. Shape: Decided design, "The search reads less"; the
hand-off of the not-read list is D2. Harvest: `design@agent-skills@search-relevance-grade`
rewritten. The owner's words, round 3, as above.

### A goal set through goal-setting is not part of a design search's work `##one-search-per-question`

Proposed by the session in the reply to round 2: only the goal's design-side consequences are part
of the work, here the retrospective question. Approved. Arguments:
`argument@graded-entries-and-id-lookup@arg-10`, `argument@graded-entries-and-id-lookup@arg-11`.
Shape: Decided design, "The search reads less". Harvest: the head the decision-recording skill
judges it belongs to, `design@agent-skills@record-searched-before-the-work` or
`design@agent-skills@one-record-searcher`. The owner's words, round 3, as above.

### A passing `worklist --verify` writes `bears.md` and `may-bear.md`; a failing one writes nothing `##graded-entries-assembled`

Proposed by the owner in round 1: "assemble mechanically a report holding every entry rated bears
(and maybe one with entries rated may bear) after the grounding step, so that the design session
can easily read it as mandated." Shaped by the session in the reply to round 4. Approved.
Arguments: `argument@graded-entries-and-id-lookup@arg-1`, `argument@graded-entries-and-id-lookup@arg-12`.
Shape: Decided design, "The assembly". Harvest: `design@core@worklist-writes-and-verifies-the-reports`
rewritten. The owner's words, round 5: "graded-entries-assembled accepted".

### `worklist --out` writes the show text of every entry into `shown.md`, and `--verify` assembles from it `##entry-text-snapshot-at-out`

Proposed by the session in the reply to round 4. Approved. Arguments:
`argument@graded-entries-and-id-lookup@arg-13`, `argument@graded-entries-and-id-lookup@arg-14`.
Shape: Decided design, "The snapshot". Harvest:
`design@core@worklist-writes-and-verifies-the-reports` rewritten. The owner's words, round 5:
"entry-text-snapshot-at-out accepted".

### The assembly reads the model again at assembly time `##gather-reads-the-model`

Proposed by the session in the reply to round 4 as the rival of
`thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`. Ruled out, closed `argued` by the
session, uncontested by the owner. Arguments: `argument@graded-entries-and-id-lookup@arg-14`. The
argument that closed it, the reply to round 5: "It either reverses "reads no model" or adds a
command step to every grounding, and its one gain is a reference list current to the minute."
Harvest: `register@core@rejected`, if the tests of
`skill@knowledge-architect-decision-recording@losing-alternatives` admit it.

### `klarch id <id>...` lists what the entity table defines under each id, over every kind and anchor `##id-lookup-command`

Proposed by the owner in round 1: "a command to test that a thread slug is free, to facilitate
choosing thread names during design discussions, for which right now every session writes a small
bash script". Shaped and named by the session in the reply to round 4. Approved. Arguments:
`argument@graded-entries-and-id-lookup@arg-1`, `argument@graded-entries-and-id-lookup@arg-15`,
`argument@graded-entries-and-id-lookup@arg-16`, `argument@graded-entries-and-id-lookup@arg-17`.
Shape: Decided design, "`klarch id`". Harvest: a new head of the core, whose slug is the thread's, by
entry test 1. The owner's words, round 5: "id-lookup-command accepted with this shape and name".

### A standing retrospective question asks for mechanical tasks carried by a script or by hand `##retrospective-asks-for-hand-work`

Proposed by the owner in round 1: "This is something the retrospective skill should ask
explicitely of consumer projects too: whether there was a need to write a script or carry manually
a task that the installed workflow mandates and could be automated." Worded by the session in the
reply to round 4. Approved. Arguments: `argument@graded-entries-and-id-lookup@arg-18`,
`argument@graded-entries-and-id-lookup@arg-19`, `argument@graded-entries-and-id-lookup@arg-20`.
Shape: Decided design, "The retrospective question". Harvest:
`design@agent-skills@premortem-as-watch-points` rewritten. The owner's words, round 5:
"retrospective-asks-for-hand-work accepted."

## Arguments

### The request, and the scope of the goal `##arg-1`

Round 1, the owner. Bears on `thread@graded-entries-and-id-lookup@graded-entries-assembled`,
`thread@graded-entries-and-id-lookup@id-lookup-command`,
`thread@graded-entries-and-id-lookup@retrospective-asks-for-hand-work`. "Two more things I would
like to mechanically automate for the design skill" / "Also, I would like to make it a project goal
that whenever the agentic workflow could be made more efficient with a mechanical tool, one is
proposed and the klarch binary tries to provide it. Maybe the correct scope to apply it is "when the
workflow asks for a large mechanical task", not just any small task. Maybe a condition is that the
task requires multiple steps, or needs writing an intermediate script."

### The grounding read too much `##arg-2`

Round 2, the owner. Bears on `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`.
"reading all of those entries was far too much for a design session, and I do not believe all of
these entries are necessary to judge the change I proposed, this seems like far too much. Maybe the
criteria by which entries are judged to bear is too wide. Also, maybe reading all of these may bear
entries is not needed." / "Or maybe none of that should change and my initial proposals actually do
reach this widely, this is what I want your opinion on."

### What the grounding cost, and what it gave `##arg-3`

The reply to round 2. Bears on `criterion@graded-entries-and-id-lookup@grounding-context-spent`.
"202 entries, 65 graded `bears` and 137 graded `may-bear` only. As `show` prints them, with grade
lines, that is 576 KB. It took 12 chunked reads" / "Entries that changed a proposal, by my count
after reading: about 30. Only 6 of them were `may-bear`" / "Two of those six are cited by a `bears`
entry I read anyway".

### A goal as seed makes every entry citing it unrelatable `##arg-4`

The reply to round 2. Bears on `thread@graded-entries-and-id-lookup@seeds-name-decisions`. "I
seeded `goal@knowledge-architect@agents-get-a-complete-workflow`. An entry that cites a seed can
never be graded `unrelated`. 29 of the 137 `may-bear` entries carry that seed mark" / "The installed
text allows a goal as a seed ("the decisions and goals read so far"), so the text permits this
mistake."

### "Cannot rule out" has no bar `##arg-5`

The reply to round 2. Bears on `thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt`.
"For 80 of the 137 entries, at least one reason says "no bearing established" or "shares the
subject"." The proposal: "`may-bear` requires naming the element of the work and the sentence of
the entry that might conflict. "Shares the subject" becomes `unrelated`."

### Goals are graded once per agent and read twice `##arg-6`

The reply to round 2. Bears on `thread@graded-entries-and-id-lookup@goals-listed-not-reshown`,
`thread@graded-entries-and-id-lookup@seeds-name-decisions`. "18 goals came back, 14 of them
`bears`, mostly "constrains" in a sense that holds for any change. I had already read every goals
home whole, as `design@agent-skills@searcher-grades-goals` requires." The proposal of the
assembly's goal form: "lists goal grades and reasons without the goal text, since the goals homes
are read whole anyway."

### The `may-bear` reading cannot move wholesale to the plan review `##arg-7`

The reply to round 2. Bears on `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`. "the
search runs before the work so that a bearing entry costs a ruling before the work, not rework
after (`design@agent-skills@record-searched-before-the-work`). The one `may-bear` entry that mattered
most here was the cost argument against a standing retrospective question. If it had surfaced only
at plan review, it would have reopened your ruling on the question afterwards. That is the failure
`rejected@agent-skills@conformance-reader-at-the-checkpoint` records. Also, the in-change path has
no plan review, so `may-bear` entries would have no reader there."

### The intake's rule at the design grounding `##arg-8`

The reply to round 2. Bears on `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`. "read
a `may-bear` entry whole only where a proposal touches its subject; list the rest with the
searcher's reason as "not read"; hand that list to the plan's design-conformance reviewer, or to the
review before the merge on the in-change path, as the backstop." / "Here that would have meant
reading about 10 of the 137, the 6 above included."

### Narrowing the reading of issues and tripwires changes a ruling of the owner `##arg-9`

The reply to round 2. Bears on `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`. "that
same head records that you asked for reported issues and tripwires to be read whole rather than from
a summary. 40 of the 137 are issues or tripwires, so narrowing their reading changes your ruling."
The owner ruled it in round 3, with the four other threads.

### The goal was only being added `##arg-10`

Round 3, the owner. Bears on `thread@graded-entries-and-id-lookup@one-search-per-question`. "If i
understand correctly, the main reason this happened was because I was proposing adding a goal, and
one that does reach quite widely, right ? So the searchers marked as bear or may bear most entries
that could end up reshaped by this goal, even if not in scope of this session ? (we are only adding
the goal, it does not have to be met on all instructions after this work)"

### The searchers graded what the work might lead to later `##arg-11`

The reply to round 3. Bears on `thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt`,
`thread@graded-entries-and-id-lookup@one-search-per-question`. "Partly, but not mainly. The goal
accounts for about a third of the `may-bear` entries, and proposals 1 and 2 for more than half." A
keyword match over the reasons, "approximate": of the 65 `bears`, 22 only the goal, 23 only
proposals 1 or 2, 19 both, 1 neither; of the 137 `may-bear` only, 42, 73, 7 and 15. "So there is a
fourth mechanism beside the three I named: the searchers graded what the work might lead to later,
not only what it changes now." / "Adding a goal does not oblige meeting it now." / "I would make
that explicit in its wording: the element must be one this change makes, not one it could lead to
later."

### The assembly's shape `##arg-12`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@graded-entries-assembled`.
"Each entry is printed as `show` prints it" / "Above each entry, every agent's grade line is
listed, identical lines merged." / "An entry graded by several agents goes under its highest grade
(`bears` above `may-bear`). In practice that means goals, which every agent grades. In this search,
`goal@agent-skills@installed-text-leaves-room-to-judge` got 8 `bears` and 3 `may-bear`." / "The table
of contents in `may-bear.md` is what #may-bear-read-on-touch needs."

### The text comes from a snapshot written by `--out` `##arg-13`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`.
"`--out` already holds the model and writes each entry's text into the agents' `entries.md` files.
It would also write one file, `<dir>/shown.md`" / "That file is the session's copy, never given to
an agent, so the agents' word load does not change." / "`--verify` keeps "reads no model"" / "The
session reads exactly the text the searchers graded." / "The cost is one file of roughly the whole
record's `show` output. Here that would be about 2 MB, an estimate from 576 KB for 202 of about 820
entries."

### Reading the model at assembly time loses either way `##arg-14`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@gather-reads-the-model`,
`thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`. "`--verify` itself reads the
model. This reverses that head's "reads no model". A tree that gained a phase-2 finding since
`--out`, such as an edit in progress on the in-change path, would then block the assembly." / "A
third mode, `--gather`, does it. This adds a command step to every grounding." / "Its one gain is a
`referenced at:` list current to the minute. The tree rarely changes in the minutes between `--out`
and `--verify`."

### No other command answers what defines an id, and a free answer over an incomplete table is false `##arg-15`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@id-lookup-command`. "Exit
codes: 0 when every argument is a well-formed id that nothing defines, 1 otherwise, 2 when the model
is incomplete. A "free" answer over a table missing an unread file's entries would be false." / "No
other command answers "what defines this id", so `design@core@one-question-one-command` holds. The
bare-id lint already computes this set internally."

### The command lists every kind and anchor, and the skill keeps the judgement `##arg-16`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@id-lookup-command`. "It lists
every kind and every anchor, and the design skill keeps the judgement." / "Any other collision is
shown and allowed, since ids are unique per register and anchor
(`design@core@an-entity-belongs-to-its-anchor`)." / "The nearest rival is a narrow test, design and
`rejected` entries of one named anchor. It loses because the owning Component is often not settled
when a thread is named. In this discussion, my threads span core and agent-skills."

### The name, and why it earns a head `##arg-17`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@id-lookup-command`. "The name
`id` is my default. It is a command line, so it earns a head under entry test 1."

### The real-use evidence for the retrospective question `##arg-18`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@retrospective-asks-for-hand-work`,
`criterion@graded-entries-and-id-lookup@behaviour-needs-real-use`. "your observation that every
session writes a slug-check loop; my own assembly script in this session; five searchers' fill
scripts in this session." The reply to round 2 counted them: "5 of the 10 searchers wrote their own
script to fill in their report".

### The cost argument against a standing question does not hold here `##arg-19`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@retrospective-asks-for-hand-work`.
"`rejected@agent-skills@retrospective-asks-for-unrecognised-lines` lost because every retrospective
would load and answer it while most sessions never ran the command. That asymmetry does not hold
here: every session following the workflow runs its procedures."

### The question watches a goal, and nothing else sees consumer sessions `##arg-20`

The reply to round 4. Bears on `thread@graded-entries-and-id-lookup@retrospective-asks-for-hand-work`.
"`design@agent-skills@premortem-as-watch-points` says each question watches a decision. This one
watches a goal. Its argument (seen in real sessions before any check could see it) covers a goal
too, so the head's member set widens to "a decision or a goal"." / "The nearest rival is no
question, relying on your own observation. It loses because consumer sessions are seen only through
their retrospectives."

### The goal's draft, part by part `##arg-21`

The reply to round 4. Bears on the goal `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
already written. "The aim and "the klarch binary tries to provide it" are your statement." /
""Several steps or a script" is your hedged scope, which I adopted." / ""With no judgement" as the
test of "mechanical" is my proposal. It keeps out tasks such as grading an entry." / ""The owner
decides whether it is built" is my proposal. It matches
`goal@knowledge-architect@the-workflow-improves-through-real-use`, and it keeps out a command that is
a bad trade, such as `rejected@core@commit-subcommand`." The reply to round 2 found that
`goal@xtask@repeated-tasks-are-automated` "already says that a task the installed text asks of
every project is the published checker's to automate", so the new goal rewrote it.

### AC1 and AC2 against real use, and tripwires instead `##arg-22`

Round 6, the owner. Bears on `acceptance@graded-entries-and-id-lookup@assembly-places-each-entry-once`,
`acceptance@graded-entries-and-id-lookup@shown-text-equals-show`. "AC1 and AC2 go against the
improve through real use goal, and no-synthetic-evidence design head. Instead, I think tripwires
watching future behaviors of grounding on a real worklist and task would be more appropriate here."

### The head admits the checker tested over mock projects `##arg-23`

The reply to round 6. Bears on the same items. "`design@agent-skills@synthetic-evidence-not-built`
refuses runs that observe how an agent follows the workflow. Its "Admitted" bullet allows these
tests" / "None of the acceptance criteria runs an agent. Each runs `klarch` over a fixture under
`path@core@tests/projects/`, as every core test does. In AC1, the "graded reports" are input files
the test writes. They are a planted fixture, not an agent's output." The "under
`path@core@tests/projects/`" is imprecise: the binary's tests copy a project from there or write
one, and a unit test states its project as text; none runs an agent.

### A silent failure of the assembly is a test's, not a tripwire's `##arg-24`

The reply to round 6. Bears on the same items, and on the premortem. "Suppose the assembly drops an
entry, or merges two grade lines wrongly. The session never sees an entry it was never shown, so the
failure is silent." / "The tripwire standard sends that case to a test, not a tripwire. Its
condition 1 says: "Where a script could compute the firing, write that check instead. A check of the
work being built is that work's acceptance criterion.""

### The synthetic-evidence head belongs in the root CLAUDE.md `##arg-25`

Round 6, the owner, and the reply. "One question, is  no-synthetic-evidence mentionned in root
CLAUDE.md ? If not, it probably should" / the reply: "No. A search of CLAUDE.md and the primer for
"synthetic" and "real use" returns nothing." / "In the installed text, it reaches sessions in two
weak ways: `%%` comments in the design skill and the intake skill, which the build removes; two
general sentences: the design skill's "Evidence that the project's goals or design heads rule out is
not built", and the intake's "No subagent replays the reported session or builds a scenario of agent
behaviour"." / "The sentence must carry the admitted boundary, or a reader will make the reading
above". Round 7, the owner: "The synthetic evidence head should still
be mentionned in root CLAUDE.md anyway, since it was previously not followed, as a small commit
during the work." Carried out on this branch, before this spec.

### The owner's correction `##arg-26`

Round 7, the owner. Bears on AC1 to AC4. "Oh right, I had misunderstood your proposals here, sorry
for that. Keep AC1 to 4." / "I was being overly cautious when reading your proposals due to these
past events. Proceed"

## New names, in one place

An illustration of the shapes, not authority; the implementing session decides the final names.

```text
crates/core/src/cli/worklist.rs
  const SHOWN_FILE = "shown.md"        new: the snapshot, written by --out
  const BEARS_FILE = "bears.md"        new: the assembly's bears file, written by a passing --verify
  const MAY_BEAR_FILE = "may-bear.md"  new: the assembly's may-bear file, with its table of contents
  fn render                            existing; the entries files and reports it returns unchanged
  fn verify                            existing; keeps every grade line it judges, and writes the
                                       assembly when it finds nothing
  fn assemble(dir, shown, grades)      new: the two files from the snapshot and the grade lines

crates/core/src/cli/mod.rs
  Command::Id(IdArgs)                  new: `klarch id <id>...`
  struct IdArgs { ids: Vec<String> }   new: one or more ids, positional
  fn show                              existing; its printing of a register entry and its inbound
                                       sites is shared with the snapshot, so both print one text
  fn id                                new, here or in a module of its own under cli/

crates/core/src/entity.rs
  Entities::by_id, is_entry_id         existing; what `klarch id` reads
```

## Decided design

### The snapshot

`worklist --out <dir>` writes `<dir>/shown.md` beside `agents.txt` and the agent directories. It
holds, for every entry of the record the work list holds, one entry per agent line's reference with
no repeat, the show text of that entry, as `show` prints it over the same model: the entry's line,
its body, and its `referenced at:` list or `referenced by nothing`. The text is computed by the code
`show` uses, not by a copy of it, so the two cannot drift: `show`'s printing becomes a function that
returns the show text, which `show` prints and `--out` writes. The inbound sites of every entry are
gathered in one pass over the observations, rather than one scan per entry as `records::inbound`
does for `show`'s one entry.

The snapshot's shape is the entries file's: for each entry, a line `=== <reference>`, a blank line,
then the show text in a fence longer than any run of backticks it holds, as `render` fences a body.
`--verify` finds an entry's text by the reference on its `===` line and writes the text without the
fence. An illustration of one entry, not authority:

````text
=== <kind>@<anchor>@<id>

```text
<kind>@<anchor>@<id>  <file>:<line>

<the entry's body>

referenced at:
  <file>:<line>
```
````

- The snapshot is written with the other files, after every refusal, so a refused run still writes
  nothing.
- It is never given to an agent: the entries files and the reports are byte-identical to what
  `--out` writes today. AC3 judges it.
- Its size is about the whole record's `show` output, an estimate of 2 MB in this repository,
  `argument@graded-entries-and-id-lookup@arg-13`.

Nearest rival: `thread@graded-entries-and-id-lookup@gather-reads-the-model`. The fact that defeated
it: either form reverses "reads no model" of `design@core@worklist-writes-and-verifies-the-reports`
or adds a command step to every grounding, for a gain of an inbound list current to the minute,
`argument@graded-entries-and-id-lookup@arg-14`.

### The assembly

When `worklist --verify <dir>` finds nothing, it writes `<dir>/bears.md` and `<dir>/may-bear.md`,
then prints its `PASSED` line, naming the two files. When it finds anything, or stops with an error,
it writes neither, removes the two files a previous run left, and prints what it prints today, so a
session never reads an assembly from reports that no longer pass. It still reads no model: the text
comes from the snapshot, the grades from the reports.

- **Which file.** Each entry goes under its highest grade over every report that grades it: `bears`
  to `bears.md`, `may-bear` to `may-bear.md`, `unrelated` to neither. An entry appears once in all.
- **Above each entry**, every grade line of every agent that graded it `bears` or `may-bear`, each
  with its agent's name, identical grade lines merged into one line naming every agent that gave it.
- **A goal** appears as its grade lines and its site only, without its body and without its inbound
  list, per `thread@graded-entries-and-id-lookup@goals-listed-not-reshown`: the session reads every
  goals home whole at the grounding, per `design@agent-skills@searcher-grades-goals`.
- **`may-bear.md` opens with a table of contents**: one row per entry, its reference and its
  `may-bear` reasons, so the session picks the entries a proposal touches.
- **Order**: the entries in the order of `agents.txt`, the first agent line of each entry deciding
  its place, which needs a structure that keeps line order where `verify` holds a `BTreeMap` today.
  The order is the implementing session's to change, as long as it is fixed by the inputs.
- **A rerun** after a passing one overwrites both files.
- **A snapshot missing, unreadable, or lacking an entry `agents.txt` lists**, as in a directory an
  `--out` older than this work wrote: the reports are judged as today, and when they pass, no
  assembly is written, the `PASSED` line is not printed, and `--verify` says what the snapshot lacks
  and that `--out` must run again, and exits 2, since it could not do the whole of its job, per
  `design@core@exit-code-ladder`.

An illustration of the shape of one entry of `bears.md`, not authority:

```text
=== <kind>@<anchor>@<id>
agent-3, agent-7: grade: bears | relation: constrains | reason: <reason>
agent-5: grade: bears | relation: may change | reason: <another reason>

<kind>@<anchor>@<id>  <file>:<line>

<the entry's body>
referenced at:
  ...
```

Nearest rival: the session's own reading, one `show` call per graded entry, as today. The fact that
defeated it: the owner's request of round 1, and the round-1 session wrote its own assembly script,
`argument@graded-entries-and-id-lookup@arg-18`; a script each session writes again is what
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s` assigns to the tool.

### `klarch id`

`klarch id <id>...` takes one or more arguments. For each, in the order given, it prints one line
or a block:

- an argument outside the id grammar: the argument, then
  `not an id: an id matches [a-z0-9]+(-[a-z0-9]+)+`;
- a well-formed id nothing defines: the argument, then `free`;
- a well-formed id the entity table defines: the argument, then every entity defined under it, one
  per line, each as the reference `show` accepts for it: a register entry of any anchor, a plan
  item with its plan as anchor, a skill or an agent by its name, and a section slug with its owner,
  as `Entities::by_id` lists them.

An illustration of the output, after the reply to round 4, its slugs written as placeholders, not
authority:

```text
$ klarch id <free-slug> <taken-slug> two
<free-slug>   free
<taken-slug>  design@<anchor>@<taken-slug>
two           not an id: an id matches [a-z0-9]+(-[a-z0-9]+)+
```

**Exit codes**: 0 when every argument is well-formed and free; 1 otherwise, a malformed argument
included, as "What is already decided" argues under `design@core@exit-code-ladder`; 2 when it
cannot run, an incomplete model included. It builds the model and runs the first three phases, as
`complete_working_tree` does for a writer, with a message of its own, since it writes nothing: a
"free" over a table missing an unread file's entries would be false,
`argument@graded-entries-and-id-lookup@arg-15`. Under `harness = []` the table defines no skill,
agent or section, so `klarch id` lists none.

**Scope**: every kind and every anchor. The design skill keeps its rule,
`design@agent-skills@thread-slug-is-entry-id`: only a design or `rejected` entry of the owning
Component refuses a slug; any other collision is shown and allowed.

Nearest rival: a narrow test of the design and `rejected` entries of one named anchor. The fact
that defeated it: the owning Component is often not settled when a thread is named,
`argument@graded-entries-and-id-lookup@arg-16`.

### The search reads less

Four changes to the installed text, each a thread approved in round 3, the widening in round 4:

- **A named doubt**, `thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt`. In the
  record searcher, the `may-bear` row of
  `agent@knowledge-architect-record-searcher@grade-each-entry` becomes an entry where a named element
  this change makes might conflict with a named sentence of the entry; an entry that only shares the
  work's subject, or that the work might lead to change later, is `unrelated`. The opening sentence
  "Grade `may-bear` an entry you cannot rule out, with the doubt as its reason" and the `may-bear`
  reason of `agent@knowledge-architect-record-searcher@fill-the-report` follow it. An entry that is
  or cites a seed stays `bears` or `may-bear`, as `--verify` checks.
- **Seeds**, `thread@graded-entries-and-id-lookup@seeds-name-decisions`. Step 1 of
  `skill@knowledge-architect-design@discussion-loop` seeds the search with the decisions read so far
  that the question rests on or may change, never a goal.
- **One search per question**, `thread@graded-entries-and-id-lookup@one-search-per-question`. In the
  same step: a goal the discussion sets through `skill@knowledge-architect-goal-setting` is not part
  of the search's work; its consequences on the design are.
- **Read on touch**, `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`. In the same step:
  the session reads `bears.md` whole, reads whole the entries of `may-bear.md` a proposal touches,
  chosen from its table of contents, and lists the others with the searcher's reason as not read.
  The first round states which were read. The not-read list goes to the review that reads the work
  against the record, as D2 says.

The searcher's description, which carries the dispatch, ends with the reading of the assembly:
`bears.md` whole, and `may-bear.md` as the dispatching skill says. The design audit, point 2 of
`skill@knowledge-architect-planning@working-a-slice`, and the intake, point 3 of
`skill@knowledge-architect-retrospective-intake@ground-before-judging`, read from the assembly,
each under its own reading rule, which this work does not change; the design audit's seeds are D3's.

Nearest rival: the owner's proposal of round 2, every `may-bear` entry left to the plan review. The
fact that defeated it: a bearing entry found at the plan review reopens a ruling, as
`rejected@agent-skills@conformance-reader-at-the-checkpoint` records, and the in-change path has no
plan review, `argument@graded-entries-and-id-lookup@arg-7`.

### The retrospective question

A fifth standing question in `skill@knowledge-architect-retrospective@what-it-examines`, worded as
the owner approved it in round 5:

> Did this session, or one of its subagents, write a script or carry by hand over several steps a
> task the installed workflow asks of it, whose result follows from the tree and the workflow's
> rules with no judgement? For each: the task, the instruction that asks it, and the steps or the
> script. Each is a finding on the checker, in the workflow's file.

It watches `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, a goal, where the four
others each watch a decision; `design@agent-skills@premortem-as-watch-points` is rewritten to "a
decision or a goal" at the harvest. Nearest rival: no question, relying on the owner's own
observation. The fact that defeated it: consumer sessions are seen only through their
retrospectives, `argument@graded-entries-and-id-lookup@arg-20`.

## Mapping tables

Where an entry goes in the assembly, by the set of grades the reports give it:

| grades given | `bears.md` | `may-bear.md` |
| --- | --- | --- |
| at least one `bears` | the entry, with its `bears` and `may-bear` grade lines | nothing |
| `may-bear`, no `bears` | nothing | the entry, with its grade lines, and its row in the table of contents |
| `unrelated` only | nothing | nothing |
| a goal, in either of the first two rows | in the file its row gives, its grade lines and its site only, with no body and no inbound list | the same, where its row gives this file |

What `klarch id` prints per entity `Entities::by_id` lists under an id, by kind:

| kind | printed |
| --- | --- |
| a register entry, of any declared or built-in register | `<kind>@<anchor>@<id>` |
| a plan item: a thread, an argument, a criterion, an acceptance criterion | `<kind>@<plan>@<id>` |
| a plan document, a spec or a milestone | `spec@plans@<id>`, `milestone@plans@<id>`, or `spec@<milestone>@<id>` |
| a skill or an agent | `skill@<id>` or `agent@<id>` |
| a section of a skill or an agent | `skill@<owner>@<id>` or `agent@<owner>@<id>` |
| a section of the primer or of the root instructions | `primer@<id>` or `instructions@<id>` |

A kind the table holds that this table misses is printed in the form `show` accepts for it; the
implementing session adds its row here.

## Losing alternatives

- **The assembly reads the model again at assembly time**, in `--verify` or in a third mode
  `--gather`. Lost to `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out` on
  `argument@graded-entries-and-id-lookup@arg-14`.
- **A narrow slug test of the design and `rejected` entries of one named anchor.** Lost to
  `thread@graded-entries-and-id-lookup@id-lookup-command` on
  `argument@graded-entries-and-id-lookup@arg-16`.
- **Every `may-bear` entry left to the plan review**, the owner's proposal of round 2. Superseded by
  `thread@graded-entries-and-id-lookup@may-bear-read-on-touch` on
  `argument@graded-entries-and-id-lookup@arg-7`.
- **No retrospective question, the owner's own observation instead.** Lost to
  `thread@graded-entries-and-id-lookup@retrospective-asks-for-hand-work` on
  `argument@graded-entries-and-id-lookup@arg-20`.
- **The incumbents this work reverses**, each judged at the harvest by the tests of
  `skill@knowledge-architect-decision-recording@losing-alternatives`, as the reversal rule asks:
  - reading every `may-bear` entry whole at the design grounding, refuted by the measurement of
    `argument@graded-entries-and-id-lookup@arg-3`, 202 entries and 576 KB for about 30 that changed
    a proposal, 6 of them `may-bear`; beside it stands
    `rejected@agent-skills@intake-reads-every-may-bear-entry-whole`, refuted the same way;
  - `may-bear` as an entry the searcher "cannot rule out", lost to
    `thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt` on
    `argument@graded-entries-and-id-lookup@arg-5` and `argument@graded-entries-and-id-lookup@arg-11`;
  - goals among the seeds of the design grounding, lost to
    `thread@graded-entries-and-id-lookup@seeds-name-decisions` on
    `argument@graded-entries-and-id-lookup@arg-4`.
- **Tripwires in place of AC1 and AC2**, the owner's proposal of round 6. Withdrawn by the owner in
  round 7, on `argument@graded-entries-and-id-lookup@arg-23`, the owner's misreading of the
  proposals; `argument@graded-entries-and-id-lookup@arg-24` is the session's, and bears on it.

## Readings

None. The work reads no external specification.

## Premortem

Assume the work shipped and failed. The causes, as the reply to round 5 gives them; the verdicts,
as the reply to round 6 read them, to be judged at the harvest by
`skill@knowledge-architect-decision-recording@ruled-tripwires`:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| 1. "the session picks too few `may-bear` entries. One that bore surfaces only at the plan review, after your rulings, which is the rework the search exists to prevent." | `thread@graded-entries-and-id-lookup@may-bear-read-on-touch` | named as a cause for the harvest to judge as a tripwire. The cause is the session's, named in the reply to round 5; the owner's proposal of round 6, "tripwires watching future behaviors of grounding on a real worklist and task", bears on it, and the reply read the cause as "Likely admitted. It would fire when, in two separate pieces of work, a plan review or a merge review finds a `may-bear` entry listed as not read that a decided thread contradicts." The owner's word admits no tripwire alone |
| 2. "searchers grade `unrelated` an entry that bore, so recall falls." | `thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt` | folded into `tripwire@agent-skills@record-search-misses`; the harvest judges whether its wording or its horizon changes |
| 3. "a goal the work strains is no longer seeded and gets missed." | `thread@graded-entries-and-id-lookup@seeds-name-decisions` | folded into `tripwire@agent-skills@goal-missed-though-graded`, beside the session's reading of every goals home; the harvest judges whether its wording or its horizon changes |
| 4. "the session reads text that went stale because it edited an entry between `--out` and `--verify`." | `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out` | named as a cause for the harvest to judge as a tripwire: "In doubt. A short horizon at most." |
| 5. "sessions keep writing loops, because a command reaches a session only where the text names it at the moment of use" | `thread@graded-entries-and-id-lookup@id-lookup-command` | named for the harvest to judge; the reply to round 6 read it as dropped, since the failure is not silent: the new retrospective question reports it, condition 1 |
| 6. "the answers either fill with trivial items, a single grep for instance, or come back "none" while sessions did write scripts." | `thread@graded-entries-and-id-lookup@retrospective-asks-for-hand-work` | named for the harvest to judge, each half on its own condition. The reply to round 6 read it as dropped on condition 2, which fits the second half only: an answer "none" while a session wrote a script leaves its evidence in the transcript alone. The first half's answers land in the retrospective files the intake receives, so condition 2 admits them; condition 1 likely excludes it, since the intake judges each finding before any becomes work |

AC1 to AC4 were proposed with the premortem, in the reply to round 5, and kept on the owner's word
of round 7: "Keep AC1 to 4."

## Acceptance criteria

### AC1: a passing `--verify` places each graded entry once, under its highest grade, and a failing one writes neither file `##assembly-places-each-entry-once`

Guards `thread@graded-entries-and-id-lookup@graded-entries-assembled`, and
`thread@graded-entries-and-id-lookup@goals-listed-not-reshown`. Judged at step 2. Over a work list
directory the test writes whole, `agents.txt`, the snapshot and the reports, where two agents grade
one goal `bears` and `may-bear` and one head `may-bear`, a passing `--verify` writes the goal once,
under `bears.md`, with both grade lines and no goal text, and the head in `may-bear.md`, listed in
its table of contents. A `--verify` that fails leaves neither file, a previous run's included.
`--verify` reads no model, so the directory needs no project behind it; `worklist_project` gives
every goal to one agent at the default bound and at `--bound 60`, so it cannot hold this case.
Fires on any other placement, on a missing or duplicated entry or grade line, on goal text in the
assembly, or on either file present after a failing run.
Response: reopen `thread@graded-entries-and-id-lookup@graded-entries-assembled`. Applied on the
owner's word, round 7.

### AC2: `shown.md` holds, for every entry, the text and the `referenced at:` list `show` prints `##shown-text-equals-show`

Guards `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`. Judged at step 1. For every
entry of a mock project, `shown.md` holds the same text and `referenced at:` list that `show` prints
over the same tree. Fires on any entry whose text or list differs, or that is missing. Response:
reopen `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`. Applied on the owner's
word, round 7.

### AC3: the entries files `--out` writes are byte-identical before and after the change `##agents-entries-unchanged`

Guards `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`. Judged at step 1. The
`entries.md` files `--out` writes over a mock project are byte-identical to those the binary before
the change writes over it, so the agents' load does not change. The reference copy is taken first:
the step's first commit adds, beside the worklist tests, the `entries.md` files the unchanged binary
writes over `worklist_project` with a fixed seed and bound, and the test that compares a fresh
`--out` with them, passing before any change to `write` or `render`. Fires on any byte that
differs.
Response: reopen `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out`. Applied on the
owner's word, round 7.

### AC4: `klarch id` answers free, defined and malformed ids, and refuses an incomplete model `##id-lookup-answers-and-exits`

Guards `thread@graded-entries-and-id-lookup@id-lookup-command`. Judged at step 3. Over a mock
project: a free id exits 0; an id defined by a design entry in one anchor and an issue in another
lists both and exits 1; a one-segment argument is reported as not an id and exits 1; a tree with a
phase-2 finding exits 2. Fires when any of the four does otherwise. Response: reopen
`thread@graded-entries-and-id-lookup@id-lookup-command`. Applied on the owner's word, round 7.

## Implementation sequence

Steps 1 to 3 follow the project's development procedure, `skill@klarch-development`, and pass the
gates they owe. Step 4 edits the installed text, which that skill excludes: it follows
`path@agent-skills@CLAUDE.md`, its section "Editing an installed skill or agent", read whole, and
`skill@knowledge-architect-agent-configuration`. The `worklist` tests of `path@core@tests/binary.rs`
build their project with `worklist_project`; the new tests follow them. Each default under
"Defaults awaiting the owner" is built as written until the owner rules on it.

1. **The snapshot.**
   - Builds: `shown.md`, written by `write` in `path@core@src/cli/worklist.rs` with the other files;
     the text computed by the code `show` uses, shared rather than copied. Nothing reads the
     snapshot yet.
   - Claims: AC2, by a test that runs `--out` over a mock project and compares each entry of
     `shown.md` with the output of `show` on its reference; shown to fail by dropping the inbound
     list from the snapshot. AC3, by the comparison AC3's item describes, its reference copy
     committed first; shown to fail by changing one byte of `render`'s entry line. A refused `--out` still writes nothing:
     `worklist_refuses_an_incomplete_model_and_writes_nothing` and
     `worklist_refuses_a_directory_it_must_not_write_and_an_unknown_seed` pass unchanged.
   - Fails alone on: a snapshot whose text differs from `show`'s.
2. **The assembly.**
   - Builds: `verify` keeps the grade lines it judges, in the order of `agents.txt`, and, when it
     finds nothing, writes `bears.md` and `may-bear.md` from the snapshot, per "The assembly"; the
     removal of both files on a failing or stopped run; the message and exit 2 of a snapshot missing,
     unreadable or lacking an entry.
   - Claims: AC1, by a test over a work list directory it writes whole, giving the grades AC1 names;
     shown to fail by placing an entry under its first grade instead of its highest, and by a
     failing run that leaves a previous run's files. The assembly reads no model: a test edits an
     entry in the tree after `--out`, and `bears.md` holds the text of the snapshot; shown to fail
     against an assembly that calls `show`. A snapshot lacking an entry: a test removes one entry from
     `shown.md` after `--out`, fills the reports to pass, and asserts exit 2, no `PASSED` line and no
     assembly; shown to fail against a `--verify` that skips the missing entry. The existing tests of
     `--verify` pass with their outputs unchanged but for the `PASSED` line.
   - Fails alone on: an entry placed twice or missing, or goal text in the assembly.
3. **`klarch id`.**
   - Builds: `Command::Id` and its arguments, the first three phases before the answer, the output
     of "`klarch id`" from `Entities::by_id` and `is_entry_id`.
   - Claims: AC4, by a test over a project built with `Sandbox` holding the two definitions and a
     phase-2 defect in a copy; shown to fail by an implementation that skips the phase gate, which
     answers `free` over the defect. `cli_declaration_is_consistent` in `path@core@src/main.rs`
     passes with the new command, and `run` in `path@core@src/cli/mod.rs` dispatches it.
   - Fails alone on: a defined id reported free, or an answer over an incomplete model.
4. **The installed text and the documentation.**
   - Builds, in the installed text, each passage named in "What the work is". The installed copies
     differ from the sources from the first edit until the install runs, and phase 2 reports them,
     so run `cargo klarch install-agent-skills` before any `klarch id` or `worklist --out` in this
     step:
     - the record searcher: the `may-bear` definition, per "The search reads less"; the body's
       sentences "The session that sent you reads whole the entries you grade `bears` or
       `may-bear`, as its skill says" and "An entry you grade `may-bear` that does not bear costs the
       session at most one read", brought to the reading of the dispatching skill; its description's
       last sentence reads `bears.md` whole and `may-bear.md` as the dispatching skill says;
     - the design skill: step 1 of `skill@knowledge-architect-design@discussion-loop`, for the four
       changes of "The search reads less", the reading pointing to the assembly; the naming
       paragraph of `skill@knowledge-architect-design@threads-and-states` checks every candidate
       slug with one call, `{{command}} id <slug>...`, and keeps its rule on which collision refuses
       a slug; under D2's default, the in-change path of
       `skill@knowledge-architect-design@decision-authority` lists the not-read entries in the commit
       message that carries the deliberation;
     - the planning skill: point 2 of `skill@knowledge-architect-planning@working-a-slice` reads from
       the assembly, and, under D3's default, seeds the search with decisions alone; under D2's
       default, the row "What is already decided" of
       `skill@knowledge-architect-planning@spec-contents` admits "not read" for a `may-bear` entry,
       with the searcher's reason;
     - the design-conformance reviewer, under D2's default: its sections
       `agent@knowledge-architect-design-conformance-reviewer@find-the-record` and
       `agent@knowledge-architect-design-conformance-reviewer@read-against-record` read whole each
       entry a document or a commit message lists as not read, and its description admits the review
       before the merge of an in-change deliberation;
     - the review skill, under D2's default: the design-conformance row of
       `skill@knowledge-architect-review@review-axes` applies at the review before the merge of a
       branch whose commit carries an in-change deliberation with a not-read list;
     - the intake skill: point 3 of `skill@knowledge-architect-retrospective-intake@ground-before-judging`
       reads from the assembly, its rule unchanged;
     - the retrospective skill: the question of "The retrospective question", verbatim, among the
       standing questions of `skill@knowledge-architect-retrospective@what-it-examines`.
   - Then `cargo klarch install-agent-skills`, and the installed copies committed.
   - Builds, in the documentation: `path@core@README.md`, its command listing, its section
     `worklist` for the snapshot and the assembly, a section or a listing line for `id`, and its
     exit-code table: `id`'s three outcomes, and `--verify`'s exit 2 on a snapshot it lacks.
   - Builds, in the `Next release` section of CHANGELOG.md, after reading
     `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`
     whole, then `cargo x changelog`:
     - New features: `cli`, minor: `klarch id <id>...` says whether each id is free, and lists what
       defines it;
     - New features: `library`, minor: `cli::Command::Id` and its arguments' type, in the shape of
       the existing entry for `cli::Command::Worklist` and `cli::WorklistArgs`;
     - `worklist --out` writing `shown.md` and a passing `worklist --verify` writing `bears.md` and
       `may-bear.md`: `worklist` is not released yet, so the work edits its entry in `Next release`
       rather than writing one, per `design@knowledge-architect@changelog-entries`;
     - Workflow: `agent-skills`, patch: the retrospective asks a fifth standing question, on
       mechanical tasks carried by a script or by hand;
     - Workflow: `agent-skills`, patch: the design session reads the search's `bears` entries from
       one file, and a `may-bear` entry only where a proposal touches it, the others listed as not
       read; a searcher grades `may-bear` only on a named doubt;
     - Workflow: `agent-skills`, patch: the design skill seeds the search with decisions only, and,
       under D3's default, the design audit too;
     - Workflow: `agent-skills`, patch, under D2's default: the design-conformance reviewer reads the
       entries listed as not read, and runs at the review before the merge of an in-change
       deliberation.
     The tests of the two heads decide each entry and its class; a line they exclude is dropped.
   - Claims: `cargo klarch check` passes with the installed copies committed;
     `cargo test -p knowledge-architect-agent-skills` passes; the installed edits cite no entry and
     write `{{command}}`, by a reading; the changelog copies equal the root file, by the test that
     compares them.
   - Fails alone on: an installed text that still sends a session to `show` per graded entry, or a
     changelog copy that differs.
5. **The harvest**, below, reviewed before the merge. The spec is deleted in a commit of its own
   after that review.

## Order rationale

Step 1 before step 2: the assembly reads the snapshot that step 1 proves. Step 2 before step 3:
`klarch id` is independent of the worklist, and comes after it so the two worklist steps are read
together. Step 3 before step 4: the installed text names the commands and files steps 1 to 3 built.
Step 4 before step 5: the harvest records the decisions the built work implements.

## Defaults awaiting the owner

- **D1**, on the plan document's kind, from the author. The planning skill,
  `skill@knowledge-architect-planning@spec-or-milestone`, gives the choice between a spec and a
  milestone to the owner: "Propose which one, with the reason, and let the owner rule." The session
  chose a spec after the owner's "Proceed" of round 7, without putting it to the owner. The default:
  a spec, since the work is one branch and one pull request. The alternative: a milestone, with the
  design split by slice. **Ruled**: the owner, after round 7: "Agreed on the spec shape, to land in
  a single PR with the work".
- **D2**, on `thread@graded-entries-and-id-lookup@may-bear-read-on-touch`, from the author when the
  spec was first written. The approved shape hands the not-read list "to the plan's
  design-conformance reviewer, or to the review before the merge on the in-change path". Today no
  text carries it there: the row "What is already decided" of
  `skill@knowledge-architect-planning@spec-contents` asks, for each graded entry, how it bears or
  that it bears on nothing, which an unread entry cannot state; the design-conformance reviewer reads
  every goals, design and `rejected` home of the Components a document touches, but no issue, no
  tripwire and no list of entries not read, and its scope is a plan document; and no axis of the
  review before the merge, `skill@knowledge-architect-review@review-axes`, reads the record against
  an in-change deliberation. The default, which adds an obligation to the approved shape and widens
  `design@agent-skills@plan-read-against-the-record`, rewritten at the harvest:
  - the row admits, for a `may-bear` entry, "not read", with the searcher's reason;
  - the design-conformance reviewer reads whole each entry a document lists as not read, issues and
    tripwires included, and reports one a decided shape contradicts;
  - on the in-change path, the commit that carries the deliberation lists the not-read entries, and
    the review before the merge dispatches the design-conformance reviewer on that list and the
    branch's diff.

  The alternative: the not-read list is recorded, as above, and read by no reviewer; premortem cause
  1 is then watched only by the tripwire the harvest may write. **Ruled**: the owner, round 9,
  "Agreed on all defaults here, Q1-5 and D2 D3, proceed": the default.
- **D3**, on `thread@graded-entries-and-id-lookup@seeds-name-decisions`, from the author at the
  review of the spec. The owner approved the seeds rule for step 1 of
  `skill@knowledge-architect-design@discussion-loop`. The design audit, point 2 of
  `skill@knowledge-architect-planning@working-a-slice`, seeds the same search, for a slice, with the
  decisions of its milestone document and "the decisions and goals the slice's spec cites", and for
  a spec with its own "What is already decided", which can hold goals, as this spec's does; a goal
  seed inflates its grades by the mechanism of `argument@graded-entries-and-id-lookup@arg-4`. The
  default, which widens the owner's ruling: the audit seeds with the decisions alone, for a slice and
  for a spec, as step 1 does. The alternative: the audit keeps its goal seeds, and the inflation
  stays there until a real audit shows it. The intake's seeds, the received files' own citations,
  are outside both. **Ruled**: the owner, round 9, the same words: the default.

## Harvest

At step 5, under `skill@knowledge-architect-decision-recording` and
`skill@knowledge-architect-issue-tracking`. The recording tests decide whether each decision and
each alternative earns an entry; an item of this row they exclude is named in the harvest's
commit, with the test it fails.

| item | home |
| --- | --- |
| `thread@graded-entries-and-id-lookup@entry-text-snapshot-at-out` and `thread@graded-entries-and-id-lookup@graded-entries-assembled` | `design@core@worklist-writes-and-verifies-the-reports` rewritten in place: the snapshot, the assembly, "reads no model" kept; each text listed under "What is already decided" read again |
| `thread@graded-entries-and-id-lookup@goals-listed-not-reshown` | the same head, for the assembly's form of a goal; `design@agent-skills@search-relevance-grade` for the reading |
| `thread@graded-entries-and-id-lookup@id-lookup-command` | a new head of the core, its slug the thread's, by entry test 1: the command line, its scope, its exit codes, the phase gate on a reader, the nearest rival; `design@core@the-core-cli-is-a-library-module`'s list of commands gains `id` |
| `thread@graded-entries-and-id-lookup@may-bear-needs-a-named-doubt` and `thread@graded-entries-and-id-lookup@may-bear-read-on-touch` | `design@agent-skills@search-relevance-grade` rewritten: the named doubt, the reading on touch at the design grounding, the not-read list and its reader per D2's ruling; the owner's ruling of round 3 on narrowing the reading of issues and tripwires attributed to the owner |
| `thread@graded-entries-and-id-lookup@seeds-name-decisions` and `thread@graded-entries-and-id-lookup@one-search-per-question` | `design@agent-skills@record-searched-before-the-work` rewritten, since its seeds sentence turns false, with the design audit's seeds as D3 is ruled; the one-search rule there or in `design@agent-skills@one-record-searcher`, as the decision-recording skill judges, or in neither where a test excludes it |
| `thread@graded-entries-and-id-lookup@retrospective-asks-for-hand-work` | `design@agent-skills@premortem-as-watch-points` rewritten: the new question, and "a decision or a goal" |
| `thread@graded-entries-and-id-lookup@gather-reads-the-model` and every item of "Losing alternatives" | `register@core@rejected` or `register@agent-skills@rejected`, each as the tests of `skill@knowledge-architect-decision-recording@losing-alternatives` admit, cited by the head whose argument covers it |
| premortem causes 1, 4, 5 and 6, cause 6 half by half | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires`, each keep or drop listed under "Decided" with the condition that decides it |
| premortem causes 2 and 3 | `tripwire@agent-skills@record-search-misses` and `tripwire@agent-skills@goal-missed-though-graded` read again; their wording or horizon changed where the standard asks it |
| AC1 to AC4 | reported on at the landing; each spent, or, if it recurs, judged as a cause |
| D2, D3 | ruled before the merge; D2's ruling is what the rewritten `design@agent-skills@search-relevance-grade` and `design@agent-skills@plan-read-against-the-record` state, and D3's what the rewritten `design@agent-skills@record-searched-before-the-work` states |
| this spec | deleted in a commit of its own after the review before the merge, cited as `spec@plans@graded-entries-and-id-lookup` |

As harvested, each thread with the entry that records it:

- #graded-entries-assembled and #entry-text-snapshot-at-out →
  `design@core@worklist-writes-and-verifies-the-reports`;
- #goals-listed-not-reshown, #may-bear-needs-a-named-doubt and #may-bear-read-on-touch →
  `design@agent-skills@search-relevance-grade`;
- #seeds-name-decisions and #one-search-per-question → `design@agent-skills@seeds-name-decisions`;
- #id-lookup-command → `design@core@id-lookup-command`;
- #retrospective-asks-for-hand-work → `design@agent-skills@premortem-as-watch-points`;
- #gather-reads-the-model → `rejected@core@assembly-reads-the-model`.

## Later consequences

- `issue@core@a-searcher-fills-its-report-by-a-script-of-its-own` stays open: an unmet instance of
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` this discussion found, not built here.
- `issue@core@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history` and
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries` are instances of the
  same goal, not built here. The first is the sibling of `klarch id` for plan numbers; the second
  asks for the inbound sites of any set of entries, which the assembly gives only for the graded
  ones.
- The seeds of the intake still admit what the received files cite, goals included; the design
  audit's seeds are D3's.
