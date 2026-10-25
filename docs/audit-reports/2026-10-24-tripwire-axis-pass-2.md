# Report, pass 2 of the tripwire axis (a re-check)

The run's scratch directory is /home/teo/.cache/claude-tmp/claude-1000/-home-teo-dev-knowledge-architect/00654c6d-ba37-4196-bc80-21e64964b15d/scratchpad/audit2.

- as1, as2 and c1 are the groups agent-skills-1, agent-skills-2 and core-1. For example, as2/13 is draft 13 of agent-skills-2.
- `<W>` is the SHA that the commit writing a `**Watches until:**` line gets on main after the rebase merge. Find it with `git log -S'<the line>' --format=%h origin/main -- <home> | tail -1` (as2/00-common.md).
- Every horizon command below uses `<W>`.

## 1. Header

- **Commit audited:** the commit "[audit] Pass 1 of the tripwire axis applied: every tripwire kept has a horizon, and 20 leave".
- **Previous pass audited:** the commit pass 1 audited, "[review] Repairs of the review of the tripwire standard's work, steps 1 to 3, AC1 fired and answered, and the owner's rulings on D3 and on a doubt's weight".
- This pass is a re-check.

**The agents of each step** (crates/agent-skills/content/workflows/tripwire-audit.js):

| step | agent |
| --- | --- |
| preflight | 1 general agent |
| setup | 1 general agent |
| drafts | 3 `agent@knowledge-architect-tripwire-auditor`, one per group |
| calibration | 1 `agent@knowledge-architect-tripwire-auditor` |
| verify | 3 `agent@knowledge-architect-tripwire-auditor`, one per group |
| cross-check | 1 general agent |
| report | 1 general agent |

**The groups:**

| group | first entry | last entry | entries |
| --- | --- | --- | --- |
| agent-skills-1 | `tripwire@agent-skills@a-shortcut-decision-earns-a-head` | `tripwire@agent-skills@head-created-without-deliberation` | 17 |
| agent-skills-2 | `tripwire@agent-skills@overlap-missed` | `tripwire@agent-skills@verifiers-wave-findings-through` | 17 |
| core-1 | `tripwire@core@a-second-producer-chain` | `tripwire@core@walked-count-differs-between-machines` | 11 |

**Entries:** 45. entries.json gives agent-skills 34 and core 11, which matches the commit audited's "45 against 64".

**Drafts of record per outcome:**

| outcome | count | drafts |
| --- | --- | --- |
| kept | 38 | as1 01-05, 07-13, 15-17; as2 02-09, 11-17 (as2/13 is "kept, with its horizon line reworded"); c1 02-06, 08-10 |
| reworded | 3 | as2/01, as2/10, c1/01 |
| converted | 3 | as1/06, as1/14, c1/11 |
| removed (condition 3) | 1 | c1/07 |
| absorbed, decision gone, horizon passed, fired | 0 | |

**Verdicts:**

| verdict | count | drafts |
| --- | --- | --- |
| verified | 40 | |
| refuted | 4 | c1/02, c1/03, c1/04, c1/10 |
| plausible | 1 | as2/13 |

The cross-check writes "16 verified, and 13 plausible". There, "13" is a draft number, not a count.

**Marks:**

| mark | count | entries |
| --- | --- | --- |
| bearing on a ruling | 1 | c1/07; the cross-check corrects its ground |
| firing | 0 | |
| known | 5 | as1/06, as1/14, as2/02, as2/05, c1/11 |
| on the kept list | 0 | the kept list reads "The owner ruled no entry, text or rule to keep against a finding" |

**Counts asked by the `skill@knowledge-architect-project-audit@record-of-an-audit` section:**

