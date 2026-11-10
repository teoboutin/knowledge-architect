---
kind: design
---
# No head says whether the tests of this Component keep to what a consumer can use as a rule or as a guideline

## Summary

`design@agent-skills@integrated-test-through-public-api` states, for every target project, that an
integrated test uses only what a consumer could use, and calls the statement a guideline. A goal of
this Component held the same statement for the tests of `testing` as a rule that bound outright. No
head of this repository decides which of the two strengths these tests keep.

## Details

### What

**The instances.** The statements, as the Component's goal held them:

1. Its title: "A test of this Component of an effect a consumer could see uses only what a consumer
   can, and what it needs and cannot observe there is published". It bound every test under
   `path@testing@tests/` of an effect a consumer could see.
2. "A test of this Component of an effect a consumer could see reaches knowledge-architect through
   what a consumer can use." It bound the same tests: none reaches a private item.
3. "What it needs and cannot observe there is hidden from consumers too, and what is published is
   changed so that the test observes it." It bound the published interface: a gap such a test
   meets is repaired there.
4. "A claim whose effect no consumer could see is outside this goal." It bound nothing: it set the
   limit of the three statements above.
5. "Requires, for instance: a finding a consumer sees that no public item returns is made public,
   or its test is rewritten to reach it as a consumer does." It bound the same repair as
   statement 3.
6. "no item is published only so that such a test can reach it", from the goal's measure, and
   "Rules out, for instance: a hidden item, a feature or a module published only so that a test
   here of such an effect can reach it." They bound what the published interface may hold for a
   test.
7. "Admits, for instance: the published interface growing because such a test showed a need; a
   public item that only such a test uses at first, when the effect it shows is one a consumer
   could see." It bound the limit of statement 6. The owner stated that such a public item is
   admitted.

`design@agent-skills@integrated-test-through-public-api` holds statements 2 to 4 in its own words,
as "a property of the default shape" of `design@agent-skills@default-code-shape`, a guideline as
that head argues. The layout holds part of statement
2: cargo builds each file of `path@testing@tests/` as a crate apart from the code it tests, so the
build refuses a private item.

**Suspected mechanism**: the rule was written as a goal of the Component, which bound outright, and
the head that holds its statement was written for the installed text, where it is a guideline.

**The re-entry point**: the first change that makes an item of a published crate public for a test
of this Component, or that leaves such a test reading what a consumer cannot.

### Why it matters

A session that meets a gap between what a test of this Component needs and what the published
interface returns reads a guideline, which room to judge lets it set aside, where the project held
a rule. `design@knowledge-architect@mock-projects-in-one-crate` rests on these tests testing the
public API.

### What would close it

A discussion under `skill@knowledge-architect-design` decides whether this repository's tests of
`testing` hold to the statement as a rule. Its nearest rival is the guideline of the installed text
alone, which already applies here. A rule that stands on a fact takes a head of
`register@testing@design`, and statements 6 and 7, which answer what the published interface may
hold for a test, a head of their own beside it; otherwise the guideline stands and this entry closes with the reason.
