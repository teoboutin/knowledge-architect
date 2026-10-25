---
kind: todo
---
# No check compares an audit pass's count of severe violations left with the previous pass's

## Summary

`design@agent-skills@root-cause-repair` rests on the premise that repairing each cause rather than
each finding makes the violations an audit leaves fall from one pass to the next. Of the two counts
that show it, the total on old text is already compared at each re-check, and the severe ones, those
whose outcome went on the owner list, are counted by each pass and compared by nothing. A check in
the re-check's report would compare them.

## Details

### What

The convergence of an audit was written as an acceptance criterion the owner proposed, AC4 of the
discussion that made `design@agent-skills@root-cause-repair`: from one pass to the next, the
violations left on old text fall, and the severe ones among them, those whose outcome is the owner
list, fall while above zero. Old text is text the previous pass also read, whose words are
unchanged since the commit it audited, per `design@agent-skills@convergence-count-scope`; a first
pass is not counted. The owner ruled its tripwire removed, judging it, in the owner's view, no
tripwire but a recurring acceptance criterion of the work that creates or changes an audit. Its
second half meets the four conditions of the tripwire standard, so the removal rests on the owner's
word, not on a condition. Of its
two halves:

- The first already stops the work: under
  `design@agent-skills@audit-passes-repeat-while-the-count-falls`, a pass where the count on old
  text does not fall goes to the owner as a question, with the counts, and starts no pass on its
  own.
- The second is a comparison of two counts each pass already records:
  `skill@knowledge-architect-project-audit@record-of-an-audit` asks each pass for the count of
  violations left on old text and on new text, and of those on old text that went on the owner
  list.

The missing work: at the report step of a re-check, each axis's saved workflow writes a line that
compares the count of violations left on old text that went on the owner list with the previous
pass's count, and the stop rule's question to the owner carries it beside the total. Three saved
workflows take `previous` for a re-check: `path@agent-skills@content/workflows/agentic-workflow-audit.js`,
`path@agent-skills@content/workflows/design-record-audit.js` and
`path@agent-skills@content/workflows/tripwire-audit.js`.

Evidence when the criterion was written: in the first agentic-workflow audit of this repository, as
an acceptance criterion, it failed at passes 1 to 3 and 5 on the plain counts, and at pass 6 on the
old-text counts: 21 violations left against pass 5's 16, the severe ones falling from 8 to 7. The
owner stopped the audit after pass 6's application.

### Why it matters

Until the check exists, a pass whose total falls while its severe violations do not is read only
by the session that writes the pass's commit, from counts no step compares. It strains
`design@agent-skills@root-cause-repair` and `design@agent-skills@audit-passes-repeat-while-the-count-falls`.

### What would close it

The comparison line written by each of the three saved workflows at a re-check's report step; and a
pass whose severe count does not fall while above zero put to the owner as a question, as the stop
rule puts a pass whose total does not fall, since the stop rule alone never reaches the case where
the total falls and the severe count does not.
