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
- **Assembled from one transcript**, the session's log
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/474d34ec-235f-4827-8a52-f3ad062ff4cf.jsonl`,
  whose discussion begins at the owner's message "For the citations: we'll do the repair, and try to
  think of a more general, long term fix, in the same PR, as a spec that we will merge with it." A
  round here is one owner message and the replies to it, numbered from that message.
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
| a defined id | an id the entity table defines: a register entry's (design, goal, tripwire, issue, a plan document, a plan item) or a section slug of a harness kind (a skill's, an agent's, the primer's, the root `CLAUDE.md`'s) |
| class A | a reference-shaped token outside any backtick span: a known kind, `@`, and the segments a reference of that kind takes |
| class B | a backticked span that is exactly a defined id |
| class C | a plain word, outside any backtick span, that is exactly a defined id |
| a subject slug | a slug that names what a decision is about, such as a domain term, rather than the decision; `primer@design-heads` asks that a slug stay aligned with its decision's full scope |
| a quotation | text between `"` and `"`, or `“` and `”`, within one paragraph, or a blockquote line beginning with `>`; one left open ends at its paragraph |
| the census | the measurement of round 1 and round 3, scripts in the scratch directory of the session, figures below and in the arguments |

## What the work is

**What exists today.**

- The id grammar is `[a-z0-9]+(-[a-z0-9]+)*`: the `ID` regex of `path@core@src/entity.rs`,
  `^[a-z0-9]+(?:-[a-z0-9]+)*$`, and the statement "An id is `[a-z0-9]+(-[a-z0-9]+)*`" in
  `design@core@an-entry-is-a-heading-at-the-register-level`. The installed text restates it in
  `primer@checker-model` ("An id matches"), in `primer@design-heads` ("The slug is an id in the
  grammar"), in `skill@knowledge-architect-issue-tracking@issue-entry`, in
  `skill@knowledge-architect-design@threads-and-states` and in
  `skill@knowledge-architect-planning@spec-contents`, which also numbers arguments `a1`, `a2`.
- The scanner, `path@core@src/scan.rs`, records a backticked one-word span in the id grammar as
  `Observation::BareName`, and `path@core@src/check/references.rs` reports one only where it is a
  skill's or an agent's name, per `design@core@bare-skill-name-reported`. A register's id in that
  form is silent, and so is any reference-shaped token outside backticks, per
  `design@core@candidate-rule-and-retired-forms`, and any plain word.
- the issue `a-bare-mention-of-a-deleted-entry-is-never-reported`, a `todo`, holds the owner's
  preferred fix, "report as a finding any bare mention of a declared slug", and asks for a census
  first. This spec schedules that work, so the commit that adds it closes the entry, per
  `skill@knowledge-architect-planning@spec-or-milestone`; its content is in this spec.
- One-word ids in this tree: the 30 argument items of `spec@plans@path-quickfixes`, `a1` to
  `a30`; the section slug `outcomes` of `skill@knowledge-architect-project-audit`; in the mock
  projects under `path@core@tests/projects/`, `a1` twice and `checked` once; and `Not_An_Id`, a
  planted invalid id, already outside the grammar.

**Outside the work.**

- The `#<id>` form: the design skill prescribes `#slug` for a thread in prose, and the plain `#`
  item name is the open question of `issue@core@a-plain-hash-item-name-is-read-by-nothing`.
- Writing the repair: `check --fix` writes nothing for these findings; respelling is the subject of
  `spec@plans@path-quickfixes`, held on the owner's word (#lint-is-report-only).
- An id of another project, named in prose: it is not defined here, so it is silent, per
  `issue@core@cross-project-references`.

## What is already decided

**Decisions the design rests on, not argued again:**

- `design@core@finding-names-the-repair`: each finding names its repair.
- `design@agent-skills@plain-text-is-no-repair`: a record of a deliberation names a removed entry
  by its bare slug; a removed entry is not defined, so the lints leave it silent.
- `design@core@grammars-not-prefixes`: which bytes of a file are prose, the regions the lints read.
- `design@core@a-commit-message-is-a-document`: the lints apply to commit messages through
  `commits`, each message against its own tree.
- `design@knowledge-architect@changelog-section-fixed-at-release`: a released section's content
  never changes and its structure may, so a slug written there is repaired as its reference.
- `design@core@plan-item-scope`: a plan item is cited from inside its own plan only.

**Decisions the work reverses or rewrites:**

