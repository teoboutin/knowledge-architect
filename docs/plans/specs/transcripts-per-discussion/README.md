# A design discussion's transcript ends at the owner's word on its last convergence checkpoint, one numbered file per discussion, pushed with no word of the owner

## Status and audience

**This spec was written after its work was built.** The design discussion it records converged on
the full path's checkpoint, and the session then took the in-change path, which
`skill@knowledge-architect-design@decision-authority` gives only to a decision met inside a change
already under way or to a decision no work implements: neither held, since no change was under way
and the decision needs work. The work was built on that wrong path, without a spec, in these
commits of the branch, in order:

1. "[model] A plan document's directory holds its transcripts and its search files numbered, one
   per design discussion and per design audit";
2. "[agent-skills] A design discussion's committed transcript ends at the owner's word on its last
   convergence checkpoint, one file per discussion, and is pushed with no word of the owner", whose
   message carries the deliberation;
3. "[goals] A push to the project's repository is no action that cannot be undone the owner
   decides, and what a push sets off is judged on its own";
4. "[issues] Sessions misread which course a design discussion takes, an issue on the owner's
   request", which records this instance and others the owner reported:
   `issue@agent-skills@sessions-misread-which-course-a-design-discussion-takes`.

The owner then directed that the spec be written as the skill mandated, then reviewed, then the
implementation reviewed against it before the merge. So its steps and its harvest describe the work
as built, for that review to compare them; they plan nothing new. What the full path owed before
the work and the work did not take: the code-split scout and the design-record auditor over the
heads the harvest touches, at the grounding and at the list of touched heads, and the reviews of a
plan document, which the review of this spec now runs.

The review of this spec repaired the work in the commit "[review] The committed transcript's end
point reads one way, its number is taken at its commit, and the record says what the narrower span
leaves unchecked", named below the first review repair. It changed the planning skill's transcript
bullet, which now names one end point and says to cut the output after it;
`design@agent-skills@committed-transcript`, whose number commit 2 took in the order the
discussions began and which it aligned with the skill's next number;
`design@agent-skills@plan-records-the-exchange`; the reason
`rejected@agent-skills@owner-words-verbatim-in-plan-items` lost on, now that the committed
transcripts hold the words of the design discussions in context;
`issue@agent-skills@sessions-misread-which-course-a-design-discussion-takes`; and the doc comment
of the test `each_odd_shape_of_the_plans_layout_is_one_phase_two_finding` in
`path@model@src/tree.rs`. The review before the merge repaired the work again, in the commits that
follow the commit "[review] The spec on committed transcripts states what its work rewrites and
departs from, and corrects its counts and attributions" on the branch.

This spec is written for a session that did not witness the discussion. It leaves the repository in
a commit of its own, cited as `spec@plans@transcripts-per-discussion`, after the review before the
merge has read the tip that holds it.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- No default awaits the owner, per "Defaults awaiting the owner".

**The discussion and its rounds.** It was assembled from the transcript of one session, the one
holding the owner's message that begins "i want to discuss the recently introduced step of the
workflow that commits the transcript". The committed transcript,
[transcripts/1.txt](transcripts/1.txt), runs from that message to the owner's word on the last
convergence checkpoint table, the message that begins "For the goal: you can drop". The rounds,
each named by the opening words of the owner's message that opens it:

| round | the owner's message begins | in the transcript |
| --- | --- | --- |
| 1 | "i want to discuss the recently introduced step of the workflow" | yes |
| 2 | "agreed on B, agreed on transcript-commit-timing" | yes |
| 3 | "transcript-files-per-discussion agreed on general shape" | yes |
| 4 | "Agreed on drafting a sentence for push-reading-in-goal" | yes; the session's last message of this round shows the convergence checkpoint |
| 5 | "For the goal: you can drop" | its opening message only, which closes the checkpoint |
| 6 | "agreed on this wording" | no: a ruling on the goal's wording after the checkpoint, attributed in `thread@transcripts-per-discussion@push-reading-in-goal` and in the message of commit 3 |

After round 6, the owner asked why no spec was written, asked for the issue of commit 4, and
directed that this spec be written; those messages bear on how the work proceeds, not on its
design, and no item cites them.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec. Here the work is
built: its review before the merge reads it against this spec.

## Names

