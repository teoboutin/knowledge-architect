# The threads and arguments of the spec on the committed transcripts of design discussions

## Threads

### The committed transcript ends at the owner's word on the last convergence checkpoint `##transcript-span`

Proposed by the session in round 1, as shape B, from the end point the owner described in round 1
(`argument@transcripts-per-discussion@arg-2`). Final state: approved. Arguments:
`argument@transcripts-per-discussion@arg-1`, `argument@transcripts-per-discussion@arg-2`,
`argument@transcripts-per-discussion@arg-3`, `argument@transcripts-per-discussion@arg-4`,
`argument@transcripts-per-discussion@arg-6`, `argument@transcripts-per-discussion@arg-13`,
`argument@transcripts-per-discussion@arg-29`. Shape: Decided design, "Where a committed transcript
ends, and when it is committed". Harvest: `design@agent-skills@committed-transcript`, rewritten in
place. The owner agreed to shape B in round 2. The session's default of round 2, that a plan review
reopening a thread before the plan document's branch merges extends the same discussion's file to
the new checkpoint, entered the checkpoint table shown at the end of round 4, which the owner
approved in round 5.

### The transcript is committed in the commit that adds or rewrites the plan document `##transcript-commit-timing`

Proposed by the session in round 1. Final state: approved. Arguments:
`argument@transcripts-per-discussion@arg-2`, `argument@transcripts-per-discussion@arg-4`,
`argument@transcripts-per-discussion@arg-7`. Shape: Decided design, "Where a committed transcript
ends, and when it is committed". Harvest: `design@agent-skills@committed-transcript`. The owner
agreed in round 2.

### A transcript is committed only for a design discussion that reaches a convergence checkpoint on the full path `##transcript-on-reopening`

Proposed by the session in round 1; the owner added in round 2 that a slice owes a transcript only
where a design discussion was held for it. Final state: approved. Arguments:
`argument@transcripts-per-discussion@arg-1`, `argument@transcripts-per-discussion@arg-5`,
`argument@transcripts-per-discussion@arg-8`, `argument@transcripts-per-discussion@arg-9`. Shape:
Decided design, "Which discussions commit a transcript, and where it is filed". Harvest:
`design@agent-skills@committed-transcript`. The owner agreed in round 2, with the addition on
slices.

### One transcript file per design discussion, numbered from 1 `##transcript-files-per-discussion`

Opened by the owner's requirement of round 2 (`argument@transcripts-per-discussion@arg-10`); the
shape, a `transcripts/` directory, was the session's proposal of round 2, with files named by date.
Final state: approved, with numbers in place of dates. Arguments:
`argument@transcripts-per-discussion@arg-10`, `argument@transcripts-per-discussion@arg-14`,
`argument@transcripts-per-discussion@arg-18`, `argument@transcripts-per-discussion@arg-23`. Shape:
Decided design, "The numbered files of a plan document's directory". Harvest:
`design@model@plan-document-directory` and `design@agent-skills@committed-transcript`. The owner
agreed to the general shape in round 3 and rejected the dates, choosing numbers on the argument
of `argument@transcripts-per-discussion@arg-18`.

### The record search's generated lines take the discussion's number, and a design audit's fit lines a number of their own `##search-files-per-discussion`

Proposed by the session in round 2, as the same defect in `search/`. Final state: approved. The
owner confirmed the defect in round 3; the names `search/discussion-<n>.txt` and
`search/design-audit-<n>.txt` were the session's default of round 3, and the default that a second
search inside one discussion appends to that discussion's file was raised by the record search and
put in the checkpoint table, both approved by the owner's word on the table in round 5. Arguments:
`argument@transcripts-per-discussion@arg-15`, `argument@transcripts-per-discussion@arg-19`,
`argument@transcripts-per-discussion@arg-21`, `argument@transcripts-per-discussion@arg-29`. Shape:
Decided design, "The numbered files of a plan document's directory". Harvest:
`design@model@plan-document-directory`, which states the numbering of the files of `search/`,
`design@agent-skills@plan-commits-the-generated-lines`, and the planning skill's search bullet,
which states the shared number and the append, with their reason and rival in a comment of the
skill's source.

### A committed transcript is pushed with its branch, with no word of the owner for it `##transcript-push-approval`

