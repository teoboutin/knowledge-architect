# Changelog

One section per released version, and one working section above them, `Next release`, which the
release renames to its version. Which change gets an entry, under which subsection, and with which
class is `design@knowledge-architect@changelog-entries`. Inside a subsection, entries are sorted by
surface, in the order `checks`, `cli`, `manifest`, `library`, `agent-skills`, `gates`; an empty
subsection is omitted.

## Next release

### Migration

- `library`, minor: a binary that dispatches the core's command enum, as the setup skill's
  maintenance crate and the crate documentation's extension example do, hands `Command::Init` to
  `cli::init` before `cli::locate`, with the libraries and packages it gives
  `cli::refuse_a_foreign_build`. Without that arm the binary still compiles, and its `init`
  refuses inside a project and cannot run outside one. The setup skill's main and the crate
  documentation show the arm.
- `agent-skills`, patch: a project that serves the `claude` harness adds
  `"env": {"SHELLOPTS": "pipefail"}` to its checked-in `settings.json` of the `.claude` directory, merged
  with what the file holds, as the setup skill's new section on the harness's shell writes it. Every shell the
  harness starts then fails a pipeline when any of its commands fails, so a command chained after a
  piped check does not run over a failing verdict. `head` cutting a producer short then exits 141,
  and a `grep` that matches nothing fails its pipeline.
- `agent-skills`, patch: every tripwire of a project meets the new standard on what earns a
  tripwire, and carries a fourth line, `**Watches until:**`, naming the occasions after which it
  leaves unfired. Run the tripwire axis of the project audit,
  `skill@knowledge-architect-project-audit@tripwire-axis`, which gives each entry its horizon, a
  rewording or its deletion, applied in the branch and listed in its report; only a removal on the
  doubt's weight goes to the owner.
- `agent-skills`, patch: the rules on what else a design head holds and how it is shaped leave the
  primer's section on design heads for the decision-recording skill's new section
  `skill@knowledge-architect-decision-recording@head-shape`. A project's text that cites
  `primer@design-heads` for one of them, such as the alignment of a slug, present tense or one
  decision per head, cites `skill@knowledge-architect-decision-recording@head-shape` instead;
  `primer@design-heads` keeps what earns a head, whether a reason belongs in a head or in a comment,
  and what a head stands on.
- `agent-skills`, patch: a head that cites the owner's words as its ground is read by the new rule
  on what a head stands on: an approval is no ground, a pick at a fork is recorded in a weak form,
  and an intent is drafted as a goal. Run the design-record axis of the project audit,
  `skill@knowledge-architect-project-audit@design-record-axis`, which re-grounds each such head in
  the branch, and puts each intent on the owner list as a goal to draft.
- `agent-skills`, patch: a plan document under way whose acceptance criteria await the owner's
  ruling drops the ruling and their `AC` labels; a criterion whose failure a test, a gate or a review
  would report becomes a claim of the step that builds it.
- `agent-skills`, patch: a project whose own text cites `knowledge-architect-standing-entry-searcher`
  or `knowledge-architect-design-entry-searcher` by its reference rewrites the citation to
  `agent@knowledge-architect-record-searcher`, which replaces both.
- `agent-skills`, patch: every goal of a project meets the goal standard of eight rules the
  goal-setting skill now states: one outcome the owner intends, one reading, nothing ruled out that
  the owner has not ruled out knowingly, one intent, no contradiction with another goal, a measure
  that stays checkable, a refinement that neither widens nor contradicts its parent, and an
  exclusion only where it names the design it excludes. Run the review of the whole goal set,
  `skill@knowledge-architect-goal-setting@whole-set-review`: it runs in rounds, each with one goal
  reviewer per set of goals and one on the whole set, the owner answering between rounds, its record
  committed in docs/goal-reviews/, until the owner stops it or the count falls to a fifth of the first
  round's; then it puts every goal to the owner with its boundary block, the goals in the tree and
  the blocks in the record, ruled on the batch. The pin
  move runs it in a branch of its own, as it runs an audit axis.

### New features

- `cli`, minor: `klarch init [PATH]` creates a project: its manifest, pinned to the binary that
  runs it, the installed agent files, and every document and register home the check requires,
  then the indexes and the check. It overwrites no hand-written file outside the installer's
  namespace and asks nothing; `--component` adds Components, `--command` declares the command,
  `--vcs` decides whether `git init` runs, and `--rust` adds the maintenance crate, its workspace
  and its cargo aliases. The core's README says
  what it refuses.
- `cli`, minor: `klarch new-component <DIR>` adds a Component to a project: its line in
  `[project] components`, keeping every other byte of the manifest, and its documents and homes.
- `cli`, minor: `klarch worklist --out <dir>` writes the work list of a search of the whole
  record: every entry grouped into clusters rooted at a design home's subjects or a declared
  register's groups, packed into agents under a bound in words, with each agent's entries and the
  report it fills, and a snapshot of every entry as `show` prints it; `klarch worklist --verify
  <dir>` checks the filled reports and, when they pass, assembles the entries graded `bears` into
  `bears.md` and those graded `may-bear` into `may-bear.md`.
- `cli`, minor: `klarch id <id>…` says whether each id is free, or lists every entity the project
  defines under it; it refuses an incomplete model.
- `library`, minor: `cli::init`, `cli::InitArgs`, `cli::NewComponentArgs`, `cli::Vcs` and the
  command enum's variants `Init` and `NewComponent`; and the agent-skills crate's `SNIPPETS`, the
  files the setup skill's Rust section shows.
- `library`, minor: `cli::Command::Worklist` and `cli::WorklistArgs`.
- `library`, minor: `cli::Command::Id` and `cli::IdArgs`.
- `cli`, minor: `klarch transcript list`, `show` and `search` read the session transcripts Claude
  Code writes: the project's sessions, cut where a new session starts, `/clear` included, each line
  labelled by its fields as the owner's, injected, the agent's, a dispatch or a tool's, and a header
  naming every line and value the command does not recognise. The core's README says how.
- `library`, minor: `cli::Command::Transcript`, `cli::TranscriptArgs`, `cli::TranscriptVerb`,
  `cli::TranscriptListArgs`, `cli::TranscriptShowArgs` and `cli::TranscriptSearchArgs`.
- `agent-skills`, patch: the project audit gains its tripwire axis, with the agent
  `agent@knowledge-architect-tripwire-auditor` and the saved workflow `knowledge-architect-tripwire-audit`.
- `agent-skills`, patch: the primer's last section, `primer@coined-terms`, defines in one sentence,
  with a pointer to its home, each coined term the primer uses, and separates the senses of a word
  the workflow uses in more than one; a project's own text can cite it.
- `agent-skills`, patch: the agent `agent@knowledge-architect-goal-reviewer` judges goals against
  the goal standard, in a draft, a whole-set and a merge mode.

### Workflow

- `agent-skills`, patch: a session loads the design skill before starting a change the owner asks
  for or proposes, unless a plan document, an outcome the owner ruled in an audit, an intake or a
  review, a design head or a failing test already states what the owner asks for or proposes, or the change decides nothing: a request
  or a proposal of the owner no longer counts as a settled design.
- `agent-skills`, patch: the primer's list of installed skills opens with an instruction: load the
  skill of a moment the list names, with each skill of the project that says it adds to it, and
  follow them whole, a section found by a search not being the skill; a skill from outside the
  installed set and the project never takes their place. The list's closing line on the project's
  own skills is folded into it.
- `agent-skills`, patch: an item met outside the task whose fix lies in the installed text or in
  the checker, and not in what the project adds to them, goes to the retrospective's workflow file,
  never to a fix of the project nor to an issue entry of it but the one beside a pointer no checked
  form expresses, as the primer's section on it now says.
- `agent-skills`, patch: the goal on repeated tasks that the setup skill proposes for a maintenance
  tool takes only the mechanical tasks local to the project: a task the installed text asks of every
  project is the checker's to automate.
- `agent-skills`, patch: the transcript reviewer, and the subagent that assembles a plan document,
  extract a session with `klarch transcript show` instead of writing a script, and find each session
  with `klarch transcript list`, never by a session identifier; a reviewer lists under "Met outside
  the task" each line the command's header reports as not recognised.
- `agent-skills`, patch: the standards a session applied by reading a pointed section whole are
  copied into each text that applies them: the rules on design heads, the admission standard,
  the retrospective's file format and kinds of defect, the decision-recording skill's rules on the
  Component, rejected designs, the deliberation and a reversal, the audit's outcomes, the rule that
  a skill is self-sufficient, and the trigger test. The rereads of the primer's section on design
  heads are gone; the reviewers and auditors that judge by these standards hold them.
- `agent-skills`, patch: a size cost of agent-facing text is weighed in the context of the
  sessions that load it, never by the text's length, a rule the primer states, and every text that
  holds the admission standard's shape of a repair.
- `agent-skills`, patch: the setup skill names `klarch init` as how a project is created, and
  `new-component` as how a Component is added; its Rust section shows the virtual workspace a
  project gains, and says `init --rust` writes its blocks but the continuous integration workflow.
