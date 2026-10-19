# The retrospective intake ships as an installed skill, for the analysis of a project's own retrospective file

## Status and audience

This spec is the plan of the work that ships the intake of a retrospective's findings to every
project that installs the workflow. A project then analyses its own retrospective file finding by
finding, with the evidence in front of the owner, before anything changes in its text. This
repository keeps its own intake skill, reduced to what only the workflow's upstream does: the
analysis of the workflow files other projects send.

- It is written for a session that did not witness the design discussion that produced it.
- It leaves the repository in the commit that completes its harvest.
- Where it and a design home disagree, on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits.
- **Assembled from one transcript**, the session's log
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/474d34ec-235f-4827-8a52-f3ad062ff4cf.jsonl`,
  whose discussion begins at the owner's message "I would like to make the retrospective intake
  skill part of the publihsed skill set". A round here is one owner message and the reply to it,
  numbered from that message: round 2 holds the reply the session titled "Design discussion,
  round 1", and round 3 the one it titled "Round 2".
- The branch `ship-retrospective-intake` already holds two commits of the discussion: the goals of
  agent-skills reworded and split on the owner's word, and the issue entry
  `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`. The work
  of this spec lands on the same branch.

## How the work is done

`skill@knowledge-architect-planning`, its section `working-a-slice`, as the work of a spec.

## Names

| name | what it is |
| --- | --- |
| the local intake | `skill@klarch-retrospective-intake`, at .claude/skills/klarch-retrospective-intake/SKILL.md: this repository's own skill |
| the installed intake | the skill this work ships, named knowledge-architect-retrospective-intake once installed, whose source is `planned@agent-skills@content/skills/retrospective-intake/SKILL.md` |
| the retrospective | `skill@knowledge-architect-retrospective`, source `path@agent-skills@content/skills/retrospective/SKILL.md` |
| the project file | the retrospective's first file, `<YYYY-MM-DD>-<project>-<subject>.md`: findings P on the project's own instructions, and the project's side of an interaction finding, per `skill@knowledge-architect-retrospective@two-files` |
| the workflow file | the retrospective's second file, `<YYYY-MM-DD>-<project>-<subject>-klarch-workflow.md`: findings W on the installed files, C on the checker, and the answers to the standing questions |
| an analysis | one file of docs/retrospective-reports/ at a project's root, named by the stem of the file it analyses, holding one section per finding with a `Default` line and an `Outcome` line |
| the version line | the line of a retrospective that names the version the session used, in `skill@knowledge-architect-retrospective@what-it-examines` |
| the primer | `path@agent-skills@content/PRIMER.md`, installed as .claude/knowledge-architect/PRIMER.md |
| `%%` line | a line of shipped text that the build removes, per `design@agent-skills@shipped-text-line-comments` |
| sent upstream | the fourth outcome this work adds, for a finding whose fix lies in the installed text |

## What the work is

**What exists today.**

- The local intake takes, per `skill@klarch-retrospective-intake@what-it-takes`, the workflow
  files of any project, this repository's own project file, and another project's file for
  grounding only. It establishes eight items per finding
  (`skill@klarch-retrospective-intake@what-to-establish`), writes an analysis into
  `path@knowledge-architect@docs/retrospective-reports/`
  (`skill@klarch-retrospective-intake@analysis-and-ruling`), offers three outcomes, and keeps the
  analysis until its last outcome is carried out
  (`skill@klarch-retrospective-intake@after-the-ruling`). It cites 10 entries of this repository:
  a goal, seven design heads, an issue and a tripwire, which shipped text may not cite, per
  `design@agent-skills@shipped-text-cites-no-entry`.
- The retrospective sends a project file's kept finding straight to an issue entry:
  `skill@knowledge-architect-retrospective@what-becomes-of-files`, "each finding the owner keeps
  becomes an issue entry in the project's own register". The head is
  `design@agent-skills@retrospective-destination`.
- The version line names, for a project that pins the checker, the exact pin only; for a project
  that builds the checker from source, the main commit, the branch and whether the tree held
  uncommitted changes.
- The primer's knowledge table has a row for docs/audit-reports/, and none for an analysis of a
  retrospective file. Its list of installed skills, section `installed-skills`, has no intake.
- This repository's root `CLAUDE.md`, section `where-knowledge-goes`, holds the row that makes
  docs/retrospective-reports/ the home of an analysis of a received file, and section
  `verify-before-relying` holds the line `ls docs/retrospective-reports/`.
- No README, the root's or agent-skills', says how a project extends an installed skill with a
  skill of its own: a search of both for "own skill", "project skill", "add to", "adds to" and
  "extend" found nothing on the subject, in round 3.

**Outside the work.**

- The intake of a workflow file in a consumer project: the consumer does not do the workflow's
  job, on the owner's word (#intake-input).
- Delivering a directive by a copy that the build fills, or by a harness hook: recorded as
  `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`, outside
  this work on the owner's word (#restatement-by-substitution).
- A search instruction for pending analyses: not added (#pending-analysis-search).
- Superseding an installed instruction, rather than adding to it:
  `issue@agent-skills@patching-an-installed-skill`.

## What is already decided

**Decisions the design rests on, not argued again:**

- `design@agent-skills@shipped-text-cites-no-entry` and `design@agent-skills@shipped-text-line-comments`:
  what the installed intake may cite, and where its citations of entries go.
- `design@agent-skills@committed-audit-report`: the model of a committed file of rulings that
  leaves once acted on, which the owner proposed "just like retro intake reports".
- `design@agent-skills@knowledge-table-home`: the workflow's rows ship in the primer, a project's
  own rows sit in its root `CLAUDE.md` and refine them.
- `design@agent-skills@overlay-by-separate-skills` and `design@agent-skills@project-name-prefix`:
  a project adds to an installed skill through a skill of its own, named with its prefix.
- `design@agent-skills@admission-rules-home`: the standard a fix of agent-facing text is shaped
  by lives in `skill@knowledge-architect-agent-configuration@admission-standard`.
- `design@agent-skills@behaviour-additions-need-real-use`: an instruction about how an agent
  behaves is added only on evidence from real use.
- `design@agent-skills@expectation-set-bounds-scope`, `design@agent-skills@synthetic-evidence-not-built`,
  `design@agent-skills@standing-entry-search-agent` and `design@agent-skills@standing-entry-search-groups`:
  cited by the local intake, kept, and cited from `%%` lines of the installed intake.
- `design@knowledge-architect@workflow-findings-stay-here`: this repository handles its received
  workflow files itself, and the local intake keeps saying so.
- The goals `goal@agent-skills@one-skill-per-activity`, which names the intake of a retrospective's
  findings among the activities, and `goal@agent-skills@no-contradiction-in-workflow`, both as the
  first commit of the branch writes them.

**Decisions the work reverses or rewrites, and every text that references each:**

| decision | change | referencing texts, from `cargo klarch show` | judged or updated by |
| --- | --- | --- | --- |
| `design@agent-skills@retrospective-destination` | rewritten in place: the project's findings go to the intake | crates/agent-skills/docs/design.md:85 and :2001; `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`; docs/design.md:436 (in `design@knowledge-architect@workflow-findings-stay-here`) | the harvest reads each; the issue closes in step 6 |
| `design@knowledge-architect@committed-findings-analysis` | narrowed to what this repository adds: the analysis of received workflow files and of audit files | `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search`, closed in step 6; the local intake, rewritten in step 4; `path@knowledge-architect@CLAUDE.md`:58; `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`, closed in step 6; `path@knowledge-architect@docs/retrospective-reports/README.md`, step 4 | the harvest |
| `design@knowledge-architect@intake-shares-the-admission-standard` | moved to agent-skills, same slug, since it decides what the installed intake does | the local intake, line 119, rewritten in step 4 | the harvest |

**Standing entries the discussion's search returned, and their outcome:**

| entry | outcome |
| --- | --- |
| `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis` | closed by this work, step 6 |
| `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search` | closed into `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search`, widened to both directories, step 6 (#pending-analysis-search) |
| `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search` | widened in step 6 |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`, case 2 | the overlap of the installed and the local intake is made on purpose, and the local intake's description names it (AC3); the issue stays open |
| `issue@agent-skills@an-upstream-finding-stays-in-every-later-old-text-count` | bears on the audit's "sent upstream" outcome only; nothing |
| `issue@agent-skills@expectation-sets-for-the-installed-skills` | cited from a `%%` line of the installed intake; stays open |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | AC1 checks the installed intake by hand; the issue stays open |
| `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it` | its trigger names the local intake, which keeps its name; the audit reads whether the trigger also reaches the installed intake |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | its re-entry names the local intake's search of tripwires, which stays; nothing |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` | names the local intake, which keeps its name; nothing |
| `issue@agent-skills@patching-an-installed-skill` | outside the work, above |
| `issue@agent-skills@the-audit-assumes-every-project-merges-by-rebase` | the installed intake states no reason that rests on a rebase merge (step 1); nothing |
| `issue@agent-skills@the-retrospective-counts-no-review-cost` | the retrospective changes in step 3 in sections that issue does not touch; nothing |
| `issue@agent-skills@the-workflow-assumes-one-unnamed-owner` | the installed intake addresses "the owner" as the retrospective does; nothing |
| `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` | the intake ships on real use: its runs in this repository on received files; nothing |
| `tripwire@core@fix-refusal-routed-around` | its re-entry is "each retrospective intake", read in this repository by the local intake, which stays; nothing |
| `tripwire@agent-skills@expectation-set-closes-a-contradiction` | cited from a `%%` line of the installed intake, step 1 |
| `tripwire@agent-skills@pointer-not-followed` | the owner did not see it fire with certainty (round 5); recorded in the new issue |
| `tripwire@agent-skills@agent-text-written-without-the-standard` | the installed text this work writes is written under `skill@knowledge-architect-agent-configuration`, and reviewed on the agent-facing-text axis |
| `tripwire@agent-skills@head-created-without-deliberation`, `tripwire@agent-skills@member-beyond-the-argument`, `tripwire@agent-skills@owner-intent-stripped`, `tripwire@agent-skills@rule-title-wider-than-its-argument`, `tripwire@agent-skills@split-rule-from-exception`, `tripwire@agent-skills@ruling-lost-in-assembly`, `tripwire@agent-skills@ruling-lost-in-change` | read at the harvest, which writes and rewrites heads from this spec's threads |
| `tripwire@agent-skills@plain-text-pointer-found`, `tripwire@agent-skills@search-missed-before-the-work`, `tripwire@agent-skills@axis-not-run-at-a-pin-move`, `tripwire@agent-skills@departure-overridden-as-a-reading`, `tripwire@agent-skills@record-answered-questions-reach-the-owner`, `tripwire@agent-skills@real-use-refusal-overruled`, `tripwire@agent-skills@structure-rejection-overruled`, `tripwire@agent-skills@argued-closure-reopened`, `tripwire@agent-skills@auditor-skips-part-of-the-corpus` | judged by the searchers as possibly bearing; none fired in the discussion; nothing |

