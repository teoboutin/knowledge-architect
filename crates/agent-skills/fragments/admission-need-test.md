%% Delivered as a fragment: `design@agent-skills@admission-rules-home`.
Every statement in agent-facing text passes one test: **name the act of an agent following the
workflow that would differ without it**: what the agent does, when, where, or a fact it must use.
A statement for which no such act can be named is not written, and an existing one leaves as the
table below gives. The
unit is a sentence, or a clause when only part of a sentence fails.

- **Whose act.** Any agent that reads the text in its work counts, an agent that maintains the text
  or writes a brief from it included. A statement whose only reader is the text's maintainer is at
  most one sentence; in installed text it is a comment of the workflow's source, removed by the
  build, since no installing project maintains the installed text.
- **A reason** passes when it states the mechanism an instruction relies on or the failure it
  prevents, even when the instruction has no exception. A second reason for the same instruction
  passes only when it names another mechanism or failure. A sentence that narrates where an
  instruction came from or how it turned out fails: "learned by getting them wrong", "and it
  worked".
- **A duplicate inside one text fails while its twin stays.** A text is read whole once it is
  loaded, so a second statement of the same act adds none. The finding names the copy that stays:
  the one in the section whose subject it is. The primer and the root `CLAUDE.md` that imports it
  are one text, since both are loaded together, and the primer's copy is the one that stays. Across
  other texts, `primer@where-knowledge-goes` decides.
- **A lead, a transition or an emphasis** fails when the statements it introduces or stresses
  already carry every act. A heading is structure, not a statement.
- **A remark about the text itself** fails unless it tells a reader where to look or what to edit:
  what the text is classified as, or that something is not restated here.
- **A definition** passes when an instruction anywhere in the workflow uses the term. **An example**
  passes when it shows how a definition or an instruction applies.
- **A pointer is needed** when its target is needed to act, or when it names the home of a
  restatement.
- **A statement that can turn false with no change to the project** (the state of another project,
  a tool's version, a person's plan) is suspect: it is written only when an act needs it. "This
  issue was reported upstream" fails, since no act differs.

Wording and style follow `primer@working-prose`; placement across files is judged by other rules.

Current reality only: no history, no dates, no commit hashes, no session records. Examples are kept
minimal.

**The outcomes of a need-test failure.**

| the failing statement | outcome |
| --- | --- |
| it carries no instruction: narration, a remark about the text, outside state no act uses, a duplicate whose twin stays, a lead or an emphasis whose statements carry every act | deleted in the branch: by the session, which applies an audit's outcomes, or as a review's repair |
| it is maintainer-only, in installed text | moved to a comment of the workflow's source |
| it is maintainer-only, in a project's own text, and longer than one sentence | cut to one sentence |
| it is an instruction whose act no agent performs | a decision, since deleting it changes what agents are told, and whether its act is still wanted is the intent of whoever wrote it, which the project's record may not hold |
| a suspect statement that passes | no finding; a suspect statement found false is a factual error |

In a project that installs the workflow, a finding on installed text, of any row, is sent upstream,
as every finding on installed text of an audit is.
