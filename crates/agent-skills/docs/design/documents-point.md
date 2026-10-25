# Agent skills design: How documents point at each other

## How documents point at each other

### A reference is written where the text would have to be revisited if the entry it names changed `##a-reference-claims-a-revisit`

The rule is the premise the workflow relies on to be useful: an entry is referenced wherever it is
load-bearing, so the checker lists what a reversal, a closure or a firing touches, and the scope of
reversing a decision is assessed from that list, per
`goal@knowledge-architect@structure-and-workflow-work-together`. Every citeable thing has one
reference form, `<kind>@<anchor>@<id>`, and two mechanisms of the
checker give a reference its value. `cargo klarch show <ref>` prints every reference to an entry,
so what depends on an entry is computed from the tree and never written by hand. An entry that is
deleted or renamed dangles every reference to it, and `cargo klarch check` reports each one, per
`design@core@an-unresolved-reference-is-reported`, so the repair list a change produces is the
list of texts that depended on what changed. A reference is
therefore a claim of dependence: this text is to be revisited when that entry is reversed, closed,
fired, abandoned or renamed.

**The test for writing one is that claim.** Where a change to the entry would leave the text
unaffected, the reference is decoration and costs a repair for nothing; where it would not, the
reference is owed, or the change reaches nobody. What follows from the test, by the kind of text:

| the text | names | so that |
| --- | --- | --- |
| a design head | the goal its argument derives a constraint from | `show` on the goal lists what abandoning it reopens |
| a design head | a decision of another component it depends on | a reversal reaches it |
| an issue entry | the decision it strains, and the goal it threatens when it does directly | `show` on a decision lists what is outstanding against it before it is reopened, and on a goal what stands between the project and it |
| a guard, a workaround, a stub or a test that pins behaviour an open entry describes | the issue it exists because of, in the comment at the site | closing the entry dangles the comment, so the site is revisited and the workaround removed |
| a tripwire | the decision it guards | a reversal dangles its tripwires |
| a design head | the `rejected` entries its argument covers | a session reading the head meets the designs it did not keep, and an entry that goes `void`, is reopened or is renamed reaches the head |
| a commit message | every entry it opens, closes, reverses or argues from | the commits gate judges it against the tree it was written against |
| a restatement of a directive sentence, per `design@agent-skills@restatement-size-test` | its home | a drift between the two is found from either end |

**An entry never lists what references it.** The inbound list is `show`'s to compute, and a
hand-written one is stale at the next reference written elsewhere.

**A reference in prose is checked wherever it stands**, a Rust comment and a fenced block
included; a string literal bound to a name yields none, per
`design@core@a-reference-is-live-in-every-prose`. So a comment in code naming an issue is
as live as a sentence in a document, and closing the issue reaches the code. This serves
`goal@knowledge-architect@design-is-recorded-with-its-arguments`.

### A sentence about the past whose reference dangles is rewritten to the present or removed, except a deleted skill's or agent's reference in a released changelog section and an absent entry, skill or agent in a record of a deliberation, and a quotation of the owner is kept with a reference beside it `##a-past-sentence-is-rewritten`

When an entry is deleted or renamed, a sentence that recorded its past, such as what an earlier
step wrote, is rewritten to state the present, or removed, and its history stays in the commit
messages. Retargeting it to the new name would make it false, and plain text would take it out of
the check, per `design@agent-skills@plain-text-is-no-repair`. A verbatim quotation of the
owner that names a renamed entry is left as it is, with a reference to the current entry beside
it, since rewriting it at a rename, with no change to what it decides, risks misstating the owner,
the risk `design@agent-skills@attribution-replaces-quotation` names, against
`goal@knowledge-architect@the-owner-decides`.
In a released section of CHANGELOG.md, a reference to a deleted skill or agent goes back to the
bare name instead, per `design@agent-skills@plain-text-is-no-repair`. In a record of a deliberation,
an entry its tree does not hold is named by its bare slug instead, and a skill or an agent by its
bare name, per the same head.

### Plain text is never the repair of a finding, except text in the checker's syntax beside the issue that records its missing form, a deleted skill's or agent's name in a released changelog section, an absent entry's, skill's or agent's bare name in a record of a deliberation, and a quotation, and no instruction offers an unchecked form as the way to clear one `##plain-text-is-no-repair`

A form a writer can use is either one the checker judges, and the workflow recommends it, or one
the checker does not read, and the workflow never directs a pointer into it to clear a finding. So
no finding's repair, and no installed or project instruction, offers an unchecked form as the way
to clear a finding. A repair names a checked form: the right anchor, `path@elsewhere@<path>` for a path this
tree does not hold, `planned@<anchor>@<path>` in a plan document for a path its work will create,
per `design@core@planned-path-form`, an angle-bracket placeholder for an illustration, or a
rewrite of the sentence.
One exception is text in the checker's syntax that no checked form expresses, written without
backticks beside a reference to the issue entry that records the missing form. The check reads it:
a reference-shaped token outside backticks is reported, and is silent where its paragraph holds a
backticked reference to an issue entry, per `design@core@reference-shape-outside-span`. Prose that mentions a
directory without asking the reader to follow it is not a repair and is outside this head.

The checker cannot express every reference a project needs. The first exception covers text in the checker's
syntax, which would be read as a candidate if it were backticked: a reference,
`<kind>@<anchor>@<id>`, or a path of two or more segments. Where no checked form expresses what such
text points at, it may be written without backticks, and only beside a reference to an issue entry
of the writing project that records the missing form. Any other text that names something, in the
tree or outside it, such as another project's commit, an address on the web or a description in
words, is outside the exception. Whether it needs a reference is decided by
`design@agent-skills@a-reference-claims-a-revisit`. A project that meets a gap of the checker itself opens that
entry in its own register, since a reference resolves only inside its own project, per
`issue@core@cross-project-references`. A need that a checked form already serves is not a gap.

