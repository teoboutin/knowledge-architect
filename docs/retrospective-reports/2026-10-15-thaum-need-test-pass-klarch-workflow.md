# 2026-10-15-thaum-need-test-pass-klarch-workflow

- **Received file**: 2026-10-15-thaum-need-test-pass-klarch-workflow.md, the workflow file of a
  pass in thaum, scoped by its owner after the pin move to 0.7.0, applying the need test and the
  attribution rule to the project's own text.
- **Version used**: the published checker, 0.7.0, read at the tag `v0.7.0`.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**: `tripwire@agent-skills@owner-intent-stripped` and
  `tripwire@agent-skills@attribution-loses-the-scope`, near W2 and not met by it.

## W1, with W5 of 2026-10-15-thaum-pin-move-0-7-0-klarch-workflow. The cold-implementer reviewer's scope reads "the skills installed in it"

- **a. What it says.** The reviewer's definition scopes its reading to "the skills installed in it".
  "Installed" names the files the install writes, so the scope can be read to exclude a project's
  own skills, which that project's planning procedure needs. The project resolved it in its brief.
  The fix proposed: "every skill under the agent configuration directory, the project's own
  included".
- **b. Whether it still applies.** Yes. `path@agent-skills@content/agents/cold-implementer-reviewer.md`,
  lines 10-11, unchanged at both commits: "with nothing but the document, the repository and the
  skills installed in it." The primer gives "installed" that sense: its section "The installed
  skills" ends "A project's own skills add to these". Line 29 of the same file already has the
  reviewer read `skill@knowledge-architect-planning` "and every other skill the document names", so
  a project skill the document names is read; the two readings remain for one it does not name.
- **c. Whether it is already recorded.** No; no `rejected` entry covers it.
- **d. Its kind.** Unclear, narrowly: an instruction whose words allow two acts, provable by
  reading.
- **e. Its scope.** Installed text, no head on the reviewer's scope.
- **f. A better fix.** The proposal names a directory, which varies with the harness. The rival is
  a rewording of the same phrase: "the skills it holds, a project's own among them". No change loses
  to the reading of a project that had to correct the reviewer in every brief. Instructions added: 0.
- **g. Its route.** Installed text edit.
- **h. The proposal.** Handle now: the phrase becomes "the skills it holds, the project's own
  included".
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W2, with W4 of 2026-10-15-thaum-pin-move-0-7-0-klarch-workflow. The attribution rule's list does not name a `rejected` entry

- **a. What it says.** The paragraph "A durable record attributes a decision to the owner; it does
  not quote the owner" lists "A head, a commit message, a tripwire and an issue entry". A rejected
  alternative is not listed; the session read it as covered by "A durable record". The fix proposed:
  name it, or say the list is an example.
- **b. Whether it still applies.** Yes, unchanged: `path@agent-skills@content/PRIMER.md`, lines
  494-497. Since 0.7.0 the rejected designs are entries of a built-in register, `rejected`, so the
  omission now concerns a register's entries.
- **c. Whether it is already recorded.** No. `tripwire@agent-skills@owner-intent-stripped` guards
  the removal of the owner's ground from a head, and
  `tripwire@agent-skills@attribution-loses-the-scope` a misstated attribution; neither covers the
  list. `rejected@agent-skills@owner-words-quoted-in-head` is about heads.
- **d. Its kind.** Unclear: the opening sentence admits every durable record, and the list that
  follows names four.
- **e. Its scope.** The decision is `design@agent-skills@attribution-replaces-quotation`, whose body
  carries the same list of four. A `rejected` entry is a member its rule, "a durable record",
  admits, and its argument covers it: the cost a verbatim quotation imposes on the checker's syntax,
  and the owner's decisions shown as the owner's, hold for a `rejected` entry as for a head. Part of
  the head's ground is the owner's judgement on the value of verbatim quotations. Whether the four
  were the owner's words naming the members is not established from the head, so the member goes to
  the owner as one proposal, as `primer@design-heads` says of a new member: this section is that
  proposal.
- **f. A better fix.** Rivals: name the `rejected` entry in the list and in the head (the
  proposal's first form); say the list is an example (its second form), which leaves the next
  register a project declares to the same reading; or rewrite the list as the rule, "every record
  that is not where the deliberation is kept", which `primer@design-heads` asks of a title, not of a
  sentence of the body. The first is the smallest edit that closes both readings, and the opening
  sentence already carries the rule for any other record.
- **g. Its route.** Installed text edit, and the head's body in the same commit, under
  `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** Handle now: the primer's list and the head's list name "a `rejected` entry".
- **Default**: handle now, as h says.
- **Outcome**: accepted.
