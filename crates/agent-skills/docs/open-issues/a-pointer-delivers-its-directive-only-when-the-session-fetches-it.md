---
kind: design
---
# A pointer delivers its directive only when the session fetches the section it names, and a copy filled by substitution would deliver it with no drift

## Summary

The installed text delivers a directive longer than one sentence by a pointer to its home, per
`design@agent-skills@restatement-size-test`. A pointer delivers nothing unless the session reads
the section it names at that moment. The owner proposes a rival: the text at the point of delivery
is a copy that the build or the install fills from the one home, so it is delivered where it is
needed and cannot drift. A second rival of the owner's is a harness hook that delivers the named
sections whenever a session reads an instruction text holding a pointer. No real session is
established to have missed a pointed directive.

## Details

### What

**Instances seen: none established.** The owner has not seen a session miss a directive that a
pointer stood for with certainty, and is fairly certain that sessions do not always read the
pointed section again when they meet such a pointer. The owner's reasons:

- no instruction of the installed set asks, everywhere, that a pointed section be read again; the
  primer asks it pointer by pointer instead: `primer@where-knowledge-goes` delivers a directive
  longer than one sentence "by a pointer to its home, with an instruction to read the home whole at
  that moment". Not established: whether every pointer that needs that instruction carries it;
- agents tend to work from what they remember of a text already read;
- `tripwire@agent-skills@pointer-not-followed` watches this failure, and its firing evidence is in
  session transcripts that no occasion reads, the gap
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` records.
  A tripwire that exists is not a tripwire that is read.

The owner also bounds the problem: in most cases, working from memory of a pointed section is a
defensible call that `primer@room-to-judge` leaves to the session, and only some directives must be
read again at the moment they are applied.

**The suspected mechanism, in one sentence:** a pointer moves the cost of delivery from the
writer's text to the session's choice to fetch it, and the session chooses from memory.

**The rivals, both the owner's, recorded here as leads:**

- **A copy filled by substitution.** The build already fills placeholders in the shipped text, per
  `path@agent-skills@build.rs`: `{{snippet:<file>}}` inlines a file of
  `path@agent-skills@snippets/`, the delivery placeholders of its `SUBSTITUTIONS` table are
  replaced by their literals, and a level-two heading's slug placeholder is rendered into the
  heading. The install fills the project's command, `PLACEHOLDER` in
  `path@core@src/agents.rs`. What does not exist is a placeholder that copies a section of one
  installed text into another.
- **A harness hook** that, when a session reads a skill, an agent definition, the primer or the
  root `CLAUDE.md` holding a pointer to a section of an instruction text, delivers that section.

The questions a design discussion would answer:

- the size ground of `design@agent-skills@restatement-size-test`, the owner's, that restating a
  directive everywhere it is delivered loads unrelated text: a substituted copy answers drift and
  still loads the text, so the head's two grounds may split;
- the unit a substitution copies: a sentence, a paragraph or a slugged section, and how the checker
  reads a section whose slug would then be defined twice;
- which directives are the ones that must be read again at the moment they apply, since only those
  need a delivery other than a pointer;
- the first candidate sites: the pointers of the installed retrospective intake to the report's
  format in `skill@knowledge-architect-retrospective`, since the intake runs in a fresh session
  that never read the retrospective skill, once that intake ships.

### Why it matters

`goal@agent-skills@no-contradiction-in-workflow` is met only while no restatement drifts from its
home, and `goal@agent-skills@one-skill-per-activity` only while an agent can carry out an activity
from its skill, reading another only at a step that points to it. A pointer that is not followed
fails the second without anyone seeing it, and a manual restatement fails the first. The entry
strains `design@agent-skills@restatement-size-test`, whose rival lost on drift alone.

### What would close it

A design discussion under `skill@knowledge-architect-design` on the delivery of a directive longer
than one sentence: keep the pointer, add substitution, add a hook, or a combination, and at which
sites. It ends in a reversal or a rewrite of `design@agent-skills@restatement-size-test`, or in the
owner's ruling that the pointer stays.

### Re-entry

That design discussion, proposed to the owner when work next reshapes how the installed text
delivers a directive, or when a retrospective or a review reports a directive missed behind a
pointer, which also fires `tripwire@agent-skills@pointer-not-followed`.
