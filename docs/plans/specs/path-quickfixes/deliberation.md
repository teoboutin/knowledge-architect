# The threads and arguments of the spec on `check --fix` respellings

## Threads

### A fix may edit a hand-written document only as a respelling, and the head states the principle `##fix-admission`

Proposed by the agent in the reply to round 1, against the owner's claim of round 1 that the three
methods are "safe under the recorded condition for admitting quickfixes". Approved. Arguments:
`argument@path-quickfixes@arg-2`, `argument@path-quickfixes@arg-3`, `argument@path-quickfixes@arg-5`,
`argument@path-quickfixes@arg-17`, `argument@path-quickfixes@arg-18`, `argument@path-quickfixes@arg-19`,
`argument@path-quickfixes@arg-20`, `argument@path-quickfixes@arg-21`. Shape: Decided design, "What a fix
may write". Harvest: `design@check@safe-fix-definition` rewritten in place, and
`tripwire@check@fix-makes-a-choice` deleted, per D4. The owner's words, round 2: "fix-admission: agreed."
After the agent's correction of the census, reply to round 2, the owner's words, round 3: "We'll go
with the current design."

### Respellings of a target the span already names, and the shapes left to the writer `##respelling-fixes`

Proposed by the agent in the reply to round 1. It absorbs the owner's proposal (b) of round 1, the
normalization of `path@<component>@../<path>`. Approved. Arguments: `argument@path-quickfixes@arg-4`,
`argument@path-quickfixes@arg-7`, `argument@path-quickfixes@arg-8`, `argument@path-quickfixes@arg-9`,
`argument@path-quickfixes@arg-10`, `argument@path-quickfixes@arg-22`, `argument@path-quickfixes@arg-23`.
Shape: Decided design, "The respellings". Harvest: `design@check@fix-scope` rewritten. The owner's
words, round 2: "respelling-fixes: this is less than I was hoping for. But I have to agree that my
bare backticked path fix proposal probably cannot be made safely..."; round 3: "We'll go with the
current design."; round 4, on the checkpoint table: "Keep all tripwires and AC, proceed." Two
changes after the reviews: the trailing-slash row is dropped, per D3, a material finding; and a
name both a skill and an agent hold is not respelled, `argument@path-quickfixes@arg-28`, which
narrows the row to the case the principle admits.

### An unanchored path's repair asks which candidate was meant, and `--fix` writes nothing for it `##raw-path-inference`

Proposed by the owner in round 1 as proposal (a), "look at the path, under each component
(including root), and at paths relative to the containing file", applied when a single candidate
exists. The agent's reply to round 1 put three options: A, the owner's; B, the agent's default,
naming the one candidate in the repair; C, a separate flag. The owner proposed the question form
in round 2, which supersedes B. Approved. Arguments: `argument@path-quickfixes@arg-1`,
`argument@path-quickfixes@arg-6`, `argument@path-quickfixes@arg-11`, `argument@path-quickfixes@arg-12`,
`argument@path-quickfixes@arg-13`, `argument@path-quickfixes@arg-14`, `argument@path-quickfixes@arg-24`,
`argument@path-quickfixes@arg-25`. Shape: Decided design, "The question form". Harvest:
`design@check@unanchored-path-first-segment-here` gains it. The owner's words, round 2: "maybe the
canditates could be shown in the proposed repair: "Did you mean to write one of <path1>, <path2>...
?" (not as an affirmation "write <path1>", because as you said it could mean something else."; round
3: "We'll go with the current design."

### A `path` citation of a skill or an agent is a finding, and `--fix` respells it `##skill-cited-by-path`

Proposed by the owner in round 1 as proposal (c): a path to a subagent or a skill is written in
the reference form of a skill or an agent; shaped by the agent in the reply to round 1 as a new
check, since such a citation is not a finding today. Approved. Arguments:
`argument@path-quickfixes@arg-15`. Shape: Decided design, "The skill and agent citation". Harvest:
`design@model@harness-kinds-cited-without-anchor` and `design@check@every-path-names-its-anchor`. The
owner's words, round 2: "skill-cited-by-path: approved."

### The respellings run after the gate, are printed one per line, and touch no commit message `##fix-run-order`

Proposed by the agent in the reply to round 1. Approved, as the checkpoint table of the reply to
round 3 showed it: installed files, gate, refusal of a partial commit's mismatch, respellings,
model rebuilt, generated files, check. D1 replaced that order, approved by the owner in round 5. Arguments:
`argument@path-quickfixes@arg-16`, `argument@path-quickfixes@arg-29`. Shape: Decided design, "The order of a
run". Harvest: `design@check@fix-before-the-checks` rewritten, and the sentence of
`design@check@fix-refusal-mixed-state` named in "What is already decided". The owner's words, round
3: "We'll go with the current design."; round 4, on the checkpoint table: "Keep all tripwires and
AC, proceed."

