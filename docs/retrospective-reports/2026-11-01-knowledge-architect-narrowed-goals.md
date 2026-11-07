# 2026-11-01-knowledge-architect-narrowed-goals

- **Received file**: 2026-11-01-knowledge-architect-narrowed-goals.md, the project file of a retrospective of this repository.
- **Version used**: knowledge-architect 0.9.0, the version the project pins.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@agent-config@a-rebase-continue-strips-a-message-s-hash-lines`, `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`, `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`, `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`.

## P1, with C2 of its workflow file: a slug in the definition form inside a verbatim quotation is reported as a misplaced definition

- **a.** What it says: the owner's message quoted two goal slugs in the definition form, a hash
  pair and the slug, in backticks. Copied verbatim into the goal-review record, the checker
  reported each "written mid-line" and "defines nothing". The session removed the backticks and
  said so beside the quotation, with no issue reference, which the primer's rule on a pointer
  written bare does not admit (P1). Proposals: a form marking a block as a verbatim quotation, or
  the misplaced-slug finding ignoring a span inside a block-quote line (C2); open the issue that
  records the missing form, and cite it next time (P1).
- **b.** Whether it still applies: yes, reproduced at HEAD over a mock project made by `klarch init`
  in this group's scratch directory: a line `The owner wrote: "rename` followed by the definition
  form of a goal's slug in backticks, and a block-quote line holding the same span, each give
  "is written in the middle of a line and defines nothing", with the repair "move it there, or
  delete it". The code: `path@model@src/scan.rs`, line 617, records every slug-shaped span of a line
  as a definition site with no regard to a quotation; `path@model@src/entity.rs`, line 1286, writes
  the finding.
- **c.** Whether it is already recorded: no issue. Two heads bear on it:
  - `design@model@a-slug-elsewhere-defines-nothing` reports a slug "in the middle of a line",
    because "A mid-line slug is a pointer written in the definition form, and recording it is what
    makes such a pointer visible".
  - `design@check@quotation-read-by-no-id-lint` exempts a quotation, text between double quotes or
    a block-quote line, from the three lints on ids alone, on the argument "A quotation is someone
    else's words … which its writer may not respell, so a finding there would have no repair the
    writer may make". The misplaced-slug finding is no lint on ids, so the exemption does not reach
    it.
  - `design@agent-skills@a-past-sentence-is-rewritten` keeps a verbatim quotation of the owner as it
    is.
- **d.** Its kind: wrong, a finding with no repair the writer may make: the only repairs it names
  alter the owner's words.
- **e.** Its scope: the checker, `model`; it serves `goal@knowledge-architect@documentation-stays-consistent`
  and the attribution rule of `primer@design-heads`. Exempting a quotation adds an exception to
  `design@model@a-slug-elsewhere-defines-nothing`, a change of that head's statement, so it goes to
  `skill@knowledge-architect-design`.
- **f.** A better fix: the nearest rival is a new mark for a verbatim block, as C2's first shape. The
  deciding fact: the checker already has a quotation, defined once in
  `design@check@quotation-read-by-no-id-lint` and computed by `path@model@src/scan.rs`; a second
  definition of a quotation would be a second home. The argument that exempts the lints covers the
  misplaced slug, since a slug quoted from the owner defines nothing either way.
- **g.** Its route: an issue, since the change is the checker's and changes a head.
- **h.** The proposal: open an issue in `model`, kind `design`, id
  `a-slug-in-a-quotation-is-reported-as-a-misplaced-definition`.
  - What: a slug in the definition form inside a quotation, as the owner writes one when naming a
    goal, is reported as a misplaced definition, whose repairs alter the quoted words; reproduced
    over a mock project.
  - Why it matters: it strains `design@check@quotation-read-by-no-id-lint`'s argument and
    `design@agent-skills@a-past-sentence-is-rewritten`, and pushes a session to edit the owner's
    words or write a bare pointer.
  - What would close it: a discussion under `skill@knowledge-architect-design` that extends the
    quotation's exemption to the misplaced-slug finding, rewriting
    `design@model@a-slug-elsewhere-defines-nothing`, with a test over a mock project, or keeps the
    head and states why.
  P1's own repair is then that issue, cited beside the next such quotation, as the primer's rule
  asks. The record P1 names left with its batch, so no site remains to repair.