Opened by the owner in round 2, who said the per-transcript approval held little value and would
consider removing it (`argument@transcripts-per-discussion@arg-11`,
`argument@transcripts-per-discussion@arg-12`); the session proposed the removal in round 2 and put
the reading of `goal@the-owner-decides` to the owner. Final state: approved. Arguments:
`argument@transcripts-per-discussion@arg-11`, `argument@transcripts-per-discussion@arg-12`,
`argument@transcripts-per-discussion@arg-16`, `argument@transcripts-per-discussion@arg-17`,
`argument@transcripts-per-discussion@arg-20`, `argument@transcripts-per-discussion@arg-28`,
`argument@transcripts-per-discussion@arg-29`. Shape: Decided design, "No word of the owner before
the push; the removal step stays". Harvest: `transcript-pushed-on-the-owner-s-word` reversed, its
successor `design@agent-skills@transcript-pushed-with-the-branch`; the removal step split out to
`design@agent-skills@transcript-removal-step`. The owner ruled the removal in round 3, on the
owner's reading of `goal@the-owner-decides` that a push is no action that cannot be undone. That
the reading does not reach a publication on another repository was the session's answer to two
verdicts of the record search, in the checkpoint table the owner approved in round 5.

### The owner's reading of a push is written into the goal `##push-reading-in-goal`

Proposed by the session in round 3. Final state: approved. Arguments:
`argument@transcripts-per-discussion@arg-20`, `argument@transcripts-per-discussion@arg-22`,
`argument@transcripts-per-discussion@arg-24`, `argument@transcripts-per-discussion@arg-25`,
`argument@transcripts-per-discussion@arg-26`, `argument@transcripts-per-discussion@arg-27`,
`argument@transcripts-per-discussion@arg-31`, `argument@transcripts-per-discussion@arg-32`,
`argument@transcripts-per-discussion@arg-33`, `argument@transcripts-per-discussion@arg-34`. Shape:
Decided design, "The goal's sentence on a push". Harvest: `goal@the-owner-decides`, under
`skill@knowledge-architect-goal-setting`. The owner agreed to drafting a sentence in round 4. In
round 5 the owner ruled F6 reached, against the session's default, and the session read the
owner's word on the table as taking its defaults on F4 and F5. In round 6 the owner approved the
wording reviewed in the goal reviewer's second round, with the session's default on each of N1 to
N5. These rulings came after the last convergence checkpoint, so the transcript does not hold
round 6; this item and the message of the goal's commit are their record.

## Arguments

### The owner's account of the feature's purpose: a design discussion is where the owner says much `##arg-1`

Round 1, the owner. Bears on `thread@transcripts-per-discussion@transcript-span` and
`thread@transcripts-per-discussion@transcript-on-reopening`. The owner said the feature had
deviated from the request: it was never meant to require a transcript up to the end of the
session, which only the owner could reach by doing the merge alone. Its point was to keep the
transcript only where there is a design discussion, since only there does the owner say much;
elsewhere, the implementation of a slice and a design audit included, the owner mostly approves
defaults or asks for detail, of little historical value, unless a critical finding reopens the
discussion.

### The owner's intended end point: the owner's approval of the checkpoint table `##arg-2`

Round 1, the owner. Bears on `thread@transcripts-per-discussion@transcript-span` and
`thread@transcripts-per-discussion@transcript-commit-timing`. The owner described the intended
feature as the verbatim of the design discussion, probably up to the owner's approval of the
checkpoint table at its end and no further, extracted and committed with the plan document written
right after that point.

### The extension to the merge was the session's decision, never the owner's `##arg-3`

Round 1, the session. Bears on `thread@transcripts-per-discussion@transcript-span`. "The
milestone's design first ended the transcript **at the plan document's commit** (line 1710 of that
transcript). The extension to the merge was my own decision, not yours. The transcript review made
it, and it was listed under \"Decided\" with this reason: > Otherwise this milestone's transcript
would miss your rulings on D1 and Q10 to Q13." / "Those rulings were approvals of defaults [...]
Under `primer@design-heads` an approval of a default is no ground: it is attributed, and its
wording adds nothing. So the reason behind the extension is weak, and you were never asked about
the extension itself."

### What the incumbent span cost on the last spec `##arg-4`

