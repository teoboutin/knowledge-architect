# `klarch init` creates a project and `klarch new-component` adds a Component, never overwriting a hand-written file

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
- Every code snippet and every starting text below is an illustration of a shape, untested, and no
  authority: a name it uses is defined under New names, and where a snippet and the prose disagree,
  the prose holds.

## How the work is done

The work of this spec follows `skill@knowledge-architect-planning@working-a-slice`, its last
paragraph, and the repository's development procedure, `skill@klarch-development`.

## Names

| name | what it is |
| --- | --- |
| the core | the package knowledge-architect, `path@core@Cargo.toml`, and its binary `klarch`, `path@core@src/main.rs` |
| the command enum | `knowledge_architect::cli::Command`, `path@core@src/cli/mod.rs`. The core's main uses it as its subcommand field; a binary that registers extensions flattens it beside its own commands, as the crate documentation's example in `path@core@src/lib.rs` does; the xtask snippet nests it under its `klarch` subcommand, so an xtask user types `cargo klarch <command>` |
| `run` | `cli::run`, the dispatch of the command enum over a located manifest, `path@core@src/cli/mod.rs` |
| `locate` | `cli::locate`, which finds `knowledge-architect.toml` at or above the working directory and fails when there is none |
| the build-origin refusal | `cli::refuse_a_foreign_build`, `path@core@src/build_origin.rs`, per `design@core@a-foreign-build-is-refused`: it refuses a binary built from another checkout of the tool, and leaves alone a tree that holds no copy of the tool's crates |
| the version refusal | `cli::refuse_another_version`, per `design@core@installed-binary-version-check` |
| the writer gate | `cli::complete_working_tree`, which runs phases 1 to 3 and refuses to write over an incomplete model |
| the install | `agents::shipped` and `agents::install`, `path@core@src/agents.rs`, which `install-agent-skills` runs: it writes every shipped file whose bytes differ and deletes every file of the installer's namespace the shipped set does not hold |
| the index writer | `cli::generated_list` with `cli::check_destinations`, `path@core@src/cli/mod.rs`, the code `index` and `check --fix` share, per `design@core@one-list-of-generated-files`. It writes no index for an instance whose directory does not exist |
| the anchors | `entity::Anchors::of`, `path@core@src/entity.rs`, which gives every anchor, its Component flag and each register instance's home |
| required documents | `Manifest::required_documents`, `path@core@src/manifest.rs`: README.md, and CLAUDE.md while the project serves the `claude` harness |
| the plans README | docs/plans/README.md, required by its own rule in `path@core@src/check/tree.rs`, not by a register home |
| the setup skill | `skill@knowledge-architect-setup`, whose source is `path@agent-skills@content/skills/setup/SKILL.md` |
| the snippets directory | `path@agent-skills@snippets/`, files the build inlines into the installed text through a `{{snippet:<file>}}` line; the build fails on a snippet no such line names, per `path@agent-skills@build.rs` |
| the xtask snippet | `path@agent-skills@snippets/xtask-main.rs`, compiled by `path@xtask@examples/setup_snippet.rs`; `path@xtask@tests/snippets.rs` fails on a snippet no example includes |
| a scratch repo | a temporary directory a test creates, outside every worktree unless the test says otherwise |
| M1 to M7 | the measurements under "What the work is" |
| C1 to C8 | the labels the discussion gave the criteria: C1 #adoption-by-documentation, C2 #gates-without-design, C3 #nothing-project-specific, C4 #no-goal-written, C5 #no-content-lost, C6 #runs-without-a-terminal, C7 #pin-is-own-version, C8 #one-document-list |
| Q1 to Q6 | the questions put to the owner, listed below with the answers |

**The questions put to the owner, and the answers**, each quoted in the thread it closed:

- Q1. Is the name `init`, with an optional PATH, or `bootstrap <name>`? Answer: `init`.
- Q2. Should `init` serve an existing project, keeping its README and CLAUDE.md, or only an empty
  directory? Answer: an existing project.
- Q3. Should `--rust` also write a GitHub Actions CI workflow? Answer: no.
- Q4. Does the argument against a Y/N confirmation hold, or is the prompt wanted? Answer: the
  argument holds.
- Q5. After the correction on `git init`: shape A (follow `cargo init`, no guard) or shape B
  (refuse a non-empty directory outside a worktree unless `--vcs git`)? Answer: B.
- Q6. Which of the five premortem items, AC1, AC2, AC3, T1 and T2, are applied or recorded?
  Answer: all five.

## What the work is

**What exists today.**

- `path@core@src/main.rs` runs `locate`, then the build-origin refusal and the version refusal over
  the located root, then `run`. With no manifest, no command runs.
- `install-agent-skills` (`install_agent_skills` in `path@core@src/cli/mod.rs`) writes the shipped
  set and only says, never writes, a missing primer import line, since the root `CLAUDE.md`
  belongs to the project, per `design@core@owned-namespace-check`.
- `check --fix` (`fix_then_check` in `path@core@src/cli/mod.rs`) writes the installed files and the
  generated indexes, and nothing hand-written, per `design@core@safe-fix-definition` and
  `design@core@fix-scope`.
- The setup skill opens with the smallest manifest the install accepts, written by hand, and its
  section `skill@knowledge-architect-setup@rust-project` holds four fenced blocks: the maintenance
  crate's Cargo.toml, its cargo aliases with the `[env]` tie, its main (the xtask snippet), and a
  CI workflow. It states the workspace a project gains in prose, with no block. The three blocks
  other than the main are checked by nothing,
  `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked`.
- The snippets directory holds one file, the xtask snippet. The build's header says a snippet is
  code a workspace target compiles, and `path@xtask@tests/snippets.rs` requires every snippet to be
  included by an example of the xtask crate.
- The core depends on `toml` (`path@core@Cargo.toml`), whose `Table` is a map of values and holds
  no comment, so a manifest parsed and written back loses its comments. No format-preserving TOML
  editor is a dependency.
- No command of the checker reads stdin: no read of stdin in `path@core@src/` or
  `path@gates@src/`, and every git child gets a null or piped stdin (`path@core@src/git.rs`).
- `Manifest::parse` returns `Ok` for a kind name, a reserved anchor word or a collision, and
  records each refusal in `Manifest::complaints()`.

**The measurements the design rests on**, taken on the owner's workstation with the release build
of this checkout's binary, 0.8.0, `cargo build --release -p knowledge-architect --bin klarch`, and
each re-taken by the code-claims review of this spec. `<klarch>` is that binary.

