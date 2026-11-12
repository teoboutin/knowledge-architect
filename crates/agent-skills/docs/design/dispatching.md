# Agent skills design: Dispatching work

## Dispatching work

### A session divides the work between itself and the agents it dispatches by a judgement on how the pieces relate, and an instruction fixes a dispatch only on a property of the step that holds whatever the task's size `##work-divided-by-judgement`

`primer@dispatching-work` states the judgement and the purpose it weighs toward: the main session
dispatches so that its context stays for what only it can do, the design, the decisions and the
exchange with the owner, while agents carry the rest, and an agent that dispatches weighs the same
way for its own piece. The judgement is built on two relations between pieces of the
work: one carrier, the session or one agent, takes two pieces that reason over the same details;
separate carriers take two pieces when one judges the other, or when neither needs what the other
reads and splitting them saves context. The question whether the session or an agent carries a
piece is the case of the first piece the session holds: the session dispatches a piece whose brief
can carry what it needs and whose result alone the session needs, carries one whose details it will
reason over later or whose brief would cost about as much as the work, and weighs what lies
between, as implementing a design does. A dispatched piece is what its brief gives the agent, and
the agent carries all of it. Point 3 of `skill@knowledge-architect-planning@working-a-slice` points
to the section, where implementation starts.

An instruction fixes a dispatch at a step only on a property of the step that holds whatever the
task's size. The admission standard,
`skill@knowledge-architect-agent-configuration@admission-standard`, states the rule, per
`design@agent-skills@admission-rules-home`, so it binds a project's own skills as it binds the
installed text, and the primer says only that a dispatch a skill or an agent fixes is made as that
text says. The rule's list of properties is open, each one a property of the step whatever the
task's size; the properties the installed text uses today are three: a point of view apart from the
session's (the reviews, the goal reviewer, the audits, whose agents judge the project by a standard
and are judged again by their verifiers); an output whose form a command checks (the record search,
through `klarch worklist --verify`; the transcript extraction at a plan's assembly, whose lines a
command, `klarch transcript show`, writes, which fixes their form); and an occasion most work meets (a merge, a development
checkpoint). An output verified by a session other than its author is no such property: under
`design@agent-skills@at-least-one-independent-verification` every returned conclusion is, so it
would admit any dispatch. Any other "always dispatch X" is a size claim in disguise.

The standing argument:

- **It serves `goal@session-orchestrates-the-work`**, under which the main session orchestrates
  agents that carry the work. The goal fixes no when, no kind and no size for a dispatch; this head
  reads it as settled by a judgement with named exceptions, against a threshold. The section states
  the purpose, since a session judges a division better knowing what it serves.
- **A judgement, not a threshold of size or kind.** Size varies without bound, and a threshold is
  a rule per interaction, which `design@agent-skills@capabilities-not-structure` argues against. A
  relation between pieces, not a list of cases, judges a pairing nobody anticipated, as
  `goal@only-needed-instructions` asks: a case that judgement handles gets no instruction.
- **The harness's default, in doubt, is not to dispatch.** Claude Code's Agent tool description
  weighs both sides, read in the tool list of a session running 2.1.321 and found again in the
  2.1.322 binary: "Reach for this when you have independent work to run in parallel,
  when the user asks for a side quest that shouldn't block your main thread, or when answering would
  mean reading across several files", and "Do the work yourself when it is a handful of tool calls …
  When in doubt, don't spawn."; `claude --version` names the version a
  session runs. No installed text weighed toward dispatching. The section is a weighing beside that
  default, not a precedence over it. The owner reports that the main session nearly never dispatched where the owner believes it
  could have been useful, and spent context that should probably have been kept for the design
  discussion; that in the session that drafted the goal, the
  owner's leads were applied and the session ended with context to spare; and that since, dispatch
  happens occasionally, not reliably. That report is the evidence of real use
  `design@agent-skills@behaviour-additions-need-real-use` asks. The nearest rival, no text and
  `primer@room-to-judge` alone, loses on it.
- **The primer is its home**, since a session meets the question in every activity and no skill
  loads at that moment, per `design@agent-skills@primer-limit`. The rival, guidance in each
  activity's skill, the development skill's included, restates the judgement in each and reaches
  only the sessions that load it. A dispatched developer loads the development skill through the
  primer's list of installed skills, and sends that skill's checkpoint reviews itself. The primer
  can dilute in a long session, per `rejected@agent-skills@head-rules-whole-in-primer`; the case
  the owner saw fail, implementation, starts at a known point, which gets one sentence and its
  pointer, per `design@agent-skills@restatement-size-test`.
- **The two contexts are not worth the same.** The main session's context holds the exchange with
  the owner, which no brief rebuilds, while an agent's is rebuilt by dispatching it again. Every
  later turn costs more as the context grows: the Messages API is stateless, and each request
  carries, and is billed for, the whole conversation, cached or not. The readings:
  https://platform.claude.com/docs/en/build-with-claude/working-with-messages says "The Messages API
  is stateless, which means that you always send the full conversational history to the API.",
  beside message threads, a beta in which the API keeps the conversation; and
  https://platform.claude.com/docs/en/build-with-claude/prompt-caching prices a cache read at "10
  percent of base input token price", with per-model exceptions. A fresh agent first reloads its
  instructions, measured at about 36,000 tokens over 40 searchers' transcripts in
  `rejected@query@one-search-agent-per-anchor`, which grounds the clause on a brief that costs as
  much as the work.