- `agent-skills`, patch: a plan document is deleted in a commit of its own, after the review before
  the merge has read the tip that holds it, where it was deleted in the commit that completed its
  harvest; that commit counts among the review's repairs, as does a commit landing after the review
  on the owner's answer to one of its items or to meet a gate.
- `agent-skills`, patch: every review brief asks the reviewer to end its report with a line asking
  the dispatcher to save it whole, and a general-purpose reviewer returns its report as text; the
  record of the last transcript review's items is the message of its own repair commit, and the
  review ends with the repairs of the decision-record review of those repairs.
- `agent-skills`, patch: the primer asks for `commits <main>..HEAD` after each commit, and explains
  the two repairs `check --fix` names when it refuses.
- `agent-skills`, patch: a pin move whose Migration entries cite an audit axis opens one `todo`
  issue per owed axis, in the root Component, which the audit's commit closes.
- `agent-skills`, patch: a premortem's causes are no longer put to the owner. At the end of the work
  that implements the decision, the session judges each cause by the tripwire standard, records
  those that pass, and lists every cause with its outcome in its turn's "Decided" section and in the
  harvest's or the in-change commit message; the owner reads it and contests an outcome with a goal,
  a fact or an argument. The `T` label is gone.
  A recorded tripwire's wording, scope and removal are the session's under the standard, listed in
  "Decided"; a removal on the doubt's weight alone is the owner's. The owner's word admits and keeps
  no tripwire the standard refuses.
- `agent-skills`, patch: a cause becomes a tripwire only where its failure is silent, its evidence
  lands in the tree, the history, an audit report or a received retrospective file, it is what the
  decision forbids, and a number in it has a basis; one slip a review repaired fires none.
- `agent-skills`, patch: the standing-state reviewer asks of each tripwire whether its horizon has
  passed, and the decision-record reviewer judges each tripwire a branch adds by the standard and
  looks for every premortem cause's outcome in the branch's messages.
- `agent-skills`, patch: an acceptance criterion is admitted only where its failure is silent, its
  firing would show the decision it guards wrong, and a number in it has a basis; a check whose
  firing would show the work unfinished is a claim of the step that builds it. No criterion is put
  to the owner, and the `AC` label is gone; a value a criterion's basis leaves open is listed among
  the defaults awaiting the owner. A criterion that fires opens a design discussion with the owner
  on the decision it guards, where the measurement, the argument and the owner's forks each answer
  their part.
- `agent-skills`, patch: the owner changes a decision through a goal, a fact or an argument; no word
  decides against an argument without one. The design skill's owner decides a fork and contests any
  closure, "Stop" ends a discussion and decides nothing, a reopening decides nothing until a goal, a
  fact or an argument backs a new pick, and a bounded default is stopped, or redirected with a
  reason. The expectation set says the same, and adds that the owner states the project's long-term
  intent as goals.
- `agent-skills`, patch: a design head stands on its argument, and what the owner brings lands by
  its kind: a fact as a premise, an intent as a goal, an argument on its merits, an approval as no
  ground. A choice at a fork is recorded in a weak form, the session's when the owner approved its
  default and the owner's when the owner picked, and reopens without the owner on an argument or a
  fact that ranks the shapes and that the fork's record does not hold. Entry test 4 is gone: the
  owner's intent is drafted as a goal, and its `Q` questions with it.
- `agent-skills`, patch: the transcript reviewer reports a ruling of the owner only where the record
  reverses it, relaxes it, turns its direction or attributes to the owner what the owner did not
  decide; a refinement in its direction is no finding. The rules on heads, the record reviewers and
  auditors, the plan reviews and the retrospective read an approval the same way.
- `agent-skills`, patch: the retrospective's version line names the main commit the session's tree
  held when its work began; a retrospective intake reads whole the entries its proposals rest on,
  every issue and tripwire its search grades `bears` or `may-bear` and every `bears` entry, and lists
  the rest of what its search grades as bearing as not read; a finding already repaired or recorded takes a short section.
- `agent-skills`, patch: one installed agent, `agent@knowledge-architect-record-searcher`, replaces
  the two searchers. The design skill's grounding, once the session has read the record, grepped it
  and drafted and challenged the shapes, with the problem and those shapes as the work, the design
  audit of a slice or a spec and the retrospective intake each run `worklist`, dispatch one searcher per agent directory, and check
  the reports with `worklist --verify`, again for a later thread or a proposed fix on a subject the
  first work did not cover; the design audit now reads the design entries and the `rejected` entries
  too.
- `agent-skills`, patch: the decision-recording skill, the decision-record reviewer and the
  design-record auditor state when a subject of a design home is split: when `worklist` names its
  cluster above the bound, or when its heads answer questions about parts that change apart. The
  project audit's design-record axis and the decision-record review report a subject to split.
- `agent-skills`, patch: a design session reads whole the entries its search grades `bears`, and an
  entry graded `may-bear` only where a proposal touches it, from the files `worklist --verify`
  writes; it lists the others with the searcher's reason as not read, in the plan document, or in
  the commit message of an in-change deliberation or of bounded work. A searcher grades `may-bear` only on a named
  doubt, an element of the change and a sentence of the entry; an entry that only shares the work's
  subject is `unrelated`.
- `agent-skills`, patch: the design skill's grounding and the design audit seed the record search
  with decisions only, never a goal, the grounding with those the problem and the drafted shapes rest
  on or may change; a goal a discussion sets is no part of its search's work.
- `agent-skills`, patch: the design-conformance reviewer reads whole every entry a plan document
  lists as not read, issues and tripwires included, and runs at the review before the merge of a
  branch whose commit message lists such entries.
- `agent-skills`, patch: the design skill checks a thread's candidate slugs with one call of
  `klarch id`.
- `agent-skills`, patch: the retrospective asks one more standing question: whether the session or
  a subagent wrote a script, or carried by hand over several steps, a mechanical task the installed
  workflow asks of it.
- `agent-skills`, patch: before proposing a fix or any other change to code, a session reads the
  record of that code: the entries its comments cite, the entries that cite its file, listed by
  `klarch show path@<anchor>@<file>`, the heads on its subject in its Component's design home, and
  the entries on it in its `rejected` home, per the primer's new subsection in
  `primer@verify-before-relying`. A fix that would contradict a head, or bring back a rejected
  design, goes to the design skill.
- `agent-skills`, patch: goal-setting dispatches the goal reviewer before every ruling of the owner,
  on the whole batch of new and reworded goals with every relevant statement of the owner, again on
  every rewording and on every repaired draft; the owner sees each goal with its boundary block, the
  cases it rules out and admits and the sibling goals it touches, and each contradiction between the
  owner's statements and a goal at the top of the draft.
- `agent-skills`, patch: a goal-setting batch of more than three goals is drafted in the goals homes
  of the working tree, its boundary blocks and the reviewer's findings in a record under
  docs/goal-reviews/, and the owner rules on the batch as the tree holds it, naming each goal dropped
  or reworded; three goals or fewer are shown in the message and ruled by slug.
- `agent-skills`, patch: a goal's body has no bound of two to four sentences any more: it holds what
  the goal standard asks of it, in short sentences, and a list where it names the members of a set.
- `agent-skills`, patch: a goal's wording proposed in another activity, a plan's default, a review's
  repair, an audit's item or a design discussion's goal, is ruled as its own item under
  goal-setting, never by a word given on a batch of other items.
- `agent-skills`, patch: the review before a merge owes the goals axis, the goal reviewer in its
  merge mode, when the diff adds, rewords or removes a goal.
- `agent-skills`, patch: a goal's wording that rules out what the owner has not ruled out knowingly
  is a finding of the transcript reviewer, of a plan review and of the retrospective, though a
  stricter refinement of any other ruling stays none.

## 0.8.0

### Migration

- `checks`, major: every Component carries the home of a built-in heading register, `rejected`,
  `register@*@rejected`, a file rejected.md or a directory rejected/ under its docs/, and its
  rejected-alternatives.md under docs/ is no longer a required document. Move each Component's rejected-alternatives document to its
  `rejected` home and turn each entry into a level-three heading that names the design not kept,
  its slug last, its body opening with its `live` or `void` marker, its "lost to" clause dropped;
  then cite each entry, `rejected@<anchor>@<slug>`, from the design heads whose argument covers it.
  A new check reports an entry that no design entry cites. Choosing the slugs and the citing heads
  can be handed to subagents, one per Component, that propose them for review and then apply them.
  This holds for every project the consumer's tests build, mock projects included. A plan named
  `rejected` is now refused, as any kind's name is. Run `cargo klarch check` after the move: its
  findings name a missing home, an unslugged heading and an uncited entry. Then run the
  design-record axis of the audit, `skill@knowledge-architect-project-audit@design-record-axis`.
- `checks`, minor: every id the checker defines has two segments or more, `[a-z0-9]+(-[a-z0-9]+)+`:
  a register entry's id, a plan document's, a plan item's, a skill's or an agent's name and a
  section slug. A slug of one segment is reported where it is defined, and an issue file, a
  milestone, a spec, a skill or an agent named by one word is refused. Rename each such id with a
  second segment, and every reference to it; a plan document numbers its argument items `arg-1`,
  `arg-2`, and so on. A register's name and its directory keep the grammar of one segment or more.
  This holds for every project the consumer's tests build, mock projects included.
