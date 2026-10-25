# `klarch transcript` lists, shows and searches the sessions of Claude Code's transcripts, and the installed text uses it in place of a hand extraction

## Status and audience

This spec is the plan of one branch and one pull request, on the owner's word in round 3: "This
work should be a single spec and PR". The work adds a command of the core, `transcript`, with three
verbs, `list`, `show` and `search`, that read the session transcripts Claude Code writes on disk,
cut them into sessions, label every kept line by its fields, and report every line they do not
recognise. The installed text that tells a session to extract a transcript by hand, and to find a
transcript file by searching it, is rewritten to run the command.

It is written for a session that did not witness the design discussion that produced it. It leaves
the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point that "What is already decided" does not list
  as rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. No default awaits the owner.
- The discussion ran in one session. The message where it begins, the owner's first message, opens
  with: "Right now, the transcript reviewer needs to assemble the transcript by hand." A reviewer
  finds the session's file by that message, never by a session identifier.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, its last paragraph, for the work of a spec.

## Names

| name | what it is |
| --- | --- |
| the harness | Claude Code, the only agent harness this command reads, per `criterion@transcript-command@claude-harness-only` |
| transcript directory | `<config>/projects/`, where `<config>` is the value of `CLAUDE_CONFIG_DIR`, or `~/.claude` where it is unset; one subdirectory per project the harness ran in, holding one `<sessionId>.jsonl` file per session |
| line | one JSON object of a `.jsonl` file, with a `type` field |
| chain line | a line whose `type` is `user`, `assistant`, `attachment` or `system`: these carry `uuid`, `parentUuid`, `sessionId`, `timestamp`, `cwd`, `gitBranch` and `version` |
| session | a run of chain lines of one file, cut by `thread@transcript-command@transcript-session-split`, with the subagent files attached to it |
| handle | the name the command gives a session: the first 8 characters of its `sessionId`, with `#2`, `#3` … where one `sessionId` holds more than one session |
| subagent file | `<project dir>/<sessionId>/subagents/agent-<agentId>.jsonl`, beside `agent-<agentId>.meta.json`; workflow agents sit under `subagents/workflows/wf_<id>/` |
| label | the role the command gives a kept line, per the mapping table "Line to label" |
| header | the lines a verb prints before its rows, each opening with `# `: the files read, the harness versions seen, and the inventory of what was not recognised |
| the extraction rule | today, `agent@knowledge-architect-transcript-reviewer@extract-transcripts`: which lines are kept, and which are the owner's |
| P1 … P6 | the causes of the premortem, under Premortem |
| AC1, AC2, AC3 | the labels under which the acceptance criteria were put to the owner |
| K1 … K8 | the labels under which the criteria were shown to the owner, each an item under Criteria |

## What the work is

What exists today at each site the work touches:

- **The core's commands**, the enum `Command` of `path@core@src/cli/mod.rs`, dispatched by `run`
  in the same file: `check`, `show`, `issues`, `tripwires`, `index`, `model`, `anchors`, `commits`,
  `install-agent-skills`, `init`, `new-component`, `worklist`. No command reads a transcript.
- **The core's dependencies**, `path@core@Cargo.toml`: no JSON parser. `serde_json` 1.0.151 is
  already in `path@knowledge-architect@Cargo.lock`, through `tree-sitter` and tools/xtask.
- **The core's README**, `path@core@README.md`: a command summary block, the table under "Exit
  codes", one section per command.
- **The extraction rule**, `agent@knowledge-architect-transcript-reviewer@extract-transcripts`:
  keep `user` and `assistant` lines and their `text` items, `queued_command` attachments, and the
  owner's answers to a question tool; label each `user` line by `isMeta`, `isCompactSummary` and
  `origin.kind`; fall back to a harness tag at the start of the text only in a log that carries
  neither field. Each session that applies it writes its own script.
- **Finding a transcript file**, `skill@knowledge-architect-review@review-axes`, its paragraph
  "Find each transcript file by the message where the work begins, never by a session identifier":
  search the transcript directory for the message's opening words.
- **The assembly of a plan document**, `skill@knowledge-architect-planning@spec-contents`, its
  paragraph "Assembly from the transcript": a subagent applies the extraction rule, and the status
  section quotes the message where the discussion begins in each session.
- **The re-take of a premise**, `design@agent-skills@ledger-from-transcript`: a `jq` command finds a
  log's compaction boundaries.

What the harness writes, measured on this machine over 569 files and 181,280 lines, versions 2.1.306
to 2.1.310, by the scripts of `arg@transcript-command@arg-3`:

- the top-level `type` values: `user`, `assistant`, `attachment`, `system`, and the metadata types
  `api-request`, `api-request-blob`, `api-request-shape`, `last-prompt`, `atis-latch`, `mode`,
  `permission-mode`, `ai-title`, `queue-operation`, `pr-link`, `file-history-delta`,
  `file-history-snapshot`, `cost-state`; and in a workflow's `journal.jsonl`, `launched`,
  `started`, `result`;
- the `system` subtypes: `turn_duration`, `compact_boundary`, `scheduled_task_fire`,
  `informational`, `local_command`;
- the `origin.kind` values on `user` lines and on `queued_command` attachments: `human`, `peer`,
  `task-notification`, `coordinator`;
- the `attachment.type` values: `total_tokens_reminder`, `deferred_tools_record`, `prompt_snapshot`,
  `environment`, `queued_command`, `date`, `instructions`, `model`, `session_context`,
  `credential_org`, `mcp_instructions_delta`, `remote_session_change`, `skill_listing`, `auto_mode`,
  `silent_turn_reminder`, `deferred_tools_delta`, `agent_listing_delta`, `edited_text_file`,
  `nested_memory`, `command_permissions`, `read_truncation_notice`, `structured_output`,
  `hook_success`, `hook_additional_context`, `file`, `task_status`, `compact_file_reference`,
  `invoked_skills`, `hook_system_message`, `thinking_drop`;
