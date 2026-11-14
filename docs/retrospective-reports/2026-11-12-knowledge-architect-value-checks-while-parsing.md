# 2026-11-12-knowledge-architect-value-checks-while-parsing

- **Received file**: 2026-11-12-knowledge-architect-value-checks-while-parsing.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from this repository's own source: main at `v0.9.0-369-g8e23dd57`." / "Project main commit at the start: `8e23dd57`. Branch: `value-checks-while-parsing`. The tree was clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@cli@exit-code-ladder`: constrains, respect.
  - `design@agent-skills@questions-go-to-their-authority`: constrains, respect.
  - `design@agent-skills@intake-outcome-authority`: constrains, respect.
  - `rejected@agent-skills@owner-rules-each-intake-finding`: constrains, respect.
  - `design@agent-skills@derived-decision-proceeds-on-default`: constrains, respect.
  - `design@agent-skills@default-ruled-before-merge`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@knowledge-architect@versioning-policy`: conflicts, check-reversal.
  - `issue@knowledge-architect@command-output-is-not-declared-a-contract`: depends-on, rule-before-work.
  - `goal@the-owner-decides`: constrains, respect.
  - `design@cli@command-parsing-home`: constrains, respect.
  - `design@cli@arguments-parse-through-clap`: constrains, respect.
  - `design@cli@a-refusal-by-arguments-writes-nothing`: constrains, respect.
  - `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation`: touches, respect.
  - `design@create@init-name-and-path`: constrains, respect.
  - `design@create@init-refuses-the-installer-s-prefix`: constrains, respect.
  - `design@testing@binary-tests-in-testing`: constrains, respect.
  - `design@agent-skills@default-code-shape`: constrains, respect.
  - `design@agent-skills@test-economy`: constrains, respect.
  - `design@agent-skills@integrated-test-through-public-api`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `design@agent-skills@top-level-binary-crate`: constrains, respect.
  - `issue@check@a-home-for-developer-contracts-outside-agent-configuration`: touches, respect.
  - `design@knowledge-architect@cli-crate`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.

## contract-fix-class-unclear: the versioning class of a fix that brings a command to its documented contract

Members: 2026-11-12-knowledge-architect-value-checks-while-parsing P2.

- **a. What it says.** The session made `--since` refuse `2026-13-45` with exit 2, where the code had answered exit 1 and the README's exit table already put it at 2. `design@knowledge-architect@versioning-policy` classes as major "a command change that breaks existing usage"; the session classed it major, reading the row literally. Under 0.x a major and a minor bump the same position, so no version number changed. A fix toward a documented contract has no row. Proposal: none, "unless the owner wants such a fix classed by the contract it meets rather than by the code it changes", since "the table is the owner's scheme".
- **b. Whether it still applies.** On origin/main the row stands, `register@knowledge-architect@design` line 599, and the head says "The table is the owner's scheme. The bullets below refine it." No bullet names a fix toward a documented contract. The README's table, `path@cli@README.md` line 83, puts at exit 2 "`transcript` with no transcript directory, over a file it cannot read, or with a regular expression or a `--since` date that does not parse", and line 85 says "A caller scripting against a run reads the exit code", so the exit code is a documented contract. It applies, as an unclear case of the table.
- **c. Whether it is already recorded.** `issue@knowledge-architect@command-output-is-not-declared-a-contract` (`depends-on` / `rule-before-work`, agent-15), kind `design`: "The versioning policy classes changes to "a command" without saying which part of a command's behaviour is its usage", which is this question: whether code that departs from its README is the usage a change "breaks". Its re-entry point: "That discussion, opened on the owner's word, at the latest before the project leaves 0.x". `design@knowledge-architect@versioning-policy` (`conflicts` / `check-reversal`, with an owner-question): a row for the case adds a member to a table the head attributes to the owner. `design@cli@exit-code-ladder` (`constrains`): the contract the change meets. `design@agent-skills@questions-go-to-their-authority`, `design@agent-skills@intake-outcome-authority` and `design@agent-skills@derived-decision-proceeds-on-default` (`constrains`): the question is the owner's only where it is a fork or a goal; `rejected@agent-skills@owner-rules-each-intake-finding` (`constrains`): no ruling is owed on each finding.
- **d. Its kind.** A defect of the text, unclear: the major row's "breaks existing usage" admits two readings for a change that brings a command to its documented exit code. One instance, with no effect on a version number under 0.x.
- **e. Its scope.** `goal@every-target-project-can-adopt-it`, which the issue names: under 0.x a patch and a minor are different numbers. The owner of the fix: the root design home, whose table is the owner's scheme. A row added now would add a member to that table, which `primer@design-heads` sends to the owner as one proposal, since the table's words name its members and its choice was the owner's.
- **f. A better fix.** The rivals: put a row to the owner now, as the verdict's owner-question frames it, or record the case in the design issue whose discussion decides which part of a command is its usage. The fact that decides: the case turns on that very question, whether a README's exit table or the code's behaviour is the usage, so a row ruled now fixes one answer of a policy before it is designed. The two readings give different numbers under 0.x only in a release whose other entries are all patches, since a major and a minor bump the same position and a patch bumps the other, per `design@knowledge-architect@versioning-policy`; in the release that met it, 0.10.0, other entries made the bump a minor, so the class changed no number, and the case waits with the issue that already holds that cost in its Why it matters. The issue's discussion is opened on the owner's word, so the owner keeps the question; recording the instance decides nothing the owner holds.
- **g. Its route.** An issue only: an instance added to `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
- **h. The proposal.** Add the bullet below to that entry's list of instances, under `### What`. No change to the table. Instructions: 0.

