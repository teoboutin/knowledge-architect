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
  `design@agent-skills@default-code-shape` and the planning skill's text; no head of its own unless
  the entry tests give one.

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