Round 1, the session. Bears on `thread@transcripts-per-discussion@transcript-span` and
`thread@transcripts-per-discussion@transcript-commit-timing`. "The `doc-gate-and-lints` spec was
designed and built on the same branch. So \"the last message before the merge\" covered the whole
implementation and both reviews. The transcript was 1953 lines. It needed a commit of its own after
the review before the merge (ee2e3427), and that commit had to include your approval of the
transcript itself. The next commit (773f3b74) deleted it along with the spec. A milestone does not
hit this case, because its plan document merges before slice 1 starts."

### Only the full path commits a transcript `##arg-5`

Round 1, the session. Bears on `thread@transcripts-per-discussion@transcript-on-reopening`. "Only
the full path commits a transcript. The in-change path and bounded work commit none. So the problem
is where the transcript ends and when it is committed. The rule on which discussions get a
transcript is already right."

### Shape B against shapes A and C, and why the last checkpoint `##arg-6`

Round 1, the session. Bears on `thread@transcripts-per-discussion@transcript-span`. "**Against A:**
what it adds beyond B is approvals of defaults and, for a spec, the implementation. Neither needs
your words verbatim. Attribution in the items and in commit messages already records them." /
"**Against C:** between the checkpoint and the commit there is only assembly and the plan reviews.
Their findings come back as defaults, so C adds nothing over B. It also has no clear boundary to
cut at." / "**Why \"the last\" checkpoint matters:** if a plan review raises a material finding,
the thread reopens. The design skill then proposes convergence again with a new checkpoint. That
second discussion is real design talk, and B includes it without any special rule." The session
also weighed the criterion that a session can follow the rule without the owner's help: "B: the
checkpoint is a displayed table in the transcript".

### The transcript goes in the plan document's commit `##arg-7`

Round 1, the session. Bears on `thread@transcripts-per-discussion@transcript-commit-timing`. "It
goes into the plan document's commit, scrubbed first." / "The transcript no longer has to include
your approval of itself." / "Rival: commit it separately, before the first push. That gains
nothing, so I propose the same commit."

### A reopened discussion gets its own transcript, a single finding none `##arg-8`

Round 1, the session. Bears on `thread@transcripts-per-discussion@transcript-on-reopening`. "a
design session opened at a design audit already takes the full path. It gets its own excerpt, from
its opening message to your word on its checkpoint." / "A single material finding that one word of
yours settles gets no excerpt. It is attributed like any other ruling." / "Nearest rival: an
excerpt for every reopened thread. It brings back the \"only approvals\" content you want left out,
and its end point is unclear." / "the current text is ambiguous about slices. A slice directory is
laid out like a spec, and it owes `transcript.txt` \"where the document was assembled from a
transcript\". Read as it stands, that would carry a slice's implementation into its transcript up
to its merge. This thread closes that reading."

### A slice owes a transcript only where a design discussion was held for it `##arg-9`

Round 2, the owner. Bears on `thread@transcripts-per-discussion@transcript-on-reopening`. The owner
added, with the agreement, that a slice owes a transcript only when a design discussion was held.

### One plan document can have several design discussions `##arg-10`

Round 2, the owner. Bears on `thread@transcripts-per-discussion@transcript-files-per-discussion`.
The owner said a slice or a milestone may go through several audits and discussions, for example
one designed far in advance and discussed again before its turn, and asked that the shape and the
instructions make clear that several transcripts can be committed, in separate files.

### The earlier regime pushed the owner's verbatim words with no approval `##arg-11`

Round 2, the owner. Bears on `thread@transcripts-per-discussion@transcript-push-approval`. The owner
said that before the committed transcript, the workflow put the owner's verbatim words in specs and
milestones, and paths of the owner's home directory such as the path of a transcript in the
harness's cache, without asking for the owner's word before the push; the owner disliked it, but
not to the point of requiring an approval.

### A pushed transcript can be removed, and a token in it has already leaked `##arg-12`

Round 2, the owner. Bears on `thread@transcripts-per-discussion@transcript-push-approval`. The owner
said it is not true that a pushed transcript cannot be taken back: it can be removed from the
long-term history, though nothing ensures that no information leaked; that a secret token or key in
a transcript has already leaked by being there; and that the owner's explicit approval at this step
holds little value, which the owner would consider removing.

### A plan review that reopens a thread extends the same discussion's file `##arg-13`

Round 2, the session. Bears on `thread@transcripts-per-discussion@transcript-span`. "The plan
reviews come after the plan document's commit. If one of them reopens a thread, a new checkpoint
follows. My default: while the plan document's branch is not merged, the same discussion goes on,
so the session regenerates that discussion's file up to the new checkpoint. A discussion after the
merge gets a new file, per the next thread."

