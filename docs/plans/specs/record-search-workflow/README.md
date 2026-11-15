# A saved workflow runs the record search, and a passing `worklist --verify` writes the closing sections and orders each verdict file by the act it asks

## Status and audience

This spec is the plan of the work that moves the record search's dispatch out of the record
searcher's description into a saved workflow the installed text ships, and that widens what a
passing `worklist --verify` writes so that the session reads what the workflow returns. It is
written for a session that did not witness the design discussion that produced it, and it leaves
the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. Q1, Q2 and D2, under "Defaults awaiting the owner", are ruled.
- Its line numbers name the tree at the commit "[intake] The forty-seven retrospective analyses
  leave, every outcome carried out", on which it was written.
- It was assembled from the transcript of one session. The discussion begins at the owner's message
  that opens "There's an issue about the record search being launched manually", and ends at the
  owner's message that opens "Q2 agreed, D2 too". It holds five owner messages, the rounds:
  - **round 1**, opening "There's an issue about the record search being launched manually";
  - **round 2**, opening "I had plans to design, but not specific plans";
  - **round 3**, opening "This new shape looks right";
  - **round 4**, opening "q1 no. The design looks right, proceed";
  - **round 5**, opening "Q2 agreed, D2 too", the rulings on the defaults the plan's review put to
    the owner.

  "The reply to round n" is the session's messages after the owner's message of round n. The reply
  to round 3 holds the draft of this spec, the code-split scout's report, the record search run
  through a draft of the workflow, and the experiment on a cross-check stage; the reply to round 4
  holds the premortem.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec. The work goes on the
branch that holds this spec, which merges once its work and its harvest have landed.

## Names

- **the search**: the record search, one run of `klarch worklist --out`, the record searchers it
  dispatches and `klarch worklist --verify`, at one of its occasions: the grounding of a design
  discussion, the grounding of a retrospective intake, the design audit of a milestone slice or of a
  spec.
- **the searcher**: `agent@knowledge-architect-record-searcher`; **its description**, the
  `description` field of `path@agent-skills@content/agents/record-searcher.md`, which carries the
  dispatch today, per `design@agent-skills@one-record-searcher`.
- **the workflow**: the saved workflow this work ships, `knowledge-architect-record-search`, under
  New names.
- **an act verdict**: a verdict other than `unrelated` whose verdict is not `constrains` or
  `touches`, or whose action is not `respect` or `none`, or whose owner-question is not empty. Every
  other verdict but `unrelated` is **a light verdict**. The total mapping is under Mapping tables.
- **the closing sections**: a report's `## Decided` and `## Met outside the task`, the constants
  `DECIDED` and `MET_OUTSIDE` of `path@query@src/worklist.rs`, lines 57 and 58.
- **the notices**: every line `worklist --out` prints on stdout, by `outln!` in `write`,
  `path@query@src/worklist.rs`, lines 396 to 445: the words of the work and the bound left, printed
  only where the run has a `--work`; the seeds `--seeds-from` took; under `--since`, the count of
  entries changed since the commit; one line per agent, its words and its items; a notice per group
  above the bound (`notice`, line 455); and the count of agents.
- **the flag builder**: the pure function of the workflow that turns its `args` into the whole
  command line of `worklist --out`, `worklistCommand` under New names.
- **low effort**: the option `effort: 'low'` of the harness's `agent()` call, which the Workflow
  tool's authoring reference names among the options of `agent()`; no shipped workflow uses it yet.
- **the authoring reference**: the Workflow tool's authoring reference, which the harness delivers
  to a session that writes a saved workflow's script; it says a script has "No filesystem or
  Node.js API access", so a script reads no file and runs no command.
- **the writer and the judge of `worklist.rs`**: in `path@query@src/worklist.rs`, the writer is
  `write`, line 211, and the functions after it up to `longest_backtick_run`, line 892, which
  `--out` runs; the judge runs from `Listed`, line 903, to `judge`, line 1667, the end of the code
  before the tests at line 1763, which `--check` and `--verify` run, `verify`, `judge_reports`,
  `judge_report`, `remove_outputs` and `outputs` among them.
- **case A and case B**: the two cases the reply to round 2 compared for
  #verdicts-verified-in-the-workflow: case A, the session reads every verdict other than
  `unrelated` with its quotation and verifies it itself, as today; case B, a verifier per subject
  inside the workflow tries to refute each verdict, and the session acts on the survivors without
  checking them again.
- **the parse check**: `path@agent-skills@src/workflow_parse.rs`, which wraps each installed saved
  workflow in `async function body() {…}` and hands it to `node --check`; it skips without `node`
  unless the `KNOWLEDGE_ARCHITECT_CI` variable that `cargo x gates --ci` sets requires it (`CI_VAR`,
  `required`, `checked`).
- **the experiment**: the cross-check stage the draft of the workflow ran in the reply to round 3,
  one agent per subject trying to refute the searchers' verdicts, real use per
  `design@agent-skills@synthetic-evidence-not-built`.
- **Q1, Q2, D2**: the questions and the default put to the owner, under "Defaults awaiting the
  owner".

## What the work is

What exists today at each site the work touches:

- **The dispatch** is the searcher's description: the session runs `klarch worklist --out <dir>`
  with `--work`, `--subject` and `--seed`, the flags the description names, and at an audit
  `--seeds-from` and `--since`, which point 2 of `skill@knowledge-architect-planning@working-a-slice`
  names (lines 573 to 576 of the source); sends one
  searcher per `agent-<n>` directory, as many at once as the harness admits; writes into its
  `report.md` each report an agent returned as text because the harness refused its write; runs
  `klarch worklist --verify <dir>`; sends back each agent whose report fails, naming the findings;
  then reads the verdict files, verifies and acts on each verdict, and carries each report's closing
  sections into its own. The skills that dispatch the search point to it:
  `path@agent-skills@content/skills/design/SKILL.md` (loop step 1, line 533),
  `path@agent-skills@content/skills/planning/SKILL.md` (point 2 of
  `skill@knowledge-architect-planning@working-a-slice`, line 571) and
  `path@agent-skills@content/skills/retrospective-intake/SKILL.md` (point 4, line 66). The
  retrospective and review skills name the search's outputs, and dispatch none.
- **The saved workflows** the installed text ships are the three audit axes',
  `path@agent-skills@content/workflows/`, per `design@agent-skills@audit-axis-runs-a-workflow`.
  `path@agent-skills@build.rs` installs any `workflows/<name>.js` as the installer's prefix and the
  file's stem, lines 309 to 311, and, at lines 124 to 128, calls `check_workflow_name` of
  `path@agent-skills@src/render.rs`, line 308, which asserts that the `meta` name is that stem, the
  prefix included, so `knowledge-architect-record-search` here, and requires the literal
  `export const meta = {` (`WORKFLOW_META`, line 296) and one top-level `name`, per
  `design@agent-skills@saved-workflow-name-is-its-file-stem`. A script reads no file and takes its
  inputs in `args`, per the same audit head, and runs no command, since it has "No filesystem or
  Node.js API access", per the authoring reference; an audit workflow's preflight
  agent runs the checker's commands, as `{{command}} issues` at line 280 of
  `path@agent-skills@content/workflows/design-record-audit.js`.
- **What a passing `--verify` writes** is `outputs`, `path@query@src/worklist.rs`, line 1401, per
  `design@query@occasion-shaped-output`: per subject `verdicts-<subject>.md`, every verdict but
  `unrelated` whose `against` names it, in the order of `agents.txt`, each as its `===` line, the
  agent's verdict line (`Verdict::line`, line 1584, every field including the quotation and the
  reason) and the entry's own text and inbound list, fenced; outside an audit,
  `generated-lines.md`; at an audit, `gaps.md` and `fit-lines.md` in its place. One `block` per
  verdict, built at line 1430, goes both into `gaps.md`, for every verdict but `constrains`, and
  into each verdict file its `against` names. A verdict `against: -`, admitted on `constrains` with
  the action `none`, has a generated line and is in no verdict file. `remove_outputs`,
  line 1259, removes the outputs of a previous run first; `verify`, line 953, prints one count line
  per output. `judge_report`, line 1171, refuses a report missing either closing section, and ends
  the verdict lines at the first line that, its trailing spaces trimmed, is either section's
  heading (line 1184);
  `judge_reports`, line 1121, reads each `report.md` and keeps no text of it, iterating `agents`, a
  `BTreeMap` that sorts `agent-10` before `agent-2`, where only `order` keeps the order of
  `agents.txt`; `outputs` takes no report text, and no file holds the closing sections' content.
