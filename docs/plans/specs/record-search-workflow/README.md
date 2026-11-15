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
  waits. Every default this spec lists is ruled.
- Its line numbers name the tree at the commit "[intake] The forty-seven retrospective analyses
  leave, every outcome carried out", on which it was written.
- It was assembled from the transcript of one session. The discussion begins at the owner's message
  that opens "There's an issue about the record search being launched manually", and ends at the
  owner's message that opens "q1 no. The design looks right, proceed". It holds four owner messages,
  the rounds:
  - **round 1**, opening "There's an issue about the record search being launched manually";
  - **round 2**, opening "I had plans to design, but not specific plans";
  - **round 3**, opening "This new shape looks right";
  - **round 4**, opening "q1 no. The design looks right, proceed".

  "The reply to round n" is the session's messages after the owner's message of round n. The reply
  to round 3 holds the draft of this spec, the code-split scout's report, the record search run
  through a draft of the workflow, and the experiment on a cross-check stage; the reply to round 4
  holds the premortem.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec.

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
- **the notices**: the lines `worklist --out` prints on stdout, by `outln!` in `write`,
  `path@query@src/worklist.rs`, lines 396 to 445: the words of the work and the bound left, the
  seeds `--seeds-from` took, a notice per group above the bound (`notice`, line 455), and the count
  of agents.
- **the flag builder**: the pure function of the workflow that turns its `args` into the
  arguments of `worklist --out`, `worklistCommand` under New names.
- **the parse check**: `path@agent-skills@src/workflow_parse.rs`, which wraps each installed saved
  workflow in `async function body() {…}` and hands it to `node --check`; it skips without `node`
  unless the `KNOWLEDGE_ARCHITECT_CI` variable that `cargo x gates --ci` sets requires it (`CI_VAR`,
  `required`, `checked`).
- **the experiment**: the cross-check stage the draft of the workflow ran in the reply to round 3,
  one agent per subject trying to refute the searchers' verdicts, real use per
  `design@agent-skills@synthetic-evidence-not-built`.
- **Q1, D1**: the question and the default put to the owner, under "Defaults awaiting the owner".

## What the work is

What exists today at each site the work touches:

- **The dispatch** is the searcher's description: the session runs `klarch worklist --out <dir>`
  with `--work`, `--subject` and `--seed`, and at an audit `--seeds-from` and `--since`; sends one
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
  file's stem, lines 309 to 311, and asserts that the `meta` name is that stem, lines 124 to 128,
  per `design@agent-skills@saved-workflow-name-is-its-file-stem`. A script reads no file and runs no
  command, and takes its inputs in `args`, per the same audit head; an audit workflow's preflight
  agent runs the checker's commands, as `{{command}} issues` at line 280 of
  `path@agent-skills@content/workflows/design-record-audit.js`.
- **What a passing `--verify` writes** is `outputs`, `path@query@src/worklist.rs`, line 1401, per
  `design@query@occasion-shaped-output`: per subject `verdicts-<subject>.md`, every verdict but
  `unrelated` whose `against` names it, in the order of `agents.txt`, each as its `===` line, the
  agent's verdict line (`Verdict::line`, line 1584, every field including the quotation and the
  reason) and the entry's own text and inbound list, fenced; outside an audit,
  `generated-lines.md`; at an audit, `gaps.md` and `fit-lines.md` in its place. `remove_outputs`,
  line 1259, removes the outputs of a previous run first; `verify`, line 953, prints one count line
  per output. `judge_report`, line 1171, refuses a report missing either closing section, and no
  file holds their content.
- **The parse check** runs no part of a script and judges none of its logic: its module doc, lines
  8 to 10. The xtask test `the_parse_test_reads_the_library_s_ci_variable`,
  `path@xtask@src/gates.rs` line 100, reads the module by `include_str!` to check the spelling of
  `CI_VAR`.
