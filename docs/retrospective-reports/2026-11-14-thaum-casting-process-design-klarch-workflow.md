# 2026-11-14-thaum-casting-process-design-klarch-workflow

- **Received file**: 2026-11-14-thaum-casting-process-design-klarch-workflow.md, the workflow file of a retrospective of a project that installs the workflow.
- **Version used**: "knowledge-architect `0.10.0`, the exact version the project pins."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@search-apart-from-the-standing-state-review`: constrains, respect.
  - `design@agent-skills@verdict-report`: conflicts, check-reversal.
  - `design@agent-skills@verdict-states-a-conclusion`: constrains, respect.
  - `rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.
  - `design@agent-skills@design-home-is-built-intent`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `goal@the-workflow-improves-through-real-use`: constrains, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `goal@session-orchestrates-the-work`: constrains, respect.

## searcher-returns-summary-not-path: searchers return long summaries though their definition asks for a path and a line

Members: 2026-11-14-thaum-casting-process-design-klarch-workflow W2.

- **a. What it says.** In a project at 0.10.0, of 30 searchers over two searches, nearly every one returned a summary of its verdicts, many of 60 to 120 lines, beside the path and the line. Each arrived in the dispatching session's context, which also held the verdict files `--verify` writes with the same content. It blames `agent@knowledge-architect-record-searcher@what-to-return`: "Return the path of your report, the last line `--check` printed on it, or that it could not run, and every command you ran." It says: "The instruction is clear; the agents did not follow it." No text fix; the maintainers may weigh whether `--check` should print the one line the dispatcher needs.
- **b. Whether it still applies.** The text is as blamed: `git show v0.10.0:crates/agent-skills/content/agents/record-searcher.md` and origin/main hold the same section, lines 127 to 130, last in the definition, unchanged since 33f5c59d and a4543abf. `--check` already prints a last line the definition asks the searcher to return ("run it again until it prints `PASSED`").
- **c. Whether it is already recorded.** No entry records it. `design@agent-skills@behaviour-additions-need-real-use` (`constrains`) and `goal@only-needed-instructions` (`constrains`) bound any added instruction. The split-guidelines-plan-directories file of this repository says its 19 search reports "returned into the session's context as messages, a cost the dispatch shape imposes", which states no size and blames no text, so it is no second instance (it is consistent with returns of the asked size).
- **d. Its kind.** A lapse of the dispatched agents: the instruction exists, is clear, and was not followed. It is delivered last in the text the searcher loads, at the moment it returns, so no move of its delivery is available.
- **e. Its scope.** The installed searcher definition, Component `agent-skills`; `goal@session-orchestrates-the-work` (`constrains`), whose cost lands on the dispatching session's context.
- **f. A better fix.** Rivals: a sentence stressing "nothing else" in the section "what-to-return" (an emphasis, which the need test fails while the instruction already carries the act, per `skill@knowledge-architect-retrospective-intake@admission-rules`); a dispatcher's brief asking the same (a brief is the dispatcher's judgement, no text owed); no change. The lapse rule decides: a clear instruction not followed stays as it is, and one session reports it.
- **g. Its route.** Nothing.

