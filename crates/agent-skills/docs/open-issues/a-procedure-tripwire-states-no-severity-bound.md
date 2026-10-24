---
kind: design
---
# A tripwire on a procedure of the workflow states no bound on how much the failure must matter

## Summary

The tripwire shape asks for firing evidence "specific enough that both parties would agree it
fired", and nothing about how much the failure must matter. A tripwire that guards a procedure of
the workflow then fires, by its letter, on a slip that is cheap to repair, and its response
reopens a recorded decision. The owner ruled one such firing out, and had that one tripwire bounded
to a critical gap; four others keep the unbounded shape, and a fifth fired on such a slip.

## Details

### What

Instances seen:

- During the synthetic-evidence branch, a review found a stale restatement, one sentence listing
  the tripwire rulings but not the acceptance-criterion rulings, in the work of a spec that had
  skipped its design audit. The standing-state reviewer reported the tripwire guarding the skipped
  audit as plausibly fired, with "fired" as its default, which would reopen the rule that lets a
  spec skip its audit. The owner ruled it not fired: "The point of this kind of tripwires is to
  guard against critical failures in the workflow, not small mistakes that are not load bearing.
  We won't make significant change to the workflow over nitpicks like this." The change "The
  tripwire on a skipped spec audit fires only on a critical gap, on the owner's word" bounded
  `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap`: "A gap is critical when the
  work's result, or a decision it rests on, is wrong without its repair."
- `tripwire@agent-skills@ruling-lost-in-assembly` carries a bound too, through
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes`.
- Four tripwires on a procedure still fire on one instance with no bound:
  `tripwire@agent-skills@ruling-lost-in-change`, the sibling of the bounded one on assembly, which
  watches the same failure, a misstated ruling of the owner;
  `tripwire@agent-skills@rule-title-wider-than-its-argument`;
  `tripwire@agent-skills@pointer-not-followed`; and
  `tripwire@agent-skills@attribution-loses-the-scope`, which watches a misstated ruling as
  `tripwire@agent-skills@ruling-lost-in-change` does.
- Two of the four have since been met by their letter, on slips repaired before their merge, and
  ruled not fired on that ground. `tripwire@agent-skills@attribution-loses-the-scope`: in the
  branch that set what enters agent-facing text, the transcript review found two attributions wider
  than the owner's words, one from a quotation converted at the harvest that dropped a quantifier
  and detached a clause, as the commit "[review] Repairs of the transcript review: two attributions
  narrowed to the owner's words, an open question recorded" says.
  `tripwire@agent-skills@rule-title-wider-than-its-argument`: in the owner-authority milestone, the
  transcript review of its third slice found a head's title wider than its body, as the commit
  "[review] Repairs from the transcript review of slice 3: the last "approved" site, the bounded
  head's title and hedge, the reviewer's rivals, and the changelog's fork" says. Both were judged
  by the intake of the session's retrospective files.
- A fifth, the tripwire that guarded `design@agent-skills@extension-follows-the-ground`, was met by
  its letter on two members a review named before their merge, in thaum, and the owner ruled it
  fired at the intake of 2026-10-24: it is now
  `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`. So the owner has
  ruled a slip repaired before its merge both ways.
- Two more cases met by their letter, read at the intake of thirteen retrospective files:
  - `tripwire@agent-skills@rarer-rule-in-the-model` was met by a rule that the branch writing the
    primer's model section wrote, and that its own review found and removed before the merge. The
    owner ruled it not fired and wrote that exception into the tripwire's own clause, in the commit
    "[review] The owner's ruling on Q2: a rarer rule caught and removed on its own branch does not
    fire T2". That session's retrospective proposed one sentence in the tripwire shape instead: a
    tripwire watches the record once its decision is on the main branch, and what the branch's own
    reviews find before the merge is a repair. W3 of its workflow file.
  - `tripwire@agent-skills@attribution-loses-the-scope` was met by its letter again in two sessions,
    one of this repository and one of thaum: the transcript reviewer caught, before the merge, an
    attribution that dropped the owner's hedges or a hedged proposal recorded as the owner's word,
    none load-bearing. Two other sessions reported the same slips caught by other reviewers. The owner
    ruled it not fired, on the ground the earlier instance above was ruled on.
- The shape is the issue-tracking skill's section on a tripwire entry, "**the firing evidence**,
  meeting the falsifiability bar: an event, or a count crossing a bound, specific enough that both
  parties would agree it fired", restated by the design skill's premortem. The standing-state
  reviewer asks of each tripwire "has it fired?", and weighs nothing.

The suspected mechanism, in one sentence: the falsifiability bar makes a firing clause precise
about the event and silent about its weight, so a procedure that fails a little fires as loudly as
one that fails badly.

Re-entry point: the next change to the tripwire shape of the issue-tracking skill, or to any of
the four tripwires above, or earlier on the owner's word. The discussion then states the bound in
the shape and puts each of the four tripwires to the owner, each having been written on the
owner's word.

### Why it matters

A firing reopens a decision, so a tripwire that fires on a nitpick costs the owner a ruling, against
`goal@knowledge-architect@the-owner-decides`. The owner's words above state the rule for "this kind
of tripwires", and every unbounded tripwire on a procedure repeats the ruling until the shape says
it. The shape is the issue-tracking skill's, and each tripwire changed is a ruled tripwire, per
`skill@knowledge-architect-decision-recording@ruled-tripwires`.

### What would close it

A design discussion under `skill@knowledge-architect-design` that either states in the tripwire
shape that a tripwire on a procedure names the failure that matters, as the bounded tripwire above
does, and brings each of the four tripwires to the owner with a bound; or keeps the shape and
records why, with the standing-state reviewer weighing each firing instead.