- `checks`, minor: a defined id written outside its reference form is reported, by three lints of
  the last phase: a reference-shaped token outside backticks, an id in backticks alone, which was
  reported for a skill's or an agent's name only, and an id written as a plain word. Each finding
  names the reference to write, or every candidate where several entries define the id; a plain
  word meant as an ordinary term shows a slug that names a subject, and the entry is renamed to
  state its decision. Text inside double quotes, typographic or straight, or on a blockquote line,
  is read by none of the three, and a token beside a backticked reference to an issue entry in its
  paragraph is not reported. Run `cargo klarch check` and repair each finding. Commit messages are
  judged too, by `commits`.
- `manifest`, major: `rejected` is now a built-in register, so a manifest that names a Component or
  a location `rejected`, sets a key in a table `[registers.rejected]`, or declares a register of
  `scope = "component"` whose directory is `rejected` is refused. Rename the Component or the
  location, or give the declared register another name and directory.
- `library`, major: `document::Observation` gains the variants `UnquotedReference` and
  `PlainWord`, and `BareName` is no longer recorded inside a quotation. A `match` over it adds the
  two arms.
- `agent-skills`, patch: the section `outcomes` of the project-audit skill is now
  `skill@knowledge-architect-project-audit@finding-outcomes`, and the section `keep-or-change` of
  the design skill is now `skill@knowledge-architect-design@incumbent-evaluation`. Rewrite each
  reference to either.
- `agent-skills`, patch: the primer states the checker's model, the verification rules, the
  rules of working prose and the checker's CLI facts, the staged check before each commit among them,
  in `primer@checker-model`, `primer@verify-before-relying` and `primer@working-prose`. A project's
  root CLAUDE.md that restates any of these deletes its copy: the primer and the root CLAUDE.md are
  one text, and a duplicate inside one text fails the need test of
  `skill@knowledge-architect-agent-configuration@admission-standard`. The setup skill no longer
  recommends that the root CLAUDE.md state the staged check. A hand-written list of the project's
  anchors in the root CLAUDE.md can be deleted too: `anchors` prints it from the manifest. Then run
  the agentic-workflow axis of the audit, `skill@knowledge-architect-project-audit@agentic-workflow-axis`.

### New features

- `cli`, minor: `anchors` lists every anchor of the project, the constructed ones included, with
  its kind and its path, and, under every anchor but a plan's, the home of each register it
  carries; it exits 1 when a home is missing.
- `manifest`, minor: a register declares `shape = "directory"` and `carries = [...]`: one directory
  per entry, each an anchor named by its basename that carries the one named `opt-in` register at
  its own directory, so an entry of it is cited `<kind>@<entry>@<id>`, as a milestone's slice specs
  are.
- `manifest`, minor: an `opt-in` register declares `dir = "."` to keep its home at the own
  directory of the location that carries it, or of each entry of a directory register that carries
  it, as `<location>/<entry>/`. A Component cannot carry such a register.
- `manifest`, minor: an `opt-in` register names in `components = [...]` the Components that carry
  it, the root included, each at its own `docs/<dir>/`.
- `agent-skills`, patch: `knowledge-architect-design-entry-searcher` reads the goals, the design
  entries and the `rejected` entries of one Component whole and grades each against a piece of work:
  `bears`, with the relation `constrains`, `may change` or `may bring back`, or `may-bear`, listing
  every entry it read.
- `agent-skills`, patch: `skill@knowledge-architect-retrospective-intake` analyses a project's own
  retrospective files, several at once, finding by finding, and the owner rules one outcome per
  finding: handle now, open an issue, sent upstream, or no change with the reason. Its grounding
  sends the standing-entry searcher and the design-entry searcher. Once every outcome is carried
  out, the intake marks the project file it analysed as analysed.

### Workflow

- `agent-skills`, patch: the setup skill shows how a project declares a register of each shape,
  heading, file and directory, attaches it to a Component or to a location, and uses `dir = "."`,
  so an adoption can propose one for records the built-in registers do not hold.
- `agent-skills`, patch: a design discussion's grounding sends the design-entry searcher beside the
  standing-entry searcher, one per Component the question touches, and again for a thread whose
  subject the first search was not given; its first round names the entries graded `bears`, and a
  plan document lists them under "What is already decided". A design head cites the `rejected`
  entries its argument covers, and a `rejected` entry names no winner as such.
- `agent-skills`, patch: a plan document's status section names each transcript it was assembled
  from by the message where the discussion begins in it, and no longer by the transcript file's
  path, which holds the session identifier and the machine's home directory.
- `agent-skills`, patch: before each commit, a session lists the factual claims of the commit's
  message and checks each against the tree; a design discussion's grounding opens by reading
  `primer@verify-before-relying` again.
- `agent-skills`, patch: a retrospective's project file no longer becomes issue entries directly.
  It waits where the files went until the owner hands it to an intake, which writes an analysis
  into docs/retrospective-reports/, commits it with the owner's rulings, and deletes it once its
  last outcome is carried out. A finding whose fix lies in the installed text or in
  the checker is added to the retrospective's workflow file, or to the issue that carries it
  upstream.
- `agent-skills`, patch: a retrospective's project file names the project's main commit, the
  branch and whether the tree held uncommitted changes, so that an intake reads the blamed text as
  the session read it; the workflow file names them only where the project builds the checker from
  source.
- `agent-skills`, patch: each step of a spec's implementation sequence states its Claims, as a
  slice's entry does, so the commits that land the step name claims the spec holds.
- `agent-skills`, patch: a plan document is read again by its reviewers after its decided design
  changes by a ruling of the owner, as after a design session that converges.
- `agent-skills`, patch: the dispatcher of a review saves each reviewer's report in a file outside
  the project as it arrives, and builds the record of the review from those files.
- `agent-skills`, patch: a head is touched, and brought to the rules on heads, when a change adds or
  removes a reference in it; only re-pointing a reference to a renamed entry or a migrated form
  leaves it untouched.
- `agent-skills`, patch: a record of a deliberation names a skill or an agent that its work removes
  or is still to create by its bare name in backticks, as it names such an entry by its bare slug.
- `agent-skills`, patch: a plan document numbers its argument items `arg-1`, `arg-2`, and so on.

## 0.7.0

### Migration

- `checks`, minor: a `path` reference whose target is a register's home is reported: a heading
  register's file, its directory or that directory's `README.md`, a file register's directory, and
  the plans directory's `specs/` and `milestones/`. A project writes it as
  `register@<anchor>@<register>`, as `register@*@goal` for every Component's goals home; this holds
  for mock projects too. The finding's repair names the form, and no `check --fix` writes it. A
  file-or-directory pair in one sentence becomes one span, a span with a line number or a fragment
  is repaired by hand, and a project's root CLAUDE.md that restates a home is among the sites. This
  command rewrites the common forms in the files it is given:

  ```sh
  perl -pi -e 's{\x60path\@([^\@\x60]+)\@docs/(design|goals|tripwires)(?:\.md|/README\.md|/)?\x60}{"\x60register\@$1\@".{design=>"design",goals=>"goal",tripwires=>"tripwire"}->{$2}."\x60"}ge; s{\x60path\@([^\@\x60]+)\@docs/open-issues/?\x60}{\x60register\@$1\@issue\x60}g; s{\x60path\@plans\@(spec|milestone)s/?\x60}{\x60register\@plans\@$1\x60}g' <files>
  ```

- `manifest`, major: a register named `register`, and a Component, a location or a plan named
  `register`, are refused. A project renames it.

- `agent-skills`, patch: a design head, a commit message, a tripwire and an issue entry attribute a
  decision to the owner instead of quoting the owner's words verbatim, which stay in plan documents
  and in committed files that leave; entry test 4 and a head whose ground is the owner's attribute
  that ground. A head already holding a quotation is converted when a change touches it; the
  design-record axis of the project audit,
  `skill@knowledge-architect-project-audit@design-record-axis`, brings a project's heads to the rule.

- `agent-skills`, patch: the sections `repair-standard` of the project-audit skill and
  `content-and-style` of the agent-configuration skill are replaced by one section,
  `skill@knowledge-architect-agent-configuration@admission-standard`; a project's own text that
  cites either section cites it instead.

- `agent-skills`, patch: every statement of a project's own agent-facing text, its `CLAUDE.md`
  files, skills and agents included, meets the need test of
  `skill@knowledge-architect-agent-configuration@admission-standard`. The primer and the root
  `CLAUDE.md` that imports it are one text, so a root sentence that the primer states too fails as
  a duplicate, and the primer's copy stays. The agentic-workflow axis of the project audit,
  `skill@knowledge-architect-project-audit@agentic-workflow-axis`, brings a project's text to it.

### New features

- `checks`, minor: a register's home is cited `register@<anchor>@<register>`, at a Component, a
  location, `plans`, or `*` for every Component's own; it names the home in whichever shape it has,
  so a home that outgrows one file breaks no reference.