- **the checkpoint table**: the table of every thread with its state that the design skill shows
  for the owner's batch confirmation before the hand-off to planning, per
  `skill@knowledge-architect-design@threads-and-states`; "the last convergence checkpoint" is the
  last such table a discussion showed before the commit that adds or rewrites its plan document:
  the one before the hand-off to planning, or, where a plan review reopens a thread, the one that
  follows the reopening.
- **a design discussion**: a discussion under `skill@knowledge-architect-design` on its full path.
- **the full path, the in-change path**: the design skill's two paths, per
  `skill@knowledge-architect-design@decision-authority`.
- **the removal step**: the reading of a transcript whole before its commit, removing every path
  under a home directory, every e-mail address, every credential and token, and the pasted material
  the owner marks, per `design@agent-skills@transcript-removal-step`.
- **the record search**: the saved workflow `knowledge-architect-record-search`, run at this
  discussion's grounding, whose generated lines are [search/discussion-1.txt](search/discussion-1.txt).
- **the single-file names**: `transcript.txt`, `search/discussion.txt` and
  `search/design-audit.txt`, the names a plan document's directory admitted before this work.
- **the numbered names**: `transcripts/<n>.txt`, `search/discussion-<n>.txt` and
  `search/design-audit-<n>.txt`, `<n>` decimal digits counted from 1 with no leading zero.
- **F1 to F6, N1 to N5**: the findings of the goal reviewer's first and second rounds on the
  goal's sentence, listed in `argument@transcripts-per-discussion@arg-25` and
  `argument@transcripts-per-discussion@arg-33`: F1 to F3 on the first wording, which the second
  replaced; F4 to F6 cases a push can hold or set off; N1 to N4 questions on intent, and N5 a
  wording defect.
- **the doc-gate-and-lints spec**: the spec deleted by the commit "[plans] The spec on the doc gate
  and the documentation lints leaves, its work landed, with the tripwire whose horizon its deletion
  passes", on main; its transcript is the measurement of `argument@transcripts-per-discussion@arg-4`.
- **the `dirhome` mock**: the mock project `path@testing@tests/projects/dirhome/`, whose plan documents
  the conformance tests check.

## What the work is

What existed before the work, at each site it touches:

- **The planning skill**, `path@agent-skills@content/skills/planning/SKILL.md`: a plan document's
  directory held one `transcript.txt`, the transcript of its discussion from the message where it
  began to the owner's last message before the branch that adds the plan document merges,
  committed after the review before that merge; no commit holding it was pushed before the owner's
  word for that transcript. `search/` held `discussion.txt` and `design-audit.txt`.
- **The checker**, `path@model@src/manifest.rs`, `path@model@src/plan.rs` and
  `path@model@src/tree.rs`: a plan document's directory admitted `transcript.txt` and, in
  `search/`, the two single-file names, and refused any other file.
- **The design record**: `design@agent-skills@committed-transcript` held the span, the timing and
  the removal step in one head; the head `transcript-pushed-on-the-owner-s-word` held the owner's
  word before each push; eleven heads of the design homes named the single-file names, counted at
  origin/main, the branch's base, as the level-three headings whose body names one:

  ```sh
  for f in $(git ls-tree -r --name-only origin/main | grep -E '(^|/)docs/design(\.md|/.*\.md)$'); do
    git show origin/main:$f | awk -v f=$f '/^## /{h=""} /^### /{h=$0}
      /transcript\.txt|discussion\.txt|design-audit\.txt/ && h!="" {print f, h}'
  done | sort -u
  ```
- **The goal**: `goal@the-owner-decides` reserved to the owner every action that cannot be undone,
  and said nothing of a push.

Outside the work:

- **Which course a design discussion takes**, the in-change path, the full path or bounded work:
  `issue@agent-skills@sessions-misread-which-course-a-design-discussion-takes`, whose discussion
  the owner asked to hold later.
- **A command option that does the removal step mechanically**:
  `issue@transcript@an-option-anonymizes-what-transcript-prints`, whose "Why it matters" the work
  rewrites and whose work it does not do.
- **A publication on another repository**, such as an issue an adopting project opens on
  knowledge-architect: `design@agent-skills@installed-fix-goes-upstream` and
  `design@agent-skills@intake-outcome-authority` stand.
- **The splits of `resolver.rs`, `manifest.rs` and `binary.rs`**: not weighed, per "What is
  already decided".

## What is already decided

The decisions the design rests on and does not argue again:

