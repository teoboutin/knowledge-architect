# The threads and arguments of the slice on splits planned before the work

## Threads

### A split inside a crate is decided and planned at design time, and earns no design head unless a test of the head rules holds `##in-crate-split-unrecorded`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-2` and
  `argument@split-guidelines-and-plan-directories@arg-6`; the session's reading of the owner's words,
  `argument@split-guidelines-and-plan-directories@arg-23`, offered in round 1 for correction.
- Final state: `approved`. It stood `presumed-settled` in the checkpoint table as the session's
  reading; the owner's word on the checkpoint table, attributed in the milestone's Threads,
  closed it. In round 3 the session noted that `design@git@git-is-asked-through-one-module` shows a
  module decision that does earn a head, by test 2 of `primer@design-heads`.
- Its shape: this spec's "Decided design", "A module split's record". Harvest: this slice's, into
  the development skill's text, which alone states it, so it earns no head by test 2 of
  `primer@design-heads`.

### The module guidelines are a fragment of their own, hosted by the development skill, the code-shape reviewer and the code-split scout `##module-guidelines-fragment`

- Opened at this slice's design audit, by the searchers' verdicts on
  `design@agent-skills@shared-text-fragment`: Q5, put in round 3 and agreed in round 4, had the
  module guidelines inside the fragment of the default code shape and the scout host that fragment
  whole, and was put as a host list alone, without that head's rule that a text is a host only where
  its act applies the whole fragment. The fragment of the default code shape holds the test rules
  too, which the scout, reporting split signals, applies none of. The premise was the session's.
- Put to the owner as Q16, three shapes: (b), the session's default, the module guidelines a
  fragment of their own hosted by the development skill, the code-shape reviewer and the scout, the
  scout hosting it and the fragment on when to split, at the cost of one more fragment and of Q5's
  first default; (a), the scout hosting the whole default shape and the host rule narrowed, at the
  cost of the test rules loaded into every scout run and of a weaker host rule for every fragment;
  (c), the scout reading the default shape by a pointer, which `design@agent-skills@shared-text-fragment`
  already defeats.
- Final state: `approved`, on the owner's word on Q16: shape (b). It replaces Q5's delivery.
- Its shape: this spec's "Decided design", "The module guidelines". Harvest: this slice's, into
  `design@agent-skills@default-code-shape` or a head of its own, and the host lists of
  `design@agent-skills@shared-text-fragment`.

### The fragment on when to split holds the occasion alone, and the procedure and the signals against a cut go to a fragment of their own `##when-to-split-fragment-split`

- Opened by the review of this slice's work: the agent-facing-text and self-consistency reviewers
  found that the scout, made a host of `path@agent-skills@fragments/when-to-split.md` by Q16,
  hosts two sentences no act of it applies, the instruction to load the component-creation skill
  before placing the work and the sentence that a creation or a split is never a step taken while
  writing code, which the scout's own text then overrode.
- Put to the owner as Q17, with the session's default: drop the scout from the fragment's hosts,
  the scout reading the component-creation skill's criteria and signals against a split by their
  references. Rival: keep the hosting as Q16 has it.
- Final state: `approved`, on the owner's word on Q17, which chose neither: the fragment is split,
  since a fragment named for when to split holds no content on how to split. The occasion stays in
  `path@agent-skills@fragments/when-to-split.md`, hosted by the design skill, the development skill
  and the scout; the instruction to load the component-creation skill, the signals that a split is
  cut in the wrong place and the sentence that a creation or a split is a design decision go to a
  fragment of their own, hosted by the design and development skills. The owner stated that the
  fragment exists mostly for the scout, where the record has it written for the design and
  development skills before the scout existed, per `design@agent-skills@component-skill-delivery`;
  the ruling's ground, the fragment's name, stands without that premise.
- On confirming the split, the owner stated that the scout sends back signals of code that might
  need a split, and that the session receiving them analyses them, decides whether and how to split,
  and plans the split.
- Its shape: this spec's "Builds". Harvest: this slice's, into the host lists of
  `design@agent-skills@shared-text-fragment` and the delivery of
  `design@agent-skills@component-skill-delivery`.

### A fragment's host list is the session's under the host rule, each change reported under "Decided" `##host-lists-are-the-session-s`

- Opened by the decision-record review before this slice's merge: `design@agent-skills@shared-text-fragment`
  held every host list but the admission standard's four to be the owner's, so that a host added
  went to the owner, on the ground that the owner approved each list, which the table of what the
  owner brings in `primer@design-heads` counts as no ground; the same head records the module
  guidelines' list as the session's choice.
- Put to the owner as Q18, two shapes: (a), the session's default, the lists reserved to the owner,
  the reservation then standing on the owner's word as a reservation of an action, at the cost of a
  question to the owner at every change of a list; (b), the session decides each list by the host
  rule and reports each change, at the cost that a list changes before the owner has seen it.
- Final state: `approved`, on the owner's word on Q18, shape (b): the session decides each host list
  by the head's rule that a text is a host only where its act applies the whole fragment, and
  reports each change of a list under the turn's "Decided" section, per `primer@decided-section`,
  where the owner can contest it. The owner chose (b) over the session's default, and reported
  knowing of no argument for keeping the lists to the owner.
- Its shape and harvest: `design@agent-skills@host-lists-are-the-session-s`, written by this
  slice's review, which `design@agent-skills@shared-text-fragment` cites.

## Arguments

### A split within a crate needs no decision record `##arg-2`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded`.
The owner held that in most cases a split within one crate should need no decision record, since it
does not directly affect what is decided or implemented.

### A module split is decided at design time and earns no head unless a head test holds `##arg-23`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded`.
"My reading of your words: a module split is *decided at design time*, written in the plan document as
a step of its own, and *earns no head* unless one of the tests in `primer@design-heads` holds. Its
reason goes in a comment at the module and in the message of the move commit. In this repository a
module split never passes test 1, since the facade in `design@knowledge-architect@api-facade` keeps
public paths unchanged. That reconciles "a split is a design decision" with "it needs no record".
Correct me if you meant something else." The claim on test 1 is imprecise: the facade of
`design@knowledge-architect@api-facade` covers `check`, `cli`, `model`, `query`, `transcript`,
`create` and `testing`, and `gates` declares `pub mod process` in `path@gates@src/lib.rs`, so a
split of that module changes a public path. "A module split's record" below states the case
behind the facade only.