- `cli`, minor: `show register@<anchor>@<register>` prints the home the tree holds, the register's
  shape and every site that cites it.

### Workflow

- `agent-skills`, patch: before finishing a change that edits a design head, whatever the edit,
  the decision-recording skill runs `show` on the head, reads every text that cites it, and
  re-reads the head's whole body against the change.
- `agent-skills`, patch: a session answers from its source a question that a specification's text,
  a measurement or the argument from the goals and the record answers, cites the source and tells
  the owner, instead of asking; the owner is asked about a goal, a fact only the owner holds, a
  contradiction no argument resolves, an action that cannot be undone, and a choice between shapes that all survive the
  argument. An answer of the owner on a reading that a source contradicts gives way to the source at
  once, with the owner told, instead of being put back to the owner. A ruling of the owner whose
  premise proves false, or a contradiction met outside the task, holds the part of the work that
  rests on it until the owner answers, instead of proceeding on a default. A divergence between a
  design head and the code holds nothing where the head stands: the code is fixed or an issue
  opened, and the work builds on the head; it holds its part only where the head's argument is
  found defeated, while the head's reversal is decided, by the owner where it is a fork or the head
  rests on the owner's words. A turn that decided
  something the owner could have decided ends with a "Decided" section. A project whose own
  instructions override `primer@owner-word-premise` to apply a source over an answer of the owner
  can remove that override.
- `agent-skills`, patch: the transcript reviewer checks the scope of a ruling of the owner only for a
  question the owner answers, and reports an answer on a reading or a fact that a record calls the
  owner's ruling. A design head that stands on its argument is reversed with the owner's approval
  only where the reversal is a fork.
- `agent-skills`, patch: in a design discussion, a thread whose rivals all lost to a named fact or
  binding constraint closes on its argument, in the new state `argued`, shown for the owner to
  contest; a fork, two shapes that both survive the argument, or another question the primer
  reserves to the owner, waits for the owner's word.
  Bounded work builds its default after its one message instead of waiting for the owner's word, and
  a factual question goes to the owner only for a fact the owner alone holds.
- `agent-skills`, patch: a default or a finding of a plan review, of a review or of an audit that the
  record, a measurement or a source settles is decided by the session and reported, not listed as
  awaiting the owner; an audit puts a rule read two ways on its owner list only where no record
  states the reading; the setup of an existing project places a document a row of the knowledge
  table fits, and asks the owner only about the others.
- `agent-skills`, patch: a session that writes or judges agent-facing text, a review, an audit, a
  retrospective's proposal or a skill moving text included, applies one standard: it names, for
  each statement, the act an agent would do differently without it, and a statement for which none exists leaves as the standard's outcome table gives: deleted
  when it carries no instruction, such as a narration of where an instruction came from, and put to
  the owner when it is an instruction. It shapes each repair as that standard's repair section
  says: at the cause, with the instructions it adds and removes counted and its nearest rival
  named; where the repairs applied together add more instructions than they remove, the owner is
  told the net figure and rules on it before any is applied.
- `agent-skills`, patch: the retrospective opens the workflow's issue on the repository's new
  address, <https://github.com/teoboutin/knowledge-architect>.
- `agent-skills`, patch: an instruction about how an agent behaves is added to a project's own
  agent-facing text only on evidence from real use, as for the installed text; a predicted gap
  becomes an issue entry instead.
- `agent-skills`, patch: a review whose diff adds to or changes agent-facing text dispatches a new
  installed agent, `agent@knowledge-architect-agent-text-reviewer`, which judges each statement
  against that standard.
- `agent-skills`, patch: the agentic-workflow axis of the project audit gains a lens, L9, which
  reports statements that fail the need test; its findings are deleted, moved to a comment of the
  source, or put to the owner, as the standard's table gives.
- `agent-skills`, patch: a record of a deliberation, a plan document, an audit's report file or a
  commit message, names an entry its tree does not hold, nor, for a commit message, its first
  parent's, removed or renamed by the work or still to be created by it, by its bare slug in
  backticks, instead of a description in words, and a renamed entry's old slug carries the reference
  to the entry under its new slug beside it; the audit's report file does so for the entries its
  pass removes, renames or creates.
- `agent-skills`, patch: a branch that builds a default put to the owner merges once the owner has
  ruled on it, an acceptance criterion awaiting the owner included; the session puts every default
  still open to the owner by the merge at the latest, and the work goes on with the default
  meanwhile.
- `agent-skills`, patch: the design-record axis of the project audit judges the session's
  application of a pass in a second run of its saved workflow, `from` set to `application` and
  `applied` to the application's commit: every head and rejected alternative the application
  changed, with the tripwires and issues that cite it; and the run's check judges a proposed retitle
  as it judges a created head.
- `agent-skills`, patch: before finishing a change that writes into a design home, the
  decision-recording skill applies every test of the primer's section on design heads again to each
  head the change writes or extends, and reads the owner's words beside each attribution of the owner
  it writes, checking that their scope and every hedge are kept.
- `agent-skills`, patch: the commits of a slice or a spec name each claim of its spec with its test
  and its outcome, and a claim over every site of a corpus is tested by reading the corpus, not by a
  phrase search alone; the slice's design audit reads the tripwires of the milestone document's
  premortem against the planned code.
- `agent-skills`, patch: a commit message that states a plan a later commit of the branch changes is
  not reworded when the later commit's message names the change.

## 0.6.0

### Migration

- `checks`, minor: under the `claude` harness, every level-two heading of the root CLAUDE.md, of
  each project skill, `.claude/skills/<name>/SKILL.md`, and of each project agent,
  `.claude/agents/<file>.md`, ends with a slug, two hashes and the id in backticks. A project adds
  one to each such heading; this holds for mock projects serving `claude` too.
- `checks`, minor: a project skill's directory and a project agent's file are named in lower-case
  words joined by hyphens, and a frontmatter `name`, where one is set, equals that name. A
  skill's directory is not `synced`, nor begins `anthropic-skills`, which the harness does not
  load. A project renames a skill's directory, an agent's file or a frontmatter `name` that breaks
  one of these rules.
- `checks`, minor: under the `claude` harness, a backticked span that is exactly the name of a
  skill or an agent, installed or the project's own, is reported. A project writes it as the
  reference, `skill@<name>` or `agent@<name>`; this holds for mock projects serving `claude` too.
- `checks`, minor: the installer's namespace holds every
  `.claude/workflows/knowledge-architect-<name>.js`: `install-agent-skills` writes and removes
  those files, and `check` compares them byte for byte with the shipped set, as it does the skills
  and agents. A project renames its own saved workflow of such a name.
- `checks`, patch: under the `claude` harness, a slug at a level-two heading of the root
  CLAUDE.md, of a project skill or of a project agent is no longer reported as a misplaced
  definition; it defines a section. A project whose tests pin the misplaced-definition findings of
  the core's checks removes those it expected there.
- `cli`, major: `check --fix` refuses, exit 2, or 1 where it already installed agent files, when
  the index differs from HEAD and a generated file it would write differs from the one the staged
  tree needs, or when the staged tree's generated files cannot be computed, and it refuses
  `--staged`. A script or a CI step that runs `check --fix` over a partly
  staged index handles exit 2, or runs `index --staged` first.
- `manifest`, major: a Component, a location or a plan named after a kind, such as `design`,
  `issue`, `path`, a declared register or one of `skill`, `agent`, `primer` and `instructions`, is
  refused, and so is a register named after one of those four. A project renames it.
- `library`, major: `document::Observation` has a variant `BareName`, a backticked span that is
  one word in the id grammar. Code that matches the enum exhaustively adds an arm.
- `library`, major: `extension::Tree::Commit(&CommitTree)` is `Tree::Snapshot(&Snapshot)`, and
  `Snapshot::revision` says whether the snapshot is a commit's tree or the one git's index would
  commit. An extension renames the variant and the type; under `check --staged` it is prepared
  over a snapshot with `Purpose::Check`, and under `index --staged` and the comparison of
  `check --fix` with `Purpose::Index`.
- `library`, major: `cli::Command::Index` carries `IndexArgs`, and `cli::CheckArgs` has the field
  `staged`. Code that builds either by name adds the field.
- `agent-skills`, patch: the installed skills and agents no longer number their sections, and cite
  each other's sections by reference, as `skill@<name>@<slug>`. A project's own text that cites a
  section of an installed skill or agent by its number cites it by its reference instead.
- `agent-skills`, patch: the decision-recording skill's section "Does it earn a document entry at
  all?" leaves it; its content
  is in the primer's new section on design heads. A project's text that cites that section of the
  skill cites `primer@design-heads` instead.
- `agent-skills`, patch: the design homes and the rejected alternatives a project holds are brought
  to the rules of `primer@design-heads` and to its new entry test 2, by running the design-record
  axis, `skill@knowledge-architect-project-audit@design-record-axis`, after the pin moves.
- `agent-skills`, patch: the decision-recording skill's section on a premortem's tripwires becomes
  `skill@knowledge-architect-decision-recording@ruled-tripwires`, the one home of when and where any
  tripwire the owner ruled on is written, a parked thread's included. A project's text that cites
  the old section cites the new one.