- **M1.** `git init` in an empty directory, the setup skill's smallest manifest written, then
  `<klarch> install-agent-skills`, then `<klarch> check --fix`: exit 1, phase 2, 8 findings, for
  README.md, CLAUDE.md, docs/design.md, docs/goals.md, docs/tripwires.md, docs/rejected.md,
  docs/open-issues and docs/plans. Without the install, `<klarch> check` reports 34 phase-2
  findings: those 8 and one per shipped file.
- **M2.** Adding 10 hand-written files (README.md; CLAUDE.md holding the import line; the four
  heading homes, each a title and "No entry yet."; a README.md in docs/open-issues/, docs/plans/,
  docs/plans/specs/ and docs/plans/milestones/), then `<klarch> check --fix`: it writes 3 index.md
  files and prints `PASSED: no findings`, with or without `git add -A` first. An empty goals home
  passes, since nothing checks it yet, `issue@core@a-component-states-at-least-one-goal`.
- **M3.** The M2 tree copied into a subdirectory of another git repository: `<klarch> check` exits 0.
- **M4.** `grep -rn 'stdin' crates/core/src crates/gates/src`: no command reads stdin, as above.
- **M5.** Outside every git worktree, `<klarch> check` exits 2 naming `git ls-files` and "not a git
  repository".
- **M6.** The `toml` crate's `Table`, in its source under the cargo registry, is a map of values
  with no comment field; `grep -n toml_edit Cargo.lock` finds nothing.
- **M7.** `ls crates/agent-skills/snippets/` lists `xtask-main.rs` alone.
- **The behaviour of `cargo init`**, cargo 1.98.0, each case in a fresh directory, with
  `cargo init </dev/null`:
  - it prints no question, and exits 0;
  - in a directory holding `notes.txt`, outside any repository: `ls -A` shows `.git`, `.gitignore`,
    `Cargo.toml`, `notes.txt` and `src`, and `git status --short` lists `?? notes.txt`;
  - in a subdirectory of a repository: no `.git` and no `.gitignore`;
  - run a second time in the same directory: "`cargo init` cannot be run on existing Cargo
    packages", exit 101;
  - `cargo init d` creates `d/`;
  - at a repository root holding a `.gitignore` of one line, `mine`: the file then holds `mine`, a
    blank line, `# Added by cargo`, a blank line and `/target`.

**Outside the work**, each with what owns it:

- What a new Component owes beyond its files, its goals and the decisions it inherits:
  `issue@agent-skills@a-skill-for-creating-a-component`, which stays open.
- A check that a Component states a goal: `issue@core@a-component-states-at-least-one-goal`.
- A CI workflow written by `init --rust`: ruled out by the owner, Q3. The setup skill's CI block
  stays the route to it.
- Adding the maintenance crate to an existing Cargo workspace or package: `init --rust` refuses
  when a `Cargo.toml` exists, #rust-scaffold.
- A hook through which an extension contributes its manifest tables to `init`:
  #init-under-an-extension, ruled by the owner, D4.