Default: no change.
Outcome: decided by the session: no change, a lapse. Ground: `agent@knowledge-architect-record-searcher@what-to-return` states the return clearly at the moment of return; the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules` adds no text for a clear instruction not followed, and one session reports it, so no recurrence moves its delivery.

## fires-verdict-on-unbuilt-design: `fires` read as met by a design's planned code, and carried as a firing

Members: 2026-11-14-thaum-casting-process-design-klarch-workflow W1.

- **a. What it says.** At the grounding of a design discussion, "Searchers graded six tripwires
  `fires` because the draft's planned code would meet their clauses; but each clause names built
  evidence [...], which no design draft holds. The session carried 'fired' into the plan document's
  outcomes." Three review axes reported "a tripwire recorded as fired while its home is untouched,
  which the movement instruction does not allow"; the repair rewrote each outcome to "not fired; the
  slice that builds it reads it at its audit". The blamed text: the verdict table of
  `agent@knowledge-architect-record-searcher`, "`fires` | `tripwire` | a tripwire whose firing clause
  the work meets", which "can be read as 'the work, as planned, would meet' or 'the work, as it
  stands, meets'". Proposed: reword the row so that `fires` is given only where the evidence the
  clause names exists, planned code meeting it graded like `touches` (0 added, or +1 if a verdict is
  added).
- **b. Whether it still applies.** Yes. The row reads so at `v0.10.0`, the file's pin, and at
  origin/main, `v0.10.0-59-g605d1538`, line 73 of
  `path@agent-skills@content/agents/record-searcher.md`. Its action is `rule-before-work` (line 84),
  which loop step 1 of `path@agent-skills@content/skills/design/SKILL.md` gives as "ruled before the
  work is designed further or built: a firing by
  `skill@knowledge-architect-issue-tracking@movement-instruction`".
- **c. Whether it is already recorded.** The record reads the planned case as a firing, ruled at
  once, and gives the moves that follow:
  - `design@agent-skills@record-searched-before-the-work`: the audit "lists as a gap every standing
    entry the slice's code bears on: a tripwire whose firing condition, or a deferred trigger, the
    planned code meets"; the planning skill's audit says "A firing found at the audit is ruled before
    the code is written", in `path@agent-skills@content/skills/planning/SKILL.md`;
  - the design skill's `rule-before-work`, quoted in item b: a firing is ruled "before the work is
    designed further or built", so before any evidence is built;
  - the tripwire lifecycle, `path@agent-skills@fragments/tripwire-lifecycle.md`, shipped in the
    issue-tracking skill: "it fires: its heading and body are deleted, and an issue is opened or the
    decision is reopened, as its response says, unless the paragraph below keeps it", and that
    paragraph: "Where a fired tripwire's decision is reopened and reversed only at a plan document's
    harvest, the tripwire is narrowed instead of deleted, to except that plan document's work,
    citing it". Nothing in it asks for the evidence to be built before a firing;
  - `design@agent-skills@verdict-report` (conflicts against the proposal): a `tripwire` admits
    `fires`, `constrains` and `unrelated`, and "A verdict exists only where it routes an act", so
    planned code graded "like `touches`" needs a new verdict for a tripwire, which reverses part of
    that head.
- **d. Its kind.** Two things. The row is read two ways in real use: the searchers took "the work"
  as the draft, the planned code, as every row of the table does ("an issue the work would close");
  the session's repair took it as built evidence only. That is a defect of the text, unclear, by
  `design@agent-skills@no-two-installed-instructions-contradict` ("an instruction that a reading
  confirms is read two ways is repaired, even where every act it allows is acceptable"). The session
  carrying "fired" into the plan's outcomes with the tripwires left in their home is a lapse against
  the movement instruction, which the three axes caught; that instruction is clear.
- **e. Its scope.** The installed record searcher, owned by `agent-skills`. A rewording to the
  planned reading reverses nothing: it states the reading that
  `design@agent-skills@record-searched-before-the-work`, the audit and `rule-before-work` already
  hold.
- **f. A better fix.** The rival the file proposes, `fires` only on built evidence, contradicts the
  three texts of item c and adds a verdict against `design@agent-skills@verdict-report`; it loses on
  those texts. A second rival, an issue holding the act owed to a planned firing as undecided, loses
  because the record decides it: ruled at once, the tripwire deleted, or narrowed where the decision
  is reversed at the plan's harvest. Where the ruling keeps the decision, the guarded premise has
  been argued again against the planned evidence, and a doubt that remains is a premortem cause of
  the discussion, so no watch is lost. The fix is the row's wording, where the searcher grades.
- **g. Its route.** A fix in the installed text, at its source under `path@agent-skills@content/`,
  as `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says.
- **h. The proposal.** Handle now. In `path@agent-skills@content/agents/record-searcher.md`, line
  73 becomes:

  ```text
  | `fires` | `tripwire` | a tripwire whose firing clause the work meets, by what it builds or by what it plans to build |
  ```

  Instructions added: 0; removed: 0; one reworded. Nearest rival: the file's `fires` on built
  evidence only, which loses as item f says. The argument goes in the commit message, per
  `design@agent-skills@instruction-record-is-minimal`.

Default: handle now, with the rewording above.
Outcome: decided by the session: handle now, with the rewording above; the session's carrying of
"fired" without moving the tripwires is a lapse, caught by the review. Ground: item c; the audit's
gap list, `rule-before-work` and the lifecycle's narrowing paragraph read a clause met by planned
code as a firing ruled before the code exists, so the rewording states the record's reading, adds
no instruction and adds no verdict.

