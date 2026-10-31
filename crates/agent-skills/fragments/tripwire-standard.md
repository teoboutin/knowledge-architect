%% The decisions: `design@agent-skills@tripwire-admission-test`, `design@agent-skills@tripwire-horizon`.
A tripwire watches one premise of a recorded decision: a failure that would show the decision
wrong and that nothing else in the project reports. Most causes a premortem names are not that,
and are not written.

**A cause becomes a tripwire only when all four conditions hold.**

1. **The failure is silent.** Every procedure that meets it counts it as success: the tests and
   gates pass, no work is stopped by it, no review reports it as a defect of the decision, and no
   request names it. A failure that fails a test or a check, stops the work that meets it, or
   arrives as a request for what the decision refused reaches the decision without a tripwire.
   Where a script could compute the firing, write that check instead. A check of the work being
   built is no tripwire: it is that work's acceptance criterion where its firing would show a
   decision wrong and it meets conditions 1, 3 and 4, and otherwise a claim of the step that builds
   it. A check that can be written only after another event,
   such as the end of a migration, is a `deferred` issue whose trigger is that event.
2. **The evidence lands where tripwires are read.** It is in the tree, the history, an audit
   report, or a retrospective file the intake receives. Evidence that exists only in a session's
   transcript, or only in what the owner sees while driving a session, reaches no occasion that
   reads the tripwires.
3. **The evidence is what the decision says must not happen.** A clause that counts what the
   decision instructs or endorses fires on the decision working.
4. **A number has a basis.** A bound in the firing clause comes from a measurement or an argument
   the decision's record carries, its head, or its plan document before the head is written, and
   exceeds the noise of the instrument that takes it. Otherwise the clause names the event,
   specific enough that two readers agree whether it occurred: "it gets slow" names none.

**The owner's word admits no tripwire alone.** A cause the owner proposes, or one derived from the
owner's words, is judged by these conditions like any other, and a tripwire that fails them is not
kept, whoever proposed it. A ruling of the owner that a tripwire restates, such as a bound or an
exception, lives in a design head, which the tripwire cites.

**A slip is no firing.** A clause on how sessions follow an instruction never fires on one instance
that a review, a check or a retrospective reported under its own procedure: one slip is the review
working. It fires on the same failure recurring in separate pieces of work, or on one instance that
reached the main branch with no procedure reporting it. Where a clause fires on both a slip and
something more, strike the slip.

**A doubt keeps the cause, briefly.** A cause that no condition clearly excludes is recorded with a
short horizon. A wrong keep costs reading until its horizon; a wrong drop loses the evidence for
good, unless the owner would see the failure anyway, by using the project or by carrying work with
its agents: then a wrong drop costs little, and a cause in doubt may be dropped. That weighs a
doubt only; it never drops a cause the four conditions admit.

The entry is a level-two heading in the tripwires home of the Component that owns the decision, its
slug last:

```markdown
## Guarding `design@<anchor>@<id>`: <the premise watched> `##<slug>`

**Fires when:** the event, or a count with its basis.
**Response:** reopen the decision by its reference, or open an issue of a named kind.
**Re-entry:** an occasion that holds the evidence.
**Watches until:** the occasions after which the cause, unfired, no longer will.
```

**Watches until** names the first occasions that exercise the decision, so that a command over the
history counts them from the commit that wrote the line: commits on the main branch that change a
named path or the guarded head's text, release tags, audit reports of an axis, plan documents
deleted at their landing, or one event the tree shows, such as the first issue a consumer of an
interface opens; never an occasion that only a session's transcript holds. A relocation, a commit
whose added lines are exactly the lines it removes, is no occasion, since it exercises no decision.
Its length is the writer's judgement, short for a cause kept on a doubt, and no count is kept in the
entry.
`standing` is for a guarantee whose failure stays silent and permanent, with the reason in the
entry.
