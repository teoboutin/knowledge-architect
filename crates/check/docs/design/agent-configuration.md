# Core design: The agent configuration

## The agent configuration

### The `[agents]` table declares which agent harnesses a project serves `##agents-table`

`[agents] harness` lists the agent harnesses the project serves. The tool knows one, `claude`, and
it is the default when the table is absent, because the tool is built for projects developed
mostly by agents. Under `claude`, every component carries a CLAUDE.md, and the agent files this
version ships must be installed, per `design@check@owned-namespace-check`. An empty list declares
no agent configuration: no CLAUDE.md is required, and nothing is installed or checked. A value the
tool does not know is refused in phase 1, and leaves the list.

Under `claude`, the four harness kinds exist, per `design@model@harness-kinds`; under an empty
list, they do not.

Under an empty list, the files of the installer's namespace are walked as the project's own
documents, like any others. A harness for another provider would be a value added to the list,
requiring its own file and its own layout, which
`issue@check@configuration-for-several-agent-providers` records;
`goal@knowledge-architect@serves-one-kind-of-project` asks every target project to run its agents
under `claude`, and excludes a mode of the checker for another harness. The empty list serves a
target project that uses the checker alone, which that goal admits, and the checker's mock projects.

The design this decision did not keep: `rejected@check@claude-md-in-every-component`.

### A project declares the command it runs the checker by `##declared-command`

`[project] command` names the command a project runs the checker by; when it is absent the command
is `klarch`, the binary's name. The checker prints it in its messages, in the header of every
generated index, and in the agent files it installs, which are templates holding the placeholder
`{{command}}`. No fixed name serves every project: a project with an extension runs its own binary,
as thaum runs its rules extension through a cargo alias, and `check`'s binary refuses a manifest
holding a table only an extension claims. An extension project therefore declares a command that
runs its own binary, preferably a cargo alias such as `cargo klarch`, and does not install its
binary under the plain name. A command that cannot be printed on one line in a code span, empty or
holding a line break or a backtick, is refused in phase 1, and the default stands. This serves
`goal@knowledge-architect@every-target-project-can-adopt-it`: every project, extended or not, reads
instructions that name the command it actually runs.

### The installed agent files are committed `##installed-files-committed`

`install-agent-skills` writes each file this version ships at its install path, rendered with the
project's command, and the project commits them, so a session started from a fresh clone, a web
session or a review in a new worktree has them without building anything, which serves
`goal@knowledge-architect@agents-get-a-complete-workflow`. The rival, installing them on demand, gives
such a session nothing until something has run the install. The installed files are outside the walk by
construction, as the generated indexes are: their prose is the shipped text's, judged where that text is
written, and its illustration paths would otherwise be reported in every installing project.

**The installed copies are also read, for definitions only**, per
`design@model@installed-entities-from-the-tree`: the model holds them parsed, no check walks them
for references, and they raise no finding of the section rule.

### The installed agent files are checked byte for byte, in a namespace whose ownership is decided by a name `##owned-namespace-check`

The install deletes every file of the installer's namespace that the version does not ship. The namespace
is the prefix `knowledge-architect-` among the skill directories, the agent files and the saved workflow
files of `.claude/`, and the directory .claude/knowledge-architect/. A project's own skill takes a name of
its own, so ownership is decided by a name, with no record and no history: a renamed skill shows as the old
file to remove and the new one missing; one install repairs both, and the removal awaits staging like any
deletion, which the check names until it is staged. The rival, a record of installed files with a digest,
is `rejected@check@install-record-file`.

A symbolic link on an owned path is refused by the install before it touches anything, because following
one would write or delete in another directory. Under `claude`, the family `agents` of the last phase
reports each shipped file missing or whose bytes differ from its rendered template, and each file of the
namespace the version does not ship. No check reads these findings, and a model built over a stale copy or a
missing one is complete, per `design@check@phases-gate-the-report`. Only `check` compares: `commits` does
not, because the running binary ships its own version's text, and an older commit's installed set would fail
against it with no repair a commit already in history can take. The comparison is of bytes: the binary holds
the text, so a digest would add a dependency and nothing else.

The install edits nothing outside the namespace. A change a project needs to an installed skill belongs in a
skill of its own, routed from the project's CLAUDE.md.

### The root CLAUDE.md delivers the primer by an import line, which the project writes `##primer-import-line`

The install does not edit the root CLAUDE.md, which belongs to the project: it says the import line is
missing, and the family `agents` reports a root CLAUDE.md that does not import the shipped primer with a
line holding exactly an at sign followed by .claude/knowledge-architect/PRIMER.md, with the line as its
repair. The rival is `rejected@check@session-start-hook-primer`.