- **Dispatch:** the saved workflow, with 3 groups of 17, 17 and 11 entries, a sample of 5, 3 verifiers, a cross-check and a report.
- **Calibration reading proposed:** R1. The session has not settled it.
- **Old and new text, as the verifiers classified it.** The verifiers classified whole entries, and all 45 are new.
- **Old and new text, as this report classifies it.** I classified the text each finding is on, and checked it with `git show the commit pass 1 audited:<home>`.
  - Five R1 headings are unchanged since the commit pass 1 audited, so they are old text: core tripwires.md at the commit pass 1 audited, lines 166, 48, 84, 96 and 11.
  - The `tripwire@agent-skills@overlap-missed` clause is also unchanged (agent-skills tripwires.md at the commit pass 1 audited, lines 389-390), so it is old text too.
  - Proposed counts, if R1, the F6 extension and as2/13 are all confirmed:
    - on old text: 6, or 5 if walked-count's heading is left as an exception;
    - on new text: 5 (`tripwire@core@issue-kind-list-grows`, `tripwire@agent-skills@rule-misread-across-groups`, `tripwire@agent-skills@ruling-lost-in-change`, `tripwire@core@a-second-producer-chain`, and the attribution in `tripwire@agent-skills@audit-passes-do-not-converge`);
    - on old text and on the owner list: 0.
  - Pass 1 counted none ("Violations left: not counted, this is a first pass"), so there is no convergence question for the owner.
- **Owner list, as the cross-check proposes it:** 1 item, "removed on condition 3", holding `tripwire@core@issue-kind-list-grows`. A second item is conditional (part 5).
- **Rulings against the default:** not asked yet.

**Entries missing from a return:** none.

**Refuted drafts, each with its reason** (verify/core-1/verdicts.md):

- **c1/02 `tripwire@core@candidate-rule-silence`:** "Under calibration R1: the heading ... names a subject, and the entry has no preamble stating a premise ... reworded comes before kept in the order. The rest of the draft holds."
- **c1/03 `tripwire@core@checker-source-literals-are-data-purpose`:** "Under R1: heading "...`' purpose", no premise in the body; outcome reworded ..., the rest kept on the doubt over conditions 1 and 3, which holds."
- **c1/04 `tripwire@core@checker-source-literals-are-data-whole-source-scope`:** "Under R1: heading "...`' whole-source scope", no premise in the body; outcome reworded, the rest kept on the doubt over condition 2, which holds."
- **c1/10 `tripwire@core@private-item-needed`** (c1/15 is a copy): "Calibration R1 is on this entry: heading ... states no premise and the body opens on "**Fires when:**"; outcome reworded, the rest kept on the doubt X3 settled."

## 2. The calibration

The calibration states: "Proposed readings: 1. Entries whose rule admits both outcomes: 0."

**`tripwire@agent-skills@audit-drafts-mostly-noise`** (draft of record as1/05)
- All three groups: kept, on a doubt over condition 3.
- as2/18 states the two readings:
  - Reading A, exclude: "a draft the verifier refutes is that test working".
  - Reading B, keep: "a draft reporting a predicted gap as provable breaks it, and recurring across two passes shows the standard's wording failing".
- What the doubt rests on:
  - the standard: "A cause that no condition clearly excludes is recorded with a short horizon";
  - the lifecycle: "An entry is proposed removed only when the words of a condition, quoted, exclude it";
  - pass 1: "Neither reading is settled by the text".
- The three groups use three different path patterns in their horizon commands, which give the same count today.

**`tripwire@agent-skills@pipefail-status-misread`** (as2/04, copy as2/19)
- All three groups: kept, on a doubt over condition 1.
- They agree on the slip: "the clause watches the head's premise "Its costs fail safe"".
- The groups differed on old versus new text. The verifier settled it: "new (label only; clause old)".

**`tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap`** (as2/14, copy as2/20)
- All three groups: kept; not fired; the attribution stands per R3.
- as1's command does not filter out README.md. The calibration says: "The verifier should prefer the filtered command."

**`tripwire@core@private-item-needed`** (c1/10)
- The groups disagree:
  - as1/20: reworded (the heading), then kept on X3;
  - as2/21: kept, "Shape: conforms";
  - c1: kept, "shape judged on the Response line only".