- Where developer contracts live instead of a scoped CLAUDE.md:
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration`. The commands write the
  documents required documents names at the time, so they follow that work when it lands.

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
- `design@core@a-foreign-build-is-refused`: `init` runs the build-origin refusal over PATH where
  PATH exists; a directory holding no copy of the tool's crates passes it.
- `design@core@anchor-collisions-refused`, `design@core@kind-names-refused-as-anchors`,
  `design@core@reserved-anchors`, `design@core@plans-is-a-reserved-anchor`,
  `design@knowledge-architect@no-component-directory-named-after-the-project`: the names the commands
  refuse, read from the manifest's own complaints.
- `design@agent-skills@project-name-prefix`: a project's own skills take its name and a hyphen as a
  prefix, which grounds the refusal of a name in the installer's namespace.
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

**Decisions this work rewrites**, each with every text `cargo klarch show` lists as referencing it
outside this spec, and each restatement no reference reaches, with what judges or updates it:

| decision | what changes | referencing texts, and what judges or updates each |
| --- | --- | --- |
| `design@core@installed-binary-version-check` | `init` is dispatched before `locate` and calls no version refusal: there is no manifest to read a pin from, and it writes its own version as the pin | restating the timing "before any command", updated at step 1: the doc comment of `cli::refuse_another_version`; the crate documentation of `path@core@src/lib.rs`, its paragraph that restates the head and its extension example; at step 5: the setup skill's sentence "Every binary that dispatches a command of the checker calls `cli::refuse_another_version` before it dispatches". Citing it, judged at step 6 with no change expected unless the text states the timing: the core's design home at its sections on the shape of a run and the agent configuration; `path@core@src/manifest.rs` (two comments); `path@core@tests/binary.rs` (one test comment); two heads of `register@agent-skills@design`; `issue@gates@the-gate-list-head-bundles-the-library-s-independence` (two lines); `register@knowledge-architect@design` (one line) |
| `design@core@the-core-cli-is-a-library-module` | the list of the core's commands gains `init` and `new-component`; every binary that dispatches the command enum dispatches `init` before `locate`, through `cli::init`, as the core's main, the xtask snippet and the crate documentation's example do, so `init` keeps one contract in every binary | updated at step 1: the crate documentation's example in `path@core@src/lib.rs` and the xtask snippet; at step 5: `path@core@README.md`, "runs the same commands unchanged"; judged at step 6: `path@core@tests/binary.rs` (its head comment) |
| `design@core@phases-gate-the-report` | "a writer refuses over an incomplete model": `new-component` passes through the writer gate; `init` writes where no model exists, and its no-loss guarantee is its destination check instead; it writes the indexes only once phases 1 to 3 find nothing | updated at step 5: `path@core@README.md`, the paragraph listing the commands that run the first three phases before writing. Judged at step 6: `path@core@CLAUDE.md`; `issue@core@installed-file-findings-belong-in-phase-four`; `spec@plans@path-quickfixes`, whose citations are about `--fix`; the other referencing texts cite the phase order, which does not change; `tripwire@core@phases-gate-the-report-two` is re-read there |
| `design@core@nothing-of-a-project-is-compiled-in` | the bullet on what the word Component means gains the starting text of each document the commands write; the bullet on what this tool writes gains the maintenance crate's files `--rust` writes, which are the setup skill's snippets | judged at step 6: `path@core@CLAUDE.md` and `path@core@README.md`, which restate the head's first sentence, unchanged by the two bullets |
| `design@agent-skills@exact-pin` | "The setup skill writes the key": `init` writes it, and the setup skill moves it | judged at step 6: `register@knowledge-architect@design`, one line |
| `design@agent-skills@pinned-binary-runs-inside-the-project` | `init` runs before any pin exists, from whatever binary the user installed, such as a user-scope install; every later command runs the pinned checker as the head says | updated at step 5: the setup skill's section `skill@knowledge-architect-setup@pin-the-checker`, which gains no change unless it states that no machine-wide binary runs at all |

**The design entries the discussion's search graded as possibly changed by the work, with their
outcome:**

| entry | outcome |
| --- | --- |
| `design@core@a-foreign-build-is-refused` | not changed: `init` runs it over PATH, as above |
| `design@core@check-fix-flag` | not changed: `index` and `install-agent-skills` stay the commands for their one fix; `init` calls their code once, at creation, and answers another question, so `design@core@one-question-one-command` holds |
| `design@core@owned-namespace-check` | not changed: the install, run by `init`, still edits nothing outside the namespace; `init` itself writes the root CLAUDE.md only where none exists, at the user's request, and never edits one |
| `design@knowledge-architect@binary-bundles-workflow` | not changed: `install-agent-skills` stays the one command that installs; `init` runs the same code once |
| `design@knowledge-architect@stays-at-zero-x` | not changed: the issue it lists, `issue@agent-skills@a-skill-for-creating-a-component`, stays open |
| `design@agent-skills@adopting-existing-docs` | not changed: `init` over an existing project creates the homes and keeps every existing document; moving the existing documents stays planned work |
| `design@agent-skills@expectation-set-bounds-scope` | not changed: the setup skill's assumptions about the owner and the project's shape hold for a project `init` created |
| `design@xtask@one-module-per-subcommand` | not changed: it governs this repository's xtask; the template is the setup skill's snippet |

**The standing entries the discussion's searches returned, with their outcome:**

| entry | outcome |
| --- | --- |
| `issue@agent-skills@a-skill-for-creating-a-component` | bears: `new-component` creates the files and leaves the issue open; the setup skill points to the command, step 5 |
| `issue@core@a-component-states-at-least-one-goal` | bears: the commands create empty goals homes, as M2 shows the check accepts; unchanged |
| `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked` | bears: closed by step 4, per #rust-templates-one-home |
| `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | bears: the commands take the document list from required documents, so they follow that work |
| `tripwire@core@phases-gate-the-report-two` | bears: re-read at step 6; `init` adds no flag around the gate, and `new-component` passes through it |
| `tripwire@core@fix-makes-a-choice` | bears on nothing: no creation runs under `--fix` |
| `issue@knowledge-architect@command-output-is-not-declared-a-contract` | bears: the commands' printed lines are not declared a contract; their changelog entries say what they do, not what they print |
| `issue@core@tooling-for-project-skills` | bears on nothing built: the manifest `init` writes declares no prefix field, which does not exist |
| `issue@core@configuration-for-several-agent-providers` | bears on nothing: `init` writes the default harness |
| `issue@core@a-missing-root-document-finding-names-a-repair-that-does-not-apply` | bears on nothing: a tree `init` wrote holds every root document |
| `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text` | bears on nothing: `init` writes no gate and no instruction about commits |
| `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` | bears on nothing: the move of existing documents is not this work |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | bears on nothing: `init` is no upgrade, and the root CLAUDE.md it writes restates nothing |
| `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` | bears on nothing: `init` writes no harness settings |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | bears: the starting texts are shipped into every project, so they hold no reference |
| `issue@core@a-kind-name-refusal-quotes-a-future-migration-reason` | bears on nothing: the commands read the manifest's complaint, whatever its text |
| `issue@agent-skills@expectation-sets-for-the-installed-skills` | bears on nothing: no expectation set changes |
| `issue@core@installed-file-findings-belong-in-phase-four` | bears on nothing: `init` writes the installed set before any check |
| `issue@core@a-project-workflow-can-declare-an-installed-workflow-s-name` | bears on nothing: `init` changes no name ownership |
| `issue@core@core-heads-bundle-decisions-with-many-citing-sites` | bears: it names `design@core@owned-namespace-check` as a bundle due for a split when touched; this work does not touch that head |
| `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head` | bears on nothing: the import line the starting text writes is the constant `agents::IMPORT_LINE`, not a span |
| `issue@core@references-are-read-in-markdown-and-rust-only` | bears on nothing: the commands write Markdown and TOML |
| `tripwire@core@issue-kind-list-grows` | bears on nothing: the manifest written declares no issue kinds |
| `tripwire@agent-skills@structure-rejection-overruled` | bears on nothing: no instruction was rejected on the structure goal |
| `tripwire@agent-skills@axis-not-run-at-a-pin-move` | bears on nothing: the setup skill's pin-move step does not change |
| `tripwire@agent-skills@agent-text-written-without-the-standard` | bears: steps 4 and 5 edit the setup skill, each after loading `skill@knowledge-architect-agent-configuration` |