- `goal@the-owner-decides`, which the work reads as the owner ruled, per
  `thread@transcripts-per-discussion@push-reading-in-goal`;
- `primer@design-heads`: an approval of a default is no ground, and a durable record attributes the
  owner's words;
- `rejected@agent-skills@owner-words-verbatim-in-plan-items`: committing no transcript;
- `design@agent-skills@search-runs-once-the-shapes-settle`: a thread opened after the search gets a
  search of its own;
- `design@model@a-harvested-slice-is-frozen`: a harvested slice is judged by no rule;
- `design@knowledge-architect@changelog-entries`: a change to something unreleased edits the entry
  that announces it.

**Reversed or rewritten**, each repaired in commit 2 unless named otherwise:

| decision | how | texts repaired |
| --- | --- | --- |
| `design@agent-skills@committed-transcript` | its span reason defeated by `thread@transcripts-per-discussion@transcript-span`; rewritten in place, its removal step split out to `design@agent-skills@transcript-removal-step`, per `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`, and the numbering's reason and rivals moved to `design@model@plan-document-directory` by the review before the merge | the planning skill's layout, file table, term "transcript" and assembly bullets; the review skill's sentence on the commit that adds a transcript after the review; the transcript reviewer's agent |
| `transcript-pushed-on-the-owner-s-word`, now `design@agent-skills@transcript-pushed-with-the-branch` | reversed by `thread@transcripts-per-discussion@transcript-push-approval` | the planning skill's sentence on the owner's word before a push, deleted; `issue@transcript@an-option-anonymizes-what-transcript-prints`, its "Why it matters" |
| `design@agent-skills@attribution-replaces-quotation` | reworded: the verbatim words of a design discussion stay in its transcript, and a ruling after its last checkpoint is attributed only | the design skill, the primer's row of plan documents, and the head-ground fragment, which ships in the primer, the decision-recording and design skills, the decision-record reviewer and the design-record auditor; `issue@agent-skills@the-quick-fixes-spec-quotes-the-owner-in-its-items`, which names the transcript in words |
| `design@model@plan-document-directory` | rewritten to the numbered names, in commit 1 | in commit 1: the regime table of `design@model@a-harvested-slice-is-frozen`, `rejected@model@spec-as-one-file`, `path@check@README.md`, and the comments and fixtures that named the single-file names |
| `goal@the-owner-decides` | rewritten in commit 3 and reworded at the review before the merge, on the owner's rulings: one sentence added, that a push to the project's repository is no action that cannot be undone the owner decides, and that an action the push sets off stays the owner's where it cannot be undone, beyond notifying what the push holds, per `thread@transcripts-per-discussion@push-reading-in-goal` | none: `design@knowledge-architect@owner-pushes-the-release-tag` reads the goal on a push, that a push can be undone, and agrees with the sentence |

**Departures the wrong path produced**, from heads that stand and are not reversed:

- `design@agent-skills@in-change-path`: a requested change that is not bounded and whose work has
  not started when its discussion opens takes the full path; this one took the in-change path, per
  "Status and audience".
- `design@agent-skills@plan-landing-is-not-tied-to-its-work`: a plan document lands before any work
  that changes what the per-commit gate checks; commit 1 changes it, and this spec is committed
  after the work, on the same branch.

The reversal check of commit 2 searched the design homes, the `rejected` homes, the scoped
`CLAUDE.md` files and the installed text's source for the wording of both reversed heads; no
tripwire guards either.

**The heads the harvest touches** without reversing or rewriting their decision, each rewritten to
the numbered names in commit 2 with its rule unchanged, unless named otherwise:
`design@agent-skills@discussion-record-apart`, `design@agent-skills@ledger-from-transcript`,
`design@agent-skills@plan-records-the-exchange`, whose premise that the committed transcript lets a
reviewer check the rulings is narrowed to the rulings of the design discussions, repaired in the
first review repair;
`design@agent-skills@plan-commits-the-generated-lines`, which names the numbered files as
`design@model@plan-document-directory` does: the numbering's reason and rivals are in that head,
and the shared number of a discussion's search and its transcript, with the append of a second
search, is stated by the planning skill alone, its reason and rival in a comment of the skill's
source beside it, as the review before the merge repaired them;
`design@agent-skills@search-record-kept-per-occasion`,
`design@agent-skills@a-seed-and-its-citers-are-never-unrelated`,
`design@agent-skills@design-audit-checks-fit`,
`design@agent-skills@verdicts-reach-the-conformance-review`, and, in commit 1, the regime table of
`design@model@a-harvested-slice-is-frozen`. No design-record auditor read them
before the work, per "Status and audience".

