# `klarch init` creates a project and `klarch new-component` adds a Component, each writing only what does not exist yet

## Status and audience

This spec is the plan of the work that gives the checker two commands that create documents:

- `klarch init [PATH]` creates a knowledge-architect project: the manifest, the installed agent
  files, the root `CLAUDE.md` importing the primer, the documents and register homes the root
  Component carries, the plans directory and the generated indexes; with `--component` it adds
  Components, and with `--rust` the maintenance crate, its cargo aliases and its workspace;
- `klarch new-component <DIR>` adds one Component to an existing project: its documents, its
  register homes and its line in the manifest.

Both follow `cargo init` wherever the design discussion found no reason to differ.

The spec is written for a session that did not witness the design discussion that produced it. The
discussion ran in one session, which begins at the owner's message "Hey, I've been thinking of
adding a `bootstrap` subcommand to klarch". It leaves the repository in the commit that completes
its harvest.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where it marks a point as awaiting the owner's word, `skill@knowledge-architect-planning@working-a-slice`
  says what waits: a default listed under "Defaults awaiting the owner" is built as written, and
  the branch merges once the owner has ruled on it.
- Every code snippet below is an illustration of a shape, untested, and no authority: a name it
  uses is defined under New names, and where the snippet and the prose disagree, the prose holds.

## How the work is done

The work of this spec follows `skill@knowledge-architect-planning@working-a-slice`, its last
paragraph, and the repository's development procedure, `skill@klarch-development`.

## Names

| name | what it is |
| --- | --- |
| the core | the package knowledge-architect, `path@core@Cargo.toml`, and its binary `klarch`, `path@core@src/main.rs` |
| the command enum | `knowledge_architect::cli::Command`, `path@core@src/cli/mod.rs`, which every binary that dispatches the checker's commands flattens |
| `run` | `cli::run`, the dispatch of the command enum over a located manifest, `path@core@src/cli/mod.rs` |
| `locate` | `cli::locate`, which finds `knowledge-architect.toml` at or above the working directory and fails when there is none |
| the version refusal | `cli::refuse_another_version`, per `design@core@installed-binary-version-check` |
| the writer gate | `cli::complete_working_tree`, which runs phases 1 to 3 and refuses to write over an incomplete model |
| the install | `agents::shipped` and `agents::install`, `path@core@src/agents.rs`, which `install-agent-skills` runs |
| the index writer | the code `index` and `check --fix` share, per `design@core@one-list-of-generated-files`, over `index::generated_index_paths`, `path@core@src/index.rs` |
| the anchors | `entity::Anchors::of`, `path@core@src/entity.rs`, which gives every anchor, its Component flag and each register instance's home |
| required documents | `Manifest::required_documents`, `path@core@src/manifest.rs`: README.md, and CLAUDE.md while the project serves the `claude` harness |
| the setup skill | `skill@knowledge-architect-setup`, whose source is `path@agent-skills@content/skills/setup/SKILL.md` |
| the snippets directory | `path@agent-skills@snippets/`, files the build inlines into the installed text through `{{snippet:<file>}}` |
| the xtask snippet | `path@agent-skills@snippets/xtask-main.rs`, compiled by `path@xtask@examples/setup_snippet.rs` |
| a scratch repo | a temporary directory a test creates, outside every worktree unless the test says otherwise |
| M1, M2, M3 | the measurements of the discussion, restated under "What the work is" |

## What the work is

**What exists today.**

- Every command of the core starts with `locate`, in `path@core@src/main.rs`, then the build-origin
  refusal and the version refusal, then `run`. With no manifest, no command runs.
- `install-agent-skills` writes the shipped set and only says, never writes, a missing primer import
  line, since the root `CLAUDE.md` belongs to the project, per `design@core@owned-namespace-check`.
- `check --fix` writes the installed files and the generated indexes, and nothing hand-written, per
  `design@core@safe-fix-definition` and `design@core@fix-scope`.
- The setup skill opens with the smallest manifest the install accepts, written by hand, and its
  section `skill@knowledge-architect-setup@rust-project` shows four blocks: the maintenance crate's
  Cargo.toml, its cargo aliases with the `[env]` tie, its main (the xtask snippet, compiled), and a
  CI workflow. The three blocks other than the main are checked by nothing,
  `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked`.
- The core depends on `toml` (`path@core@Cargo.toml`), which drops comments when it serialises a
  file; no format-preserving TOML editor is a dependency.
- No command of the checker reads stdin.

**The measurements the design rests on**, taken on the owner's workstation with
`target/release/klarch` 0.8.0, re-taken by the commands given:

- **M1.** In an empty git repository with the setup skill's smallest manifest,
  `klarch install-agent-skills` then `klarch check --fix` exits 1 with 8 phase-2 findings: README.md,
  CLAUDE.md, docs/design.md, docs/goals.md, docs/tripwires.md, docs/rejected.md, docs/open-issues,
  docs/plans.
- **M2.** Adding 10 hand-written files (README.md; CLAUDE.md holding the import line; the four
  heading homes, each a title and "No entry yet."; a README.md in docs/open-issues/, docs/plans/,
  docs/plans/specs/ and docs/plans/milestones/), then `git add -A` and `klarch check --fix`, writes
  3 index.md files and passes. An empty goals home passes, since nothing checks it yet,
  `issue@core@a-component-states-at-least-one-goal`.
- **M3.** The M2 tree copied into a subdirectory of another git repository passes `klarch check`.
- **The behaviour of `cargo init`**, cargo 1.98.0, stdin from /dev/null: it asks for no
  confirmation; in a non-empty directory outside a repository it runs `git init`, writes
  `.gitignore` and keeps the existing files; in a subdirectory of a repository it creates no
  repository and no `.gitignore`; run again over its own package it refuses, exit 101; `cargo init d`
  creates `d/`; at a repository root with a `.gitignore`, it appends `# Added by cargo` and `/target`
  and keeps every existing line. After `cargo init` in a non-empty directory outside a repository,
  `git status --short` lists the files that were already there as untracked.