Every other entry of the 142 the three searchers read was judged to bear on nothing.

## Criteria

### The installed text names no project's paths or conventions `##names-no-project`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met by #analysis-home-row, which
makes docs/retrospective-reports/ a convention of the workflow, and by
#shipped-citations-in-comments.

### Every activity has one installed skill `##one-skill-each`

Binding, from `goal@agent-skills@one-skill-per-activity`. Met by #intake-skill-home.

### No two installed texts contradict `##no-two-contradict`

Binding, from `goal@agent-skills@no-contradiction-in-workflow`. Met by
#retrospective-hands-project-file-to-intake, which changes the retrospective's fate of the project
file with the intake; AC2 checks it.

### No fixed sequence where judgement serves `##no-fixed-sequence`

Binding, from `goal@agent-skills@installed-text-leaves-room-to-judge`. Met by
#retrospective-hands-project-file-to-intake: the text fixes no moment for the intake.

### The owner rules each finding with its evidence in front of them `##owner-rules-each-finding`

Binding, from `goal@knowledge-architect@the-owner-decides`. Met by the outcomes of the installed
intake, three and "sent upstream" (#installed-fix-goes-upstream).

### An instruction about agent behaviour is added only on real use `##real-use-for-behaviour`

Binding as a presumption, from `design@agent-skills@behaviour-additions-need-real-use`. Met by
#pending-analysis-search. #project-commit-in-version-line adds a field to a record on the owner's
word, and no instruction about behaviour.

### A consumer does not do the workflow's own job `##no-upstream-job`

Binding, from the owner's ruling of round 2. Met by #intake-input.

## Threads

### The installed intake analyses the project file only `##intake-input`

Proposed by the session in round 1 (a1, a2). Approved by the owner in round 2: "yes, the input to
analyze and resolves is only the project file. There is no need to do the job of knowledge-architect
in consumer project, that would be weird." The session's reading, that the workflow file may still
be read for the other side of an interaction finding and nothing resolved from it (a14), was marked
presumed-settled in round 2 and stood through the checkpoint of round 4 without contest, under the
owner's "Agreed" of round 5 on the items put. Shape: Decided design, "What the installed intake
takes". Harvest: the body of the head of #intake-skill-home.

### Design citations of the intake move to `%%` lines, and its "decides nothing" sentence ships with no ground `##shipped-citations-in-comments`

Proposed by the session in round 1 (a4, a5), corrected by the owner in round 2: "the first entry
needs no grounding. Repointing the grounding like you propose would make it a false, unrelated
ground." and "For the other design citations, those should be kept as comments under `%%` lines
that are stripped at publish, not just removed." The session checked that the ungrounded sentence
conflicts with no installed text (a15). Shape: Decided design, "Citations". Harvest: none; the rule
is `design@agent-skills@shipped-text-line-comments`.

### The intake is an activity with an installed skill of its own `##intake-skill-home`

Proposed by the session in round 2 (a16 to a19). The owner gave the facts in round 3 (a34) and
raised the merge with the retrospective (a35). The session argued against the merge (a45, a46,
a47). The owner approved the goal wording of round 3 in round 4, "Agreed on your wording for the
goals.", whose measure takes the reading where a skill points to another at a step (a42). Closed
`argued` in round 4 on that wording. Shape: Decided design, "The installed intake". Harvest: a
design head of agent-skills, slug `intake-skill-home`.

### A finding whose fix lies in installed text is sent upstream `##installed-fix-goes-upstream`

Proposed by the session in round 1 (a6) and round 2 (a20, a21). The owner set the action in round
3: "the action to take should be to edit the retrospecive workflow report, or directly the github
issue if it exists." The session proposed the two forms and the comment (a48, a49). Approved by the
owner in round 4: "installed-fix-goes-upstream: agreed with your proposed process." Shape: Decided
design, "Sent upstream". Harvest: a design head of agent-skills, slug `installed-fix-goes-upstream`.

### The retrospective leaves the project file to an intake the owner starts `##retrospective-hands-project-file-to-intake`

Proposed by the session in round 1 (a7) and round 2 (a22, a23). Approved by the owner in round 3
with its use: "I can agree with that, though it is not really the intended use case, that I
described above. I prefer doing that task in a new session, and if possible with multiple reports at
once." The session reworded the shape for that use (a50). Shape: Decided design, "The retrospective's
project file". Harvest: `design@agent-skills@retrospective-destination`, rewritten in place, and
the losing shape in agent-skills' rejected alternatives if it meets a recording test.

### The version line names the project's main commit in every project `##project-commit-in-version-line`

Proposed by the session in round 1 (a8), withdrawn by the session in round 2 (a24). Reopened and
approved by the owner in round 3: "I think that is worth it. It would save a bit of time and effort
for everyone. It's minor enough that no session would bother asking for it, but it would
definitively be useful, and it carries no downside." The session stated its shape (a51). Shape:
Decided design, "The version line". Harvest: an extended `%%` comment at the version line; whether
it earns a head by entry test 4 is put to the owner at the harvest (D2).

### The primer's table gives the analysis a home, docs/retrospective-reports/ `##analysis-home-row`

Proposed by the session in round 1 (a3) and round 2 (a28, a29), closed `argued` in round 2, and
agreed by the owner in round 3: "agreed on that, this is needed to publish the process, and it
carries no downsides." Shape: Decided design, "The analysis and its home". Harvest: a design head
of agent-skills, slug `committed-intake-analysis`, pair `#analysis-home-row → committed-intake-analysis`,
since the thread's name names the row and not the decision.

### This repository keeps a reduced intake of its own that adds to the installed one `##local-intake-fate`

Proposed by the session in round 1 (a10) and round 2 (a30 to a33). Approved by the owner in round 3:
"agreed. This is quite commonly done (thaum does it too to expand the review and planning skills,
with a thaum-review skill)." Shape: Decided design, "What stays local". Harvest:
`design@knowledge-architect@committed-findings-analysis`, narrowed; `design@knowledge-architect@intake-shares-the-admission-standard`,
moved to agent-skills.

