# Goal batch of the review before the merge of the goal-standard branch, 2026-10-31

The record of the goal batch the review before the merge of this branch raised, kept under
`skill@knowledge-architect-goal-setting@drawing-out-intent` for a batch of more than three goals.
It leaves with the commit that writes the goals the owner rules on.

## What the owner rules on

- **Q81, the boundary blocks.** No block existed when the owner ruled the 31 goals in the commit
  "[goals] Step 6 lands: every goal of this repository meets the goal standard, laid out with no
  sentence bound, as the owner ruled it". The blocks below are every goal's, the goal reviewer's
  whole-set reading of the tree with the drafts below in it. The owner reads them, and confirms the
  goals or names those to reword.
- **Q84, the drafts.** The working tree holds, uncommitted, the rewordings of six goals and the
  parents of the two goals split off on this branch:
  - `goal@knowledge-architect@any-project-can-adopt-it`: the tree the crates are built from is set
    aside only for what is compiled in; for every other statement of the goals, this repository is
    a project that adopts, installs and follows the workflow and uses the checker;
  - `goal@knowledge-architect@the-workflow-improves-through-real-use`: where
    `goal@knowledge-architect@the-owner-decides` gives a change in the workflow to an owner, that
    owner is knowledge-architect's;
  - `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: the criteria include the plan
    documents the work answers, and the owner's statements and rulings it carries out, as far as each
    binds the writer; it refines `goal@knowledge-architect@agents-get-a-complete-workflow`;
  - `goal@agent-skills@installed-text-works-anywhere`: "the command the project declares for running
    the checker, or the checker's default where it declares none", with no command named;
  - `goal@agent-skills@installed-text-works-in-any-environment`: an environment's addition never
    takes an installed one's place "unless an installed instruction itself defers to it"; it refines
    `goal@knowledge-architect@agents-get-a-complete-workflow`;
  - `goal@agent-skills@only-needed-instructions`: it refines
    `goal@knowledge-architect@agents-get-a-complete-workflow`;
  - `goal@xtask@one-command-runs-every-gate`: kept as it stands, its list of gates being the owner's
    proposal.

The owner's answer to Q81 to Q84, the session's defaults: "agreed on defaults".

## The reviewer's findings on the drafts, and their outcomes

- `goal@knowledge-architect@any-project-can-adopt-it`: which goals the scope clause reached had
  several readings, and so had what the carve-out scoped; repaired by setting the tree the crates are
  built from aside for what is compiled in alone, and naming this repository a project that adopts,
  installs and follows the workflow and uses the checker for every other statement. Left as they
  stand, minor: the clause states another goal's scope (rule 4), and two statements are unmeasured
  (rule 6).
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: the sentence on whose owner
  decides holds its own intent (rule 4, minor) and is unmeasured (rule 6, minor), left as they stand;
  whether the admission rule its measure states is the owner's outcome or a mechanism of
  `design@agent-skills@behaviour-additions-need-real-use` is kept as the owner's outcome, the text
  being the owner's.
- `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: the owner's statements and rulings read
  as binding categorically, against `goal@knowledge-architect@the-owner-s-word-is-verified`;
  repaired by "as far as each binds the writer".
- `goal@agent-skills@installed-text-works-anywhere`: "the command by which the project runs the
  checker" read two ways for a project that declares none; repaired by "the command the project
  declares for running the checker, or the checker's default where it declares none".
- `goal@agent-skills@installed-text-works-in-any-environment`: its parent spoke of projects, not of
  environments, so it now refines `goal@knowledge-architect@agents-get-a-complete-workflow`; a step
  of an installed skill that defers to a user-level instruction, as the retrospective's directory
  does, is admitted by "unless an installed instruction itself defers to it". The line between a
  harness's mechanism and its instruction stays as Q67 drew it.
- `goal@agent-skills@only-needed-instructions`: "well enough" stays as the owner approved it; its
  reason requirement is unmeasured (rule 6, minor), left as it stands.
- `goal@xtask@one-command-runs-every-gate`: whether its six gates are the outcome stays the owner's
  proposal, as Q84 kept it; its three behaviours are unmeasured (rule 6, minor), left as they stand.

## Every goal's boundary block