**Outside the work**, each with what owns it:

- What a new Component owes beyond its files, its goals and the decisions it inherits:
  `issue@agent-skills@a-skill-for-creating-a-component`, which stays open.
- A check that a Component states a goal: `issue@core@a-component-states-at-least-one-goal`.
- A CI workflow written by `init --rust`: ruled out by the owner, #rust-scaffold. The setup skill's
  CI block stays the route to it.
- Adding the maintenance crate to an existing Cargo workspace or package: `init --rust` refuses
  when a `Cargo.toml` exists, #rust-scaffold.
- A hook through which an extension contributes its manifest tables to `init`:
  #init-under-an-extension.
- Where developer contracts live instead of a scoped CLAUDE.md:
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`. The commands write the
  documents the check requires at the time, so they follow that work when it lands.

## What is already decided

**Decisions the design rests on, not argued again:**

- `design@core@safe-fix-definition` and `design@core@fix-scope`: `--fix` writes no hand-written
  file, which rules out creating the documents under `--fix`.
- `design@core@components-carry-the-same-documents`, `design@core@anchors-are-components-and-locations`,
  `design@core@plans-dir-fixed`, `design@core@plans-split-dirs`, `design@core@plans-at-root`,
  `design@core@heading-register-two-shapes`, `design@core@a-file-register-is-a-directory-of-entries`,
  `design@core@a-component-carries-what-a-project-attaches`, `design@core@agents-table`: what a
  Component and the root carry, which the commands create and never restate.
- `design@core@one-list-of-generated-files`, `design@core@generated-files-are-pure`: the indexes are
  written by the index writer, never templated.
- `design@core@index-takes-no-dry-run`: a write that loses nothing owes no dry run, and every
  destination is checked before any is written.
- `design@core@arguments-parse-through-clap`, `design@core@ne-minimal`,
  `design@core@exit-code-ladder`: the new commands are clap subcommands of the `#[non_exhaustive]`
  command enum, tested with `debug_assert`, exit 0, 1 or 2.
- `design@core@anchor-collisions-refused`, `design@core@kind-names-refused-as-anchors`,
  `design@core@reserved-anchors`, `design@core@plans-is-a-reserved-anchor`,
  `design@knowledge-architect@no-component-directory-named-after-the-project`: the names the commands
  refuse, by the manifest's own parse.
- `design@core@git-is-asked-through-one-module`: `git init` runs through `path@core@src/git.rs`.
- `design@knowledge-architect@version-lockstep`, `design@agent-skills@xtask-pins-checker`,
  `design@gates@gates-crate`, `design@gates@a-project-holds-its-gate-list`,
  `design@agent-skills@gates-convention`, `design@knowledge-architect@a-build-is-tied-to-its-checkout`:
  the shape `--rust` writes.
- `design@agent-skills@primer-limit`, `design@agent-skills@knowledge-table-home`: the root CLAUDE.md
  written holds the title and the import line, and no restatement.
- `design@agent-skills@goals-required`, `design@agent-skills@goals-change-through-goal-setting`: no
  command writes a goal.
- `design@agent-skills@behaviour-additions-need-real-use`: the setup skill's new text names the
  commands, a fact, which a reading admits.

**Decisions this work rewrites**, each with the texts `cargo klarch show` lists as referencing it
that the harvest judges:

| decision | what changes | judged or updated at |
| --- | --- | --- |
| `design@core@installed-binary-version-check` | `init` dispatches before `locate` and calls no version refusal: there is no manifest to read a pin from, and it writes its own version as the pin | the harvest; the comments at `cli::refuse_another_version` and in `path@core@src/main.rs` that state "before it dispatches any command" are updated in step 1 |
| `design@core@phases-gate-the-report` | "a writer refuses over an incomplete model": `new-component` complies through the writer gate; `init` writes where no model exists, and its no-loss guarantee is the destination check instead | the harvest; `tripwire@core@phases-gate-the-report-two`, whose re-entry covers a writing command, is re-read there |
| `design@core@the-core-cli-is-a-library-module` | the list of the core's commands gains `init` and `new-component` | the harvest |
| `design@core@nothing-of-a-project-is-compiled-in` | the bullet on what the word Component means gains the starting text of each document the commands write; the bullet on what this tool writes gains the maintenance crate's files `--rust` writes | the harvest |
| `design@agent-skills@exact-pin` | "The setup skill writes the key": `init` writes it, and the setup skill moves it | the harvest |
| `design@agent-skills@pinned-binary-runs-inside-the-project` | `init` runs before any pin exists, from whatever binary the user installed, such as a user-scope install; every later command runs the pinned checker as the head says | the harvest |

**The standing entries the discussion's searches returned, with their outcome:**

