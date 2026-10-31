# Core design: Session transcripts

## Session transcripts

### The agent harness's session transcripts are read by a command of the core, `transcript`, so every binary offers it `##transcript-command-in-core`

`klarch transcript` reads the transcripts the agent harness writes on disk, for the transcript
reviewer, the assembly of a plan document and any survey of past sessions. It is a variant of the
core's command enum, so every binary that flattens it offers the command, per
`design@core@the-core-cli-is-a-library-module`, and it runs inside a project, after the version
refusal of `design@core@installed-binary-version-check`, whatever the project's
`[agents] harness`: the transcripts are the harness's own, whether or not the project installs the
workflow. The reader is internal; its argument types are public through `cli::Command`, per
`design@core@api-facade`.

It serves `goal@knowledge-architect@agents-get-a-complete-workflow`, since without it a session that
consults a transcript writes its own script. The task it takes from sessions is one the installed
text asks of every project that adopts the workflow, with no procedure of this repository's own, so
it is the published checker's to automate, not the maintenance tool's, per
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s` and
`goal@xtask@repeated-tasks-are-automated`. The nearest rival, a command of the maintenance tool, is
never published, so a project that adopts the workflow, such as thaum, would have no copy; a second
crate or binary would be a second package to publish and pin, which `design@core@single-crate`
argues against.

### The transcript command reads Claude Code's transcripts, and no other harness's `##transcript-reads-claude-code-alone`

The reader knows one harness's layout and fields, Claude Code's. That is the owner's word: the
owner uses no other harness, and another waits for a consumer's request, should one ever come. A
second harness is `issue@core@configuration-for-several-agent-providers`.

### The lists a session reads from a project's transcripts, for the workflow's procedures and for a survey of past sessions, are verbs of `transcript`, computed from the transcripts `##transcript-query-verbs`

Three verbs are built. `list` prints one row per session of the project, oldest first, with its handle, its first and last
time, its git branches, its count of lines the owner typed, of compactions and of subagents, and its
first message from the owner; `--branch`, `--containing` and `--since` filter. `show` prints one
session's kept lines in file order, each with its line number and its label, from `--from` a text
the owner typed, with `--subagents`, `--tools` and `--full`. `search` prints each kept line whose
text matches a regular expression, with `--role`, `--tool` and `--tools`. Each prints a header
first, per `design@core@transcript-format-tolerated-and-reported`, and `--json` prints one object
per row or line instead. The output, text and JSON alike, is for a reader and no contract, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`. The exit codes are
`design@core@exit-code-ladder`'s: 1 when a verb finds nothing to print, or `show`'s argument names
no session or more than one, or `--from` matches no line; 2 when the transcript directory, a file,
a regular expression or a date cannot be read.

`list --branch` computes the sessions that worked on a branch from the `gitBranch` field every chain
line carries, which the review before a merge needs, and `list --containing` the session holding
the message where a piece of work begins. The nearest rival, one verb that prints a whole
extraction, leaves those two lists and every survey to a script each session writes again.

### A transcript file is cut into sessions where the `sessionId` changes and where a root that is no compaction opens, and a session carries the subagent files it dispatched `##transcript-session-split`

A session opens at the first chain line of a top-level `.jsonl` file, at a chain line whose
`sessionId` differs from the previous one's, and at a chain line whose `parentUuid` is null or absent that
is no `compact_boundary` line. A compaction and a resumed process stay in their session. A subagent
file, under the parent file's directory `subagents/`, is attached to the session of its `sessionId`,
and where that id holds several, to the one holding the `tool_use` its `meta.json` names. A
session's handle is its `sessionId`'s first 8 characters, with `#2` and on for a later session of
the same id. A line whose `uuid` was already read in the session is printed once and counted.

The rule turns on how the harness writes, measured over every transcript of the machine it was
designed on, Claude Code 2.1.306 to 2.1.310, on 2026-10-25: a compaction writes a boundary with a
null parent and keeps its file and its `sessionId`, in 7 of 7; a resumed session appends to its file
under its `sessionId`, its first new line with a parent, in 3 of 3; `/clear` starts a new file under
a new `sessionId`, while the process keeps the earlier id in its paths, in the owner's test of it.
Re-take it with `klarch transcript list --all-projects --json`, whose rows name each session's
`file` and count its `compactions`, and with
`jq -r 'select(.sessionId) | .sessionId' <file> | sort -u` over a file. The nearest rival, one
session per file, agrees with every file measured; the rule is kept because the format is
internal, per `design@core@transcript-format-tolerated-and-reported`, and a file holding two
sessions would read as one with nothing reporting it.

### Every kept line of a transcript is labelled by its fields, never by a substring of its text `##transcript-labels-in-code`