### The README advertises extending an installed skill with a project skill `##extension-by-project-skill`

Proposed by the owner in round 3: "I think this could become the advertised way to expand on
published skills in projects that use it." The session found the agent-facing rule present and no
user-facing text (a52, a53). Approved by the owner in round 4: "extension-by-project-skill: agreed,
good if the agent config skill already carries that." Shape: Decided design, "The README". Harvest:
none; the README is the home, and `design@agent-skills@overlay-by-separate-skills` the decision.

### No instruction is added for a pending analysis, and the two issues on it merge `##pending-analysis-search`

Proposed by the session in round 1 (a9) and round 2 (a25, a26, a27). Closed `argued` in round 4
(a62). Shape: Decided design, "Pending analyses". Harvest: the issue entries of step 6.

### Restatement by substitution is recorded as a design issue `##restatement-by-substitution`

Proposed by the owner in round 4 (a55, a56), after the session found the owner's restatement
paragraph to be a mechanism (a44). The session argued an issue over a head (a58 to a61). Approved by
the owner in round 5: "Agreed on opening the design issue." The owner answered Q1 in round 5 (a64,
a65, a66), and the session corrected one reason from the source (a67). Shape: done; the entry is
`issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`, on the
branch already. Harvest: none.

## Arguments

### The consumer cannot analyse the workflow file `##a1`

Round 1, the session; #intake-input. "the consumer cannot run checks b, c, e and f on it. They need
knowledge-architect's history, rejected alternatives and design heads, which a consumer does not
have."

### Triage before sending filters lapses and cannot check upstream history `##a2`

Round 1, the session; #intake-input. "triage the workflow file before sending it upstream, which
filters lapses but cannot check upstream history."