**The design and `rejected` entries graded `bears`** are the decisions above, and these, with their
outcome: `rejected@core@claude-md-in-every-component` (not brought back: CLAUDE.md is written only
under the `claude` harness); `rejected@core@plans-directory-per-component` (not brought back:
`new-component` creates no plans directory); `rejected@gates@each-project-writes-its-runner` (not
brought back: the generated crate depends on the gates library); `design@xtask@gates-list-primary-home`
(bears on nothing: it governs this repository's own xtask, not the template);
`design@agent-skills@content-mirrors-the-install-layout` (bears: the templates live in the snippets
directory, not in content/, which holds what the install writes);
`design@agent-skills@shipped-text-cites-no-entry` (bears: the templates hold no reference);
`design@agent-skills@audit-triggers` (bears on nothing: `init` adds no audit occasion);
`design@core@declared-command` (bears: a project created through an extension binary needs its own
command declared, D5).

## Criteria

### A new project reaches a passing check by following the documentation alone `##adoption-by-documentation`

Binding, from `goal@knowledge-architect@any-project-can-adopt-it` and
`goal@knowledge-architect@adoption-is-easy`. Met by #init-command and #init-delivery.

### A Rust project gets its gates without designing them `##gates-without-design`

Binding, from `goal@knowledge-architect@setup-brings-quality-tools`. Met by #rust-scaffold for the
maintenance crate and its gates command; the CI the goal also lists stays reached through the setup
skill's CI block, which #rust-templates-one-home puts under a test.

### Nothing about one project is compiled in, and the Rust parts are optional `##nothing-project-specific`

Binding, from `design@core@nothing-of-a-project-is-compiled-in` and
`goal@agent-skills@installed-text-works-anywhere`. Met by #init-file-contents and #rust-scaffold.

### No command writes goal text `##no-goal-written`

Binding, from `goal@knowledge-architect@the-owner-decides` and
`design@agent-skills@goals-change-through-goal-setting`. Met by #init-file-contents.

### No run loses content, and every destination is checked before any is written `##no-content-lost`

Binding, from the argument of `design@core@index-takes-no-dry-run`. Met by #init-confirmation,
#init-existing-files, the manifest edit of #component-manifest-edit, and the append rule of
#rust-scaffold.

### Each command runs without a terminal `##runs-without-a-terminal`

Binding as a presumption, from `design@core@arguments-parse-through-clap`, "These binaries run
blind". Met by #init-confirmation.

### The pin written is the binary's own version, and so are the maintenance crate's dependencies `##pin-is-own-version`

Binding, from `design@knowledge-architect@version-lockstep` and `design@agent-skills@exact-pin`.
Met by #init-file-contents for the manifest's pin, and by #rust-scaffold for the dependencies.

### The documents a Component carries, and the index bytes, come from the code the check uses `##one-document-list`

Binding as a presumption, from `design@core@one-question-one-command` and
`design@core@one-list-of-generated-files`. Met by #component-command and #init-file-contents.

## Threads

### `init` creates a project: the manifest, the installed files, the documents and the indexes `##init-command`

Proposed by the owner, round 1, as a `bootstrap` subcommand; shaped by the session, round 1. Final
state: approved. The `check --fix` variant was ruled out on `design@core@safe-fix-definition`
(#arg-2); the status quo loses on #arg-3. Shape: "Decided design", "What a run of `init` writes,
and what it refuses". Harvest: a design head of core. The owner, round 2: "init-command,
init-name-and-path accepted (agreed that init is a better name)."

### The command is `klarch init [PATH]`, with the name defaulting to PATH's basename `##init-name-and-path`

Proposed by the session, round 1 (#arg-4), against the owner's `bootstrap <name>`. Final state:
approved. Shape: "Decided design", "The command line". Harvest: a design head of core. The owner,
round 2: "init-command, init-name-and-path accepted (agreed that init is a better name)." and "Q1:
init is good."

### `init` asks for no confirmation; it refuses an existing manifest at PATH and overwrites no hand-written file `##init-confirmation`

The owner proposed a Y/N confirmation, round 1; the session argued against it, round 1 (#arg-5).
Final state: approved on the session's argument; the owner asked that `init` match `cargo init`'s
habits, and the session measured that `cargo init` asks no confirmation (#arg-21). Shape: "Decided
design", "What a run of `init` writes, and what it refuses". Harvest: a design head of core, under
the slug `init-asks-no-confirmation`, since the thread's name describes the question rather than
the decision: `#init-confirmation → init-asks-no-confirmation`. The owner, round 2: "Q4: your
argument hold. I think it is important to match usage habits of cargo init, does it not ask for a
confirmation ?"

### An existing README or CLAUDE.md is kept, so `init` serves an existing project `##init-existing-files`

Proposed by the session, round 1 (#arg-6). Final state: approved. Shape: "Decided design", "What a
run of `init` writes, and what it refuses", item 2. Harvest: a design head of core,
`init-keeps-existing-files`, apart from #init-confirmation's, since the two lost to different
rivals: `#init-existing-files → init-keeps-existing-files`. The owner, round 2: "init-existing-files:
approved." and "Q2: it can serve an existing project that I would want to migrate for example."

### `init` runs `git init` outside a worktree only into an empty or new directory, unless `--vcs git` asks `##init-runs-git-init`

Proposed by the session, round 1 (#arg-7); reopened by the session, round 2, on a premise the
owner's acceptance rested on that the session's measurement showed false (#arg-20); closed on the
owner's choice of shape B over shape A. Final state: approved. Shape: "Decided design", "Git".
Harvest: a design head of core; tripwire T1. The owner, round 2: "init-runs-git-init: well this
solves the problem I mentionned above I guess. If cargo init does it too, we can follow the same
model. It is probably best to mimic cargo init as much as possible." Round 3: "Q5: B". The owner's
own guard of round 2, a refusal wherever PATH is not a repository's root, was not put to the owner
again after the correction: D3.

### The files `init` writes are M2's tree, with an empty goals home and a CLAUDE.md of a title and the import line `##init-file-contents`

Proposed by the session, round 1 (#arg-8). Final state: approved. Shape: "Decided design", "The
files" and its mapping table. Harvest: the rewrite of `design@core@nothing-of-a-project-is-compiled-in`,
which holds that the starting texts are compiled in; the core head on `init` cites it. The owner,
round 2: "init-file-contents agreed".

### `new-component <DIR>` writes a Component's documents and homes from the check's own list, and `init --component` runs it `##component-command`

Proposed by the owner, round 1 ("we could provide a command to create a component from scratch
too"); shaped by the session, round 1 (#arg-9). Final state: approved. Shape: "Decided design",
"`new-component`". Harvest: a design head of core; tripwire T2. The owner, round 2:
"component-command: agreed. Indeed, the issue is about content and scope of a component, not just
basic structure."

### The manifest is edited with a format-preserving TOML editor, and validated before it is written `##component-manifest-edit`

Proposed by the session, round 1 (#arg-10). Final state: approved. Shape: "Decided design",
"`new-component`". Harvest: a design head of core, by entry test 3, since its argument turns on the
`toml` crate's behaviour, M6. The owner, round 2: "component-manifest-edit: agreed."

### `init --rust` writes a virtual workspace, the maintenance crate, its aliases and `/target` in `.gitignore`, and no CI `##rust-scaffold`

Proposed by the owner, round 1 ("a basic Cargo.toml, .cargo/config.toml, even tools/xtask
possibly"); shaped by the session, round 1 (#arg-11). Final state: approved. Shape: "Decided
design", "`--rust`". Harvest: a design head of core. The owner, round 2: "rust-scaffold: agreed."
and "Q3: no."

### The maintenance crate's blocks have one home, the snippets directory, which the setup skill inlines and `--rust` writes `##rust-templates-one-home`

Proposed by the session, round 1 (#arg-12). Final state: approved. Shape: "Decided design", "The
templates". Harvest: a design head of agent-skills; closes
`issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked`. The owner, round 2:
"rust-templates-one-home: agreed."

### `init` is documented in the core's README and run from a binary the user installed; the setup skill names the commands `##init-delivery`

Proposed by the session, round 1 (#arg-13); the owner gave the use case, round 2 (#arg-15,
#arg-19). Final state: approved. Shape: "Decided design", "Delivery". Harvest: the rewrites of
`design@agent-skills@exact-pin` and `design@agent-skills@pinned-binary-runs-inside-the-project`.
The owner, round 2: "init-delivery: see my use case described above. An agent could still need it
to init another project from an existing one, but this is not really the point and would rarely
happen. I would run the init from a global user scope install of the klarch binary that I did
myself."

### An extension binary that runs `init` gets a manifest without its tables, and the check reports it `##init-under-an-extension`

Proposed by the session, round 1 (#arg-14). Final state: closed by the session at the round-2
checkpoint, reading the owner's round-3 answers as the batch confirmation of the table that listed
it; the owner's round-3 words answered Q5 and Q6 alone, and the owner confirmed it in round 4,
D4, adding that a project with an extension would not need to run `klarch init` anyway.
Shape: "Decided design", "Extension binaries". Harvest: the rewrite of
`design@core@the-core-cli-is-a-library-module`, and a design head of core on the absence of an
extension hook, by entry test 2. The owner, round 3: "Q5: B\nQ6: all
five."

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
`git clean` or a delete removes." The last sentence was corrected by #arg-20.

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

Round 2, the session. Bears on #init-confirmation, #rust-scaffold. The measured behaviour under
"What the work is"; members admitted by #arg-17: `--vcs <git|none>`, and the append of `/target`
to an existing `.gitignore`.

## New names, in one place

```text
crates/core/src/cli/mod.rs
  Command::Init(InitArgs)                 the `init` subcommand
  Command::NewComponent(NewComponentArgs) the `new-component` subcommand
  InitArgs { path: Option<PathBuf>, name: Option<String>, component: Vec<PathBuf>,
             rust: bool, vcs: Option<Vcs>, command: Option<String> }
                                          `command`, per D5
  NewComponentArgs { dir: PathBuf }
  Vcs { Git, None }                       the value of `--vcs`
  init(args: InitArgs, libraries: &[Library], packages: &[&str], checker: &[&Path],
       extensions: &mut [Box<dyn Extension>]) -> Result<ExitCode, String>
                                          runs `init` with no located manifest, with the same
                                          `libraries` and `packages` a binary hands the
                                          build-origin refusal, and the same `checker` and
                                          `extensions` it hands `run`; public, so every binary
                                          dispatches it before `locate`
crates/core/src/cli/create.rs             (new) the writer both commands share: the destination
                                          check, the starting text of each document, the
                                          manifest edit, the `.gitignore` append
crates/core/Cargo.toml                    dependency `toml_edit`
crates/agent-skills/snippets/
  workspace-cargo.toml                    (new) the virtual workspace's Cargo.toml, a new block
                                          of the setup skill's Rust section
  xtask-cargo.toml                        (new) the maintenance crate's Cargo.toml, moved from
                                          the setup skill's block
  cargo-config.toml                       (new) the aliases and the `[env]` tie, moved
  ci.yml                                  (new) the CI workflow, moved
  xtask-main.rs                           existing
crates/agent-skills/src/lib.rs
  SNIPPETS: &[(&str, &str)]               (new) each snippet file's name and text, generated by
                                          the build from the directory, as FILES is
```

`path@core@src/cli/create.rs`, `planned@agent-skills@snippets/workspace-cargo.toml`,
`planned@agent-skills@snippets/xtask-cargo.toml`, `planned@agent-skills@snippets/cargo-config.toml`,
`planned@agent-skills@snippets/ci.yml`.

## Decided design

### The command line

```text
klarch init [PATH] [--name <NAME>] [--component <DIR>]... [--rust] [--vcs <git|none>]
            [--command <CMD>]
  PATH         defaults to "."; created, with its parents, when it does not exist
  --name       defaults to PATH's basename
  --component  a Component to add, as `new-component` adds it; repeatable; DIR is relative to PATH
  --rust       the maintenance crate, its aliases and its workspace
  --vcs        whether `git init` runs; the default is the rule under "Git"
  --command    the project's command, written as [project] command; `--rust` defaults it to
               `cargo klarch`; absent otherwise, which means `klarch`, per D5

klarch new-component <DIR>
  DIR          relative to the working directory, and inside the project's root; written into
               the manifest relative to the root, with `/` separators
```

The nearest rival, `bootstrap <name>`, names the same act in a word no Rust user already types for
it (#arg-4).

### What a run of `init` writes, and what it refuses

1. **It refuses, exit 2, having written nothing**, with each reason named:
   - PATH holds `knowledge-architect.toml`. A manifest above PATH does not refuse: a project nested
     in another is legal, and the outer project's `[walk] exclude` row keeps it out of that walk;
   - the manifest it would write has a non-empty `Manifest::complaints()` (a kind name, a reserved
     anchor word, a collision), or fails to parse;
   - the name begins with `knowledge-architect-`: the project's skill prefix would be the
     installer's namespace, whose files the install deletes, per
     `design@agent-skills@project-name-prefix`. This is a new rule of `init`;
   - a `--component` would fail as `new-component` fails, or `--rust` refuses;
   - the git rule below refuses;
   - the build-origin refusal refuses over PATH, where PATH exists;
   - the destination check of item 2 refuses.
2. **It checks every destination before writing any.** A hand-written document that exists as a
   file is **kept**, not written, and listed as `kept`. A destination that is a directory where a
   file is due, or a symlink, refuses the run. The installer's namespace is the install's: the
   install writes and deletes there as `install-agent-skills` does, since those files are the
   tool's, per `design@core@owned-namespace-check`, and each is listed as `wrote` or `deleted`. So
   no hand-written file is overwritten (#arg-5, #arg-6).
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

4. **Its order**: the refusals; PATH created; `git init` where the git rule runs it; the manifest;
   the install, through the install code; the documents and homes; under `--rust`, its files and
   the `.gitignore`; then, once phases 1 to 3 find nothing over the written tree, the indexes
   through the index writer, and the check. This is the order of `design@core@fix-before-the-checks`
   after the creation.
5. **Its exit code** once it has written: 0 when the final check passes; 1 when phases 1 to 3 stop
   the indexes or the check reports findings, such as a kept CLAUDE.md that lacks the import line,
   with the check's report printed; 1 also when the check cannot run, with its reason, since files
   were written and exit 2 promises an untouched tree. Outside every worktree under `--vcs none`,
   no index and no check run: the output says to run `git init` then `klarch check --fix`, and the
   exit code is 0.

### Git

- PATH inside a git worktree: no repository is created, whatever the directory holds (M3), unless
  `--vcs git` asks. Then `git init` runs at PATH, and the outer project's check reports the nested
  repository in phase 2 unless its `[walk] exclude` keeps it out.
- PATH outside every worktree, and new or empty: `git init` runs, as `cargo init`'s does.
- PATH outside every worktree, and holding any file: `init` refuses, exit 2, naming the directory
  and `--vcs git`, unless `--vcs git` or `--vcs none` is given. This is the one departure from
  `cargo init`, on the owner's choice of shape B: an accidental run in such a directory would leave
  the user's own files untracked beside the new ones, where `git clean` deletes both (#arg-20).
- `--vcs none` never runs `git init`.

Whether PATH is inside a worktree is asked through `path@core@src/git.rs`, with
`git rev-parse --is-inside-work-tree` run at the nearest existing directory at or above PATH; the
module gains it and `git init`.

### The files

The writer asks required documents and the anchors for a Component's documents and homes, so a
register declared later, or a change to what a Component carries, reaches the commands with no edit
to them. The plans README is the one root document no register home names: the writer takes it
from the constant the check's own rule uses. A heading register's home is written as a file,
`<dir>.md`, one of the two shapes of `design@core@heading-register-two-shapes`. The index writer
runs after the homes exist, since it writes no index for a missing directory.

The starting text of each, by mapping table:

| document | its text, as an illustration |
| --- | --- |
| the manifest | `[project]` with `name`, `checker-version` set to the binary's `CHECKER_VERSION`, and `components` listing every `--component` (and `xtask` under `--rust`); `[walk]` with three empty lists, as the setup skill's smallest manifest |
| a Component's README.md, the root's included | `# <name>`, then "What <name> is for, and how to use it: not written yet." |
| the root CLAUDE.md | `# <name>`, then the import line alone, `agents::IMPORT_LINE` |
| another Component's CLAUDE.md, under the `claude` harness | `# <name>`, then "The contracts and traps a developer of <name> needs: none recorded yet." |
| the goal home | `# Goals`, then "What <anchor> is for. No entry yet: the goals are stated with the owner." |
| the design home | `# Design`, then "How <anchor> is built, and why. No entry yet." |
| the rejected home | `# Rejected designs`, then "The designs <anchor> did not keep, and why. No entry yet." |
| the tripwire home | `# Tripwires`, then "The evidence that would reopen a decision of <anchor>. No entry yet." |
| the issue directory's README.md | `# Open issues`, then "One file per outstanding item of <anchor>. No entry yet." |
| a declared register's home | `# <register name>`, then "The entries of the <register name> register. No entry yet." |
| the plans README | `# Plans`, then "The plan documents of work designed and not yet built." |
| docs/plans/specs/README.md and docs/plans/milestones/README.md | `# Specs` and `# Milestones`, each then "No entry yet." |

The texts hold no reference, no link and no heading at a register's level, so they pass every
grammar check in any project. Nothing writes a goal. No directory of audit reports or of
retrospective analyses, and no roadmap, is written: each is created when first needed.

### `new-component`

1. It runs through `run`, after `locate`, the build-origin refusal and the version refusal, and
   through the writer gate: over an incomplete model it refuses, exit 2, writing nothing.
2. It edits the manifest with `toml_edit`, appending `<DIR>` to `[project] components` and keeping
   every other byte, comments included. It reads the edited text with `Manifest::parse` before
   writing anything, and refuses, exit 2, when the parse fails or its complaints gain one, quoting
   the complaint.
3. It writes the Component's required documents and the home of every register it carries, by the
   rules of "What a run of `init` writes", item 2, and the texts of "The files"; CLAUDE.md only
   while the project serves the `claude` harness. An `opt-in` register a Component should carry is
   not guessed: the user names the Component in that register's `components`, and the check
   reports the home.
4. It runs the index writer, then the check, and exits with the check's 0 or 1.

`init --component <DIR>` runs step 3 alone: the manifest `init` writes already lists DIR, so step 2
would list it twice.

### `--rust`

It refuses, exit 2, writing nothing, when PATH holds a `Cargo.toml`. Otherwise it adds:

```text
Cargo.toml                 the virtual workspace, members = ["xtask"]
xtask/Cargo.toml           the maintenance crate, both crates pinned "=<CHECKER_VERSION>"
xtask/src/main.rs          the xtask snippet, with <PROJECT> replaced
.cargo/config.toml         the aliases x and klarch, and the [env] tie
.gitignore                 "/target"; appended under "# Added by klarch" to an existing one,
                           every existing line kept; nothing written when a line of it is
                           already `/target`, `/target/`, `target` or `target/`
manifest                   command = "cargo klarch"; "xtask" in components
xtask/README.md, CLAUDE.md, docs/...   the Component's documents, as new-component writes them
```

`<PROJECT>` is the name upper-cased with each `-` replaced by `_`, as this repository's
`KNOWLEDGE_ARCHITECT_CHECKOUT`. No CI file is written; the setup skill's CI block is the route to
it.

### The templates

The setup skill's Rust section gains one block, the virtual workspace's Cargo.toml, which shows the
workspace its prose already describes. Its three blocks other than the main move into the snippets
directory beside the xtask snippet, and the section inlines all five with `{{snippet:<file>}}`. The
agent-skills build generates `SNIPPETS` from the directory, as it generates `FILES`, and `--rust`
writes four of them, substituting `<version>` and `<PROJECT>`; `ci.yml` it does not write. One text,
two readers: the skill shows what `init --rust` writes, and the CI block besides.

The snippets directory's contract widens from "code a workspace target compiles" to "text a test
checks", which the build's header comment and `path@xtask@tests/snippets.rs` state:

- a `.rs` snippet is included by an example of the xtask crate, as now;
- the three TOML snippets are checked by AC2, which builds a project `init --rust` wrote;
- `ci.yml` is checked by a test in the xtask crate that takes the words after `gates` on its line
  holding `x gates`, and parses them with the gates library's `GatesArgs` through clap's
  `try_parse_from`.

### Delivery

- The core's README, `path@core@README.md`, gains an "Adopting the workflow" passage: install a
  binary the user keeps, for instance `cargo install --locked knowledge-architect`, run
  `klarch init` in the project, then start the agent session, whose setup skill takes over.
- The setup skill's opening names `init` as how the manifest is made, and its Components section
  names `new-component` as how a Component is added; its Rust section says `init --rust` writes the
  blocks it shows except the CI workflow. Its sentence on the version refusal gains the exception
  of `init`. Each edit loads `skill@knowledge-architect-agent-configuration` first.
- After `init`, the project runs its pinned checker as before: `cargo klarch` through the
  maintenance crate, or an install under its own root. A user-scope binary of another version is
  refused by the version refusal, which names both versions.

### Extension binaries

Every binary that dispatches the command enum dispatches `Command::Init` to `cli::init` before
`locate`: the core's main, the xtask snippet, and the crate documentation's example of an extension
binary. `run` refuses `Command::Init`, exit 2, with a message naming `cli::init`, so a binary that
skips the step says why instead of acting. The manifest `init` writes holds
no extension's table, and the first check reports each, with its repair (#arg-14). Its `command` is
the one `--command` gives, per `design@core@declared-command` and D5; a project with an extension
starts from the core's own `klarch init` in practice, on the owner's word, D4.

## Mapping tables

The command line to the files written, over every combination of the flags that write files:

| flags | root documents and homes | the shipped set | `--component` Components | the maintenance crate and the xtask Component | `.gitignore` |
| --- | --- | --- | --- | --- | --- |
| none | written | written | none | none | untouched |
| `--component D`... | written | written | each D | none | untouched |
| `--rust` | written | written | none | written | `/target` ensured |
| `--rust --component D`... | written | written | each D | written | `/target` ensured |

`--vcs` changes whether `git init` runs and, outside a worktree, whether a non-empty PATH is
refused; `--name` changes only the name. The shipped set is written only under the `claude`
harness, which is the only harness `init` writes.

The git rule, over every case:

| PATH | no `--vcs` | `--vcs git` | `--vcs none` |
| --- | --- | --- | --- |
| inside a worktree | no `git init` | `git init` at PATH | no `git init` |
| outside, new or empty | `git init` | `git init` | no `git init`; no index, no check |
| outside, holding files | refused, exit 2 | `git init` | no `git init`; no index, no check |

The starting text of each document is the table under "The files".

## Losing alternatives

- **`check --fix` creating the missing homes**, ruled out in #init-command on
  `design@core@safe-fix-definition`: a README is neither an installed nor a generated file (#arg-2).
- **The name `bootstrap <name>`**, lost to `init [PATH]` in #init-name-and-path on the owner's word
  (#arg-4).
- **A Y/N confirmation, with `--yes` for scripts**, lost in #init-confirmation: `cargo init` asks
  none, an agent has no terminal, and nothing hand-written is overwritten (#arg-5, #arg-21).
- **Refusing when any destination exists**, lost in #init-existing-files: `init` would then serve
  only an empty directory (#arg-6).
- **Shape A, `git init` in any directory outside a worktree, as `cargo init`**, lost in
  #init-runs-git-init on the owner's choice of shape B (#arg-20).
- **A refusal wherever PATH is not a git repository's root**, the owner's own proposal in round 2
  (#arg-16). The owner set it aside in the same message on the premise that `git init` makes
  cleanup easy, which #arg-20 then showed false, and it was not put to the owner again: D3. The
  session's argument against it: a refusal at a subdirectory of a repository would refuse a layout
  M3 shows the check accepts. Shape B keeps its protection for a directory that holds files, and
  T1 brings back its stronger form, a refusal outside a worktree whatever the directory holds, on
  one report of a messy cleanup.
- **Printing the manifest line for the user to add**, lost in #component-manifest-edit: the command
  then cannot finish in one run (#arg-10).
- **A CI workflow written by `--rust`**, ruled out by the owner, Q3 (#arg-11).
- **Templates kept apart from the setup skill's blocks**, lost in #rust-templates-one-home: a fourth
  unchecked copy (#arg-12).
- **An extension hook contributing tables to `init`**, lost in #init-under-an-extension: the
  finding names its repair (#arg-14). Ruled by the owner, D4.

## Readings

`cargo init`'s behaviour, measured with cargo 1.98.0 and restated with its commands under "What the
work is", is the external behaviour #init-confirmation, #init-runs-git-init and #rust-scaffold
follow. It is recorded in the harvest's core head on `init`, with those commands.

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
knowledge-architect-gates at this workspace's crates, and runs `cargo build --offline -p xtask` and
`cargo klarch check` there, the offline build resolving clap from the cache this workspace's own
build fills; it fires when either fails. Response: reopen #rust-templates-one-home. Its place in the
test suite awaits the owner, D2.

### AC3: `init` then `check` passes, root only, with a Component, and with `--rust` `##init-then-check-passes`

Guards #init-file-contents. Judged at steps 2, 3 and 4. Observable: a test runs `init` in a scratch
repo in each of the three shapes, then `check`; it fires when `check` does not print
`PASSED: no findings`. Response: reopen #init-file-contents.

## Implementation sequence

### Step 1: `init` writes the manifest alone, behind every refusal

Builds: `Command::Init`, `InitArgs`, `Vcs`, `cli::init`; its dispatch before `locate` in the core's
main, the xtask snippet and the crate documentation's example; the refusal of `Command::Init` in
`run`; the refusals of "What a run of `init` writes", item 1; the git rule, with the two git
functions; the manifest text, with `--command`. The comments that state the version refusal's timing, listed under
"Decisions this work rewrites", gain the exception of `init`. No document, no install.

Claims:
- in a scratch repo, `init` writes `knowledge-architect.toml` whose pin is `CHECKER_VERSION`, and
  `check` then exits 1 in phase 2 with the 8 missing documents of M1 among its findings; shown to
  fail by writing another version;
- `init` in a directory holding a manifest exits 2 and writes nothing; under a directory above that
  holds one, it writes;
- a name `design`, `plans`, `elsewhere` or `knowledge-architect-x` exits 2 and writes nothing;
- each row of the git table holds, each in a scratch directory;
- stdin from /dev/null changes nothing;
- the clap declaration passes `debug_assert` in the core's binary.

Fails alone on: the manifest's text or the git rule.

### Step 2: `init` writes the root's documents, the shipped set and the indexes

Builds: the starting texts, the destination check over documents, the install through the install
code, the index writer call, the final check and the exit codes of item 5.

Claims: AC3 in its root-only shape; an existing README.md and CLAUDE.md are kept byte for byte and
listed as `kept`, and a kept CLAUDE.md without the import line makes `init` exit 1 with the check's
finding; a symlink or a directory at a destination refuses the run with nothing written; under
`--vcs none` outside a worktree, `init` writes the documents and exits 0 with the note.

Fails alone on: a document's text, or a home the anchors list and the writer misses.

### Step 3: `new-component`, and `init --component`

Builds: `Command::NewComponent`, the `toml_edit` dependency and the manifest edit, the writer gate,
`--component`.

Claims: AC1; AC3 in its Component shape; over a tree with a phase-2 finding, `new-component` exits 2
and writes nothing; a DIR whose basename is the project's name exits 2 quoting the manifest's
complaint and writes nothing; `init --component D` lists D once.

Fails alone on: the manifest edit.

### Step 4: the snippets, `SNIPPETS`, and `--rust`

Builds: the four new snippet files, the setup skill's blocks replaced by `{{snippet:<file>}}` and
its new workspace block, after loading `skill@knowledge-architect-agent-configuration`; the widened
contract in the build's header and in `path@xtask@tests/snippets.rs`; `SNIPPETS`; the `--rust` flag
and the `.gitignore` append; the test of the CI block's gates command.

Claims: AC2, with its run time measured and reported in the step's commit; AC3 in its `--rust`
shape; the installed setup skill differs from before the move only by the new workspace block,
compared with `diff` of the two installed copies; an existing `.gitignore` keeps every line, gains
`/target` once, and is untouched when it already holds one of the four forms; a PATH holding a
`Cargo.toml` exits 2 and writes nothing.

Fails alone on: a snippet's text, or the build of the generated crate.

### Step 5: the documentation and the changelog

Builds: the core README's passage and its two updated paragraphs; the setup skill's new sentences,
after loading `skill@knowledge-architect-agent-configuration`; the changelog entries: `cli`, minor,
for each command; `library`, minor, for `cli::init`, `InitArgs`, `NewComponentArgs`, `Vcs` and the
two variants of the command enum, and for `SNIPPETS`; a Workflow entry for the setup skill; then
`cargo x changelog`.

Claims: `cargo klarch check` passes; the agent-facing-text review axis finds each added sentence
names an act.

Fails alone on: the documentation.

### Step 6: the harvest

Per the harvest below.

## Order rationale

- Step 1 before step 2: the refusals and the git rule are judged before anything but the manifest
  can be written.
- Step 2 before step 3: `new-component` reuses the writer and the starting texts step 2 lands.
- Step 3 before step 4: `--rust` adds a Component through the code step 3 lands.
- Step 4 before step 5: the setup skill's text names what steps 1 to 4 built.
- Step 5 before step 6: the harvest records what the documentation states.

## Defaults awaiting the owner

All five ruled, round 4: "All five defaults accepted. Note on D4: a project with an extension would
not need to run `klarch init` anyway." The work lands on the spec's own branch, in the same pull
request, on the owner's word in the same message.

- D1, ruled. The work is planned as one spec, on one branch, rather than a milestone. It bears on every
  thread.
- D2, ruled. AC2 runs in the default test suite. Default: it does, unless its run time measured at step 4
  exceeds 60 seconds on the owner's workstation, a threshold that is the owner's to reset; above it,
  it is marked `#[ignore]` and the owner rules where it runs. It bears on #rust-templates-one-home.
- D3, ruled. The owner's guard of round 2, a refusal wherever PATH is not a git repository's root, is
  recorded as lost to shape B and T1 on the session's argument (M3), not on the owner's word. It
  bears on #init-runs-git-init.
- D4, ruled. No extension hook contributes tables to `init`; an extension binary's `init` writes a
  manifest the check then reports. It bears on #init-under-an-extension.
- D5, ruled. `init` gains `--command <CMD>`, written as `[project] command`, which `--rust` sets to
  `cargo klarch` and an extension project sets to its own command, per `design@core@declared-command`.
  Default: it does. It bears on #init-under-an-extension and #init-name-and-path.

## Harvest

| what | where | when |
| --- | --- | --- |
| #init-command | a design head of core on `init`, with the reading of `cargo init` | step 6 |
| #init-name-and-path | a design head of core on `init`'s command line | step 6 |
| #init-confirmation | a design head of core, `init-asks-no-confirmation` | step 6 |
| #init-existing-files | a design head of core, `init-keeps-existing-files` | step 6 |
| #init-runs-git-init | a design head of core | step 6 |
| #init-file-contents | the rewrite of `design@core@nothing-of-a-project-is-compiled-in` | step 6 |
| #component-command | a design head of core on `new-component` | step 6 |
| #component-manifest-edit | a design head of core, by entry test 3 | step 6 |
| #rust-scaffold | a design head of core on `--rust` | step 6 |
| #rust-templates-one-home | a design head of agent-skills; closes `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked` | step 4 for the issue, step 6 for the head |
| #init-delivery | the rewrites of `design@agent-skills@exact-pin` and `design@agent-skills@pinned-binary-runs-inside-the-project` | step 6 |
| #init-under-an-extension | the rewrite of `design@core@the-core-cli-is-a-library-module`; a head of core on the absent hook | step 6 |
| the rewrites under "What is already decided" | each head named there | step 6 |
| the losing alternatives | `rejected` entries of core where `skill@knowledge-architect-decision-recording@losing-alternatives` admits them; the candidates are the Y/N prompt, shape A, the refusal outside a worktree whatever the directory holds, and the extension hook | step 6 |
| T1, T2 | `register@core@tripwire` | step 6 |
| AC1, AC2, AC3 | reported on in the landing commits, then proposed as tripwires or deleted with the spec | step 6 |

## Later consequences

- A check that a Component states a goal, `issue@core@a-component-states-at-least-one-goal`, would
  make a tree `init` writes fail until goal-setting has run: the setup skill's goal-setting step
  then becomes part of reaching a passing check.
- The skill on what a new Component owes, `issue@agent-skills@a-skill-for-creating-a-component`,
  will name `new-component` as its first act.
- Adding the maintenance crate to an existing Cargo workspace remains to design, if a project asks.
