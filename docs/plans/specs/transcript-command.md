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
  waits: D1, under "Defaults awaiting the owner", is built as written, and the branch merges once
  the owner has ruled on it.
- The discussion ran in one session. The message where it begins is the owner's first message: a
  pasted block whose first line is "Right now, the transcript reviewer needs to assemble the
  transcript by hand." A reviewer finds the session's file by those words, never by a session
  identifier.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, its last paragraph, for the work of a spec.

## Names

| name | what it is |
| --- | --- |
| the harness | Claude Code, the only agent harness this command reads, per `criterion@transcript-command@claude-harness-only` |
| transcript directory | `<config>/projects/`, where `<config>` is the value of `CLAUDE_CONFIG_DIR`, or `~/.claude` where it is unset; one subdirectory per project the harness ran in, holding one top-level `<sessionId>.jsonl` file per `sessionId`, which holds one session or more |
| line | one JSON object of a `.jsonl` file, with a `type` field |
| chain line | a line whose `type` is `user`, `assistant`, `attachment` or `system`: these carry `uuid`, `parentUuid`, `sessionId`, `timestamp`, `cwd`, `gitBranch` and `version` |
| session | a run of chain lines of one top-level file, cut by `thread@transcript-command@transcript-session-split`, with the subagent files attached to it |
| handle | the name the command gives a session: the first 8 characters of its `sessionId`, with `#2`, `#3` … for the second and later sessions of one `sessionId`, counted over every file read |
| subagent file | `<project dir>/<sessionId>/subagents/agent-<agentId>.jsonl`, or under `subagents/workflows/wf_<id>/` for a workflow's agent, beside `agent-<agentId>.meta.json`, a JSON object whose fields `agentType` (the agent's type, such as `general-purpose`), `description` and `toolUseId` (the id of the `tool_use` item that dispatched it, absent for a workflow's agent) the command reads |
| a workflow's journal | `subagents/workflows/wf_<id>/journal.jsonl`, whose lines are of the types `launched`, `started` and `result`; the command does not read it |
| label | the role the command gives a kept line, per the mapping table "Line to label" |
| header | the lines a verb prints before its rows, each opening with `# `, per "The header" |
| the extraction rule | today, `agent@knowledge-architect-transcript-reviewer@extract-transcripts`: which lines are kept, and which are the owner's |
| Q1, Q2 | the questions put to the owner in round 1: Q1, whether `/clear` reuses the transcript file, answered by the owner's test of round 2 (`argument@transcript-command@arg-12`); Q2, whether transcripts should be kept longer than the 30 days the harness keeps them (`argument@transcript-command@arg-5`) |
| P1 … P6 | the causes of the premortem, under Premortem |
| AC1, AC2, AC3 | the labels under which the acceptance criteria were put to the owner |
| K1 … K8 | the labels under which the criteria were shown to the owner, each an item under Criteria |
| D1 | the default awaiting the owner, under "Defaults awaiting the owner" |

## What the work is

What exists today at each site the work touches:

- **The core's commands**, the enum `Command` of `path@core@src/cli/mod.rs`, dispatched by `run`
  in the same file: `check`, `show`, `issues`, `tripwires`, `index`, `model`, `anchors`, `commits`,
  `install-agent-skills`, `init`, `new-component`, `worklist`. Every variant's arguments are a
  public `…Args` struct, since `cli` is a public role module. No command reads a transcript.
- **The core's dependencies**, `path@core@Cargo.toml`: no JSON parser. `serde_json` 1.0.151 is
  already in `path@knowledge-architect@Cargo.lock`, through `tree-sitter` and tools/xtask.
- **The core's README**, `path@core@README.md`: a command summary block; a section per command or
  group of commands; the table under "Exit codes", one row per code, each command a clause in a
  cell.
- **The extraction rule**, `agent@knowledge-architect-transcript-reviewer@extract-transcripts`:
  keep `user` and `assistant` lines and their `text` items, `queued_command` attachments, and the
  owner's answers to a question tool; label each `user` line by `isMeta`, `isCompactSummary` and
  `origin.kind`; fall back to a harness tag at the start of the text only in a log that carries
  neither field. Each session that applies it writes its own script ("You may write your extraction
  script and its output to a scratch directory the brief names").
- **Finding a transcript file**, five sites of the installed text:
  - `skill@knowledge-architect-review@review-axes`, its paragraph "Find each transcript file by the
    message where the work begins, never by a session identifier": search the transcript directory
    for the message's opening words;
  - `skill@knowledge-architect-planning@spec-contents`, its paragraph "Assembly from the
    transcript": a subagent applies the extraction rule, and "each file is found by that message,
    never by a session identifier";
  - `skill@knowledge-architect-planning@plan-reviews`: "name in its brief the transcript files the
    assembly read";
  - `agent@knowledge-architect-transcript-reviewer@extract-transcripts`: "The message where the
    work begins is found by the opening words the brief gives", and "A file named in the brief that
    does not hold that message is reported at once";
  - `agent@knowledge-architect-transcript-reviewer@how-to-report`: "State where you wrote the
    extraction".
- **The re-take of a premise**, `design@agent-skills@ledger-from-transcript`: a `jq` command finds a
  log's compaction boundaries.

What the harness writes, measured over every transcript on this machine at the discussion's
grounding, written by Claude Code 2.1.306 to 2.1.310. The corpus grows and the harness deletes
transcripts after 30 days, so each figure keeps the count that is its claim and names its command:

- the top-level `type` values: `user`, `assistant`, `attachment`, `system`, and the metadata types
  `api-request`, `api-request-blob`, `api-request-shape`, `last-prompt`, `atis-latch`, `mode`,
  `permission-mode`, `ai-title`, `queue-operation`, `pr-link`, `file-history-delta`,
  `file-history-snapshot`, `cost-state`. Re-taken with
  `find ~/.claude/projects -name '*.jsonl' ! -name journal.jsonl -exec jq -r .type {} + | sort | uniq -c`;
- the `system` subtypes: `turn_duration`, `compact_boundary`, `scheduled_task_fire`,
  `informational`, `local_command`; re-taken with the same command on `select(.type=="system") | .subtype`;
- the `origin.kind` values on `user` lines and on `queued_command` attachments: `human`, `peer`,
  `task-notification`, `coordinator`; on `.origin.kind` and on `.attachment.origin.kind`;
- the `attachment.type` values: `total_tokens_reminder`, `deferred_tools_record`, `prompt_snapshot`,
  `environment`, `queued_command`, `date`, `instructions`, `model`, `session_context`,
  `credential_org`, `mcp_instructions_delta`, `remote_session_change`, `skill_listing`, `auto_mode`,
  `silent_turn_reminder`, `deferred_tools_delta`, `agent_listing_delta`, `edited_text_file`,
  `nested_memory`, `command_permissions`, `read_truncation_notice`, `structured_output`,
  `hook_success`, `hook_additional_context`, `file`, `task_status`, `compact_file_reference`,
  `invoked_skills`, `hook_system_message`, `thinking_drop`; on `.attachment.type`;
- the content item types: `text`, `tool_use` and `thinking` on `assistant` lines, `text` and
  `tool_result` on `user` lines; on `select(.type=="user" or .type=="assistant") | .message.content[]?.type`;
- a compaction stays in its file and its `sessionId`, in 7 of 7 compactions: a `system` line of
  subtype `compact_boundary` with `parentUuid` null, a `logicalParentUuid` and
  `compactMetadata.trigger`, then, after up to three `attachment` lines, a `user` line with
  `isCompactSummary` true. Re-taken with the command of `design@agent-skills@ledger-from-transcript`
  and `jq -r 'select(.sessionId) | .sessionId' <file> | sort -u`;
- a resumed session appends to its original file under its original `sessionId`, with no new root,
  in 3 of 3 resumes; the first chain line after the resume has a non-null `parentUuid` in 3 of 3; the
  new process's id shows in a `session_id` field in 2 of the 3;
- `/clear` starts a new file under a new `sessionId`, whose lines carry the earlier id in
  `session_id`. The new file opens with hook attachments, a `user` line with `isMeta` true holding
  `<local-command-caveat>`, then the command as a `user` line `<command-name>/clear</command-name>`
  with neither `isMeta` nor `origin`, then a `system` line of subtype `local_command` whose
  `commandRun` is `{"command": "clear", "args": ""}` and whose `parentUuid` is that `user` line's
  `uuid`, per `argument@transcript-command@arg-13`;
- every subagent file opens with a `user` line holding the dispatcher's brief, with `isSidechain`
  true, `parentUuid` null and neither `isMeta` nor `origin`; its lines carry the parent's
  `sessionId`. Every other `user` line with no field and no `tool_result` in a subagent file is a
  workflow's task text, `[Workflow harness — computed task]`, or, once, the owner's interrupt;
  re-taken with `jq` over `subagents/` on `select(.type=="user" and .isMeta==null and .origin==null)`;
- `user` lines with no field and no `tool_result` in top-level files: the owner's interrupt,
  `[Request interrupted by user]`; the owner's `!` shell command, as `<bash-input>` and
  `<bash-stdout>` texts; the prompt of a session run with `claude -p`, whose `entrypoint` is
  `sdk-cli`; and the `/clear` line above;
- no `uuid` appears in more than one file; file order is parent-before-child but for one
  `scheduled_task_fire` line written one line before its parent;
- a large tool result is stored beside the file, in `<sessionId>/tool-results/<id>.txt`, and the
  line holds a reference to it;
- `gitBranch` is on every chain line, and `cwd` can change inside a file, as when a session works
  in a worktree; the first chain line's `cwd` is the directory the session started in.

Outside the work:

- **Another harness.** `criterion@transcript-command@claude-harness-only`;
  `issue@core@configuration-for-several-agent-providers` holds the general case.
- **Watching the tripwires of the installed skills from consumer sessions**: the owner's later aim,
  in `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`. The
  command is a tool that work can use; `design@agent-skills@tripwire-admission-test` condition 2,
  which keeps a tripwire's evidence out of transcripts, is unchanged by it.
- **The retention of transcripts.** The harness deletes them after `cleanupPeriodDays`, 30 days by
  default. The owner answered Q2 in round 2 that no longer retention is needed; the session's
  default of round 1, that the command's README states the default and the setting, stood
  uncontested.
- **Whether a command's output is a contract**: `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
  This command's text and `--json` output are declared no contract, as `anchors` and `worklist` do.
- **The reader as a library API.** The reader module is `pub(crate)`; the command's argument types
  are public through `cli::Command`, as every command's are.

## What is already decided

The decisions the design rests on and does not argue again:

- `design@core@arguments-parse-through-clap`, `design@core@exit-code-ladder`,
  `design@core@a-failed-parse-is-loud`, `design@core@one-question-one-command`,
  `design@core@single-crate`, `design@core@agents-table`,
  `design@core@installed-binary-version-check` (the command runs inside a project, after the version
  refusal), `design@core@declared-command`, `design@core@api-facade` (the argument types are
  public), `design@core@ne-minimal` (`Command` is non-exhaustive, so a variant adds),
  `design@core@a-foreign-build-is-refused`, `design@core@checker-source-literals-are-data` (the
  tests' literals are data).
- `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@restatement-size-test`,
  `design@agent-skills@context-cost-of-agent-text`, `design@agent-skills@capabilities-not-structure`,
  `design@agent-skills@statement-names-an-act`, `design@agent-skills@behaviour-additions-need-real-use`
  (step 4 adds no instruction about how an agent behaves: it rewrites the existing ones to run the
  command), `design@agent-skills@transcript-review-last-before-merge`,
  `design@agent-skills@plan-records-the-exchange`, `design@agent-skills@synthetic-evidence-not-built`
  (the fixtures test the command, which the head admits).
- `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@changelog-entries`,
  `design@knowledge-architect@the-branch-writes-its-changelog-entries`,
  `design@knowledge-architect@package-include-whitelist` (fixtures under crates/core/tests are not
  shipped), `design@knowledge-architect@version-lockstep`.

The decisions the work rewrites, with every text `cargo klarch show` lists as citing them:

- `design@agent-skills@transcript-reviewer-agent`, its sentence "It is an agent, not a line in a
  skill, because its standard and its extraction rule are fixed, and the rule is learned from a
  failure". The reversal lives where the incumbent lives: the head is rewritten in place to say that
  the agent holds the standard and the command holds the extraction rule, citing the core head that
  the harvest writes for `thread@transcript-command@transcript-labels-in-code`, which keeps the
  failure the rule was learned from. Cited by `design@agent-skills@attribution-replaces-quotation`
  alone, which names the reviewer's paraphrase of the owner, not the extraction rule: unaffected.
- `design@agent-skills@ledger-from-transcript`, its re-take: the `jq` command stays valid, and the
  harvest adds the `compactions` column of `transcript list` and the measurement of 7 of 7
  compactions with its command. Cited by `tripwire@agent-skills@ruling-lost-in-assembly` and
  `design@agent-skills@thread-slug-is-entry-id`, whose premises are unchanged.
- `design@agent-skills@plan-written-in-the-converging-session`, its clause "the transcript the
  harness keeps of it, which that session names exactly": after `/clear` the session's identifier
  names the earlier file, `argument@transcript-command@arg-13`; the harvest rewrites the clause to
  the session finding its transcript with `transcript list`. Cited by
  `design@agent-skills@ledger-from-transcript`, unaffected.
- `design@core@the-core-cli-is-a-library-module`, its list of the core's commands, gains
  `transcript`: a member its argument covers. Cited by `design@core@single-crate`,
  `design@core@exit-code-ladder`, `design@core@arguments-parse-through-clap`,
  `design@agent-skills@xtask-pins-checker`, `path@core@README.md` ("runs every command below",
  which step 4's section keeps true), `path@core@Cargo.toml`, `path@core@src/cli/mod.rs`,
  `path@core@src/cli/create.rs`, `path@core@src/main.rs` and `path@core@tests/binary.rs`: each
  cites the head for the module's shape, which the new member leaves standing.
- `design@core@nothing-of-a-project-is-compiled-in`, its fifth bullet, "The agent harnesses it
  knows, and the installer's namespace … what this tool writes, not what a project holds": the
  harness's transcript layout and the known value sets are compiled in, which the head's argument
  covers, since none of them describes a project; the harvest rewrites the bullet to name what the
  tool reads of a harness beside what it writes. Cited by `path@core@CLAUDE.md`,
  `path@core@README.md`, `path@core@src/cli/create.rs`, the core heads on components, plans,
  version pins and `init`, and an issue of agent-skills; each cites the head for its rule, which
  stands.

The section `agent@knowledge-architect-transcript-reviewer@extract-transcripts` keeps its slug, so
no consumer citation dangles and no Migration entry is owed.

The search of the whole record at the discussion's grounding, seeded with
`goal@knowledge-architect@agents-get-a-complete-workflow`,
`goal@knowledge-architect@structure-and-workflow-work-together`,
`goal@knowledge-architect@any-project-can-adopt-it` and the issue
`a-general-purpose-subagent-is-refused-a-file-access`, since closed, graded 173 entries `bears` or
`may-bear`, each read whole. Three were graded `bears` by one searcher and `may-bear` by another,
and are listed as `bears`. Each entry, with its outcome:

**Bears**, each with its outcome:

- `design@agent-skills@a-head-is-owed-by-an-entry-test`: decides at the harvest which thread earns a head.
- `design@agent-skills@admission-rules-home`: its standard judges step 4's edits of the installed text.
- `design@agent-skills@behaviour-additions-need-real-use`: step 4 adds no instruction on how an agent behaves; P5 keeps the existing ones.
- `design@agent-skills@capabilities-not-structure`: the command is a capability; step 4 adds no structure.
- `design@agent-skills@context-cost-of-agent-text`: step 4 weighs the text it rewrites by it.
- `design@agent-skills@design-home-write-loads-recording`: the harvest loads the decision-recording skill.
- `design@agent-skills@instruction-record-is-minimal`: step 4's rewordings take no head; the harvest rows name the decisions that do.
- `design@agent-skills@ledger-from-transcript`: rewritten: its re-take.
- `design@agent-skills@plan-records-the-exchange`: `show` delivers each message whole and in order.
- `design@agent-skills@plan-written-in-the-converging-session`: rewritten: its clause on naming the transcript.
- `design@agent-skills@premortem-as-watch-points`: graded "may change"; by the reversal check its decision stands, the primer question asked about behaviour with no brief edited; its fact that a session sees no subagent's context is a later consequence.
- `design@agent-skills@restatement-size-test`: step 4 replaces the hand procedure by the command, a pointer no longer than one sentence.
- `design@agent-skills@review-judges-additions`: step 4's diff owes the agent-facing-text review.
- `design@agent-skills@shipped-text-cites-no-entry`: step 4's text names `{{command}}` and no entry.
- `design@agent-skills@statement-names-an-act`: each statement step 4 writes names the act of running the command.
- `design@agent-skills@synthetic-evidence-not-built`: the fixtures test the command, which it admits.
- `design@agent-skills@transcript-review-last-before-merge`: `list --branch` gives every session of the branch.
- `design@agent-skills@transcript-reviewer-agent`: rewritten in place: the extraction rule is the command's.
- `design@core@a-foreign-build-is-refused`: the command runs after the refusal.
- `design@core@agents-table`: the command reads the harness's files under any declared harness.
- `design@core@arguments-parse-through-clap`: the verbs are clap subcommands.
- `design@core@checker-source-literals-are-data`: the tests' literals are data.
- `design@core@declared-command`: step 4 writes `{{command}}`.
- `design@core@exit-code-ladder`: the verbs' exit codes follow it.
- `design@core@installed-binary-version-check`: the command runs after the version refusal.
- `design@core@ne-minimal`: `Command` is non-exhaustive, so the variant adds.
- `design@core@nothing-of-a-project-is-compiled-in`: rewritten: its fifth bullet.
- `design@core@one-question-one-command`: the hand procedure leaves the installed text, K4.
- `design@core@section-homes-carry-slugs`: the slug of `agent@knowledge-architect-transcript-reviewer@extract-transcripts` is kept.
- `design@core@single-crate`: `serde_json` lands in the one crate.
- `design@core@the-core-cli-is-a-library-module`: rewritten: its list gains `transcript`.
- `design@knowledge-architect@changelog-entries`: step 4's entries.
- `design@knowledge-architect@package-include-whitelist`: the fixtures are not shipped.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: step 4 writes them.
- `design@knowledge-architect@version-lockstep`: the installed text and the command ship at one version.
- `design@knowledge-architect@versioning-policy`: the command is a minor addition, the skill change a patch.
- `goal@agent-skills@installed-text-leaves-room-to-judge`: step 4 adds no fixed sequence; an `unlabelled` line is left to judgement.
- `goal@agent-skills@installed-text-works-anywhere`: K5; the installed set is written under the `claude` harness alone, whose transcripts the command reads.
- `goal@agent-skills@no-contradiction-in-workflow`: step 4 changes the three texts together.
- `goal@knowledge-architect@adoption-is-easy`: the README section.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: the command removes a hand script from three activities.
- `goal@knowledge-architect@any-project-can-adopt-it`: K1: claude only, on the owner's word; the command reads no project's layout.
- `goal@knowledge-architect@structure-and-workflow-work-together`: the tool computes another of the workflow's work lists.
- `goal@knowledge-architect@the-owner-decides`: the owner's words are labelled by fields, K2.
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: the header reaches the retrospective, `thread@transcript-command@unrecognised-entries-reach-upstream`.
- `goal@xtask@repeated-tasks-are-automated`: its wording meets the core's command: D1.
- `a-general-purpose-subagent-is-refused-a-file-access`, an issue since closed: closed on its closing test, P6.
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: a later consequence.
- `issue@core@configuration-for-several-agent-providers`: outside the work.
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: the output is declared no contract.

**May-bear, bearing on nothing in this work**: each cites a seed with no bearing on reading a
transcript, or bears on a procedure the branch follows or on another question:

- `design@agent-skills@a-reference-claims-a-revisit`
- `design@agent-skills@a-scratch-directory-per-subagent`
- `design@agent-skills@answer-keeps-its-authority`
- `design@agent-skills@attribution-replaces-quotation`
- `design@agent-skills@audit-axis-runs-a-workflow`
- `design@agent-skills@audit-outcomes`
- `design@agent-skills@bounded-path-in-design`
- `design@agent-skills@checker-model-in-primer`
- `design@agent-skills@content-is-in-the-walk`
- `design@agent-skills@content-mirrors-the-install-layout`
- `design@agent-skills@design-hands-off-to-planning`
- `design@agent-skills@exact-pin`
- `design@agent-skills@existing-heads-on-touch`
- `design@agent-skills@extension-follows-the-ground`
- `design@agent-skills@goals-change-through-goal-setting`
- `design@agent-skills@harness-shell-fails-on-a-pipe`
- `design@agent-skills@in-change-path`
- `design@agent-skills@knowledge-table-home`
- `design@agent-skills@misstated-ruling-rated-by-what-it-changes`
- `design@agent-skills@new-or-reshaped-head-needs-design`
- `design@agent-skills@no-external-handoff`
- `design@agent-skills@no-progress-record`
- `design@agent-skills@no-size-budget`
- `design@agent-skills@no-untested-snippet-is-authority`
- `design@agent-skills@one-decision-per-head`
- `design@agent-skills@one-record-searcher`
- `design@agent-skills@pinned-binary-runs-inside-the-project`
- `design@agent-skills@plain-text-is-no-repair`
- `design@agent-skills@plan-document-leaves-at-landing`
- `design@agent-skills@primer-limit`
- `design@agent-skills@retrospective-destination`
- `design@agent-skills@roadmap-orders-issues`
- `design@agent-skills@shared-text-fragment`
- `design@agent-skills@shipped-text-line-comments`
- `design@agent-skills@standing-argument-in-head`
- `design@agent-skills@thread-slug-is-entry-id`
- `design@agent-skills@title-states-the-rule`
- `design@agent-skills@tripwire-admission-test`
- `design@agent-skills@undesigned-work-is-an-issue`
- `design@agent-skills@version-line-names-the-main-commit`
- `design@agent-skills@workflow-axis-corpus`
- `design@agent-skills@xtask-pins-checker`
- `design@core@a-failed-parse-is-loud`
- `design@core@an-extension-builds-its-own-model`
- `design@core@an-extension-plugs-in-through-phased-hooks`
- `design@core@anchors-listing-command`
- `design@core@api-facade`
- `design@core@binary-name`
- `design@core@clusters-packed-under-a-word-bound`
- `design@core@components-carry-the-same-documents`
- `design@core@declarable-directory-register`
- `design@core@every-path-names-its-anchor`
- `design@core@git-is-asked-through-one-module`
- `design@core@harness-kinds`
- `design@core@harness-kinds-cited-without-anchor`
- `design@core@index-takes-no-dry-run`
- `design@core@init-asks-no-confirmation`
- `design@core@init-command`
- `design@core@owned-namespace-check`
- `design@core@phases-gate-the-report`
- `design@core@reserved-anchors`
- `design@core@retired-slug-reference-reported`
- `design@core@worklist-writes-and-verifies-the-reports`
- `design@knowledge-architect@binary-bundles-workflow`
- `design@knowledge-architect@capability-cost-before-migration`
- `design@knowledge-architect@no-future-breaking-cost-argument`
- `design@knowledge-architect@two-crates`
- `design@knowledge-architect@workflow-findings-stay-here`
- `goal@agent-skills@one-skill-per-activity`
- `goal@core@declared-instructions-are-checked`
- `goal@core@records-reach-their-reader`
- `goal@knowledge-architect@agents-work-without-drift`
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`
- `goal@knowledge-architect@documentation-stays-consistent`
- `goal@knowledge-architect@rust-projects-come-first`
- `goal@knowledge-architect@the-owner-s-word-is-verified`
- `issue@agent-skills@a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise`
- `issue@agent-skills@a-handled-retrospective-file-is-marked-where-it-lies`
- `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`
- `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`
- `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`
- `issue@agent-skills@patching-an-installed-skill`
- `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`
- `issue@agent-skills@the-cold-reader-standard-s-scope-reads-two-ways`
- `issue@agent-skills@the-design-record-auditor-s-return-names-two-verdict-vocabularies`
- `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`
- `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`
- `issue@agent-skills@the-retrospective-counts-no-review-cost`
- `issue@agent-skills@the-void-marker-s-reason-reads-two-ways`
- `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`
- `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule`
- `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`
- `issue@core@check-under-a-sparse-checkout-reports-every-file-outside-the-cone`
- `issue@core@citation-regime-over-any-pinned-text`
- `issue@core@core-heads-bundle-decisions-with-many-citing-sites`
- `issue@core@no-checked-form-cites-a-point-inside-a-section`
- `issue@core@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`
- `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`
- `issue@core@references-are-read-in-markdown-and-rust-only`
- `issue@core@the-tests-read-the-machine-s-git-configuration`
- `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes`
- `issue@knowledge-architect@ci-does-not-run-on-a-draft-and-no-record-says-why`
- `issue@knowledge-architect@the-reduced-mode-outside-rust-is-not-documented`
- `rejected@agent-skills@checker-model-printed-by-binary`
- `rejected@agent-skills@ledger-written-during-discussion`
- `rejected@agent-skills@published-checker-reads-percent-lines`
- `rejected@agent-skills@two-readings-left-to-judgement`
- `rejected@agent-skills@workflow-script-inline-in-skill`
- `rejected@core@check-refusing-a-pipe`
- `rejected@knowledge-architect@workflow-as-marketplace-plugin`
- `tripwire@agent-skills@a-shortcut-decision-earns-a-head`
- `tripwire@agent-skills@agent-text-written-without-the-standard`
- `tripwire@agent-skills@attribution-loses-the-scope`
- `tripwire@agent-skills@head-created-without-deliberation`
- `tripwire@agent-skills@pipefail-status-misread`
- `tripwire@agent-skills@record-answered-questions-reach-the-owner`
- `tripwire@agent-skills@rule-title-wider-than-its-argument`
- `tripwire@agent-skills@ruling-lost-in-assembly`
- `tripwire@agent-skills@ruling-lost-in-change`
- `tripwire@core@checker-source-literals-are-data-purpose`

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

Weighed. Source: Anthropic's documentation, `argument@transcript-command@arg-2`. Met by
`thread@transcript-command@transcript-format-tolerated-and-reported` and
`acceptance@transcript-command@ac-unknown-values-reported`.

### The output is compact in an agent's context, and leaves tool traffic out unless asked (K7) `##compact-output`

Weighed. Met by `thread@transcript-command@transcript-query-verbs`; cause P5 names what remains.

### A session started from a worktree of the project is found (K8) `##worktree-sessions-found`

Weighed. Met by `thread@transcript-command@transcript-directory-resolution` for a worktree under the
project's root; cause P4 names a worktree elsewhere.

## Threads

### The command is a command of the core, `transcript`, offered by every binary `##transcript-command-in-core`

Proposed by the session, round 1. Approved. Arguments: `argument@transcript-command@arg-1`,
`argument@transcript-command@arg-6`. Shape: "Decided design", its first subsection. Harvest: a head
of `register@core@design` if an entry test passes; the member `transcript` of
`design@core@the-core-cli-is-a-library-module`. The owner, round 2: "transcript-command-in-core
agreed." It meets `goal@xtask@repeated-tasks-are-automated` read literally: D1.

### A session is a run of chain lines cut where the `sessionId` changes or a root that is no compaction opens, with its subagent files attached `##transcript-session-split`

Proposed by the session, round 1. Approved. Arguments: `argument@transcript-command@arg-4`,
`argument@transcript-command@arg-7`, `argument@transcript-command@arg-12`,
`argument@transcript-command@arg-13`. Shape: "Decided design", "Sessions". Harvest: a head of
`register@core@design`, test 3. The owner, round 2: "transcript-session-split accepted", and Q1
answered by the test whose files the session read.

### Three verbs, `list`, `show` and `search`, with filters and a JSON form `##transcript-query-verbs`

Proposed by the session, round 1. Approved. Arguments: `argument@transcript-command@arg-1`,
`argument@transcript-command@arg-8`. Shape: "Decided design", "The verbs". Harvest: a head of
`register@core@design`, test 1; the core README. The owner, round 2: "transcript-query-verbs
accepted".

### The extraction rule moves into the command, with a regression fixture, and the installed text runs the command `##transcript-labels-in-code`

Proposed by the session, round 1; the label `owner-command` added in round 2, from the owner's Q1
test. Approved. Arguments: `argument@transcript-command@arg-9`,
`argument@transcript-command@arg-14`. Shape: "Decided design", "Labels", and the mapping table
"Line to label". Harvest: a head of `register@core@design`, test 3, for the mapping and the failure
it was learned from; the rewrite in place of `design@agent-skills@transcript-reviewer-agent`, which
owns where the rule lives. The owner, round 2: "transcript-labels-in-code accepted".

### Unknown lines are tolerated, and every run reports what it did not recognise `##transcript-format-tolerated-and-reported`

Proposed by the session, round 1. Approved. Arguments: `argument@transcript-command@arg-2`,
`argument@transcript-command@arg-3`, `argument@transcript-command@arg-10`,
`argument@transcript-command@arg-16`. Shape: "Decided design", "The header". Harvest: a head of
`register@core@design`, test 3. The owner, round 2: "transcript-format-tolerated-and-reported:
agreed".

### A project's sessions are found by the `cwd` field of their lines `##transcript-directory-resolution`

Proposed by the session, round 1. Approved. Argument: `argument@transcript-command@arg-11`. Shape:
"Decided design", "Which files are read". Harvest: a head of `register@core@design`, test 3. The
owner, round 2: "transcript-directory-resolution accepted".

### A line the command does not recognise reaches this repository through the command's header, the transcript reviewer's report and the retrospective's checker item `##unrecognised-entries-reach-upstream`

Proposed by the owner, round 2, as shape A, a standing question of the retrospective; shape B
proposed by the session, round 2. Closed on B by the owner's word at a fork. Arguments:
`argument@transcript-command@arg-17`, `argument@transcript-command@arg-18`. Shape: "Decided
design", "The header", and step 4. Harvest: judged for a head of `register@agent-skills@design` by
test 2, two sites, and test 4. The owner, round 3: "unrecognised-entries-reach-upstream: agreed on
B".

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

Round 1, the session, measured over every transcript on this machine, versions 2.1.306 to 2.1.310,
with the commands under "What the work is"; each new type or key first appeared with its feature.
One window proves nothing beyond itself. Bears on
`thread@transcript-command@transcript-format-tolerated-and-reported`.

### The premise that `/clear` reuses the file was unverified, and a resumed session writes to its old file under its old id `##arg-4`

Round 1, the session. The docs, the hook source `clear` and the changelog pointed to a new file; no
`/clear` had run on this machine. A resume appends to the original file, while the new process's id
shows only in `session_id`. Bears on `thread@transcript-command@transcript-session-split`.

### The harness deletes transcripts after 30 days by default `##arg-5`

Round 1, the session, read on the same page: `cleanupPeriodDays`, default 30 days; the setting is
not set on this machine. A survey reaches 30 days back at most, and a branch older than that loses
its early transcripts before its review. Put to the owner as Q2, answered by
`argument@transcript-command@arg-15`.

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

```text
`path@core@src/transcript.rs`        the reader: files, sessions, labels, inventory; pub(crate)
`path@core@src/cli/transcript.rs`    the three verbs and their output; TranscriptArgs and
                                         TranscriptVerb, public through cli::Command
`path@core@tests/transcripts/`       fixture templates, one directory per case, each a
                                         `projects/` tree whose `cwd` fields hold {{root}}
`path@core@tests/binary.rs`             the tests: a module of the binary's existing suite
`path@core@src/cli/mod.rs`              Command::Transcript(TranscriptArgs), beside Command::Worklist
`path@core@Cargo.toml`                  serde_json = "1", a new dependency
`path@knowledge-architect@Cargo.lock`   the knowledge-architect entry gains serde_json
```

Every other name the implementation needs is its own choice.

## Decided design

### `transcript` is a command of the core

A variant of `Command`, `Command::Transcript(TranscriptArgs)`, dispatched by `run`, so every binary
that flattens the core's commands offers it, per `design@core@the-core-cli-is-a-library-module`. It
runs inside a project, after the version refusal, and reads the project's root from the manifest;
it reads no model, so it runs whatever the check finds, and under any `[agents] harness`, since the
transcripts are the harness's whether or not the project installs the workflow. The reader is
`pub(crate)`; `TranscriptArgs` and `TranscriptVerb` are public through `cli::Command`, per
`design@core@api-facade`. The nearest rival, a command of tools/xtask, lost because it is never
published and thaum needs the command, `argument@transcript-command@arg-6`.

### Which files are read

The transcript directory is `$CLAUDE_CONFIG_DIR/projects/`, or `~/.claude/projects/` where the
variable is unset. A top-level `.jsonl` file of any project directory belongs to the project when
the `cwd` of its first chain line is the project's root or a path under it, compared after
canonicalisation of both. `--all-projects` takes every top-level file. A subagent file is read with
its parent file; a workflow's journal and the `tool-results/` and `workflows/` directories are not
read. The tests point `CLAUDE_CONFIG_DIR` at a fixture directory and never read the machine's own.

The nearest rival, recomputing the directory's name from the root, lost on
`argument@transcript-command@arg-11`: the rule is internal, its truncation hash is undocumented, an
environment variable overrides it, and a worktree's sessions sit elsewhere.

### Sessions

The chain lines of a top-level file are read in file order. A new session opens at the first chain
line, and at a chain line whose `sessionId` differs from the previous chain line's, or whose
`parentUuid` is null and which is no `compact_boundary` line. Each line is judged on its own, so a
line written before its parent cuts nothing. A compaction and a resumed process stay inside their
session: a resume's first chain line has a parent, measured under "What the work is".

A subagent file is attached to a session of the file whose name is the `sessionId` its lines carry:
where that `sessionId` holds one session, to it; where it holds several, to the one holding the
`tool_use` item whose id the subagent's `toolUseId` names, searched through the top-level file and
the other subagent files; and otherwise to the last. A line whose `uuid` was already read in the
session is not printed again, and is counted in the header. A session's branches are the
`gitBranch` values of its lines, in the order first seen.

The nearest rival, one file per session, lost on `argument@transcript-command@arg-7`.

### Labels

Each kept line takes one label, from the first row of the mapping table "Line to label" whose
condition it meets, in the table's order. The labels a reader treats as the owner's words are
`owner`, `owner-answer`, `owner-queued` and `owner-command`; `injected` carries its kind; `agent` is
the session's own text; `dispatch` is the text a subagent's dispatcher or its workflow wrote; `tool` is a tool call
or its result, kept only under `--tools`. A `user` line holding only `tool_result` items is a tool
result, never a message, except the owner's answer to a question tool, `owner-answer`. A line the
table maps to `unlabelled` is printed whole with that label and counted in the header.

The fallback: in a file none of whose `user` lines carries `isMeta`, `isCompactSummary` or
`origin`, such as a log written before those fields or a session run with `claude -p`, a line the
table would map to `unlabelled` is labelled by its text instead: a text that opens with a harness
tag, `<` then a name of lowercase letters, digits, `-` and `_`, then `>` or a space, is
`injected tag`, and any other text is `owner`. The header names each file the fallback labelled a
line in. The fallback is a top-level file's: a subagent file's lines with no field are its
dispatcher's, which the `dispatch` row places.

The nearest rival, the rule kept in the agent's text with the command as a helper, lost on
`argument@transcript-command@arg-9` and `criterion@transcript-command@one-home-for-the-rule`.

### The verbs

An illustration of the shape, not of the exact output:

```text
$ <command> transcript list [--branch <b>] [--containing <text>] [--since <date>] [--all-projects]
# 28 files; harness 2.1.306 to 2.1.310
session   started (UTC)     ended  branches         owner  compactions  subagents  first owner message
dde8ed67  2026-10-25 15:36  16:07  main, <branch>   3      0            12         Right now, the transcript reviewer needs to…

$ <command> transcript show <session> [--from <text>] [--subagents] [--tools] [--full] [--all-projects]
[L8    owner          15:36] <pasted_content id="58a1"> Right now, the transcript reviewer needs to assemble…
[L39   injected skill]       Base directory for this skill: … (412 lines omitted; --full prints them)
[L173  injected peer]        I graded all 123 entries…
[L475  agent          15:59] ## Round 1: transcript command …

$ <command> transcript search <regex> [--role <label>]… [--tool <name>] [--tools] [--branch <b>] [--since <date>] [--all-projects] [--subagents]
dde8ed67 L475 agent  … the matching line, cut to 200 characters …
```

- `list` prints one row per session, oldest first, with these columns: the handle; the first and
  last `timestamp`, in UTC; the branches; the count of lines labelled `owner` or `owner-queued`; the
  count of `compact_boundary` lines; the count of attached subagent files; and the first `owner` or
  `owner-queued` line's text, cut to 80 characters. `--branch` keeps a session one of whose lines
  carries the branch; `--containing` keeps a session holding a line labelled `owner` or
  `owner-queued` whose text contains the text; `--since <YYYY-MM-DD>` keeps a session whose last
  `timestamp`, read as text, is on or after that date, in UTC.
- `show` takes a handle, or a prefix of a `sessionId` that names one session. An argument equal to
  a handle names that session; a prefix whose `sessionId` holds several sessions, or that several
  `sessionId` values share, names none, and the run lists the candidates. It resolves among the
  project's files, or every file under `--all-projects`. It prints the kept lines in file order,
  each with its line number and label; an injected text above 40 lines prints its first 5 unless
  `--full`; an owner's, the agent's or a dispatch text is never cut. `--from` starts at the first
  line labelled `owner` or `owner-queued` whose text contains the text. `--subagents` prints each
  attached subagent file after the session, under a line naming its `agentType` and `description`,
  each line numbered `<agentId's first 8 characters> L<n>`. A tool result stored in
  `tool-results/` prints as the reference its line holds.
- `search` prints one line per kept line whose text matches the regex, in the `regex` crate's
  syntax, with the handle, the line number and the label. `tool` lines are searched only under
  `--tools`, or under `--tool <name>`, which keeps the tool calls of that name and their results. A
  tool call's text is its name and its input as compact JSON; a tool result's is its text items
  joined, or its string.
- `--json`, on every verb, prints one JSON object per row or line instead, the header first, as
  objects of their own. It is declared no contract, beside the text.
- Exit codes, per `design@core@exit-code-ladder`: 0 when a verb printed at least one row or line,
  header lines not counted; 1 when `list` or `search` found none, when `show`'s argument names no
  session or more than one, the candidates named, or when `--from` matches no line; 2 when the
  transcript directory is absent or unreadable, a file cannot be read, or the regex does not parse.
- The cuts at 80, 40, 5 and 200 are defaults of this spec, the owner's to reset; none is a
  measurement.

The nearest rival, one verb that prints a whole extraction, leaves a survey and the search for a
branch's sessions to scripts again, against `argument@transcript-command@arg-1`.

### The header

Every verb prints, before its rows, lines that open with `# `: the number of files read and the
harness versions seen; and, where any is non-zero:

- each top-level `type`, each `system` subtype, each `attachment.type`, each content item type and
  each `origin.kind` value, on a `user` line or on a `queued_command` attachment, absent from the
  sets the code knows, with its count and its first file and line;
- the count of lines labelled `unlabelled`;
- each line that is no JSON, by file and line;
- the count of lines skipped for a repeated `uuid`;
- that the fallback ran, and in which files.

Where any of the first three is non-zero, the header ends with the line: "each is a gap of this
command: report it as a finding on the checker in the retrospective". The known sets are the values
listed under "What the work is", each marked in the code as used or ignored on purpose. The header
counts the files the verb read: for `list` and `search`, every file of the project or of every
project; for `show`, the session's file, and its subagent files under `--subagents`. With
`--json`, the header is the first objects. Its exit code is unchanged by the inventory.

The nearest rivals: a strict parser, which every version of the window would have failed, and a
silent tolerant one, which `criterion@transcript-command@nothing-unrecognised-silent` rules out,
`argument@transcript-command@arg-10`; for the route upstream, a standing question of the
retrospective, which the owner set aside for this shape, `argument@transcript-command@arg-18`.

## Mapping tables

### Line to label

The first row whose condition a line meets gives its label. Within a line of several content items,
each item is mapped by the rows of its line's type.

| line | condition | label |
| --- | --- | --- |
| `user` | the parent of a `system` `local_command` line with a `commandRun`, by `uuid` over the session, with neither `isMeta`, `isCompactSummary` nor `origin` | consumed by that line, not printed |
| `user` | holds only `tool_result` items, and its `toolUseResult` holds `questions` and `answers` | `owner-answer`: the questions, their options with their descriptions, the answers and any `annotations` |
| `user` | holds only `tool_result` items, otherwise | `tool`, with the name of the call it answers where the run holds it, kept under `--tools` |
| `user` | in a subagent file, with neither `isMeta`, `isCompactSummary` nor `origin` | `dispatch` |
| `user` | `isCompactSummary` true | `injected compact-summary` |
| `user` | `isMeta` true and an `origin.kind` | `injected` with that kind where the code knows it, else `injected unknown`, counted in the header |
| `user` | `isMeta` true, no `origin` | `injected skill` when its text opens "Base directory for this skill:", else `injected meta` |
| `user` | `origin.kind` `human` | `owner` |
| `user` | `origin.kind` `peer`, `task-notification` or `coordinator` | `injected` with that kind |
| `user` | `origin.kind` any other value, or an `origin` with no `kind` | `injected unknown`, counted in the header |
| `user` | none of the above | `unlabelled`, counted in the header; the fallback where it runs |
| `user` item | a content item type other than `text` or `tool_result` | dropped, counted in the header |
| `assistant` item | `text` | `agent` |
| `assistant` item | `tool_use` | `tool`, kept under `--tools` |
| `assistant` item | `thinking` | dropped |
| `assistant` item | any other type | dropped, counted in the header |
| `attachment` | `type` `queued_command`, `attachment.origin.kind` `human` | `owner-queued`, its text `attachment.prompt` |
| `attachment` | `type` `queued_command`, `attachment.origin.kind` `peer`, `task-notification` or `coordinator` | `injected` with that kind |
| `attachment` | `type` `queued_command`, any other `attachment.origin.kind`, or an `attachment.origin` with no `kind` | `injected unknown`, counted in the header |
| `attachment` | `type` `queued_command`, no `attachment.origin` | `injected meta` where `attachment.isMeta` is true, else `unlabelled`, counted in the header; the fallback where it runs |
| `attachment` | any other known `type` | dropped |
| `attachment` | an unknown `type` | dropped, counted in the header |
| `system` | subtype `local_command` with a `commandRun` | `owner-command`: `/<command> <args>` |
| `system` | subtype `local_command` with no `commandRun` | dropped, counted in the header as an unknown shape |
| `system` | subtype `compact_boundary` | `injected compact-boundary`, one line naming `compactMetadata.trigger` |
| `system` | any other known subtype | dropped |
| `system` | an unknown subtype | dropped, counted in the header |
| a known metadata type | | dropped |
| an unknown top-level `type` | | dropped, counted in the header |
| no JSON | | counted in the header with its file and line; a line of whitespace alone is no line |

The `injected skill` kind reads a text prefix for a line its fields already mark injected, so it
chooses a kind and never a side: it keeps `criterion@transcript-command@labels-by-fields`.

## Losing alternatives

- **A command of tools/xtask**, against `thread@transcript-command@transcript-command-in-core`:
  never published, so thaum has no copy, `argument@transcript-command@arg-6`.
- **A second crate or binary**, against the same thread: a second package to publish and pin, which
  `design@core@single-crate` argues against.
- **A script installed into each project**, against the same thread: needs Python, and the copies
  drift.
- **One file per session**, against `thread@transcript-command@transcript-session-split`: wrong for
  a file holding two sessions, `argument@transcript-command@arg-7`.
- **The rule kept in the agent, with the command as a helper**, against
  `thread@transcript-command@transcript-labels-in-code`: two homes for one rule,
  `argument@transcript-command@arg-9`.
- **The rule as a shared fragment**, against the same thread: every session still writes the
  script.
- **A strict parser**, against `thread@transcript-command@transcript-format-tolerated-and-reported`:
  fails on every release that adds a type, `argument@transcript-command@arg-10`.
- **A silent tolerant parser**, against the same thread: hides a change,
  `criterion@transcript-command@nothing-unrecognised-silent`.
- **Recomputing the directory's name**, against `thread@transcript-command@transcript-directory-resolution`:
  `argument@transcript-command@arg-11`.
- **A standing question of the retrospective**, shape A, against
  `thread@transcript-command@unrecognised-entries-reach-upstream`: the owner chose B,
  `argument@transcript-command@arg-18`.

## Readings

The work reads no external specification the project implements. The harness's behaviour it relies
on is documented by Anthropic as internal, `argument@transcript-command@arg-2`, and measured, under
"What the work is"; each measurement the decided design rests on goes into the head that harvests
it, with its command.

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
- **P3**: an owner's message that opens with a harness tag, such as pasted text, is labelled
  injected: the founding failure. This discussion's first message is one, opening with
  `<pasted_content id="58a1">`. Stresses `thread@transcript-command@transcript-labels-in-code`.
  Becomes AC3: `acceptance@transcript-command@ac-owner-tag-text-stays-owner`.
- **P4**: a session started in a worktree outside the project's root is missed by `list --branch`,
  and its rulings never reach the review. Stresses
  `thread@transcript-command@transcript-directory-resolution`. Named as a cause for the harvest to
  judge as a tripwire of `register@core@tripwire`; `--all-projects` reaches it.
- **P5**: `show` of a long session floods a reviewer's context, which reads part of it and misses an
  owner's line. Stresses `thread@transcript-command@transcript-query-verbs`. It survives into the
  existing instructions that step 4 keeps, the output written to a file of the scratch directory and
  the extraction read in full; no instruction is added, per
  `design@agent-skills@behaviour-additions-need-real-use`. Named as a cause for the harvest to judge
  as a tripwire of `register@agent-skills@tripwire`.
- **P6**: the harness's classifier refuses the command run by a general-purpose subagent. Stresses
  `thread@transcript-command@transcript-command-in-core`. Not silent, since it stops the work: no
  tripwire. Untested for the command, which does not exist yet. The refusal of a file read that the
  issue `a-general-purpose-subagent-is-refused-a-file-access` recorded is not reproduced: at this
  spec's assembly, in harness 2.1.310, a general-purpose subagent read this session's transcript,
  and the issue closed on that test.

## Acceptance criteria

### AC1: a value the command does not know is reported, and its line is not dropped from the count `##ac-unknown-values-reported`

Guards `thread@transcript-command@transcript-format-tolerated-and-reported`. Judged by step 1.
Observable: over a fixture whose lines carry a top-level `type`, an `attachment.type`, a `system`
subtype, a content item type and an `origin.kind` that the command does not know, the header of
`show` names each with its count, and the `user` line with the unknown `origin.kind` is printed as
`injected unknown`. It fires when any of the five is missing from the header, or the line is missing
from `show`. Response: reopen the thread on its known sets.

### AC2: a cleared session is listed apart, and found by its first owner message `##ac-cleared-session-found`

Guards `thread@transcript-command@transcript-session-split`. Judged by steps 1 and 2. Observable:
over a fixture pair in the shape of the owner's test, a file before the clear and a new file whose
lines carry the earlier id in `session_id` and that holds, after its opening hook and caveat lines,
the `/clear` command lines, `show` on the second prints `/clear` labelled `owner-command` (step 1);
`list` prints two sessions, and `list --containing "<the second session's first owner words>"`
prints the second alone (step 2). It fires on any other result. Response: reopen the thread.

### AC3: an owner's message that opens with a harness tag stays the owner's `##ac-owner-tag-text-stays-owner`

Guards `thread@transcript-command@transcript-labels-in-code`. Judged by steps 1 and 2. Observable:
over a fixture whose `user` line with `origin.kind` `human` holds a text opening `<system-reminder>`,
`show` labels it `owner` (step 1) and `list --containing` with words from it finds the session (step
2). It fires on any other label. Response: reopen the thread.

## Implementation sequence

### Step 1: the reader, the labels and `show`

Builds `path@core@src/transcript.rs` (the files of a project, the sessions, the subagent
attachment, the known sets, the mapping table, the fallback and the inventory), `Command::Transcript`
with `show` and its options, its header and its `--json`, the dependency on `serde_json` with its
lock entry, and the fixture templates of `path@core@tests/transcripts/` the claims need.

The tests run the binary in the mock `core` in place, through the helpers of
`path@core@tests/binary.rs`, with `CLAUDE_CONFIG_DIR` set to a temporary directory into which the
test writes a fixture template with `{{root}}` replaced by the mock's root: the command reads
nothing of the project but its root, so no copy is needed. The mock gains no file. No test reads the machine's transcripts.

Claims:

- The split: a fixture with a compaction, a resumed process and a `sessionId` change gives the
  sessions the rule gives, each shown by `show`. Shown to fail against a split by file.
- Each row of the mapping table, over a fixture holding one line per row. Shown to fail by
  inverting a condition, and by swapping two rows the order separates.
- AC1: the header names the five unknown values with their counts, and the `user` line prints as
  `injected unknown`. Shown to fail by removing one value's check.
- AC3: the fixture's line is labelled `owner`. Shown to fail against a rule that tests the text's
  opening tag before `origin.kind`.
- AC2's first half: `/clear` prints as `owner-command`, and its `user` line is not printed
  separately.
- A subagent file prints under `--subagents`, its brief labelled `dispatch`, with no count of
  `unlabelled` lines.
- A repeated `uuid` prints once and is counted.
- The fallback runs only in a file with no field, and the header says so.
- `--from` on a text no owner line holds exits 1; a fixture whose `cwd` is a worktree under the
  root is shown, and one whose `cwd` is another project only under `--all-projects`.

Measure: the wall time of `show` on the longest session of this machine's transcripts, with its
line count, recorded in the commit message as the baseline later work compares with.

Fails alone on: `show` over the fixtures.

### Step 2: `list`

Builds `list` with its columns, its filters and `--json`.

Claims: AC2's second half and AC3's second half; `--branch` and `--since` select as stated; no
session exits 1.

Fails alone on: `list` over the fixtures, where `show` passes.

### Step 3: `search`

Builds `search` with its filters and `--json`.

Claims: `--role`, `--tool` and `--tools` select as stated; a regex that does not parse exits 2; no
match exits 1.

Fails alone on: `search` over the fixtures, where `show` passes.

### Step 4: the installed text, the README and the changelog

Builds, writing each source under `path@agent-skills@content/` as `path@agent-skills@CLAUDE.md`,
"Editing an installed skill or agent", says:

- `agent@knowledge-architect-transcript-reviewer@extract-transcripts` rewritten: find the sessions
  with `{{command}} transcript list --branch <branch>` and `--containing "<opening words>"`; write
  `{{command}} transcript show <session> --from "<opening words>" --subagents` to a file of the
  scratch directory and read it in full; what each label means, the owner's words being the four
  `owner` labels; an `unlabelled` line read and judged, with the judgement stated; a header that
  names unrecognised lines listed under "Met outside the task". The sentence "You may write your
  extraction script and its output to a scratch directory" becomes the command's output; "The
  message where the work begins is found by the opening words the brief gives" and the file named
  in the brief that does not hold the message become a session named by the brief, checked with
  `--containing`. The slug is kept.
- `agent@knowledge-architect-transcript-reviewer@how-to-report`: "State where you wrote the
  extraction" names the command's output file.
- `skill@knowledge-architect-review@review-axes`, its paragraph on finding each transcript file:
  the sessions are listed by the command, never found by a session identifier, since after `/clear`
  the harness keeps the earlier session's identifier in its paths.
- `skill@knowledge-architect-planning@spec-contents`, its paragraph "Assembly from the transcript":
  the subagent runs the command, and each session is found with `transcript list --containing`;
  and `skill@knowledge-architect-planning@plan-reviews`: the brief names the sessions the assembly
  read, by their opening words.
- `path@core@README.md`: a section `transcript`, its lines in the command summary, its clauses in
  the three cells of the table under "Exit codes", and the retention default and setting.
- `CHANGELOG.md`, `Next release`: a New features entry, surface cli, minor, which names the public
  argument types the library gains; a Workflow entry, surface agent-skills, patch; then
  `cargo x changelog`.
- `cargo klarch install-agent-skills`, so the installed copies match.

Claims: `grep -rn "extraction script\|opening words of that message\|opening words the brief gives" crates/agent-skills/content`
returns nothing; every new sentence of the installed text names the command as `{{command}}`; the
core's test `the_shipped_set_cites_only_sections_it_defines`, in `path@core@src/agents.rs`, passes.

Fails alone on: the agent-facing-text review of the step's diff.

## Order rationale

- Step 1 before step 2: `list` counts and filters by labels that step 1 builds, and a defect of the
  split shows first in `show` of one session.
- Step 2 before step 3: `search` takes `list`'s filters.
- Step 3 before step 4: the installed text names only verbs that exist.

## Defaults awaiting the owner

- **D1**, bearing on `thread@transcript-command@transcript-command-in-core`: the goal
  `goal@xtask@repeated-tasks-are-automated` says "A task that sessions perform in this repository
  more than once, by hand and in the same steps, becomes a command of the tool", and "None exists
  today". The hand extraction is such a task, and the approved thread makes it a command of the
  core, not of tools/xtask. The goal's measure, "no such task is left to be done by hand", is met by
  the core's command. Default: the command is built in the core as approved; the owner rules whether
  the goal's wording changes, for instance to "a command of the tool, or of the checker where every
  project needs it", and its last sentence, which is false at the base of this branch. A goal
  changes only under `skill@knowledge-architect-goal-setting`.

## Harvest

| when | what | where |
| --- | --- | --- |
| the commits of step 4 | the decisions of the six threads `thread@transcript-command@transcript-command-in-core`, `thread@transcript-command@transcript-session-split`, `thread@transcript-command@transcript-query-verbs`, `thread@transcript-command@transcript-labels-in-code` (the mapping, and the failure it was learned from), `thread@transcript-command@transcript-format-tolerated-and-reported` and `thread@transcript-command@transcript-directory-resolution`, each judged by the entry tests, each measurement with its command; the member `transcript` of `design@core@the-core-cli-is-a-library-module`; the fifth bullet of `design@core@nothing-of-a-project-is-compiled-in` | `register@core@design` |
| the same | the rewrite in place of `design@agent-skills@transcript-reviewer-agent` (the extraction rule is the command's), under `skill@knowledge-architect-decision-recording@reversal-check`, citing the core head of the labels; the re-take and the measurement of `design@agent-skills@ledger-from-transcript`; the clause of `design@agent-skills@plan-written-in-the-converging-session` | `register@agent-skills@design` |
| the same | `thread@transcript-command@unrecognised-entries-reach-upstream`, judged by tests 2 and 4; the rule that a transcript is found by the command's listing, never by a session identifier, stated by the review skill, the planning skill and the transcript reviewer, judged by test 2 | `register@agent-skills@design` |
| the same | each losing alternative, judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | `register@core@rejected`, `register@agent-skills@rejected` |
| the same | causes P4 and P5, judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` | `register@core@tripwire` for P4, `register@agent-skills@tripwire` for P5 |
| the landing | the report on AC1, AC2 and AC3, and the outcome of each of P1 to P6 | the commit messages |
| after the review before the merge | the deletion of this spec | a commit of its own |

## Later consequences

- The owner's aim of a retrospective that watches the tripwires of the installed skills,
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, can use
  `search` over a consumer's sessions; nothing of it is built here.
- `design@agent-skills@premortem-as-watch-points` rests on "A session sees a subagent's brief and
  its report, not its context". `show --subagents` lets a later retrospective read a subagent's
  context; whether the retrospective asks it to is later work, on real use.
- The status of `--json` follows `issue@knowledge-architect@command-output-is-not-declared-a-contract`
  when that discussion lands.
- A second harness adds its own reader under `issue@core@configuration-for-several-agent-providers`.