### The audit-report row is an installed model of the same lifecycle `##a3`

Round 1, the session; #analysis-home-row. "committed with the owner's rulings and deleted when its
last outcome is carried out. That is the same lifecycle as `path@knowledge-architect@docs/retrospective-reports/`."

### Entry citations cannot ship `##a4`

Round 1, the session; #shipped-citations-in-comments. "These can't ship, per
`design@agent-skills@shipped-text-cites-no-entry`", with a proposed re-grounding of the first
entry on the retrospective's "on the owner's word only", which the owner refused (a11).

### The rebase-merge argument of the local head does not transfer `##a5`

Round 1, the session; #shipped-citations-in-comments, #analysis-home-row. "The head at line 188
argues from GitHub's rebase merge dropping commits that change no file. That fact is this
repository's merge setup. A consumer that merges with merge commits doesn't have that problem".

### A P finding fixed in installed text becomes an interaction finding `##a6`

Round 1, the session; #installed-fix-goes-upstream. "The consumer cannot edit installed text. My
default: route c turns it into an interaction finding for the workflow file, sent upstream."

### The retrospective's fate of the project file changes a recorded head `##a7`

Round 1, the session; #retrospective-hands-project-file-to-intake. "both say \"each finding the
owner keeps becomes an issue entry\". That would become \"goes through the intake\", which changes a
recorded head".

### A pinning project's version line names no commit `##a8`

Round 1, the session; #project-commit-in-version-line. "For a project that pins the checker it
records only the pin version (lines 33–37), so check b would have to locate the blamed text with
`git log -S` on the quoted instruction."

### No installed text names a pending analysis's directory `##a9`

Round 1, the session; #pending-analysis-search. "A consumer would get nothing, unless the installed
primer's \"Check whether it is already known\" names the directory. It names neither directory today".

### The upstream part could stay a project skill or move into the root `##a10`

Round 1, the session; #local-intake-fate. "That part could stay as a project skill that adds to the
installed one, or move into the root `CLAUDE.md`."

### Re-grounding the first citation would give a false ground `##a11`

Round 2, the owner; #shipped-citations-in-comments. "the first entry needs no grounding. Repointing
the grounding like you propose would make it a false, unrelated ground."

### Design citations are kept as `%%` lines `##a12`

Round 2, the owner; #shipped-citations-in-comments. "those should be kept as comments under `%%`
lines that are stripped at publish, not just removed."

### Only the project file is analysed in a consumer `##a13`

Round 2, the owner; #intake-input. "There is no need to do the job of knowledge-architect in
consumer project, that would be weird."

### The workflow file may be read for the other side of an interaction `##a14`

Round 2, the session; #intake-input. "reading the workflow file to understand the other side of an
interaction finding, without resolving anything from it."

### The ungrounded sentence conflicts with no installed text `##a15`

Round 2, the session; #shipped-citations-in-comments. "The session establishes the facts, such as
\"already repaired\", and states them in the `Default` line. The owner rules what changes in the
project's text."

### Two artifacts make two activities `##a16`

Round 2, the session; #intake-skill-home. "\"does it produce a nameable artifact? Then it is an
activity, and it owns a skill.\" The retrospective produces the two files outside the project. The
intake produces the analysis file".

### The intake applies at a moment the retrospective's description does not name `##a17`

Round 2, the session; #intake-skill-home. "The intake applies again while its outcomes are carried
out ... A session carrying out a ruled outcome would never load a skill described that way."

### A merged skill would load about twice the text `##a18`

Round 2, the session; #intake-skill-home. "The retrospective has 209 lines and the intake 202. As a
section, every retrospective offer would load about twice the text."

### A new skill adds a member to a goal's list `##a19`

Round 2, the session; #intake-skill-home. "A new skill adds a member to that list. A goal changes
only through `skill@knowledge-architect-goal-setting`, on your word."

### The three outcomes fail on a fix in installed text `##a20`

Round 2, the session; #installed-fix-goes-upstream. "handle now: there is nothing the consumer can
edit; open an issue: the entry would sit in the consumer's register, unclosable there ...; no
change: the finding is lost."

### The audit already sends such a finding upstream `##a21`

Round 2, the session; #installed-fix-goes-upstream. "`skill@knowledge-architect-project-audit@outcomes`
already has a \"sent upstream\" outcome for the same case."

### The direct path to an issue would have recorded 14 of 40 findings wrongly `##a22`

Round 2, the session; #retrospective-hands-project-file-to-intake. "of 40 findings, 14 were already
repaired, already recorded, rested on a false premise, credited a repair the history places
earlier, or proposed a fix that would reverse a recorded head." The figure is the local intake's
first use, recorded in `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`.

### That measurement was not taken on a consumer's project file `##a23`

Round 2, the session; #retrospective-hands-project-file-to-intake. "most of their findings were
workflow findings. Nobody has measured it on a consumer's project file. That is an assumption".

### Each finding quotes its instruction, so no commit is needed `##a24`

Round 2, the session; #project-commit-in-version-line, the withdrawal. "each finding already
\"quotes the instruction\" ...; check b already finds the blamed text with `git log -S` on that
quote; no real session has shown the gap".

### Shipping the row defeats the agent-config issue's premise `##a25`

Round 2, the session; #pending-analysis-search. "Shipping the row defeats the agent-config issue's
premise: the installed text can now name the directory."

### An analysis stays on main across branches by design `##a26`

Round 2, the session; #pending-analysis-search. "An intake analysis is designed to stay on main
across branches until its last outcome is carried out. So the predicted gap is the normal case for
the intake, but it is still only predicted".

### Naming the directory in the primer is a behaviour addition with no session behind it `##a27`

Round 2, the session; #pending-analysis-search. "It loses on the real-use criterion: it is a
behaviour addition with no session behind it."

### The audit report's shape was the owner's, modelled on the intake `##a28`

Round 2, the session; #analysis-home-row. "`design@agent-skills@committed-audit-report` records
that the audit file's shape was your own proposal, \"just like retro intake reports\"."

### A home each project chooses leaves the skill no path `##a29`

