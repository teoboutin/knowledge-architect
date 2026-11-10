# The goal standard: how other fields state and review high-level goals

Findings established outside this project that the design of the goal standard leans on: the rules of
`skill@knowledge-architect-goal-setting@goal-or-not`, argued in
`design@agent-skills@goal-standard-rules`, and the way `agent@knowledge-architect-goal-reviewer`
grades its findings, and the way goal-setting stops its reviews, argued in
`design@agent-skills@goal-finding-grades`, `design@agent-skills@goal-review-stops-on-old-wording`
and `design@agent-skills@whole-set-rounds`. Each finding names its source and how it was read:

- **read**: the passage was read in the primary text;
- **secondary**: it was read in a summary or a quotation, not in the primary text;
- **abstract read**: only the abstract of the source was read;
- **assumed**: it is recalled, and no source was read for it.

A finding read secondary or assumed is a lead, and is read at its source before an argument rests on
it alone.

**The revision of the standard rests first on this project's own record**, argued in the heads
above. On the owner's word, the proposals of the discussion that made that revision weighed more
than these findings, and a finding changed one of them only by an argument the owner weighed.

## The questions

1. How is a goal stated so that it stays stable while the work moves toward it?
2. How does a goal stay unambiguous without being narrowed?
3. How is a goal met in what the project builds told apart from a goal judged only in use?
4. When do two goals conflict?
5. How does a review of a set of goals stop?

## A goal is stable because it names no way of achieving it

- **KAOS: the higher a goal, the more stable it is.** A. van Lamsweerde, "Goal-Oriented Requirements
  Engineering: A Guided Tour", RE'01, 2001, https://webperso.info.ucl.ac.be/~avl/files/RE01.pdf
  (read): "A requirement represents one particular way of achieving some specific goal; the
  requirement is therefore more likely to evolve, towards another way of achieving that same goal,
  than the goal itself. The higher level a goal is, the more stable it will be." Goals are found
  upward by asking why of a description of what is built, and refined downward by asking how.
- **i\*: a goal leaves the means to whoever pursues it.** E. Yu, "Towards Modelling and Reasoning
  Support for Early-Phase Requirements Engineering", RE'97, 1997,
  http://www.cs.toronto.edu/pub/eric/RE97.pdf (read): in a goal dependency, "It is up to the
  participant how to attain that goal"; a task dependency is the form that fixes the means.
- **Commander's intent: purpose and end state, method only to clarify.** US Joint Publication 3-0
  (2010), quoted in "Intent (military)", Wikipedia (secondary): intent is "a clear and concise
  expression of the purpose of the operation and the military end state", and lets subordinates act
  "without further orders, even when the operation does not unfold as planned" (US FM 100-8 and later
  doctrine, secondary). NATO AJP-01 (2010, secondary): how the force achieves the results "is
  provided only to clarify".
- **The "what, not how" boundary is relative to a level.** A. M. Davis, *Software Requirements:
  Objects, Functions and States*, 1993 (secondary): "one person's 'how' is another person's 'what'".

These support a rule that a goal keeps its meaning in a project emptied of its design and its
code: a goal tied to a part as built changes when that part is redesigned, which is the instability
the sources place in requirements and plans, never in goals. A statement about one part of the
project, or one grounded on another goal, is a "how" of the goal above it, in the same sources'
terms, which the standard records as a design head that cites that goal, by its rules 1 and 8.

**Where a source goes against it.** Two sources contradict a reading in which a goal never changes:

- W. Swartout and R. Balzer, "On the Inevitable Intertwining of Specification and Implementation",
  CACM 25(7), 1982 (abstract, secondary): what is built legitimately changes and adds to the
  specification;
