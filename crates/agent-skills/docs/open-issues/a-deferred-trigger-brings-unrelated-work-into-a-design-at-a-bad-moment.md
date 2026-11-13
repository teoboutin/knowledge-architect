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
`milestone@plans@split-guidelines-and-plan-directories`, attributed with its hedge: in the owner's
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