### New features

- `checks`, minor: under the `claude` harness, a skill, an agent and their sections, and a section
  of the primer or of the root CLAUDE.md, are cited with no anchor: `skill@<name>`,
  `skill@<name>@<slug>`, `agent@<name>`, `agent@<name>@<slug>`, `primer@<slug>` and
  `instructions@<slug>`. Each resolves against the installed copies and the project's own files,
  and `show` prints each.
- `cli`, minor: `check --staged` judges the tree git's index would commit, HEAD's tree with the
  staged changes, by `check`'s rules, and prints a `tree: staged` line; it says when nothing is
  staged.
- `cli`, minor: `index --staged` writes the generated files the staged tree needs into git's index,
  and touches no working-tree file, for a commit of part of the working tree.
- `library`, minor: `extension::Tree` is `Clone` and `Copy`.
- `agent-skills`, patch: `skill@knowledge-architect-project-audit` audits a whole project on one
  axis, as opposed to a review of a diff, and its first axis, the design-record axis, re-applies
  the rules on design heads, on the Component a head belongs to and on rejected alternatives to
  every entry; the agent `agent@knowledge-architect-design-record-auditor` drafts its verdicts, one
  per entry, in groups calibrated on a shared sample, and verifies another auditor's drafts. A pass
  of that axis is one run of the saved workflow `knowledge-architect-design-record-audit`: it lists
  the entries, cuts the groups, drafts, compares the sample's verdicts, verifies the drafts, checks
  each head a proposed edit creates, maps the findings that touch one paragraph or one slug and
  the tripwires and issues an edit would leave false, and writes a report the session verifies.
  Its second
  axis, the agentic-workflow axis, reads every instruction the harness delivers, the root CLAUDE.md,
  the installed skills, agents and primer, and the project's own skills and agents, as one whole,
  for instructions a session cannot follow; the agent `agent@knowledge-architect-workflow-auditor`
  drafts its findings through eight lenses, each agent reading the whole corpus, one of them
  comparing what the reviews and the audits count as a finding, and the findings on the installed
  text go into a file for the workflow's maintainers. The confirmed findings are
  grouped by cause and repaired by cause, a pass whose repairs add more instructions than they
  remove goes to the owner with its net figure. A pass is one run of a saved workflow for Claude
  Code's Workflow tool, `knowledge-architect-agentic-workflow-audit`: it drafts, verifies the drafts
  adversarially, clusters the findings by cause, proposes the repairs and writes a report the
  session verifies before the owner list; a session without the tool runs its steps by hand.
- `agent-skills`, patch: the shipped set holds saved workflows of Claude Code's Workflow tool,
  installed as `.claude/workflows/knowledge-architect-<name>.js`; each declares its installed name
  as the name the harness calls it by.

### Workflow

- `agent-skills`, patch: every level-two heading of the installed skills, agents and primer carries
  a slug, and the setup skill and the agent-configuration skill state that the root CLAUDE.md and a
  project's own skills and agents carry one too, and that a subagent's file name equals its
  frontmatter `name`.
- `agent-skills`, patch: the setup skill recommends `check --staged` after staging and before each
  commit, and the issue-tracking and planning skills name `index --staged` and `check --staged` for
  a commit of part of the working tree.
- `agent-skills`, patch: the rules on what a design head records and how move into one section of
  the primer, `primer@design-heads`, which every session holds; the decision-recording skill keeps
  the procedures of recording and opens by asking for that section to be read again, and the other
  installed skills and agents point to it.
- `agent-skills`, patch: a head's title states the rule that decided, and its body names the members
  built; a head holds one decision, split where two parts lose to different nearest rivals; a head
  stands on its argument and cites the owner only for what came from the owner; a member the
  argument covers is recorded directly, and one the owner's words name goes to the owner as one
  proposal; a head a change touches is brought to these rules.
- `agent-skills`, patch: before a head is written from an approval with no argument behind it, the
  decision-recording skill searches for its arguments, costs and rivals, and an even result goes to
  the owner as a tie.
- `agent-skills`, patch: a directive is restated only where the restatement is no longer than a
  pointer to it, one sentence; a longer one is a pointer to its home, read whole. The routing
  reviewer reports a longer restatement, and the design skill's in-change path reads the section on
  design heads again before its grounding.
- `agent-skills`, patch: entry test 2 counts each text that states an instruction as one site of a
  decision about agent-facing text, a design head that restates it included, other than the
  decision's own, so a decision one text states has one site, and its reason
  lives beside the instruction rather than in a head.
- `agent-skills`, patch: moving the pin runs the audit axis a Migration entry of a version crossed
  cites, in a branch of its own started from the main branch once the pin move has merged; the
  work that moves an adopting project's existing documents ends by running the design-record axis.
- `agent-skills`, patch: an entry of the rejected alternatives leaves its file when it is reopened
  and chosen, when the owner rules that it fails every recording test, or when the decision it lost
  to leaves the design home and the text that keeps that decision's reason names the alternative.
- `agent-skills`, patch: a milestone document points to the procedure of working a slice and adds
  what its work needs, rather than restating the procedure.
- `agent-skills`, patch: in a design discussion the agent reopens a closed thread by presenting a
  material finding, and the thread closes again on the owner's word; a parked thread's tripwire
  enters a tripwires home only on the owner's word.
- `agent-skills`, patch: something met outside the task that bears on the work is presented to the
  owner with a default, and the work proceeds on the default; a subagent that edits nothing
  reports such items to the session that dispatched it.
- `agent-skills`, patch: a setup run without the owner writes no goal.
- `agent-skills`, patch: a review dispatches every axis of the review skill's table whose condition
  holds, and those the skill of each activity whose work it reviews adds; every finding is repaired, opened as an issue, or
  judged to need nothing.
- `agent-skills`, patch: a divergence between a design home and the code no longer stops the task:
  it holds only the part of the task that rests on the diverging point until the owner's word, and
  the rest of the task goes on.
- `agent-skills`, patch: before diagnosing a problem, a session lists the issues and the tripwires
  of every anchor.
- `agent-skills`, patch: the review before a merge is the last review of a branch, whichever
  activity's skill dispatched it, and there the transcript reviewer runs only alone and last; an
  activity's skill may defer the transcript axis of an earlier review to it; every subagent that
  judges a commit, a reviewer or an audit agent, gets a worktree detached at that commit, outside
  the project or under an ignored path; a change made in answer to a plan document's
  review, a default included, is a repair, reviewed again only where it changes a design head.
- `agent-skills`, patch: planning's point 4 is a review of the work, and the harvest's review the review
  before the merge; the root's restatements of committing on a branch become pointers to
  `primer@branch-commits`; the agents' descriptions no longer say "Dispatch it; do not read it.";
  the primer no longer gives the owner every decision on delivering a directive at a point; approved
  bounded work whose work does not land in the change under way takes the full path to a spec; a
  default awaiting the owner is built as written until the owner rules, at no fixed moment; a slice's
  spec carries only the sections its slice fills; the installed files are defined by their
  property; a restatement carries its pointer; the primer says a
  macro inside a `let` is bound.
- `agent-skills`, patch: what waits for an owner who has not ruled during the work of a plan
  document is stated once, at the opening of the planning skill's procedure for a slice: a listed
  default is built as written, a scope change waits for its ruling, and a load-bearing gap of the
  design audit stops the slice.
- `agent-skills`, patch: a new section of the primer, `primer@labelled-items`, numbers each label
  put to the owner within the record that keeps its ruling: a plan document, a milestone's
  documents, a file, or the commit messages of one branch; the skills that ask for rulings point to
  it.
- `agent-skills`, patch: a new section of the primer, `primer@branch-commits`, states that every
  commit of a branch is judged by the checker at its tip, so a change that sets what that checker
  requires goes in the branch's first commit, and a repair that would leave an earlier commit
  failing is folded into it; planning, review and setup point to it, and a landing's report on its
  acceptance criteria may span the landing's commits.
- `agent-skills`, patch: the agent-configuration and setup skills no longer keep a routing table in
  the root CLAUDE.md; a project skill reaches a session through its description, and a review's
  added axes through the dispatching activity's skill. After an install that changes the installed
  files, each project skill and agent is read against the installed text that changed. A project
  may delete its routing table.
- `agent-skills`, patch: a retrospective whose moment falls during a task is offered when the task
  ends.
- `agent-skills`, patch: the move of an adopting project's existing documents starts under the
  design skill, which hands it to planning.
- `agent-skills`, patch: an issue entry is deleted with `git rm`, and the files an install removed
  are staged, before the check runs.
- `agent-skills`, patch: a subagent that edits nothing lists what it met outside its task under a
  heading "Met outside the task" at the end of its report.
- `agent-skills`, patch: the assembly of a plan document and the transcript reviewer count as the
  owner's a message the owner typed while the session worked; in a slice's work, the transcript
  reviewer runs once, at the harvest's review.
