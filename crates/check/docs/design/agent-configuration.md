# Check design: The agent configuration

## The agent configuration

### The `[agents]` table declares which agent harnesses a project serves `##agents-table`

`[agents] harness` lists the agent harnesses the project serves. The tool knows one, `claude`, and
it is the default when the table is absent, because the tool is built for projects developed
mostly by agents. Under `claude`, every component carries a CLAUDE.md, and the agent files this
version ships must be installed, per `design@check@installed-files-compared-by-bytes`. An empty list declares
no agent configuration: no CLAUDE.md is required, and nothing is installed or checked. A value the
tool does not know is refused in phase 1, and leaves the list.

Under `claude`, the four harness kinds exist, per `design@model@harness-kinds`; under an empty
list, they do not.

Under an empty list, the files of the installer's namespace are walked as the project's own
documents, like any others. A harness for another provider would be a value added to the list,
requiring its own file and its own layout, which
`issue@check@configuration-for-several-agent-providers` records;
`goal@serves-one-kind-of-project` asks every target project to run its agents
under `claude`, and excludes a mode of the checker for another harness. The empty list serves a
target project that uses the checker alone, which that goal admits, and the checker's mock projects.

The design this decision did not keep: `rejected@check@claude-md-in-every-component`.

### A project declares the command it runs the checker by `##declared-command`

`[project] command` names the command a project runs the checker by; when it is absent the command
is `klarch`, the binary's name. The checker prints it in its messages, in the header of every
generated index, and in the agent files it installs, which are templates holding the placeholder
`{{command}}`. No fixed name serves every project: a project with an extension runs its own binary,
as thaum runs its rules extension through a cargo alias, and `klarch` refuses a manifest
holding a table only an extension claims. An extension project therefore declares a command that
runs its own binary, preferably a cargo alias such as `cargo klarch`, and does not install its
binary under the plain name. A command that cannot be printed on one line in a code span, empty or
holding a line break or a backtick, is refused in phase 1, and the default stands. This serves
`goal@every-target-project-can-adopt-it`: every project, extended or not, reads
instructions that name the command it actually runs.

### The installed agent files are committed `##installed-files-committed`

`install-agent-skills` writes each file this version ships at its install path, rendered with the
project's command, and the project commits them, so a session started from a fresh clone, a web
session or a review in a new worktree has them without building anything, which serves
`goal@agents-get-a-complete-workflow`. The rival, installing them on demand, gives
such a session nothing until something has run the install.

### The installed agent files are outside the walk `##installed-files-outside-the-walk`

They leave the walk by construction, as the generated indexes do: their prose is the shipped text's, judged
where that text is written, and its illustration paths would otherwise be reported in every installing
project. The rival, walking them as any other document, reports those paths in a project that can repair
none of them.

The model holds them parsed for definitions, per `design@model@installed-entities-from-the-tree`: no check
walks them for references, and they raise no finding of the section rule or of a slug defined twice.

### Ownership of an installed file is decided by a name, in a namespace the installer owns `##owned-namespace-check`

The install deletes every file of the installer's namespace that the version does not ship. The namespace
is the prefix `knowledge-architect-` among the skill directories, the agent files and the saved workflow
files of `.claude/`, and the directory .claude/knowledge-architect/. A project's own skill takes a name of
its own, so ownership is decided by a name, with no record and no history: a renamed skill shows as the old
file to remove and the new one missing; one install repairs both, and the removal awaits staging like any
deletion, which the check names until it is staged. The rival, a record of installed files, is
`rejected@check@install-record-file`.

The install edits nothing outside the namespace. A change a project needs to an installed skill belongs in a
skill of its own, per `design@agent-skills@overlay-by-separate-skills`.

### The installed agent files are compared with the shipped text byte for byte `##installed-files-compared-by-bytes`

Under `claude`, the family `agents` of the last phase reports each shipped file missing or whose bytes differ
from its rendered template, line endings normalised, and each file of the namespace the version does not ship.
No check reads these findings, and a model built over a stale copy or a missing one is complete, per
`design@check@phases-gate-the-report`. The binary holds the text, so the rival, a digest, would add a dependency
and nothing else. Only `check` compares, per `design@check@only-check-compares-installed-files`.

### Only `check` compares the installed files, and `commits` does not `##only-check-compares-installed-files`

The running binary ships its own version's text, and an older commit's installed set would fail against it
with no repair a commit already in history can take. `check --staged` compares, since its snapshot is the
next commit, per `design@check@staged-check-semantics`. A commit's tree is judged against the installed set it holds, per
`design@model@installed-entities-from-the-tree`.

### The root CLAUDE.md delivers the primer by an import line on its first line, which the project writes `##primer-import-line`

The install does not edit the root CLAUDE.md, which belongs to the project: it says the import line is
not on the file's first line, and the family `agents` reports a root CLAUDE.md whose first line is not
the import line, an at sign followed by .claude/knowledge-architect/PRIMER.md, with that first line as
its repair. The import line anywhere else does not count. The rival delivery is
`rejected@check@session-start-hook-primer`.

The check serves `goal@agents-get-a-complete-workflow`: it passes only where the harness delivers the
primer. Nothing comes before the first line, so no fence, indented code block or comment can enclose
it, and whether the harness evaluates the import there is known without modelling the harness's
markdown parser. Any lower line can be enclosed by what precedes it, the last line included, since an
unclosed fence runs to the end of the document in CommonMark. A check that accepts the line lower down
must parse as the harness parses, which is not documented as CommonMark, and every difference is a
root CLAUDE.md that passes with no primer delivered: `rejected@check@import-in-prose-by-parser`. The
saving of that rival, no line moved by a project that placed it elsewhere, is a migration, the lowest
cost per `design@knowledge-architect@capability-cost-before-migration`, and no goal asks for another
place. The check relies on the harness evaluating an import on any line of prose, which a session of
the `claude` harness shows: the root CLAUDE.md reaches it with the import line as text, and the primer
as a separate block after the whole file.

### The first line is compared with the import line byte for byte `##import-line-exact-bytes`

A byte-order mark before the line and the line ending after it, LF or CRLF, are not part of it, and
nothing else is stripped: a first line with a space before or after the import line fails the check of
`design@check@primer-import-line`. A leading space is refused because four of them make an indented
code block, where the harness evaluates no import. A trailing one is harmless to the harness, and
refusing it keeps one byte string the only accepted shape, which the finding's repair states.