- military doctrine expects intent to change between the phases of a long operation ("Intent
  (military)", Wikipedia, secondary).

The rule judges dependence on a design choice, not immutability, so neither contradicts it.

D. M. Berry, "What, Not How? When Is 'How' Really 'What'?", University of Waterloo,
https://cs.uwaterloo.ca/~dberry/what.not.how.html (abstract read): "In several specific domains, it
may be necessary to say how to make the what precise enough". This is weak evidence, an abstract with
no worked case. Rule 2 of the standard admits a kind of interface as an outcome where it passes the
purpose test of rule 1.

## Unambiguous is not narrow

- **ISO/IEC/IEEE 29148:2018 keeps two characteristics apart** (secondary, as summarised in Table I of
  S. Lubos et al., "Leveraging LLMs for the Quality Assurance of Software Requirements", arXiv
  2408.10886, 2024, https://arxiv.org/pdf/2408.10886):
  - *unambiguous*: "allows only one interpretation";
  - *appropriate*: "The level of abstraction is adequate, excludes unnecessary constraints, and avoids
    implementation details".

  A wide statement at a high level meets both, so the standard does not equate one reading with a
  narrow scope.
- **A softgoal is a legitimate goal with no clear-cut criterion.** RE'01 (read), on the NFR framework
  of J. Mylopoulos, L. Chung and B. Nixon (IEEE TSE 18(6), 1992): softgoals "can rarely be said to be
  satisfied in a clear-cut sense", and subgoals are "expected to achieve the parent goal within
  acceptable limits, rather than absolutely".
- **Who judges "enough" is the one who depends on the goal.** RE'97 (read), softgoal dependency: "It
  is up to the depender to decide what measures are enough". On this project's terms, the extent of a
  goal is the owner's judgement, never the reviewer's.

These support a rule of one direction that judges ambiguity, two readers moving in different
directions, and never breadth, by rule 3 of the standard. They also support a finding on the reach of a goal that asks the
owner, and does not narrow the goal by default.

**What the sources add on narrowing.** KAOS lists "goal weakening" among the ordinary resolutions of
an obstacle (RE'01, read): first sketches of goals "are often too ideal". The field narrows a goal as
the resolution of an identified obstacle, a condition the domain makes possible, never as a default
repair of wording.

## A goal judged in use is shown met by what learns from use

- **KAOS sorts goals by who can enforce them.** A. van Lamsweerde and E. Letier, "From Object
  Orientation to Goal Orientation", Monterey'02, LNCS, 2003,
  https://webperso.info.ucl.ac.be/~avl/files/avl-MontereyLNCS.pdf (read): goals are about "the
  software-to-be together with its environment". A goal under one software agent is a requirement.
  A goal under an agent of the environment is an expectation: "Unlike requirements, assumptions cannot
  be enforced by the software-to-be" (RE'01, read).
- **Verification and validation are two activities.** ISO 9000:2015 (assumed): verification confirms
  that "specified requirements have been fulfilled"; validation, that "the requirements for a specific
  intended use or application have been fulfilled". 29148's requirement that every requirement be
  *verifiable* (secondary) is the pressure that pushes a high-level goal toward an observable of the
  tree. Validation is the standard's own way out.
- **What is built can collect evidence from use.** V. E. S. Souza, A. Lapouchnian, W. N. Robinson
  and J. Mylopoulos, "Awareness Requirements for Adaptive Systems", SEAMS 2011 (secondary): these
  are requirements about "the runtime success/failure/quality-of-service of other requirements",
  operated "through a feedback loop". Runtime monitoring of assumptions in KAOS is the same idea:
  M. Feather, S. Fickas, A. van Lamsweerde and C. Ponsard, IWSSD 1998 (secondary).
- **A measure taken as the goal replaces it.** D. Manheim and S. Garrabrant, "Categorizing Variants of
  Goodhart's Law", arXiv 1803.04585, 2018 (abstract read): "The importance of Goodhart effects depends
  on the amount of power directed towards optimizing the proxy". Agents working toward a goal for a
  long time apply that power. W. Choi, G. Hecht and W. Tayler, The Accounting Review 87(4), 2012
  (secondary): "surrogation" is the tendency to "lose sight of the strategic construct(s) the measures
  are intended to represent". In their report, several measures reduce it.

These support that an outcome judged only in use is shown met by what the project builds to pursue
it and to learn from its use, never by a list of the properties it is taken to consist of. The
standard states no measure in a goal, by its rule 7, so a design head that says how a goal is shown
met is what these findings bear on. They also support a test such a head can apply to sort the
outcome it pursues: **does it depend on
anything the project does not control, such as its users, their data or their environment?** KAOS's
sort by who can enforce a goal gives this wording. It states more directly than a sort by kind of
outcome the case of a project whose effects land in its users' environment. A correctness outcome
that depends on what a user supplies is then judged in use, as KAOS sorts it.

**A separate, revisable layer of measures.** Google re:Work, "Set goals with OKRs",
https://rework.withgoogle.com/en/guides/set-goals-with-okrs (read): "There are some objectives that
will stay the same quarter after quarter", while "the key results should evolve". Gilb's Planguage,
*Competitive Engineering*, 2005 (assumed), separates an ambition from its scale and meter. Both keep
the stable statement and its measure in two texts with two lifetimes, the measure replaced without
changing the goal. The standard keeps the measure in a design head that cites the goal, by its
rule 7.

## A goal is expected to be unmet

re:Work (read): "success means achieving 70% of the objectives", and a team that always reaches 1.0
has objectives that "aren't ambitious enough". It warns against "business-as-usual" objectives built
on what a team already does. This supports the reading that a new or reworded goal is drafted to
move what the project builds, never shaped so that the tree as it stands meets it.

## When two goals conflict

- **Logical conflict.** RE'01 (read): goals conflict when no behaviour meets them all under the domain.
  A behaviour a goal does not constrain is unconstrained. So a goal that rules out what another goal
  only admits is no conflict: a state without that behaviour meets both. This agrees with rule 6 of the
  standard, "No state of the project meets one goal only by failing another", under a reading in
  three layers: what a goal requires, what it admits, and what it rules out.
- **Divergence under a boundary condition.** RE'01 (read): goals G1 to Gn are divergent when a
  non-trivial condition B, possible in the domain, makes them jointly inconsistent, while every proper
  subset stays consistent. Logical conflict is the special case. A divergence is two outcomes that
  compete under a condition, which rule 6's ranking sentence covers: "Where two purposes compete,
  including two that can both be met except under a condition the project or its use makes
  possible, a ranking states which comes first". So a divergence is a question for the owner, with its condition.
- **Graded contributions.** The NFR framework grades how a design or a goal contributes to another:
  make, help, hurt, break (L. Chung, B. Nixon, E. Yu, J. Mylopoulos, *Non-Functional Requirements in
  Software Engineering*, 2000, secondary). A "hurt" is a trade-off that a design weighs, argued in its
  head. It is neither a conflict between goals nor an exclusion.
- **Antigoals and accepted risk.** G. Klein, *Sources of Power*, 1998 (secondary), lists antigoals,
  outcomes to avoid, among the parts of an intent, and JP 3-0 (secondary) has the intent state "where
  the commander will accept risk". Antigoals are what a goal rules out. A cost the owner accepts in
  pursuing a goal is a ranking between two purposes, by rules 1 and 6, or a local cost a design head
weighs. The
  standard needs no fourth layer for it.

## How a review of a goal set stops

Gilb's Specification Quality Control (secondary, articles on methodsandtools.com and
modernanalyst.com) checks a sample page of 300 words and counts only major defects. A specification
exits when the estimated density of remaining majors falls under about one per page. By Gilb's data,
one pass finds about 30% of the defects. The source supports a stop rule that counts only the
findings that change what is built, set in advance. If that rate transferred to goal reviews, which
is an assumption, one round with no blocking finding would be weak evidence that none remain.

This project's own measurement of the mechanism behind a review that does not stop, the repairs of
each pass writing the text the next pass finds defects in, is recorded for its audits in
`design@agent-skills@convergence-count-scope`.
