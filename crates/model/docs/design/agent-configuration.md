# Model design: The agent configuration as entities

## The agent configuration as entities

### Under the `claude` harness, skills, agents, the primer and the root CLAUDE.md are entities of kinds that are no register `##harness-kinds`

The four harness kinds, `skill`, `agent`, `primer` and `instructions`, exist under the `claude`
harness alone, per `design@check@agents-table`. They are kinds, not registers: a register is what
an anchor carries, and no anchor carries them and no project declares them, since the harness fixes
where each lives. So they sit beside `path`, `planned` and `register`, the kinds already no
register. The
rival, built-in registers of a new shape carried by no anchor, would make every reader of a
register handle a register with no carrier: the manifest's refusals, the index, the extension view.

| kind | an entity | named by |
| --- | --- | --- |
| `skill` | a directory directly under `.claude/skills/<name>/` holding `SKILL.md` | the directory, per `design@model@harness-entity-names` |
| `agent` | a Markdown file at any depth under the agents directory whose frontmatter sets `name` | the file, its `.md` removed once, per `design@model@harness-entity-names` |
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
`goal@check@records-reach-their-reader`: a citation of a skill, an agent or a section is checked,
and `show` lists every one.

### Under the `claude` harness, the root CLAUDE.md is modelled and no scoped CLAUDE.md is `##root-instructions-alone-modelled`

The root CLAUDE.md alone is modelled, as the owner chose: "I'd model only root CLAUDE.md
sections." A scoped CLAUDE.md is not modelled: what such a file holds is the subject of
`issue@check@a-home-for-developer-contracts-outside-agent-configuration`.

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

A skill or an agent named outside the id grammar of `design@model@two-segment-ids` could not be
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
  per `design@check@installed-binary-version-check`, so definitions from the shipped set would fail
  every earlier commit citing a section the tip renamed. That is why
  `design@check@owned-namespace-check` already compares no installed copy under `commits`.
- **An installed copy raises no finding**, per `design@model@section-homes-carry-slugs`.
- **This repository's shipped text defines nothing in its walk**: content/ writes each section slug
  as a build placeholder, per `design@agent-skills@content-mirrors-the-install-layout`, so the
  definitions are the installed copies', which this repository installs with each change to
  content/.
