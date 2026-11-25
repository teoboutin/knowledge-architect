---
kind: design
---
# A deferred issue's trigger, or a re-entry point, brings unrelated work into a design's scope at a bad moment

## Summary

The owner reported that deferred issues often bring unrelated work into scope at bad moments. An
issue that waits for an event, a `deferred` trigger or a `design` entry's re-entry point, is met by
whatever work happens to cause the event, and the record search hands it to that work as a
question to rule before the work starts, whether or not the work is the right place for it.

## Details

### What

The owner's report, in the design discussion of
the milestone on split guidelines and plan directories, attributed with its hedge: in the owner's
view, these deferred issues are problematic, since they often bring unrelated work into scope at bad
moments.

The instance of that discussion: `issue@knowledge-architect@the-changelog-holds-references-no-check-reads`,
then a `design` entry, named as its re-entry point the next change that writes a Migration entry
citing a section of the installed text. The milestone's first slice writes one. The record search
graded the issue `trigger-met`, with the action `rule-before-work`, so the session put its design
discussion into the milestone, at the first slice's design audit. The owner moved it out: the issue
became a `todo`, with no re-entry point, to be designed with
`issue@knowledge-architect@a-mechanical-changelog-check`, when the changelog gets a model and checks
of its own.

The suspected mechanism, in one sentence: the trigger test of
`skill@knowledge-architect-issue-tracking@trigger-test` asks that a trigger name an occasion whose
own work includes the work the trigger names, and a Migration entry's work includes deciding how
that entry is written, not how every entry of the changelog cites what it cites, so the trigger
passed the test by its words while the occasion it named did not need the work. Not established:
whether the other instances the owner has in mind share this mechanism.

A second instance of the same discussion: `issue@agent-skills@a-record-search-misses-what-is-added-after-it`,
a `design` entry, named as its re-entry point the next design discussion that changes the record
search or the grounding's steps. The milestone changes the grounding's steps, so the record search
graded the issue `trigger-met`, and the session put to the owner whether reopening
`design@agent-skills@one-record-searcher` was part of the milestone. The owner kept it out, and the
issue's re-entry point was rewritten to name a discussion whose own work includes that reopening.

A third instance, from 2026-11-13-knowledge-architect-plan-directories-slice-1 P2: the re-entry
point of `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
"when work next changes what a session reads or runs at a commit", was graded `trigger-met` at the
audit of the first slice of the plan directories, which adds steps at three kinds of commit; the
session argued the slice was not the occasion, and the design-conformance reviewer argued it was.
Two readers applied the point in opposite directions, so it names no event they agree on. The
file proposed naming the event: a change to the commit step of an installed skill or to what
`cargo klarch commits` prints. It also shows a review reading a re-entry point, the question of
`issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`.

A further instance, at a slice's design audit: in the slice on splits planned before the work, the
audit graded an issue `trigger-met`, `rule-before-work`, its re-entry met by the slice's change to
one head. The planning skill's sorts at a design audit name no verdict action, so the
session had two readings, arguing the instance in the slice or ruling its re-entry. A harvest agent
took the first and closed the thread with no search of its own, against
`design@agent-skills@no-thread-closes-before-the-search`; the review before the merge withdrew the
argument and the re-entry was rewritten so that a change of a host list no longer meets it, per W4
of the retrospective file `2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow`.

Further instances, on the issues a split signal opens, whose re-entry point loop step 1 of the
design skill fixes as the design or the planning of the next work that changes that code, per
`design@agent-skills@split-issue-reenters-at-planning`:

- the port of the gates to Windows and macOS changed by one call at each site the code of seven of
  the eight past-one-read issues that file counts; the search graded each `trigger-met`,
  `rule-before-work`, the reviewers read each, a review reading a `design` issue's re-entry point
  against the premise `design@agent-skills@split-issue-reenters-at-planning` names, each gained an
  instance line saying no split there, and no split was made, per W4 of the retrospective file
  `2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow`, whose cause is that the
  re-entry point is met by any edit, so a port owes a weighing per file;
- the work on committed transcripts wrote one line each into three modules whose split issues it
  then reopened; the session reworded the three issues to defer the weighing, and the scout and two
  reviewers reported size signals on modules the work touched by a line, per the standing answer on
  code rules of the retrospective file
  `2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow`;
- `issue@testing@binary-rs-is-past-one-read` has been met by five works, the record search's saved
  workflow, the mutation survivors of three suites, the plan directories' numbered files, the
  cleanup of the temporary directory and the port of the gates, each of which weighed a split and
  made none, its instance lines say; the mutation survivors' session reported that the next branch
  adding a test meets the same occasion with the same argument, per P1 of the retrospective file
  `2026-11-16-knowledge-architect-mutation-survivors` and the standing answer on code rules of its
  workflow file. Three of that branch's reviewers reported the occasion of the entry's closing
  condition met, while the entry was still of kind `todo`.

A further instance, on the bounded path: W1 of
2026-11-25-knowledge-architect-intake-items-divided-klarch-workflow. The search graded an issue
`trigger-met`, `rule-before-work`, with an owner-question, to take a wider discussion now or leave
it. The design skill's action says "ruled before the work is designed further or built", while the
bounded path's one message builds its default and `primer@question-authority` builds a default put
to the owner and waits for the ruling before the merge; the session ruled the question itself,
built, and put it as contestable, and the owner answered after the build, before the merge. Two
reviewers could not tell from the record whether it was answered. The intake that received the
file reworded the action to name the authority `primer@question-authority` gives each question,
which says what waits; whether a met trigger is ruled before the work at all stays this entry's
question.

`issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` asks the opposite question,
whether more reviews should read a re-entry point against a change; a decision on either entry
bears on the other.

Re-entry point: a design discussion under `skill@knowledge-architect-design` on the owner's word,
which the owner holds, since the instances are the owner's to report.

### Why it matters

Work pulled into a design by an event it happens to cause is designed at the wrong moment, inside a
scope that was not built for it, or delays the design that met it, against
`goal@agents-work-without-drift`. It strains `design@agent-skills@standing-state-review-before-every-merge`,
which has every tripwire and every deferred trigger read against every change, and the trigger
test; a `design` entry's re-entry point is read by no review, as
`issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` records, and the record search
meets it instead.

### What would close it

A design discussion that decides how an issue that waits for an event is met: the trigger test made
stricter, the search's action on a met trigger changed from a ruling before the work to a report, the
kinds `deferred` and `design` reshaped, or the current text kept with its reason; recorded in the
heads above, and `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` judged in the
same discussion.
