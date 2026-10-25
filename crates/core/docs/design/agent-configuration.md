# Core design: The agent configuration

## The agent configuration

### The `[agents]` table declares which agent harnesses a project serves `##agents-table`

`[agents] harness` lists the agent harnesses the project serves. The tool knows one, `claude`, and
it is the default when the table is absent, because the tool is built for projects developed
mostly by agents. Under `claude`, every component carries a CLAUDE.md, and the agent files this
version ships must be installed, per `design@core@owned-namespace-check`. An empty list declares
no agent configuration: no CLAUDE.md is required, and nothing is installed or checked. A value the
tool does not know is refused in phase 1, and leaves the list.

Under `claude`, the four harness kinds exist, per `design@core@harness-kinds`; under an empty
list, they do not.

Under an empty list, the files of the installer's namespace are walked as the project's own
documents, like any others. A harness for another provider is a value added to the list, requiring
its own file and its own layout, which `issue@core@configuration-for-several-agent-providers`
records. This is what serves `goal@knowledge-architect@any-project-can-adopt-it` for a project
without agents.

The design this decision did not keep: `rejected@core@claude-md-in-every-component`.

### A project declares the command it runs the checker by `##declared-command`

`[project] command` names the command a project runs the checker by; when it is absent the command
is `klarch`, the binary's name. The checker prints it in its messages, in the header of every
generated index, and in the agent files it installs, which are templates holding the placeholder
`{{command}}`. No fixed name serves every project: a project with an extension runs its own
binary, as thaum runs its rules extension through a cargo alias, and the core's binary refuses a
manifest holding a table only an extension claims. An extension project therefore declares a
command that runs its own binary, preferably a cargo alias such as `cargo klarch`, and does not
install its binary under the plain name. A command that cannot be printed on one line in a code
span, empty or holding a line break or a backtick, is refused in phase 1, and the default stands.
This serves `goal@knowledge-architect@any-project-can-adopt-it`: every project, extended or not,
reads instructions that name the command it actually runs.

### The installed agent files are committed, and checked byte for byte in a namespace the installer owns `##owned-namespace-check`

`install-agent-skills` writes each file this version ships at its install path, rendered with the
project's command, and deletes every file of the installer's namespace that the version does not
ship. The namespace is the prefix `knowledge-architect-` among the skill directories, the agent
files and the saved workflow files of `.claude/`, and the directory .claude/knowledge-architect/. A project's own skill takes
a name of its own, so ownership is decided by a name, with no record and no history: a renamed
skill shows as the old file to remove and the new one missing; one install repairs both, and the
removal awaits staging like any deletion, which the check names until it is staged.

The installed files are committed, so a session started from a fresh clone, a web session or a
review in a new worktree has them without building anything, which serves
`goal@knowledge-architect@agents-get-a-complete-workflow`. A symbolic link on an owned path is
refused by the install before it touches anything, because following one would write or delete in
another directory. Under `claude`, phase 2 reports each
shipped file missing or whose bytes differ from its rendered template, each file of the namespace
the version does not ship, and a root CLAUDE.md that does not import the shipped primer with a line
holding exactly an at sign followed by .claude/knowledge-architect/PRIMER.md. These are facts about what the tree
holds, like a missing required document, so a run stops before references while one stands, per
`design@core@phases-gate-the-report`. A command that writes a generated file refuses while one
stands, as over any phase-2 finding;
`issue@core@installed-file-findings-belong-in-phase-four` records why the last phase would fit
them better. Only `check` compares: `commits` does not, because the running binary ships its own
version's text, and an older commit's installed set would fail against it with no repair a commit
already in history can take. The comparison is of bytes: the binary holds the text, so a
digest would add a dependency and nothing else. The installed files are outside the walk by
construction, as the generated indexes are: their prose is the shipped text's, judged where that
text is written, and its illustration paths would otherwise be reported in every installing
project.

**The installed copies are also read, for definitions only**, per
`design@core@installed-entities-from-the-tree`: the model holds them parsed, no check walks them
for references, and they raise no finding of the section rule.

The install edits nothing outside the namespace. In particular it does not edit the root
CLAUDE.md, which belongs to the project: it says the import line is missing, and the check reports
it with the line as its repair. A change a project needs to an installed skill belongs in a skill
of its own, routed from the project's CLAUDE.md.

The designs this decision did not keep: `rejected@core@session-start-hook-primer` and
`rejected@core@install-record-file`.

### Under the `claude` harness, skills, agents, the primer and the root CLAUDE.md are entities of kinds that are no register `##harness-kinds`