- **R1, proposed:** "reworded, the heading gains the premise the entry watches. The rest of the entry is kept on the doubt over condition 1, with its horizon."
- What R1 rests on:
  - the standard: "The entry is a level-two heading ... its slug last:", with the template line "## Guarding `design@<anchor>@<id>`: <the premise watched> `##<slug>`";
  - the standard: "A tripwire watches one premise of a recorded decision";
  - the order "fired, its decision is gone, horizon passed, removed, converted, absorbed, reworded, kept", and the rule "propose the first in that order";
  - precedent: pass-1 report lines 604-606, and the commit audited's "stated no premise" about `tripwire@core@issue-kind-list-grows`.
- R1 also reaches `tripwire@core@candidate-rule-silence`, `tripwire@core@checker-source-literals-are-data-whole-source-scope` and `tripwire@core@checker-source-literals-are-data-purpose`. It does not reach `tripwire@core@planned-path-outside-plans`.
- the commit audited's application judgement covered "all 65 entries the application changed or deleted", including these entries. It failed only `tripwire@core@issue-kind-list-grows`'s heading. So it also passed the headings R1 now finds.

**`tripwire@core@walked-count-differs-between-machines`** (c1/11)
- All three groups: converted, already known.
- Under R1, reworded also applies, second in the order. The premise is in the body, and only the heading lacks it. "as2 and c1 should have named reworded."

**Common note on horizons:** "the commit audited is not an ancestor of origin/main ... The verifier should take the `<W>` form."

**Drafts judged again under a reading:**
- **c1, under R1:** 02, 03, 04 and 10 were refuted. 11 keeps its outcome. 01, 05, 06, 07 and 08 were re-read: "05's heading names the failure with no preamble, outside R1's scope". 09 is outside R1.
- **as2:** R1 bears on none. 01 and 10 were "judged together, both verified" under F6 extended. 06, 09, 11 and 15 have incomplete history lists, but the verifier says "none changes an outcome".
- **as1:** none.

## 3. The findings, grouped by the outcome the cross-check proposes

### 3.1 Removed, condition 3 (owner list): core, crates/core/docs/tripwires.md

**`tripwire@core@issue-kind-list-grows`** (c1/07). Verified, bears on a ruling, new text.
- **Head:** "An unknown kind is a finding naming the list, and adding one is a reviewed manifest diff ... so an unknown one would owe the wrong three in silence ... and `register@core@tripwire` guards the closing."
- **Clause:** "a kind outside the six compiled defaults, declared in an issue register of this project or of a consuming project as a retrospective file the intake receives reports, ends up with a single member across that project."
- **Verifier:** "`rejected@core@open-issue-kind-set` lost on "an unanticipated kind then owes the wrong three, silently", not on proliferation, and the premise "the list stops partitioning anything" appears only in the tripwire since a9ea4f2, never in the head ... no reading leaves it in doubt."
- **Cross-check correction:** "pk/10 of pass 1 ... was not an owner-list item ... It rests on pass 1's session verification, which the owner's general "I accept all defaults here, everything look fine, proceed" followed."
- **Edit:** delete lines 61-78.
- **Horizon:** "the next three release tags", counted by `git tag --contains <W> --list 'v*' | wc -l`; 0 of 3 so far.

### 3.2 Reworded (applied in the branch, listed under Decided)

**agent-skills, crates/agent-skills/docs/tripwires.md**

**`tripwire@agent-skills@overlap-missed`** (as2/01). Verified. The verifier marks it "new (clause old)"; by the check in part 1 the clause is old text.
- **Ground:** the slip rule, plus F6 ("a verifier is an agent following its brief. Default: it binds"), extended to the cross-check agent.
- **Before:**
  - "**Fires when:** during the application of a pass of the design-record axis, the session meets two findings that edit one paragraph or one slug, and the cross-check's overlap map does not list them."
  - "**Watches until:** the application of 2 passes ..."
