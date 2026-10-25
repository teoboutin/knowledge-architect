---
kind: todo
---
# The standards and large blocks of instructions read whole at several places are still delivered by pointers, not yet by fragments

## Summary

A standard, or a similar large block of coherent instructions, read whole at several places is
delivered by a fragment the build copies into each text that applies it, per
`design@agent-skills@shared-text-fragment` and `design@agent-skills@restatement-size-test`. The
tripwire standard and its lifecycle are the first. The owner ruled that the others "will be
migrated later": they are still delivered by a pointer read whole, which the session may not fetch.

## Details

### What

The missing work is the migration of the other such blocks of the installed text to fragments,
each where its act applies the whole block, with the host list of the test
`each_fragment_has_its_hosts` of `path@agent-skills@src/lib.rs` extended. Candidates, to be judged
each by whether several texts apply it whole:

- the report format of `skill@knowledge-architect-retrospective`, which the installed retrospective
  intake points to and reads in a fresh session that never read the retrospective skill;
- the admission standard of `skill@knowledge-architect-agent-configuration@admission-standard`, which
  the agent-text reviewer, the workflow auditor and the retrospective intake each point to;
- the rules on design heads of `primer@design-heads`, which the decision-record reviewer and the
  design-record auditor point to; the primer reaches every session, so whether a copy in an agent's
  definition is needed is the question to judge.

The owner's reasons for the fragment are in the head: a pointer moves the cost of delivery from the
writer's text to the session's choice to fetch it, and the session chooses from memory. No real
session is established to have missed a pointed directive.

### Why it matters

A block delivered by a pointer is applied from memory where the session does not fetch it, and a
hand-written restatement drifts from its home, against `goal@agent-skills@no-contradiction-in-workflow`.
Each block left behind a pointer keeps that cost where a fragment would remove it.

### What would close it

Each candidate judged: migrated to a fragment with its hosts, or kept behind its pointer with the
reason; the migrated ones listed in the hosts test.