| entry | outcome |
| --- | --- |
| `issue@agent-skills@a-skill-for-creating-a-component` | bears: `new-component` creates the files and leaves the issue open; the setup skill points to the command, step 5 |
| `issue@core@a-component-states-at-least-one-goal` | bears: the commands create empty goals homes, as M2 shows the check accepts; unchanged |
| `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked` | bears: closed by step 4, per #rust-templates-one-home |
| `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | bears: the commands take the document list from required documents, so they follow that work |
| `tripwire@core@phases-gate-the-report-two` | bears: re-read at the harvest; `init` adds no flag around the gate, and `new-component` passes through it |
| `tripwire@core@fix-makes-a-choice` | bears on nothing: no creation runs under `--fix` |
| `issue@knowledge-architect@command-output-is-not-declared-a-contract` | bears: the commands' printed lines are not declared a contract; their changelog entries say what they do, not what they print |
| `issue@core@tooling-for-project-skills` | bears on nothing built: the manifest `init` writes declares no prefix field, which does not exist |
| `issue@core@configuration-for-several-agent-providers` | bears on nothing: `init` writes the default harness |
| `issue@core@a-missing-root-document-finding-names-a-repair-that-does-not-apply` | bears on nothing: a tree `init` wrote holds every root document |
| every other standing entry the three searches returned with a doubt (14 entries, listed in the discussion's grounding) | bears on nothing, except `tripwire@agent-skills@agent-text-written-without-the-standard`, which step 5 meets by loading `skill@knowledge-architect-agent-configuration` before editing the setup skill |

**The design and `rejected` entries graded `bears`** are the decisions above, and these, with their
outcome: `rejected@core@claude-md-in-every-component` (not brought back: CLAUDE.md is written only
under the `claude` harness); `rejected@core@plans-directory-per-component` (not brought back:
`new-component` creates no plans directory); `rejected@gates@each-project-writes-its-runner` (not
brought back: the generated crate depends on the gates library); `design@xtask@gates-list-primary-home`
(bears on nothing: it governs this repository's own xtask, not the template);
`design@agent-skills@content-mirrors-the-install-layout` (bears: the templates live in the snippets
directory, not in content/, which holds what the install writes);
`design@agent-skills@shipped-text-cites-no-entry` (bears: the templates hold no reference);
`design@agent-skills@audit-triggers` (bears on nothing: `init` adds no audit occasion).

## Criteria

### A new project reaches a passing check by following the documentation alone `##adoption-by-documentation`

Binding, from `goal@knowledge-architect@any-project-can-adopt-it` and
`goal@knowledge-architect@adoption-is-easy`. Met by #init-command and #init-delivery.

### A Rust project gets its gates without designing them `##gates-without-design`

Binding, from `goal@knowledge-architect@setup-brings-quality-tools`. Met by #rust-scaffold.

### Nothing about one project is compiled in, and the Rust parts are optional `##nothing-project-specific`

Binding, from `design@core@nothing-of-a-project-is-compiled-in` and
`goal@agent-skills@installed-text-works-anywhere`. Met by #init-file-contents and #rust-scaffold.

### No command writes goal text `##no-goal-written`

Binding, from `goal@knowledge-architect@the-owner-decides` and
`design@agent-skills@goals-change-through-goal-setting`. Met by #init-file-contents.

### No run loses content, and every destination is checked before any is written `##no-content-lost`

Binding, from the argument of `design@core@index-takes-no-dry-run`. Met by #init-confirmation,
#init-existing-files and the append rule of #rust-scaffold.

### Each command runs without a terminal `##runs-without-a-terminal`

Binding as a presumption, from `design@core@arguments-parse-through-clap`, "These binaries run
blind". Met by #init-confirmation.

### The pin written is the binary's own version, and so are the maintenance crate's dependencies `##pin-is-own-version`

Binding, from `design@knowledge-architect@version-lockstep` and `design@agent-skills@exact-pin`.
Met by #rust-scaffold.

### The documents a Component carries, and the index bytes, come from the code the check uses `##one-document-list`

Binding as a presumption, from `design@core@one-question-one-command` and
`design@core@one-list-of-generated-files`. Met by #component-command and #init-file-contents.

## Threads

### `init` creates a project: the manifest, the installed files, the documents and the indexes `##init-command`

Proposed by the owner, round 1, as a `bootstrap` subcommand; shaped by the session, round 1. Final
state: approved. The `check --fix` variant was ruled out on `design@core@safe-fix-definition`
(#arg-2); the status quo loses on #arg-3. Shape: "Decided design", the command. Harvest: a design
head of core. The owner, round 2: "init-command, init-name-and-path accepted (agreed that init is
a better name)."

### The command is `klarch init [PATH]`, with the name defaulting to PATH's basename `##init-name-and-path`

Proposed by the session, round 1 (#arg-4), against the owner's `bootstrap <name>`. Final state:
approved. Shape: "Decided design", the command line. Harvest: the core design head of
#init-command. The owner, round 2: "init-command, init-name-and-path accepted (agreed that init is
a better name)." and "Q1: init is good."

### `init` asks for no confirmation; it refuses an existing manifest and overwrites nothing `##init-confirmation`

