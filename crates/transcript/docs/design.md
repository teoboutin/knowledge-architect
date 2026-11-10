# Design

How transcript is built, and why.

## The transcripts

### One command, `transcript`, reads the agent harness's session transcripts over a project, whichever harness the project declares, so every binary offers it `##transcript-command-reads-any-project`

`klarch transcript` reads the transcripts the agent harness writes on disk, for the transcript
reviewer, the assembly of a plan document and any survey of past sessions. It is a variant of
`cli`'s command enum, so every binary that flattens it offers the command, per
`design@cli@the-commands-are-a-library-module`, and it runs inside a project, after the version
refusal of `design@check@installed-binary-version-check`, whatever the project's
`[agents] harness`: the transcripts are the harness's own, whether or not the project installs the
workflow. Its function takes the project's root, and is this crate's public interface, per
`design@knowledge-architect@transcript-crate`.

It serves `goal@knowledge-architect@agents-get-a-complete-workflow`, since without it a session that
consults a transcript writes its own script. The task it takes from sessions is one the installed
text asks of every project that adopts the workflow, with no procedure of this repository's own, so
it is a published crate's to automate, not the maintenance tool's, per
`design@xtask@a-local-task-is-the-maintenance-tool-s`:
`goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` has knowledge-architect provide
the tool, so that each session of a target project gets the same result. The nearest rival, a command of the maintenance tool, is
never published, so a project that adopts the workflow, such as thaum, would have no copy.

### The transcript command reads Claude Code's transcripts, and no other harness's `##transcript-reads-claude-code-alone`

The reader knows one harness's layout and fields, Claude Code's.
`goal@knowledge-architect@serves-one-kind-of-project` asks every target project's agents to run
under `claude`, and a second harness waits for a change of that ask, per
`issue@check@configuration-for-several-agent-providers`.

### The lists a session reads from a project's transcripts, for the workflow's procedures and for a survey of past sessions, are verbs of `transcript`, computed from the transcripts `##transcript-query-verbs`

Three verbs are built. `list` prints one row per session of the project, oldest first, with its handle, its first and last
time, its git branches, its count of lines the owner typed, of compactions and of subagents, and its
first message from the owner; `--branch`, `--containing` and `--since` filter. `show` prints one
session's kept lines in file order, each with its line number and its label, from `--from` a text
the owner typed, with `--subagents`, `--tools` and `--full`. `search` prints each kept line whose
text matches a regular expression, with `--role`, `--tool` and `--tools`. Each prints a header
first, per `design@transcript@transcript-format-tolerated-and-reported`, and `--json` prints one object
per row or line instead. The output, text and JSON alike, is for a reader and no contract, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`. The exit codes are
`design@cli@exit-code-ladder`'s: 1 when a verb finds nothing to print, or `show`'s argument names
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
internal, per `design@transcript@transcript-format-tolerated-and-reported`, and a file holding two
sessions would read as one with nothing reporting it.

### Every kept line of a transcript is labelled by its fields, and a line no field places by a whole text or a leading tag the harness writes, never by a substring of its text `##transcript-labels-in-code`