### A head that states the instance built rather than the principle is recorded as a design issue `##head-states-the-principle`

Proposed by the owner in round 2 as a remark; the agent proposed the issue entry in the reply to
round 2. Approved. Arguments: `argument@path-quickfixes@arg-17`, `argument@path-quickfixes@arg-21`.
Harvest: none left; the question it recorded is decided by
`design@agent-skills@title-states-the-rule`. The owner's words, round 3: "head-states-the-principle: agreed on
the issue."

### An inference applied when its single candidate lies under the file's own anchor `##same-anchor-inference`

Measured by the agent in the reply to round 2, which did not propose it. Parked on the owner's word,
round 3: "The narrower variant you identified could be interesting to rediscuss, when I restart
working on this and the spec goes through a design audit." Arguments: `argument@path-quickfixes@arg-22`,
`argument@path-quickfixes@arg-26`.

- **Tripwire**: a census over a consumer project, other than thaum, in which the variant keeps at
  least half of the correct inferences in files (it kept 23 of 53 in the census) and makes no
  misbinding. The threshold is a default, the owner's to reset.
- **Re-entry point**: the design audit of this spec, when the work restarts.

Whether this tripwire is recorded in a tripwires home if the thread is still parked at the harvest
is D2. Adopting the variant applies an inference, so it reopens
`thread@path-quickfixes@fix-admission`, as the status section says.

## Arguments

### The request, its frequency and its safety claim `##arg-1`

Round 1, the owner. Bears on `thread@path-quickfixes@raw-path-inference`, `thread@path-quickfixes@respelling-fixes`, `thread@path-quickfixes@skill-cited-by-path`,
`thread@path-quickfixes@fix-admission`. "It is by far the most common finding, because agents always perfer the raw
backticked form by habit." / "the idea is to search for candidates, and apply only if a single one
exists." / "I think those methods are safe under the recorded condition for admitting quickfixes."

### The recorded condition excludes every method proposed, and its tripwire fires `##arg-2`

The reply to round 1. Bears on `thread@path-quickfixes@fix-admission`. `design@check@safe-fix-definition` admits only
"files of the installer's namespace or of the generated list", on the argument "A fix that makes a
choice, or touches git or a hand-written file, would rewrite what a writer meant."
`tripwire@check@fix-makes-a-choice` fires on "a fix … that writes or removes a file outside the
installer's namespace and the generated list", with the response "reopen
`design@check@safe-fix-definition` before the fix is added". "All three of your methods edit
hand-written documents, so the second clause excludes every one of them."

### A respelling keeps the writer's meaning; an inference chooses one `##arg-3`

The reply to round 1. Bears on `thread@path-quickfixes@fix-admission`. "the writer's meaning is already in the span, so the
fix changes only its spelling. #no-silent-misbinding holds by construction." The test, three
conditions: the span already names exactly one entity, under the checker's own resolution or under
a recorded head that declares what the span means; the rewrite names that same entity in the form
the check requires; no byte outside the span changes.

### A reference fix needs the entity table, and shipped text must stay portable `##arg-4`

The reply to round 1. Bears on `thread@path-quickfixes@fix-run-order`, `thread@path-quickfixes@respelling-fixes`. "a reference fix needs the entity
table, so it must run after the phase gate, never over an incomplete model", from
`tripwire@check@phases-gate-the-report-two`; and "a fix that writes a project-specific anchor into
shipped text produces a reference that passes here and dangles in every installing project", from
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`.

### The census of the unique-candidate rule: 7 apparent misbindings in 104 resolved cases `##arg-5`

The reply to round 1. Bears on `thread@path-quickfixes@fix-admission`, `thread@path-quickfixes@raw-path-inference`. Of 179 raw-path findings, "54
have 0 candidates, 110 have exactly 1, and 15 have 2 or more". Of the 110: 81 repaired exactly as
the rule would rewrite them, 6 with no repair found, 23 repaired otherwise, of which "Seven … are
the case that matters. The writer meant a path in the layout of every project, and the rule's
single candidate is this tree's copy … Nothing reports it afterwards." Why the other 16 do not
count against the rule: "The other 16 disagreements are prose rewrites, backticks removed …, and
five references to a file that the same branch deleted. In those 16 cases the rule's rewrite would
have done no silent harm: the deleted target dangles and is reported." The figure was corrected in
the reply to round 2, `argument@path-quickfixes@arg-18`.