| decision | change | referencing texts | judged or updated by |
| --- | --- | --- | --- |
| `design@core@candidate-rule-and-retired-forms` | rewritten: a reference-shaped token outside backticks is a finding (#reference-shape-outside-span) | the texts `cargo klarch show` lists for it, read at the harvest; `tripwire@core@candidate-rule-silence`, which fired on the census (round 1) | the harvest |
| `design@core@bare-skill-name-reported` | rewritten: a backticked span that is exactly any defined id is reported, a skill's or an agent's name a member (#backticked-bare-id) | the texts `cargo klarch show` lists for it | the harvest |
| `design@core@an-entry-is-a-heading-at-the-register-level` | its statement of the id grammar changes to two or more segments (#two-segment-ids) | the installed restatements listed above, rewritten in step 1 | step 1 and the harvest |

**Standing entries the discussion's search returned, and their outcome:**

| entry | outcome |
| --- | --- |
| the issue `a-bare-mention-of-a-deleted-entry-is-never-reported` | closed by the commit that adds this spec |
| `tripwire@core@candidate-rule-silence` | fired on the census; its decision is rewritten at the harvest, which restates the tripwire for the shapes still silent, a typo inside the kind among them, or deletes it |
| `tripwire@agent-skills@plain-text-pointer-found` | names "a mechanical check for plain-text pointers" among its candidates; not fired; read at the harvest |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing` | outside the work; stays open |
| `issue@core@cross-project-references`, `tripwire@core@item-of-another-plan-named` | an id of another project or another plan is not defined here, so silent; nothing |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | a neighbouring check; class A covers its path and kind spans written without backticks, and the entry stays open for those written with them |
| `tripwire@core@checker-source-literals-are-data-self-location` and its two siblings | the census found no defined id in this repository's Rust comments or literals; step 1's tests judge it again |
| `issue@core@four-heads-bundle-decisions-with-many-citing-sites` | its re-entry is this change to `design@core@candidate-rule-and-retired-forms`; read at the harvest |
| `issue@core@the-retired-slug-finding-names-a-design-repair-for-every-register` | the same repair-naming question for every register; #ambiguous-id-candidates answers it for these lints |
| `issue@core@tooling-for-project-skills`, `issue@core@references-are-read-in-markdown-and-rust-only`, `issue@core@the-rust-parser-choice-has-no-head`, `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry` | bear on the regions the lints read; nothing changes in them |
| `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument` | the owner ruled that released sections are linted, findings ruled one by one (round 4); nothing |

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
"The grammar". Harvest: a head of `register@core@design`, slug `two-segment-ids`, and the grammar
statement of `design@core@an-entry-is-a-heading-at-the-register-level`.

### A backticked span that is exactly a defined id is a finding `##backticked-bare-id`

Proposed by the session in round 1 (`argument@defined-ids-outside-references@arg-8` to `argument@defined-ids-outside-references@arg-11`), with a one-word exception the owner's
#two-segment-ids made unneeded (a20). Closed `argued` in round 4, its rivals out: the one-word
exception by `argument@defined-ids-outside-references@arg-20`, no finding by `argument@defined-ids-outside-references@arg-10`. In the checkpoint the owner confirmed in round 5, "threads
agreed". Shape: Decided design, "Class B". Harvest: `design@core@bare-skill-name-reported`,
rewritten.

### A plain word that is a defined id is a finding, quotations exempt `##plain-word-lint`

Proposed by the owner in round 1 (`argument@defined-ids-outside-references@arg-1`) and round 2 (`argument@defined-ids-outside-references@arg-15`): "I would still lint. And if the linted term
is meant as an ordinary word, it is probably that the slug is wrongly chosen and not sufficiently
specific." Argued against by the session at the owner's request, round 3 (`argument@defined-ids-outside-references@arg-22` to `argument@defined-ids-outside-references@arg-26`). Approved by
the owner in round 4: "plain-word-lint approved", with the quotation exemption, "quoting verbatim: I
can agree on that, it seems very reasonnable", no changelog exemption, and the concept-index use
ruled out (`argument@defined-ids-outside-references@arg-29`, `argument@defined-ids-outside-references@arg-30`). Shape: Decided design, "Class C". Harvest: a head of `register@core@design`,
slug `plain-word-lint`, pair `#plain-word-lint → plain-word-mention-is-reported`, since the
thread's name names the change.

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

Proposed by the session in round 1 (`argument@defined-ids-outside-references@arg-12`, `argument@defined-ids-outside-references@arg-13`). Ruled out by the owner in round 2: "I would still
lint." Harvest: the losing alternatives.

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
`ruling-lost-in-change` in an issue here, and thaum's README lists of heads by bare slug.

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
thread written `#slug` is excluded by its `#`.

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
words; thaum 5, all `keep-or-change`; one-word section slugs `outcomes` here and `terms`, `skills`,
`git` in thaum.

### An id defined at several places cannot choose its repair `##arg-33`

Round 4, the session, measured. 18 such ids here and 41 in thaum, mostly agent section slugs such as
`your-brief`; picking the nearest definition makes the checker choose a meaning, which
`tripwire@core@fix-makes-a-choice` guards against.

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
`path@core@src/check/references.rs`, and `design@core@candidate-rule-and-retired-forms`.

## New names, in one place

```
crates/core/src/entity.rs          existing: the ID regex, which becomes ^[a-z0-9]+(?:-[a-z0-9]+)+$
crates/core/src/scan.rs            existing: Observation::BareName, recorded for every backticked
                                   span in the grammar; new: an observation for a class A token and
                                   one for a class C word, names the implementing session chooses
crates/core/src/check/references.rs  existing: the BareName lint, widened to every defined id
arg-1, arg-2, …                    new: the numbering of plan argument items
```

## Decided design

### What is defined

Every id the entity table defines: the entries of every register, a plan document's id, a plan
item's id inside its own plan, and the section slugs of the harness kinds. An id of another project
or of a removed entry is not defined, so it is silent.

### The grammar

An id is `[a-z0-9]+(-[a-z0-9]+)+`: two or more segments. A definition outside it is the finding
the checker reports today for an id outside the grammar, which now names the two-segment rule. The
planning skill numbers argument items `arg-1`, `arg-2`, …, and every installed restatement of the
grammar changes with it. Nearest rival: one-word ids exempt from the lints (`argument@defined-ids-outside-references@arg-20`); second rival:
plan items exempt (`argument@defined-ids-outside-references@arg-21`).

### Class A

A token outside any backtick span that starts with a known kind and has the segments a reference of
that kind takes is a finding: "a reference written without backticks", repair "write
`` `<the token>` ``". It applies in every prose region the checker reads: markdown outside fenced
blocks, the comments and unbound string literals of Rust source, per
`design@core@grammars-not-prefixes`, and commit messages through `commits`. A head that is no kind,
as in an email address or a URL, is silent. Nearest rival: the same check limited to `%%` comment
lines of the shipped text (`argument@defined-ids-outside-references@arg-7`).

### Class B

A backticked span that is exactly a defined id is a finding: "`<id>` names <the entry> without its
kind", repair "write `` `<kind>@<anchor>@<id>` ``". A skill's or an agent's name is a member, as it
is today. Nearest rival: no finding (`argument@defined-ids-outside-references@arg-10`).

### Class C

A plain word outside any backtick span that is exactly a defined id is a finding, its repair
"write `` `<kind>@<anchor>@<id>` ``; where the word is meant as an ordinary term, the slug names a
subject: rename the entry to state its decision". A word is bounded as an id is: no letter, digit,
hyphen, `/`, `@`, `.` or `#` on either side. **A quotation is exempt**, as Names defines it. No
other region is exempt: a released changelog section is linted, and a finding there is ruled by the
owner case by case, its repair a structural change. Nearest rival: a listing in `show` (`argument@defined-ids-outside-references@arg-12`),
ruled out by the owner.

### The finding

- **An id defined at several places** gets one finding that lists every candidate reference and
  chooses none.
- **No `check --fix`** for these findings (#lint-is-report-only).
- Each finding is a lint of the last phase, consuming the entity table, per
  `design@core@phases-gate-the-report`.

### The tree's own findings

Every finding the lints give on this repository is repaired in the commit that lands them, since
`cargo klarch commits` judges every tree of the branch with the tip's checker, per
`primer@branch-commits`. The census found, besides the 30 argument items, `outcomes` and the mock
fixtures:

- class B: the bare `ruling-lost-in-change` in an issue entry, and the planning skill's `a1`,
  `a2`, rewritten with the numbering;
- class C: the subject slug `design@agent-skills@need-test`, cited 9 times, and the section slug
  `skill@knowledge-architect-design@keep-or-change`, cited 2 times, each renamed to state its
  decision or its subject reworded where used as a word; "the path-quickfixes spec" written in
  words; an issue naming itself; section slugs named in words, such as "the primer's
  intent-and-claims rule";
- the owner's words quoted in `spec@plans@path-quickfixes`, which the quotation exemption silences.

The implementing session re-takes the list with the built lints, which supersede the census.

## Mapping tables

The work needs none: every existing id keeps its meaning, and a one-word id's new name is chosen per
entry by the implementing session, reported in the commit.

## Losing alternatives

- **A one-word exception to class B and class C**, superseded by #two-segment-ids: `argument@defined-ids-outside-references@arg-20`.
- **Plan argument items exempt from the grammar**, lost to #two-segment-ids: `argument@defined-ids-outside-references@arg-21`.
- **Class A limited to `%%` comment lines of the shipped text**, the retrospective's proposal, lost
  to #reference-shape-outside-span: `argument@defined-ids-outside-references@arg-7`.
- **A listing in `show` instead of a class C finding**, ruled out by the owner: `argument@defined-ids-outside-references@arg-15`, round 2.
- **Released changelog sections exempt from class C**, withdrawn by the session on the owner's
  reading of the record: `argument@defined-ids-outside-references@arg-29`, `argument@defined-ids-outside-references@arg-31`.
- **Subject slugs kept as a concept index**, ruled out by the owner: `argument@defined-ids-outside-references@arg-30`.
- **No class B finding, a listing only**, lost to #backticked-bare-id: `argument@defined-ids-outside-references@arg-10`.
- **The nearest definition chosen for an ambiguous id**, lost to #ambiguous-id-candidates: `argument@defined-ids-outside-references@arg-33`.

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

Guards #plain-word-lint. Judged at step 2. Observable: a test over a fixture where a defined id
follows an unclosed `"` in one paragraph and stands plain in the next, and another where an id sits
inside a closed quotation on a line that also holds it plain, shows a finding missing on the
second paragraph's id or on the line's plain id. Response: the quotation boundary is repaired, and
the exemption's definition reopens if no boundary keeps both.

### AC2: the lints cost little time `##lints-cost-little`

Guards #reference-shape-outside-span, #backticked-bare-id and #plain-word-lint. Judged at step 2.
Observable: `cargo klarch check` on this repository, release build, same machine, median of 5 runs
each, takes more than 10% longer with the lints than without them. The 10% is the session's
default, the owner's to reset. Response: the scan is reworked before landing, or the owner rules on
the cost.

## Implementation sequence

1. **The grammar.** The `ID` regex takes two or more segments; the planning skill numbers items
   `arg-<n>`; every installed restatement of the grammar is rewritten and installed; every one-word
   id of this repository is renamed with its references: the items of `spec@plans@path-quickfixes`,
   the section `outcomes` of the project-audit skill, and the mock fixtures with their tests. Fails
   alone on: the existing definition finding, and the tests that pin it.
2. **The lints**, classes A, B and C, with the quotation exemption and the candidate list, and in
   the same commit every finding they give on this repository's trees repaired, per "The tree's own
   findings". AC1 and AC2 are judged here. Fails alone on: a finding the lints give that the census
   did not foresee.
3. **The changelog**: a Migration entry, a project renames its one-word ids and repairs the
   findings the lints give; the classes as a stricter check, a minor, per
   `design@knowledge-architect@versioning-policy`, read whole with
   `design@knowledge-architect@changelog-entries`; then `cargo x changelog`.
4. **The review of the work, the report, the harvest and the deletion**, as
   `skill@knowledge-architect-planning@working-a-slice` points 4 to 7 say.

## Order rationale

- 1 before 2: the lints' two-segment premise holds only once no one-word id remains.
- 2 before 3: the changelog states the finished checks.
- 3 before 4: the harvest records what landed.

## Defaults awaiting the owner

- **D1**: this is a spec, not a milestone: one branch, one pull request for its work.
- The threshold of AC2, 10%, is the session's default, the owner's to reset.

## Harvest

| thread | home | when |
| --- | --- | --- |
| #reference-shape-outside-span | `design@core@candidate-rule-and-retired-forms`, rewritten; `tripwire@core@candidate-rule-silence` restated or deleted | step 4 |
| #two-segment-ids | a head of `register@core@design`, slug `two-segment-ids`; the grammar statement of `design@core@an-entry-is-a-heading-at-the-register-level` | step 4 |
| #backticked-bare-id | `design@core@bare-skill-name-reported`, rewritten to every defined id, its slug renamed if it no longer names the decision | step 4 |
| #plain-word-lint | a head of `register@core@design`, slug `plain-word-mention-is-reported`, pair `#plain-word-lint → plain-word-mention-is-reported`; T1 in `register@core@tripwire` | step 4 |
| #section-slugs-in-scope, #ambiguous-id-candidates, #lint-is-report-only | the bodies of the class heads | step 4 |
| the losing alternatives | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | step 4 |

## Later consequences

- A consuming project meets the lints at its upgrade, and its owner rules on findings a slug's
  rename cannot repair, as T1 watches.
- `issue@core@a-plain-hash-item-name-is-read-by-nothing` decides the `#<id>` form separately.
- A later `check --fix` respelling for these findings belongs to `spec@plans@path-quickfixes`.