### Several discussions apart take a checker change, a directory of files, and a migration `##arg-14`

Round 2, the session. Bears on `thread@transcripts-per-discussion@transcript-files-per-discussion`.
"Today the checker allows exactly one `transcript.txt` per plan document
([a line of `path@model@src/manifest.rs` and one of `path@model@src/tree.rs`]). So keeping several
discussions apart takes a checker change, not only a change to the skill text." The proposal named
each file by the date of its discussion's opening message, a second one on a date taking a suffix;
"One discussion that spans several sessions stays one file, with its sessions in order, as today."
/ "A discussion that rewrites several slices, or the milestone's `README.md`, is filed under the
milestone. One that rewrites only one slice is filed under that slice." Rivals: "Numbered flat
files at the top (`transcript-2.txt`): these are ordered, but a reader cannot tell when each
discussion took place." / "Keeping the bare `transcript.txt` for the first discussion: this avoids
a migration. The cost is two names for one kind of file, and the checker has to accept both." /
"A project that already holds a `transcript.txt` must rename it, which takes a Migration entry in
the changelog."

### `search/` holds one file per occasion, which a second occasion overwrites `##arg-15`

Round 2, the session. Bears on `thread@transcripts-per-discussion@search-files-per-discussion`.
"`search/` makes the same single-occasion assumption. [a line of `path@model@src/plan.rs`] names exactly
`discussion.txt` and `design-audit.txt`, so a second discussion or a second audit would overwrite
the first one's generated lines." / "It costs little to do here, and it is the same question."

### The owner's arguments on the push, checked `##arg-16`

Round 2, the session. Bears on `thread@transcripts-per-discussion@transcript-push-approval`. "This
is true of a pushed branch, and of main by a rewrite, even though this repository forbids rewriting
main. Copies others already fetched cannot be recalled. So the premise is partly false: the file
can be removed, a leak cannot." / "It has reached the model provider. A public push widens the
leak, and the removal step already strips credentials." / "the workflow already pushes commit
messages, plan items and issue entries written by sessions, with no approval of each push. A
session could paste a token into any of them. So the approval guards one file among many pushed
texts, against a risk all of them share."

### Remove the approval, keep the removal step; the choice turns on a reading of the goal `##arg-17`

Round 2, the session. Bears on `thread@transcripts-per-discussion@transcript-push-approval`. "Keep
the removal step: home paths, e-mail addresses, credentials, and pasted material you mark." / "A
project that wants approval adds it in its root `CLAUDE.md`." / "the removal step becomes the only
guard. That makes `issue@transcript@an-option-anonymizes-what-transcript-prints` [...] worth
more." Nearest rival: "Keep approval as the installed default, and let a project waive it once in
its root `CLAUDE.md`, as this one would. It keeps every adopting owner's decision explicit under
`goal@the-owner-decides`. It costs every project that does not opt out one approval round per
transcript." / "If publishing a scrubbed transcript is \"an action that cannot be undone\", the
rival is required. If it is no different from any other pushed text, as argument 3 shows, the
proposal stands. That is a reading of `goal@the-owner-decides` only you can give." "Argument 3" is
point 3 of the same message, quoted last in `argument@transcripts-per-discussion@arg-16`.

### Numbers, not dates `##arg-18`

Round 3, the owner. Bears on `thread@transcripts-per-discussion@transcript-files-per-discussion`.
The owner said dates do not fit, since two discussions cannot be expected to fall on separate days
under agent-driven development; that the date is of little use and git history usually gives it;
and that plain numbers have none of these problems and lose little.

### The same defect in `search/` `##arg-19`

Round 3, the owner. Bears on `thread@transcripts-per-discussion@search-files-per-discussion`. The
owner agreed that `search/` has the same defect.

### The owner does not rank a push as an action that cannot be undone `##arg-20`

Round 3, the owner. Bears on `thread@transcripts-per-discussion@transcript-push-approval` and
`thread@transcripts-per-discussion@push-reading-in-goal`. The owner ruled the approval removed and
said that, for `goal@the-owner-decides`, the owner does not rank a push as an action that cannot be
undone, for the arguments given before; ranking it so would forbid every push, too great a
constraint, too far against the agents' autonomy.

