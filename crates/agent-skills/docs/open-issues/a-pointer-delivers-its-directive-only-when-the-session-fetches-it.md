---
kind: design
---
# A pointer delivers its directive only when the session fetches the section it names, and a copy filled by substitution would deliver it with no drift

## Summary

The installed text delivers a directive longer than one sentence by a pointer to its home, per
`design@agent-skills@restatement-size-test`. A pointer delivers nothing unless the session reads
the section it names at that moment. The owner proposes a rival: the text at the point of delivery
is a copy that the build or the install fills from the one home, so it is delivered where it is
needed and cannot drift. A second rival of the owner's is a harness hook that delivers the
pointed sections whenever a session reads a text holding a pointer to an instruction text. No real
session is established to have missed a pointed directive.

## Details

### What

**Instances seen: none established.** The owner has not seen with certainty a session miss a
directive that a pointer stood for, and is fairly certain that sessions do not always read the
pointed section again when they meet such a pointer. The owner's reasons, each with the owner's
hedge:

- as far as the owner knows, no instruction tells a session, everywhere, to read a pointed section
  again;
- agents tend to work from memory, especially of a text already read, though recent models seem to
  improve on this;
- as far as the owner knows, `tripwire@agent-skills@pointer-not-followed`, which watches this
  failure, is not watched closely: watching it would probably require reading session
  transcripts, the gap `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`
  records. An existing tripwire does not always mean the problem is watched.

The session's reading of the source, on the first reason: `primer@where-knowledge-goes` asks it
pointer by pointer, delivering a directive longer than one sentence "by a pointer to its home, with
an instruction to read the home whole at that moment". Not established: whether every pointer that
needs that instruction carries it, which the owner also doubts.

The owner also bounds the problem: in most cases, reading the pointed section is not important,
and working from memory is a defensible call that the room to judge, `primer@room-to-judge`,
allows; in some cases reading it again is necessary, some of which the text states, and the owner
is not certain it is asked wherever it is needed.

In the owner's rewording of the agent-skills goals, left out of the goal wording the owner
approved and kept here on the owner's word: restatements are avoided because they lead to drift
and conflicts, and they can be avoided by pointers or by text injected at several places by
substitution, at build or at install time.

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
- **A harness hook** that delivers the pointed sections whenever a session reads a text holding a
  pointer to an instruction text: a skill, an agent definition, the primer or the root
  `CLAUDE.md`.

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
  that never read the retrospective skill.

### Why it matters

A manual restatement that drifts until it contradicts its home fails
`goal@agent-skills@no-contradiction-in-workflow`. A pointer that is not followed leaves its
directive undelivered at the step that needs it, and nothing sees it, since its tripwire is read by
no occasion that holds a session's record. The entry strains
`design@agent-skills@restatement-size-test`, whose rival lost on drift alone.

### What would close it

A design discussion under `skill@knowledge-architect-design` on the delivery of a directive longer
than one sentence: keep the pointer, add substitution, add a hook, or a combination, and at which
sites. It ends in a reversal or a rewrite of `design@agent-skills@restatement-size-test`, or in the
owner's ruling that the pointer stays.

### Re-entry

That design discussion, proposed to the owner when work next reshapes how the installed text
delivers a directive, or when a retrospective or a review reports a directive missed behind a
pointer. Where that pointer had replaced a restatement under
`design@agent-skills@restatement-size-test`, the report also fires
`tripwire@agent-skills@pointer-not-followed`.