Each kept line takes the label of the first row of the mapping it meets, over `type`, `isMeta`,
`isCompactSummary`, `origin.kind`, `interruptedMessageId`, `toolUseResult`, `attachment.type`,
`attachment.origin.kind`, the `system` subtypes, and whether the line sits in a subagent file. The
owner's words are the lines labelled `owner`, `owner-queued`, `owner-answer` (an answer to a
question tool) and `owner-command` (a command such as `/clear`, read from its `local_command`
line's `commandRun`); `owner-interrupt` is the owner's interrupt of a turn and `owner-shell` a
command the owner ran with the `!` prefix, acts of the owner and no words; `injected` carries the
kind of what the harness or another agent delivered, a model's refusal of a turn, a shell command's
output and a fork's inherited context included; `agent` is the session's text; `dispatch` is a
field-less `user` line of a subagent file that no row above it places, the text its dispatcher or
its workflow wrote; `tool` is a tool call or its result; a line no row places is `unlabelled`,
printed and counted, in a file none of whose `user` lines carries a field as in any other.

A field-less `user` line, one that carries none of `isMeta`, `isCompactSummary` and `origin`, is
read by its text in three rows, each before the `dispatch` row: its whole text equal to one of the
interrupt texts the harness writes is `owner-interrupt`, where `interruptedMessageId` is absent; and,
in a top-level file, a text that opens with the harness's `<bash-input>` tag is `owner-shell`, and one
that opens with `<bash-stdout>`, on the child of such a line, is `injected shell-output`, so a
message that opens with a pasted tag, and that no field places, stays unlabelled. A forked subagent's file opens with a
`fork-context-ref` line, outside the chain, which is shown as `injected fork-context` with the line of
the parent's file, found by its `parentLastUuid`, up to which the fork inherits its context; the
copy of the parent's dispatching call and of its result that the fork's file holds next is not
shown, since the parent's file shows the call, and shown again it would read as one the fork made.

The rule is learned from a failure: a filter on text substrings once dropped one of the owner's
messages from a review, and an owner's message that opens with a pasted tag reads as injected text
to any tag test. So no text row reads a line its fields place: an owner's typed message carries
`origin.kind` `human`, and keeps its label whatever its text; and a row reads a whole text or a
leading tag the harness writes, never a substring. The lines they read are those no field places, which
would otherwise be unlabelled or read as `dispatch`, so a text row moves no line out of an owner's
label, and in a subagent file a shell tag places nothing, since no owner types there. What is
excluded, labelling a line the owner typed by a phrase in its text, reaches a test that reads what
a line means in the discussion, and not a tag the harness writes around the owner's command, on the
owner's word. The one text the field rows read is the prefix that names a skill's body, inside a line its fields already mark
injected: it chooses a kind, never a side. The command holds the rule, per
`design@agent-skills@transcript-extraction-is-the-checker-s`, with a test over a fixture of each
row.

The text rows exist because no field places those lines,
`rejected@transcript@fields-alone-place-every-line`. Measured over every transcript of the owner's
machine, Claude Code 2.1.306 to 2.1.316, on 2026-11-07: an interrupt with no
`interruptedMessageId`, at top level or in a subagent file, a `<bash-input>` line, and a subagent
file's dispatcher lines carry the same key beyond those of every chain line, `promptId`, and every
other interrupt carries the id. The key sets of the `user` lines no field
places are re-taken with
`{ find ~/.claude/projects -mindepth 2 -maxdepth 2 -name '*.jsonl'; find ~/.claude/projects -path '*/subagents/*' -name 'agent-*.jsonl'; } | xargs -d '\n' jq -c 'select(.type=="user" and .isMeta==null and .origin==null and .isCompactSummary==null) | select((.message.content|type)=="string" or ([.message.content[]? | select(.type=="tool_result")] | length) == 0) | (.message.content | if type=="string" then . else ([.[]? | select(.type=="text") | .text] | join(" ")) end) as $t | {sub:(input_filename|test("subagents")), keys:([keys[]|select(. as $k | ["parentUuid","isSidechain","type","uuid","timestamp","userType","entrypoint","cwd","sessionId","version","gitBranch","message","slug","agentId","requestId"]|index($k)|not)]|join(",")), t:($t|.[:22])}' | sort | uniq -c`.
A file with no field is a session run with `claude -p`, or a log older than the fields: its prompt
carries `turnOrigin` `sdk` where a typed message carries `human`, and no field names who called
`claude -p`, a person or a script, so its prompt is unlabelled and reported rather than given to the
owner by a test on what it says, which this head excludes.

### The transcript reader tolerates every line it does not know, and every run reports each one `##transcript-format-tolerated-and-reported`

The reader knows, and marks as used or ignored on purpose, the top-level types, the `system`
subtypes, the `attachment.type` values, the content item types and the `origin.kind` values the
harness writes, as measured. A value outside them is counted, never a failure:
every verb's header names each with its count and its first file and line, with the count of
`unlabelled` lines and each line that is no JSON, and ends, where any of them is non-zero, with the line asking for it to be reported as a finding
on the checker in the retrospective. It serves `goal@knowledge-architect@the-workflow-improves-through-real-use`
by that route, per `design@agent-skills@unrecognised-entries-reach-upstream`.

The harness documents the format as internal: "The entry format is internal to Claude Code and
changes between versions, so scripts that parse these files directly can break on any release", at
https://code.claude.com/docs/en/sessions. Over the window measured, each version added types or
keys and none removed or renamed one the labels use; each line type, with the versions that wrote
it, is listed by
`{ find ~/.claude/projects -mindepth 2 -maxdepth 2 -name '*.jsonl'; find ~/.claude/projects -path '*/subagents/*' -name 'agent-*.jsonl'; } | xargs -d '\n' jq -r '"\(.type) \(.attachment.type // .subtype // "") \(.version // "-")"' | sort -u`,
over the files the reader reads, where a metadata line, which carries no `version`, shows `-`, and the header of `klarch transcript list --all-projects` names any value outside the sets. A strict reader
would fail on each release that adds a type, `rejected@transcript@strict-transcript-reader`; a silent
tolerant one hides a renamed field, against `design@model@a-failed-parse-is-loud`.

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