Default: an instance added to `issue@knowledge-architect@command-output-is-not-declared-a-contract`; the table unchanged.
Outcome: decided by the session: no change to the table, an instance added to the recorded design issue. Ground: the case turns on the question that issue holds, which part of a command's behaviour is its usage; a row now would add a member to the owner's table ahead of that discussion, per `primer@design-heads`; the class changed no number in the release that met it, and the cost of a wrong class under 0.x is the one the issue's Why it matters already holds; recording the case decides nothing of the owner's.

```text
- A fix toward a documented exit code: `--since` of `klarch transcript` refused an impossible date
  such as `2026-13-45` with exit 1, while `path@cli@README.md`'s exit table put it at exit 2; the
  branch that made it exit 2 classed the change major, reading "a command change that breaks
  existing usage" literally, from 2026-11-12-knowledge-architect-value-checks-while-parsing P2.
  The release's other entries made its bump a minor, so the class changed no number. Whether such
  a fix is classed by the contract it meets or by the code it changes turns on which part of a
  command's behaviour is its usage, so the row this entry's discussion gives the exit code answers
  it.

## declaration-check-hides-tool-call: a check moved into the clap declaration hides the tool's own call from every binary test

Members: 2026-11-12-knowledge-architect-value-checks-while-parsing P1.

- **a. What it says.** The session moved three value checks from the tools into the clap declaration. Once clap refused `--name knowledge-architect-x`, the binary test `a_refused_name_writes_nothing` no longer reached `init`'s own call of `check_name`, so removing that call survived the whole suite; the same held for the re-check of `--since` in the transcript tool's `list` and `search`. The session's mutations missed it; two adversarial reviewers found it, and the repair added a PATH case and an in-process test. It blames `path@cli@CLAUDE.md`, "A new command is a declaration here and a function there, and a check of a plain value goes in the declaration.", and `design@cli@command-parsing-home`, which keeps the rule in the tool's crate. Cause, per the report: none of the four kinds, a trap of the code as it stands, whose home is the scoped `CLAUDE.md`. Proposal: one sentence after that one, "The tool's own call of that rule is then reached by no test that passes the value through the binary, so it is tested in process, or through a path the declaration does not see, such as a name taken from PATH." (+1, 0 removed). Rival: nothing, relying on the adversarial axis; "The owner weighs it."
- **b. Whether it still applies.** At the version, `8e23dd57`, `git show 8e23dd57:crates/cli/CLAUDE.md` holds the sentence at line 5; on origin/main `path@cli@CLAUDE.md` line 5 holds it unchanged, with no sentence on the trap. The code: `check_name` is public, `path@create@src/command.rs` line 61, and `init` calls it at line 154, so the call behind the declaration still exists. The repair's test, `a_refused_name_writes_nothing` at `path@cli@tests/init.rs` line 169, is on main. The trap stands for the next check moved into a declaration.
- **c. Whether it is already recorded.** `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation` (`touches`, respect) lists survivors of the commands' moved code, `path@transcript@src/command.rs` among its files, but none of a backstop behind a declaration. `issue@check@a-home-for-developer-contracts-outside-agent-configuration` (`touches`, respect), deferred: the owner intends a home for developer contracts outside the agent configuration; its trigger, "The owner opens the design session they announced ... or the configuration for several agent providers is designed", is not met by this subject, and until then the primer's knowledge table puts "a contract or a trap that only a developer needs, true of the code as it stands" in "the scoped `CLAUDE.md` nearest the code". The heads `design@cli@command-parsing-home` ("a test holds the declaration and the conversion each"), `design@cli@arguments-parse-through-clap`, `design@cli@a-refusal-by-arguments-writes-nothing`, `design@create@init-refuses-the-installer-s-prefix` and `design@agent-skills@integrated-test-through-public-api` (`constrains`, respect) are kept by the sentence: it adds a test of the backstop beside the declaration's own, and the integrated head admits "a consumer path" such as the PATH case. `goal@only-needed-instructions` (`constrains`): the proposal owes the showing that judgement did not handle the case.
- **d. Its kind.** A defect of the text, missing: a provable gap. The rule "a check of a plain value goes in the declaration" and the tool's own call of the same rule, which `design@cli@command-parsing-home` keeps, together make the call unreachable from any binary test that passes the value; that follows from reading, and the session's own mutations, the instrument the development skill gives it, missed it.
- **e. Its scope.** `goal@the-workflow-leads-to-better-code`. The owner: the `cli` Component's scoped `CLAUDE.md`. It reverses nothing: it states a consequence of the heads above.
- **f. A better fix.** The nearest rival is no text, the adversarial axis catching each instance. The fact that decides: the knowledge table routes a trap true of the code to the scoped `CLAUDE.md`, so the record answers where it goes, and the act it changes is the test written with the move, where the rival costs a review finding and a repair commit each time; the judgement of the session that made the move did not catch it. The question of the deferred issue, where developer contracts live, does not change where this one goes today.
- **g. Its route.** A text edit of a scoped `CLAUDE.md`, under `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** In `path@cli@CLAUDE.md`, after "and a check of a plain value goes in the declaration.", add the block below. Instructions: 1 added, 0 removed. Nearest rival: no text, defeated in item f. Who holds it: the report says the owner weighs it, but the knowledge table and the provable gap settle it, so it is no fork, per `primer@question-authority`.

```text
The tool's own call of that rule is then reached by no test that passes the value through the
binary, so it is tested in process, or through a path the declaration does not see, such as a
name taken from PATH.
```

Default: handle now, the sentence above.
Outcome: decided by the session: handle now. Ground: a provable gap (item d), a trap true of the code whose home the primer's knowledge table names, `primer@where-knowledge-goes`; the session's mutations missed it, and the adversarial review's catch cost a repair commit; it keeps every `cli` head the search returned.
