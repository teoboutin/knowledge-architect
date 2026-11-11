---
kind: design
---
# A slug in the definition form inside a quotation is reported as a misplaced definition

## Summary

When the owner names an entry by its slug in the definition form, a hash pair and the slug in
backticks, and a session copies the owner's words verbatim into a document, the checker reports the
span as "written in the middle of a line" and defining nothing. The only repairs the finding names,
move it or delete it, alter the owner's words.

## Details

### What

Reproduced at the intake of 2026-11-07, with the checker built from this repository, over a mock
project made by `klarch init --name demo` in a fresh git repository, with these lines appended to
its README.md:

```markdown
The owner wrote: "rename `##some-goal` please".

> keep `##other-goal` as it is
```

`klarch check` exits 1 with two findings, one per line, each "`##<slug>` is written in the middle
of a line and defines nothing", the line's slug in place of the placeholder, with the repair "a slug
is defined at the end of a heading at its register's level, inside that register's home; [...] move
it there, or delete it". The same first line inside a fenced block gives no finding.

The code: in `path@model@src/scan.rs`, the function `scan` records every slug-shaped span of a line
as an `Observation::SlugDef`, a mid-line one as `SlugSite::Inline`, with no regard to a quotation;
in `path@model@src/entity.rs`, `heading_definitions` writes the finding.

The instance: the session that narrowed the goals, 2026-11-01. The owner's message quoted two goal
slugs in the definition form; copied verbatim into the goal-review record, each was reported. The
session removed the backticks and said so beside the quotation, with no issue reference, since no
entry recorded the missing form.

The heads that bear on it:

- `design@model@a-slug-elsewhere-defines-nothing` reports a slug in the middle of a line, because
  "A mid-line slug is a pointer written in the definition form, and recording it is what makes such
  a pointer visible";
- `design@check@quotation-read-by-no-id-lint` exempts a quotation, text between double quotes or a
  block-quote line, from the three lints on ids alone, since "a finding there would have no repair
  the writer may make". The misplaced-slug finding is no lint on ids, so the exemption does not
  reach it;
- `design@agent-skills@a-past-sentence-is-rewritten` keeps a verbatim quotation of the owner as it
  is.

Suspected mechanism, in one sentence: the quotation the lints on ids already compute is not
consulted by the slug-site check.

Ruled out: a new mark for a verbatim block, the rival a retrospective file proposed. The checker
already has a quotation, defined once in `design@check@quotation-read-by-no-id-lint` and computed by
the scan; a second definition would be a second home.

### Why it matters

The finding has no repair the writer may make, which strains the argument of
`design@check@quotation-read-by-no-id-lint` and the rule of
`design@agent-skills@a-past-sentence-is-rewritten`, and it pushes a session to edit the owner's
words or to write a pointer bare, against `goal@documentation-stays-consistent`.
A session meeting the case now cites this entry beside the quotation, as `primer@where-knowledge-goes`
asks of a pointer no checked form expresses.

### What would close it

The re-entry point: a discussion under `skill@knowledge-architect-design`, since exempting a
quotation adds an exception to `design@model@a-slug-elsewhere-defines-nothing`. It either extends the
quotation's exemption to the misplaced-slug finding, rewriting that head, with a test over a mock
project holding the two lines above, or keeps the head and states why.