- `agent-skills`, patch: on the in-change path an acceptance criterion is put to the owner and its ruling travels with the
  deliberation, and a record no commit carries is the report to the owner; a label carried into a
  second record keeps its number; the primer defines a heading as markdown's ATX heading; a subagent
  is told to read from disk any file that changed after the session started, an install included;
  several restatements in the installed skills and agents become pointers to their homes.
- `agent-skills`, patch: the primer's table gains a row: the evidence of one pass of a project
  audit, and the owner's ruling on each of its items, go to one report file per pass in
  docs/audit-reports/ at the project's root, deleted once its outcomes are carried out.
- `agent-skills`, patch: a requested change that is not bounded and whose work has not started
  takes the full path and a plan document, even when one pull request builds it; the in-change path
  is for a decision met in a change begun before its discussion, or for a decision no work
  implements. What the planning skill says an acceptance criterion names applies to a plan
  document's criteria only. A load-bearing gap of a design audit that defeats a premise of a ruling
  stops the slice; the session does not proceed on the default the primer names.
- `agent-skills`, patch: a slice's design audit writes one line in its commit message for each
  standing entry the search returned, with its outcome, one judged to bear on nothing included,
  and a plan document assembled from a discussion lists the entries that discussion's search
  returned, each with its outcome; where the slice's first change sets what the per-commit gate
  requires, the audit's documents land in that change's commit.
- `agent-skills`, patch: after the reviewers return, the dispatcher compares `git status --short` of
  the live tree with its own changes, and puts anything else to the reviewer before touching it.
- `agent-skills`, patch: a parked thread's tripwire is put to the owner under a `T<n>` label; a gap
  of a plan review whose one answer widens or narrows a ruling goes to the owner as a scope change;
  a plan document's later consequences move to the issue or plan document of the work they name,
  or to a `todo` issue, before it leaves; a criterion found firing after a landing reported it is
  reported as fired at the next landing.
- `agent-skills`, patch: a default awaiting the owner enters a design head only once ruled, and a
  plan document leaves only once every such default is ruled; a recurring criterion is folded
  into an existing tripwire that guards its decision; a fired tripwire whose reversal waits on a
  plan document's harvest is narrowed to except that document's work; a milestone's own documents
  re-point a leaving slice's citation with no question issue; a plan document cites its own items
  by their checked form; a step whose trees cannot pass alone lands with the step that makes them
  pass; a title brought to what its head argues is not reviewed again.

## 0.5.0

### Migration

- `checks`, minor: the second section every spec of specs/ and every milestone document owes is
  titled `How the work is done`, where it was `How a step is worked`. A project renames that
  heading in each of its plan documents; this holds for mock projects too.

### Workflow

- `agent-skills`, patch: a milestone's part, one branch and one PR with its own spec, is called a
  slice, and "step" names any item of an implementation sequence. The planning skill says how a
  spec's work takes the procedure once, and how many commits a step takes is the session's to judge.
- `agent-skills`, patch: a spec's design audit runs only when the work does not start in the session
  where the discussion converged, or when commits other than the spec's own have landed on the main
  branch since the spec was written.
- `agent-skills`, patch: a plan document lands before its work only when the work changes what the
  per-commit gate checks; a change only the working-tree check sees, such as installed text, does
  not count.
- `agent-skills`, patch: the primer says the installed text leaves room to judge where it is
  silent, and the retrospective counts an instruction as missing only where the workflow needed it.
- `agent-skills`, patch: the rule on writing a pointer without backticks covers only a reference or
  a path in the checker's syntax; the retrospective's standing question asks about those alone.
- `agent-skills`, patch: the design skill grounds a requested change whose design is not settled,
  and for bounded work, which reverses nothing, earns no design head and has no second defensible
  shape, sends one proposal with its default and waits for the owner's word, where it left the next
  step to the owner.
- `agent-skills`, patch: a decision earns a design head only when one of the rewritten entry tests
  passes: an interface others consume, a reason several sites or no site must respect, the
  behaviour of something outside the project, or the owner's own intent, which the agent asks the
  owner to confirm in a numbered question rather than judging it; the decision-record reviewer
  checks that such a head carries the owner's answer. The primer says a reason recorded at the code binds
  as intent below the design home and is read before code is removed, and the decision-record
  reviewer reports a diff that defeats one.
- `agent-skills`, patch: the transcript reviewer rates a misstated ruling critical only when it is
  reversed or changes what is built or a load-bearing decision, and the retrospective counts a minor one caught before the
  merge as no finding.
- `agent-skills`, patch: the retrospective names its files
  `<date>-<project>-<subject>.md` and `<date>-<project>-<subject>-klarch-workflow.md`, never
  overwrites one, and cites a finding by its id and its file's stem, as "W3 of
  <date>-<project>-<subject>-klarch-workflow".
- `agent-skills`, patch: a review item whose outcome turns on a ruling the owner has not given is
  put to the owner before its outcome is recorded; a defect a reviewer finds predating the change
  is routed by the primer's table of what is met outside the task, a fix checkable from the diff
  landing in a commit of its own that the review's record names; and the last transcript review
  waits until the owner has answered every ruling the repairs asked for.
- `agent-skills`, patch: the review of a slice or of a spec's work gives each finding one of the
  review skill's three outcomes, where the planning skill made every unrepaired finding an issue
  entry.
- `agent-skills`, patch: a dispatcher names a scratch directory of its own to each subagent sent
  together with others, and the standing-entry searcher writes its working files there.
- `agent-skills`, patch: moving the pin runs the project's gates command, or the check and the
  tests, before committing; a project writing an extension's tests reads the checker crate's
  section on testing an extension first.
- `agent-skills`, patch: on the design skill's in-change path, where a decision lands is proposed
  only after decision-recording decides its Component, for a reversal, and whether it earns a head.
- `agent-skills`, patch: the retrospective gives the owner the full path of each file it writes,
  and no longer prints either file into the conversation, nor the workflow's file when `gh` fails.
- `agent-skills`, patch: a session loads the decision-recording skill before any write into a design
  home, with no condition to judge first, as the skill's description and the primer say; a session
  editing a skill, an agent or a `CLAUDE.md` also loads it before editing a text whose behaviour a
  head describes, as the agent-configuration skill says.
- `agent-skills`, patch: a review repair that adds, removes or reverses a design head is reviewed
  by the decision-record axis whichever axis asked for it, and no other repair is, where any
  transcript-review repair that made a decision was; decision-recording's finishing step applies
  the entry tests again once the change is written.
- `agent-skills`, patch: a new reviewer, `agent@knowledge-architect-design-conformance-reviewer`,
  reads a plan document against the goals, the design heads and the rejected alternatives of every
  Component it touches, as the fourth reviewer the planning skill sends when a plan document is
  written or a decided shape in one is revised; the review skill's axis row of the standing-state
  reviewer is named "standing state".
- `agent-skills`, patch: the owner rules on each acceptance criterion, as on each tripwire, at the
  premortem; a criterion first proposed later waits as a default awaiting the owner and is not
  judged before the ruling.
- `agent-skills`, patch: the items the owner rules on one by one carry labels: `T<n>` for
  tripwires, `AC<n>` for acceptance criteria and `D<n>` for a plan document's defaults, and `Q<n>`
  where the label stays in the conversation, as for review items, the setup's proposed
  destinations, and an audit's gaps put to the owner in one question.
- `agent-skills`, patch: the design skill builds no discriminating evidence that the project's
  goals or design heads rule out, and sends the fork to the owner as a tie.
- `agent-skills`, patch: the primer says a word of the owner holds only as far as its premise; when
  the premise proves false, the session puts the corrected premise to the owner with a default, and
  proceeds on the default unless the owner answers otherwise, holding any part that cannot be undone.
- `agent-skills`, patch: the retrospective's standing question on the primer asks whether the
  session or one of its subagents acted as if a rule of the primer were absent, where it asked
  whether the primer reached the subagents.

## 0.4.0

### Migration

- `checks`, minor: a heading that markdown reads and the checker does not is reported in phase 2:
  a setext heading, a line underlined with `=` or `-`; a heading after a list or block-quote marker
  on its line; and a heading with no text. A slug on a line indented four spaces or more, or by a
  tab, or after `#` marks followed by a no-break space, defines nothing and is reported as written
  in the middle of a line. A project rewrites each such heading as a line that opens with `#`
  marks, a space and text.
- `cli`, patch: the summary of `commits` reads `<n> commits, <p> passed, <f> failed`, where it read
  `<j> judged, <f> failed`, and each commit's line reads `passed` where it read `judged`. A commit
  whose message alone carries findings now counts as failed, and its line names each source of its
  findings with its count. The verdict line and the exit code are unchanged. A script that reads
  the summary changes the words it matches.
- `manifest`, major: a register named `planned` is refused, as one named `path` is, since
  `planned` is now a kind of the reference grammar. A project that declares `[registers.planned]`
  renames it.

### New features

- `checks`, minor: a plan document cites a path its work will create as
  `planned@<anchor>@<path>`. The anchor and path follow the rules of a `path` reference, and the
  target must not exist; once it does, the finding asks for the `path` form. The form is legal in
  the plans directory only, and the repair of an unanchored path in a plan document names it.
