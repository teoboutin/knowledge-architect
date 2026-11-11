---
kind: todo
---
# A review dispatched by a workflow keeps no reviewer's report in the session's transcript

## Summary

`design@agent-skills@reviewer-returns-its-report-whole` has a reviewer return its report whole as
text so that the session's transcript holds the other axes' findings for the transcript reviewer.
A review whose reviewers a workflow dispatches returns their text to the workflow's script, not to
the session, and the script's own return, or the notice that carries it, is what the transcript
holds. The installed review skill says nothing of a review run so.

## Details

### What

The instance: the review before the merge of the branch that gave reviewers their own report files
ran its ten axes, then the eight a repair owed, each as one workflow whose script returned
`{axis, returned: 'yes'}` per axis. The completion notices held those lists alone; the session read
each reviewer's report from its file, a tool result, which `klarch transcript show` drops unless
given `--tools`. The transcript reviewer found it, reading the extraction with `--tools`.

A related case: a workflow whose script returns the reviewers' texts joined can reach the session
cut, its notice ending "truncated <n> chars", as the record search of the same branch returned.

The premise it defeats in that branch's record: the ground of a premortem cause dropped on the
doubt's weight, "the owner sees each report returned in the session".

### Why it matters

The transcript reviewer is the one axis that can see whether the other axes' findings were acted on,
per `design@agent-skills@transcript-review-last-before-merge`, and it reads them as messages of the
transcript. A review run by a workflow leaves it the files alone, which a later session may not
have kept.

### What would close it

The review skill states how a review dispatched by a workflow delivers each report to the
transcript, or the transcript reviewer's brief names the report files and its definition reads
them; chosen under `skill@knowledge-architect-design`.