- **Default**: open the issue.
- **Outcome**: decided by the session: open `a-slug-in-a-quotation-is-reported-as-a-misplaced-definition`
  in `model`, kind `design`. Ground: reproduced at HEAD; no record holds it; the fix changes a
  head's statement, so it waits for a discussion, per `primer@design-heads`.

## P2, with C3 of its workflow file, P2 of 2026-11-01-knowledge-architect-pre-release-fixes with C3 of its workflow file, P4 of 2026-11-02-knowledge-architect-development-and-components, and W3 of 2026-11-01-thaum-design-record-audit-klarch-workflow: no command for the reword, the fold, the move to the front, or the wait on CI (cluster H)

- **a.** What it says.
  - P2 here: right after `gh pr ready`, the head carries a skipped run and a run in progress; the
    session wrote a polling loop twice, and the first stopped while a run was in progress. Proposal:
    add the wait to the What of `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`.
  - C3 of the workflow file: two rewords with `git filter-branch --msg-filter` and a Python script;
    once the session's count of changed commits read `git range-diff` with the wrong pattern and gave
    0 where one commit had changed. Proposal: a command that applies a message to a commit and
    reports the commits whose message changed and whether any tree did.
  - P2 of pre-release-fixes, with C3 of its workflow file: a stricter check landed mid-branch; the
    session moved its commit to the front with a scripted sequence editor; the replay stopped twice
    on a generated index and left a later commit's index stale. Proposal: a command, or one procedure
    in point 2 of the root CLAUDE.md naming the regeneration of generated files at each replayed
    commit.
  - P4 of development-and-components: the reword procedure ran five times in two slices, each with a
    new script. Proposal: `cargo x reword <subject> <script>`.
  - W3 of the thaum design-record audit: the harness refuses `git rebase -i`, and the installed text
    names no recipe; the session replayed by cherry-pick, and its first attempt amended the wrong
    commit. Proposal: one non-interactive recipe at the installed history-edit instruction (+1).
- **b.** Whether it still applies. At HEAD, `path@knowledge-architect@CLAUDE.md` gives the reword
  (line 232) and the fold (line 241) as procedures, the merge predicate as a check that a run on the
  head succeeded (line 296, line 289 at `ccc5057d`) and no wait; and no procedure moves a commit to
  the front. `path@xtask@src/` holds the commands `gates`, `changelog`, `skill` and `mutate`, and
  none of these. The installed text names no recipe: `path@agent-skills@content/skills/review/SKILL.md`,
  line 203, "by a history edit of the branch after that", unchanged since `v0.9.0`. Every part
  still holds. One fact bears on the move to the front: `path@agent-config@knowledge-architect/PRIMER.md`,
  line 447, offers the squash as the other move, "goes in the branch's first commit … or the branch
  is squashed to one commit before its review".
- **c.** Whether it is already recorded.
  - `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command` records the reword,
    the fold and the merge predicate, and one further instance; it holds no wait.
  - `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history` records the three
    tasks `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` takes in (a conflict-free rebase
    onto a moved main, a reword, a fold). The move of a commit to the front is not among them; that
    goal says any other such task is owed once the owner takes its generic part in. Its leads name
    the owner's Python tool, whose commands include `move-commits`.
  - `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing` records that
    the installed set has no skill for branch work, rewording and folding included, which is where
    W3's recipe would live.
  - `issue@agent-config@a-rebase-continue-strips-a-message-s-hash-lines` records a defect of the
    conflict-resolution step a replayed reorder also runs.