- `cli`, minor: `show` resolves a `planned@<anchor>@<path>` reference, says whether its target now
  exists, and lists the plans that cite it. `show` on a path also lists its planned citations, and
  its citations written with or without a trailing slash.
- `library`, minor: `Finding` implements `Hash`, so findings can be collected in a set.
- `library`, minor: `CHECKER_VERSION` is the version of the core library, the value a project's
  `[project] checker-version` must hold. A test that copies a mock project out of its library's
  directory writes it into the copy's manifest.

### Workflow

- `agent-skills`, patch: the setup skill tells a test that copies a mock project out of its
  library's directory to write the library's `CHECKER_VERSION` into the copy's manifest.
- `agent-skills`, patch: the agent-configuration skill treats an edit that changes what an agent
  is told to do as a decision. Before writing it, the session searches the design homes and the
  goals for the behaviour the edited text describes. An edit that narrows or contradicts a head
  follows the decision-recording skill, and one that strains a goal goes to the owner.
- `agent-skills`, patch: the transcript reviewer's repairs land as additional commits, as many as
  their kinds need, and the review skill sends the decision-record axis at each of them that makes
  or reverses a decision.
- `agent-skills`, patch: a session that dispatches a transcript reviewer finds each transcript
  file by the message where the work begins, never by a session identifier, and the reviewer
  reports at once a named file that does not hold that message.
- `agent-skills`, patch: a finding is never repaired by writing its pointer in plain text. A pointer
  that no checked form expresses is written beside a reference to an issue entry of the project that
  records the missing form, and a gap of the checker gets such an entry in the project's own
  register.
- `agent-skills`, patch: the retrospective asks one more standing question: whether the session
  needed a pointer that no checked form expresses, and how it wrote it.
- `agent-skills`, patch: the planning skill cites a path a plan's work will create in the planned
  form, and the commit that creates the file converts it.
- `agent-skills`, patch: the review skill gives every item of a reviewer's report an outcome, a
  note outside its axis or an observation included, and the commit that records the review lists
  each one.
- `agent-skills`, patch: the decision-recording skill reads a head against the decision the owner
  approved before writing it, and puts to the owner a title or a body that widens, narrows or drops
  part of it.
- `agent-skills`, patch: the review skill gives each claim of a reviewer's finding its own outcome:
  repaired, opened as an issue, or judged to need nothing, with the reason.
- `agent-skills`, patch: a decision met during another task, such as an issue fix, goes through
  the design skill when it creates a design head, contradicts a statement of one, or takes one
  beyond what its title states. Its description names that symptom, and the decision-recording
  skill sends such a decision back to it before its text is written. An addition within what a
  head's title states, and a relocation or rewording that adds or removes no decision, are recorded
  directly.
- `agent-skills`, patch: the decision-recording skill keeps a head's title, as well as its slug,
  aligned with the full scope of its decision, and a title states a decision only while it is
  false of the nearest rival it beat.
- `agent-skills`, patch: the design skill's one-round path for a cheap decision becomes the
  in-change path. It is open when the decision's work, or its record, lands in the change under
  way, whatever the cost of reversal, and it sets no bound on threads or rounds. It writes no plan
  document: the commit message carries every thread with its final state, the owner's words
  verbatim and the rivals that lost. A cheap decision whose work comes later takes the full path,
  with a plan document.
- `agent-skills`, patch: a design thread is named by the decision it would record, not by the change
  it proposes. When a thread's slug misdescribes the approved decision, its design entry takes a
  slug that names the decision, and the plan document's harvest row, or the commit message on the
  in-change path, states the pair.
- `agent-skills`, patch: the design skill rewords a tripwire the owner ruled to record, when it is
  written or on a review finding, on the agent's judgement, and lists each change to the owner at
  the end of the turn.
- `agent-skills`, patch: on the design skill's in-change path, where the premortem runs, nothing of
  a decision is implemented or committed before it has run and the owner has ruled on its
  tripwires.
- `agent-skills`, patch: a new installed agent, `knowledge-architect-standing-entry-searcher`,
  searches the issues and tripwires a piece of work bears on. The design skill dispatches it at the
  grounding of a discussion, and the planning skill at the design audit of a milestone step, one
  agent per group of at most 60 entries, in parallel. The session reads whole each entry it
  returns, where it read every issue and tripwire itself.
- `agent-skills`, patch: the design audit of a milestone step reads issues of every kind the
  step's code bears on, not only the deferred ones, and a step's grounding no longer reads the
  Component's issues and tripwires.
- `agent-skills`, patch: the standing-state reviewer reads every deferred issue's trigger against
  the change before every merge, as it reads every tripwire, and reports a trigger the change
  meets as a finding.
- `agent-skills`, patch: the review skill repairs a commit message carrying a mistake by amending it,
  or by a history edit of the branch, never by a correction written into a later commit's message.
- `agent-skills`, patch: the issue-tracking and decision-recording skills rewrite a sentence about
  the past whose reference dangles to state the present, or remove it, and never retarget it to the
  new name.

## 0.3.0

### Migration

- `checks`, minor: a generated index with one row says `1 entry`, not `1 entries`. A project with
  such an index runs `index`, or `check --fix`, once after moving to this version.
- `manifest`, major: every manifest carries `[project] checker-version`: the exact version of the
  checker the project runs, `"fixture"` in a mock project inside the directory of a library the
  binary links, and in a manifest a test only parses, or `"self"` where the checker is built from
  the project's own tree; it holds for mock projects too, an extension's included. A manifest
  without it, or a binary of another version, refuses every command with exit 2. A binary
  released before this one refuses a manifest carrying the key, as an unknown field.
- `library`, minor: `cli::refuse_another_version` refuses a run over a project whose
  `[project] checker-version` the binary does not satisfy. A project's maintenance crate or
  extension binary calls it in its `main`, after `cli::refuse_a_foreign_build` and before its
  match on the command, as the crate's template shows; without the call, that binary enforces no
  pin.

### New features

- `checks`, minor: a backticked path-shaped span is reported only when its first segment names a
  file or a directory the tree's listing holds, read from the root, from every anchor that is a
  directory and from the document's own directory. A media type, a unit, a git ref or another
  machine's path, such as `application/json` or `origin/main`, can be written in backticks.
- `checks`, minor: the retired slug reference is reported only when its id is an entry of the
  project, or its word an anchor or a kind. An issue number `#123` or a directive `#include` can be
  written in backticks.
- `cli`, minor: `check` names, above its verdict, each file holding a finding that git does not
  track, and says to commit it, or to move it out or ignore it.

### Workflow

- `agent-skills`, patch: the setup skill writes `[project] checker-version` into the smallest
  manifest, moves it with the pin, and its maintenance crate's `main` calls the version refusal.
- `agent-skills`, patch: a plan document may be merged to the main branch in a pull request of its
  own, whatever the time of its work; one whose work changes what the gates check is merged before
  that work, a spec as well as a milestone document.
- `agent-skills`, patch: when a project moves its pin, the setup skill fetches the new version and
  reads the changelog of each version crossed in the crate source cargo downloaded.
- `agent-skills`, patch: the setup skill's section for a Rust project ties every build to its
  checkout: a cargo `[env]` variable valued at the checkout's root, read by every library root and
  every target of a package with no library, and named by every build script. A target directory
  shared by two checkouts then rebuilds instead of running the other checkout's build.
- `agent-skills`, patch: the review skill has the dispatcher name each reviewer's worktree path in
  its brief, distinct for each reviewer.
- `agent-skills`, patch: the primer says that a design home prevails over diverging code until the
  issue closes, by the code meeting the head or by the head's reversal, never by following the code.
- `agent-skills`, patch: the planning skill keeps a check proposed before a plan document exists as
  a lead in its issue entry, which the design session judges.
- `agent-skills`, patch: after installing a new version, each open milestone document's procedure
  for working a step is restated from the new planning skill.
- `agent-skills`, patch: the record of a review names each reviewed commit by its subject, not by
  its SHA.
- `agent-skills`, patch: after editing a register, the issue-tracking, planning and setup skills
  run `check --fix`, which writes the stale generated files and checks, in place of `index` then
  `check`.
- `agent-skills`, patch: the setup skill asks the owner whether a Rust project with no workspace
  gains one, which it recommends, or runs a local install, and how to treat each finding the check
  reports in a file the project already has.

## 0.2.0

### Migration

- `checks`, major: every project carries docs/plans/ at its root, holding a `README.md`, specs/ and
  milestones/, and each of the two holds a `README.md` and a generated `index.md`. Move each spec
  into specs/ as one file, and each milestone into milestones/ as a directory holding its
  `README.md`; then run `index`. Any other file or directory directly under docs/plans/ is a
  finding.
- `checks`, major: a `path` citation of a plan document is refused. Cite a spec as
  `spec@plans@<id>`, a milestone as `milestone@plans@<id>` and a step of a milestone as
  `spec@<milestone>@<step>`. A citation from outside the plans directory of a file inside it is
  refused as reaching inside the anchor `plans`.
