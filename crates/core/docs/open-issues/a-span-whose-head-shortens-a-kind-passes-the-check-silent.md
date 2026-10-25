---
kind: defect
---
# A backticked span whose head shortens a kind's name, as `arg@<plan>@<id>` for `argument`, passes the check silent

## Summary

A plan document cited 30 of its own arguments as `arg@<plan>@arg-<n>`, a span whose head, `arg`,
is the kind `argument` cut short. `cargo klarch check` passed with no finding: the head is neither a
kind nor an anchor, so the candidate rule reads the span as no reference. A reviewer found it by
reading. `tripwire@core@candidate-rule-silence` fired on it, and this entry is the response its
firing names for a decision not reopened in the same change.

## Details

### What

- The spans: every pointer to an argument in the first commit of the spec of the transcript
  command, "[plans] The spec of the transcript command: list, show and search over Claude Code's
  sessions", 30 lines, each of the form `arg@<plan>@arg-<n>`, where the kind is
  `argument@<plan>@arg-<n>`. The decision-record reviewer of that spec's review found them.
- What the checker does: `candidate` in `path@core@src/entity.rs` reads the text before the first
  `@`; a head that is a known kind is resolved, a declared or reserved anchor in that position is
  reported as the old form, and any other head yields `Candidate::NotOne`, "Not a reference;
  silent". `cargo klarch show` on such a span refuses it as "not a reference", naming the kinds, but
  nothing runs `show` on every span.
- `cargo klarch model` records each such span as a `span` observation, so the scanner saw them; the
  silence is the resolver's.
- The decision that makes it silent: `design@core@at-span-candidate-rule`, "A typo inside the kind,
  `desing@<anchor>@<id>`, is silent under this rule, because widening it to "any span with two `@`"
  would report every email address with a plus tag."

Reproduction: in any project, a document holding a backticked `arg@<plan>@arg-1`, where `<plan>`
is a spec's id and `arg-1` an argument of it; `cargo klarch check` reports nothing for the span,
and `cargo klarch show` on it exits 2 with "is not a reference".

Ruled out: the span is no anchor collision, since no anchor is named `arg`; it is no unknown
anchor in kind position, which the old-form finding reports.

Not established: how many spans of this shape the tree's history held before this one, and whether a
narrower widening, a head that is a proper prefix of a known kind's name, reports anything else in
this tree or thaum's. A census over both trees, counting the spans whose head is a proper prefix of
a kind, answers it.

### Why it matters

A pointer that the check reads as nothing is dropped from `cargo klarch show`'s list and dangles
unseen when its target moves, against `goal@knowledge-architect@documentation-stays-consistent`.
The plan's harvest reads its argument pointers to find each head's standing argument, so a silent
pointer loses an argument at the harvest. It strains `design@core@at-span-candidate-rule`.

### What would close it

The candidate rule widened to report a span whose head is a proper prefix of a known kind's name,
or within a small edit distance of one, with the kind as its repair, under a false-positive census
over this tree and thaum's, and a test planting `arg@<plan>@arg-1` in the mock `planted`; or the
owner's ruling that the silence stays, recorded in `design@core@at-span-candidate-rule`.
