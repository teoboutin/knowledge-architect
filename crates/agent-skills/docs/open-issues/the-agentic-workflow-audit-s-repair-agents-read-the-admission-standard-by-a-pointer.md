---
kind: todo
---
# The agentic-workflow audit's cluster, merge and repair agents read the admission standard by a pointer

## Summary

The admission standard, `skill@knowledge-architect-agent-configuration@admission-standard`, is
delivered as fragments to every actor that applies it, per `design@agent-skills@admission-rules-home`.
The saved workflow of the agentic-workflow axis dispatches its cluster, merge and repair agents as
general-purpose agents, whose prompts send them to the standard's section for how a repair is
shaped. They read it only if they open the skill's file.

## Details

### What

In `path@agent-skills@content/workflows/agentic-workflow-audit.js`, the cluster, merge and repair
agents carry no `agentType` (lines 188, 201, 216 and 264 at the commit that opened this entry), and
their prompts' `KINDS` (lines 90-93) names "the section admission-standard of the skill
knowledge-architect-agent-configuration" for how a repair is shaped. A prompt of a saved workflow
is a JavaScript string, where a fragment's placeholder line cannot stand, so the fragments do not
reach them.

### Why it matters

`design@agent-skills@context-cost-of-agent-text` counts the pointer as a cost: each agent spends the
lookup, and may shape a repair from memory instead. A repair shaped against another reading of the
standard than the auditors' and the session's is the disagreement
`goal@agent-skills@no-contradiction-in-workflow` asks to avoid.

### What would close it

The repair step dispatched as an agent type that hosts `path@agent-skills@fragments/admission-repair-shape.md`
and `path@agent-skills@fragments/context-cost.md`, an existing one such as the workflow auditor
under a brief of its own, or a new one, argued against `design@agent-skills@root-cause-repair`,
which keeps those agents general-purpose; or a recorded ruling that the pointer stays, with its
reason.
