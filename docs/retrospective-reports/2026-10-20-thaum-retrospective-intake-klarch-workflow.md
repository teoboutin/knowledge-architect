# 2026-10-20-thaum-retrospective-intake-klarch-workflow

- **Received file**: 2026-10-20-thaum-retrospective-intake-klarch-workflow.md, the workflow file of
  the first run of `skill@knowledge-architect-retrospective-intake` in a project that installs the
  workflow: nine project files taken together, their outcomes carried out, and a review before the
  merge, with the owner asking that the session watch the intake process itself.
- **Version used**: the published checker, 0.8.0, read at the tag `v0.8.0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`. The intake skill and every other installed
  text this file blames but the setup skill are identical at both; every intake sentence it blames
  entered before the tag.
- **Standing entries that bear on it**:
  - for W1, W12, W13 and W14: `issue@agent-skills@a-batch-intake-reads-every-returned-standing-entry-whole`,
    whose re-entry point is the next change to the intake's grounding;
    `design@agent-skills@standing-entry-search-agent`, `design@agent-skills@search-relevance-grade`,
    `design@agent-skills@design-entry-search`, `design@agent-skills@committed-intake-analysis`,
    `tripwire@agent-skills@design-entry-search-misses`, not met: no miss is reported;
  - for W2: `design@agent-skills@installed-fix-goes-upstream`,
    `design@agent-skills@retrospective-destination`;
  - for W4 and W5: `design@agent-skills@intake-shares-the-admission-standard`,
    `design@agent-skills@owner-word-holds-as-far-as-its-premise`,
    `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`,
    `goal@knowledge-architect@the-owner-s-word-is-verified`;
  - for W8: `design@agent-skills@design-entry-search-groups`, `design@core@heading-register-two-shapes`;
  - for W15: `issue@agent-skills@the-void-marker-s-reason-reads-two-ways`,
    `design@agent-skills@rejected-citation-direction`, `rejected@core@check-refusing-a-pipe`;
  - for C1: `design@core@path-to-a-home-is-reported`,
    `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`,
    `issue@core@no-checked-form-cites-a-point-inside-a-section`;
  - for C2: `design@core@planned-path-form`, `design@core@reserved-anchors`,
    `tripwire@core@planned-path-outside-plans`,
    `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`;
  - for the standing answers: `tripwire@agent-skills@ruling-lost-in-change`, not met: the misstated
    attribution was caught by a reviewer before the merge, and the file does not place it after the
    transcript review.

## W1. Reading every returned entry whole does not scale

- **a. What it says.** Nine files received together; the two searches returned 169 distinct entries,
  most graded `may-bear`. The session read whole about 35, those its proposals rested on, judged the
  rest from the searchers' reasons, and told the owner. The fix proposed: read whole every `bears`
  entry and every entry a proposal rests on or would change, a `may-bear` entry where a proposal
  touches its subject, and list the others as unread.
- **b. Whether it still applies.** Yes. `skill@knowledge-architect-retrospective-intake@ground-before-judging`,
  step 3: "Read every entry returned, and every entry graded `bears` or `may-bear`, whole with
  `cargo klarch show <ref>`, never from its reason line." The two searchers' descriptions say the
  same. This intake met it again: twenty-four files, fifteen searchers, and a return the session did
  not read whole; it read whole the entries its proposals rest on, and the subagents of checks a to
  c read whole the entries that bore on their findings.
- **c. Whether it is already recorded.** Yes, `issue@agent-skills@a-batch-intake-reads-every-returned-standing-entry-whole`,
  a `design` entry: "the rule that the session reads each entry whole does not scale with that
  union". Its close names this shape among its candidates: "the session reads whole only the entries
  a proposal rests on, the subagents of checks a to c reading the rest". The rule rests on the
  owner's words, in `design@agent-skills@standing-entry-search-agent`: the session reads each entry
  whole "as the owner asked".
- **d. Its kind.** A recorded design question, now with two real instances in two projects, each
  departing from the rule in the same way.
- **e. Its scope.** `goal@knowledge-architect@agents-get-a-complete-workflow`; installed text and two
  heads, one of whose grounds is the owner's.
- **f. A better fix.** Rivals: the proposal; the issue's other candidates, the batch split into
  several searches, or the rule kept with its cost recorded; no change. Since the rule's ground is
  the owner's words, a change of it is the owner's, through the design discussion the issue asks for.
