# A defined id written outside a reference is a finding, and every id has two or more segments

## Status and audience

This spec is the plan of the work that makes the checker report a defined id written anywhere it
reads prose without the reference form: a reference-shaped token outside backticks, a backticked
span that is exactly an id, and a plain word that is an id. To keep those findings meaningful,
every id the checker defines takes two or more segments.

- It is written for a session that did not witness the design discussion that produced it.
- It leaves the repository in the commit that completes its harvest.
- Where it and a design home disagree, on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits.
- **Assembled from one transcript**, the session's log, whose discussion begins at the owner's
  message "For the citations: we'll do the repair, and try to think of a more general, long term
  fix, in the same PR, as a spec that we will merge with it." A round here is one owner message and the replies to it, numbered from that message.
- **Its arguments are numbered `arg-<n>`**, the numbering its step 1 gives the planning skill, which
  still prescribes `a<n>` until then, so the spec passes the grammar it plans.
- **The spec lands before its work**, on the branch `backtick-comment-citations` with the repair
  of 15 bare citations it grew out of, on the owner's word, round 1: "in the same PR, as a spec
  that we will merge with it". Its first step changes what `cargo klarch commits` judges in every
  tree of a branch, so on one branch it would have to come first, per
  `skill@knowledge-architect-planning@plan-reviews`.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, as the work of a spec. The Rust work is done
under `skill@klarch-development`.

## Names