### Two or more candidates support no precedence rule `##arg-6`

The reply to round 1. Bears on `thread@path-quickfixes@raw-path-inference`. Of the 15 findings with two or more
candidates, the session chose the candidate of the file's own anchor 2 times and another
candidate 2 times, wrote no path reference 7 times, and chose the root in 4 commit messages. "This
does not support a precedence rule."

### The finding kinds a respelling would serve are rare `##arg-7`

The reply to round 1. Bears on `thread@path-quickfixes@respelling-fixes`. In the census: 9 findings of a target inside a
deeper anchor, 12 bare skill or agent names, 1 plan document cited by path, 2 `planned` targets
that now exist, 16 retired `#<id>` forms, and 0 of the upward, explicit-current, leading-slash,
anchor-in-kind-position and trailing-slash kinds. The owner's (b) "is correct: phase 2 refuses
symlinks, so lexical normalization equals the filesystem's. But it has never occurred in real
work." It is kept "only because it costs one normalization function".

### Five shapes are not respelled `##arg-8`

The reply to round 1. Bears on `thread@path-quickfixes@respelling-fixes`. A leading `/`: "its intended base is ambiguous".
A retired `<anchor>@<path>` whose target does not exist: "the span may have been a slug reference".
A `planned` reference whose target exists: the comment of `planned` in
`path@check@src/check/references.rs` keeps the conversion with the session that created the file,
since "whether the plan still holds is that session's report to make", and "An automatic rewrite
removes that prompt." A retired `#<id>`: "the census shows those spans were not meant as
references". A line suffix or a fragment: "dropping `:12` loses bytes the writer wrote."

### The bare-name respelling inherits the open prefix question `##arg-9`

The reply to round 1. Bears on `thread@path-quickfixes@respelling-fixes`. "It inherits the risk of
`issue@check@tooling-for-project-skills`: a skill named by an ordinary word would turn every
backticked use of that word into a rewrite." This became T1.

### Nine bare-skill findings of the census came from the lint under construction `##arg-10`

The reply to round 2. Bears on `thread@path-quickfixes@respelling-fixes`. The reply to round 1 had held the bare-name
respelling until 9 unexplained findings were explained. They came from the session that built the
lint, on its branch, on spans that are design slugs. "The shipped lint matches the skill kind
exactly (`entities.defines(&Kind::new(k), "", name)` …), so it cannot report a design slug." The
agent's inference, not traced to a commit: the lint matched slugs while it was being built. "The
real data on the shipped lint is 3 bare agent names, all 3 repaired exactly as the fix would
rewrite them."

### Naming the one candidate keeps the judgement with the agent `##arg-11`

The reply to round 1. Bears on `thread@path-quickfixes@raw-path-inference`, option B. "The agent then makes a one-token
edit with the judgement still in front of it. Most of the 81 agreements become copy-paste edits."

### A separate flag teaches agents to pass it `##arg-12`

The reply to round 1. Bears on `thread@path-quickfixes@raw-path-inference`, option C. "It removes the misbinding risk only
for sessions that do not pass the flag, and agents would learn to pass it."

### The repair text changes a command's output `##arg-13`

The reply to round 1 and the reply to round 2. Bears on `thread@path-quickfixes@raw-path-inference`. "the finding text
changes, which touches `finding-texts-are-not-audited-for-a-needless-cause` and
`issue@knowledge-architect@command-output-is-not-declared-a-contract`." / "I would class it as a
patch with a changelog entry, as the `commits` summary change was classed." The first entry the
reply names has closed; the rule it read the finding texts against is
`design@check@finding-names-the-repair`.

### A question asserts no meaning `##arg-14`

Round 2, the owner, and the reply to round 2. Bears on `thread@path-quickfixes@raw-path-inference`. The owner: "maybe the
canditates could be shown in the proposed repair: "Did you mean to write one of <path1>, <path2>...
?" (not as an affirmation "write <path1>", because as you said it could mean something else." The
agent: "A question does not assert a meaning, and listing every candidate also serves the cases
with 2 or more candidates" / the line after it keeps the two non-tree forms, "so a writer who meant
neither candidate is not pushed toward one".

### A skill cited by path escapes `show` `##arg-15`