The entry owes its `Why it matters` and its `What would close it`, so the escape is available and
never free. `cargo klarch show` on the entry lists every site, and closing it, once a checked form
ships and the sites are converted, dangles each one, so the conversion list is computed. Plain text
justified in a commit message alone lost: nothing finds the site again, and nothing revisits it
when the form ships. A generic checked opt-out marker lost too: one marker fits every finding, so
it becomes the cheap silence `design@core@reserved-anchors` refuses, while a gap concrete enough to
name is closed by shipping its own form.

The scope is the checker's syntax, the owner's proposal: the rule was meant for plain text that
copies the checker's syntax but omits the backticks to avoid the check, which the owner thought a
better scope. It lets a writer tell from each span alone whether the exception
applies, per `goal@knowledge-architect@agents-get-a-complete-workflow`: every sentence names
something, and a scope of "any pointer" would ask for an issue entry beside every mention of a
thing outside the project. The evasion the exception exists to stop is a reference or a path with its
backticks removed, which this scope covers. A scope by target, any text naming something the tree
holds, lost: when such text needs a reference is already decided by the rule on references, and
whether a phrase names something cannot be decided span by span.

A commit named by its subject, as `design@knowledge-architect@no-branch-sha-is-cited` directs for a commit of the
branch, is outside the exception. It names history as git names it. A reference resolves against the
entities of a tree, per `design@core@one-entity-table`, and a commit is none of them; the one
citation of a commit the checker judges is a branch commit's SHA, which
`design@core@branch-shas-are-refused` refuses.

The second exception is a deleted skill's or agent's reference in a released section of
CHANGELOG.md: the section's content never changes, per
`design@knowledge-architect@changelog-section-fixed-at-release`, and nothing exists that a
reference could cite.
Its repair is the bare name, which `design@core@backticked-bare-id` leaves silent. It evades
nothing, since no checked form remains that it could have used. The rival, rewriting the sentence
as `design@agent-skills@a-past-sentence-is-rewritten` asks elsewhere, would change a released
section's content.

The third exception is an entry that the tree of a record of a deliberation does not hold, nor, for
a commit message, its first parent's, removed or renamed by the work the record carries, or still to
be created by it, named there by its bare slug in backticks. A skill or an agent the work removes
or is still to create is in the same case, on the owner's word, named by its bare name in
backticks, which `design@core@backticked-bare-id` leaves silent while the tree does not hold
it. A record of a deliberation is where the deliberation is kept: a plan document
or a commit message, per `skill@knowledge-architect-decision-recording@three-homes`, and a committed
file that leaves once its outcomes are carried out, per `primer@design-heads`. The exception
for an entry the work removes or is still to create is the owner's proposal: for such an entry the
checker has nothing left to check, and what this head refuses is avoiding the check on a live
entry, which leaves dangling references and makes grounding less efficient. A renamed entry's old
slug is in the same case, since nothing resolves it once the rename lands. It evades nothing, as the changelog exception does not: no entry remains,
or none exists yet, that a checked form could cite, and a bare slug no entry defines is silent, per
`design@core@backticked-bare-id`. The owner chose the form among three, the bare slug
in backticks, the bare slug in plain text and the full reference with its backticks removed, and the
scope:
records of a deliberation only, since every other document states current reality, where a
sentence naming an absent entry is rewritten, per `design@agent-skills@a-past-sentence-is-rewritten`.
The full reference with its backticks removed is the evasion this head refuses. The bare slug in
plain text, the owner's first proposal, gave way to the owner's choice of the backticked slug. A
description in words, the form the audit report's issue proposed, the owner judged confusing to
every reader. A
checked form for an entry still to be created, as `design@core@planned-path-form` is for a path,
lost too: a plan document and a committed file leave with or before the branch that creates the
entry, so such a check would fire on a file about to be deleted; a commit message is judged against
its own commit's tree, where the check would assert only that the entry is absent, which nothing
reads again; and the form would need a grammar of its own in the core.
A renamed entry still exists, under its new slug, so its old slug carries beside it the reference
to the live entry, the shape a quotation of the owner naming a renamed entry already takes, per
`design@agent-skills@a-past-sentence-is-rewritten`: the old slug alone would leave a reader no
checked way to the entry it names.

The fourth exception is a quotation, which none of the lints on ids reads, per
`design@core@quotation-read-by-no-id-lint`, where its definition and its argument are. A quotation
of the writer's own words, made to clear a finding, is the evasion this head refuses.

The set of exceptions is closed: each lets a finding clear without a checked form,
so another one weakens the check, and is argued as a change of this decision.

The argument: an unchecked form that clears a finding clears it for good, so a habit of writing one
empties the check while every run still passes, against
`goal@knowledge-architect@documentation-stays-consistent`. Two narrower stances of the checker
make the same argument: `design@core@the-regime-has-no-opt-out`, where no declaration exempts a
document from a rule, and `design@core@reserved-anchors`, where the escape anchor is refused on a
path this tree holds, since it would otherwise silence the finding on a real path.

A label beside a checked pointer is not such a form. A plan item named outside its plan as #<id>
stands beside a citation of the whole plan, because an item reference is refused there, per
`design@core@plan-item-scope`.

The design this decision did not keep: `rejected@agent-skills@checked-commit-form-by-subject`.
