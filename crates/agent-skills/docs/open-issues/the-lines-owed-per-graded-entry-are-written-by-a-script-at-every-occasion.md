---
kind: design
---
# The lines owed per graded entry are written by a script at every occasion, and their reasons fail the checker

## Summary

Four installed texts ask for one line per entry the record search graded, with the searcher's
reason or an outcome. No command writes those lines, so twelve sessions built them by a script, and
four met a cycle of checker findings on the reasons they copied. Which shape the record takes, one
line per entry, a count, or a committed file, decides what a command would write, so the two are
one decision.

## Details

### What

The four texts, as they stand:

- the design audit, `path@agent-skills@content/skills/planning/SKILL.md`: "The audit's record gives
  one line to each entry the search graded `bears` or `may-bear`, with its outcome, an entry judged
  to bear on nothing in the slice included: in the message of the audit's commit";
- the plan document's "What is already decided", the same file: "or, for a `may-bear` entry the
  grounding did not read, "not read" with the searcher's reason";
- the in-change path and bounded work, `path@agent-skills@content/skills/design/SKILL.md`: "the
  `may-bear` entries the grounding did not read, each with the searcher's reason";
- the intake, `path@agent-skills@content/skills/retrospective-intake/SKILL.md`: "The analysis lists
  each other entry graded `may-bear`, with the searcher's reason, as not read".

The reason for one line per entry is a `%%` comment at the audit's rule in the planning source,
which `cargo x skill knowledge-architect-planning` prints: a judgement that an entry bears on
nothing is the claim a later reader cannot check without it, and the rival, no record of such an
entry, sent the transcript reviewer to report every unrecorded judgement as an item with no
outcome. The searcher's reason is written for the dispatching session:
`agent@knowledge-architect-record-searcher@fill-the-report` asks "one line of reason: what in the
work meets what in the entry, or the seed it cites", in no checked form. `klarch worklist --verify`
writes `bears.md` and `may-bear.md` only (`assemble` in `path@query@src/worklist.rs`), per
`design@query@graded-entries-assembled`.

The instances, 12 sessions, each building the lines by a script; nine are this repository's,
three thaum's:

- the intake of seventeen files: 164 lines over `may-bear.md`; the reasons copied whole gave 13
  findings, which a second script rewrote from the checker's repair lines;
- the plans of the development milestone: 123 rows of "What is already decided";
- the Component rules revision of 2026-11-02: an audit record of over 200 lines generated with a
  default outcome per grade, which its file says carry no information a reader can check; and its
  revision of 2026-11-03: 88 unread entries in an in-change message, rebuilt from the entry blocks
  by a script;
- the plans of the Component split: a script checked that the plan documents give every graded
  entry an outcome, and its first run found entries missing;
- the slice that moved `git` out, the development milestone's slices (the judges' files
  concatenated, labels renumbered, a set difference against the two files), the slice that renamed
  `check`, and the slice of the integrated tests: 365 lines in the audit's message, on which
  `commits` gave 45 findings, repaired by a script and 30 by hand;
- three sessions of thaum: 171 lines, about 110 of the 125 `may-bear` lines bearing on nothing
  for citing a seed alone; an audit's 200 lines printed by a script with a default outcome; and 81
  "not read" lines whose reasons held references without backticks, bare slugs and unanchored
  paths, 11 findings.

The four cycles of findings on the reasons or the record's spans: 13, 41 and 49, 11, and 45
findings. Beside them, the audit's message of the slice that put `cli` on top, 400 lines, failed
`commits` on two runs.

Not reproduced: a session of 2026-11-03 reported the reasons in the table of contents of
`may-bear.md` cut at about 300 characters with no mark. The table of contents is written as the
reference and every reason joined by " / ", with no cut, at the file's commit and in
`path@query@src/worklist.rs` now; its lines reach 900 characters in the intake of forty files'
`may-bear.md`, by `awk` over the section with `length($0)`. The cause of that session's cut is not
established; an assumption: the tool it read the file with.

The rivals, as the analysis weighs them:

- the nearest fix for all four occasions: `--verify` writes, beside the two files, one line per
  graded entry in checked forms, its reference, its grade and its reasons, each bare slug and path
  rewritten as the checker's repair names it, for the session to complete with an outcome. It keeps
  the per-entry claim of the `%%` reason, and makes a coverage check a comparison of two lists;
- a count line for the entries that bear on nothing: it keeps the defaults out of the message and
  loses the per-entry claim, the fact the `%%` reason rests on;
- the record in a committed file that leaves with the plan document, the message naming it, as
  `design@agent-skills@committed-audit-report` keeps a project audit's record over
  `rejected@agent-skills@commit-messages-as-audit-record`: it moves the lines out of the message and
  keeps the evidence, and needs a new home in the primer's knowledge table;
- the searcher writing checked reasons: it adds an instruction to every searcher, which a command
  applying the checker's own repair makes unneeded.

The command is the mechanical half, which the owner takes in under
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

The suspected mechanism, in one sentence: the texts ask for a list a command could derive from the
search's output, so each session derives it by hand, and the searcher's reasons, written for a
reader and not for the checker, fail when copied into a checked document.

Re-entry point: a design discussion under `skill@knowledge-architect-design`, on the owner's word,
or when work next changes what `worklist --verify` writes; then `skill@klarch-development` in
`query`.

Source: C3 of the analysis 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow,
in the commit "[intake] The analyses of forty received retrospective files, three outcomes for the
owner", with the findings it groups as cluster B.

### Why it matters

Every occasion of the search repeats a hand task, and a cycle of findings at each, against
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`; the lines are what the
design-conformance reviewer reads, per `design@agent-skills@unread-entries-reach-the-conformance-review`,
so a line missing from a script's output reaches no reviewer, against
`goal@knowledge-architect@records-reach-their-reader`. A third output of `--verify` widens
`design@query@graded-entries-assembled`, and a count line argues against the `%%` reason, which
four texts state.

### What would close it

A discussion under `skill@knowledge-architect-design` that settles the record's shape at the four
occasions, the per-entry line, a count or a committed file, records it where its reason must be
respected at the four texts, and the command that writes the lines in checked forms, built in
`query`; or a ruling that the lines stay hand-built, with the reason.