- **After:**
  - "**Fires when:** in the applications of two passes of the design-record axis, the session meets two findings that edit one paragraph or one slug and that the cross-check's overlap map does not list; or two such findings, both applied, reach the main branch with no review reporting the collision."
  - "**Watches until:** the application of 3 passes ..."
- **Count:** `git log --diff-filter=D --name-only --format= <W>..origin/main -- 'docs/audit-reports/*design-record-axis-pass-*.md' | wc -l`; 0 of 3 so far.
- **Verifier:** "The extension of F6 to the cross-check is the session's to confirm."

**`tripwire@agent-skills@rule-misread-across-groups`** (as2/10). Verified, new text.
- **Before:**
  - "**Fires when:** in one pass, the session traces findings of more than one group to one misreading of one rule, and the calibration did not catch it."
  - "**Watches until:** 2 passes of the design-record axis, each counted by its report file."
- **After:**
  - "**Fires when:** in two passes of the design-record axis, the session traces findings of more than one group to one misreading of one rule that the calibration did not catch."
  - "**Watches until:** 3 passes ..."
- **Count:** `git log --diff-filter=A --name-only --format= <W>..origin/main -- 'docs/audit-reports/*design-record-axis-pass-*.md' | wc -l`.
- **Verifier:** "it is a clause on the calibration sample's coverage, which F6 names."

**`tripwire@agent-skills@ruling-lost-in-change`** (as2/13). **Plausible**, new text.
- **Before:** "**Watches until:** 10 commits on the main branch whose message carries a deliberation on the in-change path."
- **After:** "... whose message names the in-change path."
- **Count:** `git log --format=%h -i --grep='in-change path' <W>..origin/main | wc -l`.
- **Evidence:** the commit counts are 52, 29 and 14, and "no instruction asks for that phrase".
- **Verifier:** "it counts messages that only mention the path, such as the step 2 commit of this branch and the commit pass 1 audited, which the standard's "names the first occasions that exercise the decision" does not admit ... the session chooses."

**`tripwire@agent-skills@audit-passes-do-not-converge`** (as1/06). Second outcome, optional. Verified, known.
- **Before:** "The owner proposed this watch as an acceptance criterion, and kept it as a tripwire for future audits."
- **After:** "The owner proposed this watch, as an acceptance criterion of the discussion that made the decision."
- **Evidence:** 39bf4fb: "Agreed on your defaults: ... keep AC4 as a tripwire to watch future usage." By the primer, "An approval is not a ground".
- **Cross-check:** rewording it "with the issue's lines 18 to 20" is a case-2 fix.

**core, crates/core/docs/tripwires.md**

**`tripwire@core@a-second-producer-chain`** (c1/01). Verified, new text.
- **Head:** "such a check is a phase, not a check of the last one, and it goes before its consumers."
- **Before:** "**Fires when:** a check of the core, under `path@core@src/check/`, or a check of an extension reads another check's findings, or a check's output is stored for another check to consume."
- **After:** "**Fires when:** a check of the last phase, the core's under `path@core@src/check/` or an extension's, reads another check's findings, or stores its output for another check of the last phase to consume."
- **Horizon:** unchanged, "the next ten commits ... under `path@core@src/check/`", counted by `git log --format=%h <W>..origin/main -- crates/core/src/check/ | wc -l`.
- **Verifier:** "tree.rs and agents.rs ... are phase 2."

**R1 headings** (refuted drafts; all old text):

| entry | before | after |
| --- | --- | --- |
| `tripwire@core@private-item-needed` | "## Guarding `design@core@api-facade`'s membership" | as1/20: "## Guarding `design@core@api-facade`: a consumer needs an item that is private behind the facade", plus "The decision rests on the premise that the role modules publish every item a consumer of the library, a binary, an extension or the extension's tests, needs." |
| `tripwire@core@candidate-rule-silence` | "...`' silence on an unknown head" | no text written |
| `tripwire@core@checker-source-literals-are-data-purpose` | "...`' purpose" | no text written |
| `tripwire@core@checker-source-literals-are-data-whole-source-scope` | "...`' whole-source scope" | no text written |
| `tripwire@core@walked-count-differs-between-machines` | "...`'s portability" | as1/21: "## Guarding `design@core@git-supplies-the-walk`: two machines walk a different set of files for one commit" |

