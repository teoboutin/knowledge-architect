# Slice 4: the design skill's record search may wait until the scope settles

## Builds

1. **Loop step 1 of `skill@knowledge-architect-design@discussion-loop`**, in
   `path@agent-skills@content/skills/design/SKILL.md`, per #search-timing: the search may wait until the
   scope settles; no thread closes before it has run; until then, a positive word of the owner leaves
   a thread `presumed-settled`. The grounding before the first round, the reading of the homes and the
   drafting and challenge of the shapes, stays where it is.
2. **The heads rewritten in place** under `skill@knowledge-architect-decision-recording`:
   `design@agent-skills@search-after-first-challenge`, and
   `design@agent-skills@record-searched-before-the-work` where it states when the design skill
   dispatches the search; each text citing them reread, as the milestone document's "What is already
   decided" lists.
3. **Any other text of the design skill that states the order**, among them the threads and states
   section, `skill@knowledge-architect-design@threads-and-states`, where `presumed-settled` is defined,
   read and aligned.
4. **`cargo klarch install-agent-skills`**, the CHANGELOG.md entries in `Next release`, and
   `cargo x changelog`.

## Claims

No run is built to observe a discussion under the new step, per
`design@agent-skills@synthetic-evidence-not-built`.

- **The installed design skill states that the search may wait, that no thread closes before it has
  run, and that a positive word before it leaves a thread `presumed-settled`; and no sentence of the
  design skill still orders the search after the first challenge.** Test: a reading of the installed
  skill whole, recorded in the commit; a phrase search is a lead only. Shown to fail: the same
  reading of the skill today finds the search ordered after the first challenge.
- **The two heads and the skill state the same order.** Test: the decision-record axis and the
  self-consistency axis of the review. Shown to fail: the heads today state the old order.

## Audit subjects

- `skill@knowledge-architect-design` whole, in particular
  `skill@knowledge-architect-design@discussion-loop`, `skill@knowledge-architect-design@threads-and-states`,
  `skill@knowledge-architect-design@decision-authority` and `skill@knowledge-architect-design@bounded-work`.
- `design@agent-skills@search-after-first-challenge`, `design@agent-skills@record-searched-before-the-work`,
  `design@agent-skills@bounded-path-in-design`, `design@agent-skills@seeds-name-decisions`,
  `design@agent-skills@structure-the-flow`.
- `issue@agent-skills@the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`, whose
  case this slice answers in part, and
  `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`.
- `rejected@agent-skills@conformance-reader-at-the-checkpoint`,
  `rejected@agent-skills@delta-search-at-convergence` and
  `tripwire@agent-skills@record-search-misses`.
- `goal@knowledge-architect@design-with-agents-as-counterparts`.

## Fails alone on

- A thread that a session closes before the search has run, which the transcript reviewer or a
  retrospective reports.
- A sentence of the design skill that still orders the search after the first challenge.

## Premises that expire

- **The search runs once per discussion, with a search again for a thread on a new subject**, per
  `design@agent-skills@record-searched-before-the-work`. If a later change splits it into several
  searches, the synthesis's cost argument is read again.

## Decided design

### The search may wait until the scope settles, and no thread closes before it has run

Thread: `thread@development-and-components@search-timing`.

**Shape.** The synthesis of the reply to R5, which the owner approved in R6, its emphasis dropped: "the search may wait until the scope settles, but
no thread closes before it has run. Until then, your positive words leave threads `presumed-settled`."
(`argument@development-and-components@arg-63`.) The session judges when the scope has settled; the
search runs at the latest before the checkpoint that would close a thread.

**Argument.** For: in this discussion the scope widened twice before the search, whose grades were
against 17 drafted shapes, not the first three, and a first-round search would have needed further
searches for each new subject (`argument@development-and-components@arg-61`); the owner's R5: "It
allows freer discussion of the design, which will then be confronted to the current records."
(`argument@development-and-components@arg-52`). Against: a reader after the owner has ruled "reads
rulings already given", `rejected@agent-skills@conformance-reader-at-the-checkpoint`, and seven
threads closed before the search in this discussion (`argument@development-and-components@arg-62`).
The synthesis keeps the free discussion and the more precise grades, and removes the cost of rulings
already given, since no thread closes before the search.

**Nearest rival and the fact that defeated it.** The current order, after the first challenge,
`design@agent-skills@search-after-first-challenge`: a scope that widens after the search makes its
grades stale, as `issue@agent-skills@the-reading-on-touch-of-may-bear-entries-met-its-reopening-condition`
records. The owner's plain proposal, a search after a few rounds with threads closing meanwhile: the
rulings-already-given cost above. A search before and after the challenge: it doubles the search.

## Harvest

- **Heads:** `design@agent-skills@search-after-first-challenge` and
  `design@agent-skills@record-searched-before-the-work` rewritten in place, in
  `register@agent-skills@design`.
- **Rejected:** the search after a few rounds with threads closing before it, where
  `design@agent-skills@losing-alternatives-filter` admits it; the current order moves there under
  the reversal check of `skill@knowledge-architect-decision-recording`.
- **Tripwires:** premortem cause 4 of the milestone document, judged by the tripwire standard.
- **Plan document:** this slice's spec leaves last, and with it the milestone document, as its
  harvest row says.