Each kept line takes the label of the first row of the mapping it meets, over `type`, `isMeta`,
`isCompactSummary`, `origin.kind`, `interruptedMessageId`, `toolUseResult`, `attachment.type`,
`attachment.origin.kind`, the `system` subtypes, and whether the line sits in a subagent file. The
owner's words are the lines labelled `owner`, `owner-queued`, `owner-answer` (an answer to a
question tool) and `owner-command` (a command such as `/clear`, read from its `local_command`
line's `commandRun`); `owner-interrupt` is the owner's interrupt of a turn, an act of the owner and
no words, placed by its `interruptedMessageId`; `injected` carries the kind of what the harness or
another agent delivered, a model's refusal of a turn included; `agent` is the session's text; `dispatch` is a field-less
`user` line of a subagent file, the text its dispatcher or its workflow wrote, but for the one case
below; `tool` is a tool call
or its result; a line no row places is `unlabelled`, printed and counted. A top-level file none of
whose `user` lines carries a field, such as one of a session run with `claude -p`, is labelled by a
leading harness tag instead, and the header says so.

The rule is learned from a failure: a filter on text substrings once dropped one of the owner's
messages from a review, and an owner's message that opens with a pasted tag reads as injected text
to any tag test. The one text the rows read is the prefix that names a skill's body, inside a line
its fields already mark injected: it chooses a kind, never a side. The command holds the rule, per
`design@agent-skills@transcript-extraction-is-the-checker-s`, with a test over a fixture of each row. The fields
are the harness's, measured over every transcript of the machine the reader was designed on, as
`design@core@transcript-session-split` says; a field-less `user` line of a subagent file was the
dispatcher's brief or a workflow's task in every subagent file measured, but one owner's interrupt,
re-taken with
`find ~/.claude/projects -path '*subagents*' -name 'agent-*.jsonl' -exec jq -r 'select(.type=="user" and .isMeta==null and .origin==null and .isCompactSummary==null and ((.message.content|type)=="string" or ([.message.content[] | select(.type=="tool_result")] | length) == 0)) | .message.content | if type=="string" then .[:40] else ([.[] | .text? // empty] | join(" "))[:40] end' {} + | sort | uniq -c`. That interrupt carries no
`interruptedMessageId` and reads as `dispatch`: no field tells it apart, and the rule refuses a text
test. Every other interrupt measured carries that field, and no other `user` line does, over
Claude Code 2.1.306 to 2.1.316 on 2026-10-31, re-taken with
`find ~/.claude/projects -name '*.jsonl' -exec jq -r 'select(.type=="user" and .interruptedMessageId != null) | .message.content | if type=="string" then .[:45] else ([.[] | .text? // empty] | join(" "))[:45] end' {} + | sort | uniq -c`.

### The transcript reader tolerates every line it does not know, and every run reports each one `##transcript-format-tolerated-and-reported`

The reader knows, and marks as used or ignored on purpose, the top-level types, the `system`
subtypes, the `attachment.type` values, the content item types and the `origin.kind` values the
harness writes, as measured. A value outside them is counted, never a failure:
every verb's header names each with its count and its first file and line, with the count of
`unlabelled` lines, each line that is no JSON, and the files the tag fallback labelled, and ends,
where any of the first three is non-zero, with the line asking for it to be reported as a finding
on the checker in the retrospective. It serves `goal@knowledge-architect@the-workflow-improves-through-real-use`
by that route, per `design@agent-skills@unrecognised-entries-reach-upstream`.

The harness documents the format as internal: "The entry format is internal to Claude Code and
changes between versions, so scripts that parse these files directly can break on any release", at
https://code.claude.com/docs/en/sessions. Over the window measured, each version added types or
keys and none removed or renamed one the labels use; each line type, with the versions that wrote
it, is listed by
`find ~/.claude/projects \( \( -mindepth 2 -maxdepth 2 -name '*.jsonl' \) -o \( -path '*/subagents/*' -name 'agent-*.jsonl' \) \) -exec jq -r '"\(.type) \(.attachment.type // .subtype // "") \(.version // "-")"' {} + | sort -u`,
over the files the reader reads, where a metadata line, which carries no `version`, shows `-`, and the header of `klarch transcript list --all-projects` names any value outside the sets. A strict reader
would fail on each release that adds a type, `rejected@core@strict-transcript-reader`; a silent
tolerant one hides a renamed field, against `design@core@a-failed-parse-is-loud`.

### A project's transcripts are the files whose first line ran in the project's root or under it `##transcript-directory-resolution`

The transcript directory is `$CLAUDE_CONFIG_DIR/projects/`, or `~/.claude/projects/` where the
variable is unset. A top-level file belongs to the project when the `cwd` of its first chain line is
the project's root or a path under it, both canonicalised where they exist, so a session started in
a worktree under the root is the project's; from a linked worktree, the same path in the
repository's main checkout counts too, since a reviewer reads from a worktree of its own the
sessions that ran in the main checkout; `--all-projects` reads every file. The harness names
each project's directory by a rule of its own, "your working directory path with non-alphanumeric
characters replaced by `-`", truncated at 200 characters with an undocumented hash and overridden by
`CLAUDE_CODE_PROJECT_DIR_NAME`, on the same page of its documentation: the nearest rival, recomputing
that name from the root, reproduces an internal rule and misses a worktree's sessions, which sit in
another directory.
