---
kind: defect
---
# The project-audit skill says a harness may list saved workflows only when a session starts, which two observations contradict

## Summary

The installed project-audit skill, `path@agent-skills@content/skills/project-audit/SKILL.md`, says:
"A harness may list saved workflows only when a session starts, so a session that ran the install
later may not see the workflow listed; a session whose harness does not list the workflow
dispatches its steps by hand". Two sessions saw the harness list a saved workflow installed after
they started, and none saw the contrary. `design@agent-skills@audit-axis-runs-a-workflow` keeps the
same by-hand case, "A session whose harness does not list the workflow dispatches its steps by
hand".

## Details

### What

The sentence sits in the section on the method of an audit, in the paragraph that says each axis
runs each pass as one run of its saved workflow. It states a behaviour of Claude Code that no
measurement supports, and it gives the case it derives, a harness that does not list the workflow,
the by-hand line. The observations, both under Claude Code:

- In the session of the commit "[audit] Pass 4 of the agentic-workflow axis, run through the saved
  workflow: 36 findings confirmed, 23 causes repaired", a new workflow was listed right after its
  install; that commit's message says so, and says it softened the sentence to "may" for that
  reason.
- In the session that built the commit "[agent-skills] The record search's saved workflow,
  `knowledge-architect-record-search`, with a `node` test of its flag builder", after
  `cargo klarch install-agent-skills` wrote
  `path@agent-config@workflows/knowledge-architect-record-search.js` mid-session, the harness listed `knowledge-architect-record-search` with its `meta` description
  among the session's available skills, in a skill listing the session's transcript holds after the
  install's output.

Not established: whether some other version of Claude Code, or another way of adding the file than
the install, lists saved workflows only at a session's start. No session was observed doing so.

The record search's by-hand line was cut to its three cases for this reason, in
`design@agent-skills@fallback-dispatch-line`, which records both observations.

### Why it matters

Every project that installs the workflow reads the sentence. A session that ran the install
mid-session is told it may not see the workflow, and may dispatch an audit's steps by hand where it
could run the workflow, losing what `design@agent-skills@audit-axis-runs-a-workflow` puts in the
script. If the premise is false, the case it derives serves no state of `claude`, and
`goal@serves-one-kind-of-project` asks nothing built for another harness.

### What would close it

The probe, a run of the harness's mechanics that `design@agent-skills@synthetic-evidence-not-built`
admits: a session that runs the install of a workflow it did not start with, then reads the
workflows its harness lists. Then the skill's sentence and the by-hand case of
`design@agent-skills@audit-axis-runs-a-workflow` corrected to what it shows: removed where the
harness lists the workflow, or the sentence stated as measured where it does not.