- **A dispatched piece is carried whole.** Read at the size of one step, the bullet on what the
  session carries, and the harness's own default, would have the session take over a dispatched
  agent's commit. Fixing the unit at the brief's piece leaves a partition between agents possible:
  a brief that tells its agents not to commit leaves the commit outside their pieces. What an agent
  returns unfinished is a new piece. How parallel work is partitioned among agents is
  `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`.
- **The rule on fixed dispatches tests a property of the step.** Without the test, the fixed
  dispatches are a list, which a later writer extends by analogy, and the threshold comes back.
  Every fixed dispatch of the installed text fits one of the three properties above.
- **The facts on the harness** are measured in Claude Code 2.1.321, once each: a general-purpose
  subagent holds `Agent` and can dispatch a nested agent, and holds no `Workflow`, so a saved
  workflow stays with the main session; and the primer reaches a nested agent. The probe that takes
  each again: one general-purpose agent asked to list its tools and to dispatch one nested agent
  that quotes a sentence of the primer from its initial context, with no file read.

The retrospective watches this decision with two of its standing questions, per
`design@agent-skills@premortem-as-watch-points`: work the session carried that an agent could have
carried, and an agent that came back unfinished or with a report the session could not act on.
A token bound per agent in the primer lost: `rejected@agent-skills@dispatch-size-hint`.

### A claim the work acts on gets at least one independent verification before it is acted on, by a session other than its author, against the evidence its report carries, and one is enough `##at-least-one-independent-verification`

An independent verification is the verification of a claim by a session other than the one that
wrote it: the dispatching session, or an agent it dispatches. It stands for the dispatching
session's own verification of that conclusion, which `primer@verify-before-relying` asks before
acting on it. A command that checks a report's form, such as `klarch worklist --verify`, is none. A
verifier's verdict on a claim is no new claim; the claims a verifier adds of its own, such as a
repairer's account of what it repaired and what it did not, are, and get their own. The primer
states it in `primer@dispatching-work`, per `design@agent-skills@verification-in-primer`.

**One is a minimum, not a ceiling.** It forbids no further verification a skill asks for, with its
reason: the audit's session verifies findings its verifiers already verified, and samples their
refutations, per `design@agent-skills@one-run-pass`. So each instance below keeps its chain:

- the record search's verdicts, which the session acts on, per
  `design@agent-skills@session-acts-on-the-verdicts`, and which the design-conformance reviewer
  reads again, per `design@agent-skills@verdicts-reach-the-conformance-review`;
- an audit's drafts, verified inside the run by a verifier, per
  `design@agent-skills@verifier-in-the-run`, then by the session, per
  `design@agent-skills@one-run-pass`, the agentic-workflow axis's findings read again by its
  re-check, per `design@agent-skills@workflow-axis-lenses`;
- an audit's repair proposals, read by a check step before the session verifies them, per
  `design@agent-skills@repair-proposals-audited`;
- a reviewer's findings, which the dispatcher verifies before relaying or acting on them, as the
  installed review skill says, and a plan review's findings, as the planning skill says.

Each claim gets its one independent verification, a verdict is no new claim, and the main session's
own claims are verified by the review before the merge, so no chain is left open.

The standing argument:

- **Self-verification lapses often.** The primer asks every session to verify its own claims
  before writing them. The owner reports that it is the kind of instruction whose lapse is common,
  which is why a session other than the claim's author verifies it. A measurement of how often
  another instruction lapsed in real sessions agrees: dispatchers saved condensed copies of reports,
  or none, in 7 of 12 sessions, per `design@agent-skills@reviewer-writes-its-report`.
- **One verification weighs the same whoever makes it.** The owner holds that no more trust is due
  to the main session than to a subagent, so one verification by a session other than the author is
  enough. A verification may therefore be dispatched; where its reading serves the repair, as for
  review findings, one carrier takes both, by the relations of `design@agent-skills@work-divided-by-judgement`.
- **It earns a head by test 2**: the same reason is respected at the sites listed above and in the
  primer section.
- **The boundary is a claim the work acts on.** A claim whose effect is that nothing is done, a
  refuted finding, may be sampled where a skill says so: a wrong confirmation lands an edit, where a
  wrong refutation loses a finding nothing applies, which the audit's next pass and
  `tripwire@agent-skills@verifier-refutes-a-true-finding` watch. The session chose this boundary over
  verifying every refutation, which no goal ranks; the owner approved it, judging that it matters
  most where a claim is acted on, and leaving open whether a finding judged real and not worth
  acting on deserves verification. Such a finding is a decision, which `primer@decided-section`
  brings to the owner.

The retrospective watches this decision with its standing question on a returned conclusion that
proved wrong after its independent verification, or was verified again where no skill asked, per
`design@agent-skills@premortem-as-watch-points`. The wording that made the rule a ceiling lost:
`rejected@agent-skills@a-check-is-not-checked-again`.