**The record search's verdicts that changed something**, the others in
[search/discussion-1.txt](search/discussion-1.txt):

- the `conflicts` verdicts on the heads above: reversed, reworded or rewritten as listed;
- `design@agent-skills@installed-fix-goes-upstream` and
  `design@agent-skills@intake-outcome-authority`, `conflicts`: standing; that the owner's reading
  covers a push to the project's own repository only was the session's answer to these two
  verdicts, approved by the owner's word on the checkpoint table in round 5, and the goal's
  sentence says so;
- `goal@the-owner-decides`, `constrains`: rewritten in commit 3, on the owner's ruling, as the
  table above says;
- `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`,
  `trigger-met`: `design@agent-skills@committed-transcript` split in two, as above;
- `issue@model@resolver-rs-is-past-one-read`, `issue@model@manifest-rs-is-past-one-read` and
  `issue@testing@binary-rs-is-past-one-read`, `trigger-met`: no split weighed, on the owner's
  direction that work built before its spec takes in correctness changes only, and each entry
  records that the weighing passes to the next work that changes the file's code. The work changes
  the constant `TRANSCRIPT_FILE` of `path@model@src/manifest.rs` and its doc comment; in
  `path@model@src/resolver.rs`, the closure `spec_dir_file` and the comment above it, one match arm
  of `plan_document`, and the rows and the doc comment of the test
  `the_p1_repair_names_the_form_for_each_position`; and one test fixture of
  `path@testing@tests/binary.rs` and its test's doc comment;
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
  `trigger-met`: not taken up; the work moves when a transcript is committed, not when the claim
  check is read;
- `issue@transcript@an-option-anonymizes-what-transcript-prints`, `worsens`: its "Why it matters"
  rewritten, the removal step being now the one guard before the push;
- `design@agent-skills@search-runs-once-the-shapes-settle`, `constrains`: it raised the second
  search inside one discussion, answered in `thread@transcripts-per-discussion@search-files-per-discussion`.

## Criteria

### The owner's words are kept verbatim where the owner argues a design, and only there `##verbatim-where-design-is-argued`

Binding, stated by the owner in round 1, per `argument@transcripts-per-discussion@arg-1`. Satisfied
by `thread@transcripts-per-discussion@transcript-span` and
`thread@transcripts-per-discussion@transcript-on-reopening`.

### Several discussions of one plan document are kept apart `##discussions-kept-apart`

Binding, stated by the owner in round 2, per `argument@transcripts-per-discussion@arg-10`.
Satisfied by `thread@transcripts-per-discussion@transcript-files-per-discussion` and
`thread@transcripts-per-discussion@search-files-per-discussion`.

### The project's owner decides every action that cannot be undone `##owner-decides-what-cannot-be-undone`

Binding, from `goal@the-owner-decides`. Satisfied on the owner's reading of round 3, which
`thread@transcripts-per-discussion@push-reading-in-goal` writes into the goal. It replaced the
criterion of round 1, that nothing is published without the owner's word for that transcript,
which the session had derived from the goal before the owner gave that reading.

### A session follows the rule without the owner's help `##followed-without-the-owner`

Weighed, the session's. Satisfied: the end point is the checkpoint table, which the session showed
in its own conversation, and the file names are numbers the session counts.

## New names, in one place

```text
transcripts/<n>.txt              a file of a plan document's directory, the trimmed transcript of its n-th design discussion
search/discussion-<n>.txt        a file of a plan document's directory, the generated lines of the n-th discussion's record search
search/design-audit-<n>.txt      a file of a plan document's directory, the fit lines of its n-th design audit
TRANSCRIPTS_DIR                  crates/model/src/manifest.rs, the constant "transcripts", replacing TRANSCRIPT_FILE
SEARCH_OCCASIONS                 crates/model/src/plan.rs, ["discussion", "design-audit"], replacing SEARCH_FILES
is_transcript_file               crates/model/src/plan.rs
is_numbered_dir                  crates/model/src/plan.rs, names search/ and transcripts/
is_numbered_txt                  crates/model/src/plan.rs, private
is_search_file                   crates/model/src/plan.rs, existing, now admits <occasion>-<n>.txt
```