| name | what it is |
| --- | --- |
| a defined id | a name the entity table defines and a reference names: a register entry's id (design, goal, tripwire, issue), a plan document's id (a spec's, a milestone's, a slice's), a plan item's id inside its own plan, a section slug of a harness kind (a skill's, an agent's, the primer's, the root `CLAUDE.md`'s), and a skill's or an agent's name. A register's name and its directory are not ids: they are the kind and the home a reference names, and stay as they are |
| class A | a reference-shaped token outside any backtick span: a kind the entity table knows, `@`, and the segments a reference of that kind takes, holding no `<` or `>` |
| class B | a backticked span that is exactly a defined id |
| class C | a plain word, outside any backtick span, that is exactly a defined id |
| the three lints | classes A, B and C, each a lint of the last phase |
| the class heads | the heads the harvest writes for the three lints: `design@core@candidate-rule-and-retired-forms` rewritten for class A, `design@core@bare-skill-name-reported` rewritten for class B, and a new head for class C |
| a subject slug | a slug that names what a decision is about, such as a domain term, rather than the decision; `primer@design-heads` asks that a slug stay aligned with its decision's full scope |
| a quotation | text between `"` and `"`, or `“` and `”`, within one paragraph, or a blockquote line beginning with `>`; one left open ends at its paragraph. In a Rust comment, a paragraph is a run of consecutive comment lines; a string literal's own delimiters open no quotation |
| the census | the measurement taken in rounds 1, 3 and 4 by the session's scripts, which are not in the tree; its figures are the arguments', and the built lints supersede it as the instrument |
| `ENTITY_ID` | the regex of `path@core@src/entity.rs` behind `is_entity_id`, `^[a-z0-9]+(?:-[a-z0-9]+)*$` |
| `is_id` | the copy of the grammar in `path@agent-skills@src/render.rs` that validates a `{{slug:…}}` placeholder of the shipped text |

## What the work is

**What exists today.**

- The id grammar is `[a-z0-9]+(-[a-z0-9]+)*`. It is checked at several sites, which the mapping
  table lists: `ENTITY_ID`, behind `is_entity_id`, for file-register entry ids, milestone names and
  path segments, skill and agent names, and declared register names and directories; the heading
  patterns `SLUG_SITE` and `SLUG_ANY` of `path@core@src/scan.rs`, which record a heading slug only
  in the grammar, so a heading slug outside it is not recorded and yields "carries no slug" at a
  register's level; the span patterns `BARE_NAME` and `NAME_WHOLE`; and `is_id` in
  `path@agent-skills@src/render.rs`.
- The grammar is stated in `design@core@an-entry-is-a-heading-at-the-register-level` ("An id is
  `[a-z0-9]+(-[a-z0-9]+)*`"), in `design@core@unusable-harness-name-is-a-finding` for skill and
  agent names, and in `path@core@README.md`, its section on milestones. The installed text restates
  it in `primer@checker-model` ("An id matches"), in `primer@design-heads` ("The slug is an id in
  the grammar"), in `skill@knowledge-architect-issue-tracking@issue-entry`, in
  `skill@knowledge-architect-design@threads-and-states` and in
  `skill@knowledge-architect-planning@spec-contents`, which numbers arguments `a<n>` in its table
  of sections and in its Items paragraph.
- The scanner records a backticked one-word span in the grammar, and one wrapped across a line break
  at a hyphen, as `Observation::BareName`. The match arm `Observation::BareName(name) if lints` of
  `judge_part` in `path@core@src/check/references.rs` reports one only where it is a skill's or an
  agent's name, the skill chosen where both kinds hold the name, per
  `design@core@bare-skill-name-reported`. A register's id in that form is silent, and so is any
  reference-shaped token outside backticks, since every span pattern needs backticks, per
  `design@core@candidate-rule-and-retired-forms`, and any plain word. The model dump of
  `path@core@src/model.rs` omits `BareName` on purpose.
- The issue `a-bare-mention-of-a-deleted-entry-is-never-reported`, a `todo` this commit closes,
  held the owner's preferred fix, "report as a finding any bare mention of a declared slug", with
  the owner's hedge, "Maybe it is too costly, unsure about this." Its evidence was finding C3 of
  2026-10-07-knowledge-architect-workflow: reviewers found three bare slugs of deleted entries on
  two branches, repaired before the merge. Its mechanism: `cargo klarch show` lists checked
  references only, and nothing makes the session that deletes an entry look for its bare mentions.
  It asked for a census first, and whether the rejected alternative "The retired slug reference
  reported whatever it names" of `path@core@docs/rejected-alternatives.md` covers this check.
- One-word ids in this tree:
  - the 30 argument items of `spec@plans@path-quickfixes`;
  - the installed section `skill@knowledge-architect-project-audit@outcomes`, cited by two agents,
    the skill itself and two heads of agent-skills;
  - in the mock projects under `path@core@tests/projects/`: dirhome's `a1` twice and `checked`
    once; the `unsound` mock's spec and milestone `twice` and milestones `defective` and `widget`,
    pinned by tests; `Not_An_Id`, a planted invalid id;
  - in the unit tests of the core and of agent-skills, one-word slugs in fixture strings, 61 lines
    by `git grep -nE '`##[a-z0-9]+`'`, and `{{slug:a}}`, `{{slug:a1}}` in `path@agent-skills@src/render.rs`.

**How the deletion case is closed.** The three lints make every mention of a live entry a
reference, or a finding until it is one. When the entry is then deleted, its references dangle and
`cargo klarch check` reports them, as it reports every dangling reference. So the case the issue was
opened for, a bare mention left by a deletion, no longer arises, except a mention written bare in
the very commit that deletes the entry; a commit message's lints read its first parent's tree too,
per `design@core@a-commit-message-is-a-document`, which reports that one in the message.

**Outside the work.**

- The `#<id>` form: the design skill prescribes `#slug` for a thread in prose, and the plain `#`
  item name is the open question of `issue@core@a-plain-hash-item-name-is-read-by-nothing`.
- Writing the repair: `check --fix` writes nothing for these findings; respelling is the subject of
  `spec@plans@path-quickfixes`, held on the owner's word (#lint-is-report-only).
- An id of another project, named in prose: it is not defined here, so it is silent, per
  `issue@core@cross-project-references`.
- A register's name and its directory keep the one-or-more grammar.

## What is already decided

**Decisions the design rests on, not argued again:**

- `design@core@finding-names-the-repair`: each finding names its repair.
- `design@core@grammars-not-prefixes`: which bytes of a file are prose. A fenced block is prose; a
  string literal of the tool's own source is data, per `design@core@checker-source-literals-are-data`,
  so in this repository the literals of `path@core@src/` are outside the lints.
- `design@core@a-commit-message-is-a-document`: a message's lints stand where its commit's tree or
  its first parent's holds what they name; the three lints inherit that union.
- `design@core@plan-item-scope`: a plan item is cited from inside its own plan only.
- `design@core@phases-gate-the-report`: each lint consumes the entity table in the last phase and
  produces nothing another check reads.

**Decisions the work reverses or rewrites, and every text that references each.** The texts are
the lists `cargo klarch show` prints, outside this spec. Each is read at the step that changes what
it states, and else at the harvest.

| decision | change | referencing texts | judged or updated by |
| --- | --- | --- | --- |
| `design@core@candidate-rule-and-retired-forms` | rewritten: a reference-shaped token outside backticks is a finding (#reference-shape-outside-span), and its "A span with no `@` is read by three lints, and by nothing else" changes with class B | `skill@klarch-release`, its line 59; `path@agent-skills@content/agents/routing-reviewer.md`:40, :54; `register@agent-skills@design`:957, :1100, :1185; `path@core@README.md`:111; `register@core@design`:1365, :1428, :1565, :1645, :1728, :2107; `issue@core@a-plain-hash-item-name-is-read-by-nothing`, 4 lines; `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head`; `issue@core@four-heads-bundle-decisions-with-many-citing-sites`, 3 lines; `path@core@docs/rejected-alternatives.md`:97, :361, :369, :486; `tripwire@core@candidate-rule-silence`; `path@core@src/check/references.rs`:21; `spec@plans@path-quickfixes`, 4 lines | the comment of references.rs and the routing reviewer at step 1; the rest at the harvest |
| `design@core@bare-skill-name-reported` | rewritten: a backticked span that is exactly any defined id is reported, and a name both a skill and an agent hold gets the candidate list rather than the skill (#backticked-bare-id, #ambiguous-id-candidates) | `register@agent-skills@design`:1170; `path@core@README.md`:107; `register@core@design`:1477, :1570; `issue@core@tooling-for-project-skills`; `register@knowledge-architect@design`:213; `spec@plans@path-quickfixes`, 7 lines, which plans its own rewrite of this head and is read when either spec's work starts | the README at step 1; the rest at the harvest |
| `design@core@an-entry-is-a-heading-at-the-register-level` | its statement of the grammar becomes two or more segments (#two-segment-ids) | `path@agent-skills@content/agents/routing-reviewer.md`:40; `register@agent-skills@design`:2942; `path@core@README.md`:354; `register@core@design`:1272, :1382, :1532; `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`; `path@core@docs/rejected-alternatives.md`:380 | step 1 for the installed text and the README; the harvest for the rest |
| `design@core@unusable-harness-name-is-a-finding` | its grammar for skill and agent names becomes two or more segments | `issue@core@tooling-for-project-skills`; `path@core@src/entity.rs`:1349, :3059 | step 1 for the source; the harvest |
| `design@core@plan-document-kinds` | its example of a one-word slice id, `harvest`, takes two segments | its 12 citing lines, which state no grammar | the harvest |
| `design@agent-skills@plain-text-is-no-repair` | its closed set of exceptions gains the quotation, which the owner approved in round 4; and class A keeps its first exception mechanically, under D2 | 18 citing lines, the primer's `primer@where-knowledge-goes` and `tripwire@agent-skills@plain-text-pointer-found` among them | the harvest, after D2 and D3 |
| `design@knowledge-architect@changelog-section-fixed-at-release` | a member within the owner's rule of round 4, that a released section's content never changes and its structure may: a slug written in words in a released section is repaired as its reference, each such finding ruled by the owner case by case, as the owner reserved (D5) | 10 citing lines, the changelog reviewer among them | the harvest |

**Standing entries the discussion's search returned, and their outcome:**

| entry | outcome |
| --- | --- |
| the issue `a-bare-mention-of-a-deleted-entry-is-never-reported` | closed by the commit that adds this spec; its content is in "What the work is" |
| `tripwire@core@candidate-rule-silence` | fired on the census; narrowed in this branch to except this spec's work, citing it, per `skill@knowledge-architect-issue-tracking@movement-instruction`; its instance, the bare `tripwire@agent-skills@ruling-lost-in-change` in an issue entry, is repaired at step 1; the harvest restates it for the shapes still silent, a typo inside the kind among them, or deletes it |
| `tripwire@agent-skills@plain-text-pointer-found` | names "a mechanical check for plain-text pointers" among its candidates; not fired; read at the harvest with `design@agent-skills@plain-text-is-no-repair` |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing` | outside the work; its re-entry, the next change to `design@core@candidate-rule-and-retired-forms`, is this work, so the harvest reads it |
| `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head` | its close is a statement in `design@core@candidate-rule-and-retired-forms`; the harvest's rewrite of that head settles or carries it |
| `issue@core@cross-project-references`, `tripwire@core@item-of-another-plan-named` | an id of another project or another plan is not defined here, so silent; nothing |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | class A covers its path and kind spans written without backticks; the entry stays open for those written with them |
| `tripwire@core@checker-source-literals-are-data-self-location` and its two siblings | the literals of the tool's own source stay data; the lints read its comments, where the census found `primer@checker-model` written bare in a doc comment of `path@core@src/agents.rs`, repaired at step 1 |
| `issue@core@four-heads-bundle-decisions-with-many-citing-sites` | its re-entry is this change to `design@core@candidate-rule-and-retired-forms`; read at the harvest |
| `issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` | the same repair-naming question for every register; #ambiguous-id-candidates answers it for these lints |
| `issue@core@a-message-lint-is-deduplicated-in-one-tree-only` | the lints inherit the message union; read at step 1's tests |
| `issue@agent-skills@a-premortem-criterion-listed-as-a-default-reads-two-ways` | not its case: the owner applied AC2; only its threshold is a default, D4 |
| `issue@core@tooling-for-project-skills`, `issue@core@references-are-read-in-markdown-and-rust-only`, `issue@core@the-rust-parser-choice-has-no-head`, `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry` | bear on the regions the lints read; nothing changes in them |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | the retrospective's question on references written without backticks is the channel it names; under D2 class A keeps the escape that question watches, so the channel is unchanged; read at the harvest |
| `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project` | a saved workflow's `//` comment is JavaScript, which the checker does not read, per `issue@core@references-are-read-in-markdown-and-rust-only`; the lints do not reach it; nothing |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | the lints report section slugs written outside a reference; a point inside a section written in words stays that entry's question; nothing |
| `tripwire@agent-skills@rarer-rule-in-the-model` | the lints' rules go into their finding messages and the core's README, not into `primer@checker-model`, so it does not fire; step 1 adds nothing to the primer's model |
| `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument` | the owner ruled that released sections are linted, findings ruled one by one (round 4); read at the harvest |

**The rejected alternative the issue asked about**, "The retired slug reference reported whatever
it names", lost because a retired form's matches were other projects' own notation. The three
lints match defined ids of this project only, so that argument does not reach them; the alternative
stays as it is.

## Criteria

### A dangling mention is found by the check, not by a reviewer who happens to read it `##mention-found-by-check`

Binding, from `goal@knowledge-architect@documentation-stays-consistent`. Met by
#reference-shape-outside-span, #backticked-bare-id and #plain-word-lint.

### Every finding has a repair the writer may make `##finding-has-a-repair`

Binding, from `goal@knowledge-architect@any-project-can-adopt-it`, as the owner's rulings of rounds
2 and 4 frame it: the reference, or a rename of a subject slug; otherwise the owner's ruling, case
by case. Met by #plain-word-lint with its quotation exemption, and by #ambiguous-id-candidates.

### Every finding names its repair `##finding-names-repair`

Binding as a presumption, from `design@core@finding-names-the-repair`. Met by
#ambiguous-id-candidates.

### The bare slugs the record writes on purpose stay silent `##deliberate-bare-slugs-silent`

Binding as a presumption, from `design@agent-skills@plain-text-is-no-repair`. Met: a removed
entry is not defined.

## Threads

### A reference-shaped token outside backticks is a finding in every prose region the checker reads `##reference-shape-outside-span`

Proposed by the session in round 1 (`argument@defined-ids-outside-references@arg-6`, `argument@defined-ids-outside-references@arg-7`). Approved by the owner in round 2:
"reference-shape-outside-span: agreed." Shape: Decided design, "Class A". Harvest:
`design@core@candidate-rule-and-retired-forms`, rewritten.

### Every id the checker defines has two or more segments, plan argument items numbered `arg-<n>` `##two-segment-ids`

Proposed by the owner in round 2 (`argument@defined-ids-outside-references@arg-14`): "Honestly, I think that forbidding one word slugs might be a
better solution here." Argued against by the session at the owner's request, round 3 (`argument@defined-ids-outside-references@arg-18` to `argument@defined-ids-outside-references@arg-21`).
Approved by the owner in round 4: "two-segment-ids with arg-n approved". Shape: Decided design,
"The grammar", and the mapping table. Harvest: a head of `register@core@design` whose slug is the
thread's, and the statements the Harvest table lists.

### A backticked span that is exactly a defined id is a finding `##backticked-bare-id`

Proposed by the session in round 1 (`argument@defined-ids-outside-references@arg-8` to `argument@defined-ids-outside-references@arg-11`), with a one-word exception the owner's
#two-segment-ids made unneeded (`argument@defined-ids-outside-references@arg-20`). Closed `argued` in round 4, its rivals out: the one-word
exception by `argument@defined-ids-outside-references@arg-20`, no finding by `argument@defined-ids-outside-references@arg-10`. In the checkpoint the owner confirmed in round 5, "threads
agreed". Shape: Decided design, "Class B". Harvest: `design@core@bare-skill-name-reported`,
rewritten.

### A plain word that is a defined id is a finding, quotations exempt `##plain-word-lint`

Proposed by the owner in round 1 (`argument@defined-ids-outside-references@arg-1`) and round 2 (`argument@defined-ids-outside-references@arg-15`): "I would still lint. And if the linted term
is meant as an ordinary word, it is probably that the slug is wrongly chosen and not sufficiently
specific." Argued against by the session at the owner's request, round 3 (`argument@defined-ids-outside-references@arg-22` to `argument@defined-ids-outside-references@arg-26`). Approved by
the owner in round 4: "plain-word-lint approved", with the quotation exemption, "quoting verbatim: I
can agree on that, it seems very reasonnable", no changelog exemption, and the concept-index use
ruled out (`argument@defined-ids-outside-references@arg-29`, `argument@defined-ids-outside-references@arg-30`). Shape: Decided design, "Class C". Harvest: a head of `register@core@design` whose slug is the thread's.

### Section slugs are linted as register ids are `##section-slugs-in-scope`

Raised by the session in round 4 on the owner's "whatever it is" (`argument@defined-ids-outside-references@arg-1`), with the census (`argument@defined-ids-outside-references@arg-32`);
confirmed with the checkpoint in round 5. Shape: Decided design, "What is defined". Harvest: the
body of the class heads.

### An id defined at several places gets a finding that lists every candidate and chooses none `##ambiguous-id-candidates`

Proposed by the session in round 4 (`argument@defined-ids-outside-references@arg-33`); confirmed with the checkpoint in round 5. Shape: Decided
design, "The finding". Harvest: the body of the class heads.

### The lints report, and `check --fix` writes nothing for them `##lint-is-report-only`

Proposed by the session in round 4 (`argument@defined-ids-outside-references@arg-34`); confirmed with the checkpoint in round 5. Shape: Decided
design, "The finding". Harvest: the body of the class heads.

### A plain mention is listed by `show`, not reported `##plain-word-mention`

Proposed by the session in round 1 (`argument@defined-ids-outside-references@arg-12`,
`argument@defined-ids-outside-references@arg-13`). The owner answered in round 2, "I would still
lint", and asked first for the argument against it: "Please try to argue against my two points, I
really want to put them under stress before going with that." Its premise, the noise, was defeated
by the census of round 3 (`argument@defined-ids-outside-references@arg-22`). Ruled out at the
checkpoint of round 4, after "plain-word-lint approved", which the owner confirmed in round 5,
"threads agreed". Harvest: the losing alternatives.

## Arguments

### The owner's proposal and its open cost `##arg-1`

Round 1, the owner. "the checker could search the whole checked corpus of files for any string that
matches a defined slug (whatever it is, design, goal, issue...) which is not in the correct format
of a citation, and lint it. However, I'm not sure what the cost would be, to scan for all of that."

### The cost is a fact to measure `##arg-2`

Round 1, the session. "The cost question can be measured on this tree, so I'll measure it rather
than guess."

### The census fires the candidate-rule tripwire `##arg-3`

Round 1, the session. `tripwire@core@candidate-rule-silence` fires on "a backticked span that was
meant as a reference and for which `cargo klarch check` reported nothing": the bare
`tripwire@agent-skills@ruling-lost-in-change` written bare in an issue here, and thaum's README lists of heads by bare slug.

### The time cost is negligible `##arg-4`

Round 1, the session, measured. Markdown lines read 31,424 here and 40,284 in thaum; declared ids
517 and 535; Python, id table included, 1.6 s and 0.8 s. "In the checker it is a hash-set lookup
per span or per word, inside a scan it already runs. The real cost is false positives, and it
depends on the class."

### Pointers and non-pointers by class `##arg-5`

Round 1, the session, measured. Class A: 0 / 0 here, plus 15 real hits before the repair of this
branch; 0 / 0 in thaum, 7 hits being the script's span parsing. Class B: 1 / 4 here, the 4 being
`a1` and `a2`; about 11 / 8 in thaum; with ids of two or more segments and plan items inside their
plan, 1 / 0 and about 11 / 3. Class C: 4 / 15 here; 0 seen / 713 in thaum, 661 of them one-word ids.

### A known kind keeps class A from matching an address `##arg-6`

Round 1, the session. "The token must start with a known kind and have the segments of a
reference. So an email address, a URL or a git remote never matches: their head is no kind."

### Scoping class A to comment lines adds a rule with no reason `##arg-7`

Round 1, the session. "the same token is the same defect in any prose, and a special case for one
file kind adds a rule with no reason behind it."

### Class B is the register version of the bare-name lint `##arg-8`

Round 1, the session. "This is the register version of `design@core@bare-skill-name-reported`." A
removed entry is silent, since it is no longer declared; a slug still to be created is reported
once its entry exists, whose reference is then the right repair.

### Class B's measured precision `##arg-9`

Round 1, the session. "this repository 1 of 1, thaum about 11 of 14. Thaum's 3 false hits are
thread names backticked in a plan table, against the design skill's own rule ("never in
backticks")."

### Class B's rivals `##arg-10`

Round 1, the session. "include one-word ids. That adds thaum's 8 ordinary words, every one a false
finding." "no finding, only a listing. That leaves the 12 pointers found today silent."

### The noise is the owner's to weigh `##arg-11`

Round 1, the session. "About 3 false findings per 14 in thaum is a cost a consumer pays at an
upgrade."

### A listing in show for plain words `##arg-12`

Round 1, the session. "`cargo klarch show <ref>` gains a list, "mentioned in words at", of the
plain-text occurrences of the entry's id."

### The plain-word finding loses on the first census `##arg-13`

Round 1, the session. "in thaum, 713 hits and none meant as a pointer in the sample, such as the
rules term "self-replacement" and the word `combat`."

### Forbid one-word slugs `##arg-14`

Round 2, the owner. "Honestly, I think that forbidding one word slugs might be a better solution
here. Do you think the one word slugs of thaum are justified ?"

### A plain hit means the slug is badly chosen `##arg-15`

Round 2, the owner. "I would still lint. And if the linted term is meant as an ordinary word, it is
probably that the slug is wrongly chosen and not sufficiently specific."

### Put both points under stress `##arg-16`

Round 2, the owner. "Please try to argue against my two points, I really want to put them under
stress before going with that."

### A rename is no cost `##arg-17`

Round 3, the owner. "a mechanical migration/rename is not a cost. A cost is about what future
consumers will no longer be able to do, and why they would want to do it."

### Thaum's one-word ids `##arg-18`

Round 3, the session, measured. Thaum: 28, of which 20 plan argument items `a1` to `a20`, and 8
issues named after a feature area: `combat` (68 references), `counters` (46), `variants` (48),
`attachment` (33), `planeswalkers` (29), `multiplayer` (19), `battles` (17), `teams` (13). This
repository: 30 argument items and one planted invalid id.

### The umbrella-issue case for one-word ids is weak `##arg-19`

Round 3, the session. A feature's own name stays true while its issue's claim changes; "a two-word
id that names the subject without a claim, such as `combat-coverage`, keeps the same stability. The
consumer loses a few characters, not a capability."

### Two segments beat a one-word exception `##arg-20`

Round 3, the session, measured. It removes thaum's class B non-pointers from issue ids and its 661
one-word class C hits; "Under the exemption, a mention of the `combat` issue would stay invisible
to every check for good. Under your rule it becomes checkable."

### Argument items take the grammar too `##arg-21`

Round 3, the session. "One grammar for every id, with items numbered `arg-1`, `arg-2`. This needs a
change to the planning skill. No consumer loses anything." The rival, plan items exempt, "adds an
exception, justified only by numbered items being scoped."

### Every non-pointer plain hit comes from a subject slug `##arg-22`

Round 3, the session, measured. `self-replacement`, 41 markdown and 46 Rust-comment hits, whose
head's title states a decision; `card-table`, `concealed-arity`, `limited-play`, `turn-structure`.
"Every other hit is a pointer written in words." "The lint would turn that judgement into a finding
wherever the subject word also appears in prose."

### Verbatim quotations need an exemption `##arg-23`

Round 3, the session. The owner rules on threads by their slugs, 7 in the previous discussion; the
planning skill requires those words verbatim, and a thread is a declared plan item; 7 of thaum's
`self-replacement` hits are quoted rules text. "Condition: text inside quotation marks is not
linted. What that costs: a pointer someone writes inside a quotation goes unchecked."

### Released sections, misread as never changing `##arg-24`

Round 3, the session, later withdrawn (`argument@defined-ids-outside-references@arg-31`). "A slug written in words in a released section, while
its entry still exists, would be a finding nobody may repair."

### The concept index is the constraint the primer already sets `##arg-25`

Round 3, the session. "a concept-named slug acts as an index"; "the titles hold the concept, and
`cargo klarch show` lists every citation. This is a real constraint, but it is the one the primer
already asks for."

### What was tested and held `##arg-26`

Round 3, the session. No plain id in this repository's Rust; thaum's 51 Rust hits covered above; an
entry to be created is reported once it exists; a commit message is judged against its own tree; a
thread written `#slug` is excluded by its `#`. Corrected by the code-claims review: a commit message's lints read its first parent's tree too, per `design@core@a-commit-message-is-a-document`.

### The listing's premise is defeated `##arg-27`

Round 3, the session. "its premise, the noise, is defeated by the subject-slug pattern above".

### The quotation exemption is accepted `##arg-28`

Round 4, the owner. "quoting verbatim: I can agree on that, it seems very reasonnable."

### No exemption for released sections `##arg-29`

Round 4, the owner. "Where did you get that ? The rule is no content change, but structural change
allowed. Also, I'm not too keen on restricting a feature for such an edge case. Also, it is
important that it applies in the future. No exemption for changelog, and if a finding appears,
we'll see how to handle it, one by one under my ruling."

### The concept index is not the intended usage `##arg-30`

Round 4, the owner. "this is not the intended usage of knowledge-architect IMO. The thaum usage
simply predates the creation of knowledge-architect and its evolutions. I do not believe it buys
much in term of use case, and as you said the primer enforces different instructions there."

### The changelog exemption withdrawn `##arg-31`

Round 4, the session. The root `CLAUDE.md` says "a released section's content never changes; its
structure may", and `design@core@bare-skill-name-reported` already writes a name as its reference
in a released section; "Turning a slug written in words into its reference is a structural change,
so a repair is allowed and no exemption is needed."

### Section slugs behave like register ids `##arg-32`

Round 4, the session, measured. Here 34 plain hits on 10 section slugs, most of them pointers in
words; thaum 5, all `skill@knowledge-architect-design@keep-or-change`; one-word section slugs `skill@knowledge-architect-project-audit@outcomes` here and `terms`, `skills`,
`git` in thaum.

### An id defined at several places cannot choose its repair `##arg-33`

Round 4, the session, measured. 18 such ids here and 41 in thaum, mostly agent section slugs such as
the section slug of the brief in three agents; picking the nearest definition makes the checker choose a meaning, which
`tripwire@core@fix-makes-a-choice` guards against. Corrected by the code-claims review: that tripwire guards a `--fix` whose bytes the tree does not determine, and says nothing of a finding's text; the losing alternative stands on the checker choosing no meaning a writer did not write.

### Respelling belongs to the held quick-fixes spec `##arg-34`

Round 4, the session. "Respelling a backticked id into its reference is the subject of
`spec@plans@path-quickfixes`, which is on hold on your word."

### The hash form stays out `##arg-35`

Round 4, the session, decided. The design skill prescribes `#slug` for threads, and the plain `#`
item name is the open question of `issue@core@a-plain-hash-item-name-is-read-by-nothing`.

### What a quotation is `##arg-36`

Round 4, the session, decided. Text between `"` and `"`, or `“` and `”`, within one paragraph, or a
blockquote line beginning with `>`; "A quotation left open ends at its paragraph, so it cannot
silence the rest of a file."

### The premortem `##arg-37`

Round 4, the session. T1 on the premise of #plain-word-lint; AC1 on the quotation exemption; AC2 on
the cost claim, with a threshold of 10% offered as the session's default.

### The code sites `##arg-38`

Round 5, the session, read from the tree: the `ID` regex of `path@core@src/entity.rs`,
`Observation::BareName` in `path@core@src/scan.rs`, its report in
`path@core@src/check/references.rs`, and `design@core@candidate-rule-and-retired-forms`. Corrected by the code-claims review: the regex is `ENTITY_ID`, and the session's words named it by its line.

## New names, in one place

```
crates/core/src/entity.rs            existing: ENTITY_ID, kept for a register's name and directory;
                                     new: ENTRY_ID, the two-segment grammar, behind is_entry_id
crates/core/src/scan.rs              existing: Observation::BareName, SLUG_SITE, SLUG_ANY, BARE_NAME,
                                     NAME_WHOLE; new: Observation::UnquotedReference(String), a class A
                                     token; Observation::PlainWord(String), a class C word;
                                     Observation::OneSegmentSlug(String), a one-segment slug at a
                                     definition site
crates/core/src/check/references.rs  existing: the match arm Observation::BareName(name) if lints in
                                     judge_part, widened to every defined id
crates/agent-skills/src/render.rs    existing: is_id, two segments for a slug placeholder
arg-<n>                              new: the numbering of plan argument items, arg-1, arg-2, …
```

## Decided design

### What is defined

Every defined id, as Names gives it. A register's name and its directory, a Component's or a
location's name, and a kind are not ids. An id of another project or of a removed entry is not
defined, so it is silent, except in a commit message whose first parent still defines it.

### The grammar

An id is `[a-z0-9]+(-[a-z0-9]+)+`: two or more segments, at every site of the mapping table that
judges an id. A one-segment id is a finding at its definition site, "an id has two or more
segments", its repair "rename it with a second segment": the scanner keeps recording a one-segment
slug at a heading, so it is reported rather than read as no slug. The planning skill numbers argument
items `arg-<n>`, and every restatement of the grammar, installed or not, changes with it. Nearest
rival: one-word ids exempt from the lints (`argument@defined-ids-outside-references@arg-20`); second
rival: plan items exempt (`argument@defined-ids-outside-references@arg-21`).

### Class A

A token outside any backtick span, in any prose region the checker reads, fenced blocks included,
that starts with a kind the entity table knows and has the segments a reference of that kind takes:
three for a register kind, an anchor and the rest for `path` and `planned`, and the arities of
`design@core@harness-kinds-cited-without-anchor` for a harness kind. The token is bounded as a word
is: it starts after a character that is no letter, digit, `@`, `/`, `.` or backtick, and it ends
before whitespace or a punctuation mark that ends a sentence or a clause, `.`, `,`, `;`, `:`, `)`,
`!` or `?`, followed by whitespace or the end of the line. A token holding `<` or `>` is a
placeholder and is silent. The finding is "a reference written without backticks", its repair
"write `` `<the token>` ``".

Under D2, a token in a paragraph that also holds a backticked reference to an issue entry is
silent: that is the first exception of `design@agent-skills@plain-text-is-no-repair`, a pointer in
the checker's syntax that no checked form expresses, written beside the issue that records the
missing form. Nearest rival: the same check limited to `%%` comment lines of the shipped text
(`argument@defined-ids-outside-references@arg-7`).

### Class B

A backticked span that is exactly a defined id, a span wrapped at a hyphen included, is a finding:
"`<id>` names <the entry> without its kind", its repair "write `` `<kind>@<anchor>@<id>` ``". A
skill's or an agent's name is a member, as today. Nearest rival: no finding
(`argument@defined-ids-outside-references@arg-10`).

### Class C

A plain word outside any backtick span that is exactly a defined id is a finding, its repair "write
`` `<kind>@<anchor>@<id>` ``; where the word is meant as an ordinary term, the entry's slug names a
subject: rename the entry to state its decision". A word is bounded as an id is: no letter, digit,
hyphen, `/`, `@` or `#` on either side, and no `.` followed by a letter or a digit, so a file name
or a path holding an id is not a word, and an id that ends a sentence is. A released changelog
section is linted: a finding there is ruled by the owner case by case, and its repair, the
reference, is a structural change the section allows. Nearest rival: a listing in `show`
(`argument@defined-ids-outside-references@arg-12`), ruled out by the owner.

### Quotations

A quotation, as Names defines it, is exempt from class C, on the owner's word of round 4, and,
under D3, from classes A and B as well, since a quotation is someone else's words and its writer
may not respell it.

### The finding

- **An id defined at several places** gets one finding that lists every candidate reference and
  chooses none; this replaces today's choice of the skill where a skill and an agent share a name.
- **No `check --fix`** for these findings (#lint-is-report-only).
- Each finding is a lint of the last phase, a member of `Part::Lints`, whose doc comment, the
  model dump's exhaustive match in `path@core@src/model.rs`, and the candidate rule's "read by three
  lints" sentence name the new observations.
- **A generated file writes ids as references.** The milestones index and every other generated
  listing that writes an entry's id in a cell writes `` `<kind>@<anchor>@<id>` ``, so a generated file
  holds no finding and needs no exemption.

### The tree's own findings

Every finding the grammar and the lints give on this repository is repaired in the commit that
lands them, which is the work branch's first commit, per `primer@branch-commits`. The census found,
besides the one-word ids listed in "What the work is":

- class B: `tripwire@agent-skills@ruling-lost-in-change` written bare in an issue entry; three
  section slugs written bare in issue entries, among them the primer's and the root's shared slug
  for where knowledge goes; the section slug `primer@checker-model`, written bare in a doc comment of
  `path@core@src/agents.rs`; and the planning skill's `a<n>` examples, rewritten with the numbering;
- class C: the subject slug `design@agent-skills@need-test`, cited 9 times, and the section slug
  `skill@knowledge-architect-design@keep-or-change`, cited 2 times, each renamed to state its
  decision, or its subject reworded where used as a word; "the path-quickfixes spec" written in
  words; an issue naming itself; section slugs named in words, such as "the primer's
  intent-and-claims rule";
- this spec's own text, which names its threads as `#<slug>` and every other defined id by its
  reference, and quotes the owner and the session inside quotation marks.

The implementing session re-takes the list with the built lints, which supersede the census.

## Mapping tables

The sites of the id grammar, and the grammar each judges after the work:

| site | what it judges | after the work |
| --- | --- | --- |
| `is_entity_id`, entity.rs, file-register entries | an issue's id | two segments, through `is_entry_id` |
| `is_entity_id`, entity.rs, milestones | a milestone's name | two segments, through `is_entry_id` |
| `is_entity_id`, check/references.rs, plan paths | a milestone segment of a `plans` path | two segments, through `is_entry_id` |
| `is_entity_id`, entity.rs, harness entities | a skill's or an agent's name | two segments, through `is_entry_id` |
| `is_entity_id`, manifest.rs, declared registers | a register's name and its directory | one or more, `ENTITY_ID` unchanged |
| `SLUG_SITE`, `SLUG_ANY`, scan.rs | a heading slug's definition | records one or more; a one-segment slug is the new definition finding |
| `BARE_NAME`, `NAME_WHOLE`, scan.rs | a backticked bare span | one or more, since a one-segment span is no defined id and stays silent |
| `is_id`, render.rs | a `{{slug:…}}` placeholder of the shipped text | two segments |
| a spec file's basename | a spec's id | two segments |

## Losing alternatives

- **A one-word exception to class B and class C**, superseded by #two-segment-ids:
  `argument@defined-ids-outside-references@arg-20`.
- **Plan argument items exempt from the grammar**, lost to #two-segment-ids:
  `argument@defined-ids-outside-references@arg-21`.
- **Class A limited to `%%` comment lines of the shipped text**, the retrospective's proposal, lost
  to #reference-shape-outside-span: `argument@defined-ids-outside-references@arg-7`.
- **A listing in `show` instead of a class C finding**, ruled out by the owner, round 4, on the
  census `argument@defined-ids-outside-references@arg-22` and the owner's
  `argument@defined-ids-outside-references@arg-15`.
- **Released changelog sections exempt from class C**, proposed by the session and ruled out by the
  owner, round 4: `argument@defined-ids-outside-references@arg-29`; the session withdrew its premise,
  `argument@defined-ids-outside-references@arg-31`.
- **Subject slugs kept as a concept index**, ruled out by the owner:
  `argument@defined-ids-outside-references@arg-30`.
- **No class B finding, a listing only**, lost to #backticked-bare-id:
  `argument@defined-ids-outside-references@arg-10`.
- **The nearest definition chosen for an ambiguous id**, lost to #ambiguous-id-candidates:
  `argument@defined-ids-outside-references@arg-33`; the checker chooses no meaning a writer did not
  write.
- **Fenced blocks exempt from class A**, the spec's first draft, against
  `design@core@grammars-not-prefixes`, under which a fence is prose; the census found no class A
  token in a fence of either tree but two placeholders.

## Readings

The work reads no external specification. The quotation marks it recognises are the ones Names
lists.

## Premortem

| cause | thread stressed | verdict |
| --- | --- | --- |
| an owner rules a class C finding to be on a word that is neither a pointer nor a subject slug | #plain-word-lint | T1, recorded on the owner's word, round 5: "T1 recorded" |
| a quotation left open silences real pointers | #plain-word-lint | AC1, applied on the owner's word, round 5: "AC1 and AC2 applied" |
| the lints make the check slow | #reference-shape-outside-span, #backticked-bare-id, #plain-word-lint | AC2, applied, same word |

**T1**, written at the harvest in `register@core@tripwire`:

- **Fires when:** at a consuming project's upgrade, or here, the owner rules a finding of the class C
  lint to be on a word that is neither a pointer nor a subject slug: an ordinary use of a slug that
  states its decision.
- **Response:** reopen the class C head, with a listing in `show` among the candidates.
- **Re-entry:** each intake of a retrospective file that reports an upgrade.

## Acceptance criteria

### AC1: a quotation silences its own text and nothing past its paragraph `##quotation-bounded`

Guards #plain-word-lint. Judged at step 1. Observable: a test over a fixture where a defined id
follows an unclosed `"` in one paragraph and stands plain in the next, and another where an id sits
inside a closed quotation on a line that also holds it plain, shows a finding missing on the second
paragraph's id or on the line's plain id. Response: the quotation boundary is repaired, and the
exemption's definition reopens if no boundary keeps both.

### AC2: the lints cost little time `##lints-cost-little`

Guards #reference-shape-outside-span, #backticked-bare-id and #plain-word-lint. Judged at step 1.
Observable: `cargo klarch check` over this repository at the work's first commit, release build,
same machine, median of 5 runs each, takes more than 10% longer than the same commit built with the
three lints' arms removed by an uncommitted patch. The 10% is D4, the owner's to reset. Response:
the scan is reworked before landing, or the owner rules on the cost.

## Implementation sequence

1. **The grammar and the three lints, in the work branch's first commit**, since each makes the
   checker stricter and `cargo klarch commits` judges every tree of the branch with the tip's
   checker, per `primer@branch-commits`:
   - the sites of the mapping table take their grammar; the one-segment definition finding;
   - classes A, B and C, the quotation exemption, the candidate list, and the generated files'
     reference form;
   - planted defects in the `planted` mock, one per class and one one-segment definition, per
     `path@core@CLAUDE.md`; the mock and unit-test fixtures renamed to two segments;
   - every installed restatement of the grammar and the numbering, installed;
   - every finding on this repository repaired, per "The tree's own findings", the renames of
     `skill@knowledge-architect-project-audit@outcomes` and of the subject slugs with their
     references, and the items of `spec@plans@path-quickfixes` renumbered.

   Claims, each with its test: a one-segment id is reported at its definition; each class reports its
   planted defect and nothing in `dirhome`, `minimal` or `core`; a quotation is exempt, as AC1 says;
   a commit message's lint reads its parent's tree; an ambiguous id lists every candidate. AC1 and AC2
   are judged here. Fails alone on: a finding the census did not foresee.
2. **The changelog**, after reading `design@knowledge-architect@changelog-entries` and
   `design@knowledge-architect@versioning-policy` whole: a Migration entry, that a project renames
   its one-word ids, repairs the findings the lints give, and repairs its references to the
   installed sections renamed, such as `skill@knowledge-architect-project-audit@outcomes`; the lints
   as a stricter check, a minor; then `cargo x changelog`.
3. **The review of the work, the report, the harvest and the deletion**, as
   `skill@knowledge-architect-planning@working-a-slice` points 4 to 7 say. The design audit of point
   2 is owed when the work does not start in the session that converged.

## Order rationale

- 1 before 2: the changelog states the finished checks.
- 2 before 3: the harvest records what landed.

## Defaults awaiting the owner

- **D1**, bearing on the whole work: this is a spec, not a milestone; one branch, one pull request
  for its work. Ruled: accepted by the owner, the work's pull request coming later.
- **D2**, bearing on #reference-shape-outside-span: class A is silent on a token in a paragraph that
  also holds a backticked reference to an issue entry, which keeps the first exception of
  `design@agent-skills@plain-text-is-no-repair` and of `primer@where-knowledge-goes`. It narrows
  the owner's approval of class A "in every prose region". Ruled: accepted by the owner.
- **D3**, bearing on #plain-word-lint: the quotation exemption the owner approved for class C also
  holds for classes A and B. It widens that approval. Ruled: accepted by the owner.
- **D4**, bearing on AC2: its threshold, 10%, the session's default. Ruled: accepted by the owner.
- **D5**, bearing on #plain-word-lint: the member the harvest writes into
  `design@knowledge-architect@changelog-section-fixed-at-release` carries the owner's reservation,
  each finding in a released section ruled by the owner case by case. Ruled: accepted by the owner.

## Harvest

| thread | home | when |
| --- | --- | --- |
| #reference-shape-outside-span | `design@core@candidate-rule-and-retired-forms`, rewritten; `tripwire@core@candidate-rule-silence` restated or deleted | step 3 |
| #two-segment-ids | a head of `register@core@design` whose slug is the thread's; the grammar statements of `design@core@an-entry-is-a-heading-at-the-register-level` and `design@core@unusable-harness-name-is-a-finding`; the example of `design@core@plan-document-kinds` | step 3 |
| #backticked-bare-id | `design@core@bare-skill-name-reported`, rewritten to every defined id, its slug renamed if it no longer names the decision | step 3 |
| #plain-word-lint | a head of `register@core@design` whose slug is the thread's; T1 in `register@core@tripwire`; the quotation exception in `design@agent-skills@plain-text-is-no-repair`; the member of `design@knowledge-architect@changelog-section-fixed-at-release` | step 3 |
| #section-slugs-in-scope, #ambiguous-id-candidates, #lint-is-report-only | the bodies of the class heads | step 3 |
| the losing alternatives | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | step 3 |

## Later consequences

- A consuming project meets the lints at its upgrade, and its owner rules on findings a slug's
  rename cannot repair, as T1 watches.
- `issue@core@a-plain-hash-item-name-is-read-by-nothing` decides the `#<id>` form separately.
- A later `check --fix` respelling for these findings belongs to `spec@plans@path-quickfixes`.