## standing-2026-11-14-thaum-casting-process-design-klarch-workflow: the standing answers

Members: the section "Standing questions" of 2026-11-14-thaum-casting-process-design-klarch-workflow,
lines 72 to 131, of a project that installs the workflow, pinned at 0.10.0. Its counts are in the
lines for `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`, its missed entry added
after the search in the lines for `issue@agent-skills@a-record-search-misses-what-is-added-after-it`,
and its tool reports are item 8 of the owner item in the analysis of
2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.

Carried by a subject, named only:

- a fork put to the owner with a cost of "about 6 more roadmap items" uncounted, where the roadmap
  held nine (lines 76 to 79): `owner-premise-unchecked`;
- "Searchers, beyond the primer: see W2" (line 85): `searcher-returns-summary-not-path`;
- the quote repairs of the searchers (line 90): `quotation-check-refuses-cut-punctuation`; the
  repoint by hand of every citation of the issue the plan replaces, "33 files ... 79 census rows",
  while `cargo klarch check` "lists each dangling citation ... but it writes none of them" (lines 91
  to 94): `named-repairs-applied-by-hand`;
- the "fired" outcomes carried from the searchers into the plan without a check (lines 113 and
  114): W1 of this file, `fires-verdict-on-unbuilt-design`;
- the head's sentence contradicted by a thread opened after the second search (lines 115 to 119):
  `thread-shape-change-owes-no-search`, and an instance in the lines above.

Analysed here:

- **Two slices named with one-word ids; the assembling agent renamed them when the checker refused**
  (lines 80 and 81), against the id grammar of `primer@checker-model`. Kind: a lapse the checker
  caught, as it is built to.
- **An assembly agent rewrote a reference inside a quote of an earlier session's transcript to the
  new entry; two reviewers found it** (lines 82 to 84), against `primer@where-knowledge-goes` on a
  record of a deliberation naming a removed entry by its bare slug. Kind: a lapse, caught by review.
- **Each of 30 search reports and 10 review reports relayed to the owner as it arrived, one short
  message each, "which spent context and gave the owner no decision to make"** (lines 97 to 100).
  Kind: a lapse of the session's own judgement; the file names the rule kept, "parallel evidence is
  reported as one round", for the rounds, not for these status messages. One session; no text
  blamed.
- **The assembling agent dispatched three agents of its own and returned before they finished, with
  nothing committed; resumed with their reports, it finished. "The brief did not ask it to wait for
  agents it dispatched"** (lines 101 to 105). Kind: a gap of one brief, the piece returned
  unfinished being "a new piece, which the session judges", per `primer@dispatching-work`; it was
  acted on without redoing, so the report-file trigger of
  `issue@agent-skills@dispatched-agents-write-their-report-as-they-work` is not met.
- **A rules reviewer's claim, that six trailing elision marks were wrong, put into a repair brief as
  a ruling without a check; the repair agent's checker run showed it wrong** (lines 110 to 112).
  Kind: a lapse against `primer@dispatching-work`, "A conclusion that comes back gets an independent
  verification before the session acts on it", which the repair agent's run supplied. **A searcher's
  claim that a worked example did not exist**, checked against the pinned text and dropped (lines 108
  and 109): the same rule followed.
- **Installed file changed**: no. **Project skill addition**: no. **Backticks**: no. **Code rules**:
  no Rust beyond two doc comments. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: lapses against clear rules, each caught by the
checker, a review or a later agent, with no recurrence at one point of delivery in the batch; the
rest are observations.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## thread-shape-change-owes-no-search: a part of the work that changes after the search, with no new thread, gets no search

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md`, in its section on `thread-shape-change-owes-no-search`, which gives the outcome.

## whole-record-search-for-narrow-work: a search of the whole record runs for a narrow or follow-up piece of work

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow.md`, in its section on `whole-record-search-for-narrow-work`, which gives the outcome.

## named-repairs-applied-by-hand: the checker names each repair of a reference, and sessions apply them by scripts

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `named-repairs-applied-by-hand`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.

## owner-premise-unchecked: questions put to the owner on premises the session had not checked

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-08-knowledge-architect-quick-fixes-klarch-workflow.md`, in its section on `owner-premise-unchecked`, which gives the outcome.