The reply to round 1. Bears on `thread@path-quickfixes@skill-cited-by-path`. "Today `path@agent-config@skills/<name>/SKILL.md`
is **not a finding**", the skill's name written here as a placeholder, since the span is the
finding step 3 adds. The argument of `design@check@every-path-names-its-anchor` for a plan document
holds for a skill: "one name per document is what keeps `show` complete". `show` on a skill lists
no `path` citation of it: of the 3 citations then in the tree, 2 cited `skill@klarch-release` and
1 `skill@klarch-retrospective-intake`. A path to another file inside a skill's directory, and the
skills directory itself, have no harness-kind form and stay as they are.

### Where the respellings run, and what they print `##arg-16`

The reply to round 1. Bears on `thread@path-quickfixes@fix-run-order`. After the gate, since they read the entity table;
before the generated files, since "a respelling inside an issue title changes `index.md`"; "a
respelling rewrites the working-tree copy in place and loses nothing"; "The staged copy keeps the
old spelling, so `check --staged` fails until the file is staged again"; "each respelling is
printed as `file:line  old → new`"; commit messages are never touched; "rustfmt does not reflow
comments by default, so a longer comment line fails no gate."

### The current heads state the built instance as the rule `##arg-17`

Round 2, the owner, and the reply to round 2. Bears on `thread@path-quickfixes@fix-admission`, `thread@path-quickfixes@head-states-the-principle`.
The owner: "I think the current design heads are overstated on the admissible shape, I suspect they
were written as a description of what was done and not as a generic principle. I think this happens
quite often in how design heads are written. It's not too bad of an issue because reversing design
heads is not forbidden by the workflow, but it's often bothering me." The agent: "**its principle**
is in its argument: a fix must not "rewrite what a writer meant"" / "The clause states the shape
that was built as if it were the rule."

### Were the different repairs truly wrong, and only here? `##arg-18`

Round 2, the owner; the reply to round 2. Bears on `thread@path-quickfixes@fix-admission`, `thread@path-quickfixes@raw-path-inference`. The owner:
"would the inference have been truly wrong ? Or were the "different repairs" simply a consciously
chosen rewording, rather than just a repair ? There might have been checker avoidance too." The
agent's corrected figure: of the 7, "2 … were in commit messages", "1 was wrong as a misbinding",
and "**4 remain where the inference would have been truly wrong, all in this repository, and all
in text that describes another project's layout**": the skills and agents directories of the agent
configuration named in `path@model@src/entity.rs` and in the core design home, where the session
wrote the placeholder `.claude/skills/<dir>/SKILL.md`, "a checked form for a pattern … not checker
avoidance", and the plans directory in the agent-skills design home, "a path a project chooses".
"thaum: 0 silent misbindings in its 42 one-candidate findings in files." Its backtick removals
"look like checker avoidance, but they predate `design@agent-skills@plain-text-is-no-repair`".

### This repository's own needs weigh less `##arg-19`

Round 2, the owner; the reply to round 2. Bears on `thread@path-quickfixes@fix-admission`, `criterion@path-quickfixes@no-self-catering`. The owner:
"Also, if they happen only here, maybe the argument is less strong. This project imposes more
constraints on the checker it publishes and uses itself than external consumers, because of the
need to describe the internal checker logic and the published workflow content in design docs. I
want to avoid catering to this project's own needs when designing the checker." The agent: "So
your suspicion holds: the observed failures come from this repository's need to describe other
projects' trees."

### The tree cannot tell two meanings of one span apart `##arg-20`

The reply to round 2. Bears on `thread@path-quickfixes@fix-admission`, `thread@path-quickfixes@raw-path-inference`. The identical raw span naming
the skills directory of the agent configuration, with the identical single candidate,
`path@agent-config@skills/`, meant this tree's directory in 3 plan documents, and every project's
layout in `entity.rs` and in the core design home. "The tree is the same in both cases, so no rule
that reads only the tree can tell those two meanings apart." In a consumer, thaum's CLAUDE.md held
a raw span naming the library root of a crate, meaning knowledge-architect's file; "It was refused
only because it had 2 candidates." The raw spans are described here in words, since written in
backticks each would be the finding it illustrates.

### The default kept the ruling on a narrower premise `##arg-21`

The reply to round 2. Bears on `thread@path-quickfixes@fix-admission`. "a wrong inference is silent for good, since nothing
reports a reference that resolves; the consumer sample is one project; your question-form repair
… recovers most of the gain with no silent case." The alternative offered: "apply the unique
candidate, and print each rewrite marked `inferred` so the session can revert it." The owner, round
3: "We'll go with the current design."

### The same-anchor variant would exclude the misbindings and most correct inferences `##arg-22`