For the three entries with no text written, the session writes the heading from the entry's Fires-when clause.

Horizons:
- `tripwire@core@private-item-needed`: tags, 3.
- `tripwire@core@candidate-rule-silence`: `standing`.
- `tripwire@core@checker-source-literals-are-data-purpose`: `git log --format=%h <W>..origin/main -- crates/core/src/ | wc -l`, 5.
- `tripwire@core@checker-source-literals-are-data-whole-source-scope`: `git log --diff-filter=A --format=%h <W>..origin/main -- docs/retrospective-reports/ ':!docs/retrospective-reports/README.md' | wc -l`, 3.

### 3.3 Converted, already known: nothing new (ruled F1 in pass 1)

| entry | issue recording the conversion | horizon count |
| --- | --- | --- |
| `tripwire@agent-skills@audit-passes-do-not-converge` (as1/06) | `issue@agent-skills@no-check-compares-an-audit-pass-s-owner-list-count-with-the-previous-pass-s` | `git log --format=%h <W>..origin/main -- <issue> | wc -l` |
| `tripwire@agent-skills@duplicate-auditors-add-nothing` (as1/14) | `issue@agent-skills@no-check-reports-a-duplicated-workflow-lens-whose-other-agents-found-nothing` | same form |
| `tripwire@core@walked-count-differs-between-machines` (c1/11) | `issue@core@the-walk-does-not-report-what-the-per-clone-exclude-file-removes` | `git log --diff-filter=D --format=%h <W>..origin/main -- <issue> | wc -l` |

All three are verified and known, with condition 1 as the ground: "Where a script could compute the firing, write that check instead."

### 3.4 Kept, no edit (33 entries, all verified, all new text, every count 0)

**agent-skills** (horizon reach in parentheses):
- **Merged-commit counts:**
  - `tripwire@agent-skills@a-shortcut-decision-earns-a-head`: `git log -i --grep='bounded path'` (3). It stands or falls with the ruling on as2/13.
  - `tripwire@agent-skills@agent-text-findings-need-nothing`: changes under `content/` (10).
  - `tripwire@agent-skills@attribution-loses-the-scope`: `grep -c '^\[review\]'` (10).
  - `tripwire@agent-skills@head-created-without-deliberation`: `-G'^### ...'` on the design homes (20).
  - `tripwire@agent-skills@owner-intent-stripped`: the same `-G` count (20).
  - `tripwire@agent-skills@rarer-rule-in-the-model`: changes to PRIMER.md (10).
- **Retrospective analyses added:**
  - `tripwire@agent-skills@agent-text-written-without-the-standard`, `tripwire@agent-skills@axis-not-run-at-a-pin-move`, `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`, `tripwire@agent-skills@departure-overridden-as-a-reading`, `tripwire@agent-skills@expectation-set-closes-a-contradiction` (3 each);
  - `tripwire@agent-skills@piped-chain-runs-under-pipefail` (10);
  - `tripwire@agent-skills@pipefail-status-misread` (8);
  - `tripwire@agent-skills@pointer-not-followed` (12);
  - `tripwire@agent-skills@record-answered-questions-reach-the-owner` (15).
- **Audit report files:**
  - `tripwire@agent-skills@audit-drafts-mostly-noise` (2);
  - `tripwire@agent-skills@calibration-reading-rejected` (2);
  - `tripwire@agent-skills@conforming-draft-wrong` (3);
  - `tripwire@agent-skills@record-check-misses-a-head` (3);
  - `tripwire@agent-skills@verifier-refutes-a-true-finding` (3);
  - `tripwire@agent-skills@verifiers-wave-findings-through` (3).
