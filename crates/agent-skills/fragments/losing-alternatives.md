%% Delivered as a fragment: `design@agent-skills@shared-text-fragment`.
In the Component's `register@*@rejected`: a heading register, one level-three heading per entry,
cited `rejected@<anchor>@<id>`.

**Most designs not kept earn no entry.** An entry is read by someone reasoning from first
principles who was not there, and it is reached by grep from anywhere in the tree, so it is a
description of a rejected design that must never be read as the current one.

**Which threads of a discussion are candidates.** A thread's final state does not decide it. The
question is whether **a shape lost to an argument**.

| the thread | what happens to it |
| --- | --- |
| ruled-out | judged by the tests below |
| withdrawn with a defeating reason | the same as a `ruled-out` thread: its own proposer gave it up on an argument |
| withdrawn with no defeating reason | the deliberation only. It carries no argument a later reader could test. |
| superseded, when the absorbing thread carries its shape whole | the deliberation only |
| superseded, when a distinct shape lost | judged by the tests below, as a ruled-out thread |
| a shape rejected with nothing built in its place | judged by the tests below; the decision not to build it is a head of absence, "we do not do X", which entry test 2 of `primer@design-heads` admits, and which cites the entry |
| a shape that lost while the question stays open | the deliberation only, and the question is an issue under `skill@knowledge-architect-issue-tracking` |

**A design earns an entry only if at least one of these holds:**

1. it would change an interface others consume, as entry test 1 of `primer@design-heads` defines
   it;
2. its reason rests on a reading of an external specification the project implements;
3. it was refuted by **evidence that cost work to obtain**: a measurement, a prototype, a survey of
   other projects;
4. there is still some doubt that the design kept will achieve every goal of the project or of its
   Component.

Test 3 is the load-bearing one. Proposing something again costs one discussion round, and that is
healthy. What decides an entry is whether the refutation can be derived again inside that round.
Reasoning derives again for free, so a design refuted by an argument any competent reader
reconstructs needs no entry. Evidence does not, so a design refuted by measurement does.

Test 4 gives future design sessions the designs that were not kept, so they can start from
somewhere when the one kept proves insufficient.

A design meeting none of those stays in the deliberation, where
`skill@knowledge-architect-decision-recording@three-homes` keeps it, and nowhere else.

**Write the entry so that it cannot be read as the current design.** Name the design and the reason
it was not kept. Do not describe how it would work: an entry detailed enough to build from is the
one a grep turns into a specification.

One entry per design, keyed by **the design**, not by date. Flat, unordered, and **edited in
place**. Where it was a thread of a design discussion, its slug is the thread's name.

```markdown
### <The design not kept, named in a few words> `##<slug>`

`live`. <The reason it was not kept, checkable without leaving the entry.>
```

Each entry carries: what the design was, a **validity marker** opening its body, and the reason.
It names no winner as such, and its reason may name a head where the reason needs one: **each
design head whose argument covers the design cites the entry**, inside the sentence where
its argument meets that design, or in a closing sentence of its body. Writing the entry and its
citation is one act: `{{command}} check` reports an entry that no design entry cites.

- `live`: the reason still holds.
- `void`: the reason no longer holds. **Mark it; never delete it.** The design may still be wrong,
  but _this_ argument no longer supports that: proposing the design again needs no new argument
  against this reason, and the marker keeps a later session from ruling the design out again on
  the dead argument.

**A reason defeated in part**, as when a design the entry never weighed solves the problem its
reason called unsolved, is narrowed to what still holds, and the entry stays `live` for its own
design, since `void` would say its reason no longer holds while the design is still not wanted; the
head of that other design cites it, where its argument meets the entry's design.

**`live` records that the reason held against the arguments seen, not that the question is shut.**
A new argument or new evidence reopens any entry here at any time, and that is an ordinary move
rather than a transgression.
What counts as new is the record test of `skill@knowledge-architect-design@threads-and-states`.

**Every entry states a reason a reader can check without leaving the entry, or points at where the
reason is.** _"Refuted by measurement"_ satisfies neither: it asserts that a checkable reason exists
somewhere and does not say where.

**A rejected design that is reopened, chosen and implemented moves out of the home.** The home
must not describe as rejected a design the project now has.

**An entry also leaves when the owner rules that it fails every recording test**, as an audit may
propose, **or when no design head cites it any longer and no head of absence is written for it**,
as when the heads that cited it leave the design home for a line beside the instruction they state,
or when what it lost to is an instruction that no head states: the line beside that instruction then
names the design and the reason it was not kept, so a session that reopens the instruction still
meets it.