The owner proposed a Y/N confirmation, round 1; the session argued against it, round 1 (#arg-5).
Final state: approved, on the owner's condition that it match `cargo init`, which the session
measured asks none (#arg-21). Shape: "Decided design", what a run writes. Harvest: a design head of
core, under the slug `init-asks-no-confirmation`, since the thread's name describes the question
rather than the decision: `#init-confirmation → init-asks-no-confirmation`. The owner, round 2:
"Q4: your argument hold. I think it is important to match usage habits of cargo init, does it not
ask for a confirmation ?"

### An existing README or CLAUDE.md is kept, so `init` serves an existing project `##init-existing-files`

Proposed by the session, round 1 (#arg-6). Final state: approved. Shape: "Decided design", what a
run writes. Harvest: the core design head of #init-confirmation. The owner, round 2:
"init-existing-files: approved." and "Q2: it can serve an existing project that I would want to
migrate for example."

### `init` runs `git init` outside a worktree only into an empty or new directory, unless `--vcs git` asks `##init-runs-git-init`

Proposed by the session, round 1 (#arg-7); reopened by the session, round 2, on a premise the
owner's acceptance rested on that the session's measurement showed false (#arg-20); closed on the
owner's choice of shape B over shape A. Final state: approved. Shape: "Decided design", git. Harvest:
a design head of core; tripwire T1. The owner, round 2: "init-runs-git-init: well this solves the
problem I mentionned above I guess. If cargo init does it too, we can follow the same model. It is
probably best to mimic cargo init as much as possible." Round 3: "Q5: B".

### The files `init` writes are M2's tree, with an empty goals home and a CLAUDE.md of a title and the import line `##init-file-contents`

Proposed by the session, round 1 (#arg-8). Final state: approved. Shape: "Decided design", the
files. Harvest: the core design head of #init-command, and the rewrite of
`design@core@nothing-of-a-project-is-compiled-in`. The owner, round 2: "init-file-contents agreed".

### `new-component <DIR>` writes a Component's documents and homes from the check's own list, and `init --component` runs it `##component-command`

Proposed by the owner, round 1 ("a command to create a component from scratch"); shaped by the
session, round 1 (#arg-9). Final state: approved. Shape: "Decided design", `new-component`. Harvest:
a design head of core; tripwire T2. The owner, round 2: "component-command: agreed. Indeed, the
issue is about content and scope of a component, not just basic structure."

### The manifest is edited with a format-preserving TOML editor, and validated before it is written `##component-manifest-edit`

Proposed by the session, round 1 (#arg-10). Final state: approved. Shape: "Decided design",
`new-component`. Harvest: a comment at the code and the commit message, unless an entry test holds
at the harvest. The owner, round 2: "component-manifest-edit: agreed."

### `init --rust` writes a virtual workspace, the maintenance crate, its aliases and `/target` in `.gitignore`, and no CI `##rust-scaffold`

Proposed by the owner, round 1 ("a basic Cargo.toml, .cargo/config.toml, even tools/xtask
possibly"); shaped by the session, round 1 (#arg-11). Final state: approved. Shape: "Decided
design", `--rust`. Harvest: a design head of core. The owner, round 2: "rust-scaffold: agreed." and
"Q3: no."

### The maintenance crate's blocks have one home, the snippets directory, which the setup skill inlines and `--rust` writes `##rust-templates-one-home`

Proposed by the session, round 1 (#arg-12). Final state: approved. Shape: "Decided design", the
templates. Harvest: a design head of agent-skills; closes
`issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked`. The owner, round 2:
"rust-templates-one-home: agreed."

### `init` is documented in the core's README and run from a binary the user installed; the setup skill names the commands `##init-delivery`

Proposed by the session, round 1 (#arg-13); the owner gave the use case, round 2 (#arg-15,
#arg-18). Final state: approved. Shape: "Decided design", delivery. Harvest: the rewrites of
`design@agent-skills@exact-pin` and `design@agent-skills@pinned-binary-runs-inside-the-project`.
The owner, round 2: "init-delivery: see my use case described above. An agent could still need it
to init another project from an existing one, but this is not really the point and would rarely
happen. I would run the init from a global user scope install of the klarch binary that I did
myself."

### An extension binary that runs `init` gets a manifest without its tables, and the check reports it `##init-under-an-extension`

Proposed by the session, round 1 (#arg-14). Final state: approved: presumed-settled in round 2, it
was closed by the owner's positive word given against the checkpoint table that listed it with "you
did not address it; contest it or it closes with the batch". Shape: "Decided design", extension
binaries. Harvest: a comment at `cli::init`. The owner, round 3: "Q5: B\nQ6: all five."

## Arguments

### The request: a project with the workflow installed, with Rust as the primary language `##arg-1`

Round 1, the owner. Bears on #init-command, #init-name-and-path, #init-confirmation,
#component-command, #rust-scaffold. "it would create an empty project with a minimal
knowledge-architect.toml, and the installed workflow, + a CLAUDE.md holding the link to primer, +
the mandatory content and directories under docs." "Maybe it should ask for confirmation before
writing all these files"

### `--fix` may not write a hand-written file `##arg-2`

Round 1, the session. Bears on #init-command. "The rival \"`check --fix` creates the missing
homes\" is out by a fact. `design@core@safe-fix-definition` allows `--fix` to write only installed
or generated files, and a README is neither."

### The status quo costs 11 hand-written files and an order of steps `##arg-3`

Round 1, the session. Bears on #init-command. M1 and M2: after the install, the check names 8
findings, and 10 hand-written files plus the manifest make it pass.

### The cargo and git convention covers both readings of the name argument `##arg-4`

Round 1, the session. Bears on #init-name-and-path. "`cargo init` and `git init` are what a Rust
user already types."

### A prompt blocks an agent and guards against a loss that cannot happen `##arg-5`

Round 1, the session. Bears on #init-confirmation. "An agent's shell has no terminal (M4, C6). The
prompt either blocks, or needs a `--yes` that every agent then always passes, so it protects
nothing." "The worst case is running it in the wrong directory. That leaves new files there, which
`git clean` or a delete removes."

### Refusing on any existing file would leave every existing project with the hand procedure `##arg-6`

Round 1, the session. Bears on #init-existing-files. "That makes `init` useful only in an empty
directory, and every existing project keeps the hand procedure."

### A project in a subdirectory of a repository passes the check `##arg-7`

Round 1, the session. Bears on #init-runs-git-init. M3.

### What every root CLAUDE.md would write alike belongs in the primer `##arg-8`

Round 1, the session. Bears on #init-file-contents. "the root `CLAUDE.md` holds a title and the
import line, nothing else, per `design@agent-skills@primer-limit`"

### The files a Component owes are not what it owes: the skill issue stays open `##arg-9`

Round 1, the session. Bears on #component-command. "That issue asks what a Component owes beyond its
files (its goals, the decisions it inherits), and a command cannot answer that."

### The `toml` crate drops comments, and the manifest carries reason comments `##arg-10`

Round 1, the session. Bears on #component-manifest-edit. "The `toml` crate drops comments when it
rewrites a file (M6). I propose adding `toml_edit`, which keeps formatting." The rival: "It costs
no dependency, but the command then cannot finish its job in one run."

### Merging into a hand-written workspace is another design, and CI assumes a host `##arg-11`

Round 1, the session. Bears on #rust-scaffold. "It refuses if a `Cargo.toml` exists, because
merging into a hand-written workspace is a separate design." "CI is a GitHub Actions file, so it
adds a hosting assumption."

### A separate template would be a fourth unchecked copy `##arg-12`

Round 1, the session. Bears on #rust-templates-one-home. "That closes
`issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked`, where a separate template
would add a fourth unchecked copy."

### `init` runs before any installed text exists `##arg-13`

Round 1, the session. Bears on #init-delivery. "`init` runs before any installed text exists, so no
skill can tell an agent to run it. Its documentation home is the core README (C1)."

### An extension's missing tables are a finding that names its repair `##arg-14`

Round 1, the session. Bears on #init-under-an-extension. "An extension binary that runs `init`
writes a manifest without the extension's tables, so the first check reports a phase-1 finding
that names its repair."

### The owner's use: the skills installed before the agent session starts `##arg-15`

Round 2, the owner. Bears on #init-delivery, #init-confirmation. "the main use for me was to create
the project with the installed skills myself **before** starting an agentic session, which
otherwise would not have access to the skills the standard way."

### An accidental run outside a repository needs a messy cleanup `##arg-16`

Round 2, the owner. Bears on #init-confirmation, #init-runs-git-init. "Running it accidentaly in a
non git place would require manual cleanup, which can be quite a bit messy IMO. Maybe it can just
error out if ran from a place that is not a git repo root."

### Mimic `cargo init` `##arg-17`

Round 2, the owner. Bears on #init-runs-git-init, #init-confirmation. "If cargo init does it too,
we can follow the same model. It is probably best to mimic cargo init as much as possible." and "I
think it is important to match usage habits of cargo init"

### The Component issue is about content and scope `##arg-18`

Round 2, the owner. Bears on #component-command. "Indeed, the issue is about content and scope of a
component, not just basic structure."

### `init` runs from the owner's user-scope install `##arg-19`

Round 2, the owner. Bears on #init-delivery. "I would run the init from a global user scope install
of the klarch binary that I did myself."

### `git init` makes an accidental run's files untracked beside the user's own `##arg-20`

Round 2, the session. Bears on #init-runs-git-init. "it creates `.git` there; `git clean -fd` would
then delete **your existing files too**, because every one of them is untracked; the safe cleanup is
`rm -rf .git` plus the files the command wrote." Measured with `cargo init`: `git status --short`
lists the existing `notes.txt` as untracked.

### `cargo init` asks for no confirmation, and appends `/target` `##arg-21`

Round 2, the session. Bears on #init-confirmation, #rust-scaffold. The measured table under "What
the work is"; members admitted by #arg-17: `--vcs <git|none>`, and the append of `/target` to an
existing `.gitignore`.

## New names, in one place

```text
crates/core/src/cli/mod.rs
  Command::Init(InitArgs)                 the `init` subcommand
  Command::NewComponent(NewComponentArgs) the `new-component` subcommand
  InitArgs { path: Option<PathBuf>, name: Option<String>, component: Vec<PathBuf>,
             rust: bool, vcs: Option<Vcs> }
  NewComponentArgs { dir: PathBuf }
  Vcs { Git, None }                       the value of `--vcs`
  init(args: InitArgs) -> Result<ExitCode, String>
                                          runs `init` with no located manifest; public, so an
                                          extension binary can dispatch it before `locate`
crates/core/src/cli/create.rs             (new) the writer both commands share: the destination
                                          check, the starting text of each document, the
                                          manifest edit, the `.gitignore` append
crates/core/Cargo.toml                    dependency `toml_edit`
crates/agent-skills/snippets/
  workspace-cargo.toml                    (new) the virtual workspace's Cargo.toml
  xtask-cargo.toml                        (new) the maintenance crate's Cargo.toml
  cargo-config.toml                       (new) the aliases and the `[env]` tie
  ci.yml                                  (new) the CI workflow the setup skill shows
  xtask-main.rs                           existing
crates/agent-skills/src/lib.rs
  SNIPPETS: &[(&str, &str)]               (new) each snippet file's name and text, so the core
                                          writes what the setup skill shows
```

`planned@core@src/cli/create.rs`, `planned@agent-skills@snippets/workspace-cargo.toml`,
`planned@agent-skills@snippets/xtask-cargo.toml`, `planned@agent-skills@snippets/cargo-config.toml`,
`planned@agent-skills@snippets/ci.yml`.

## Decided design

### The command line

```text
klarch init [PATH] [--name <NAME>] [--component <DIR>]... [--rust] [--vcs <git|none>]
  PATH         defaults to "."; created, with its parents, when it does not exist
  --name       defaults to PATH's basename
  --component  a Component to add, as `new-component` adds it; repeatable
  --rust       the maintenance crate, its aliases and its workspace
  --vcs        whether `git init` runs; the default is the rule under "Git"

klarch new-component <DIR>
```

The nearest rival, `bootstrap <name>`, names the same act in a word no Rust user already types for
it (#arg-4).

### What a run of `init` writes, and what it refuses

1. **It refuses, exit 2, writing nothing**: when PATH, or a directory above it, holds
   `knowledge-architect.toml`; when the manifest it would write fails `Manifest::parse`, which
   refuses a kind name, a reserved anchor word and a collision; when the name begins with
   `knowledge-architect-`, the installer's namespace; when a `--component` would fail as
   `new-component` fails; when the git rule below refuses.
2. **It checks every destination before writing any**, as `index` does: a destination that exists
   as a file is **kept**, not written, and listed as `kept`; one that exists as a directory where a
   file is due, or is a symlink, refuses the run. So a run either refuses having written nothing or
   writes every missing file. Nothing is overwritten (#arg-5, #arg-6).
3. **It asks nothing**: `cargo init` asks nothing (#arg-21), and an agent's shell has no terminal
   (#arg-5). It prints the absolute root first, then one line per file:

   ```text
   creating a knowledge-architect project `probe` in /home/u/code/probe
   created  a git repository
   wrote    knowledge-architect.toml
   kept     README.md
   wrote    CLAUDE.md
   ...
   ```

4. **Its order** is the order of `design@core@fix-before-the-checks`: the manifest; the install,
   through the install code; the documents; then the indexes, through the index writer, once phases
   1 to 3 find nothing; then it runs the check and exits with the check's code, so a kept file with
   a finding shows at once.

### Git

- PATH inside a git worktree: no repository is created, whatever the directory holds (M3).
- PATH outside every worktree, and new or empty: `git init` runs, as `cargo init`'s does.
- PATH outside every worktree, and holding any file: `init` refuses, exit 2, naming the directory
  and `--vcs git`, unless `--vcs git` is given. This is the one departure from `cargo init`, on the
  owner's choice of shape B: an accidental run in such a directory would leave the user's own files
  untracked beside the new ones, where `git clean` deletes both (#arg-20).
- `--vcs none` never runs `git init`; the check then exits 2 outside a worktree, per
  `design@core@failed-git-is-exit-two`, which the output names.

The rival that lost, shape A, follows `cargo init` with no guard.

### The files

The root, with `harness` at its default:

```text
knowledge-architect.toml     [project] name, checker-version = the binary's CHECKER_VERSION,
                             components = [the --component paths]; [walk] with empty lists
README.md                    "# <name>" and one sentence for the owner to replace
CLAUDE.md                    "# <name>", then the import line alone
docs/goals.md                title, one sentence on what the home holds, "No entry yet."
docs/design.md, docs/rejected.md, docs/tripwires.md     the same shape
docs/open-issues/README.md   the same shape; index.md by the index writer
docs/plans/README.md, docs/plans/specs/README.md, docs/plans/milestones/README.md
                             the same shape; the two index.md by the index writer
.claude/...                  the shipped set, by the install code
```

The list of a Component's documents and homes is not written down a second time: the writer asks
required documents and the anchors, so a register declared later, or a change to what a Component
carries, reaches the commands with no edit to them. The starting text holds no reference, no link
and no heading at a register's level, so it passes every grammar check in any project. Nothing
writes a goal. No directory of audit reports or of retrospective analyses, and no roadmap, is
written: each is created when first needed.

### `new-component`

1. It runs through `run`, after `locate` and the version refusal, and through the writer gate:
   over an incomplete model it refuses, exit 2, writing nothing.
2. It edits the manifest with `toml_edit`, appending `<DIR>` to `[project] components` and keeping
   every other byte, comments included; it parses the edited text with `Manifest::parse` before
   writing anything, and refuses with the parser's own complaint.
3. It writes the Component's required documents and the home of every register it carries, by the
   rules of "What a run of `init` writes", items 2 and 3; CLAUDE.md only while the project serves
   the `claude` harness. An `opt-in` register a Component should carry is not guessed: the user
   names the Component in that register's `components`, and the check reports the home.
4. It runs the index writer, then the check, and exits with the check's code.

`init --component <DIR>` runs steps 2 and 3 of this over the manifest `init` is writing, before
the indexes.

### `--rust`

It refuses, exit 2, writing nothing, when PATH holds a `Cargo.toml`. Otherwise it adds:

```text
Cargo.toml                 the virtual workspace, members = ["xtask"]
xtask/Cargo.toml           the maintenance crate, both crates pinned "=<CHECKER_VERSION>"
xtask/src/main.rs          the xtask snippet, with <PROJECT> replaced
.cargo/config.toml         the aliases x and klarch, and the [env] tie
.gitignore                 "/target"; appended under "# Added by klarch" to an existing one,
                           every existing line kept, and nothing written when it already
                           ignores /target
manifest                   command = "cargo klarch"; "xtask" in components
xtask/README.md, CLAUDE.md, docs/...   the Component's documents, as new-component writes them
```

`<PROJECT>` is the name upper-cased with each `-` replaced by `_`, as this repository's
`KNOWLEDGE_ARCHITECT_CHECKOUT`. No CI file is written; the setup skill's CI block is the route to
it.

### The templates

The four blocks of `skill@knowledge-architect-setup@rust-project` become files of the snippets
directory, which the setup skill inlines with `{{snippet:<file>}}`, as it already inlines the xtask
snippet. The agent-skills crate exposes them as `SNIPPETS`, and `--rust` writes them, substituting
`<version>` and `<PROJECT>`. One text, two readers: the skill shows what `init --rust` writes. A
test builds a project `init --rust` wrote (AC2), which checks the Cargo.toml, the aliases and the
main together; the CI block, which `init` does not write, is checked by a test that its gates
command parses with the gates library's `GatesArgs`.

### Delivery

- The core's README, `path@core@README.md`, gains an "Adopting the workflow" passage: install a
  binary the user keeps, for instance `cargo install --locked knowledge-architect`, run
  `klarch init` in the project, then start the agent session, whose setup skill takes over.
- The setup skill's "Reaching this skill" names `init` as how the manifest is made, and its
  Components section names `new-component` as how a Component is added; its Rust section says
  `init --rust` writes the blocks it shows. The edit loads
  `skill@knowledge-architect-agent-configuration` first.
- After `init`, the project runs its pinned checker as before: `cargo klarch` through the
  maintenance crate, or an install under its own root. A user-scope binary of another version is
  refused by the version refusal, which names both versions.

### Extension binaries

`cli::init` is public, so an extension binary can dispatch it before `locate`, as the core's main
does. The manifest it writes holds no extension's table, and the first check reports each, with its
repair (#arg-14). A binary that does not dispatch it, as the xtask snippet does not, reaches `run`
with `init` only inside a project, where `run` refuses it, exit 2, since a manifest exists.

## Mapping tables

The command line to the files written, over every combination of the flags that write files:

| flags | root documents and homes | the shipped set | `--component` Components | the maintenance crate and the xtask Component | `.gitignore` |
| --- | --- | --- | --- | --- | --- |
| none | written | written | none | none | untouched |
| `--component D`... | written | written | each D | none | untouched |
| `--rust` | written | written | none | written | `/target` ensured |
| `--rust --component D`... | written | written | each D | written | `/target` ensured |

`--vcs` changes only whether `git init` runs, and `--name` only the name. The shipped set is
written only under the `claude` harness, which is the only harness `init` writes.

## Losing alternatives

- **`check --fix` creating the missing homes**, ruled out in #init-command on
  `design@core@safe-fix-definition`: a README is neither an installed nor a generated file (#arg-2).
- **The name `bootstrap <name>`**, lost to `init [PATH]` in #init-name-and-path on the owner's word
  (#arg-4).
- **A Y/N confirmation, with `--yes` for scripts**, lost in #init-confirmation: `cargo init` asks
  none, an agent has no terminal, and nothing is overwritten (#arg-5, #arg-21).
- **Refusing when any destination exists**, lost in #init-existing-files: `init` would then serve
  only an empty directory (#arg-6).
- **Shape A, `git init` in any directory outside a worktree, as `cargo init`**, lost in
  #init-runs-git-init on the owner's choice of shape B (#arg-20).
- **A refusal wherever PATH is not a git repository's root**, the owner's own proposal in round 2
  (#arg-16), which the same message then set aside for `cargo init`'s model (#arg-17). It is not
  dropped: shape B keeps its protection for a directory that holds files, and T1 brings back its
  stronger form, a refusal outside a worktree whatever the directory holds, on one report of a
  messy cleanup. A refusal at a subdirectory of a repository would also refuse a layout M3 shows
  the check accepts.
- **Printing the manifest line for the user to add**, lost in #component-manifest-edit: the command
  then cannot finish in one run (#arg-10).
- **A CI workflow written by `--rust`**, ruled out by the owner, Q3 (#arg-11).
- **Templates kept apart from the setup skill's blocks**, lost in #rust-templates-one-home: a fourth
  unchecked copy (#arg-12).
- **An extension hook contributing tables to `init`**, lost in #init-under-an-extension: the
  finding names its repair (#arg-14).

## Readings

`cargo init`'s behaviour, measured with cargo 1.98.0 and restated under "What the work is", is the
external behaviour #init-confirmation, #init-runs-git-init and #rust-scaffold follow. It is
recorded in the harvest's core head on `init`, with the commands that re-take it.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| inserting into `components` drops or moves a comment of the manifest | #component-manifest-edit | acceptance criterion AC1, on the owner's word: #manifest-comments-kept |
| a change to the gates library or the core breaks the generated maintenance crate, and nothing fails | #rust-templates-one-home | acceptance criterion AC2, on the owner's word: #scaffold-builds |
| the documents the check requires change, and `init` writes the old set | #init-file-contents | acceptance criterion AC3, on the owner's word: #init-then-check-passes |
| an `init` run in an unintended directory needs a cleanup beyond deleting the files it listed, or loses a file | #init-runs-git-init, #init-confirmation | tripwire T1, on the owner's word, written at the harvest in `register@core@tripwire` |
| `new-component` refuses over a phase-1 to phase-3 finding the user cannot repair first, and the user adds the Component by hand | #component-command | tripwire T2, on the owner's word, written at the harvest in `register@core@tripwire` |

T1. Fires on one report, an issue, a retrospective finding or the owner's observation, of an `init`
run in an unintended directory that needed a cleanup beyond deleting the files its output listed,
or that lost a file. Response: reopen #init-runs-git-init and #init-confirmation with the stronger
guard, a refusal outside a worktree whatever the directory holds. Re-entry: the standing-state
review.

T2. Fires on one reported case of `new-component` refused over a finding of phases 1 to 3 that the
user could not repair before adding the Component. Response: reopen the writer gate of
`design@core@phases-gate-the-report` for this command. Re-entry: the standing-state review.

## Acceptance criteria

### AC1: a manifest's comments survive `new-component` byte for byte `##manifest-comments-kept`

Guards #component-manifest-edit. Judged at step 3. Observable: a test runs `new-component` over a
manifest holding comment lines above `[project]`, inside the `components` array and after it, and in
`[walk]`; it fires when any comment line of the input is missing, changed or moved in the output.
Response: reopen #component-manifest-edit.

### AC2: a project written by `init --rust` builds against the workspace's crates `##scaffold-builds`

Guards #rust-templates-one-home. Judged at step 4. Observable: a test runs `init --rust` in a
scratch repo, adds a `[patch.crates-io]` section pointing knowledge-architect and
knowledge-architect-gates at this workspace's crates, and runs `cargo build -p xtask` and
`cargo klarch check` there; it fires when either fails. Response: reopen #rust-templates-one-home.
Its run time is measured at step 4 and reported in the step's commit; the owner rules whether the
test runs in the default suite.

### AC3: `init` then `check` passes, root only, with a Component, and with `--rust` `##init-then-check-passes`

Guards #init-file-contents. Judged at steps 2, 3 and 4. Observable: a test runs `init` in a scratch
repo in each of the three shapes, then `check`; it fires when `check` does not print
`PASSED: no findings`. Response: reopen #init-file-contents.

## Implementation sequence

### Step 1: `init` writes the manifest alone, behind every refusal

Builds: the `init` subcommand, `cli::init`, its dispatch before `locate` in `path@core@src/main.rs`,
the refusal of `init` inside `run`, the destination check, the git rule with `--vcs`, the manifest
text. No document, no install.

Claims:
- in a scratch repo, `init` writes `knowledge-architect.toml` whose pin is `CHECKER_VERSION`, and
  `check` then reports M1's 8 findings; shown to fail by writing another version;
- `init` under a directory holding a manifest exits 2 and writes nothing; shown to fail by
  removing the refusal;
- a name `design`, `plans`, `elsewhere` or `knowledge-architect-x` exits 2 and writes nothing;
- outside a worktree, a non-empty PATH without `--vcs git` exits 2 and writes nothing; an empty
  PATH gets a repository; inside a worktree, no repository is created;
- stdin from /dev/null changes nothing;
- the clap declaration passes `debug_assert`.

Fails alone on: the manifest's text or the git rule.

### Step 2: `init` writes the root's documents, the shipped set and the indexes

Builds: the starting text of each document, the install through the install code, the index writer
call, the final check.

Claims: AC3 in its root-only shape; an existing README.md and CLAUDE.md are kept byte for byte and
listed as `kept`; a symlink at a destination refuses the run with nothing written.

Fails alone on: a document's text, or a home the anchors list and the writer misses.

### Step 3: `new-component`, and `init --component`

Builds: `Command::NewComponent`, the `toml_edit` dependency and the manifest edit, the writer gate,
`--component`.

Claims: AC1; AC3 in its Component shape; over a tree with a phase-2 finding, `new-component` exits 2
and writes nothing; a `<DIR>` whose basename is the project's name exits 2 with the parser's
complaint and writes nothing.

Fails alone on: the manifest edit.

### Step 4: the snippets, `SNIPPETS`, and `--rust`

Builds: the four snippet files, the setup skill's blocks replaced by `{{snippet:<file>}}`,
`SNIPPETS`, the `--rust` flag and the `.gitignore` append, the test of the CI block's gates command.

Claims: AC2; AC3 in its `--rust` shape; the installed setup skill is byte for byte what it was before
the move, apart from the blocks now showing the snippet files' text; an existing `.gitignore` keeps
every line, gains `/target` once, and is untouched when it already ignores it; a PATH holding a
`Cargo.toml` exits 2 and writes nothing.

Fails alone on: a snippet's text, or the build of the generated crate.

### Step 5: the documentation and the changelog

Builds: the core README's passage, the setup skill's new sentences, the changelog entries
(`cli`, minor, for each command; a Workflow entry for the setup skill), `cargo x changelog`.

Claims: `cargo klarch check` passes; the agent-facing-text review axis finds each added sentence
names an act.

Fails alone on: the documentation.

### Step 6: the harvest

Per the harvest below.

## Order rationale

- Step 1 before step 2: the refusals and the git rule are judged before anything but the manifest
  can be written.
- Step 2 before step 3: `new-component` reuses the writer and the starting text step 2 lands.
- Step 3 before step 4: `--rust` adds a Component through the code step 3 lands.
- Step 4 before step 5: the setup skill's text names what steps 1 to 4 built.
- Step 5 before step 6: the harvest records what the documentation states.

## Defaults awaiting the owner

- D1. The work is planned as one spec, on one branch, rather than a milestone. It bears on every
  thread.
- AC2's place in the default suite, once its run time is measured at step 4.

## Harvest

| what | where | when |
| --- | --- | --- |
| #init-command, #init-name-and-path, #init-file-contents | a design head of core on `init`, with the reading of `cargo init` | step 6 |
| #init-confirmation, #init-existing-files | a design head of core, `init-asks-no-confirmation` | step 6 |
| #init-runs-git-init | a design head of core | step 6 |
| #component-command | a design head of core on `new-component` | step 6 |
| #component-manifest-edit | a comment at the manifest edit and the commit message, unless an entry test holds | step 3 |
| #rust-scaffold | a design head of core on `--rust` | step 6 |
| #rust-templates-one-home | a design head of agent-skills; closes `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked` | step 4 for the issue, step 6 for the head |
| #init-delivery | the rewrites of `design@agent-skills@exact-pin` and `design@agent-skills@pinned-binary-runs-inside-the-project` | step 6 |
| #init-under-an-extension | a comment at `cli::init` | step 1 |
| the rewrites under "What is already decided" | each head named there | step 6 |
| the losing alternatives | `rejected` entries of core where `skill@knowledge-architect-decision-recording@losing-alternatives` admits them; the Y/N prompt and shape A are the candidates | step 6 |
| T1, T2 | `register@core@tripwire` | step 6 |
| AC1, AC2, AC3 | reported on in the landing commits, then proposed as tripwires or deleted with the spec | step 6 |

## Later consequences

- A check that a Component states a goal, `issue@core@a-component-states-at-least-one-goal`, would
  make a tree `init` writes fail until goal-setting has run: the setup skill's goal-setting step
  then becomes part of reaching a passing check.
- The skill on what a new Component owes, `issue@agent-skills@a-skill-for-creating-a-component`,
  will name `new-component` as its first act.
- Adding the maintenance crate to an existing Cargo workspace remains to design, if a project asks.