- **Plan documents deleted:**
  - `tripwire@agent-skills@design-conformance-findings-needless` (3);
  - `tripwire@agent-skills@ruling-lost-in-assembly` (5);
  - `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap` (6, with the README filtered out);
  - `tripwire@agent-skills@design-entry-search-misses` and `tripwire@agent-skills@goal-missed-though-graded` (3 plans and 3 retrospectives each).
- **Heads added plus one design-record pass:**
  - `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head` (15 and 1);
  - `tripwire@agent-skills@rule-title-wider-than-its-argument` (20 and 1);
  - `tripwire@agent-skills@split-rule-from-exception` (20 and 1).

**core:**
- `tripwire@core@fix-makes-a-choice`: the spec's deletion (1). Its heading is outside R1, by the cross-check's default.
- `tripwire@core@init-cleanup-beyond-the-listed-files`: tags (3).
- `tripwire@core@phases-gate-the-report-two`: tags (3).
- `tripwire@core@planned-path-outside-plans`: tags (2).

Two `rejected` entries are kept on a doubt and name a kept entry. Each is judged again when that entry's horizon passes:
- `rejected@core@init-refused-outside-a-repository-root` names c1/06;
- `rejected@core@planned-path-in-every-document` names c1/09.

## 4. The overlap map and the stale texts

**Findings that touch one entry:**
- **`tripwire@agent-skills@audit-passes-do-not-converge`:** it is converted, and its rewording is optional. The issue's lines 18-20 carry the same attribution: "the session words both alike or leaves both."
- **`tripwire@core@walked-count-differs-between-machines`:** converted, plus R1. It is "the fifth of the same class". Leaving it unchanged is an exception the session states.
- **`tripwire@core@private-item-needed`:** only c1/10 counts. The session can take as1/20's edit.
- **`tripwire@agent-skills@a-shortcut-decision-earns-a-head` and `tripwire@agent-skills@ruling-lost-in-change`:** "one reading of the standard's horizon paragraph decides both."

**Absorptions:** none.

**Entries guarding one head:** no head has drafts that judge the same premise differently. The 11 heads are listed in cross-check 1c.

**Shared readings:**
- **F6 extended** (as2/01, as2/10): "Rejecting the extension refutes both edits."
- **R1:** covers five headings. The default excludes `tripwire@core@fix-makes-a-choice`.
- **The `<W>` form:** changes no outcome.

**Stale texts if `tripwire@core@issue-kind-list-grows` is deleted:**
- pass-1 report:1019 and :1408 would dangle. Repair each with the bare slug.
- `design@core@issue-kind-list-closed`, design.md:1172: "and `register@core@tripwire` guards the closing" becomes false. Strike it in the deleting commit as a case-2 fix.
- `rejected@core@open-issue-kind-set` is still cited by the head, so it is not orphaned.

**Stale texts after the rewordings:**
- rejected.md:420 (`rejected@agent-skills@workflow-repair-steps-reused`) stays true, though its doubt "takes longer to settle".
- The conversion issue for `tripwire@agent-skills@audit-passes-do-not-converge`, line 18, is "**false already**".
- Every other citing text stays true: issue :55 for `tripwire@core@private-item-needed`, a-plain-hash-item-name-is-read-by-nothing.md:77, path-quickfixes.md:240, tripwire-standard.md:196, and the pass-1 report lines.

## 5. The session's verification

The session read this report whole: the calibration's reading R1 against the four refuted drafts
and the entries it reaches, every proposed rewording against its draft, the owner-list draft against
c1/07, and the plausible verdict as2/13. Its decisions:

- **R1 confirmed**: a heading names the premise the entry watches, per the standard's entry block;
  the five headings of core that name a subject are reworded: `tripwire@core@private-item-needed`,
  `tripwire@core@candidate-rule-silence`, `tripwire@core@checker-source-literals-are-data-purpose`,
  `tripwire@core@checker-source-literals-are-data-whole-source-scope` and `tripwire@core@walked-count-differs-between-machines`.
  `tripwire@core@fix-makes-a-choice` is outside it: its heading states the premise.