- **The texts that describe the `--verify` outputs**: the worklist section of `path@cli@README.md`,
  lines 383 to 410; the help of `--verify` in `path@cli@src/cli/mod.rs`, lines 120 to 131; the doc
  comments of `WorklistMode::Verify` (lines 114 to 119), of the output constants (lines 38 to 50)
  and of `outputs` (lines 1376 to 1400) in `path@query@src/worklist.rs`.
- **The tests of the outputs**: in `path@query@src/worklist.rs`,
  `the_outputs_hold_each_subject_s_verdicts_and_the_generated_lines` (line 2538) and
  `an_audit_s_outputs_hold_its_gaps_and_its_fit_lines` (line 2647); in
  `path@testing@tests/binary.rs`, where `path@query@CLAUDE.md` puts the tests of the commands'
  output, `worklist_verify_leaves_no_output_of_a_previous_run_when_it_fails` (line 7698) and
  `worklist_verify_writes_from_the_snapshot_and_refuses_one_that_lacks_an_entry` (line 7854), built
  on `worklist_project` (line 6518).

What the two closed issues held, carried here:

- the closing condition of the search's workflow: a design argued under
  `skill@knowledge-architect-design`, then a saved workflow shipped by the installed text with what
  the design decides about testing its logic. The searches the intake of forty-seven files counted
  dispatched 24 to 27 agents each, by hand, and one design audit 36;