- `checks`, major: a plan document owes its sections: a spec of docs/plans/specs/ and a milestone's
  README the plan sections, in order, and a step spec the step sections. Add the missing ones.
- `checks`, minor: a `path@*@<path>` reference that only a declared location carries is reported,
  as the design of the generic form always said; anchor it at the location instead.
- `checks`, major: an item of a plan cited from outside it is refused; cite the plan whole. A
  backticked span whose head, before its first at sign, is `spec`, `milestone`, `thread`,
  `argument`, `criterion`, `acceptance`, `plans`, or the id of a spec or a milestone, is now a
  reference candidate.
- `manifest`, major: a declared anchor named `plans` and a `[registers.spec]` or
  `[registers.milestone]` table are refused, and so is `spec` or `milestone` in a location's
  `registers` list.
- `manifest`, major: a `[registers.thread]`, `[registers.argument]`, `[registers.criterion]` or
  `[registers.acceptance]` table is refused, and so is any of those names in a location's
  `registers` list. Rename a register of the project that carries one of them.
- `manifest`, major: a location declared at docs/plans/, and a component register whose `dir` is
  `plans`, are refused, because the tool constructs the anchor `plans` there. Move the location
  elsewhere, or give the register another `dir`.
- `library`, major: `cli::Command::Check` takes its arguments, `Command::Check(CheckArgs)`. Code
  that builds or matches the variant by name writes `Command::Check(CheckArgs { fix: false })`, or
  matches `Command::Check(_)`.
- `agent-skills`, major: the primer names the plans directory, docs/plans/, and the roadmap. Remove
  the row naming the plans directory from the project's own rows of the knowledge table, in its root
  `CLAUDE.md`.
- `agent-skills`, major: the agent `knowledge-architect-transcript-conformity-reviewer` is renamed
  `agent@knowledge-architect-transcript-reviewer`. Rename it wherever the project's own skills,
  agents or `CLAUDE.md` files name it.

### New features

- `checks`, minor: the plan items, `thread`, `argument`, `criterion` and `acceptance`: a
  level-three heading with its slug under the section of its kind, cited
  `<kind>@<plan>@<id>` from inside its plan, where each spec file and each milestone is an anchor.
- `checks`, minor: the reference kinds `spec` and `milestone`, carried by the anchor `plans`, and
  one anchor per milestone directory, carrying `spec` for its step specs. `show` prints a spec
  or a milestone document, and every reference to it.
- `checks`, minor: `commits` refuses a citation of a commit of its range by SHA, in a message or in
  a document of a commit's tree, where the manifest turns it on.
- `cli`, minor: `check --fix` applies every safe fix before checking: it installs the agent files
  when one is missing, differs or is no longer shipped, writes every stale or missing generated
  file, lists each, then runs the check. An upgrade that removes a shipped file takes a
  `git add` and a second run.
- `cli`, minor: `--version` prints the version of the checker that runs, from any directory.
- `manifest`, minor: `[commits] refuse-branch-shas`, off when absent.

### Workflow

- `agent-skills`, patch: a plan document names every text that references a decision its work
  reverses or rewrites, restatements in a `CLAUDE.md` or a skill included, with the step or the
  harvest that judges or updates each.
- `agent-skills`, patch: a plan document is committed before its reviews, and each repair lands
  after them.
- `agent-skills`, patch: a milestone document whose first step changes what the gates check lands in
  a merge of its own, before that step.
- `agent-skills`, patch: a design discussion's first round states which grounding commands ran, and
  a measured fact carries the command that re-takes it.
- `agent-skills`, patch: a retrospective names each finding by a letter and a number (W, C, P), and
  states the version of knowledge-architect the session used.
- `agent-skills`, patch: a design audit lists an answer that widens or narrows a ruling of the owner
  as a default awaiting the owner, ruled at the audit before its point is implemented.
- `agent-skills`, patch: a step's harvest is reviewed before the merge, on the decision-record,
  routing and standing-state axes, and by the transcript reviewer where the transcript is
  available.
- `agent-skills`, patch: a review repair that would leave an earlier commit of the branch failing
  the project's checks is folded into that commit by a history edit, and the review's record says
  so.
- `agent-skills`, patch: the planning skill writes a plan document's threads, arguments, criteria
  and acceptance criteria as items, ``### <statement> `##<id>` ``, under the section titles the
  checker matches, with Arguments right after Threads. It assembles the document from the discussion's
  transcript through a subagent, and the transcript reviewer reads every assembled document.
- `agent-skills`, patch: an optional roadmap, docs/roadmap.md, orders known work on the owner's
  word. The commit that adds a plan document rewrites the row of the issue it closes, and the commit
  that deletes a plan document removes its row.
- `agent-skills`, patch: a milestone's design is written into one spec per step from the start,
  the milestone document keeping what crosses steps. A step's design audit edits those documents in
  place, listing its findings in the commit message, and a new step found at an audit gets a spec
  of its own, listed as a scope change the owner rules on before it is implemented.
- `agent-skills`, patch: the transcript reviewer checks that everything a work's sessions
  established has a durable outcome, a reviewer's finding acted on included, and that no ruling of
  the owner is misstated. It no longer reports detail added inside a ruling. It runs once more,
  alone and last, before every merge to the main branch.
- `agent-skills`, patch: a step's design audit lists the tripwires and the `deferred` issues the
  step's planned code would fire, with `tripwires` and `issues --kind deferred`, so the owner rules
  on them before the code is written.
- `agent-skills`, patch: a load-bearing gap at an audit that is a choice among shapes stated in
  full, where the design skill's conditions for its one-round path hold, is ruled in one round with
  a default, several gaps in one question, and written into the step's documents; a gap that defeats
  what an approved thread rests on still opens a full design session. A harvest names in its commit
  each item of its row that the recording tests exclude.
- `agent-skills`, patch: the setup skill creates docs/plans/, with its `README.md`, specs/ and
  milestones/, when a project adopts the workflow, and no longer asks the owner for the plans
  directory's path.
- `agent-skills`, patch: the setup skill proposes, for a crate the project publishes, a short
  `CRATES-IO.md` as its crates.io page, named by `readme`, while its `README.md` stays
  repository-facing.
- `agent-skills`, patch: the commit that deletes a plan document cites it by its kind. A citation of
  it left in another plan is removed, and a `question` issue is opened on the citing plan.

## 0.1.0

- `manifest`: the manifest file is `knowledge-architect.toml`.
- `cli`: the binary is `klarch`.
- `library`: the library and the binary are one crate, `knowledge-architect`, imported as
  `knowledge_architect`.
- `manifest`: `[project] command` declares the command a project runs the checker by; `klarch`
  when absent. It is printed in messages and in generated index headers.
- `manifest`: `[agents] harness` declares the agent harnesses a project serves; `["claude"]` when
  absent. An empty list drops the CLAUDE.md requirement.
- `cli`: `install-agent-skills` writes the shipped agent files and removes unshipped ones from its
  namespace.
- `checks`: phase 2 reports an installed agent file missing, differing or unshipped, its deletion
  not staged, and a root CLAUDE.md that does not import a shipped primer or that the walk keeps
  out. `commits` does not compare installed files.
- `manifest`: a `command` that is empty or holds a line break or a backtick is refused.
- `cli`: `install-agent-skills` refuses a symbolic link on an owned path, and a manifest holding a
  refused declaration.
- `library`: the public API is the crate root and four modules: `cli` for a binary's `main`,
  `extension` for writing an extension, `document` for reading a document's parse, `testing`
  for running the core over a mock project. Every other module is private.
- `library`: `cli::Gathered` gathers what a run reads before any check; `complete_working_tree`
  returns it.
- `library`: `cli::Command`, `extension::Inputs`, `extension::ExtensionReport` and
  `extension::Resolution` are non-exhaustive.
- `library`: the `testing` feature is removed.
- `agent-skills`: the first installed skills, decision-recording and issue-tracking. The
  package ships its build script and its content/ directory, from which the list of shipped files
  is generated.
- `agent-skills`: the planning skill, and the transcript-conformity reviewer agent.
- `agent-skills`: review, and the reviewer agents standing-state, decision-record,
  routing, code-claims and cold-implementer.
- `agent-skills`: setup, agent-configuration, and the primer, which the project's root
  CLAUDE.md imports.
- `agent-skills`: goal-setting and the retrospective.
- `agent-skills`: goal-setting places a goal in the Component responsible for it; setup
  proposes two goals for a project's maintenance tool.
- `gates`: the package knowledge-architect-gates, the library that runs a project's merge gates,
  with the recommended list of a Rust project that uses the checker.
- `agent-skills`: setup gains a section for a Rust project: one maintenance crate pins the
  checker and runs the gates library, with its aliases and a continuous integration workflow.
- `agent-skills`: the design skill, forked from designing-together 0.6.0; the retrospective carries
  its expectation set.
- `agent-skills`: every installed skill is named by its activity as a noun: design,
  decision-recording, issue-tracking, review, setup, goal-setting, agent-configuration, planning,
  retrospective.
- `agent-skills`: the transcript-conformity reviewer tells a scope change from an agent's addition,
  and the planning and review skills keep an addition until the owner rules.
