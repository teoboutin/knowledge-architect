# Review of the whole goal set: the scope narrowed to one kind of project, 2026-11-01

The record of a goal-setting batch of more than three goals, run as a review of the whole goal set
under `skill@knowledge-architect-goal-setting@whole-set-review` at the owner's request. The drafts
are in the goals homes of the working tree, uncommitted. This record leaves with the commit that
writes the goals the owner rules last.

## The owner's statements

The owner, 2026-11-01, verbatim:

> I want to change the orientation of the goals of the project a little bit. This is not a complete redirection, but it will narrow a few existing goals, and add or remove some.
> Basically, here's what I want:
> - The project should focus on serving a single type of project. It is made for my own use, in my own projects, and I aim at using a single, common toolchain as much as possible, because this allows me to centralize agentic instructions and mechanical tools, and be more efficient in each of my projects.
> The scope I want to focus on is pretty much the one of this project and thaum: A Rust project, were agents do all the work, that requires linear history. I work on both open source projects (such as tools, scientific codes) and private projects (mostly games). Those examples are just for context. I'm not certain what other specific aspects are important to mention here. You can find my two projects that currently use knowledge architect under ../rootpoint/ and ../games/thaum/ for reference if needed.
> Anyone stays free to employ it for their own projects, but if they do not follow the above conditions, the checker and/or the workflow are not guaranteed to function properly.
> - As stated above, centralizing generic tooling and procedures is a main goal. Basically, what happens is that when working in a project, I will set up tooling that serves the project and improves it (either for efficiency of development or for quality of the project), or carry a procedure with agents which can be generalized (such as external surveys, or new type of reviewer, or anything else, those are just examples). I want them to be generalize, so that they can start applying in my other projects when it makes sense. This should go through integration into knowledge-architect.
> This makes the project to become about a bit more than just documentation records of a project, which is fine.
> - I want to cover the full range of generic activities an agent carries in a project of the first point here (a Rust project). This goal is partially derived from the one above, which states that I want to centraliwe here any generic tooling and procedure, and an agent activity is one such procedure. This means at least the development part should be covered by installed text. Not about how to code in Rust, but about the procedures such as were to ground, what to be careful of, of to test properly, and how to review and repair (just like the current local skill AFAIK). For later, probably also the release steps, with exact publishing and versionning policy staying project local, but with generic guidelines, and mechanical tooling.
> Probably git workflow too (at least rebase/history rewrite rules). Except CI and PR workflow, because in private projects were I am the only contributor, I prefer to avoid using CI for PRs (mostly because minutes are limited on private, and PR have little value if there is a single contributor). Merge conditions stay unified to "cargo x gates" passes.
> - About the development part, I want it to carry the goal of promoting better code quality overall, with a focus on single responsibility of code, and testability. It should promote writing over two layers, the pure logic and the practical usage with "side effects", as well as promote compartimentalization of the project, under multiple crates/components which can focus on clearly separated goals, design and implementation.
>
> This is only going to be work on the goals for now.
> I want you to help me determine which existing goals should be removed or reworded, as well as propose new goals and drafts to add.
> Those goals will all enter unmet, which is fine, I plan to slowly work toward meeting them.
> In particular, I expect the goals ##owner-attention-on-what-is-theirs
> ##any-project-can-adopt-it and `goal@knowledge-architect@rust-projects-come-first` to change quite a bit, or be removed or absorbed.
>
> For context only, and with little details and argumentation (since we won't carry any design or implementation here), here are a few design consideration that I think would work toward those goals:
> - adding the development skill to the published set, under a generic shape, and probably taking some inspiration from the one in thaum which has been improved a bit.
> - having the retrospective  skill ask for any procedure or tooling that was put in place by the session, in order to improve efficiency of development or quality of the project(on subjects such as code quality, testing, publishing methods,reviewing ... non exhaustive), so that I can centraliwe them in knowledge-architect
> - having a skill on how to create a component, that maximizes compartimentalisation of the projects under smaller components, because this makes goals and design docs much easier to read and consult, and also tends to improve code quality itself. It also greatly simplifies refactors IMO.
> - having tools to help rewrite git history, such as rebases, or changing the message of a specific commit, which are often needed under linear history projects (or more generally due to the workflow which imposes strict quality of commit messages)

The backticks around the two `##` slugs of the owner's message are removed here, since the
checker reads a backticked `##` span as a slug definition.

And, during the same turn:

> since this is a large change, I would prefer yoou to use the whole set procedure for reviewing drafts, rather than the standard one

## The batch and its sources

| goal | change | source |
| --- | --- | --- |
| `serves-one-kind-of-project` (root) | new; absorbs `goal@knowledge-architect@rust-projects-come-first` and `goal@knowledge-architect@owner-attention-on-what-is-theirs` | the owner's statement, first point; the four conditions proposed from the root CLAUDE.md of thaum and of rootpoint, their sections on git |
| `every-target-project-can-adopt-it` (root) | rewords and renames `goal@knowledge-architect@any-project-can-adopt-it` | the owner's statement, first point; the rest kept from the approved text |
| `generic-tools-and-procedures-are-shipped` (root) | new | the owner's statement, second point; the measure proposed from the owner's second design consideration |
| `goal@knowledge-architect@agents-get-a-complete-workflow` | reworded, slug kept | the owner's statement, third point |
| `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` | reworded, slug kept: "the checker" becomes "a published crate"; history rewriting named | proposed from the owner's fourth design consideration and second point |
| `code-is-single-purpose-and-testable` (root) | new | the owner's statement, fourth point, its first intent |
| `projects-are-split-into-components` (root) | new | the owner's statement, fourth point, its second intent; the reasons from the third design consideration |
| `goal@knowledge-architect@setup-brings-quality-tools` | reworded: continuous integration and the branch for another language removed | proposed from the third point ("Except CI and PR workflow") and the first point |
| `goal@knowledge-architect@agents-work-without-drift` | the exception list drops `goal@knowledge-architect@owner-attention-on-what-is-theirs`, adds the scope goal | proposed, follows from the removals |
| `goal@knowledge-architect@design-with-agents-as-counterparts` | unchanged since round 1, which restored `goal@knowledge-architect@owner-attention-on-what-is-theirs` | |
| `goal@agent-skills@one-skill-per-activity` | the code exception removed; developing, branch work and releasing listed | proposed, follows from the third point |
| `goal@agent-skills@installed-text-works-anywhere` | reworded and renamed `installed-text-works-in-every-target-project`: "of any language" removed, may rely on what every target project holds | proposed, follows from the first point |
| `goal@gates@gates-from-a-list` | "A Rust project" becomes "A target project" | proposed, one reading of the term |
| the root goals home's head | rewritten for the new scope | proposed |
| `goal@knowledge-architect@rust-projects-come-first` | removed, absorbed by `serves-one-kind-of-project` | the owner's statement |
| `goal@knowledge-architect@owner-attention-on-what-is-theirs` | kept as at HEAD since round 1, as the default of Q5 | the owner's statement names it; round 1 found that no other goal holds its outcome |
| `goal@agent-skills@only-needed-instructions` | adds: an instruction a goal asks for is needed (round 1, Q14) | proposed |
| `goal@xtask@repeated-tasks-are-automated` | "the published checker's" becomes "a published crate's"; a task whose procedure would serve another target project is not local (round 1) | proposed, follows from the batch |
| `goal@knowledge-architect@the-workflow-improves-through-real-use` | "a contribution submitted" becomes "work meant to merge into that project's main branch" (round 1) | proposed, follows from the exclusion of pull requests |

## Consequences of the rulings

- A removed or renamed slug leaves references dangling, and the commit that writes the goals
  repairs each, as `skill@knowledge-architect-goal-setting@when-it-runs-again` says. Each citing text
  derived something from the goal, and is read again:
  - `goal@knowledge-architect@any-project-can-adopt-it`: 19 files beside the goals homes, design heads of the core mostly;
  - `goal@agent-skills@installed-text-works-anywhere`: 10 files, `path@agent-skills@CLAUDE.md` among them;
  - `goal@knowledge-architect@owner-attention-on-what-is-theirs`: none while Q5's default stands;
    were it removed, 4 files, among them `design@agent-skills@goal-batch-ruled-in-the-tree`, whose
    only goal it is, and two `rejected` entries that argue from it;
  - `goal@knowledge-architect@rust-projects-come-first`: 2 files; the issue
    `issue@knowledge-architect@the-reduced-mode-outside-rust-is-not-documented` loses the goal it
    serves, and closes as no longer wanted.
- Under Q9's default, no installed instruction assumes a pull request. The installed primer's
  coined terms and the planning skill define a spec and a slice by "one branch and one pull
  request", and the design skill speaks of "several pull requests": each changes in later work.
- `design@core@agents-table` argues `harness = []` from a project without agents, which the scope
  now leaves out; it is read again with the references to `goal@knowledge-architect@any-project-can-adopt-it`. The issue
  `issue@core@a-home-for-developer-contracts-outside-agent-configuration` says "a project without
  agents is a minor case today": wording that becomes false with the scope.
- The installed setup skill recommends continuous integration on every ready pull request. Under
  the reworded `goal@knowledge-architect@setup-brings-quality-tools` that recommendation no longer
  serves a goal; it changes in later work, not in this batch.
- This repository keeps its pull requests and its continuous integration: they become its own
  choice, and its root `CLAUDE.md` states them already.
- Every new goal enters unmet; per `skill@knowledge-architect-goal-setting@drawing-out-intent`, each
  one that no plan document schedules gets a `todo` issue in the commit that writes it.

## The boundary blocks

The draft of each goal is in its goals home. For each universal and each exclusion, a case it rules
out and a case it admits.

**`serves-one-kind-of-project`**

- "most of it Rust": admits rootpoint, which builds a part for the browser through wasm-bindgen, and
  thaum, with its client crate; rules out a Python project with one Rust extension module.
- "AI agents do all its work": admits the owner writing goals and ruling forks; rules out a project
  where persons write the code and agents only review it. It does not say the owner never edits a
  file by hand.
- "linear history": rules out a project that merges with merge commits, even with a passing gate
  command; admits a fast-forward merge done locally (thaum, rootpoint) and a rebase merge through
  GitHub (this repository).
- "one command that runs every gate": admits `cargo x gates`; rules out a project whose merge
  condition is a set of separate CI jobs. The command's name is not in the goal: it is a decision,
  per the goal standard's first rule.
- "Nothing built, kept or repaired only to serve a project that is not a target project": rules out
  the reduced mode of `goal@knowledge-architect@rust-projects-come-first`, its documentation issue, and the checker run alone
  in a project with no agents; admits a feature that also happens to work elsewhere. A defect that
  shows only outside a target project is closed as no defect.
- Siblings: absorbs `goal@knowledge-architect@rust-projects-come-first`; it is the scope every other goal reads "target
  project" from; `goal@knowledge-architect@agents-work-without-drift` ranks below it.

**`every-target-project-can-adopt-it`**

- "names no path or convention particular to one target project": rules out the installed text
  naming the layout of thaum's maintenance tool alone; admits it naming Cargo, a workspace, and the one gate
  command, which every target project holds.
- "a new one is the primary target": unchanged from the approved text.
- Siblings: refined by `installed-text-works-in-every-target-project`.

**`generic-tools-and-procedures-are-shipped`**

- "would serve another target project": admits thaum's census of rule coverage only in a generic
  part, if any (a coverage record kept by a tool); rules out shipping its Comprehensive Rules
  adoption, which no other target project has. Admits the local `skill@klarch-development`, whose
  procedure is generic, so this repository fails the goal's second bullet until it ships.
- The second bullet ranges over this repository's own skills, agents and commands: `klarch-` skills,
  `klarch-` agents, and `cargo x` commands. It admits `cargo x changelog`, if the changelog procedure
  is particular to this repository; it rules out keeping `cargo x skill` local if another target
  project ships installed text, which none does.
- Siblings: touches `goal@xtask@repeated-tasks-are-automated`, which keeps the local part of a
  repeated task; touches `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

**`goal@knowledge-architect@agents-get-a-complete-workflow`**

- "every generic activity": admits development, branch work and release; rules out a skill on how
  to write Rust code, and one on the project's domain.
- "No installed skill covers either": rules out an installed skill for continuous integration or for
  pull requests; admits a project's own skill for them, and admits the installed text naming the
  merge condition, the one command.
- Siblings: refined by `goal@agent-skills@one-skill-per-activity`; tension with
  `goal@agent-skills@only-needed-instructions`, which bounds what each of the new skills holds.

**`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`**

- "a published crate": admits a history tool in `knowledge-architect-gates` or in a new crate, not
  only in `klarch`; rules out a task left to a script each session writes.

**`code-is-single-purpose-and-testable`**

- "the default shape of a change": admits a departure the change argues at the code; rules out an
  installed text silent on the two layers.
- "It does not cover how to write Rust itself": rules out idiom guidance, such as error types or
  lifetimes; admits the two layers in a Rust example.
- Siblings: refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

**`projects-are-split-into-components`**

- "a purpose that no existing Component's goals hold": admits a new crate for a new rendering
  backend; rules out a new Component for a feature that an existing Component's goals already state.
- Siblings: touches `goal@core@relocation-is-one-manifest-edit`, which keeps moves cheap.

**`goal@knowledge-architect@setup-brings-quality-tools`**

- rules out the setup skill recommending a CI shape as one of its tools; admits the setup skill
  stating that CI is the project's choice.

## Round 1

One reviewer per set in draft mode and one in whole-set mode, each brief naming this record. The
count of the stop rule is the findings that are not `minor`, every reviewer together: 79, so the
review stops on that rule once a round counts 15 or fewer.

| reviewer | findings | not minor | contradiction |
| --- | --- | --- | --- |
| set A, scope and adoption | 13 | 11 | 1 (b) |
| set B, workflow coverage and tooling | 17 | 16 | 2 (b) |
| set C, code quality and Components | 12 | 12 | 0 |
| set D, the record and the checker | 14 | 14 | 0 |
| set E, the owner and agents | 5 | 5 | 0 |
| whole set | 24 | 21 | 1 (b) |
| total | 85 | 79 | 4 |

The findings, condensed, each with its outcome. The same finding from several reviewers is one line,
its sources named.

- **"A project" ranges over two sets** (D F1 and its 8 sites, A S1, whole F1, F2): every goal that
  speaks of a project, an adopter or every project now ranges over target projects alone, by one
  sentence of `serves-one-kind-of-project`. Repaired.
- **The removal of `goal@knowledge-architect@owner-attention-on-what-is-theirs` drops its outcome and its rank** (A R1
  contradiction (b), E F3, whole F5): the goal is restored as at HEAD, and its rank over
  `goal@knowledge-architect@agents-work-without-drift` with it. Default, Q5.
- **The clause moved into `goal@knowledge-architect@design-with-agents-as-counterparts` reads two ways and narrows its
  source** (E F1, F2, whole F18): moot once Q5's default restores the source goal; the clause is
  removed. Repaired.
- **The owner's part reads as a closed list** (E F4, A S5): replaced by a pointer to the two goals
  that give it. Repaired.
- **"All its work"** (E F5, A S2): Q1.
- **The unified gate command** (A S3, T1, whole F4, B met outside): Q2.
- **"Kept" goes further than "not guaranteed"** (A S4, whole F3): Q3.
- **No goal tells an outside user the conditions** (A S7, whole F7 minor): Q4, which also gives the
  scope goal an observable measure.
- **"Published tools"** (A S6, whole F6): replaced by the setup skill and the published crates'
  READMEs in the new measure. Repaired.
- **"One Cargo workspace" rules out a generated second workspace** (whole F8): "its code is Rust,
  built by Cargo". Repaired.
- **"This repository is a target project" stated twice** (A E1, minor): kept in
  `every-target-project-can-adopt-it` alone. Repaired. **A E2** (minor, several intents in the
  approved text): no change, carried over from the approved goal.
- **The head drops the quality aim, shortens the definition, and names one main purpose where the
  owner names centralizing as "a main goal"** (D H1, H2, H3, B 11): the head restates the aim, points
  to the definition, and names two main purposes, drift first. H1 and H2 repaired; the rank is Q6.
- **`goal@knowledge-architect@agents-work-without-drift`** (D D1, D2, D3, D4): "a target project's work", no "more of"; it
  never ranks over the scope goal whole. Repaired; the boundary blocks of round 1 are below.
- **Release and branch work were hedged by the owner** (B 1 contradiction (b), whole F14): Q7.
- **"Generic" has two meanings, and releasing is no activity of every target project** (B 2, 10,
  15, whole F9): a generic activity is one more than one target project carries out; one that only
  some carry out is covered for those. Default, Q8.
- **"can run neither" turns the owner's preference into an impossibility** (B 3 contradiction (b),
  whole F11, A and E met outside): the goal states the owner's reasons, minutes and value. Repaired.
- **The exclusion of continuous integration and pull requests reads two ways, and conflicts with
  shipping a CI tool** (B 4, A Q1, whole F12): Q9.
- **Merging is covered or excluded** (B 5): merging once the scope goal's condition holds is listed
  under branch work. Repaired.
- **The release item names tools** (B 6, minor): removed; tools are
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`'s. Repaired.
- **Rewording a commit takes judgement** (B 7, whole F16): the command applies what the session
  gives it. Repaired. **A design consideration became a named case** (B 8): Q13.
- **"Every generic tool" names no set** (B 9, whole F19): "the generic part of each such tool and
  procedure". Repaired.
- **The retrospective named in the measure is a mechanism** (C met outside, whole F20): "reported to
  knowledge-architect's owner". Repaired. **The measure leaves out this repository's `CLAUDE.md`
  files** (whole F21, minor): added. Repaired.
- **"Submitted" assumes a pull request** (B 12): "work meant to merge into that project's main
  branch". Repaired.
- **The code exception removed from `goal@agent-skills@one-skill-per-activity` widens it** (B 13, whole F10, C C4):
  the parent states the exclusion as Rust idiom and the domain, and says the shape of the code is
  covered; the refinement ranges over generic activities as the parent defines them. Repaired.
- **Code review sits in two activities** (B 14, whole F13, C C5): developing holds when code is
  reviewed during development, on which axes, and its repair; reviewing holds the dispatch. Default,
  Q11.
- **`goal@xtask@repeated-tasks-are-automated` still names the checker, and "local" overlaps "generic"** (B 16,
  17, whole F17): reworded. Repaired.
- **The new goals compete with `goal@agent-skills@only-needed-instructions`** (whole F15, C on P3): Q14.
- **The two layers and "promote"** (C C1, C2): Q10. **"Better code quality overall" dropped** (C C3):
  restored in the title and the body. Repaired.
- **Components** (C P1, P2, P3, P4, P5, P7, whole F22, F23): the primer's sense of Component, one
  package, several or none; a default a design may argue against; applied when work is designed;
  one purpose, one Component meets it; "a change to the structure of the code" for "a refactor".
  Q12. **The procedure that creates a Component is a mechanism** (C P6): removed from the goal.
  Repaired.
- **`goal@gates@gates-from-a-list` reads as a fact about every target project; thaum runs its own** (A G1): Q15.
- **"Used alone" in `goal@knowledge-architect@documentation-stays-consistent`** (D C2): read as a
  target project that runs the checker without the workflow; no change of text. Default, Q16.
- **Rule 4 on the CI exclusion** (whole F24, minor): no change; it qualifies what the workflow
  covers.

The reviewers' boundary blocks of round 1 stand beside the blocks above, as the reports gave them;
the next round's reviewers derive them again from the repaired text.

## Questions put to the owner

Each with the session's default, which the drafts hold.

- **Q1.** A target project's work is done by agents. Default: an edit a person makes now and then,
  the owner or an outside contributor, leaves it a target project. Case: an outside pull request to
  rootpoint, or the owner fixing a typo by hand.
- **Q2.** "Merge conditions stay unified to cargo x gates passes." Default: the goal says one
  command, the same in every target project; its name, `cargo x gates`, goes in a design head, so
  the installed text may write it. Case ruled out: a target project whose gate command is
  `just gates`.
- **Q3.** "Not guaranteed to function properly." Default: nothing is built or repaired only for a
  project outside the target, and what exists only for one may be removed, without being required to
  go. Case: the setup skill's branch for another language may stay unmaintained, or be removed.
- **Q4.** Default: the scope goal is met while the setup skill and each published crate's README
  state the four conditions and that nothing is guaranteed outside them.
- **Q5.** Default: `goal@knowledge-architect@owner-attention-on-what-is-theirs` stays as it is, with
  its rank over `goal@knowledge-architect@agents-work-without-drift`. Removed, nothing would rule out an installed step that
  hands the owner a raw review log to read, since `goal@knowledge-architect@the-owner-decides` bounds
  only what is sent for a ruling.
- **Q6.** Default: two main purposes, drift first and centralizing second. Case: a generic procedure
  proven in thaum whose installed form would weaken a step drift relies on; drift wins.
- **Q7.** You wrote "for later, probably" for releasing and "probably" for the git workflow. Default:
  both are listed now and enter unmet, like the other new goals.
- **Q8.** Default: a generic activity is one that more than one target project carries out, and
  releasing is covered for the target projects that release. Case: thaum, which publishes nothing,
  gets the release skill installed; whether it loads there is a design question.
- **Q9.** Default: the installed text requires neither continuous integration nor pull requests,
  and no instruction assumes either; the setup skill may offer a CI shape, such as thaum's weekly run
  on main, and requires none. Consequence: the installed definitions of a spec and a slice by "one
  pull request" change in later work.
- **Q10.** Default: the three properties are the default shape of a change; a departure is argued at
  the code, and the review judges whether it is. The side-effect layer "takes no decision a test of
  the logic should cover", and needs no tests of its own. Case: a retry loop moves to the logic; a
  thin `main` that reads a file and prints a result needs no test.
- **Q11.** Default: developing holds when code is reviewed during development, on which axes, and
  its repair; reviewing holds how a review is dispatched.
- **Q12.** Default: a Component may hold one Cargo package, several or none; a new purpose goes in a
  new Component unless the design argues otherwise; a project with one purpose and one Component
  meets the goal. Case admitted: a history tool placed in the gates crate, where its design argues
  it.
- **Q13.** Default: rewriting a branch's history is named as a task of
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, though you gave it as a design
  consideration.
- **Q14.** Default: `goal@agent-skills@only-needed-instructions` says that an instruction a goal asks
  for is needed. Case: sessions keep I/O out of the logic unprompted; the instruction stays.
- **Q15.** Default: `goal@gates@gates-from-a-list` says a target project *can* get its gates from
  the library. Case: thaum runs its own runner and stays a target project.
- **Q16.** Default: in `goal@knowledge-architect@documentation-stays-consistent`, "used alone" means
  a target project that runs the checker without the workflow, and the guarantees hold there.

## Questions still open

Q1 to Q16.
