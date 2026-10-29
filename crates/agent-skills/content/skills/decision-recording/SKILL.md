---
name: knowledge-architect-decision-recording
description: MUST use before writing into a design home, whatever the activity, and when a design decision has been made or reversed — a choice about how the project or one of its Components is built, including anything a consumer of it may depend on — in order to decide whether it earns durable text at all (most implementation choices do not) and where that text lands. Covers when recording happens, the reversal check that comes before everything else, which Component owns it, the split between the design home, the rejected designs and history, what earns a head, what a head holds and how it is shaped, where a head is written and how a slug is renamed, and how a premortem's causes become tripwires.
---

# Recording a decision

Scope: turning a decision about **what we build** into durable text. Whether it earns durable text
at all, when it is written, which Component owns it, what goes in the design home, what stays in
history, and which of a premortem's causes become tripwires, and when.

Not covered here: **having** the discussion (`skill@knowledge-architect-design`);
**writing the spec or the milestone document** that carries a decision until its work lands
(`skill@knowledge-architect-planning`); **parking something undecided**, and writing a tripwire's
entry (`skill@knowledge-architect-issue-tracking`). A decision about the agent configuration itself
is both this and `skill@knowledge-architect-agent-configuration`: that skill owns how to write the
configuration, this one owns where the argument lands.

## When recording happens {{slug:when-recording-happens}}

**A decision is recorded when the work that implements it lands, not when it is decided.** A
design home holds built intent: the design as built and its reasons, against which the code is
checked. A head written before the code exists would report a defect in code nobody has written.
While the work is open, the decision is unbuilt intent, and the spec or the milestone document is
the only place it exists, on the main branch or on the work's. Writing it into the design homes from there is
the **harvest**, and it happens in the change that lands the work.

A decision that no work implements, such as a policy, and that is not part of any spec, is
recorded when it is made: there is no implementing change to wait for. The second entry test of
`skill@knowledge-architect-decision-recording@what-earns-a-head` admits it, as a decision that no code and no text of the project states.

**A default awaiting the owner is built as written, per
`skill@knowledge-architect-planning@working-a-slice`, and enters a design head once the owner rules
on it.** Until then it is no approved decision, and a head is read against the owner's approval
before it is written, per `skill@knowledge-architect-decision-recording@head-shape`.

## Does it reverse something already recorded? {{slug:reversal-check}}

{{fragment:reversal-check.md}}

## What earns a head {{slug:what-earns-a-head}}

{{fragment:head-entry-tests.md}}

## Which Component owns it {{slug:owning-component}}

{{fragment:owning-component.md}}

## A decision approved with no argument {{slug:unargued-approval}}

**A decision approved with no argument is argued before it is recorded.** Before a head is written
from an approval that no argument for the decision, no rival and no cost stand behind, such as a
default approved in a batch, search for the arguments for and against it, its costs and its rivals,
and record what the search finds. When the search leaves the rivals equal, put the fork to the
owner as a tie. The owner's choice is then the head's ground, as a decision that came from the
owner, per `skill@knowledge-architect-decision-recording@head-shape`.

## Three homes, split by function {{slug:three-homes}}

{{fragment:three-homes.md}}

## What a head holds and how it is shaped {{slug:head-shape}}

### What a head stands on

{{fragment:head-ground.md}}

### What else a head holds, and how it is shaped

{{fragment:head-shape.md}}

## The current design {{slug:current-design}}

In the Component's design home, `register@*@design`: one file, or, for a Component whose design
has outgrown one file, a directory. In the directory shape the decision goes in the
subdocument owning its subject, never in the directory's `README.md`, which is the head and the
index. A new subdocument is linked from that index, `[title](file.md)`, the target relative to the
README, conventionally one bullet per subdocument.

{{fragment:subject-split.md}}

**What a head holds and how it is shaped is `skill@knowledge-architect-decision-recording@head-shape`.** The template:

```markdown
### <The decision, stated as a sentence> `##<slug>`

<The shape, present tense. Then the standing argument, per `skill@knowledge-architect-decision-recording@head-shape`.>
```

The slug is an identifier. Code comments, tripwires, other documents and `git log -G` all cite it,
so **renaming one means rewriting every reference in the same change.** Grep before you rename.

## Rejected designs {{slug:losing-alternatives}}

{{fragment:losing-alternatives.md}}

## Recording a premortem's tripwires {{slug:ruled-tripwires}}

%% Who records and when: `design@agent-skills@tripwire-recording-authority`.

A premortem names causes. **At the end of the work that implements the
decision**, the session judges each surviving cause by the standard below, on the wording the cause
has then: on the full path of `skill@knowledge-architect-design`, at the harvest; on its in-change
path, in the commit that records the decision. A parked thread's tripwire is judged the same way, and so is a tripwire anything else names, such
as a recurring acceptance criterion, written in the change that records its decision or in a later
one.
**It writes each cause that passes with the decision it guards**, in the tripwires home of the
Component that owns that decision, so the decision's head exists before the tripwire that names it,
and **lists every cause with its outcome**, a dropped one with the condition it fails, in its turn's
"Decided" section and in the record of the work: the harvest's commit message, or the in-change
commit message. No cause is put to the owner, who reads the work and contests any outcome with a
goal, a fact or an argument.

**A recorded tripwire's wording, scope and removal are the session's**, judged by the standard
below and listed under "Decided": it is reworded while it watches the same cause, and removed when a
condition excludes it, the commit naming the condition. A removal on the doubt's weight alone is the
owner's, since it rests on the owner's own weighing of what the owner would see. A parked thread
whose question the owner deferred records no decision: its cause is the trigger of a `deferred`
issue instead. The entry's lifecycle is `skill@knowledge-architect-issue-tracking`.

{{fragment:tripwire-standard.md}}

## Before you finish {{slug:before-you-finish}}

```sh
{{command}} check
```

Every reference you wrote must resolve.

Then re-read each head you wrote: present tense, as if the design had always been so (`skill@knowledge-architect-decision-recording@head-shape`).

Then apply the tests of `skill@knowledge-architect-decision-recording@what-earns-a-head` again to each decision of the change, and to each
head it writes or extends, now that every text of it is written: a site the change itself wrote counts, a skill or a restatement of the project's own
included. A decision judged while its texts were still being written misses the second site the
same change adds.

Then, for each head the change edits, whatever the edit, run `{{command}} show` on it and read
every text that cites it, and re-read the head's whole body, its argument included, against the
change; repair in the change each text it leaves false. A reference is a claim that its text is
revisited when the entry changes, and an edit that keeps the slug dangles no reference, so no check
reports a citing text it made false.

Then, for each attribution of the owner the change writes, read the owner's words beside it, from
the plan document or the transcript, and check that it keeps their scope and every hedge, as
`skill@knowledge-architect-decision-recording@head-shape` asks.

Then re-read what you wrote against the head you replaced: **rewriting argued text is where
fidelity gets lost.** If you cannot restate a rejected design as strongly as it was written, you
have not understood it well enough to move it.