- **A refusal of `--out`** is printed on stderr as `error: <e>`, with exit 2, by
  `path@cli@src/main.rs` line 115; clap refuses some values while it parses, such as a subject that
  is no id. Stdout holds only the notices.
- **The parse check** runs no part of a script and judges none of its logic: its module doc, lines
  8 to 10. The xtask test `the_parse_test_reads_the_library_s_ci_variable`,
  `path@xtask@src/gates.rs` line 100, reads the module by `include_str!` to check the spelling of
  `CI_VAR`.
- **The texts that describe the `--verify` outputs**: the worklist section of `path@cli@README.md`,
  lines 383 to 413, from the files the run removes to "A run that fails or stops leaves none of
  these files"; the help of `--verify` in `path@cli@src/cli/mod.rs`, lines 119 to 131; in
  `path@query@src/worklist.rs`, the doc comments of `WorklistMode::Verify` (lines 114 to 119), of
  the output constants (lines 38 to 50), of `worklist` (lines 149 to 153), of `verify` (lines 945
  to 952), of `remove_outputs` (lines 1256 and 1257), of `Outputs` (lines 1366 to 1368) and of
  `outputs` (lines 1376 to 1400), and the error text of `write_all`, line 1501, "No verdict file and
  no generated line was written."
- **The tests of the outputs**: in `path@query@src/worklist.rs`,
  `the_outputs_hold_each_subject_s_verdicts_and_the_generated_lines` (line 2538) and
  `an_audit_s_outputs_hold_its_gaps_and_its_fit_lines` (line 2647); in
  `path@testing@tests/binary.rs`, where `path@query@CLAUDE.md` puts the tests of the commands'
  output, `worklist_verify_leaves_no_output_of_a_previous_run_when_it_fails` (line 7698),
  `worklist_verify_writes_from_the_snapshot_and_refuses_one_that_lacks_an_entry` (line 7854) and
  `worklist_verify_at_an_audit_writes_the_gaps_and_the_fit_lines` (line 8596), built on
  `worklist_project` (line 6518). Four of their assertions turn false under steps 1 and 2:
  - the first unit test asserts `headed(a) == ["d@x@one", "d@x@two"]`, line 2605, a `constrains,
    respect` verdict before a `conflicts`, which the act-first order reverses, and
    `(out.lines, out.files.len(), out.gaps) == (4, 4, None)`, line 2634, which a fifth file breaks;
  - the second asserts the exact file list `["verdicts-spec-audit.md", "gaps.md", "fit-lines.md"]`,
    line 2700;
  - `worklist_verify_writes_from_the_snapshot_and_refuses_one_that_lacks_an_entry` asserts that the
    verdict file of a `constrains, respect` verdict holds its entry's text, which a light line no
    longer holds;
  - `worklist_verify_leaves_no_output_of_a_previous_run_when_it_fails` asserts
    `outputs() == ["generated-lines.md", "verdicts-whole-work.md"]`, line 7724, through the filter
    on output names that step 1 widens.

What the two closed issues held, carried here:

- the closing condition of the search's workflow: a design argued under
  `skill@knowledge-architect-design`, then a saved workflow shipped by the installed text with what
  the design decides about testing its logic. The issue's words "on the owner's plans" were
  answered in round 2: the owner had plans to design, and no specific plans for how. Its counts, read
  in `git show a973e5cb:crates/agent-skills/docs/open-issues/a-saved-workflow-runs-the-record-search.md`:
  the five searches the intake of forty-seven files counted dispatched 24 to 27 agents each, by hand;
  a session of this repository ran the 36 searchers of a design audit as a dynamic workflow instead,
  each agent in its own worktree, the one run of this work's shape before the discussion's own;