### A discussion's search file takes the discussion's number `##arg-21`

Round 3, the session. Bears on `thread@transcripts-per-discussion@search-files-per-discussion`.
"`search/discussion-<n>.txt`. Its number is the discussion's number, so the search lines and the
transcript of discussion 2 are found together." / "`search/design-audit-<n>.txt`, numbered by
audit."

### A reading that holds across decisions is drafted as a goal `##arg-22`

Round 3, the session. Bears on `thread@transcripts-per-discussion@push-reading-in-goal`. "Your
reading covers every push the workflow makes, not only transcripts. So it is a scope of the goal,
and it holds across decisions. Under `primer@design-heads`, such an intent is drafted as a goal,
and a head does not hold it alone." / "**Rival:** record the reading only in the reversed head, as
your reading. A session that later weighs another push, such as an audit report, would not find it
there and could ask you again."

### The checker's file set changes, and the premortem runs `##arg-23`

Round 3, the session. Bears on `thread@transcripts-per-discussion@transcript-files-per-discussion`
and `thread@transcripts-per-discussion@search-files-per-discussion`. "**The checker's file set
changes**, in the `model` Component. A project that already holds a `transcript.txt`,
`search/discussion.txt` or `search/design-audit.txt` must rename it, so the release needs a
Migration entry. This repository holds none of these files today." / "**The premortem runs**
before convergence, since a later reversal would touch stored files." "None" holds of the
repository's own plan documents, as the session's round-2 message said of `transcript.txt`: the
`dirhome` mock held eight such files, which commit 1 renamed but for the two of its harvested
slice.

### The first wording of the goal's sentence avoids naming a git command `##arg-24`

Round 4, the session. Bears on `thread@transcripts-per-discussion@push-reading-in-goal`. The
wording: "Publishing a text to the project's shared history is no action that cannot be undone:
what it published can be removed from that history, and a copy someone took before is a risk every
published text shares." / "A goal must not name a git command, which is a mechanism, so I used
words that keep their meaning without one." / "As worded, it does not cover a release to
crates.io, which is a registry and not the history. A published version cannot be deleted there,
only yanked, so a release stays an action you decide."

### The goal reviewer's first round: no block, five questions on intent, one wording defect `##arg-25`

Round 4, the goal reviewer in its draft mode, as the session received it. Bears on
`thread@transcripts-per-discussion@push-reading-in-goal`. F1: "The words \"the project's shared
history\" can be read two ways", main's history or any ref pushed to the shared remote, and "\"A
text\" narrows \"a push\" the same way". F2: "The clause after the colon is an argument, not an
outcome", and on this repository's main, guarded by `design@knowledge-architect@main-is-guarded-by-a-ruleset`
and `design@knowledge-architect@git-flow`, "a text merged to main cannot be removed from that
history unless the ruleset changes first". F3: "The project uses \"publish\" for the release." F4:
"an agent commits and pushes a configuration file holding a live token that was never exposed
before." F5: "a push that starts a publish to a package registry or a deployment." F6: "a force
push that replaces or drops commits of the shared history", which
`design@knowledge-architect@no-operation-loses-content` guards against. It also named, as an
unverified lead, that GitHub may keep commits reachable after a force push.

### The clause after the colon goes, and the sentence says "a push" `##arg-26`

Round 4, the session. Bears on `thread@transcripts-per-discussion@push-reading-in-goal`. "It is an
argument, not an outcome, and in this repository it is false for main" / "A goal binds outright,
so a false premise inside one would push sessions in two directions: some would hold back from
main, others would argue to relax the ruleset." / "\"Publish\" already means the release here
[...] \"Shared history\" could also be read as main only, which would leave out the branch pushes
you ruled on. The word \"push\" keeps its meaning because `goal@serves-one-kind-of-project`
requires a git repository."

### Defaults on F4 to F6 `##arg-27`

Round 4, the session. Bears on `thread@transcripts-per-discussion@push-reading-in-goal`. F4:
"**Default: reached.** [...] The guard is the removal step, not your word." Rival: "exclude it.
That would bring back a judgement of what each push holds, which is the approval you just
removed." F5: "**Default: the push is cleared, and what it sets off is judged as its own action.**
A publish to a registry stays yours wherever it cannot be undone." F6: "Your arguments covered a
text that is added and could be removed later. A rewrite can destroy the only copy of a commit
[...] **Default: not reached.**"