Two new design heads, in the agent-skills design home's plan-documents.md:
`design@agent-skills@transcript-removal-step` and
`design@agent-skills@transcript-pushed-with-the-branch`.

## Decided design

### Where a committed transcript ends, and when it is committed

For `thread@transcripts-per-discussion@transcript-span` and
`thread@transcripts-per-discussion@transcript-commit-timing`.

- The committed transcript of a design discussion is, for each session the discussion spans in
  order, the output of `cargo klarch transcript show <session> --from "<opening words>"`, without
  `--tools`, from the message where the discussion begins to the owner's word on the last
  checkpoint table the discussion showed before the commit that adds or rewrites the plan
  document: the one the design skill shows before the hand-off to planning, or, where a plan
  review reopens a thread before the plan document's branch merges, the one that follows the
  reopening, the same discussion's file then extracted again.
- `cargo klarch transcript show` has no option that ends its output, so the last session's output
  is cut after that message of the owner.
- It is committed in the commit that adds or rewrites the plan document, after its removal step.
- A ruling given after the last checkpoint is in no transcript: its attribution in the items is its
  whole record.

The argument: a design discussion is where the owner says what is worth keeping verbatim; what
follows the checkpoint, the assembly, the plan reviews and, for a spec built on its plan
document's branch, the implementation, holds approvals of defaults and requests for detail, which
an attribution records whole, an approval being no ground. The nearest rival, the incumbent, ended
at the owner's last message before the branch merges: it adds those approvals, for a spec built on
its branch the whole implementation, and can be committed only after the review before the merge;
its reason, that the rulings given at the reviews are in it, was the session's decision, never the
owner's, and those rulings were approvals of defaults (`argument@transcripts-per-discussion@arg-3`,
`argument@transcripts-per-discussion@arg-4`). The rival ending at the plan document's commit adds
only the assembly and the plan-review defaults, and has no boundary the session sees in its
conversation.

### Which discussions commit a transcript, and where it is filed

For `thread@transcripts-per-discussion@transcript-on-reopening`.

- A transcript is committed only for a design discussion that reaches a convergence checkpoint on
  the full path: the one that produced a spec or a milestone, or a later one, such as a design
  session opened at a slice's or a spec's design audit, or a rediscussion of a milestone before a
  slice's turn.
- A discussion the harness kept no transcript of, an implementation, a design audit that reopens
  nothing, and a material finding settled on one word of the owner commit none.
- A transcript is filed beside the `README.md` of the document its discussion produced or
  rewrites: a discussion that rewrites one slice is filed under that slice; one that rewrites
  several slices or the milestone's `README.md`, under the milestone.

The nearest rival, an excerpt for every reopened thread, brings back the approvals the owner wants
left out and has no clear end.

### The numbered files of a plan document's directory

For `thread@transcripts-per-discussion@transcript-files-per-discussion` and
`thread@transcripts-per-discussion@search-files-per-discussion`.

- A plan document's directory holds `transcripts/`, one file per design discussion,
  `transcripts/<n>.txt`, `<n>` the next number of that `transcripts/`, from 1, taken when the
  transcript is committed, filed as "Which discussions commit a transcript, and where it is filed"
  says. One discussion spanning several sessions stays one file.
- `search/` holds `search/discussion-<n>.txt`, `<n>` the number of the discussion's transcript, or,
  for a discussion that commits none, the next number that no transcript and no discussion's
  search file of that document takes; and `search/design-audit-<n>.txt`, numbered from 1 per design
  audit.
- A second search inside one discussion, for a thread opened after the first, appends its generated
  lines to that discussion's file, after a line naming that search.
- `<n>` is decimal digits with no leading zero, so each number has one name. The checker refuses
  any other name in `search/` or `transcripts/`, a directory included, as one phase-2 finding at
  that path, and refuses the single-file names. A harvested slice, judged by no rule, may still hold
  the single-file names.