- **g. Its route.** A decision, under `skill@knowledge-architect-design`.
- **h. The proposal.** Handle now: the issue gains both instances, and its design discussion runs,
  with the proposal as its default; the owner rules on the reading the owner asked for.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W2. "Sent upstream" has no case for a workflow file already acted on upstream without an issue

- **a. What it says.** Seven of the eight workflow files present had been acted on upstream outside
  any issue, and one had gone as an issue not yet acted on; one received project file cited a workflow
  file no longer in the directory. The owner ruled that every upstream finding of the intake goes into
  the intake session's own retrospective, this file. The fix proposed: a fourth case saying so, with
  the consequence that the retrospective is finished after the review before the merge.
- **b. Whether it still applies.** `skill@knowledge-architect-retrospective-intake@analysis-and-ruling`
  lists three cases: not sent yet, sent as an editable issue, and "sent with no issue, or an issue
  that cannot be edited: open a new issue for the finding, as the retrospective opens one".
- **c. Whether it is already recorded.** As the decision, `design@agent-skills@installed-fix-goes-upstream`:
  "Where neither applies, a new issue carries it", and "The action, an edit of the workflow file or of
  the issue that carries it, is the owner's."
- **d. Its kind.** An observation of the owner's handling. The upstream that acted on those files
  outside any issue is this repository, where the owner takes the files locally, per
  `design@knowledge-architect@workflow-findings-stay-here`. The third case covers a file acted on
  with no issue, and the owner's word directed a send that differs from it, as the head leaves the
  action to the owner.
- **f. A better fix.** Rivals: the fourth case, which writes into the installed text one owner's
  handling of one machine; no change, the third case and the owner's word.
- **h. The proposal.** No change.
- **Default**: no change, as d says.
- **Outcome**: accepted.

## W3. An occurrence recorded without an id is cited in words

- **a.** The chained-pipe lapse appeared in five of nine files, two under an id; the analysis cited
  the other three by file and section, which works, and no text says it is the intended form. The fix
  proposed: one clause in step 4.
- **b.** Step 4: a cluster "cites every id in it". This intake did the same, citing standing answers
  by their file and section.
- **d.** An observation: two intakes chose the same form by judgement, as
  `primer@room-to-judge` leaves to them.
- **h.** No change.
- **Default**: no change, as d says.
- **Outcome**: accepted.

## W4. Item f does not point at the lapse rule

- **a.** The lapse rule decided three clusters; item f lists "a fix where the instruction is
  delivered" as a rival and does not name the rule, which item d names. The fix proposed: item f's
  rival names the rule as its test.
- **b.** Item f of `skill@knowledge-architect-retrospective-intake@what-to-establish` ends "The fix is
  shaped, and read against the record, as `skill@knowledge-architect-agent-configuration@admission-standard`
  says, read whole", the section that holds the lapse rule.
- **c.** `design@agent-skills@intake-shares-the-admission-standard`: "read whole where it shapes a
  fix, and by its lapse rule where it judges a finding's kind".
- **d.** The pointer exists, with an instruction to read its target whole, and the session reached
  the rule: no act differs.
- **h.** No change.
- **Default**: no change, as d says.
- **Outcome**: accepted.

## W5. A recurring lapse of the premise check, read at one moment and needed at another

- **a. What it says.** Across three earlier sessions of that project, a premise the session supplied
  went unchecked when a question went to the owner: defaults on unchecked premises, a question on the
  premise that a specification is silent, a tripwire on an unchecked premise, each caught by a
  reviewer. The fix proposed: deliver the check where a question is put to the owner, one sentence in
  `skill@knowledge-architect-design@decision-authority` and in the planning skill's step that puts
  questions to the owner, each pointing to `primer@owner-word-premise`.
- **b. Whether it still applies.** Yes. `primer@owner-word-premise`: "A premise the session supplied
  is the session's to check before the owner rules on it." The design skill cites that section only
  for a finding that defeats the premise of a ruling given; the planning skill does not cite it.
- **c. Whether it is already recorded.** `design@agent-skills@owner-word-holds-as-far-as-its-premise`
  keeps the directive in the primer, since a word of the owner can arrive in any session; the proposal
  keeps it there and adds its delivery at the two moments a question is built.
  `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it` is about a
  directive longer than one sentence, which this is not.