### The issue on the anonymizing option rests on the defeated premise `##arg-28`

Round 4, the session, from the goal reviewer's report. Bears on
`thread@transcripts-per-discussion@transcript-push-approval`.
"`issue@transcript@an-option-anonymizes-what-transcript-prints` also rests on the premise this
change removes. [...] I'll rewrite that section in the same change: the removal step becomes the
only guard, which makes the option worth more."

### The record search: two reversals, nine renames, a second search, another repository `##arg-29`

Round 4, the record search's verdicts, as the session reported them. Bears on every thread of the
discussion. "It graded 167 entries as bearing on the work, and 19 of them call for an action. None
of them defeats a decided thread." `design@agent-skills@committed-transcript` "loses its reason
\"so that the rulings given at its reviews are in it\" to #transcript-span", and
`transcript-pushed-on-the-owner-s-word`, now `design@agent-skills@transcript-pushed-with-the-branch`,
"is reversed by #transcript-push-approval". Nine heads that name the single-file names are
rewritten: "Their rule stays the same: the set of files is still closed, and only the generated
lines are kept." On a second search inside one discussion, per
`design@agent-skills@search-runs-once-the-shapes-settle`: "**Default:** they are appended to the
same `search/discussion-<n>.txt`, after a header line." Rival: "number the searches separately.
The search file and the transcript of one discussion would then stop sharing a number." On
`design@agent-skills@installed-fix-goes-upstream` and `design@agent-skills@intake-outcome-authority`:
"a publication elsewhere stays yours. Both heads stand as written." On
`issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`:
`design@agent-skills@committed-transcript` "holds two decisions with separate rivals: what is committed, and its span;
the removal step, whose rival was to commit no transcript at all", with the default to split it.

### The premortem `##arg-30`

Round 4, the session. Bears on `thread@transcripts-per-discussion@transcript-span`,
`thread@transcripts-per-discussion@transcript-push-approval` and
`thread@transcripts-per-discussion@transcript-files-per-discussion`. Its four causes and their
outcomes are the section Premortem of the spec's `README.md`. "No acceptance criterion and no
tripwire."

### A push that removes commits is no worse `##arg-31`

Round 5, the owner. Bears on `thread@transcripts-per-discussion@push-reading-in-goal`. The owner
ruled that the sentence drops "that adds commits": a push that removes commits is no worse, its
commits most likely stay in a reflog in the vast majority of cases, and the owner does not want to
approve every force push in projects that require a linear history, where force pushes are common.

### F6 reached changes nothing for this repository's main `##arg-32`

Round 5, the session. Bears on `thread@transcripts-per-discussion@push-reading-in-goal`. "The
sentence only stops a force push from being an action that waits for you. It does not require force
pushes to be allowed. The ruleset that blocks force pushes on main,
`design@knowledge-architect@main-is-guarded-by-a-ruleset`, still stands."

### The goal reviewer's second round: no block, four questions on intent, one wording defect `##arg-33`

Round 5, the goal reviewer in its draft mode, as the session received it. Bears on
`thread@transcripts-per-discussion@push-reading-in-goal`. N1: "\"is no action that cannot be
undone\" reads two ways", a classification or a statement of fact. N2: "\"on any branch\" may leave
out tags and ref deletion." N3: "\"the project's repository\" is open." N4: "\"is judged on its
own\" is open where nothing gates what the push sets off." N5: "\"a release\" means a procedure in
this project." For information: a force push or a deletion of another contributor's branch that
the agent's clone never fetched leaves no reflog the agent holds; "The owner's hedge \"vast
majority\" suggests this minority was accepted knowingly, so it is not a finding."

### Defaults on N1 to N5 `##arg-34`

Round 5, the session. Bears on `thread@transcripts-per-discussion@push-reading-in-goal`. N1:
"**Default: A**, which your words \"nothing can make sure no information leaked\" support." N2:
"**Default: any push.** Wording: drop \"on any branch\"." N3: "**Default: the repositories the
project pushes its own work to**, its mirrors included. A fork owned by the agent's account is not
the project's." N4: "**Default: B.** Otherwise the second clause clears nothing in exactly the case
it exists for." N5: "**Default:** \"such as a publish to a package registry\"." / "The rounds stop
after round 2, with no repair", per the stop rule of
`skill@knowledge-architect-goal-setting@drawing-out-intent`.