Round 2, the session; #analysis-home-row. "The skill could name no path, and the primer's last row
(\"ask the owner\") would fire on the first use in every project."

### The root reaches every session; the upstream procedure is needed at an intake `##a30`

Round 2, the session; #local-intake-fate. "Per where-text-goes, the root reaches every session,
while about 40 lines of procedure are needed only at an intake."

### Keeping the local name keeps three citing entries valid `##a31`

Round 2, the session; #local-intake-fate. "Keeping the name keeps valid the three entries that cite
it: the tripwire, the deferred issue and the core issue in the grounding table."

### The overlap is made on purpose and named `##a32`

Round 2, the session; #local-intake-fate. "The overlap between the two skills is created here on
purpose and named in the local skill's description."

### The admission-standard head becomes a decision about the published workflow `##a33`

Round 2, the session; #local-intake-fate. "moves to agent-skills, since it is now a decision about
the published workflow."

### The intake is its own activity, batched, in a fresh session `##a34`

Round 3, the owner; #intake-skill-home, #retrospective-hands-project-file-to-intake. "the intake is
somehting that happens on my own word only, and I usually batch the intake with a few retrospective
reports, in a later branch, after a few work sessions have produced reports. It is absolutely its
own activity, that nearly never happens right after the retrospective, unless a critical finding
has been found. And the vast majority of times, I do it on a fresh session without memory of the
work and writing the report."

### A merge would avoid duplicating how to read the report `##a35`

Round 3, the owner; #intake-skill-home. "maybe it could be merged with the retrospective skill, to
avoid duplicating informations about how to read the report itself. Not sure what's more natural."

### Two goals were conflated in one `##a36`

Round 3, the owner; the goals. "There were two goals conflated in a single one too, so I split."

### The action is an edit of the workflow file or of the GitHub issue `##a37`

Round 3, the owner; #installed-fix-goes-upstream. "edit the retrospecive workflow report, or
directly the github issue if it exists."

### The intended use is a new session with several reports `##a38`

Round 3, the owner; #retrospective-hands-project-file-to-intake. "I prefer doing that task in a new
session, and if possible with multiple reports at once."

### The commit saves time and costs nothing `##a39`

Round 3, the owner; #project-commit-in-version-line. "It's minor enough that no session would
bother asking for it, but it would definitively be useful, and it carries no downside."

### The row is needed to publish the process `##a40`

Round 3, the owner; #analysis-home-row. "this is needed to publish the process, and it carries no
downsides."

### A project skill that extends an installed one is common practice `##a41`

Round 3, the owner; #local-intake-fate, #extension-by-project-skill. "thaum does it too to expand
the review and planning skills, with a thaum-review skill ... this could become the advertised way
to expand on published skills".

### Every installed skill fails the strict reading of the goal's measure `##a42`

Round 3, the session; #intake-skill-home. "I counted the other installed skills each one points to:
between 2 (issue-tracking) and 8 (project-audit), and none points to zero."

### The split leaves three heads citing the wrong goal `##a43`

Round 3, the session; the goals. Lines 362, 491 and 966 of agent-skills' design home cited the old
goal for its no-contradiction clause; the first commit of the branch re-points them.

### The restatement paragraph is a mechanism `##a44`

Round 3, the session; #restatement-by-substitution. "By the goal test, the same end could be reached
another way, so it is a decision."

### The intake points to the retrospective's sections and copies nothing `##a45`

Round 3, the session; #intake-skill-home. "The intake points to these sections and copies nothing,
so nothing is duplicated."

### The fresh session defeats the merge `##a46`

Round 3, the session; #intake-skill-home. "A merged skill would load the retrospective's procedure
for examining \"this session\" into a session that has no session to examine."

### Build-time injection is needed only under the strict reading `##a47`

Round 3, the session; #intake-skill-home. "A third shape would inject the shared format text into
both skills at build time. It is only needed under the strict reading of your goal."

### The two forms of sending upstream `##a48`

Round 3, the session; #installed-fix-goes-upstream. "The finding is added to that file as a W
finding with the next free number. It becomes an interaction finding" / "Its body is edited with
`gh issue edit <n> --body-file <file>`, on your word, after you have read it."

### A body edit notifies nobody `##a49`

Round 3, the session; #installed-fix-goes-upstream. "editing an issue body notifies nobody. ... My
default: the edit also posts a one-line comment that names the added finding".

### Held findings are in no register, by the owner's choice `##a50`

Round 3, the session; #retrospective-hands-project-file-to-intake. "until the intake runs, the
findings are in no register ... You accept that by holding the files until a batch".

### The intake reads the blamed text at the named commit `##a51`

Round 3, the session; #project-commit-in-version-line. "The rule \"a commit of the branch itself is
not named\" stays." / "The intake's step 2 then reads the blamed text at that commit, with
`git show <commit>:<path>`".

### The agent-facing rule exists, no user-facing text advertises it `##a52`

Round 3, the session; #extension-by-project-skill. "So the agent-facing rule exists. No user-facing
text advertises it".

### No agent-facing text is added `##a53`

Round 3, the session; #extension-by-project-skill. "No agent-facing text is added, since the need
test would delete a restatement."

### The draft keeps the retrospective in the activity list `##a54`

Round 3, the session; the goals. "the old text had it, and your list is marked non-exhaustive."

### Pointers fail at delivery `##a55`

Round 4, the owner; #restatement-by-substitution. "pointers fail on a very important aspect:
delivery of the instruction, which requires fetching the pointed section. The substitution method
would probably be much better to improve delivery without manually maintained, drifting
restatements. Though applying it widely in the project falls out of the scope of our current
session."

### The owner wants the idea recorded `##a56`

Round 4, the owner; #restatement-by-substitution. "Maybe it should be a design head instead." / "I
believe it to be a very good idea, that I want recorded."

### The README is enough where the agent configuration already carries the rule `##a57`

Round 4, the owner; #extension-by-project-skill. "agreed, good if the agent config skill already
carries that."

### Substitution escapes the reason the restatement rival lost on `##a58`

Round 4, the session; #restatement-by-substitution. "Your mechanism keeps one source and fills the
copies at build time, so it escapes the reason that rival lost on."

### The weakness is watched by a tripwire `##a59`

Round 4, the session; #restatement-by-substitution. `tripwire@agent-skills@pointer-not-followed`.

### A substituted copy answers drift, not size `##a60`

Round 4, the session; #restatement-by-substitution. "A substituted copy still loads its text, so it
answers drift, not size."

### A design head now fails on built intent and on a hedged word `##a61`

Round 4, the session; #restatement-by-substitution. "A design home holds built intent" / "Your
words, \"would probably be much better\", are hedged."

### The precedent of the audit-report issue `##a62`

Round 4, the session; #pending-analysis-search. "Ground: `design@agent-skills@behaviour-additions-need-real-use`,
and the precedent of the audit-report issue for the same mechanism."

### Checks a grep can apply are acceptance criteria `##a63`

Round 4, the session; the premortem. "These three can only be checked by grep on the built text, so
they are acceptance criteria rather than tripwires".

### Agents do not always read a pointed section again `##a64`

Round 5, the owner; #restatement-by-substitution. "there is no global instruction that tells to do
that AFAIK." / "agents tend to work from memory, especially of content they have already read" /
"An existing tripwire does not always mean the problem is being watched."

### Working from memory is often a defensible call `##a65`

Round 5, the owner; #restatement-by-substitution. "in most cases, reading the pointed section is not
truly important, and working from memory is actually a very defensible call" / "in some cases,
rereading the instruction is important (necessary even). ... I'm not certain it is always asked when
needed."

### A harness hook could deliver the pointed sections `##a66`

Round 5, the owner; #restatement-by-substitution. "a harness hook that would deliver the pointed
sections whenever you read a text with a pointer to an instruction text (skill, agent, primer and
root CLAUDE.md)."

### The primer asks a re-read pointer by pointer `##a67`

Round 5, the session; #restatement-by-substitution. "`primer@where-knowledge-goes` does require an
instruction at each pointer ... It is a rule for each pointer, not a global one, and whether every
pointer that needs it carries it is unverified."

## New names, in one place

```
crates/agent-skills/content/skills/retrospective-intake/SKILL.md   new: the installed intake's source
.claude/skills/knowledge-architect-retrospective-intake/SKILL.md     new: its installed copy, written by the install
knowledge-architect-retrospective-intake                             new: the installed skill's name
sent upstream                                                         new: the fourth outcome of the installed intake
docs/retrospective-reports/                                           existing here; new as a workflow convention
.claude/skills/klarch-retrospective-intake/SKILL.md                  existing: the local intake, rewritten
```

## Decided design

### The installed intake

A skill of its own, `planned@agent-skills@content/skills/retrospective-intake/SKILL.md`, installed
as knowledge-architect-retrospective-intake. Its description triggers when the owner hands the
session one or more of the project's retrospective files, and while an outcome of an analysis in
docs/retrospective-reports/ is carried out. It says it runs on the owner's word, usually in a
session that did not write the files, on several files at once. It points to the retrospective for
the report's format, and copies none of it: `skill@knowledge-architect-retrospective@two-files`
for the files and the ids, `skill@knowledge-architect-retrospective@what-it-examines` for
unclear, missing and wrong, and `skill@knowledge-architect-retrospective@expectation-sets` for the
owner's behaviour.

Nearest rival: a section of the retrospective (a35). Defeated by the owner's facts (a34): the intake
runs in a fresh session with no session to examine, and it writes another artifact than the
retrospective (a16, a46). Under the goal wording the owner approved, a pointer to the retrospective's
sections at the step that needs them meets `goal@agent-skills@one-skill-per-activity` (a42).

### What the installed intake takes

The project file, any number of them at once. The workflow file of the same retrospective is read
only for the other side of an interaction finding, and nothing of it is resolved (a14). A finding
is cited by its id and its file's stem, as the retrospective gives it.

Nearest rival: triage the workflow file before sending it (a2), defeated by a1 and by the owner's
ruling (a13).

### Citations

Every reference of the local intake to a goal, a design head, an issue or a tripwire of this
repository becomes a `%%` line beside the sentence it supports, in the installed source; the
sentence ships. The sentence "This skill decides nothing", and its explanation that the session
establishes the facts and proposes, while the owner rules, ships with no ground and no `%%` line
for a ground (a11). References to installed skills, agents and the primer stay, since every
installing project holds them. The head at line 188 of the local intake, whose argument rests on
this repository's rebase merge, is cited from a `%%` line only beside the lifecycle, whose installed
argument is the audit record's: the commit messages alone keep the outcomes and lose the evidence
(a5).

### The checks a to h, in the installed intake

As in `skill@klarch-retrospective-intake@what-to-establish`, with these changes:

- **b**: the blamed text is read at the main commit the version line names, with
  `git show <commit>:<path>`, then on the main branch as it stands (a51).
- **d**: the expectation sets bound only a finding about the owner's behaviour under an installed
  skill; a finding on the project's own skill has no set.
- **e**: the owner of the fix is the Component that owns the blamed text, or the installed text.
  A fix in the installed text routes to "sent upstream".
- **g**: the routes are `skill@knowledge-architect-agent-configuration` for the project's own
  agent configuration; `skill@knowledge-architect-design` for a decision whose design is not
  settled; the project's own development procedure for its code; an issue; sent upstream; nothing.

### Sent upstream

A finding whose fix lies in the installed text takes the outcome "sent upstream", on the owner's
word and after the owner has read the text sent:

- **the workflow file of the same retrospective is not sent yet**: the finding is added to it as a
  W finding with the next number of that file, and becomes an interaction finding, each id naming
  the other, as `skill@knowledge-architect-retrospective@what-it-examines` says of interactions;
- **the GitHub issue that carries it exists**: its body is edited, `gh issue edit <n> --body-file <file>`,
  and a one-line comment names the added finding, `gh issue comment <n>`, since a body edit
  notifies nobody (a49). Where `gh` is absent or fails, the owner is given the text and the
  issue's address, as the retrospective does.

Nearest rival: the three outcomes alone (a20), each of which loses the finding or files it where it
cannot close.

### The retrospective's project file

`skill@knowledge-architect-retrospective@what-becomes-of-files`, the bullet on the project's file,
says that the file stays where the files went until the owner hands it to an intake, usually in a
later session and with other files, under the installed intake, which puts each finding to the
owner with its evidence and a default outcome. The retrospective offers no intake at its own end.
`design@agent-skills@retrospective-destination` is rewritten in place to match. Illustration of the
shape, not authority:

```
- The project's file: it stays where the files went until the owner hands it to an intake,
  usually in a later session and with other files, under the installed intake skill, which puts
  each finding to the owner with its evidence and a default outcome.
```

Nearest rival: a kept finding straight to an issue entry, defeated by a22, with the limit a23 kept.

### The version line

In every project, the version line names the main commit the session's tree contains,
`git rev-parse --short $(git merge-base HEAD <main branch>)`, the branch the session worked on, and
whether the tree held uncommitted changes. Where the project pins the checker, it also names the
exact pin; where it builds the checker from source, `git describe --tags` of that commit. A commit
of the branch itself is still not named. The `%%` comment above the line gains the reason: the
intake reads the blamed text at that commit (a51). Ground: the owner's weighing (a39).

### The analysis and its home

The primer's knowledge table gains a row: the analysis of a retrospective's project file, and the
owner's ruling on each of its findings; its home is one file per analysed file in
docs/retrospective-reports/ at the project's root, as the installed intake says; it leaves when the
commit that carries out its last outcome, or a later commit of the same branch, deletes it. The
primer's list of installed skills, section `installed-skills`, gains the intake's line. The
analysis's sections, its `Default` and `Outcome` lines, the ruling written in the file, and its
life after the ruling, are those of `skill@klarch-retrospective-intake@analysis-and-ruling` and
`skill@klarch-retrospective-intake@after-the-ruling`, with four outcomes and "under the project's
own branch rules" for this repository's `## Git`. The installed intake asks for no README in the
directory.

Nearest rival: a home each project chooses (a29).

### What stays local

`skill@klarch-retrospective-intake` keeps its name (a31) and becomes a skill that adds to the
installed intake. Its description says so and that both load. It holds only:

- workflow files of any project, W, C and the standing answers, and the files of an audit's
  agentic-workflow axis; and this repository's own project file;
- another project's file, read for grounding only, and the rule that an analysis names nothing of
  that project in detail;
- the version of a received file, taken with `git describe --tags origin/main`;
- the routes to `path@agent-skills@content/` under `path@agent-skills@CLAUDE.md`, section "Editing
  an installed skill or agent", and to `skill@klarch-development`;
- for a workflow file, the three outcomes without "sent upstream", since this repository is the
  upstream;
- the pointer to `design@knowledge-architect@workflow-findings-stay-here`.

This repository's root `CLAUDE.md` row for docs/retrospective-reports/ refines the primer's new row
with the received files, per `design@agent-skills@knowledge-table-home`, and its line
`ls docs/retrospective-reports/` stays. The directory's README says it also holds the analyses of
this repository's own project files.

Nearest rival: the upstream part moved into the root `CLAUDE.md` (a30).

### The README

`path@agent-skills@README.md` gains a section on extending an installed skill: a project skill
named with the project's prefix and the activity of the installed skill it adds to, whose
description says it adds to that skill and loads with it, adding steps, an axis or a convention and
never contradicting it. It names thaum's review skill and this repository's intake as examples, and
points to `skill@knowledge-architect-agent-configuration@installed-files-never-edited`. No
agent-facing text is added (a53). `path@knowledge-architect@README.md`, whose line 13 lists the
activities ending "and a retrospective", names the intake too.

### Pending analyses

No instruction is added. `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search`
closes into `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search`, which is
widened to docs/retrospective-reports/, with the fact a26 that an analysis stays on the main branch
across branches by design. Nearest rival: the directory named in the primer's "Check whether it is
already known" (a27).

## Mapping tables

The local intake, section by section, to where its text goes:

| local section or part | installed intake | local intake |
| --- | --- | --- |
| description | rewritten: the project's files, the owner's word, a fresh session, several files | rewritten: adds to the installed intake, for received workflow and audit files |
| scope and "not covered" | kept, generalised | a one-line scope naming what it adds |
| "This skill decides nothing" | kept, no ground | not restated |
| `what-it-takes`, the workflow file | the other side of an interaction only | kept |
| `what-it-takes`, this repository's project file | the project file, `<date>-<project>-<subject>.md` | kept, as one more file it takes |
| `what-it-takes`, another project's file | dropped | kept |
| `what-it-takes`, `workflow-findings-stay-here` | dropped | kept |
| `what-it-takes`, the citation form | kept | not restated |
| `ground-before-judging` 1, 3, 4 | kept; design citations in `%%` lines | not restated |
| `ground-before-judging` 2 | the version line's commit and pin | `git describe --tags origin/main` for a received file |
| `what-to-establish` a, c, f, h | kept; design citations in `%%` lines | not restated |
| `what-to-establish` b | the named commit, then the main branch | not restated |
| `what-to-establish` d | kept, the sets for installed skills only | not restated |
| `what-to-establish` e, g | generalised, with sent upstream | the routes to content/ and to `skill@klarch-development` |
| `intake-subagents` | kept; its design citation in a `%%` line | not restated |
| `analysis-and-ruling`, the path and the sections | kept, docs/retrospective-reports/, four outcomes | three outcomes for a workflow file |
| `analysis-and-ruling`, "Name nothing of the other project" | dropped | kept |
| `analysis-and-ruling`, the directory's README | dropped | kept |
| `after-the-ruling` | kept, "under the project's own branch rules" | not restated |

## Losing alternatives

- **The intake as a section of the retrospective**, lost to #intake-skill-home: a34, a46.
- **Build-time injection of the report's format into both skills**, lost to #intake-skill-home: a47,
  under the loose reading the owner approved. The general question is the new issue.
- **The workflow file triaged in the consumer**, lost to #intake-input: a1, a13.
- **The three outcomes alone**, lost to #installed-fix-goes-upstream: a20.
- **A kept finding straight to an issue entry**, lost to #retrospective-hands-project-file-to-intake:
  a22.
- **A home each project chooses**, lost to #analysis-home-row: a29.
- **The upstream part in the root `CLAUDE.md`**, lost to #local-intake-fate: a30.
- **The directory named in the primer's search of what is known**, lost to #pending-analysis-search:
  a27.
- **A design head for restatement by substitution now**, lost to #restatement-by-substitution: a61.
- **The version line unchanged**, the session's withdrawal (a24), superseded by the owner's
  reopening (a39).

## Readings

The work reads no external specification.

## Premortem

| cause | thread stressed | verdict |
| --- | --- | --- |
| the intake costs more than it yields on consumer project files, which nobody measured (a23) | #retrospective-hands-project-file-to-intake | T1, not recorded on the owner's word, round 5: "keep none of the three" |
| in this repository a workflow file is handled under the installed intake's project-file scope | #local-intake-fate | T2, not recorded, same word; AC3 checks the description |
| a finding added to a GitHub issue after its analysis here is missed | #installed-fix-goes-upstream | T3, not recorded, same word; converted into the comment of "Sent upstream" |
| an installed text still cites an entry of this repository | #shipped-citations-in-comments | AC1, applied on the owner's word, round 5: "You can keep the three acceptance criteria." |
| an installed text still sends a kept finding straight to an issue | #retrospective-hands-project-file-to-intake | AC2, applied, same word |
| the local and installed intakes overlap unnamed | #local-intake-fate | AC3, applied, same word |
| a pending analysis is missed by a design | #pending-analysis-search | survives into the widened issue of step 6 |

## Acceptance criteria

### AC1: the installed intake cites no entry outside a `%%` line `##no-entry-cited`

Guards #shipped-citations-in-comments. Judged at step 1 and again at the review. Observable: a
search of the source for a span of the kinds `goal`, `design`, `issue`, `tripwire`, `spec` or
`milestone` on a line that does not begin with `%%`, and of the installed copy for any span of those
kinds, finds one. Response: the span moves to a `%%` line, and the review asks whether
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` needs its check now.

### AC2: no installed text sends a kept project finding straight to an issue `##no-direct-issue-path`

Guards #retrospective-hands-project-file-to-intake. Judged at step 3. Observable: a reading of the
retrospective, the primer and the issue-tracking skill, after a search for "becomes an issue entry"
and "each finding the owner keeps" matched across a line break, finds such a sentence. Response:
the sentence is rewritten to name the intake, or the thread reopens if a text needs the direct path.

### AC3: the local intake names the installed one it adds to, and both load `##local-adds-to-installed`

Guards #local-intake-fate. Judged at step 4. Observable: the local intake's description does not
name knowledge-architect-retrospective-intake as the skill it adds to, or the installed copy is
absent from .claude/skills/, or `cargo klarch check` fails. Response: repair the description or
the install; a check failure that the overlap causes reopens the thread.

## Implementation sequence

1. **The installed intake.** Write `planned@agent-skills@content/skills/retrospective-intake/SKILL.md`
   from the local intake, by the mapping table and the decided design, under
   `skill@knowledge-architect-agent-configuration`, read whole, and
   `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent". Add its line to
   the primer's list of installed skills. Run `cargo klarch install-agent-skills` and commit the
   installed copy with the source. Fails alone on: the build refusing a `%%` line or a slug
   placeholder; the check; AC1.
2. **The primer's row** for docs/retrospective-reports/, then the install. Fails alone on: the
   check of the installed primer.
3. **The retrospective**: the bullet on the project's file and the version line, then the install.
   Fails alone on: AC2.
4. **The local intake reduced**, with the root `CLAUDE.md` row, the directory's README and AC3.
   Fails alone on: AC3, and a dangling reference to a section of the local intake that moved.
5. **The READMEs**: the extension section of `path@agent-skills@README.md` and the activity of
   `path@knowledge-architect@README.md`. Fails alone on: the check of their links and references.
6. **The issues**: close `issue@agent-skills@a-received-retrospective-file-has-no-installed-analysis`
   and `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search`;
   widen `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search`. Fails alone
   on: a dangling reference to a closed entry.
7. **The changelog**: its entries in `Next release` of `path@knowledge-architect@CHANGELOG.md`,
   after reading `design@knowledge-architect@changelog-entries` and
   `design@knowledge-architect@versioning-policy` whole, then `cargo x changelog`. Fails alone on:
   the test that compares the shipped copies.
8. **The harvest**, the review before the merge, and the deletion of this spec.

## Order rationale

- 1 before 2: the row names the installed intake, which must exist for its pointer to resolve.
- 2 before 3: the retrospective sends the project file to a home the primer already states.
- 3 before 4: the local intake states what it adds once the installed text it adds to is final.
- 4 before 5: the README names the local intake as an example in its final shape.
- 5 before 6: the closing issues' sites are all rewritten before the entries leave.
- 6 before 7: the changelog describes the finished change.
- 7 before 8: the harvest records what the commits built.

## Defaults awaiting the owner

- **D1**: this is a spec, not a milestone: one branch, one pull request. Bears on the whole work.
  The owner rules at the review.
- **D2**, at the harvest: #project-commit-in-version-line earns no design head, and its reason is
  the `%%` comment at the version line and the commit; it earns one only if the owner confirms, by
  entry test 4 of `primer@design-heads`, that it records the owner's own intent. The question is
  put as `Q<n>` at the harvest.

## Harvest

| thread | home | when |
| --- | --- | --- |
| #intake-skill-home, with #intake-input in its body | a head of `register@agent-skills@design`, slug `intake-skill-home` | step 8 |
| #installed-fix-goes-upstream | a head of `register@agent-skills@design`, slug `installed-fix-goes-upstream` | step 8 |
| #retrospective-hands-project-file-to-intake | `design@agent-skills@retrospective-destination`, rewritten in place; the direct path in `path@agent-skills@docs/rejected-alternatives.md` if it meets a recording test | step 8 |
| #analysis-home-row | a head of `register@agent-skills@design`, slug `committed-intake-analysis`, pair `#analysis-home-row → committed-intake-analysis` | step 8 |
| #local-intake-fate | `design@knowledge-architect@committed-findings-analysis`, narrowed; `design@knowledge-architect@intake-shares-the-admission-standard`, moved to `register@agent-skills@design` | step 8 |
| #project-commit-in-version-line | the `%%` comment at the version line, and a head only under D2 | step 3, and D2 at step 8 |
| #shipped-citations-in-comments, #extension-by-project-skill, #pending-analysis-search, #restatement-by-substitution | no head: their homes are named in their threads | — |
| the losing alternatives | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | step 8 |

## Later consequences

- A design discussion on `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`
  may replace the installed intake's pointers to the retrospective with copies the build fills.
- A consumer's intake that finds the cost too high reaches this repository only through the
  owner's observation or a workflow file, since T1 was not recorded.