- a compaction stays in its file and its `sessionId`: a `system` line of subtype `compact_boundary`
  with `parentUuid` null and a `logicalParentUuid`, then a `user` line with `isCompactSummary` true
  (7 of 7 compactions);
- a resumed session appends to its original file under its original `sessionId`, after a
  `cost-state` line, while the new process's id shows only in a `session_id` field (3 of 3 resumes);
- `/clear` starts a new file under a new `sessionId`, whose lines still carry the earlier id in
  `session_id`; the new file holds the command as a `user` line `<command-name>/clear</command-name>`
  with neither `isMeta` nor `origin`, then a `system` line of subtype `local_command` whose
  `commandRun` is `{"command": "clear", "args": ""}` and whose `parentUuid` is that `user` line's
  `uuid`, per `arg@transcript-command@arg-13`;
- `gitBranch` is on every `user` and `assistant` line of every top-level file measured.

Outside the work:

- **Another harness.** `criterion@transcript-command@claude-harness-only`;
  `issue@core@configuration-for-several-agent-providers` holds the general case.
- **Watching the tripwires of the installed skills from consumer sessions**: the owner's later aim,
  in `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`. The
  command is a tool that work can use; `design@agent-skills@tripwire-admission-test` condition 2,
  which keeps a tripwire's evidence out of transcripts, is unchanged by it.
- **The retention of transcripts.** The harness deletes them after `cleanupPeriodDays`, 30 days by
  default. The owner ruled in round 2 that no longer retention is needed; the README of the command
  states the default and the setting, per `arg@transcript-command@arg-15`.
- **Whether a command's output is a contract**: `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
  This command's text and `--json` output are declared no contract, as `anchors` and `worklist` do.
- **A library API.** Nothing of the transcript reader is public: the command is the interface.

## What is already decided

The decisions the design rests on and does not argue again:

- `design@core@the-core-cli-is-a-library-module`: the new command joins the core's commands, so every
  binary offers it.