The reply to round 2. Bears on `thread@path-quickfixes@same-anchor-inference`, `thread@path-quickfixes@raw-path-inference`. "It would have
excluded all 4 misbindings. It would also have dropped 30 of the 53 correct inferences in files, so
I do not propose it."

### The respellings are cheap and serve few findings `##arg-23`

The reply to round 2. Bears on `thread@path-quickfixes@respelling-fixes`, `criterion@path-quickfixes@covers-the-common-finding`. "Measured value of the
respelling fixes in the census: about 13 findings, against 179 raw paths. They are cheap to build
because the code already computes each target form. The value of the work is mainly in the
question-form repair and in #skill-cited-by-path."

### Most one-candidate repairs agree with the rule `##arg-24`

The reply to round 1. Bears on `thread@path-quickfixes@raw-path-inference`, `criterion@path-quickfixes@covers-the-common-finding`. 81 of the 110
one-candidate findings were repaired exactly as the rule would rewrite them; the question form
puts that rewrite in front of the session.

### Commit messages hold a large share of raw paths `##arg-25`

The reply to round 1. Bears on `thread@path-quickfixes@raw-path-inference`, `thread@path-quickfixes@fix-run-order`. "**179** (70 of them in commit
messages)". No `--fix` reaches a commit message, whatever the rule.

### The work is much for its share `##arg-26`

Round 3, the owner. Bears on `criterion@path-quickfixes@covers-the-common-finding`, `thread@path-quickfixes@same-anchor-inference`. "I'm not fully
satisfied, because this design is quite a lot of work (especially the rewriting part), and yet it
only addresses a small portion of the findings that happen. The narrower variant you identified
could be interesting to rediscuss, when I restart working on this and the spec goes through a
design audit."


### A respelling of the kind claim is the fix the guarding tripwire was written against `##arg-27`

The decision-record and design-conformance reviews of this spec. Bears on `thread@path-quickfixes@respelling-fixes`. The
trailing-slash row rewrites a span's kind claim to match the tree. `design@model@trailing-slash-claims-directory`
places the claim "in the span itself … rather than inferred from what happens to exist". The spec
that created `tripwire@check@fix-makes-a-choice`, the pre-release spec, deleted at its harvest and
read with `git show b75d4c4:docs/plans/specs/pre-release-fixes.md`, put cause P3 to the owner, and
the tripwire was recorded from it on the owner's word: "a later fix is added that makes a choice, such as adding a trailing slash, because the
option invites growth, and it rewrites what a writer meant". A span whose claim disagrees with the
target also fails the respelling test, since under the checker's own resolution it names nothing.
The owner approved the row without this premise; it is D3.

### A name both kinds hold names two entities `##arg-28`

The design-conformance review of this spec. Bears on `thread@path-quickfixes@respelling-fixes`. The lint's repair for a
name both a skill and an agent hold picks the skill, and the code's own comment gives the reason:
"the order below is the only reason". A respelling there would choose between two entities the
span names equally, which the respelling test excludes. The round-1 table of the reply to round 1
named the row "bare skill or agent name → `skill@<name>` or `agent@<name>`", with no precedence; the
precedence was added at assembly and is removed.

### Under D1, a refusal can follow a written respelling, and a stop stays a report `##arg-29`

The design-conformance reviews of this spec. Bears on `thread@path-quickfixes@fix-run-order`. With
the respellings before the second gate and the refusal, a refusal may come after hand-written files
were written. Exit 2 promises an untouched tree, so the refusal exits 1 once any file was written,
and the sentence of `design@check@phases-gate-the-report` that names "only the installed files" as
written before the gate is rewritten. A stop at a gate is not a failure to run: it reports
findings, and exits 1 whatever was written, per `design@cli@exit-code-ladder`, "1 | the command
ran, and reports a negative answer about its subject", and per the same sentence of
`design@check@phases-gate-the-report`, "it prints the stopped report and exits 1". The first
revision of this spec made a stop exit 2 when nothing was written, which contradicted both heads;
D7 restores exit 1.

### A bare-name respelling rests on the head's presumption, not on a guarantee `##arg-30`

The design-conformance re-review of this spec. Bears on `thread@path-quickfixes@respelling-fixes`.
`design@check@backticked-bare-id` states "A bare name is a pointer written with no kind", and
also keeps a census that re-takes its false positives: "one on a span that is not meant as a
pointer reopens the exact match". So the row respells under the head's presumption that a matched
name is a pointer. A false positive is the case T1 watches, and the owner kept T1 on the premortem
cause that named it, round 4. The harvest rewrites the head to say that `--fix` applies its repair,
and that a false positive reopens both the exact match and the respelling.