- **d. Its kind.** A lapse that recurs at one point of delivery, three times.
- **e. Its scope.** `goal@knowledge-architect@the-owner-s-word-is-verified`; installed text.
- **f. A better fix.** Rivals: the one sentence restated with its pointer at the two sites, which
  `primer@where-knowledge-goes` admits for a directive of one sentence; no change, the lapse rule's
  outcome for a lapse that does not recur. Instructions added: 0, one sentence restated at two sites.
- **g. Its route.** An installed text edit.
- **h. The proposal.** Handle now: the sentence, with its pointer, where the design skill and the
  planning skill put a question or a default to the owner.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W6 and W11. `pipefail` in the harness's environment, and the primer's account of a piped verdict

These findings are analysed with P1 of 2026-10-20-knowledge-architect-defined-ids-lints, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-20-knowledge-architect-defined-ids-lints.md`.

## W7. A fork's default has no marker that the weighing is the owner's

- **a.** One finding was a fork; its default said in words that the weighing was the owner's. The
  file proposes nothing yet, one instance.
- **h.** No change: one instance.
- **Default**: no change, as h says.
- **Outcome**: accepted.

## W8. A subdocument that states its subject at another level than two

- **a. What it says.** One subdocument of a directory-shaped design home opened its subject with a
  level-one heading; the session gave it to a group as one subject, and the project then fixed the
  heading. The fix proposed: a subdocument of a directory home is one subject, whatever its heading
  levels.
- **b. Whether it still applies.** Yes. `agent@knowledge-architect-design-entry-searcher`: "cut into
  groups of whole level-two subjects of that home", and nothing on a subdocument with no level-two
  heading.
- **c. Whether it is already recorded.** `design@agent-skills@design-entry-search-groups`, "never
  inside a subject", whose grouping is the owner's; `design@core@heading-register-two-shapes` defines
  a subdocument as a file the README links.
- **d. Its kind.** A provable gap: an input of the cut, the subject of a subdocument with no level-two
  heading, that no text supplies; one instance, settled by the session's judgement and then by the
  project's own fix.
- **f. A better fix.** Rivals: the proposal, which follows the owner's rule that a subject is never
  cut; no change, since the case dissolves when the subdocument carries level-two subjects as the
  rule assumes. The proposal is a member of the owner's grouping rule, so it goes to the owner as one
  proposal, per `primer@design-heads`.
- **g. Its route.** An installed text edit of the searcher's definition, on the owner's word.
- **h. The proposal.** No change: one instance, which the project's own fix removed, and a rule the
  owner set.
- **Default**: no change, as h says.
- **Outcome**: accepted.

## W9. Searchers report a known item without checking the registers

- **a.** Four design-entry searchers each reported that a Component's goals home holds no goal, which
  an issue already recorded. The file proposes nothing, an observation.
- **b.** `primer@verify-before-relying`, "Check whether it is already known": "Before diagnosing
  anything as a problem, or reporting one met while doing something else, find out whether it is
  already recorded", which reaches a subagent of the `claude` harness. The file's premise that the
  searchers were not told is contradicted by that sentence.
- **d.** A lapse of the subagents; the dispatcher routed each item, at the cost of one listing.
- **h.** No change.
- **Default**: no change, a lapse.
- **Outcome**: accepted.

## W10. A commit that a received file names is a claim too

- **a. What it says.** One received file named its merge commit, which the repository did not hold;
  the session copied it into its analysis and found it at its claim-checking pass. The fix proposed:
  item b resolves a commit a file names with `git cat-file -e` before it is cited.
- **b. Whether it still applies.** Item b covers a report's "repaired in the session" only. The
  primer's list for a document, `primer@verify-before-relying`: "a slug, a path, a section or a run
  identifier is resolved"; it names a commit only among a commit message's claims.
- **c. Whether it is already recorded.** In part:
  `issue@agent-skills@a-retrospective-version-line-names-a-commit-the-repository-does-not-hold`, for
  the version line only.
- **d. Its kind.** Unclear: the primer's list of identifiers a document resolves omits the commit,
  which the list for a message names; the session's claim pass caught it anyway.
- **f. A better fix.** Rivals: a clause in item b, which covers the intake only; the commit added to
  the primer's list for a document, which covers every document, an analysis included. Instructions
  added: 0, a member added to a list.
- **g. Its route.** An installed text edit of the primer.
- **h. The proposal.** Handle now: "a slug, a path, a section, a commit or a run identifier is
  resolved".
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W12. Step 2 reads as if every received file names its main commit

- **a.** Seven of nine received files predated the instruction that the project's file names its
  main commit; item b's fallback covered them, and step 2 does not point to it. The fix proposed: one
  clause in step 2.
- **b.** Step 2: "Note the version each file used, from its version line: the main commit the
  session's tree contained, and the checker's version." The fallback is in item b, "a file whose
  version line names no commit is matched by `git log -S '<phrase>'`".
- **d.** Unclear at one step; the fallback exists two sections down.
- **f.** Rivals: a pointer in step 2; no change, since the session found the fallback. The pointer is
  where the input is first needed. Instructions added: 0. It is a change to the intake's grounding,
  the re-entry point of the batch-intake issue, which W1 already reopens.
- **h. The proposal.** Handle now: step 2 says that a file naming no commit is dated as item b says.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W13. A finding already carried out costs a full section

- **a. What it says.** Twelve of thirty-four ruled sections defaulted to "already repaired" or
  "already recorded", and each still took items a to h. The fix proposed: such a finding takes a
  short section, the evidence of item b or c and its two lines.
- **b. Whether it still applies.** `skill@knowledge-architect-retrospective-intake@analysis-and-ruling`:
  "One section per finding or cluster … with a bullet per item of … what-to-establish", and "Every
  section ends with a `Default` line and an `Outcome` line". Earlier analyses of this repository, and
  this intake's, already write such sections with the bullets that establish them, and the record of
  the earlier ones passed review.
- **c. Whether it is already recorded.** `design@agent-skills@committed-intake-analysis` asks only
  that each finding's section end with a default and the owner's ruling, which a short section keeps.
- **d. Its kind.** A structure the work does not need, per
  `design@agent-skills@capabilities-not-structure`: a fixed mapping from each item to a bullet, which
  this repository's practice already departs from.
- **f. A better fix.** Rivals: the proposal, which removes structure; no change, which keeps a rule
  the practice does not follow. Instructions added: 0, a narrowing.
- **g. Its route.** An installed text edit of the intake skill.
- **h. The proposal.** Handle now: a finding that item b shows repaired, or item c shows recorded,
  takes the bullets that establish it, and its two lines.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W14. The design-entry search is scoped before the fixes it would test are known

- **a.** Fifteen searchers ran before any finding was judged; the guess at the Components held. The
  file proposes nothing, and names a possible order: the standing-entry search first, then the
  design-entry search for the Components the remaining findings touch.
- **c.** `design@agent-skills@design-entry-search` runs the search before a proposal reaches the
  owner; the batch-intake issue records the cost the owner accepted.
- **h.** No change: the cost goes to the batch-intake issue's discussion, which W1 reopens.
- **Default**: no change, as h says.
- **Outcome**: accepted.

## W15. A `live` rejected entry has no move for when another design solves the problem its reason called unsolvable

- **a. What it says.** A rejected entry recorded two hooks that would refuse a piped verdict, dropped
  on the owner's account that nothing could reliably prevent the failure. The project then adopted a
  third shape, a shell option, that the entry never weighed. The hooks are still not wanted, so the
  entry stays `live`, and its account of the problem is now false; the session's first repair mixed
  two reasons, and a second review round was needed. The fix proposed: the entry stays `live` for its
  own design, its reason is narrowed to what still holds, and the new head cites it.
- **b. Whether it still applies.** Yes. `skill@knowledge-architect-decision-recording@losing-alternatives`:
  "`live`: the reason still holds", "`void`: the reason no longer holds. **Mark it; never delete it.**"
  Nothing covers a reason defeated in part by a design the entry never weighed. This repository meets
  the same case if the option of P1 of 2026-10-20-knowledge-architect-defined-ids-lints is taken:
  `rejected@core@check-refusing-a-pipe` rests on "a process inside a pipeline does not set the
  pipeline's status", false in bash under that option.
- **c. Whether it is already recorded.** In part: `issue@agent-skills@the-void-marker-s-reason-reads-two-ways`
  asks what "the reason no longer holds" means, the other side of the same rule.
- **d. Its kind.** Missing: a move a session needs, seen in real use, and needed here soon.
- **e. Its scope.** `goal@knowledge-architect@design-is-recorded-with-its-arguments`; installed text,
  the rules on the markers.
- **f. A better fix.** Rivals: the proposal; the entry marked `void`, which says the hooks' reason no
  longer holds, false since they are still not wanted; no change, which leaves an entry whose reason
  is false. The proposal adds a member to the marker rules, so it goes through the design skill's
  in-change path, with the open issue on `void` read in the same discussion. Instructions added: 1.
- **g. Its route.** A decision, under `skill@knowledge-architect-design`.
- **h. The proposal.** Handle now: the proposal, decided with the void-marker issue, before this
  repository's own entry is brought to the shell option.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W16. History edits in a harness with no interactive rebase

- **a.** The harness refuses interactive git flags; the session rewrote older messages with
  `git filter-branch --msg-filter`, and earlier sessions rebuilt branches with `git commit-tree`. The
  content checks the skill asks for kept each edit safe. Testimony, no proposal.
- **b.** `skill@knowledge-architect-review@what-review-leaves` asks for a history edit "only with no
  uncommitted work in the tree, and each confirmed afterwards to have lost no content", and names no
  mechanism. This repository's root CLAUDE.md names its own, in `instructions@git-workflow` point 2.
- **d.** An observation: each session found a mechanism, and the checks held.
- **h.** No change.
- **Default**: no change, an observation.
- **Outcome**: accepted.

## C1. The finding that asks for a register reference loses the line a path reference carried

- **a.** An analysis kept a line number after a register reference, outside its backticks, which the
  checker does not judge. No proposal.
- **b.** Measured by a checking agent with the binary at the analysed commit: the finding's repair
  already says "A reference names the home: drop the line number or the fragment, and name the point
  inside it in prose", per `design@core@path-to-a-home-is-reported`. The session kept the suffix
  instead of following it.
- **c.** `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` names the
  intake's evidence with a location in its "Why it matters";
  `issue@core@no-checked-form-cites-a-point-inside-a-section` covers points inside skills, agents and
  the primer.
- **d.** A lapse of a repair the message names.
- **h.** No change.
- **Default**: no change, a lapse.
- **Outcome**: accepted.

## C2. The escape form already covers a planned path in a commit message

- **a. What it says.** Measured in a throwaway worktree: a commit message holding the escape form for
  a path the tree does not hold passed `commits`, and still passed once a later commit created the
  path, since a message is judged against its own commit's tree; the form for a path that exists was
  refused. The fix proposed: the primer's description of the escape names a path a plan will create,
  as the project did in its own root CLAUDE.md.
- **b. Whether it still applies.** Confirmed by a checking agent in a scratch repository. The primer,
  `primer@checker-model`: "`elsewhere` (a path the tree does not hold)", and `primer@where-knowledge-goes`:
  "`path@elsewhere@<path>` for a path the tree does not hold"; `planned` is given for a plan document
  only.
- **c. Whether it is already recorded.** The missing example cost one instance already:
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` records a session of
  that project that wrote a planned directory bare in a commit message. `design@core@planned-path-form`
  refuses the escape in a document once the work creates the file, which
  `tripwire@core@planned-path-outside-plans` watches; a commit message never meets it.