The rivals: a date in the name, which two discussions held on one day would share, while git
records the date (the owner's argument, `argument@transcripts-per-discussion@arg-18`); the bare
`transcript.txt` kept for the first discussion, which avoids a migration at the cost of two names
for one kind of file; searches numbered on their own, which breaks the shared number of a
discussion's search file and its transcript.

### No word of the owner before the push; the removal step stays

For `thread@transcripts-per-discussion@transcript-push-approval`.

- The commit that holds a transcript is pushed as any commit of the branch is, with no word of the
  owner for it; the removal step stays, and is the one guard before the push.
- A project that wants the word states it in its root `CLAUDE.md`.
- A publication on another repository is not reached.

The argument: `goal@the-owner-decides` states that a push to the project's repository, whatever
it holds, is no action that cannot be undone the owner decides; and the workflow already pushes,
with no word of the owner, commit messages, plan items and issue entries that sessions write, each
exposed to the same risk (`argument@transcripts-per-discussion@arg-16`). The nearest rival kept the
word as the installed default, waivable in a project's root `CLAUDE.md`, at one approval round per
transcript in every project that does not waive it, against a risk every text pushed with no word
of the owner shares; a project whose owner wants the word states it, so each project's owner
decides for that project. That a publication on another repository is not reached was the
session's answer, which the owner approved.

### The goal's sentence on a push

For `thread@transcripts-per-discussion@push-reading-in-goal`. One sentence at the end of the second
paragraph of `goal@the-owner-decides`, as the owner approved it at the review before the merge:

> A push to the project's repository is not among the actions that cannot be undone the owner
> decides, whatever it holds and whoever it notifies; an action the push sets off stays the owner's
> to decide where it cannot be undone, beyond notifying what the push holds.

Commit 3 wrote an earlier wording, which the owner approved in round 6: "… whatever it holds. An
action the push sets off is judged on its own: where it cannot be undone and the push is its only
gate, such as a publish to a package registry, the push waits for the owner's word." The goal
reviewer's merge mode found "gate" read two ways, since the project uses the word for a check; two
further draft-mode rounds found that each condition written on what a push sets off opened a new
reading, so the sentence now leaves what a push sets off to the paragraph's opening clause. The
owner approved the session's defaults on each case those rounds raised: a publish gated only by a
reviewer who is not the owner, or by checks, is not the owner's decision; a standing word of the
owner deciding a kind of action is admitted; a notification only spreads what the push holds. The
last round's two edge cases, a changelog posted to a public list from a private repository and a
release whose notes the host derives, were shown to the owner beside the wording and left to a
project's design.

The rival, the reading recorded in the reversed head alone, would leave a session that weighs
another push without it. The sentence was drafted under `skill@knowledge-architect-goal-setting`,
read in two rounds by the goal reviewer's draft mode, and ruled by the owner: F4 reached and F5
judged on its own, by the session's defaults the owner's word on the table took; F6 reached on the
owner's argument (`argument@transcripts-per-discussion@arg-31`); N1 to N5 on the session's
defaults (`argument@transcripts-per-discussion@arg-34`).

## Mapping tables

The single-file names to the numbered names, over every file a plan document's directory admitted
before the work but its `README.md` and a milestone's `index.md`, which are unchanged:

| before | after |
| --- | --- |
| `transcript.txt` | `transcripts/1.txt` |
| `search/discussion.txt` | `search/discussion-1.txt` |
| `search/design-audit.txt` | `search/design-audit-1.txt` |
| `deliberation.md` | `deliberation.md`, unchanged |

## Losing alternatives

- Under `thread@transcripts-per-discussion@transcript-span`: shape A, the incumbent, ending at the
  owner's last message before the merge; shape C, ending at the plan document's commit. Lost to
  shape B on `argument@transcripts-per-discussion@arg-3`, `argument@transcripts-per-discussion@arg-4`
  and `argument@transcripts-per-discussion@arg-6`.
- Under `thread@transcripts-per-discussion@transcript-commit-timing`: a commit of its own before the
  first push, which gains nothing.
- Under `thread@transcripts-per-discussion@transcript-on-reopening`: an excerpt for every reopened
  thread, which brings back approvals and has no clear end.
- Under `thread@transcripts-per-discussion@transcript-files-per-discussion`: a date in the name,
  rejected by the owner (`argument@transcripts-per-discussion@arg-18`); the bare `transcript.txt`
  kept for the first discussion; numbered flat files at the top, `transcript-2.txt`, which the
  session had set aside for losing the date.
- Under `thread@transcripts-per-discussion@search-files-per-discussion`: a date in the name, as
  above; searches numbered on their own.
- Under `thread@transcripts-per-discussion@transcript-push-approval`: the owner's word before each
  transcript's push, kept as the installed default and waivable by a project.
- Under `thread@transcripts-per-discussion@push-reading-in-goal`: the reading recorded in the
  reversed head alone; the first wording, "Publishing a text to the project's shared history",
  repaired on F1 to F3; the second, "A push that adds commits", which the owner's ruling on F6
  replaced.

None was written to a `rejected` home. A date in the name and the bare `transcript.txt` kept would
each change the plans layout, a file format, so the first test of
`skill@knowledge-architect-decision-recording@losing-alternatives` admits them by its letter; they
are left without one under the narrower reading of that test, since
`design@model@plan-document-directory` states both refutations, and the case is a further instance
of
`issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head`.
The others meet no test.

## Readings

None: the work reads no external specification.

## Premortem

| # | cause | thread | verdict |
| --- | --- | --- | --- |
| P1 | a session cuts the transcript at the wrong message, unable to find the last checkpoint | `thread@transcripts-per-discussion@transcript-span` | a claim of step 2, repaired in the first review repair: the planning skill names the end message as the owner's word on the last checkpoint table the discussion showed before the commit that adds or rewrites the plan document, and says that the output is cut after it |
| P2 | private data reaches a public repository with no person reading the file before the push | `thread@transcripts-per-discussion@transcript-push-approval` | no tripwire, condition 1: a script could compute it, and `issue@transcript@an-option-anonymizes-what-transcript-prints` is that script |
| P3 | a ruling after the checkpoint is attributed wrongly, with no verbatim copy to check it against | `thread@transcripts-per-discussion@transcript-span` | watched by `issue@agent-skills@attribution-slips-are-not-counted-against-the-regime-of-committed-transcripts`; the words that leave the transcript are approvals, so the ground on which `rejected@agent-skills@owner-words-verbatim-in-plan-items` lost still holds for argued words |
| P4 | two branches both add `transcripts/2.txt` to one plan document | `thread@transcripts-per-discussion@transcript-files-per-discussion` | dropped: a rebase conflict reports it |

## Acceptance criteria

None: the discussion admitted no acceptance criterion, per `argument@transcripts-per-discussion@arg-30`.

## Implementation sequence

The steps as the work built them, each in its commit, listed by subject under "Status and
audience".

1. **The checker admits the numbered names and refuses the single-file names**, commit 1.
   `TRANSCRIPT_FILE` gives way to `TRANSCRIPTS_DIR` in `path@model@src/manifest.rs`; in
   `path@model@src/plan.rs`, `SEARCH_FILES` gives way to `SEARCH_OCCASIONS`, `is_search_file`
   admits `<occasion>-<n>.txt`, `is_transcript_file` admits `<n>.txt`, `is_numbered_dir` names
   `search/` and `transcripts/`, which `slices` no longer reads as slices; `plan_directory` and
   `unheaded_plan` in `path@model@src/tree.rs` judge `transcripts/` as `search/`; the repair of a
   refused citation of a deleted spec's file, `plan_document` in `path@model@src/resolver.rs`,
   names the spec for a numbered file; `path@check@src/check/registers.rs` reads no `transcripts/`
   as a slice; the `dirhome` mock holds the numbered names, its harvested slice keeping the
   single-file names; the Migration entry of the `Next release` section of CHANGELOG.md that
   announces the plans layout states the numbered names. Claims, each with its test, shown to fail under the mutations commit 1's
   message records:
   - the numbered names are admitted and nothing else:
     `plan::tests::the_numbered_files_of_a_plan_document_have_one_name_each`;
   - a milestone's `search/` and `transcripts/` are no slices:
     `plan::tests::a_slice_is_live_or_harvested_by_where_its_directory_sits`;
   - each refused name, malformed number and single-file name is one phase-2 finding, and a
     milestone and a slice holding every numbered name pass:
     `tree::plans_tests::each_odd_shape_of_the_plans_layout_is_one_phase_two_finding`;
   - the repair names the deleted spec for its numbered files, and nothing for a single-file name:
     `resolver::tests::the_p1_repair_names_the_form_for_each_position`, whose `p1` is no cause of
     this spec's premortem;
   - a conformant tree with the numbered names passes every check: the `dirhome` mock, a regression
     guard.

   Fails alone on: a numbered name refused, or a single-file name admitted.
2. **The installed text and the design record**, commit 2. At the installed text's source under
   `path@agent-skills@content/`, reinstalled with `cargo klarch install-agent-skills`: the planning
   skill, the review skill, the design skill, the primer's row of plan documents, the head-ground
   fragment and the transcript reviewer, as "What is already decided" lists, and the
   design-conformance reviewer, which names the search files as
   `design@agent-skills@verdicts-reach-the-conformance-review` does; the design record as listed
   there; the two issues named there; the Workflow entry of the `Next release` section of
   CHANGELOG.md that announces the committed transcript, edited, with no entry of its own. Claims:
   the premortem's P1, repaired in the first review repair, read in the installed copy of the
   planning skill's assembly bullets; no installed text keeps the owner's word before a
   transcript's push, by the reversal check's search; `cargo klarch check` passes. Fails alone on: a
   text still naming a single-file name or the word before the push.
3. **The goal's sentence**, commit 3, under `skill@knowledge-architect-goal-setting`, on the
   owner's ruling of round 6. Claim: `goal@the-owner-decides` holds the two sentences of "The goal's
   sentence on a push" verbatim. Fails alone on: a goal sentence that differs from the ruled one.

Commit 4, the issue, carries no step of this spec: it records, on the owner's request, the path
choice that made this spec late.

## Order rationale

- Step 1 before step 2: step 1 changes what the checker requires of every tree, so it goes in the
  branch's first commit, per `primer@branch-commits`, and step 2's text names files the checker
  admits only after it.
- Step 2 before step 3: the goal's wording was ruled after the implementer of steps 1 and 2 had
  been dispatched, and the head of step 2 cites the goal, whose sentence step 3 then adds.

## Defaults awaiting the owner

None. Every default was ruled: those of the checkpoint table by the owner's word in round 5, those
on the goal's first wording in round 6, and, at the review before the merge, the goal's present
wording with the session's defaults on the cases its review rounds raised, the last being the
session's default that the goal leaves the two edge cases of its last round, a changelog posted to
a public list from a private repository and a release whose notes the host derives, to the design
of a project where they arise, which the owner approved. A standing word of the owner deciding a
kind of action is admitted by the goal, not stated, and left to a design head.

The filing rule of a later discussion's transcript, under the slice it rewrites or under the
milestone where it rewrites several slices or the milestone's `README.md`, was the session's
proposal, which the owner approved with the general shape of numbered files; no rival of it was
argued, so `design@agent-skills@committed-transcript` states it with no argument of its own.

## Harvest

Landed: the harvest was written with the work, in commit 2, the checker's head in commit 1 and the
goal in commit 3.

| item | home |
| --- | --- |
| `thread@transcripts-per-discussion@transcript-span` | `design@agent-skills@committed-transcript`, rewritten in place |
| `thread@transcripts-per-discussion@transcript-commit-timing` | `design@agent-skills@committed-transcript` |
| `thread@transcripts-per-discussion@transcript-on-reopening` | `design@agent-skills@committed-transcript` |
| `thread@transcripts-per-discussion@transcript-files-per-discussion` | `design@agent-skills@committed-transcript`, and `design@model@plan-document-directory`, commit 1, which holds the numbering, its rivals and its reason since the review before the merge |
| `thread@transcripts-per-discussion@search-files-per-discussion` | `design@agent-skills@plan-commits-the-generated-lines`, `design@model@plan-document-directory`, and the planning skill's search bullet, with its reason and rival in a comment of the skill's source |
| `thread@transcripts-per-discussion@transcript-push-approval` | `transcript-pushed-on-the-owner-s-word` reversed into `design@agent-skills@transcript-pushed-with-the-branch`; the removal step split out to `design@agent-skills@transcript-removal-step` |
| `thread@transcripts-per-discussion@push-reading-in-goal` | `goal@the-owner-decides`, commit 3 |
| the touched heads and `design@agent-skills@attribution-replaces-quotation` | rewritten in place, as "What is already decided" lists |
| every item of "Losing alternatives" | no `rejected` entry, as that section says |
| the premortem's causes | no tripwire, as "Premortem" says |
| this spec | deleted in a commit of its own after the review before the merge, cited as `spec@plans@transcripts-per-discussion` |

## Later consequences

- The discussion of `issue@agent-skills@sessions-misread-which-course-a-design-discussion-takes`
  may change when a design discussion takes the full path, which decides when a transcript is
  committed at all.
- `issue@transcript@an-option-anonymizes-what-transcript-prints` would replace part of the removal
  step, now the one guard before a transcript's push.
- The next release announces the numbered names in the entries of the `Next release` section of
  CHANGELOG.md that announce the plans layout, which no released version holds, so no project
  renames a single-file name.