Each block names, for the goal's universals and exclusions, a case it rules out and one it admits,
and the sibling goals it touches.

### docs/goals.md

- `goal@knowledge-architect@agents-work-without-drift`: rules out a branch that brings back
  `rejected@core@sentinels-accepted-everywhere` with no reversal; admits the same design reopened
  under `skill@knowledge-architect-design`. Rules out weighing
  `goal@agent-skills@only-needed-instructions` above it where an instruction is needed to catch
  drift. Siblings: ranks above every goal but the three owner goals and a goal's exclusion; refined
  by `goal@core@projects-add-their-own-checks`.
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: admits `design@core@declared-command`,
  recorded with its argument and its case; rules out a head with no rival named. Admits an inline
  comment and its commit for a choice that earns no head; rules out such a choice with its reason in
  neither. Siblings: served by `goal@knowledge-architect@structure-and-workflow-work-together` and
  `goal@knowledge-architect@records-live-beside-the-code`.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: admits each listed activity with its
  skill under .claude/skills/; rules out a listed activity with no skill. Rules out an installed skill
  on writing the code itself; admits the project's own `skill@klarch-development`. Siblings: refined
  by the goals on mechanical tasks, one skill per activity, no contradiction, the review criteria,
  the environment and only needed instructions.
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: admits `cargo klarch worklist` for the
  record search; rules out, as unmet today,
  `issue@core@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history` and
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`; admits a single
  `cargo klarch show` as no such task. Siblings: shares its scope with
  `goal@xtask@repeated-tasks-are-automated`.
- `goal@knowledge-architect@structure-and-workflow-work-together`: admits an issue in its register;
  rules out a kind of record the installed text asks for with no row of the knowledge table. Admits
  `cargo klarch show` and `cargo klarch tripwires`; rules out a list kept by hand. Siblings: refined
  by `goal@core@records-reach-their-reader`.
- `goal@knowledge-architect@records-live-beside-the-code`: admits crates/core/docs/open-issues/; rules
  out an issue tracker on GitHub as the home. Admits a retrospective's files outside the repository
  and their intake analysis in docs/retrospective-reports/.
- `goal@knowledge-architect@documentation-stays-consistent`: admits `skill@knowledge-architect-review`
  before a merge; rules out a merge with no review. Admits a reference in a Rust comment; leaves out
  a reference in a YAML file, which the checker does not read. Admits an empty list of harnesses
  switching the installed text's checks off. Siblings: refined by the three core goals on the check.
- `goal@knowledge-architect@the-owner-decides`: admits the order of known work and a publication;
  rules out asking the owner a reading a specification's text answers. Rules out the owner's bare
  word over a settled decision; admits a bare pick at a fork. Rules out a decision made in a turn with
  no "Decided" line and no durable trace. Siblings: bounds the drift goal.
- `goal@knowledge-architect@the-owner-s-word-is-verified`: admits goal-setting checking a draft's
  premises; rules out a review taking the owner's reading of the code unmeasured. Rules out reading a
  rewrap commit as a ruling.
- `goal@knowledge-architect@design-with-agents-as-counterparts`: admits a request for a new skill
  grounded under `skill@knowledge-architect-design`; a typo fix decides nothing. Admits a goal-setting
  draft as an exchange; rules out a "Decided" line as one.
- `goal@knowledge-architect@owner-attention-on-what-is-theirs`: admits a proposal put with its
  grounding and consequences; rules out asking the owner to find what is outstanding.
- `goal@knowledge-architect@any-project-can-adopt-it` (draft): rules out a compiled-in list of
  thaum's registers; admits this repository's crate directories and package names, set aside, and
  the "self" pin. Rules out an installed skill naming `cargo x gates` as every project's command;
  admits the setup skill offering it as a Rust project's recommended shape. Rules out a manifest key
  changing the reference separator. Rules out an install that moves the pin. Rules out dropping a
  check because thaum's documents fail it.
- `goal@knowledge-architect@rust-projects-come-first`: admits string literals of Rust source read as
  data; rules out removing `init --rust` because a Python project cannot use it, and a feature built
  for another language first.
- `goal@knowledge-architect@adoption-is-easy`: admits crates/core/README.md and the crate
  documentation of lib.rs; rules out an extension contract stated only in a design head. The rendered
  site is unmet today, per `issue@core@the-documents-do-not-render-as-a-site`.
- `goal@knowledge-architect@the-workflow-improves-through-real-use` (draft): admits a session in thaum
  on a contribution to thaum, and one in this repository; rules out a session in a scratch copy made
  to try a skill. Rules out a fixture project made to watch an agent follow a skill; admits the
  checker's mock projects and one dispatch checking a new agent's tools. Rules out thaum's owner
  ruling a fork of the installed text.
- `goal@knowledge-architect@setup-brings-quality-tools`: admits the setup skill's maintenance crate
  and CI shapes; rules out a Rust project left to write its own CI. For another language, admits the
  project building its own gates command.

### crates/core/docs/goals.md

- `goal@core@documents-stay-current`: admits a stale index failing the check; leaves out a project
  whose gates run no per-commit check. Admits `cargo klarch commits origin/main..HEAD`.
- `goal@core@document-rules-are-checked`: admits an empty list of harnesses and a walk exclusion;
  rules out a manifest key switching off reference resolution, per
  `design@core@the-regime-has-no-opt-out`, and the "self" pin outside the tree the checker is built
  from.
- `goal@core@relocation-is-one-manifest-edit`: admits moving tools/xtask/ by one manifest edit; rules
  out, as no move, renaming crates/gates/. Admits crates/core/docs/design/ as a directory home.
- `goal@core@records-reach-their-reader`: admits `cargo klarch show`; leaves out a reference in a
  file the checker does not read; rules out a list of open tripwires kept by hand.
- `goal@core@documents-render-as-a-linked-site`: rules out a site of other projects' documents; unmet
  today, no mdbook configuration being in the tree.
- `goal@core@projects-add-their-own-checks`: admits thaum's rules extension run through its declared
  command; rules out, as unmet today, an extension that needs to know which register a document
  belongs to, per `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`.

### crates/agent-skills/docs/goals.md

- `goal@agent-skills@one-skill-per-activity`: admits the eleven installed skills; rules out one skill
  bundling the retrospective and its intake. Rules out the review skill reading the design skill with
  no step that names it.
- `goal@agent-skills@no-contradiction-in-workflow`: admits the precedence `primer@question-authority`
  states; rules out two installed skills giving opposite orders at one step with no precedence.
- `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` (draft): admits a head written under
  `skill@knowledge-architect-decision-recording` and code written under `skill@klarch-development`;
  a typo fixed under no skill is outside it. Rules out a finding from a preference no criterion
  states. Admits the criteria of the plan document a branch answers, and rules out another plan's.
  Rules out the review and the audit giving divergent verdicts on one text.
- `goal@agent-skills@installed-text-works-anywhere` (draft): admits the setup skill's `<xtask-dir>/`
  placeholder; rules out an installed text naming crates/core/. Admits a Python project following the
  parts that depend on no language.
- `goal@agent-skills@installed-text-works-in-any-environment` (draft): admits reliance on the
  harness's agent dispatch and skill loader; rules out an installed skill that loads a plugin's
  skill. Rules out a skill reached only through a user-level instruction; admits the primer's list of
  installed skills. Rules out a plugin's skill replacing the design skill; admits the retrospective's
  deferral to a user-level directory, which the installed text itself names.
- `goal@agent-skills@only-needed-instructions` (draft): rules out an instruction that restates a
  reason with no act, and a rule of one repair commit per review finding; admits one file per
  received file in docs/retrospective-reports/, which no instruction needs to vary.

### crates/gates/docs/goals.md

- `goal@gates@gates-from-a-list`: admits tools/xtask/src/gates.rs, its list and the call; rules out
  a project's own log writer or distiller, and a project in another language.

### tools/xtask/docs/goals.md

- `goal@xtask@one-command-runs-every-gate`: admits `cargo x gates`, and CI's run of it with
  `--fail-fast` and `--require-rebased`; rules out a default that stops at the first failure, and CI
  steps that list the gates themselves.
- `goal@xtask@repeated-tasks-are-automated`: admits `cargo x changelog`; rules out, as the checker's,
  a task every adopting project meets; rules out, as unmet today, the reword of an older commit, the
  fold of a repair and the merge predicate, per
  `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`.