The four harness kinds, `skill`, `agent`, `primer` and `instructions`, exist under the `claude`
harness alone, per `design@core@agents-table`. They are kinds, not registers: a register is what
an anchor carries, and no anchor carries them and no project declares them, since the harness fixes
where each lives. So they sit beside `path`, `planned` and `register`, the kinds already no
register. The
rival, built-in registers of a new shape carried by no anchor, would make every reader of a
register handle a register with no carrier: the manifest's refusals, the index, the extension view.

| kind | an entity | named by |
| --- | --- | --- |
| `skill` | a directory directly under `.claude/skills/<name>/` holding `SKILL.md` | the directory, per `design@core@harness-entity-names` |
| `agent` | a Markdown file at any depth under the agents directory whose frontmatter sets `name` | the file, its `.md` removed once, per `design@core@harness-entity-names` |
| `primer` | the installed primer; its sections alone are entities | none |
| `instructions` | the root CLAUDE.md; its sections alone are entities | none |

The layout is the harness's, read in its documentation of skills,
https://code.claude.com/docs/en/skills.md, and of subagents,
https://code.claude.com/docs/en/sub-agents.md: a skill in a `<subdirectory>/.claude/skills/`
directory is loaded only when a session reads a file there, and is no entity here; the agents
directory is scanned "recursively"; a file there with no `name` is treated "as documentation kept
beside your agents", so it is no agent, and no README or index is owed there.

Under `harness = []` none of the four kinds exists, a span headed by one is silent, and installed
copies a project kept are its own documents, whose slugs are misplaced definitions. Serves
`goal@core@records-reach-their-reader`: a citation of a skill, an agent or a section is checked,
and `show` lists every one.

### Under the `claude` harness, the root CLAUDE.md is modelled and no scoped CLAUDE.md is `##root-instructions-alone-modelled`

The root CLAUDE.md alone is modelled, as the owner chose: "I'd model only root CLAUDE.md
sections." A scoped CLAUDE.md is not modelled: what such a file holds is the subject of
`issue@core@a-home-for-developer-contracts-outside-agent-configuration`.

### Under the `claude` harness, a skill is named by its directory and an agent by its file, and a frontmatter `name`, where one is set, equals that name `##harness-entity-names`

"The directory name also invokes the skill", in the harness's documentation of skills, even where a
frontmatter `name` sets a second command. An agent's "identity comes only from the `name`
frontmatter field", in its documentation of subagents. A frontmatter `name` that differs from the
directory's or the file's name is a finding, so the name a reference cites and the name the harness
uses are one. The match is the owner's ruling, against the frontmatter `name` alone as an agent's
id: "I'm not sure our installed workflow would be able to work properly if not, and I'd rather make
the checks more strict to keep the workflow simpler than the other way around." Its cost: the
harness documents a skill whose `name` differs from its directory as a second command for it,
which is a finding here, so a project that wants a short alias renames the directory. The `name:`
line is read alone, as YAML reads it, so frontmatter the harness reads and the checker's own
frontmatter subset refuses, a list or a multi-line description, causes no finding.

### Under the `claude` harness, a skill or an agent whose name a reference cannot spell, or that the harness does not load, is a finding `##unusable-harness-name-is-a-finding`

A skill or an agent named outside the id grammar of `design@core@two-segment-ids` could not be
cited. The harness's documentation of skills says a skill folder is not named "`synced`, in any
capitalization", and that one named `anthropic-skills` or beginning `anthropic-skills:` does not
load. So a skill whose directory begins `anthropic-skills` is a finding, and one named `synced` is
one under the grammar, which refuses a name of one segment. The rival, recording the
documentation with no check, would pass a skill no session can use. An agent's `name` is, in the
documentation of subagents, "at most 256 characters", holds no `:` and does not start with `-`; the
id grammar is stricter, and stands as its own requirement.

### The installed skills, agents and primer are defined from the installed copies of the tree being judged `##installed-entities-from-the-tree`

The model holds the installed copies, parsed like a walked document and walked by nothing:
`Model::build` reads them off the disk, and `commits` hands in each commit's own blobs. Both read
them whatever the walk rows say, so a skip over the installer's namespace leaves the two verdicts
alike. A commit is therefore judged against the installed set it holds, so a branch that renames a
section in an upgrade and repairs its citations passes `commits` on every commit.

- **Not the binary's shipped set.** `commits` judges every commit of a range with the tip's binary,
  per `design@core@installed-binary-version-check`, so definitions from the shipped set would fail
  every earlier commit citing a section the tip renamed. That is why `design@core@owned-namespace-check`
  already compares no installed copy under `commits`.
- **An installed copy raises no finding**, per `design@core@section-homes-carry-slugs`.
- **This repository's shipped text defines nothing in its walk**: content/ writes each section slug
  as a build placeholder, per `design@agent-skills@content-mirrors-the-install-layout`, so the
  definitions are the installed copies', which this repository installs with each change to
  content/.
