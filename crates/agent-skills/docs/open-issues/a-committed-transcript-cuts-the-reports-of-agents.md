---
kind: design
---
# A committed transcript cuts the reports of the agents its discussion dispatched

## Summary

The planning skill extracts a design discussion's transcript with `klarch transcript show`, without
`--tools` and without `--full`. An agent's report reaches the session as an injected text, which
`show` cuts to its first lines without `--full`, so the committed transcript holds no report of an
agent whole. A plan item that quotes or attributes an agent's finding cannot be checked against the
committed copy.

## Details

### What

- The mechanism, in `path@transcript@src/command.rs`: `print_entry` prints an entry whose label is
  not one `never_cut` names, and whose text runs over `CUT_ABOVE` lines, as its first `HEAD_LINES`
  lines and a line "(<n> lines omitted; --full prints them)". `Label::never_cut`, in
  `path@transcript@src/reader.rs`, names the owner's labels, the session's own messages, a dispatch
  and an unlabelled line; an injected text, such as a message of another session or a task
  notification, is cut.
- The instance: the transcript the commit "[plans] The spec of the discussion on committed
  transcripts, written after its work on the owner's direction" committed, as `transcripts/1.txt`
  of that spec's directory, holds three such lines. One follows the opening of the goal reviewer's
  report, delivered to the session as a message from another session, "(141 lines omitted; --full
  prints them)", while the arguments of that spec's `deliberation.md` on the sentence it added to
  `goal@the-owner-decides` cite the goal reviewer's findings. The other two follow the openings of
  the two skills the session loaded.
- What a discussion relies on of an agent's report is otherwise only in the session's own message
  that relays it, which `never_cut` keeps whole.

Suspected mechanism, in one sentence: the extraction's flags were chosen to keep the owner's and the
session's words and drop tool calls, and an agent's report reaches the session by another channel,
an injected text, which the default cut treats like any other injected text.

Re-entry point: the next change to the extraction a committed transcript takes in the planning
skill, or to what `klarch transcript show` cuts.

### Why it matters

A deliberation that rests on an agent's finding is checked against the committed transcript, per
`design@agent-skills@committed-transcript`, and the finding is not there whole: a misreported
finding goes unseen, against `goal@design-is-recorded-with-its-arguments`. Extracting with `--full`
would also print the injected skills whole, which the cut keeps out of the file on purpose.
`issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript` is a neighbouring
case: a reviewer's report read from a file, a tool result that the extraction drops.

### What would close it

A decision, under `skill@knowledge-architect-design`, on whether a committed transcript keeps the
reports of agents whole, and by what means: an option of `klarch transcript show` that keeps a
message of another session whole while it cuts the injected skills, or the planning skill's
extraction changed; or the argued decision that the session's relaying message is the record.