- **d.** Its kind: missing tooling, recorded, recurring. The count over this intake's files: rewords
  by hand in ten sessions of this repository (the files of 10-31 intake-seventeen-files, 11-01
  narrowed-goals, 11-02 development-and-components, 11-03 component-rules-revision, 11-04
  goal-standard-revision, model crate and check-renamed, 11-06 cli-on-top and consumers-out, 11-07
  integrated-tests), and history edits with no recipe in three sessions of thaum (the design-record
  audit, the count-ceilings and the log-target slices). The wait on CI: three sessions (10-31
  transcript-issues, whose first wait loop stopped on the skipped run; 11-01 narrowed-goals; 11-01
  pre-release-fixes). The move to the front: one.
- **e.** Its scope: `goal@xtask@repeated-tasks-are-automated` for this repository's own procedures,
  the reword, the fold and the merge predicate with its wait;
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` for the published tasks. Owner of the
  fix: `xtask` and a future published crate. No fix reverses a decision.
- **f.** A better fix. The rival to W3's recipe in installed text: the published command the goal
  takes in, which the goal's "Rules out" names, "installed text that has each session write its own
  script to reword a commit that is not the newest"; the recipe would be text for every project that
  the command replaces, so the instance goes to the issue of the command. The rival to a procedure
  for the move to the front (+1 instruction in the root): the squash the primer already offers, and
  one instance; so no procedure, and the move is named in the published issue as a task whose taking
  in is the owner's.
- **g.** Its route: no change to any text; edits of three issue entries.
- **h.** The proposal:
  - `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command` gains the wait in its
    merge predicate (the head's runs, one skipped while a draft and one in progress, waited for until
    none is in progress, then the `success` test), and the instances: the ten sessions above for the
    reword, with C3's misread `range-diff` count as the failure a command removes; the three for the
    wait.
  - `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history` gains the three thaum
    sessions, the harness's refusal of `git rebase -i` among the facts the command meets, and the
    move of a commit to the front, with the regeneration of the generated files at each replayed
    commit, as a task not taken in, whose taking in is the owner's word.
- **Default**: no change, with the reason: recorded; the two issues gain the instances, the wait
  and the move to the front.
- **Outcome**: decided by the session: no change to any text; the two issues gain what this cluster
  found. Ground: the tasks are recorded by the two issues, which the goals
  `goal@xtask@repeated-tasks-are-automated` and `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`
  ground; a recipe in installed text is what the second goal rules out; the move to the front has
  the squash of `primer@branch-commits` as an existing move, and its taking in is the owner's per
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

## P3: the root CLAUDE.md's opening paragraph is not revisited when the goals change

- **a.** What it says: the root's opening restates the project's purpose in prose, "A
  documentation checker and an agent workflow, shipped as Rust crates", with no reference, so
  nothing reached it when the scope changed; the session judged it true. Proposal: cite
  `register@knowledge-architect@goal` or name `goal@knowledge-architect@serves-one-kind-of-project`.
- **b.** Whether it still applies. At `ccc5057d` the paragraph already ended "The main goals are
  declared `register@knowledge-architect@goal` and Components refine or add to them"; at HEAD the same,
  `path@knowledge-architect@CLAUDE.md`, lines 5 to 8. The paragraph was revisited at a later goal
  change: the commit "[knowledge-architect] The goals the owner ruled on the duties of the checker,
  the model, the listings and the reviews" (`a00db07d`) rewrote its second sentence, "stay consistent
  with its code and with each other", to "meet the rules it holds and those the project declares".
  Its first sentence is true at HEAD.
- **c.** Not recorded.
- **d.** Its kind: an observation. The paragraph stayed true through two goal changes, and a goal
  session reached it by judgement once.
- **e.** Its scope: `goal@knowledge-architect@agents-work-without-drift`; owner: the root's own text.
- **f.** A better fix. The proposed reference fires only when the goal it names is removed or
  renamed, since a reference that resolves is never reported; a reworded goal leaves the paragraph
  unreported all the same, so the reference would not reach the case the finding names. The
  paragraph already cites the register. No change.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, an observation.
- **Outcome**: decided by the session: no change. Ground: the paragraph cites the goals' register
  and was revisited at a later goal change (`a00db07d`); a reference to one goal would not report a
  rewording, which is the case named.