- the closing condition of the closing sections: every report's closing sections put before the
  dispatching session, with a test of `--verify` over reports holding them. Its first instance: in a
  search of 2026-11-11, the harness cut 16 of 18 searchers' returns, the session read the verdict
  files and the generated lines and never read those 16 reports' closing sections, two items of
  which bore on the work and reached it only through the transcript reviewer. Its second instance,
  the intake of forty-seven files, 2026-11-14, read the eighteen reports as messages and missed
  none, reported by that session and not checked; its lead was the review skill's report file per
  reviewer, `skill@knowledge-architect-review@review-invariants`; and its criterion between shapes 1
  and 3, whether the lapse recurs, is moot, since in a workflow the searchers' returns never reach
  the session, `argument@record-search-workflow@arg-26`. The issue's three shapes were a file
  `--verify` writes (taken), the sections appended to an existing output (not argued by the
  discussion), and the reading by hand (lost, per #closing-sections-file).

Outside the work, each with what owns it:

- the review before a merge run by a workflow, `issue@agent-skills@a-saved-workflow-runs-the-review-before-a-merge`;
- how a searcher fills its report, `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`;
- a lighter pass for untouched Components, `issue@agent-skills@a-lighter-pass-for-untouched-components`;
- the count logic of the three audit workflows, `issue@agent-skills@no-test-runs-a-saved-workflow-s-logic`:
  this work tests the flag builder of its own workflow, by the second shape of that issue's leads,
  and none of the audits' counts;
- the reopening of `design@agent-skills@one-record-searcher` that the re-entry of
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it` names, on the owner's ruling
  of Q1, round 4: this work moves where the dispatch lives, and the decision on what a search reads
  stands;
- a split of `path@query@src/worklist.rs` or of `path@testing@tests/binary.rs`, per
  #worklist-module-split and #binary-tests-split.

## What is already decided

The decisions the design rests on and does not argue again:

- `goal@session-orchestrates-the-work`, `goal@tasks-a-tool-does-better-are-the-tool-s` and
  `goal@serves-one-kind-of-project`, under Criteria.
- `goal@generic-tools-and-procedures-are-shipped`: the installed text ships the workflow, not a
  `klarch-` workflow of this repository, because the owner took in the generic part of the 36-agent
  workflow of a design audit at the intake that the commit "[intake] The forty-seven retrospective
  analyses leave, every outcome carried out" closed, as a todo issue not designed then. Under the
  goal a part is taken in by the owner's choice, not because it is generic, so that take-in is a
  premise of the workflow's head.
- `design@agent-skills@audit-axis-runs-a-workflow`: the precedent. An occasion's steps live in a
  saved workflow of the installer's namespace; a session whose harness does not list the workflow
  dispatches its steps by hand, in the script's order, with its prompts, read from the script. Its
  title scopes it to the audit axes, and this work does not rewrite it.
- `design@agent-skills@saved-workflow-name-is-its-file-stem`.
- `design@agent-skills@at-least-one-independent-verification`: one independent verification of
  each claim acted on; a form check such as `--verify` is none.
- `design@agent-skills@verdicts-reach-the-conformance-review`: a verdict the session misjudges has a
  reader in the design-conformance reviewer, narrower than every verdict: on the full path it reads
  the verdict files only where its brief names their directory, which may be gone by the review, and
  otherwise runs `show` on each line whose verdict is not `constrains` or `unrelated`; on the
  in-change path and for bounded work, at the review before the merge, it runs `show` only on such
  lines of the commit message. A line graded `constrains` has a reader only while the verdict files
  exist and the brief names them.
- `design@agent-skills@design-audit-checks-fit`: an audit reads the notices of the search's own
  `worklist` run. The design keeps it by #worklist-notices-returned, so its text stays true.
- `design@agent-skills@record-split-found-by-a-command`: no agent is dispatched for the notices of
  `--out` on the record side of a split. The workflow adds no agent for the notices: its worklist
  agent runs `--out` for the step's own property, under #workflow-runs-the-worklist, and returns the
  lines it printed.
- `design@agent-skills@synthetic-evidence-not-built`: this discussion's own search ran through a
  draft of the workflow as real use, and the experiment with it.
- `design@agent-skills@behaviour-additions-need-real-use` and `goal@only-needed-instructions`:
  #search-reads-a-fixed-tree is argued as a defect provable by reading, under Decided design.
- `design@query@worklist-check-judges-one-report`: the check of a report, unchanged.
- `design@gates@ci-flag-for-ci-only-requirements`: the logic test, like the parse check, skips
  without `node` and fails under `--ci`.

**Step 0 brings the six heads this list held at the commit that adds this spec to the rules on
design heads**, per `skill@knowledge-architect-planning@cutting-steps-and-slices`, on the verdicts
of `agent@knowledge-architect-design-record-auditor`, run as a drafter on them at that commit, with
one draft per head:

| head | verdict | step 0 |
| --- | --- | --- |
| `design@agent-skills@one-record-searcher` | fails one decision per head, and the title test | retitled to the rule of one agent for every kind; split out `failing-report-goes-back-to-its-searcher` and `search-dispatch-in-the-searcher-s-description` |
| `design@agent-skills@session-acts-on-the-verdicts` | fails one decision per head: the list "Where the occasion's record goes" is a second decision | split out `search-record-kept-per-occasion`; `design@agent-skills@plan-commits-the-generated-lines` re-pointed to it |
| `design@query@occasion-shaped-output` | fails one decision per head, the title test, and "not implementation" | split out `verify-reads-the-snapshot-and-no-model`; retitled; the clause on a failed removal moved to a comment at `remove_outputs` |
| `design@query@worklist-writes-and-verifies-the-reports` | fails the title test, and one decision per head | retitled; split out `skeleton-names-the-admitted-verdicts` |
| `design@query@worklist-bound-is-40000-words` | conforms; which Component owns it reads two ways, query or cli | none: it stays in query, as the parameter of `design@query@clusters-packed-under-a-word-bound`, whose packing decides a searcher's load, the bound's argument; cli only spells the default |
| `design@agent-skills@a-scratch-directory-per-subagent` | conforms | none |

The heads this list gained at the plan's review, `design@agent-skills@report-written-through-the-shell`
and `design@agent-skills@work-divided-by-judgement`, were not in that run: the session that starts
the work dispatches the auditor on them as a drafter before step 0, and a head it finds
non-conformant joins step 0. The rest of this section names each head as step 0 leaves it.

The decisions the work rewrites, each with every text the rewrite repairs, the repair made by step
4 for the installed text and by the harvest for the design homes:

- `search-dispatch-in-the-searcher-s-description`, the head step 0 splits out of
  `design@agent-skills@one-record-searcher`: its statement that the searcher's description carries
  the dispatch, "since a dispatcher reads an agent's description and never its body", turns false:
  the workflow carries the dispatch, and the description names the workflow. The harvest rewrites
  it, its title and slug brought to the workflow. `design@agent-skills@one-record-searcher`,
  retitled, and `failing-report-goes-back-to-its-searcher` stand: one agent per directory, the check
  by `--verify`, a report sent back and never repaired by the dispatcher. Texts: the searcher's
  description; the texts that cite `design@agent-skills@one-record-searcher` today, read by step 0
  for which statement each cites and re-pointed there: the `%%` comment of
  `path@agent-skills@content/skills/retrospective-intake/SKILL.md` line 71,
  `path@agent-skills@docs/design/audit-method.md` line 169, and the issues
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it`,
  `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment` and
  `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`, each
  read by the harvest for a statement about where the dispatch lives.
- `design@agent-skills@session-acts-on-the-verdicts`: how the verdicts reach the session, per
  #verdicts-split-by-action, and where it reads the closing sections, per #closing-sections-file;
  its sentence "The searcher's description states these acts" stays, the acts staying in the
  description. Texts: the searcher's description; the design skill's loop step 1; the planning
  skill's point 2 of `skill@knowledge-architect-planning@working-a-slice`; the intake skill's point
  4; the design homes that cite it, `path@agent-skills@docs/design/dispatching.md` line 115,
  `path@agent-skills@docs/design/record-search.md` lines 16, 88 and 116, and line 200, which step 0
  re-points to `search-record-kept-per-occasion`, `path@agent-skills@docs/design/reviews.md` line
  77; and `issue@agent-skills@a-lighter-pass-for-untouched-components`, line 73, read by the harvest.
- `design@query@occasion-shaped-output`, as step 0 retitles it: its sentence that each verdict file
  holds every verdict "with its agent's verdict line and its entry's text and inbound list" turns
  false for a light verdict, and `closing-sections.md` is a new output its argument does not cover,
  argued under Decided design. The argument against the sentence: this search's verdict files
  reached about 1.5 MB, more than one session holds, as
  `rejected@agent-skills@relevance-grades-read-by-the-session` records for whole readings; the
  session runs `show` on a doubt, never once per entry, so the head's defeated rival, the session
  running `show` once per entry it acts on, does not come back. Its argument "a session that hands
  one subject of the work to an agent hands it that subject's file" stands, and keeps the files per
  subject. Texts: the doc comments and texts listed under "What the work is", repaired by steps 1
  and 2; the description of `agent@knowledge-architect-design-conformance-reviewer`,
  `path@agent-skills@content/agents/design-conformance-reviewer.md` lines 55 to 57, "each verdict
  with its entry's text", repaired by step 4.

The heads the harvest touches without reversing or rewriting them, per
`skill@knowledge-architect-planning@cutting-steps-and-slices`:

- `design@query@worklist-writes-and-verifies-the-reports`, as step 0 retitles it: it names the
  outputs only through `design@query@occasion-shaped-output`; the harvest edits it only where its
  text names an output.
- `design@query@worklist-bound-is-40000-words`: its re-take sentence names a searcher's transcript
  at `subagents/agent-<id>.jsonl`; a workflow's agents sit at `subagents/workflows/wf_<id>/`, as
  measured in the reply to round 3. The harvest rewords the path to cover both.
- `design@agent-skills@a-scratch-directory-per-subagent`: its site, "the record searcher's
  description, which carries its dispatch rule, makes each agent directory that `klarch worklist`
  writes its searcher's scratch directory", becomes the workflow, which gives each searcher its
  agent directory. Texts that cite it, which stay true: the `%%` comment of the primer's source,
  `path@agent-skills@content/PRIMER.md` line 314, at `primer@dispatching-work`, and
  `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand` line 55.
- `design@agent-skills@report-written-through-the-shell`: it gains a member, the low-effort agent of
  the workflow that writes a report returned as text into its `report.md`, by a shell redirect.
  The head's argument, that the harness refuses a subagent's `Write` over an existing `report.md`,
  covers it.
- `design@agent-skills@work-divided-by-judgement`: a fixed dispatch of the installed text stands
  only on a property of the step; "An output verified by a session other than its author is no such
  property". It rules out #verdicts-verified-in-the-workflow. Its measured fact, that a
  general-purpose subagent holds no `Workflow` tool, grounds #fallback-dispatch-line. Its list of
  properties "is open"; the workflow's fixed worklist and verify agents add a member, a step of a
  saved workflow that runs a command, which its script cannot run, argued under Decided design.

The discussion's record search, run through a draft of the workflow over the draft of this spec
with each thread as a subject, gave 233 verdicts other than `unrelated`: 221 `constrains` or
`touches` with `respect` or `none`, 6 `conflicts, check-reversal`, 4 `worsens, respect`, 2
`closes, close-with-the-work`. The experiment refuted 16 of the 430 verdicts it checked, counted as
the lines marked `refuted` in its twelve files, one per subject; three of its refutations changed
the design. Each verdict whose outcome changed something, or that the session flagged, with that
outcome:

| entry | verdict | outcome |
| --- | --- | --- |
| `design@agent-skills@one-record-searcher` | conflicts | rewritten in `search-dispatch-in-the-searcher-s-description`, above |
| `design@agent-skills@session-acts-on-the-verdicts` | conflicts | rewritten, above |
| `design@query@worklist-writes-and-verifies-the-reports` | conflicts | touched only where its text names an output, above |
| `design@query@occasion-shaped-output` | conflicts | rewritten; its argument keeps the files per subject, above |
| `design@query@worklist-bound-is-40000-words` | conflicts | its re-take path reworded, above |
| `design@agent-skills@a-scratch-directory-per-subagent` | conflicts | its site rewritten, above |
| `design@agent-skills@work-divided-by-judgement` | refuted to conflicts by the experiment | rules out #verdicts-verified-in-the-workflow; gains a property, above |
| `design@agent-skills@design-audit-checks-fit` | refuted to conflicts by the experiment | the new thread #worklist-notices-returned |
| `goal@serves-one-kind-of-project`, `issue@agent-skills@heads-argued-from-another-harness` | refuted by the experiment | the new thread #fallback-dispatch-line, argued on a measured fact of `claude`, not on another harness |
| `issue@agent-skills@a-record-search-misses-what-is-added-after-it` | refuted to trigger-met by the experiment, with an owner-question | Q1, ruled no in round 4 |
| `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` | worsens | respected: the by-hand line covers a refused run too |
| `issue@check@a-declared-command-with-a-quote-breaks-the-installed-workflows` | worsens | respected: the script uses `{{command}}` only as the audit workflows do |
| `issue@testing@a-test-run-leaves-directories-in-the-temporary-directory` | worsens | respected: each test step 1 or 2 adds removes the output directory it builds |
| `issue@model@references-are-read-in-markdown-and-rust-only` | worsens | respected: the script cites no entry, and the description keeps the pointers that need checking |
| `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project` | touches | respected: a comment for this repository's maintainers is a `%%` line |
| `goal@agents-work-without-drift` | constrains | answered under Decided design, in the split of the verdict files |
| `a-saved-workflow-runs-the-record-search`, `no-command-gathers-the-closing-sections-of-the-searchers-reports` | closes | closed by the commit that adds this spec, which schedules their work |
| `rejected@agent-skills@conformance-reader-at-the-checkpoint` | constrains, flagged in the reply to round 3 | respected: the cross-check is no reader at a checkpoint, and it is ruled out anyway |
| `issue@agent-skills@a-record-search-misses-what-is-added-after-it` | touches, flagged in the reply to round 3 | Q1, above |
| `issue@knowledge-architect@saved-workflows-under-the-harness-on-one-branch` | touches, none, flagged in the reply to round 3 | none: its member on the permission rule is answered in the row of `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` |
| `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session` | touches, flagged in the reply to round 3 | respected: the split keeps one verdict file per subject |

The other refutations of the experiment, each of which narrows an `against` list or an action and
changes nothing in the work:

| entry | subject | refutation | reading |
| --- | --- | --- | --- |
| `design@knowledge-architect@query-crate` | #binary-tests-split | `constrains, none`, or the subject dropped | the head puts the record search's own tests beside it, and no split of `binary.rs` is made |
| `design@agent-skills@discussion-record-apart` | #search-workflow-logic-tested | `unrelated` | the flag builder maps `work` as given and chooses no file |
| `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation` | #worklist-module-split | no bearing | its items are in `write`, which neither the default nor the rival moves |
| `issue@check@a-quotation-beside-a-reference-is-not-checked-against-its-entry` | #worklist-module-split | `respect`, not `none` | only the rival split would owe an edit of the issue's text, and the work does not split |
| `design@query@worklist-writes-and-verifies-the-reports` | #worklist-module-split | `constrains, respect` | the head states the commands' behaviour, which no move of the judge changes |
| `design@query@worklist-writes-and-verifies-the-reports` | #closing-sections-file | `constrains, respect` | the widening lands in `design@query@occasion-shaped-output`, as above |
| `design@agent-skills@session-acts-on-the-verdicts` | #closing-sections-file | `constrains, respect` | the act of carrying the sections is unchanged; the head stays rewritten for its other subjects |
| `design@agent-skills@a-scratch-directory-per-subagent` | #verdicts-verified-in-the-workflow | `constrains, respect` | #dispatch-text-home makes its site false, and the stage is ruled out |
| `design@knowledge-architect@xtask-crate` | #workflow-logic-test-home | `none`, not `respect` | xtask reads the module by `include_str!`, which no shape of the test home changes |
| `issue@knowledge-architect@no-gate-builds-the-documentation` | #workflow-logic-test-home | `unrelated` | `workflow_parse.rs` is a private test module, which no rendered doc links |

The other verdicts are in [the discussion's search](search/discussion.txt), one generated line per
entry, without quotation or reason; their quotations are in the search's verdict files, which stay
in the scratch space, and the work keeps within the sentence each quotes. The draft's worklist
agent returned the lines `--out` printed, `notices` in the run's result, and none was a notice on a
cluster; the draft of this spec named only the directories as that step's return,
`argument@record-search-workflow@arg-24`.

## Criteria

### The main session keeps its context for the design, the decisions and the exchange with the owner `##session-keeps-its-context`

Binding, from `goal@session-orchestrates-the-work`, derived in the reply to round 1. Satisfied by
#workflow-runs-the-worklist, #report-loop-in-the-workflow and #closing-sections-file, which take
the session's dispatches and checks; #verdicts-split-by-action keeps the session's reading to the
act verdicts whole and the light verdicts as lines.

### A task a tool does better is the tool's `##tool-does-the-tool-s-task`

Binding, from `goal@tasks-a-tool-does-better-are-the-tool-s`, derived in the reply to round 1.
Satisfied by #closing-sections-file and #report-loop-in-the-workflow: a command and a script, not a
reader, gather and loop.

### The dispatch has one home `##dispatch-has-one-home`

Binding as a presumption, from `design@agent-skills@audit-axis-runs-a-workflow`, derived in the
reply to round 1. Satisfied by #dispatch-text-home and #fallback-dispatch-line: the script holds
the steps, and the description's by-hand line points to it and restates none.

### The workflow's logic is tested, or why not is argued `##script-logic-tested`

Binding, from the closing condition of the closed issue on the search's workflow, derived in the
reply to round 1. Satisfied by #search-workflow-logic-tested and #workflow-logic-test-home.

## New names, in one place

```text
crates/agent-skills/content/workflows/record-search.js   new: the saved workflow
  meta name   knowledge-architect-record-search            installed with the installer's prefix and the stem
  args        scratch      the directory `--out` writes into, new, outside the project
              work         [paths]  each a `--work`
              subjects?    [ids]    each a `--subject`; none gives the one subject `whole-work`
              seeds?       [refs]   each a `--seed`
              since?       a commit, `--since`
              seedsFrom?   a plan document's README.md, `--seeds-from`
              bound?       `--bound`
              maxRounds?   the `--verify` runs of the report loop, default 3 (Q2)
  phases      worklist, search, verify
  worklistCommand(args)    the flag builder, a pure function: the whole command line of `--out`, one string
  worklist agent's schema  { agents: [names of agent-<n> directories], notices: text } | { refusal: text }
  searcher's schema        { written: boolean, reportText?: text }
  verify agent's schema    { passed: boolean, output: text, failing: [{ agent: name, findings: text }] }
  result      { refusal }
            | { failing, verifyOutput, notices }
            | { notices, verdictFiles, generatedLines | gaps, fitLines, closingSections, statement }
crates/agent-skills/src/workflow_parse.rs                 existing: gains the logic test of worklistCommand
crates/query/src/worklist.rs                              existing
  CLOSING_SECTIONS_FILE = "closing-sections.md"           new: written by a passing `--verify`
  Judged                                                  existing: gains each report's closing text
  outputs, Outputs, remove_outputs, verify, judge_reports existing: changed
```

The fields of the result:

- `refusal`: the text `--out` printed on stderr, verbatim, with its exit code;
- `failing`: each agent whose report still fails after `maxRounds` runs of `--verify`, with its
  findings; `verifyOutput`: the last run's output; `notices`: as below, returned in this case too;
- `notices`: every line `--out` printed on stdout, the notices under Names, as text;
- `verdictFiles`: the path of each `verdicts-<subject>.md`, `<scratch>/verdicts-<subject>.md` for
  each subject of `args.subjects`, or `whole-work` where it gives none, computed by the script;
- `generatedLines`, or at an audit `gaps` and `fitLines`, and `closingSections`: the path of
  `generated-lines.md`, of `gaps.md` and `fit-lines.md`, and of `closing-sections.md` under
  `<scratch>`;
- `statement`: the sentence that the session edits nothing in the tree until the workflow returns,
  per "The session edits nothing while the workflow runs".

## Decided design

### The search runs one saved workflow, for every occasion

From #search-runs-a-saved-workflow. The installed text ships `knowledge-architect-record-search`,
`planned@agent-skills@content/workflows/record-search.js`, for every occasion of the search: the
occasions differ only by the flags their `args` give, and `--seeds-from` alone switches the outputs
to `gaps.md` and `fit-lines.md`. It ships in the installed text, and not as a `klarch-` workflow of
this repository, on the owner's take-in of its generic part, per
`goal@generic-tools-and-procedures-are-shipped`, under "What is already decided". The nearest rival,
a rewording of the dispatch sentence so that a session dispatches better by hand, leaves one hand
dispatch and one check per searcher, 22 to 36 per search: 22 in the discussion's own search; 24 to
27 in each of the five searches the intake of forty-seven files counted, which were dispatched by
hand; and 36 in the design audit whose searchers a session of this repository ran as a dynamic
workflow, the counts of the closed issue under "What the work is".

Illustration of the shape, not authority:

```js
phase('worklist')   // one agent, effort 'low', runs worklistCommand(args), the whole `--out` command
                    // returns { agents, notices } or { refusal }
phase('search')     // one record searcher per agent directory, in parallel
                    // returns { written, reportText? }
phase('verify')     // one agent at effort 'low' per text report writes it into its report.md by a shell redirect;
                    // one agent runs `--verify` and returns { passed, output, failing: [{ agent, findings }] };
                    // each failing searcher sent again with its findings; at most maxRounds runs of `--verify`
return { notices, verdictFiles, generatedLines /* or gaps, fitLines */, closingSections, statement }
```

### An agent of the workflow runs `worklist --out`, and the workflow returns its notices

From #workflow-runs-the-worklist and #worklist-notices-returned.

- The workflow's first step is one agent at low effort that runs the command line
  `worklistCommand(args)` builds, `{{command}} worklist --out <scratch>` and its flags, since a
  script reads no file and runs no command, per the authoring reference.
- It returns, by its schema, the `agent-<n>` directories `--out` wrote and every line `--out` printed
  on stdout, the notices; or, where `--out` refuses, which it does on stderr with exit 2, its stderr
  verbatim, which the workflow returns as `{ refusal }`, and the session repairs its `args` and
  starts the workflow again.
- The workflow returns the notices with its other outputs, so a design audit reads them as
  `design@agent-skills@design-audit-checks-fit` asks: `--out` prints them on stdout only, by
  `outln!` in `write`, and writes them into no file of the directory.
- **The worklist agent, and the verify agent below, are fixed dispatches on a property of the step
  that holds whatever the task's size: each runs a command, which a workflow's script cannot run.**
  `design@agent-skills@work-divided-by-judgement` states that its list of properties is open; this
  is a new member of it, which the harvest adds. It is no size claim: a search of one agent needs
  the command run all the same, and the script has no other way to run it. `--verify` judges the
  worklist step's result too: a directory the step dropped leaves its entries with no verdict,
  which `--verify` reports. `design@agent-skills@record-split-found-by-a-command` stands: no agent
  is added for the notices, which the worklist agent returns from the run it makes for the
  command's sake.

### The workflow writes the text reports, runs `--verify`, and sends a failing report back

From #report-loop-in-the-workflow.

- Each searcher's brief gives it its agent directory, which is its scratch directory, and the path
  of `<scratch>/work.txt`; it returns whether it wrote its report, or the report as text where the
  harness refused every write of it.
- **Each report returned as text is written into its `report.md` by a low-effort agent of its
  own, through a shell redirect**, before `--verify` runs, so that no one agent's prompt holds every
  text report: a clause of the premortem's cause 1. The redirect is the form of
  `design@agent-skills@report-written-through-the-shell`, since the harness refuses a subagent's
  `Write` over an existing `report.md`; the agent is a new member of that head, which the harvest
  adds.
- One agent then runs `{{command}} worklist --verify <scratch>` and returns whether it passed, its
  output, and each failing agent with its findings. The script sends each failing agent's searcher
  again, naming the findings, and runs `--verify` again. `maxRounds` counts the runs of `--verify`,
  default 3 (Q2): at most three runs, so at most two sendings back. Past them the workflow returns
  `{ failing, verifyOutput, notices }`, and caps nothing silently.
- **On `{ failing }`**, the session reads the findings, then either sends each failing agent's
  searcher again with them, by hand, and runs `--verify` itself, or reports the search as failing
  where a searcher repeats a finding; it repairs no report.
- **No step of the workflow repairs a report**: a report that fails goes back to its searcher, per
  `failing-report-goes-back-to-its-searcher`, the head step 0 splits out of
  `design@agent-skills@one-record-searcher`.
- No cross-check stage: per #verdicts-verified-in-the-workflow. The session may still hand a
  subject's verdict file to an agent it dispatches, at its judgement.

### A passing `--verify` writes the closing sections of every report into one file

From #closing-sections-file. A passing `--verify` writes `closing-sections.md` into the directory,
at every occasion, an audit included:

- its first line is `# The closing sections of the searchers' reports`;
- then, for each agent of `agents.txt`, in its order, each agent at its first line there, a line
  `=== agent-<n>`, then the agent's report text from its first closing section, the
  line at which `judge_report` ends the verdict lines, to its end, fenced as a verdict file fences
  an entry's text by `fenced`, so that the report's headings read as no section of the file;
- an agent whose sections are empty is listed with them empty, so a session tells a report with
  nothing to carry from one missing.

`judge_reports` carries each report's closing text in `Judged`, and `outputs` takes it, since it
reads no report today. `remove_outputs` removes the file with the other outputs before a run judges
anything, and `verify` prints its line beside the other counts, `<n> reports in <path>`, `n` the
agents of `agents.txt`. The workflow returns its path, and the session carries each item into its
own sections.

The file is a new member of `design@query@occasion-shaped-output`, which its argument does not
cover today, since the head's outputs are the files a session hands to an agent or copies into its
record: this file is read by the dispatching session itself. Its argument is the head's own rival,
reworded: the reading a session would otherwise do by hand, or by a script of its own, is written
by the command, per `goal@tasks-a-tool-does-better-are-the-tool-s`.

The nearest rivals: an agent of the workflow gathering the sections, a copy by a reader of the kind
that missed sixteen reports; and the reading of each `report.md` by hand, which in a workflow is one
file read per searcher, 22 in this search, since the searchers' returns never reach the session.

### Each verdict file holds its act verdicts whole first, then its light verdicts as lines with their quotation and reason

From #verdicts-split-by-action, as revised in the reply to round 3 and approved in round 4.

- `--verify` keeps one `verdicts-<subject>.md` per subject.
- Inside each file, after its title line, the act part opens with the line `## Act verdicts`, and
  every act verdict follows, in the order of `agents.txt`, each as today: its `===` line, its
  agent's verdict line, and its entry's own text and inbound list, fenced.
- The light part opens with the line `## Light verdicts`, and every light verdict follows, in the
  order of `agents.txt`: its `===` line, with the changed-since mark where it applies, and its
  agent's verdict line, which carries the quotation **and the reason**, without the entry's text or
  its inbound list. Each part's line is written even where the part is empty.
- `verify` prints, per subject, `<n> verdicts against <subject>, <a> of them act verdicts, in
  <path>`.
- An act verdict is defined by the mapping table below. A verdict with a non-empty owner-question
  is an act verdict, since putting it to the owner is an act of the session, per
  `design@agent-skills@session-acts-on-the-verdicts`.
- **`generated-lines.md`, `gaps.md` and `fit-lines.md` do not change.** `gaps.md` keeps every
  verdict it holds today with its entry's text and inbound list, whatever its part, a `touches`
  verdict included: only the verdict files split. So `outputs` builds two forms of each verdict, the
  whole `block` it builds today, which `gaps.md` and an act verdict take, and the light line,
  which a light verdict takes in a verdict file.
- The session reads the act verdicts with their entries whole and verifies each as it acts, and
  checks each light line against the shapes it holds, running `show` where a line looks like a
  downgrade.

The argument: a search returns two populations, 1 to 11 verdicts that need an act and 19 to 177
that ask the work to keep within a sentence, over four past searches, counted as the verdict lines
of their commit messages, ``^- `<ref>`: <verdict>, <action>.$``, over `git log a973e5cb -n 400`:
"[plans] The spec of the dispatch judgement in the primer, and of one check by another session as
sufficient verification" (177 and 11), "[check] A slug written with a hash sign in backticks,
`#<id>` or `<word>#<id>`, is read by no lint" (115 and 10, one of them `worsens`), "[agent-skills]
The retrospective asks for the tools a session set up, the record search's misses and a merged
branch's review cost, and reports a needless instruction" (125 and 6), and "[agent-skills] The goal
reviewer grades a two-sense term `blocks` only where the report names the act the other sense would
produce" (19 and 1). The session must read an act verdict's entry whole to act on it, and its
context is decisive there. The first shape, light lines without the reason, let through three
load-bearing verdicts the searchers' reasons carried: the defect was the format. With their
reasons, the light lines of the discussion's search are 95 KB for the 178 entries the verdict files
hold graded `constrains` or `touches` with `respect` or `none`, against 1.5 MB of verdict files, one
entry appearing under each of its subjects; the other 43 of the 221 such verdicts are `against: -`
and in no verdict file. Both figures were taken in the reply to round 3 over the search's verdict
files, which stay in the scratch space.

**`goal@agents-work-without-drift` ranks nothing here.** The search graded it `constrains, respect`
against this thread, its reason that a downgraded verdict passed as a compact line lets a rejected
design return unnoticed. The split removes from a light line only the entry's text and its inbound
list, which no session reads whole, since the verdict files reach 1.5 MB; it keeps the quotation and
the reason, which carried every miss of the experiment. Nothing is traded against drift, so the
goal's precedence is not met.

### The session edits nothing while the workflow runs

From #search-reads-a-fixed-tree. The workflow's `description` in its `meta`, and its result's
`statement`, state that the session edits nothing in the tree until the workflow returns. The
ground is a defect provable by reading, which `design@agent-skills@behaviour-additions-need-real-use`
admits without real use: the searcher's body says "You run in the session's tree, which nobody edits
while you search" (`path@agent-skills@content/agents/record-searcher.md`, lines 33 and 34), and no
text makes that true for a workflow running in the background, where the session can edit
meanwhile. Under `goal@only-needed-instructions`, the case is one judgement does not handle from the
text alone, since nothing tells the session the premise it must keep. The rival, a reading worktree
at a commit, fails on the uncommitted draft and the uncommitted entries a search often reads.

### The dispatch lives in the script, and the searcher's description keeps what the session needs

From #dispatch-text-home and #fallback-dispatch-line.

- The searcher's description keeps: what the agent does; the inputs the session chooses, the work
  files, the subjects and the seeds, and at an audit `since` and `seedsFrom`, as the dispatching
  skill names them; the workflow's name and its `args`; what each output is for and how the session
  acts on it, the acts of `design@agent-skills@session-acts-on-the-verdicts`, and the act on
  `{ failing }` above.
- It keeps one line for a session that cannot run the workflow: one running inside an agent, which
  holds no `Workflow` tool, per `design@agent-skills@work-divided-by-judgement`; one whose run the
  permission mode refuses, per `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule`;
  and one that holds no opt-in for the Workflow tool, per the premise below. That session
  dispatches the steps by hand, in the script's order, with the script's prompts, read from the
  script. **The line points to the script and restates no step of it**: a clause of the
  premortem's cause 6. Each case is a state of `claude`, not another harness, which
  `goal@serves-one-kind-of-project` excludes building for.
- The design, planning and retrospective-intake skills name the workflow where they dispatch the
  search. **The opt-in is a premise (D2)**: the Workflow tool's description counts as an opt-in "The
  user invoked a skill or slash command whose instructions tell you to call Workflow", and
  `design@agent-skills@audit-axis-runs-a-workflow` gives as its example a skill the owner invokes,
  while these skills are most often loaded by the session itself. A standing authorization in the
  user's or the project's instructions is an opt-in; a session that holds none takes the by-hand
  line.
- The script cites no entry, as every saved workflow, per `path@agent-skills@CLAUDE.md`; a comment
  for this repository is a `%%` line.

### The flag builder is a pure function, tested under `node` beside the parse check

From #search-workflow-logic-tested and #workflow-logic-test-home.

- `worklistCommand(args)` is a pure function of the script that returns the whole command line of
  `--out`, as one string: `{{command}} worklist --out <scratch>`, then each `work` as a `--work`,
  each `subjects` entry as a `--subject`, each `seeds` entry as a `--seed`, `since` as `--since`,
  `seedsFrom` as `--seeds-from` and `bound` as `--bound`, in that order, by the mapping table. Each
  value is single-quoted for the shell, an inner `'` written `'\''`, so a backticked seed, which
  `write` accepts by trimming its backticks (`path@query@src/worklist.rs` line 259), runs no
  command substitution. `subjects` and `seeds` are optional, and none of either adds no flag. It is
  the one piece of the script whose failure is silent: a dropped or misquoted seed weakens the
  search, and nothing reports it.
- Its test lives in `path@agent-skills@src/workflow_parse.rs`, the one module that runs `node` on a
  shipped workflow and owns the CI requirement on it (`checked`, `required`, `CI_VAR`). The test
  reads the source copy, `planned@agent-skills@content/workflows/record-search.js`, strips its
  `%%` lines as the build does, substitutes `{{command}}` with a command of its own, calls the
  function with fixed `args` and compares the string it returns with the expected one, each value
  quoted as above; it skips without `node` and fails under `cargo x gates --ci`, as the parse check
  does. How it reaches the function without running the script's body, which calls the harness's
  `agent` and `phase`, is the step's to build, within what `check_workflow_name` requires of the
  script: the literal `export const meta = {` and one top-level `name`.
- The module's doc, which says it "runs no part of the script and judges none of its logic", is
  rewritten to its two purposes, and `path@agent-skills@CLAUDE.md`, lines 9 to 16, which describes
  the module as parsing each installed workflow, names the new test. The module is not renamed: the
  discussion decided none, and the xtask test reads it by its path.

### No split of `worklist.rs` or of `binary.rs` in this work

From #worklist-module-split and #binary-tests-split. The work writes into the judge of
`path@query@src/worklist.rs` alone, as Names bounds it, and adds tests to
`path@testing@tests/binary.rs` where `path@query@CLAUDE.md` puts them.

## Mapping tables

The part of a verdict file each verdict takes, over every verdict and action `verdicts` and
`actions` of `path@query@src/worklist.rs` admit (lines 913 and 932):

| verdict | action | owner-question | part |
| --- | --- | --- | --- |
| `constrains`, `touches` | `respect`, `none` | empty | light |
| `constrains`, `touches` | `respect`, `none` | not empty | act |
| `worsens` | `respect`, `none` | any | act |
| `conflicts` | `check-reversal`, `goal-setting` | any | act |
| `brings-back` | `new-argument-needed` | any | act |
| `closes` | `close-with-the-work` | any | act |
| `depends-on`, `fires`, `trigger-met` | `rule-before-work` | any | act |
| `unrelated` | `none` | any | in no verdict file, as today |

A verdict `constrains` with the action `none` and `against: -` is light, and in no verdict file, as
today, since it names no subject; it keeps its generated line. In `gaps.md` every verdict but
`constrains` and `unrelated` stays whole, whatever its part.

The part of the command line `worklistCommand` returns for each argument of the workflow, in this
order, after `{{command}} worklist`, each value single-quoted, an inner `'` written `'\''`:

| `args` | part |
| --- | --- |
| `scratch` | `--out '<scratch>'` |
| each of `work` | `--work '<path>'` |
| each of `subjects` | `--subject '<id>'`; none adds nothing, and the one subject is `whole-work` |
| each of `seeds` | `--seed '<ref>'`; none adds nothing |
| `since` | `--since '<commit>'` |
| `seedsFrom` | `--seeds-from '<path>'` |
| `bound` | `--bound '<n>'` |
| `maxRounds` | none: the script's own |

## Losing alternatives

- **A rewording of the dispatch sentence**, the rival of #search-runs-a-saved-workflow: still one
  hand dispatch and one check per searcher, 22 to 36 per search.
- **The session running `--out` itself**, the rival of #workflow-runs-the-worklist: one more act
  for the session, which must know the flags to fill `args` anyway.
- **An agent of the workflow gathering the closing sections**, and **the reading by hand**, the
  rivals of #closing-sections-file: a copy of the kind that missed sixteen reports; one file read
  per searcher, 22 in this search, the returns never reaching the session.
- **A cross-check stage inside the workflow**, #verdicts-verified-in-the-workflow, ruled out on
  `design@agent-skills@work-divided-by-judgement`.
- **Light verdicts as compact lines without their reason**, the first shape of
  #verdicts-split-by-action: it let three load-bearing verdicts through, found by the experiment.
- **One file of the light verdicts across subjects**: lost to the argument of
  `design@query@occasion-shaped-output` that a subject handed to an agent is that subject's file.
- **A reading worktree at a commit**, the rival of #search-reads-a-fixed-tree: fails on an
  uncommitted draft.
- **A line for a harness without the workflow**, the first wording of #dispatch-text-home: it
  served another harness, which `goal@serves-one-kind-of-project` excludes; superseded by
  #fallback-dispatch-line, whose cases are all states of `claude`.
- **The parse check alone**, the rival of #search-workflow-logic-tested: it judges no logic.
- **A new module running `node` beside the parse check**, the rival of #workflow-logic-test-home:
  it shares the CI items or carries a copy of the CI requirement the xtask test does not read.
- **A split of the judge of `worklist.rs`** and **the worklist tests moved to a test file of their
  own**, the rivals of #worklist-module-split and #binary-tests-split.

## Readings

Empty: the work reads no external specification the project implements. The behaviour of Claude
Code's Workflow tool it relies on is recorded in `design@agent-skills@audit-axis-runs-a-workflow`
and `design@agent-skills@work-divided-by-judgement`, and read in the authoring reference for the
option `effort` of `agent()` and for a script's lack of filesystem access, and in the tool's
description for the opt-in, D2.

## Premortem

Run in the reply to round 4: assume the workflow shipped and failed. The conditions named are those
of the tripwire standard of `skill@knowledge-architect-decision-recording@ruled-tripwires`, which
judges each cause named for the harvest at the harvest, on the cause's wording then.

| # | cause | thread stressed | outcome |
| --- | --- | --- | --- |
| 1 | Many searchers return their reports as text. The draft passes all of them through one verify agent's prompt, about 20 KB each, so that prompt grows past what one agent handles well | #report-loop-in-the-workflow | becomes a clause of the design: each text report is written by a low-effort agent of its own, before `--verify` |
| 2 | The flag builder drops or misquotes a seed or a `--since`, and the search silently grades against the wrong seeds | #search-workflow-logic-tested | a claim of the step, with the `node` test and a recorded mutation |
| 3 | The session edits the record while the workflow runs. `--verify` judges against the snapshot and passes, so the verdicts are stale | #search-reads-a-fixed-tree | named as a cause for the harvest to judge as a tripwire; expected verdict: dropped on condition 2, the evidence living only in a transcript |
| 4 | A real `conflicts` graded `constrains, respect` reaches the session only as a light line, which it does not read whole | #verdicts-split-by-action | named as a cause for the harvest to judge as a tripwire; expected verdict: dropped on condition 1 where the design-conformance reviewer reads the verdict files, on the full path while its brief names their directory; at the review before a merge, and on the in-change and bounded paths, it runs `show` only on lines whose verdict is not `constrains`, so no reviewer reads such a line there, and the harvest judges whether the cause passes the four conditions |
| 5 | In the default permission mode the workflow's run is refused, and every search falls back to dispatch by hand | #search-runs-a-saved-workflow, #fallback-dispatch-line | named as a cause for the harvest to judge as a tripwire; expected verdict: dropped on condition 1, since the session sees the refusal; `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` holds the question |
| 6 | The by-hand line drifts from the script | #fallback-dispatch-line | turned into a clause of the design: the line points to the script and restates no step |

## Acceptance criteria

None: no acceptance criterion passes conditions 1, 3 and 4 of the standard of
`skill@knowledge-architect-planning@acceptance-criteria`. Causes 2 and 6 are what the built code
checks, and the others are judged at the harvest, under Premortem.

## Implementation sequence

0. **The touched and rewritten heads brought to the rules on design heads** (the agent-skills and
   query design homes). First, the auditor judges as a drafter the two heads not yet judged, under
   "What is already decided". Then applies the splits and retitles of its table, and of any head
   that run finds non-conformant, each as the table says, the auditor's drafts of the plan's review
   giving each new head's text where they are still at hand: the new heads
   `failing-report-goes-back-to-its-searcher`, `search-dispatch-in-the-searcher-s-description`,
   `search-record-kept-per-occasion`, `verify-reads-the-snapshot-and-no-model` and
   `skeleton-names-the-admitted-verdicts`, each slug checked free by `klarch id`; the retitles of
   `design@agent-skills@one-record-searcher`, `design@query@occasion-shaped-output` and
   `design@query@worklist-writes-and-verifies-the-reports`; the clause on a failed removal moved
   from `design@query@occasion-shaped-output` to a comment at `remove_outputs`; and the re-points
   the drafts name, `design@agent-skills@plan-commits-the-generated-lines` and
   the line of `register@query@design` that cites the snapshot, 231, among them, with each citer of
   `design@agent-skills@one-record-searcher` read for which statement it cites. The step writes no
   new argument and changes no word of the owner, per
   `skill@knowledge-architect-planning@cutting-steps-and-slices`; an edit that must write one goes
   to `skill@knowledge-architect-design`, and the step waits for it. **Each head a split or a
   retitle of the step creates is judged alone by the auditor briefed to check, before the step's
   commit**, its brief naming the text of each such head as the step writes it. Fails alone on
   `cargo klarch check` and the auditor's check. Claims: every reference to a moved statement
   resolves to the head that holds it, by `cargo klarch check`; the harvest's rewrites then land in
   the heads "What is already decided" names.
1. **`--verify` writes `closing-sections.md`** (`query`). Builds the constant, the closing text
   carried by `judge_reports` in `Judged`, its write in `outputs` in the order of `agents.txt`, its
   removal in `remove_outputs`, its count line in `verify`, and the texts that describe the outputs,
   every one "What the work is" lists, the doc comments, `path@cli@README.md`, the help in
   `path@cli@src/cli/mod.rs` and the error text of `write_all`. Fails alone on the content of
   `closing-sections.md`. Claims:
   - a passing `--verify` writes every agent's closing sections, in the order of `agents.txt`, an
     empty section included, at an audit as outside one, in the format of the decided design: a
     unit test beside `the_outputs_hold_each_subject_s_verdicts_and_the_generated_lines`, with ten
     agents or more so that `agent-10` sorts before `agent-2` in `agents`, and one in
     `path@testing@tests/binary.rs` over reports holding both sections; shown to fail by a mutation
     that skips the last agent, and one that iterates `agents` in place of `order`;
   - a failing run leaves no `closing-sections.md` of a previous run: the filter on the output names
     of `worklist_verify_leaves_no_output_of_a_previous_run_when_it_fails` widened; shown to fail by
     removing the name from `remove_outputs`;
   - the existing assertions on the count of files, `(out.lines, out.files.len(), out.gaps)` and the
     audit's file list, take the fifth file;
   - each test it adds removes the output directory it builds.
2. **Each verdict file holds its act verdicts whole first, then its light lines** (`query`). Builds
   the split in `outputs`, by the mapping table, its two forms of a verdict, the count line of
   `verify` per subject, and the doc comments and texts of step 1 that describe a verdict file.
   Fails alone on the order and shape of a verdict file. Claims:
   - every act verdict precedes every light verdict of a file, under the two part lines, each act
     with its fenced text and inbound list, each light verdict as its `===` line and verdict line
     with no fence: a unit test with one verdict of each row of the mapping table; shown to fail by
     a mutation that classes `worsens` as light, and one that ignores the owner-question;
   - `gaps.md` keeps every verdict whole, a light `touches` verdict included: a unit test over an
     audit that asserts a `touches, none` verdict's entry text in `gaps.md`; shown to fail by a
     mutation that writes the light line into `gaps.md`;
   - `generated-lines.md` and `fit-lines.md` are unchanged: their existing assertions pass
     unchanged;
   - the existing assertions the split turns false are changed to the new order and shape, each
     named under "What the work is": `headed(a)` in the first unit test, and the entry text that
     `worklist_verify_writes_from_the_snapshot_and_refuses_one_that_lacks_an_entry` asserts in a
     `constrains, respect` verdict's file.
3. **The workflow** (`agent-skills`). Builds
   `planned@agent-skills@content/workflows/record-search.js`, its `meta` and the three phases of the
   decided design, the logic test of `worklistCommand` in `path@agent-skills@src/workflow_parse.rs`,
   that module's doc and the lines of `path@agent-skills@CLAUDE.md` that describe the module; runs
   `cargo klarch install-agent-skills` and commits the installed copy with the change, as
   `path@agent-skills@CLAUDE.md` says, or the check fails. Fails alone on the flag builder's test
   or the parse check. Claims:
   - `worklistCommand` maps every argument by the mapping table, each element once and
     single-quoted, a value holding a `'` and one holding a backtick included: a test with several
     seeds, a `since` and a `seedsFrom`, and one with no `subjects` and no `seeds`; shown to fail by a
     mutation that drops the last seed (the premortem's cause 2), recorded in the commit;
   - the installed workflow parses, by `every_installed_saved_workflow_parses`, and its `meta` name
     is its stem, which `check_workflow_name` asserts at the build.
4. **The texts** (`agent-skills`). Rewrites the searcher's description as the decided design says;
   re-points the design skill's loop step 1, point 2 of
   `skill@knowledge-architect-planning@working-a-slice`, and the intake skill's point 4 to the
   workflow; rewrites the sentence of
   `path@agent-skills@content/agents/design-conformance-reviewer.md`, lines 55 to 57, "each verdict
   with its entry's text", to the act verdicts whole and the light verdicts as lines; reads
   `skill@knowledge-architect-planning@spec-contents` (line 337 of the source) and
   `skill@knowledge-architect-planning@cutting-steps-and-slices` (line 397), which run `--out` or
   copy an output and need no change unless their text names the dispatch; brings to the present
   `issue@agent-skills@a-saved-workflow-runs-the-review-before-a-merge`, line 19, which says the
   installed text ships saved workflows "for the audit axes alone"; installs with
   `cargo klarch install-agent-skills`; writes the CHANGELOG.md entries and runs `cargo x
   changelog`. Fails alone on `cargo klarch check`. Claims:
   - the description's by-hand line points to the script and restates no step of it (the
     premortem's cause 6): checked by reading the installed description against the script, and by
     the agent-facing-text axis of the review;
   - each skill that dispatches the search names the workflow: a search of the installed text for
     the searcher's name, read whole.
5. **The harvest**, per the harvest table.

## Order rationale

- Step 0 before step 1: the harvest writes into heads that already conform, and the review before
  the merge judges the harvest's change, not a head's older defects.
- Step 1 before step 2: the closing sections are a new file, the split changes an existing one, so
  a failure of the second is not mistaken for one of the first.
- Step 2 before step 3: the workflow returns the outputs the first two steps write.
- Step 3 before step 4: the texts name a workflow that exists.
- Step 4 before step 5: the harvest records what the texts state.

## Defaults awaiting the owner

- **Q1, ruled.** Does this discussion reopen `design@agent-skills@one-record-searcher`, as the
  re-entry of `issue@agent-skills@a-record-search-misses-what-is-added-after-it` names? Put in the
  reply to round 3 with the default no. The owner ruled no, round 4. Bears on #dispatch-text-home.
- **Q2, ruled.** `maxRounds` defaults to 3 runs of `--verify`, a value with no basis,
  put in the reply to round 3 as open to the owner's word, and approved with the design, round 4:
  the session chose 3, which no goal ranks. The owner approved the value, round 5.
  Bears on #report-loop-in-the-workflow.
- **D2, ruled.** The opt-in the Workflow tool requires: the default is the premise of
  the decided design, that a skill the session loads itself, which names the workflow, is no opt-in
  by the tool's description, that a standing authorization in the user's or the project's
  instructions is one, and that a session holding none dispatches by the by-hand line. A material
  finding of the plan's review. The owner approved the default, round 5. Bears on #dispatch-text-home and #fallback-dispatch-line.

## Harvest

| what | where | when |
| --- | --- | --- |
| #search-runs-a-saved-workflow, #workflow-runs-the-worklist, #worklist-notices-returned, #report-loop-in-the-workflow, #search-reads-a-fixed-tree, #dispatch-text-home, #fallback-dispatch-line | `search-dispatch-in-the-searcher-s-description` rewritten, its title and slug brought to the workflow, and the heads the entry tests admit in `path@agent-skills@docs/design/record-search.md`, the workflow's head carrying the owner's take-in under `goal@generic-tools-and-procedures-are-shipped` and the opt-in premise as D2 is ruled | after step 4 |
| #verdicts-split-by-action, #closing-sections-file | `design@agent-skills@session-acts-on-the-verdicts` rewritten; `design@query@occasion-shaped-output` rewritten | after step 4 |
| the fixed worklist and verify agents | `design@agent-skills@work-divided-by-judgement` gains the property of a step that runs a command | after step 4 |
| the writer of a report returned as text | `design@agent-skills@report-written-through-the-shell` gains the member | after step 4 |
| the re-take path of a workflow's agents | `design@query@worklist-bound-is-40000-words` | after step 4 |
| the site of the searchers' scratch directories | `design@agent-skills@a-scratch-directory-per-subagent` | after step 4 |
| #verdicts-verified-in-the-workflow | `register@agent-skills@rejected`, where the tests for a losing alternative admit it, cited by the rewritten heads of agent-skills | after step 4 |
| the light lines without reason, a format of `--verify`'s output | `register@query@rejected`, where the tests for a losing alternative admit it, cited by `design@query@occasion-shaped-output` | after step 4 |
| #search-workflow-logic-tested, #workflow-logic-test-home | the doc of `path@agent-skills@src/workflow_parse.rs` and the commit of step 3; a head only where the entry tests admit one | step 3 |
| #worklist-module-split, #binary-tests-split | the commit of step 1 | step 1 |
| the issues that cite the rewritten heads | each read, and a statement about where the dispatch lives brought to the present | after step 4 |
| the premortem's causes 3, 4 and 5 | judged by the tripwire standard; every cause with its outcome in the harvest's commit message | the harvest |

## Later consequences

- The reopening of `design@agent-skills@one-record-searcher` that
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it` waits for gets a cheaper fix:
  a second search after a new thread is one call of the workflow with the extended draft.
- `issue@agent-skills@a-saved-workflow-runs-the-review-before-a-merge` and
  `issue@agent-skills@no-test-runs-a-saved-workflow-s-logic` gain an instance of a saved workflow
  outside the audits and of a pure function tested under `node`.
- The split signals the code-split scout found in `path@query@src/worklist.rs` and
  `path@testing@tests/binary.rs` still hold after this work; a later change to the writer of
  `worklist.rs`, or to the report format both its parts read, weighs them again.