- **F6 of pass 1 extends to the cross-check agent**: it is an agent following its brief, so
  `tripwire@agent-skills@overlap-missed` and `tripwire@agent-skills@rule-misread-across-groups` fire in two passes.
- **as2/13**: the horizon of `tripwire@agent-skills@ruling-lost-in-change` counts messages naming the in-change path. A
  horizon's length and instrument are the writer's judgement, and a mention count errs short, the
  cheaper error, so the rule is read one way and no item goes on the owner list.
- **The optional rewording** of `tripwire@agent-skills@audit-passes-do-not-converge`, and the same lines of its issue: the
  approval leaves the entry's text, as the primer says of an approval.
- **`tripwire@core@a-second-producer-chain`**: the clause names a check of the last phase.
- **Old and new text**: violations left on old text, 6 (the five headings and the clause of
  `tripwire@agent-skills@overlap-missed`); on new text, 6 (`tripwire@core@issue-kind-list-grows`, `tripwire@agent-skills@rule-misread-across-groups`,
  `tripwire@agent-skills@ruling-lost-in-change`, `tripwire@core@a-second-producer-chain`, the attribution of
  `tripwire@agent-skills@audit-passes-do-not-converge`, and its issue's lines); on old text and on the owner list, 0. Pass 1
  counted none, so no convergence question arises.

## 6. The owner list

### F1: removed on condition 3, `tripwire@core@issue-kind-list-grows`

Its clause counts "a kind outside the six compiled defaults ... ends up with a single member across
that project"; its head says "adding one is a reviewed manifest diff", which endorses what the clause
counts, and the premise "the list stops partitioning anything" is in no text of the head. Condition
3: "A clause that counts what the decision instructs or endorses fires on the decision working".
Pass 1 kept it, with no ruling of the owner on the entry by name. Its deletion strikes the clause
"and `register@core@tripwire` guards the closing" of `design@core@issue-kind-list-closed`.

Default: removed.
Outcome: removed, on the owner's word: "F1 this entry can be removed, I am of the same mind".

The owner, on stopping the audit after this pass: "I agree we can apply pass 2 and stop, the audit clearly converged."

## 7. What the run did not decide

**Plausible verdicts:** as2/13.

**Firings:** none is marked. Two near cases:
- as2/11 (`tripwire@agent-skills@rule-title-wider-than-its-argument`): d4f04ae is "one of the two it needs". The next such review fires it.
- as2/15 (`tripwire@agent-skills@split-rule-from-exception`): 81dbea2 is "a plan's split, not a head, so not counted".

**Heads:**
- `design@core@issue-kind-list-closed`: its last clause becomes false once the removal is applied.
- `design@agent-skills@head-ground-is-the-argument`: the approved-default issue records the owner disagreeing with part of it. This belongs to the design-record axis.
- as1/10: the premise of `design@agent-skills@standing-entries-searched-before-the-work` "moves with" the open search issue.
- No head was found false of the code.
- as2/22: the register pointer at design.md:505 "is gone at the commit audited".

**Met outside the task, from the returns:**
- **as1:**
  - Three core headings name a subject (now R1).
  - The `tripwire@agent-skills@audit-passes-do-not-converge` attribution.
- **as2:**
  - rejected.md:322 is "true of the premise, loose of the clause".
  - `<W>` must be looked up on main.
  - The approved-default issue against `design@agent-skills@head-ground-is-the-argument`.
- **c1:**
  - The `design@core@issue-kind-list-closed` clause.
  - walked-count's residual cause.
- **Verifiers:**
  - as1 verifier: the issue's lines 18-20 are a case-2 fix.
  - c1 verifier: whether R1 extends to `tripwire@core@fix-makes-a-choice` is the session's call.
- The calibration and the cross-check have nothing else.

**Failed steps:** none. Every step left its output, and no entry is missing.

**Open classification question:** the session settles old versus new text before writing the pass's counts (part 1).