- `design@core@arguments-parse-through-clap`, `design@core@exit-code-ladder`,
  `design@core@a-failed-parse-is-loud`, `design@core@one-question-one-command`,
  `design@core@single-crate`, `design@core@agents-table`, `design@core@nothing-of-a-project-is-compiled-in`
  (the harness's layout is a member of "the agent harnesses it knows"),
  `design@core@installed-binary-version-check` (the command runs inside a project, after the version
  refusal), `design@core@declared-command`, `design@core@api-facade`.
- `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@restatement-size-test`,
  `design@agent-skills@context-cost-of-agent-text`, `design@agent-skills@capabilities-not-structure`,
  `design@agent-skills@transcript-review-last-before-merge`, `design@agent-skills@plan-records-the-exchange`.
- `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@changelog-entries`,
  `design@knowledge-architect@the-branch-writes-its-changelog-entries`,
  `design@knowledge-architect@package-include-whitelist` (fixtures under crates/core/tests are not
  shipped).

The decisions the work rewrites, with every text `cargo klarch show` lists as citing them:

- `design@agent-skills@transcript-reviewer-agent`, its sentence "It is an agent, not a line in a
  skill, because its standard and its extraction rule are fixed, and the rule is learned from a
  failure": the extraction rule moves into the command, per
  `thread@transcript-command@transcript-labels-in-code`; the standard stays in the agent. Cited by
  `design@agent-skills@attribution-replaces-quotation` alone, which names the reviewer's paraphrase
  of the owner and not the extraction rule, so it is unaffected. The harvest rewrites the head, under
  `skill@knowledge-architect-decision-recording@reversal-check`.
- `design@agent-skills@ledger-from-transcript`, its re-take command: the `jq` command stays valid,
  and the harvest adds the command's header, which counts compaction boundaries, and the
  measurement of 7 of 7 compactions. Cited by `tripwire@agent-skills@ruling-lost-in-assembly` and
  `design@agent-skills@thread-slug-is-entry-id`, whose premises are unchanged.
- `design@core@the-core-cli-is-a-library-module`, its list of the core's commands, gains
  `transcript`: a member its argument covers.

The section `agent@knowledge-architect-transcript-reviewer@extract-transcripts` keeps its slug, so
no consumer citation dangles and no Migration entry is owed.

The search of the whole record at the discussion's grounding, seeded with
`goal@knowledge-architect@agents-get-a-complete-workflow`,
`goal@knowledge-architect@structure-and-workflow-work-together`,
`goal@knowledge-architect@any-project-can-adopt-it` and the issue
`a-general-purpose-subagent-is-refused-a-file-access`, since closed, graded 173 entries `bears`
or `may-bear`, each read whole. Their outcomes:

- **Bears**, each a constraint met by the decided design: the goals
  `goal@knowledge-architect@agents-get-a-complete-workflow`,
  `goal@knowledge-architect@structure-and-workflow-work-together` ("the tool computes the
  workflow's work lists"), `goal@knowledge-architect@any-project-can-adopt-it`,
  `goal@knowledge-architect@the-workflow-improves-through-real-use`,
  `goal@knowledge-architect@adoption-is-easy` (the README section),
  `goal@knowledge-architect@the-owner-decides`, `goal@agent-skills@installed-text-works-anywhere`
  (`{{command}}`), `goal@agent-skills@no-contradiction-in-workflow` (three installed texts change in
  one step), `goal@agent-skills@installed-text-leaves-room-to-judge`; the heads listed above; the
  heads `design@core@section-homes-carry-slugs` (the slug is kept),
  `design@agent-skills@review-judges-additions` and `design@agent-skills@design-home-write-loads-recording`
  (procedures the branch follows), `design@agent-skills@plan-written-in-the-converging-session`;
  `goal@xtask@repeated-tasks-are-automated`, a rival home, `thread@transcript-command@transcript-command-in-core`;
  `issue@knowledge-architect@command-output-is-not-declared-a-contract`, above;
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, above;
  `issue@core@configuration-for-several-agent-providers`, above;
  the issue `a-general-purpose-subagent-is-refused-a-file-access`, cause P6; the tripwires
  `tripwire@agent-skills@ruling-lost-in-assembly`, `tripwire@agent-skills@ruling-lost-in-change` and
  `tripwire@agent-skills@attribution-loses-the-scope`, whose re-entries read transcripts and whose
  premises the work leaves standing.
- **May-bear, bearing on nothing in this work**: every other entry the search graded `may-bear`.
  Most cite a seed and hold no bearing on reading a transcript; the rest bear on procedures the
  branch follows, such as `design@agent-skills@existing-heads-on-touch`, or on another question,
  such as `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`. Each was read
  whole; the commit that adds this spec names each with its outcome.

## Criteria

### The command reads Claude Code's transcripts and no other harness's (K1) `##claude-harness-only`

Binding. Source: the owner, round 1: "I only use claude code, so its fine for this command to only
handle transcripts from claude code … Other harnesses can wait for a request from another consumer,
if that ever happens." Met by every thread.

### Lines are selected and labelled by their fields, never by a substring of their text, and no line of the owner's is dropped (K2) `##labels-by-fields`

Binding, as a presumption derived from `design@agent-skills@transcript-reviewer-agent`: "the rule
is learned from a failure: a filter on text substrings once dropped one of the owner's messages".
Met by `thread@transcript-command@transcript-labels-in-code`, checked by
`acceptance@transcript-command@ac-owner-tag-text-stays-owner`.

### Every line the command does not recognise is counted and reported, never skipped in silence (K3) `##nothing-unrecognised-silent`

Binding, as a presumption derived from `design@core@a-failed-parse-is-loud`, and the owner's
condition, round 2: "as long as we report anything not recognized so no one misses one such item".
Met by `thread@transcript-command@transcript-format-tolerated-and-reported`, checked by
`acceptance@transcript-command@ac-unknown-values-reported`.

### The extraction rule has one home, and the hand procedure leaves the installed text (K4) `##one-home-for-the-rule`

Binding, as a presumption derived from `design@core@one-question-one-command`. Met by
`thread@transcript-command@transcript-labels-in-code`.

### The installed text names the command as the project declares it (K5) `##command-placeholder-in-installed-text`

Binding, derived from `goal@agent-skills@installed-text-works-anywhere`. Met by
`thread@transcript-command@transcript-query-verbs`: every new sentence writes `{{command}} transcript`.

### A harness release that only adds types costs no release, and one that changes a value the labels rely on is reported (K6) `##drift-cheap-and-visible`

Weighed. Source: Anthropic's documentation, `arg@transcript-command@arg-2`. Met by
`thread@transcript-command@transcript-format-tolerated-and-reported` and
`acceptance@transcript-command@ac-unknown-values-reported`.

### The output is compact in an agent's context, and leaves tool traffic out unless asked (K7) `##compact-output`

Weighed. Met by `thread@transcript-command@transcript-query-verbs`; cause P5 names what remains.

### A session started from a worktree of the project is found (K8) `##worktree-sessions-found`

Weighed. Met by `thread@transcript-command@transcript-directory-resolution` for a worktree under the
project's root; cause P4 names a worktree elsewhere.

## Threads

### The command is a command of the core, `transcript`, offered by every binary `##transcript-command-in-core`

Proposed by the session, round 1. Approved. Arguments: `arg@transcript-command@arg-1`,
`arg@transcript-command@arg-6`. Shape: "Decided design", its first subsection. Harvest: a head of
`register@core@design` if an entry test passes; the member `transcript` of
`design@core@the-core-cli-is-a-library-module`. The owner, round 2: "transcript-command-in-core
agreed."

### A session is a run of chain lines cut where the `sessionId` changes or a root that is no compaction opens, with its subagent files attached `##transcript-session-split`

Proposed by the session, round 1. Approved. Arguments: `arg@transcript-command@arg-4`,
`arg@transcript-command@arg-7`, `arg@transcript-command@arg-12`, `arg@transcript-command@arg-13`.
Shape: "Decided design", "Sessions". Harvest: a head of `register@core@design`, test 3. The owner,
round 2: "transcript-session-split accepted", and Q1 answered by the test whose files the session
read.

### Three verbs, `list`, `show` and `search`, with filters and a JSON form `##transcript-query-verbs`

Proposed by the session, round 1. Approved. Arguments: `arg@transcript-command@arg-1`,
`arg@transcript-command@arg-8`. Shape: "Decided design", "The verbs". Harvest: a head of
`register@core@design`, test 1; the core README. The owner, round 2: "transcript-query-verbs
accepted".

### The extraction rule moves into the command, with a regression fixture, and the installed text runs the command `##transcript-labels-in-code`

Proposed by the session, round 1; the label owner-command added in round 2, from the owner's Q1 test.
Approved. Arguments: `arg@transcript-command@arg-9`, `arg@transcript-command@arg-14`. Shape:
"Decided design", "Labels", and the mapping table "Line to label". Harvest: a head of
`register@core@design`, test 3; the rewrite of `design@agent-skills@transcript-reviewer-agent`.
The owner, round 2: "transcript-labels-in-code accepted".

### Unknown lines are tolerated, and every run reports what it did not recognise `##transcript-format-tolerated-and-reported`

Proposed by the session, round 1. Approved. Arguments: `arg@transcript-command@arg-2`,
`arg@transcript-command@arg-3`, `arg@transcript-command@arg-10`, `arg@transcript-command@arg-16`.
Shape: "Decided design", "The header". Harvest: a head of `register@core@design`, test 3. The
owner, round 2: "transcript-format-tolerated-and-reported: agreed".

### A project's sessions are found by the `cwd` field of their lines `##transcript-directory-resolution`

Proposed by the session, round 1. Approved. Argument: `arg@transcript-command@arg-11`. Shape:
"Decided design", "Which files are read". Harvest: a head of `register@core@design`, test 3. The
owner, round 2: "transcript-directory-resolution accepted".

### A line the command does not recognise reaches this repository through the command's header, the transcript reviewer's report and the retrospective's checker item `##unrecognised-entries-reach-upstream`

Proposed by the owner, round 2, as shape A, a standing question of the retrospective; shape B
proposed by the session, round 2. Closed on B by the owner's word at a fork. Arguments:
`arg@transcript-command@arg-17`, `arg@transcript-command@arg-18`. Shape: "Decided design", "The
header", and step 4. Harvest: judged for a head of `register@agent-skills@design` by test 2, two
sites, and test 4. The owner, round 3: "unrecognised-entries-reach-upstream: agreed on B".

## Arguments

### The request: a hand extraction is much work and error-prone, and the command should serve every task that consults a transcript `##arg-1`

Round 1, the owner. Bears on `thread@transcript-command@transcript-command-in-core` and
`thread@transcript-command@transcript-query-verbs`. "It usually writes a script for that, however
this is a lot of work, and prone to mistakes." "The command should not just handle extracting the
content, but also make any task that requires consulting the transcript simpler to carry, including
surveys of past behaviors for example." "In particular, one pain point is that when I clear a
session to start a new one, the same transcript file is reused by the harness, which often confuses
the transcript reviewer."

### The format is documented as internal, and no interface the vendor recommends reads past sessions whole in Rust `##arg-2`

Round 1, the session, read at https://code.claude.com/docs/en/sessions: "The entry format is
internal to Claude Code and changes between versions, so scripts that parse these files directly can
break on any release. To build on session data, use `/export` or the script interfaces instead."
The Agent SDK's `getSessionMessages` reads the same files, in TypeScript or Python, and "returns the
post-compaction chain". Third-party parsers, such as claude-code-log in its issue 116, broke on new
types. Bears on `thread@transcript-command@transcript-format-tolerated-and-reported`.

### Over five patch versions, nothing the extraction relies on was removed or renamed, and types were added `##arg-3`

Round 1, the session, measured on this machine: 569 files, 181,280 lines, 2.1.306 to 2.1.310; each
new type or key first appeared with its feature. One window proves nothing beyond itself. Bears on
`thread@transcript-command@transcript-format-tolerated-and-reported`.

### The premise that `/clear` reuses the file was unverified, and a resumed session writes to its old file under its old id `##arg-4`

Round 1, the session. The docs, the hook source `clear` and the changelog pointed to a new file; no
`/clear` had run on this machine. A resume appends to the original file, while the new process's id
shows only in `session_id`. Bears on `thread@transcript-command@transcript-session-split`.

### The harness deletes transcripts after 30 days by default `##arg-5`

Round 1, the session, read on the same page: `cleanupPeriodDays`, default 30 days; the setting is
not set on this machine. A survey reaches 30 days back at most, and a branch older than that loses
its early transcripts before its review. Put to the owner as Q2, answered by
`arg@transcript-command@arg-15`.

### The core is the home, since every project the owner works in needs the command `##arg-6`

Round 1, the session. A command of tools/xtask is never published, so thaum would have no copy; a
second crate or binary is what `design@core@single-crate` argues against; a script installed into
each project needs Python and drifts per copy. Bears on
`thread@transcript-command@transcript-command-in-core`.

### A mechanical split holds whether a clear reuses the file or not `##arg-7`

Round 1, the session. One file per session is wrong if a file can hold two; the rule cuts at a
change of `sessionId` and at a root that is no compaction, so it holds either way. Bears on
`thread@transcript-command@transcript-session-split`.

### `gitBranch` on every line lets the command compute the sessions of a branch `##arg-8`

Round 1, the session: the review before a merge needs "every session that worked on the branch",
and today a session searches for opening words. Bears on
`thread@transcript-command@transcript-query-verbs`.

### The rule kept in the agent beside a command would have two homes `##arg-9`

Round 1, the session. A rule in text is applied by a script each session writes; in code it is one
implementation, and the substring failure becomes a fixture a gate runs. A shared fragment still
leaves every session to write the script. Bears on `thread@transcript-command@transcript-labels-in-code`.

### A strict parser fails on every release, and a silent tolerant one hides a change `##arg-10`

Round 1, the session. Every version of the window added a type; ccusage skips malformed lines in
silence. Bears on `thread@transcript-command@transcript-format-tolerated-and-reported`.

### The directory name is an internal rule with a truncation, a hash and an override `##arg-11`

Round 1, the session, read on the same page: "`<project>` is your working directory path with
non-alphanumeric characters replaced by `-`", truncated at 200 characters with "a hash of the full
path", and `CLAUDE_CODE_PROJECT_DIR_NAME` overrides it. A worktree's sessions sit under another
directory. Matching `cwd` needs none of it. Bears on
`thread@transcript-command@transcript-directory-resolution`.

### The owner's test of `/clear` `##arg-12`

Round 2, the owner: "I asked "hey, I'm testing something about the harness, can you tell me where
this session's transcript is stored on my machine ?" twice in another session (from same dir as
here), once at the start and once after a clear. You can look at the results." Bears on
`thread@transcript-command@transcript-session-split`.

### `/clear` writes a new file, and the process keeps the earlier id in its paths `##arg-13`

Round 2, the session, measured on the two files of that test: the question before the clear is in a
file under one `sessionId`, the one after it in a new file under another, whose assistant lines
carry the earlier id in `session_id`. A session that derives its transcript from its identifier
lands on the earlier file. Bears on `thread@transcript-command@transcript-session-split`.

### The clear itself is written, and its `local_command` line carries it by fields `##arg-14`

Round 2, the session: the new file holds the command as a `user` line with neither `isMeta` nor
`origin`, and a `system` `local_command` line whose `commandRun` names it. A label for it by those
fields keeps `criterion@transcript-command@labels-by-fields`. Bears on
`thread@transcript-command@transcript-labels-in-code`.

### No longer retention is needed `##arg-15`

Round 2, the owner: "no need for transcript to be kept longer, this use case only happens
occasionally when I have doubts about agent behaviors, and recent usage is usually enough (older than
last release of klarch would have little meaning most of the time)".

### The shape is stable enough to rely on, with everything unrecognised reported `##arg-16`

Round 2, the owner: "Since the transcript reviewer existence, I do not think anything has been
removed/renamed, maybe things were added though. Overall, I doubt the format will change much, this
would be inconvenient even for antrhopic itself after all. I believe there are other third party
tools that parse it too. Overall, I believe we can rely on the general shape, as long as we report
anything not recognized so no one misses one such item." Bears on
`thread@transcript-command@transcript-format-tolerated-and-reported`.

### The retrospective should ask whether anything unrecognised was met `##arg-17`

Round 2, the owner: "Also, the retrospective skill should probably ask consumers about whether
anything unrecogniwed was met, so we can learn of it and act on it." Shape A of
`thread@transcript-command@unrecognised-entries-reach-upstream`.

### A standing question costs every retrospective; the header delivers it where it happens `##arg-18`

Round 2, the session. A standing question is loaded and answered in every retrospective of every
project, most of which never ran the command, the size cost of `primer@where-knowledge-goes`. The
header names the gap as a checker defect, the reviewer lists a non-zero count under "Met outside
the task", and the retrospective's item "4. The checker: a defect; a blind spot", in
`skill@knowledge-architect-retrospective@what-it-examines`, carries it to the workflow file. Its
cost: it relies on the session carrying the header line to its retrospective. Neither shape fails a
constraint, so the choice was the owner's. Bears on
`thread@transcript-command@unrecognised-entries-reach-upstream`.

## New names, in one place

| name | file | what it is |
| --- | --- | --- |
| the reader module | `planned@core@src/transcript.rs` | files, sessions, labels and the inventory; `pub(crate)` |
| the command module | `planned@core@src/cli/transcript.rs` | the three verbs and their output |
| the tests | `planned@core@tests/transcript.rs` | the binary's tests over the fixtures |
| the fixtures | `planned@core@tests/transcripts/` | one configuration directory per case, each a `projects/` tree read through `CLAUDE_CONFIG_DIR` |
| `Command::Transcript(TranscriptArgs)` | `path@core@src/cli/mod.rs` | the command's variant, beside `Command::Worklist` |
| `TranscriptVerb::{List, Show, Search}` | `planned@core@src/cli/transcript.rs` | the verb, a clap subcommand of `TranscriptArgs` |
| `serde_json = "1"` | `path@core@Cargo.toml` | a new dependency |

Every other name the implementation needs is its own choice.

## Decided design

### `transcript` is a command of the core

A variant of `Command`, dispatched by `run`, so every binary that flattens the core's commands
offers it, per `design@core@the-core-cli-is-a-library-module`. It runs inside a project, after the
version refusal, and reads the project's root from the manifest. Nothing of it is public in the
library. The nearest rival, a command of tools/xtask, lost because it is never published and thaum
needs the command, `arg@transcript-command@arg-6`.

### Which files are read

The transcript directory is `$CLAUDE_CONFIG_DIR/projects/`, or `~/.claude/projects/` where the
variable is unset. A top-level `.jsonl` file of any project directory belongs to the project when
the `cwd` of its first chain line is the project's root or a path under it, compared after
canonicalisation of the root. `--all-projects` takes every file. A subagent file is read with its
parent file. The tests point `CLAUDE_CONFIG_DIR` at a fixture directory and never read the
machine's own.

The nearest rival, recomputing the directory's name from the root, lost on
`arg@transcript-command@arg-11`: the rule is internal, its truncation hash is undocumented, an
environment variable overrides it, and a worktree's sessions sit elsewhere.

### Sessions

The chain lines of a file are read in file order. A new session opens at the first chain line, and
at a chain line whose `sessionId` differs from the previous chain line's, or whose `parentUuid` is
null and which is no `compact_boundary` line. A compaction and a resumed process stay inside their
session. A subagent file is attached to the session that holds the `tool_use` whose id its
`meta.json`'s `toolUseId` names; one with no `toolUseId`, a workflow agent, to the session whose
`sessionId` its lines carry; a nested agent, whose spawning call is in another subagent's file, to
that file's session. A session's branches are the `gitBranch` values of its lines, in the order
first seen.

The nearest rival, one file per session, lost on `arg@transcript-command@arg-7`.

### Labels

Each kept line takes one label, by the mapping table "Line to label". The labels a reader treats as
the owner's words are `owner`, `owner-answer`, `owner-queued` and `owner-command`; `injected`
carries its kind; `agent` is the session's own text; `tool` is a tool call or its result, kept only
under `--tools`. A `user` line holding only `tool_result` items is a tool result, never a message.
A line the table maps to `unlabelled` is printed whole with that label and counted in the header.

In a session none of whose `user` lines carries `isMeta`, `isCompactSummary` or `origin`, a log
older than those fields, the fallback of today's rule applies to the lines the table would leave
`unlabelled`: a text that opens with a harness tag, `<` then a lowercase word then `>`, is
`injected`, any other `owner`; and the header says the fallback ran.

The nearest rival, the rule kept in the agent's text with the command as a helper, lost on
`arg@transcript-command@arg-9` and `criterion@transcript-command@one-home-for-the-rule`.

### The verbs

An illustration of the shape, not of the exact output:

```text
$ <command> transcript list [--branch <b>] [--containing <text>] [--since <date>] [--all-projects]
session   started (UTC)     ended  branches                 owner  compactions  subagents  first owner message
dde8ed67  2026-10-25 15:36  16:07  main, <branch>             3      0            12         Right now, the transcript reviewer needs to…

$ <command> transcript show <session> [--from <text>] [--subagents] [--tools] [--full]
[L8    owner          15:36] Right now, the transcript reviewer needs to assemble…
[L39   injected skill]       Base directory for this skill: … (412 lines omitted; --full prints them)
[L173  injected peer]        I graded all 123 entries…
[L475  agent          15:59] ## Round 1: transcript command …

$ <command> transcript search <regex> [--role <label>]… [--tool <name>] [--branch <b>] [--since <date>] [--all-projects] [--subagents]
dde8ed67 L475 agent  … the matching line, cut to 200 characters …
```

- `list` prints one row per session of the project, oldest first. `--branch` keeps a session one of
  whose lines carries the branch; `--containing` keeps a session holding a line labelled `owner` or
  `owner-queued` whose text contains the text; `--since` keeps a session that ended on or after the
  date.
- `show` takes a handle or any unique prefix of a `sessionId`. It prints the kept lines in file
  order, each with its line number and label; an injected text above 40 lines prints its first 5
  unless `--full`; an owner's or the agent's text is never cut. `--from` starts at the first line
  labelled `owner` or `owner-queued` whose text contains the text. `--subagents` prints each
  attached subagent file after the session, under a line naming its agent type.
- `search` prints one line per kept line whose text matches the regex, in the `regex` crate's
  syntax; a tool call's text is its name and its input as compact JSON.
- `--json`, on every verb, prints one JSON object per row or line instead, with the header as
  objects of their own. It is declared no contract, beside the text.
- Exit codes, per `design@core@exit-code-ladder`: 0 when a verb printed at least one row or line;
  1 when `list` or `search` found none, when `show`'s handle names no session or more than one,
  each candidate named, or when `--from` matches no line; 2 when the transcript directory is absent
  or unreadable, a file cannot be read, or the regex does not parse.

The nearest rival, one verb that prints a whole extraction, leaves a survey and the search for a
branch's sessions to scripts again, against `arg@transcript-command@arg-1`.

### The header

Every verb prints, before its rows, lines that open with `# `: the number of files read, the harness
versions seen, and, where any is non-zero:

- each top-level `type`, each `system` subtype, each `attachment.type` and each `origin.kind` value
  absent from the sets the code knows, with its count and its first file and line;
- the count of lines labelled `unlabelled`;
- each line that is no JSON, by file and line;
- that the fallback of "Labels" ran.

Where any of the first three is non-zero, the header ends with the line: "each is a gap of this
command: report it as a finding on the checker in the retrospective". The known sets are the
values listed under "What the work is", each marked in the code as used or ignored on purpose. With
`--json`, the header is the first objects. Its exit code is unchanged by the inventory.

The nearest rivals: a strict parser, which every version of the window would have failed, and a
silent tolerant one, which `criterion@transcript-command@nothing-unrecognised-silent` rules out,
`arg@transcript-command@arg-10`; for the route upstream, a standing question of the retrospective,
which the owner set aside for this shape, `arg@transcript-command@arg-18`.

## Mapping tables

### Line to label

| line | condition | label |
| --- | --- | --- |
| `user` | holds only `tool_result` items, and its `toolUseResult` holds `questions` and `answers` | `owner-answer`: the questions, their options, the answers and any `annotations` |
| `user` | holds only `tool_result` items, otherwise | `tool`, kept under `--tools` |
| `user` | `isCompactSummary` true | `injected compact-summary` |
| `user` | `isMeta` true | `injected` with the kind of its `origin.kind` where present, else `skill` when its text opens "Base directory for this skill:", else `meta` |
| `user` | `origin.kind` `human` | `owner` |
| `user` | `origin.kind` `peer`, `task-notification` or `coordinator` | `injected` with that kind |
| `user` | `origin.kind` any other value | `injected unknown`, counted in the header |
| `user` | the parent of a `system` `local_command` line | consumed by that line, not printed |
| `user` | none of the above | `unlabelled`, counted in the header; the fallback of "Labels" where it runs |
| `assistant` | each `text` item | `agent` |
| `assistant` | each `tool_use` item | `tool`, kept under `--tools` |
| `assistant` | each `thinking` item | dropped |
| `attachment` | `type` `queued_command`, `attachment.origin.kind` `human` | `owner-queued`, its text `attachment.prompt` |
| `attachment` | `type` `queued_command`, any other `attachment.origin.kind`, or `attachment.isMeta` true | `injected` with that kind |
| `attachment` | any other known `type` | dropped |
| `attachment` | an unknown `type` | dropped, counted in the header |
| `system` | subtype `local_command` with a `commandRun` | `owner-command`: `/<command> <args>` |
| `system` | subtype `compact_boundary` | `injected compact-boundary`, one line naming the trigger |
| `system` | any other known subtype | dropped |
| `system` | an unknown subtype | dropped, counted in the header |
| a known metadata type | | dropped |
| an unknown top-level `type` | | dropped, counted in the header |
| no JSON | | counted in the header with its file and line |

The `skill` kind reads a text prefix for a line its fields already mark injected, so it chooses a
kind and never a side: it keeps `criterion@transcript-command@labels-by-fields`.

## Losing alternatives

- **A command of tools/xtask**, against `thread@transcript-command@transcript-command-in-core`:
  never published, so thaum has no copy, `arg@transcript-command@arg-6`.
- **A second crate or binary**, against the same thread: a second package to publish and pin, which
  `design@core@single-crate` argues against.
- **A script installed into each project**, against the same thread: needs Python, and the copies
  drift.
- **One file per session**, against `thread@transcript-command@transcript-session-split`: wrong for
  a file holding two sessions, `arg@transcript-command@arg-7`.
- **The rule kept in the agent, with the command as a helper**, against
  `thread@transcript-command@transcript-labels-in-code`: two homes for one rule, `arg@transcript-command@arg-9`.
- **The rule as a shared fragment**, against the same thread: every session still writes the
  script.
- **A strict parser**, against `thread@transcript-command@transcript-format-tolerated-and-reported`:
  fails on every release that adds a type, `arg@transcript-command@arg-10`.
- **A silent tolerant parser**, against the same thread: hides a change,
  `criterion@transcript-command@nothing-unrecognised-silent`.
- **Recomputing the directory's name**, against `thread@transcript-command@transcript-directory-resolution`:
  `arg@transcript-command@arg-11`.
- **A standing question of the retrospective**, shape A, against
  `thread@transcript-command@unrecognised-entries-reach-upstream`: the owner chose B,
  `arg@transcript-command@arg-18`.

## Readings

The work reads no external specification the project implements. The harness's behaviour it relies
on is documented by Anthropic as internal, `arg@transcript-command@arg-2`, and measured, under "What
the work is"; each measurement the decided design rests on goes into the head that harvests it.

## Premortem

Assume the design shipped and failed.

- **P1**: a harness release changes a value the labels rely on, such as `origin.kind` `human`
  becoming `user`; the owner's lines are labelled injected while the count of `unlabelled` lines
  stays 0. Stresses `thread@transcript-command@transcript-labels-in-code` and
  `thread@transcript-command@transcript-format-tolerated-and-reported`. Becomes an acceptance
  criterion on the owner's word, AC1: `acceptance@transcript-command@ac-unknown-values-reported`.
- **P2**: a session after `/clear` looks for its transcript by the identifier in its environment and
  reviews the earlier session. Stresses `thread@transcript-command@transcript-session-split` and
  `thread@transcript-command@transcript-query-verbs`. Becomes AC2:
  `acceptance@transcript-command@ac-cleared-session-found`; the installed text keeps "never by a
  session identifier", with this reason.
- **P3**: an owner's message that opens with a harness tag, such as pasted text beginning
  `<system-reminder>`, is labelled injected: the founding failure. Stresses
  `thread@transcript-command@transcript-labels-in-code`. Becomes AC3:
  `acceptance@transcript-command@ac-owner-tag-text-stays-owner`.
- **P4**: a session started in a worktree outside the project's root is missed by `list --branch`,
  and its rulings never reach the review. Stresses
  `thread@transcript-command@transcript-directory-resolution`. Named as a cause for the harvest to
  judge as a tripwire; `--all-projects` reaches it.
- **P5**: `show` of a long session floods a reviewer's context, which reads part of it and misses an
  owner's line. Stresses `thread@transcript-command@transcript-query-verbs`. Converted into a clause
  of the design: the installed text has the output written to a file of the scratch directory and
  read whole, and the reviewer's report says what it read. Named for the harvest.
- **P6**: the harness's classifier refuses the command run by a general-purpose subagent. Stresses
  `thread@transcript-command@transcript-command-in-core`. Not silent, since it stops the work: no
  tripwire. The refusal is not reproduced: at this spec's assembly, in harness 2.1.310, a
  general-purpose subagent read this session's transcript, and the issue that recorded the refusal,
  `a-general-purpose-subagent-is-refused-a-file-access`, closed on that test.

## Acceptance criteria

### AC1: a value the command does not know is reported, and its line is not dropped from the count `##ac-unknown-values-reported`

Guards `thread@transcript-command@transcript-format-tolerated-and-reported`. Judged by step 2.
Observable: over a fixture whose lines carry a top-level `type`, an `attachment.type`, a `system`
subtype and an `origin.kind` that the command does not know, the header of `list` and of `show`
names each with its count, and the `user` line with the unknown `origin.kind` is printed by `show`
as `injected unknown`. It fires when any of the four is missing from the header, or the line is
missing from `show`. Response: reopen the thread on its known sets.

### AC2: a cleared session is listed apart, and found by its first owner message `##ac-cleared-session-found`

Guards `thread@transcript-command@transcript-session-split`. Judged by steps 1 and 2. Observable:
over a fixture pair in the shape of the owner's test, a file before the clear and a new file whose
lines carry the earlier id in `session_id` and open with the `/clear` command lines, `list` prints
two sessions; `list --containing "<the second session's first owner words>"` prints the second
alone; `show` on it prints `/clear` labelled `owner-command`. It fires on any other result.
Response: reopen the thread.

### AC3: an owner's message that opens with a harness tag stays the owner's `##ac-owner-tag-text-stays-owner`

Guards `thread@transcript-command@transcript-labels-in-code`. Judged by step 2. Observable: over a
fixture whose `user` line with `origin.kind` `human` holds a text opening `<system-reminder>`, `show`
labels it `owner` and `list --containing` with words from it finds the session. It fires on any
other label. Response: reopen the thread.

## Implementation sequence

### Step 1: the reader and `list`

Builds `planned@core@src/transcript.rs` (the files of a project, the sessions, the subagent
attachment, the known sets and the inventory), `Command::Transcript` with `list` and its filters,
its header and its `--json`, the dependency on `serde_json`, and the fixtures of
`planned@core@tests/transcripts/` the claims need.

Claims:

- The split: a fixture with a compaction, a resumed process and a `sessionId` change lists the
  sessions the rule gives. Shown to fail against a split by file.
- The cleared pair of AC2 lists two sessions, and `--containing` finds the second. Shown to fail by
  keying the search on `session_id`.
- A fixture file whose `cwd` is a worktree under the root is listed, and one whose `cwd` is another
  project is listed only under `--all-projects`.
- The header names each unknown value of AC1 with its count. Shown to fail by removing one value
  from a known set's check.
- The tests set `CLAUDE_CONFIG_DIR`, and none reads the machine's transcripts.

Measure: the wall time of `list --all-projects` over this machine's transcripts, with the count of
files and lines, recorded in the commit message as the baseline later work compares with.

Fails alone on: `list` over the fixtures.

### Step 2: labels and `show`

Builds the mapping table "Line to label", `show` with `--from`, `--subagents`, `--tools`, `--full`
and `--json`, and the fallback.

Claims:

- Each row of the mapping table, over a fixture holding one line per row. Shown to fail by
  inverting a condition.
- AC3's fixture is labelled `owner`. Shown to fail against a rule that tests the text's opening tag
  before `origin.kind`.
- AC1's `user` line prints as `injected unknown`.
- AC2's `/clear` prints as `owner-command`, and its `user` line is not printed separately.
- `--from` on a text no owner line holds exits 1.
- The fallback runs only in a session with no field, and the header says so.

Fails alone on: `show` over the fixtures, where `list` passes.

### Step 3: `search`

Builds `search` with its filters and `--json`.

Claims: `--role` and `--tool` select by label and tool name; a regex that does not parse exits 2;
no match exits 1.

Fails alone on: `search` over the fixtures, where `show` passes.

### Step 4: the installed text, the README and the changelog

Builds:

- `agent@knowledge-architect-transcript-reviewer@extract-transcripts` rewritten: find the sessions
  with `{{command}} transcript list --branch <branch>` and `--containing "<opening words>"`; write
  `{{command}} transcript show <session> --from "<opening words>"` to a file of the scratch directory
  and read it whole; what each label means, the owner's words being the four `owner` labels; an
  `unlabelled` line read and judged, with the judgement stated; a header that names unrecognised
  lines listed under "Met outside the task". The sentence "You may write your extraction script and
  its output to a scratch directory" becomes the command's output. The slug is kept.
- `skill@knowledge-architect-review@review-axes`, its paragraph on finding each transcript file:
  the sessions are listed by the command, never found by a session identifier, since after `/clear`
  the harness keeps the earlier session's identifier in its paths.
- `skill@knowledge-architect-planning@spec-contents`, its paragraph "Assembly from the transcript":
  the subagent runs the command; the rest of the paragraph stands.
- `path@core@README.md`: a section `transcript`, its line in the command summary, its rows in the
  table under "Exit codes", and the retention default and setting.
- `CHANGELOG.md`, `Next release`: a New features entry, surface cli, minor; a Workflow entry,
  surface agent-skills, patch; then `cargo x changelog`.
- `cargo klarch install-agent-skills`, so the installed copies match.

Claims: `grep -rn "extraction script\|opening words of that message" crates/agent-skills/content`
returns nothing; every new sentence of the installed text names the command as `{{command}}`; the
core's test that every shipped reference resolves passes.

Fails alone on: the agent-facing-text review of the step's diff.

## Order rationale

- Step 1 before step 2: the labels read sessions the reader cuts, and a split defect would show as
  a label defect.
- Step 2 before step 3: `search` matches labelled lines.
- Step 3 before step 4: the installed text names only verbs that exist.

## Defaults awaiting the owner

None.

## Harvest

| when | what | where |
| --- | --- | --- |
| the commits of step 4 | the decisions of the six threads of the command, each judged by the entry tests; the member `transcript` of `design@core@the-core-cli-is-a-library-module` | `register@core@design` |
| the same | the rewrite of `design@agent-skills@transcript-reviewer-agent` (the extraction rule is the command's), under `skill@knowledge-architect-decision-recording@reversal-check`; the re-take and the measurement of `design@agent-skills@ledger-from-transcript` | `register@agent-skills@design` |
| the same | `thread@transcript-command@unrecognised-entries-reach-upstream`, judged by tests 2 and 4 | `register@agent-skills@design` |
| the same | each losing alternative, judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | `register@core@rejected`, `register@agent-skills@rejected` |
| the same | causes P4 and P5, judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` | `register@core@tripwire` |
| the landing | the report on AC1, AC2 and AC3 | the commit messages |
| after the review before the merge | the deletion of this spec | a commit of its own |

## Later consequences

- The owner's aim of a retrospective that watches the tripwires of the installed skills,
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, can use
  `search` over a consumer's sessions; nothing of it is built here.
- The status of `--json` follows `issue@knowledge-architect@command-output-is-not-declared-a-contract`
  when that discussion lands.
- A second harness adds its own reader under `issue@core@configuration-for-several-agent-providers`.