- **d. Its kind.** Unclear: the form exists, and a session does not recognise "a path the tree does
  not hold" as a path its work will create.
- **f. A better fix.** Rivals: the example in the primer, limited to commit messages so it does not
  invite the case the tripwire watches; no change. Instructions added: 0, an example.
- **g. Its route.** An installed text edit of the primer.
- **h. The proposal.** Handle now: the escape's description in `primer@where-knowledge-goes` names,
  as an example, a path the commit's work will create, in a commit message.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C3. The plain-word finding asks to rename a skill section's slug

This finding is analysed with C1 of 2026-10-20-thaum-pin-move-0-8-0-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-20-thaum-pin-move-0-8-0-klarch-workflow.md`.

## The standing answers

- **a.** Rules acted on as if absent, each caught before the merge: a commit message dropped the
  owner's "probably" and turned "in particular if" into "if"; the commit that set the harness option
  recorded no reversal search, and a later search missed a README and a code comment; commit messages
  credited a head with content it does not state and counted the texts a search found as all the
  texts the change made false; a commit message attributed to the owner a delivery the owner had not
  ruled on; five wrong claims in the analysis drafts, found by the session's claim pass.
- **b.** The rules stand: `primer@design-heads`, `skill@knowledge-architect-decision-recording@reversal-check`,
  `primer@verify-before-relying`.
- **d.** Lapses. The claims are instances of the recurrence, and the attributions of the kind,
  analysed in
  `path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`.
  The missing reversal search is one instance of a clear instruction.
- **h.** No change for the reversal search.
- **Default**: no change, as h says.
- **Outcome**: accepted.
