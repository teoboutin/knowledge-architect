---
kind: design
---
# A demand a light verdict of the record search makes on the work can be left unanswered, and no check reports it

## Summary

A light verdict, `constrains` or `touches` with the action `respect` or `none`, can carry in its
reason a demand on the work, such as a head it owes or an entry it must cite. A session that reads
only the light lines it picks leaves such a demand unanswered, and no check reports it. On one branch
17 of 144 light verdicts were unanswered, and the review before the merge is what found them.

## Details

### What

The instance: the search of the work that removes what the tests left in the temporary directory,
on the branch that added the lint on `std::env::temp_dir`.
Its 7 verdict files held 275 light lines, 144 distinct entries with one verdict each: 114
`constrains, respect`, 15 `constrains, none`, 10 `touches, respect` and 5 `touches, none`. The session
read the act verdicts and the light lines it picked by name, told the owner that it had gone through
the list, read the lines nearest the work, and that the design kept within them, and built the work. At the review, the
design-conformance reviewer read the light lines that carry an action and reported demands left
unanswered; an agent then compared all 144 with the branch's diff and messages, and judged 17
unanswered, 94 answered and 33 with no demand. Most of the 17 had been met by the review's axes too.
The message of the commit "[knowledge-architect] The entries of this branch's records are brought to
what the review found, and two defects of `mutate` are opened" records the pass and the repairs.

Three of the 17:

- `design@agent-skills@a-head-is-owed-by-an-entry-test`, `constrains, respect`, whose reason ends on
  a demand: "so the guard passes test 2 and earns a head; the draft names no head". The work was
  built with no head.
- `issue@model@a-rust-item-named-in-prose-has-no-checked-form`, `touches, respect`, quoting "a site
  naming a Rust item in prose cites it". Two entries the branch opened named Rust items and did not
  cite it.
- `design@model@unportable-names-refused`, `constrains, none`, whose reason named tests of the
  `git` Component, among others, that the draft's list of blockers omitted. The entry the branch opened
  listed none of `git`.

What reads a light verdict, and where each reading stops:

- **The session**, which `design@agent-skills@session-acts-on-the-verdicts` has check each light
  line, its quotation and its reason, and whose design skill has the round after the search state the
  outcome of each verdict other than `unrelated`. A statement that the design respects the rest
  leaves no trace of the lines that were read.
- **`klarch worklist --verify`**, which judges the searchers' reports, never what the work did with
  them.
- **The generated lines** the commit message carries, per
  `design@agent-skills@search-record-kept-per-occasion`: verdict and action, with no reason, so a
  demand does not show there, and an answered line and an unanswered one read the same.
- **The design-conformance reviewer**, per
  `design@agent-skills@verdicts-reach-the-conformance-review`: from the message it runs `show` on no
  entry whose verdict is `constrains`, so it meets those demands only where the verdict files still
  stand and its brief names their directory, as here.

### Why it matters

A demand left unanswered is found after the work is built, where it costs rework, or is not found. It
strains `design@agent-skills@session-acts-on-the-verdicts` and the premise of
`design@query@verdicts-split-by-action` and
`design@agent-skills@verdicts-reach-the-conformance-review`, that a light verdict asks the work only
to keep within the sentence it quotes, against `goal@agents-work-without-drift`.

### What would close it

The design discussion of the re-entry point, deciding where a demand that a light verdict makes is
carried and what shows its answer, or that nothing more is needed, with the reason. One instance is
no recurrence, so it moves no instruction by itself, per
`skill@knowledge-architect-agent-configuration@admission-standard`: a second one, in another piece of
work, is the evidence that text asks for.

### Suspected mechanism

A `constrains` or `touches` verdict admits no action but `respect` and `none`, per
`design@agent-skills@verdict-report`, so a demand for something the work must add sits only in the
reason of one line among many, and no later text states, line by line, whether it was answered. Not
established.

### Re-entry point

The next design discussion that changes which verdicts and actions a searcher gives, how the session
acts on the verdicts, or what the design-conformance reviewer reads of them.

### Leads

- The same line read again: `rejected@query@light-lines-without-reason` records three verdicts graded
  light that bore on a design and that the session let through, where the lines held no reason. Here
  they held it.
- `rejected@agent-skills@verdicts-verified-in-the-workflow` rests in part on "the session still reads
  every light line to keep the work within it", which this instance shows can be false.
- What found the 17 was one agent given the 144 deduplicated lines, the diff and the messages, after
  the work was built.
