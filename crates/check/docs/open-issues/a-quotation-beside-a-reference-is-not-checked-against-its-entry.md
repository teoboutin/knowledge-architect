---
kind: design
---
# A quotation beside a reference is not checked against the entry it cites

## Summary

A text that quotes an entry in quotation marks beside its reference keeps resolving when the entry
is reworded, so `cargo klarch check` passes while the quoted words are no longer the entry's. One
session found three heads quoting a goal whose rewording no longer held the quoted words. No lint
compares such a quotation with the entry, and the cost of one, its count of sites and its false
findings on paraphrases, is not measured.

## Details

### What

The instance: the workflow file of the first slice of the goals, 2026-11-11, its C3, analysed in the
section `goal-citers-not-reread` of the analysis of
2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow, which "[intake] The forty-seven
retrospective analyses leave, every outcome carried out" removed. "Three heads quoted a goal in
quotation marks beside its reference, and the goal no longer held the quoted words." The report does
not name the three heads.

The mechanism, in one sentence: a rewording keeps the slug, so every reference still resolves, and
no lint of `check` reads a quotation against the entry its paragraph cites; by
`design@check@quotation-read-by-no-id-lint`, the lints on ids do not read a quotation at all.

A check of the same kind exists outside `check`: `judge` in `path@query@src/worklist.rs`, run by
`cargo klarch worklist --verify`, refuses a searcher's quotation that is no run of whole words of
the entry's own text. No recorded decision says whether `check` may depend on `query` to reach that
matching; the design discussion of such a lint decides it.

Not measured: how many quotations beside a reference this tree holds, and how many of them
paraphrase rather than quote, which a check would refuse wrongly. The report states it did not
measure either.

The suspected shape: a lint that, for a quotation in a paragraph holding one reference, checks that
the quoted words are a run of the cited entry's text, leaving
`design@check@quotation-read-by-no-id-lint` standing for the lints on ids.

Re-entry point: a design discussion under `skill@knowledge-architect-design` once the census below
is taken, or on the owner's word.

### Why it matters

A quotation that no longer matches attributes to an entry words it does not hold, and a reader who
trusts the quotation acts on a rule that was reworded, against
`goal@documentation-stays-consistent`. The re-read of a reworded entry's citers, which the
instance's other finding asks of a session, finds such a quotation only by reading every citer.

### What would close it

A census of the quotations beside a reference in this tree, and of the share that paraphrase; then
a design discussion that either ships a lint with a test over a mock project, or rules that the
count does not pay for one, the entry deleted with that ground in the commit.