- the closing condition of the closing sections: every report's closing sections put before the
  dispatching session, with a test of `--verify` over reports holding them. Its instance: in a
  search of 2026-11-11, the harness cut 16 of 18 searchers' returns, the session read the verdict
  files and the generated lines and never read those 16 reports' closing sections, two items of
  which bore on the work and reached it only through the transcript reviewer. The issue's three
  shapes were a file `--verify` writes (taken), the sections appended to an existing output (not
  argued by the discussion), and the reading by hand (lost, per #closing-sections-file).

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
- `design@agent-skills@audit-axis-runs-a-workflow`: the precedent. An occasion's steps live in a
  saved workflow of the installer's namespace; a session whose harness does not list the workflow
  dispatches its steps by hand, in the script's order, with its prompts, read from the script. Its
  title scopes it to the audit axes, and this work does not rewrite it.
- `design@agent-skills@saved-workflow-name-is-its-file-stem`.
- `design@agent-skills@work-divided-by-judgement`: a fixed dispatch of the installed text stands
  only on a property of the step; "An output verified by a session other than its author is no such
  property". It rules out #verdicts-verified-in-the-workflow; the workflow's own fixed dispatches
  stand on "an output whose form a command checks". Its measured fact, that a general-purpose
  subagent holds no `Workflow` tool, grounds #fallback-dispatch-line.
- `design@agent-skills@at-least-one-independent-verification`: one independent verification of
  each claim acted on; a form check such as `--verify` is none.
- `design@agent-skills@verdicts-reach-the-conformance-review`: a verdict the session misjudges has
  a reader at the plan document's review and at the merge.
- `design@agent-skills@design-audit-checks-fit`: an audit reads the notices of the search's own
  `worklist` run. The design keeps it by #worklist-notices-returned, so its text stays true.
- `design@agent-skills@synthetic-evidence-not-built`: this discussion's own search ran through a
  draft of the workflow as real use, and the experiment with it.
- `design@query@worklist-check-judges-one-report` and `design@query@worklist-writes-and-verifies-the-reports`:
  the checks of a report, unchanged.
- `design@gates@ci-flag-for-ci-only-requirements`: the logic test, like the parse check, skips
  without `node` and fails under `--ci`.

The decisions the work rewrites, each with every text the rewrite repairs, the repair made by step
4 for the installed text and by the harvest for the design homes:

- `design@agent-skills@one-record-searcher`: its sentence "Its description carries the dispatch,
  since a dispatcher reads an agent's description and never its body" turns false: the workflow
  carries the dispatch, and the description names the workflow. The decision stands: one agent per
  directory, the check by `--verify`, a report sent back and never repaired by the dispatcher. Texts:
  the searcher's description; the `%%` comment of
  `path@agent-skills@content/skills/retrospective-intake/SKILL.md` line 71, which names the head and
  stays true; `path@agent-skills@docs/design/audit-method.md` line 169; the issues that cite it,
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it`,
  `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment` and
  `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`, each
  read by the harvest for a statement about where the dispatch lives.
- `design@agent-skills@session-acts-on-the-verdicts`: how the verdicts reach the session, per
  #verdicts-split-by-action, and where it reads the closing sections, per #closing-sections-file;
  its sentence "The searcher's description states these acts" stays, the acts staying in the
  description. Texts: the searcher's description; the design skill's loop step 1; the planning
  skill's point 2 of `skill@knowledge-architect-planning@working-a-slice`; the intake skill's point 4; the design homes that cite it,
  `path@agent-skills@docs/design/dispatching.md` line 115, `path@agent-skills@docs/design/record-search.md`
  lines 16, 88, 116 and 200, `path@agent-skills@docs/design/reviews.md` line 77; and
  `issue@agent-skills@a-lighter-pass-for-untouched-components`, line 73, read by the harvest.

The heads the harvest touches without reversing or rewriting them, per
`skill@knowledge-architect-planning@cutting-steps-and-slices`:

- `design@query@occasion-shaped-output`: widened by `closing-sections.md` and by the order inside
  each verdict file. Its argument "a session that hands one subject of the work to an agent hands
  it that subject's file" keeps the files per subject. Texts: the doc comments of `outputs` and of
  the output constants, `path@cli@README.md`, the help in `path@cli@src/cli/mod.rs`, repaired by
  steps 1 and 2.
- `design@query@worklist-writes-and-verifies-the-reports`: it names the outputs only through
  `design@query@occasion-shaped-output`; the harvest edits it only where its text names an output.
- `design@query@worklist-bound-is-40000-words`: its re-take sentence names a searcher's transcript
  at `subagents/agent-<id>.jsonl`; a workflow's agents sit at `subagents/workflows/wf_<id>/`, as
  measured in the reply to round 3. The harvest rewords the path to cover both.
- `design@agent-skills@a-scratch-directory-per-subagent`: its site, "the record searcher's
  description, which carries its dispatch rule, makes each agent directory that `klarch worklist`
  writes its searcher's scratch directory", becomes the workflow, which gives each searcher its
  agent directory. Text: the primer's `primer@dispatching-work`, which cites it, stays true.

The judgement of `agent@knowledge-architect-design-record-auditor` on these four heads, and on the
two rewritten ones, had not run when this spec was committed; the design audit of point 2 of
`skill@knowledge-architect-planning@working-a-slice`, or the session that starts the work, runs it
before step 1, and a head it finds non-conformant gets a step before the harvest.

The discussion's record search, run through a draft of the workflow over the draft of this spec
with each thread as a subject, gave 233 verdicts other than `unrelated`: 221 `constrains` or
`touches` with `respect` or `none`, 6 `conflicts, check-reversal`, 4 `worsens, respect`, 2
`closes, close-with-the-work`. The experiment refuted 17 of the 430 verdicts it checked; three of
its refutations changed the design. Each verdict whose outcome changed something, with that
outcome:

| entry | verdict | outcome |
| --- | --- | --- |
| `design@agent-skills@one-record-searcher` | conflicts | rewritten, above |
| `design@agent-skills@session-acts-on-the-verdicts` | conflicts | rewritten, above |
| `design@query@worklist-writes-and-verifies-the-reports` | conflicts | touched only where its text names an output, above |
| `design@query@occasion-shaped-output` | conflicts | widened; its argument keeps the files per subject, above |
| `design@query@worklist-bound-is-40000-words` | conflicts | its re-take path reworded, above |
| `design@agent-skills@a-scratch-directory-per-subagent` | conflicts | its site rewritten, above |
| `design@agent-skills@work-divided-by-judgement` | refuted to conflicts by the experiment | rules out #verdicts-verified-in-the-workflow |
| `design@agent-skills@design-audit-checks-fit` | refuted to conflicts by the experiment | the new thread #worklist-notices-returned |
| `goal@serves-one-kind-of-project`, `issue@agent-skills@heads-argued-from-another-harness` | refuted by the experiment | the new thread #fallback-dispatch-line, argued on a measured fact of `claude`, not on another harness |
| `issue@agent-skills@a-record-search-misses-what-is-added-after-it` | refuted to trigger-met by the experiment, with an owner-question | Q1, ruled no in round 4 |
| `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` | worsens | respected: the by-hand line covers a refused run too |
| `issue@check@a-declared-command-with-a-quote-breaks-the-installed-workflows` | worsens | respected: the script uses `{{command}}` only as the audit workflows do |
| `issue@testing@a-test-run-leaves-directories-in-the-temporary-directory` | worsens | respected: each test step 1 or 2 adds removes the output directory it builds |
| `issue@model@references-are-read-in-markdown-and-rust-only` | worsens | respected: the script cites no entry, and the description keeps the pointers that need checking |
| `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project` | touches | respected: a comment for this repository's maintainers is a `%%` line |
| `a-saved-workflow-runs-the-record-search`, `no-command-gathers-the-closing-sections-of-the-searchers-reports` | closes | closed by the commit that adds this spec, which schedules their work |

The other verdicts are in [the discussion's search](search/discussion.txt), and the work keeps
within the sentence each quotes. The search printed no notice on a cluster.

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
              subjects     [ids]    each a `--subject`
              seeds        [refs]   each a `--seed`
              since?       a commit, `--since`
              seedsFrom?   a plan document's README.md, `--seeds-from`
              bound?       `--bound`
              maxRounds?   the rounds of the report loop, default 3 (D1)
  phases      worklist, search, verify
  worklistCommand(args)    the flag builder, a pure function
  result      { refusal } | { failing } | { notices, verdictFiles, generatedLines | gaps, fitLines, closingSections }
crates/agent-skills/src/workflow_parse.rs                 existing: gains the logic test of worklistCommand
crates/query/src/worklist.rs                              existing
  CLOSING_SECTIONS_FILE = "closing-sections.md"           new: written by a passing `--verify`
  outputs, Outputs, remove_outputs, verify                existing: changed
```

## Decided design

### The search runs one saved workflow, for every occasion

From #search-runs-a-saved-workflow. The installed text ships `knowledge-architect-record-search`,
`planned@agent-skills@content/workflows/record-search.js`, for every occasion of the search: the
occasions differ only by the flags their `args` give, and `--seeds-from` alone switches the outputs
to `gaps.md` and `fit-lines.md`. The nearest rival, a rewording of the dispatch sentence so that a
session dispatches better by hand, leaves 22 to 36 hand dispatches and checks per search: 22 in
the discussion's own search, 24 to 27 in the five searches the intake of forty-seven files counted,
36 in one design audit.

Illustration of the shape, not authority:

```js
phase('worklist')   // one agent, low effort, runs `{{command}} worklist --out` + worklistCommand(args)
                    // returns { agents, notices } or { refusal }
phase('search')     // one record searcher per agent directory, in parallel
                    // returns { written, reportText? }
phase('verify')     // one low-effort agent per text report writes it into its report.md;
                    // one agent runs `--verify` and returns { passed, failing: [{ agent, findings }] };
                    // each failing searcher sent again with its findings, up to maxRounds
return { notices, verdictFiles, generatedLines /* or gaps, fitLines */, closingSections }
```

### An agent of the workflow runs `worklist --out`, and the workflow returns its notices

From #workflow-runs-the-worklist and #worklist-notices-returned.

- The workflow's first step is one agent at low effort that runs `{{command}} worklist --out
  <scratch>` with the arguments `worklistCommand(args)` builds, since a script reads no file and
  runs no command.
- It returns, by a schema, the `agent-<n>` directories `--out` wrote and every line `--out` printed
  on stdout, the notices; or, where `--out` refuses, its refusal verbatim, which the workflow
  returns as its result, and the session repairs its `args` and starts the workflow again.
- The workflow returns the notices with its other outputs, so a design audit reads them as
  `design@agent-skills@design-audit-checks-fit` asks: `--out` prints them on stdout only, by
  `outln!` in `write`, and writes them into no file of the directory.

### The workflow writes the text reports, runs `--verify`, and sends a failing report back

From #report-loop-in-the-workflow.

- Each searcher's brief gives it its agent directory, which is its scratch directory, and the path
  of `<scratch>/work.txt`; it returns whether it wrote its report, or the report as text where the
  harness refused every write of it.
- **Each report returned as text is written into its `report.md` by a low-effort agent of its
  own**, before `--verify` runs, so that no one agent's prompt holds every text report: a clause of
  the premortem's cause 1.
- One agent then runs `{{command}} worklist --verify <scratch>` and returns whether it passed, its
  output, and each failing agent with its findings. The script sends each failing agent's searcher
  again, naming the findings, and runs `--verify` again, for at most `maxRounds` rounds, default 3
  (D1); past them it returns `{ failing }`, and caps nothing silently.
- **No step of the workflow repairs a report**: a report that fails goes back to its searcher, per
  `design@agent-skills@one-record-searcher`.
- No cross-check stage: per #verdicts-verified-in-the-workflow. The session may still hand a
  subject's verdict file to an agent it dispatches, at its judgement.

### A passing `--verify` writes the closing sections of every report into one file

From #closing-sections-file. A passing `--verify` writes `closing-sections.md` into the directory,
at every occasion, an audit included: for each agent of `agents.txt`, in its order, the agent's
name and its report's text from its first closing section to its end, fenced as a verdict file
fences an entry's text, so that the report's headings read as no section of the file. An agent
whose sections are empty is listed with them empty, so a session tells a report with nothing to
carry from one missing. `remove_outputs` removes it with the other outputs before a run judges
anything, and `verify` prints its line beside the other counts. The workflow returns its path, and
the session carries each item into its own sections.

The nearest rivals: an agent of the workflow gathering the sections, a copy by a reader of the kind
that missed sixteen reports; and the reading of each `report.md` by hand, which in a workflow is 22
file reads per search, since the searchers' returns never reach the session.

### Each verdict file holds its act verdicts whole first, then its light verdicts as lines with their quotation and reason

From #verdicts-split-by-action, as revised in the reply to round 3 and approved in round 4.

- `--verify` keeps one `verdicts-<subject>.md` per subject.
- Inside each file, every act verdict comes first, in the order of `agents.txt`, each as today: its
  `===` line, its agent's verdict line, and its entry's own text and inbound list, fenced.
- Every light verdict follows, in the order of `agents.txt`: its `===` line, with the changed-since
  mark where it applies, and its agent's verdict line, which carries the quotation **and the
  reason**, without the entry's text or its inbound list.
- An act verdict is defined by the mapping table below. A verdict with a non-empty owner-question
  is an act verdict, since putting it to the owner is an act of the session, per
  `design@agent-skills@session-acts-on-the-verdicts`.
- `generated-lines.md`, `gaps.md` and `fit-lines.md` do not change.
- The session reads the act verdicts with their entries whole and verifies each as it acts, and
  checks each light line against the shapes it holds, running `show` where a line looks like a
  downgrade.

The argument: a search returns two populations, 1 to 11 verdicts that need an act and 19 to 177
that ask the work to keep within a sentence, over four past searches. The session must read an
act verdict's entry whole to act on it, and its context is decisive there. The first shape, light
lines without the reason, let through three load-bearing verdicts the searchers' reasons carried:
the defect was the format. With their reasons, the light lines of the discussion's search are 95 KB
for 178 entries, against 1.5 MB of verdict files, one entry appearing under each of its subjects.

### The session edits nothing while the workflow runs

From #search-reads-a-fixed-tree. The workflow's `description` in its `meta`, and its result, state
that the session edits nothing in the tree until the workflow returns, since the searchers read the
session's tree, which their instructions say nobody edits while they search. The rival, a reading
worktree at a commit, fails on the uncommitted draft and the uncommitted entries a search often
reads.

### The dispatch lives in the script, and the searcher's description keeps what the session needs

From #dispatch-text-home and #fallback-dispatch-line.

- The searcher's description keeps: what the agent does; the inputs the session chooses, the work
  files, the subjects and the seeds, and at an audit `since` and `seedsFrom`, as the dispatching
  skill names them; the workflow's name and its `args`; what each output is for and how the session
  acts on it, the acts of `design@agent-skills@session-acts-on-the-verdicts`.
- It keeps one line for a session that cannot run the workflow: one whose harness does not list it,
  one running inside an agent, which holds no `Workflow` tool, or one whose run is refused. That
  session dispatches the steps by hand, in the script's order, with the script's prompts, read from
  the script. **The line points to the script and restates no step of it**: a clause of the
  premortem's cause 6. It is argued on the measured fact of `claude`, per
  `design@agent-skills@work-divided-by-judgement`, not on another harness, which
  `goal@serves-one-kind-of-project` excludes building for.
- The design, planning and retrospective-intake skills name the workflow where they dispatch the
  search; naming it is the opt-in the Workflow tool requires, per
  `design@agent-skills@audit-axis-runs-a-workflow`.
- The script cites no entry, as every saved workflow, per `path@agent-skills@CLAUDE.md`; a comment
  for this repository is a `%%` line.

### The flag builder is a pure function, tested under `node` beside the parse check

From #search-workflow-logic-tested and #workflow-logic-test-home.

- `worklistCommand(args)` is a pure function of the script: it maps each `work` to a `--work`, each
  `subjects` entry to a `--subject`, each `seeds` entry to a `--seed`, `since` to `--since`,
  `seedsFrom` to `--seeds-from` and `bound` to `--bound`, each value quoted for the shell. It is the
  one piece of the script whose failure is silent: a dropped or misquoted seed weakens the search,
  and nothing reports it.
- Its test lives in `path@agent-skills@src/workflow_parse.rs`, the one module that runs `node` on a
  shipped workflow and owns the CI requirement on it (`checked`, `required`, `CI_VAR`). The test
  calls the function with fixed `args` and compares its result; it skips without `node` and fails
  under `cargo x gates --ci`, as the parse check does. How it reaches the function without running
  the script's body, which calls the harness's `agent` and `phase`, is the step's to build.
- The module's doc, which says it "runs no part of the script and judges none of its logic", is
  rewritten to its two purposes. The module is not renamed: the discussion decided none, and the
  xtask test reads it by its path.

### No split of `worklist.rs` or of `binary.rs` in this work

From #worklist-module-split and #binary-tests-split. The work writes into the judge part of
`path@query@src/worklist.rs` alone, and adds tests to `path@testing@tests/binary.rs` where
`path@query@CLAUDE.md` puts them.

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

The flag of `worklist --out` each argument of the workflow takes:

| `args` | flag |
| --- | --- |
| `scratch` | `--out <scratch>` |
| each of `work` | `--work <path>` |
| each of `subjects` | `--subject <id>` |
| each of `seeds` | `--seed <ref>` |
| `since` | `--since <commit>` |
| `seedsFrom` | `--seeds-from <path>` |
| `bound` | `--bound <n>` |
| `maxRounds` | none: the script's own |

## Losing alternatives

- **A rewording of the dispatch sentence**, the rival of #search-runs-a-saved-workflow: still 22 to
  36 hand dispatches and checks per search.
- **The session running `--out` itself**, the rival of #workflow-runs-the-worklist: one more act
  for the session, which must know the flags to fill `args` anyway.
- **An agent of the workflow gathering the closing sections**, and **the reading by hand**, the
  rivals of #closing-sections-file: a copy of the kind that missed sixteen reports; 22 file reads
  per search, the returns never reaching the session.
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
  #fallback-dispatch-line.
- **The parse check alone**, the rival of #search-workflow-logic-tested: it judges no logic.
- **A new module running `node` beside the parse check**, the rival of #workflow-logic-test-home:
  it shares the CI items or carries a copy of the CI requirement the xtask test does not read.
- **A split of the judge of `worklist.rs`** and **the worklist tests moved to a test file of their
  own**, the rivals of #worklist-module-split and #binary-tests-split.

## Readings

Empty: the work reads no external specification the project implements. The behaviour of Claude
Code's Workflow tool it relies on is recorded in `design@agent-skills@audit-axis-runs-a-workflow`
and `design@agent-skills@work-divided-by-judgement`.

## Premortem

Run in the reply to round 4: assume the workflow shipped and failed.

| # | cause | thread stressed | outcome |
| --- | --- | --- | --- |
| 1 | Many searchers return their reports as text. The draft passes all of them through one verify agent's prompt, about 20 KB each, so that prompt grows past what one agent handles well | #report-loop-in-the-workflow | becomes a clause of the design: each text report is written by a low-effort agent of its own, before `--verify` |
| 2 | The flag builder drops or misquotes a seed or a `--since`, and the search silently grades against the wrong seeds | #search-workflow-logic-tested | a claim of the step, with the `node` test and a recorded mutation |
| 3 | The session edits the record while the workflow runs. `--verify` judges against the snapshot and passes, so the verdicts are stale | #search-reads-a-fixed-tree | named, dropped as a tripwire: the evidence lives only in a transcript (condition 2) |
| 4 | A real `conflicts` graded `constrains, respect` reaches the session only as a light line, which it does not read whole | #verdicts-split-by-action | named, dropped: the design-conformance review reads the verdict files and reports it (condition 1, not silent) |
| 5 | In the default permission mode the workflow's run is refused, and every search falls back to dispatch by hand | #search-runs-a-saved-workflow, #fallback-dispatch-line | named, dropped: the session sees the refusal (not silent); `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` holds the question |
| 6 | The by-hand line drifts from the script | #fallback-dispatch-line | turned into a clause of the design: the line points to the script and restates no step |

## Acceptance criteria

None: no acceptance criterion passes conditions 1, 3 and 4 of the standard of
`skill@knowledge-architect-planning@acceptance-criteria`. Causes 2 and 6 are what the built code
checks, and the others are reported by reviews or refusals.

## Implementation sequence

1. **`--verify` writes `closing-sections.md`** (`query`). Builds the constant, its write in
   `outputs`, its removal in `remove_outputs`, its count line in `verify`, and the texts that
   describe the outputs: the doc comments of `WorklistMode::Verify`, the output constants and
   `outputs`, `path@cli@README.md` and the help in `path@cli@src/cli/mod.rs`. Fails alone on the
   content of `closing-sections.md`. Claims:
   - a passing `--verify` writes every agent's closing sections, in the order of `agents.txt`, an
     empty section included, at an audit as outside one: a unit test beside
     `the_outputs_hold_each_subject_s_verdicts_and_the_generated_lines` and one in
     `path@testing@tests/binary.rs` over reports holding both sections; shown to fail by a
     mutation that skips the last agent;
   - a failing run leaves no `closing-sections.md` of a previous run: the filter on the output names
     of `worklist_verify_leaves_no_output_of_a_previous_run_when_it_fails` widened; shown to fail by
     removing the name from `remove_outputs`;
   - each test it adds removes the output directory it builds.
2. **Each verdict file holds its act verdicts whole first, then its light lines** (`query`). Builds
   the split in `outputs`, by the mapping table, and the doc comments and texts of step 1 that
   describe a verdict file. Fails alone on the order and shape of a verdict file. Claims:
   - every act verdict precedes every light verdict of a file, each act with its fenced text and
     inbound list, each light verdict as its `===` line and verdict line with no fence: a unit test
     with one verdict of each row of the mapping table; shown to fail by a mutation that classes
     `worsens` as light, and one that ignores the owner-question;
   - `generated-lines.md`, `gaps.md` and `fit-lines.md` are unchanged: the existing tests pass
     unchanged.
3. **The workflow** (`agent-skills`). Builds
   `planned@agent-skills@content/workflows/record-search.js`, its `meta` and the three phases of the
   decided design, the logic test of `worklistCommand` in `path@agent-skills@src/workflow_parse.rs`
   and that module's doc. Fails alone on the flag builder's test or the parse check. Claims:
   - `worklistCommand` maps every argument by the mapping table, each element once and quoted: a
     test with several seeds, a `since` and a `seedsFrom`; shown to fail by a mutation that drops
     the last seed (the premortem's cause 2), recorded in the commit;
   - the installed workflow parses, by `every_installed_saved_workflow_parses`, and its `meta` name
     is its stem, which the build asserts.
4. **The texts** (`agent-skills`). Rewrites the searcher's description as the decided design says;
   re-points the design skill's loop step 1, point 2 of
   `skill@knowledge-architect-planning@working-a-slice`, and the intake skill's point 4 to the
   workflow; reads `skill@knowledge-architect-planning@spec-contents` (line 337 of the source) and
   `skill@knowledge-architect-planning@cutting-steps-and-slices` (line 397), which run `--out` or copy an output and need no change unless
   their text names the dispatch; installs with `cargo klarch install-agent-skills`; writes the
   CHANGELOG.md entries and runs `cargo x changelog`. Fails alone on `cargo klarch check`. Claims:
   - the description's by-hand line points to the script and restates no step of it (the
     premortem's cause 6): checked by reading the installed description against the script, and by
     the agent-facing-text axis of the review;
   - each skill that dispatches the search names the workflow: a search of the installed text for
     the searcher's name, read whole.
5. **The harvest**, per the harvest table.

## Order rationale

- Step 1 before step 2: the closing sections are a new file, the split changes an existing one, so
  a failure of the second is not mistaken for one of the first.
- Step 2 before step 3: the workflow returns the outputs the first two steps write.
- Step 3 before step 4: the texts name a workflow that exists.
- Step 4 before step 5: the harvest records what the texts state.

## Defaults awaiting the owner

- **Q1, ruled.** Does this discussion reopen `design@agent-skills@one-record-searcher`, as the
  re-entry of `issue@agent-skills@a-record-search-misses-what-is-added-after-it` names? Put in the
  reply to round 3 with the default no. The owner ruled no, round 4. Bears on #dispatch-text-home.
- **D1, ruled.** `maxRounds` defaults to 3, a value with no basis, put in the reply to round 3 as
  open to the owner's word. The owner approved the design as displayed, round 4: the session chose
  3, which no goal ranks; the owner approved it. Bears on #report-loop-in-the-workflow.

## Harvest

| what | where | when |
| --- | --- | --- |
| #search-runs-a-saved-workflow, #workflow-runs-the-worklist, #worklist-notices-returned, #report-loop-in-the-workflow, #search-reads-a-fixed-tree, #dispatch-text-home, #fallback-dispatch-line | `design@agent-skills@one-record-searcher` rewritten, and the heads the entry tests admit in `path@agent-skills@docs/design/record-search.md` | after step 4 |
| #verdicts-split-by-action, #closing-sections-file | `design@agent-skills@session-acts-on-the-verdicts` rewritten; `design@query@occasion-shaped-output` widened | after step 4 |
| the re-take path of a workflow's agents | `design@query@worklist-bound-is-40000-words` | after step 4 |
| the site of the searchers' scratch directories | `design@agent-skills@a-scratch-directory-per-subagent` | after step 4 |
| #verdicts-verified-in-the-workflow, the light lines without reason | `register@agent-skills@rejected`, where the tests for a losing alternative admit them, cited by the rewritten heads | after step 4 |
| #search-workflow-logic-tested, #workflow-logic-test-home | the doc of `path@agent-skills@src/workflow_parse.rs` and the commit of step 3; a head only where the entry tests admit one | step 3 |
| #worklist-module-split, #binary-tests-split | the commit of step 1 | step 1 |
| the issues that cite the rewritten heads | each read, and a statement about where the dispatch lives brought to the present | after step 4 |
| the premortem's causes 3, 4 and 5 | none: each dropped at the premortem | none |

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
