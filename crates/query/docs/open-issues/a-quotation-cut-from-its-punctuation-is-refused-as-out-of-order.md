---
kind: defect
---
# A quotation whose last word is cut from its punctuation is refused as words "not in this order"

## Summary

`cargo klarch worklist --verify` and `--check` refuse a quotation whose edge word stops before the
punctuation the entry's text attaches to it, such as `job` where the text has `job,`. The refusal
is right, since a quotation is matched as a run of whole words. Its message is wrong whenever the
cut word also stands whole elsewhere in the entry: it then says "each of its words is in the text,
but not in this order", which names a cause that does not hold and no repair a searcher can apply.

## Details

### What

The refusal is written by `judge` in `path@query@src/worklist.rs`: it splits the quotation and the
entry's own body on whitespace, and refuses the quotation when no window of the body's words equals
it. `quotation_miss` then names why, in two steps:

1. it takes the first word of the quotation that is no whole word of the body, and names the body
   word that holds it, "its word `<w>` is part of the word `<w,>` there";
2. where every word of the quotation is a whole word somewhere in the body, it says "each of its
   words is in the text, but not in this order".

Step 1 asks whether the word stands whole anywhere in the body, not at the place the rest of the run
matches. A cut word that also stands whole elsewhere passes step 1, and the message falls to step 2.

**Reproduction**, at the level of `judge`, with the test helpers `plain`, `line` and the subject
list of the module's tests: the body

```text
### A publish is completed by re-running the job, which skips a version `##a-head`

The job runs again.
```

with the quotation `A publish is completed by re-running the job` gives the one finding "the
quotation is not in the entry's own text: each of its words is in the text, but not in this order;
copy it verbatim from the entry's text, ...". The same quotation ending in `job,` gives no finding.
Taken in a scratch worktree with a temporary test, on the branch `fr3-heads-split`.

**Observed**: one record-searcher's report of a design grounding on the branch `finding-repairs`
had 8 quotations refused, each a run of an entry's text that stops before a comma or a colon at its
last word. One was named right, "its word `less` is part of the word `less:` there", since `less`
stands nowhere else whole. The 7 others were refused as "not in this order", among them the heading
run "A publish that stopped part-way is completed by re-running the job" of
`design@knowledge-architect@partial-publish-rerun`, where the heading has `job,` and the body has
`job` whole, and "Every version after a crate's first is published by the release workflow" of
`design@knowledge-architect@publish-credential`, where the heading has `workflow,`. Adding the
comma to those two quotations made them pass. The searcher took the matcher for broken and replaced
the 7 quotations with others, and reported "a possible defect of the quotation matcher".

Ruled out: a quotation from the line naming the entry, or from a citing site. Each of the 7 is a
substring of the entry's own text, `cargo klarch show` on the entry, whitespace collapsed; each is
followed there by `,` or `:`.

**Recurrence**, from the retrospective files received at the intake of 2026-11-14: twelve sessions,
ten of them on a tree after the message repair of the change "[query] The report check says what
keeps a quotation from being found, and which line it may not come from", report that most
searchers' first
`--check` failed on quotations cut one word short of their punctuation, or under eight words, from
2 to 50 per report, and that nearly every searcher wrote a script of its own to extend each
quotation to whole words, then ran the check 2 to 4 times. The scripts follow a right message as
well as a wrong one, so a repair of the message alone leaves the round trip.
Another, 2026-11-25, the search at the design discussion on the gates on Windows and macOS: one of its
searchers reported that 30 of its 50 quotations failed the first `--check`, most of them cut before
their punctuation, and that a script extended them.
Another, 2026-11-25, the search at the change of the retrospective intake's division of its items: one
searcher reported that the first `--check` refused 31 of its 51 quotations, all cut before the
punctuation of their last word or at a mark, and that a script of its own extended them.
The files propose three shapes: the finding prints the whole-word run it would accept, punctuation
included, which keeps every head; `--check --fix` writes that run, against
`design@query@worklist-check-judges-one-report`; or the check accepts a quotation that differs only
by punctuation at its ends, against the run of whole words of
`design@query@worklist-writes-and-verifies-the-reports` and the verbatim quotation of
`design@agent-skills@every-verdict-quotes-its-entry`. Not established: whether a printed run still
leaves each searcher scripting the edit of its refused lines.

### Why it matters

The refusal's repair is to extend the edge word to the whole word of the text, and the message
names an order problem instead, so a searcher who copied the run verbatim finds nothing to repair
and replaces the quotation, at the cost of one more `--check` round per quotation. The doc comment
of `quotation_miss` states that it names the body word holding a word "cut from its punctuation";
the case where that word also stands whole elsewhere misses that intent. It strains
`design@query@worklist-writes-and-verifies-the-reports`, whose instruction to quote each entry is
kept by the command so that the searcher does not have to guess what the command wants.

### What would close it

`quotation_miss` names the cut edge word, with the body word holding it, wherever the quotation's
other words match a run of the body that the cut word's whole word continues, whether or not the cut
word stands whole elsewhere; "not in this order" stays for a quotation whose words match no run.
A test of `judge` with the body above shows the message naming `job` and `job,`, and the mutation
that restores the search for the word anywhere in the body is caught.

Accepting such a quotation instead, by matching an edge word against the body word without its
punctuation, would change what `design@query@worklist-writes-and-verifies-the-reports` decides, a
run of whole words, and goes to `skill@knowledge-architect-design` first.

The message repair alone no longer closes this entry, since the round trip of the recurrence under
"What" follows a right message too. The entry closes with a shape, argued under
`skill@knowledge-architect-design`, by which a searcher repairs a refused quotation with no script
of its own, the message repair above built with it or made needless by it; or with the owner's
ruling, in the commit that deletes this entry, that the round trip stays.
