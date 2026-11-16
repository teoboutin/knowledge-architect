# The deliberation of the doc gate and the documentation lints

The rounds are numbered by the owner's messages, as the status section of the README names them.
The agent's replies to rounds 2 and 3 carry the titles "Round 1" and "Round 2": those titles count
the agent's design rounds, not the owner's messages, and every item below counts the owner's
messages.

## Threads

### The `doc` gate goes in `rust_project` of the gates library, after `clippy`, so that every adopting project gets it `##doc-gate-home`

- **Proposed by:** the agent, in its reply to round 1, as shape A against shape B, the gate in
  xtask alone, with A as its recommendation.
- **Final state:** approved.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-1`,
  `argument@doc-gate-and-lints@arg-2`, `argument@doc-gate-and-lints@arg-3`,
  `argument@doc-gate-and-lints@arg-4`, `argument@doc-gate-and-lints@arg-5`,
  `argument@doc-gate-and-lints@arg-8`, `argument@doc-gate-and-lints@arg-9`,
  `argument@doc-gate-and-lints@arg-10`, `argument@doc-gate-and-lints@arg-30`,
  `argument@doc-gate-and-lints@arg-32`, `argument@doc-gate-and-lints@arg-33`.
- **Shape:** the README's section "Decided design, doc gate home".
- **Harvest:** the head of `thread@doc-gate-and-lints@recommended-gate-list`, which step 3 writes from
  `design@gates@a-project-holds-its-gate-list` and `design@gates@distill-over-full-logs`, per the
  Harvest section of the README; the setup skill's list and the other restatements of the list are
  repaired at step 1.
- **Ruling:** in round 2 the owner said they favoured A, as the agent did. The agent held that
  word as presumed-settled until the record search had run. In round 3 the owner agreed with every
  proposal of the reply to round 2. The search defeated nothing, and the agent closed the thread
  approved in its reply to round 3. The rival B lost on `argument@doc-gate-and-lints@arg-8` and
  `argument@doc-gate-and-lints@arg-10`.

### The `doc` gate denies every rustdoc warning by setting `RUSTDOCFLAGS="-D warnings"` itself `##doc-gate-denial`

- **Proposed by:** the agent. The reply to round 1 named the command; the reply to round 2 made it
  a thread, shape (a), against (b), a plain `cargo doc` with an ask that each project declares
  `[lints.rustdoc]`, and (c), `--config build.rustdocflags`.
- **Final state:** argued.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-11`,
  `argument@doc-gate-and-lints@arg-12`, `argument@doc-gate-and-lints@arg-13`,
  `argument@doc-gate-and-lints@arg-14`, `argument@doc-gate-and-lints@arg-15`,
  `argument@doc-gate-and-lints@arg-16`, `argument@doc-gate-and-lints@arg-33`.
- **Shape:** the README's section "Decided design, doc gate denial".
- **Harvest:** per the Harvest section of the README.
- **Closed by:** the measured facts of `argument@doc-gate-and-lints@arg-11`: a plain `cargo doc`
  exits 0 on a broken link, which defeats (b) (`argument@doc-gate-and-lints@arg-14`), and an
  exported `RUSTDOCFLAGS` overrides `build.rustdocflags`, which defeats (c)
  (`argument@doc-gate-and-lints@arg-15`). The agent closed it argued in its reply to round 3. The
  addition of `-A missing_docs` to the gate was declined on
  `argument@doc-gate-and-lints@arg-16`.

### The `doc` gate builds the documentation of public items only, without `--document-private-items` `##doc-gate-item-scope`

- **Proposed by:** the agent, in its reply to round 2, as a fork between public items only, its
  default, and private items too.
- **Final state:** approved.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-17`,
  `argument@doc-gate-and-lints@arg-18`, `argument@doc-gate-and-lints@arg-19`,
  `argument@doc-gate-and-lints@arg-33`.
- **Shape:** the README's section "Decided design, doc gate item scope".
- **Harvest:** per the Harvest section of the README.
- **Ruling:** a fork, in the weak form: the session chose public items only over private items
  too, which no goal ranks; the owner approved it in round 3, by agreeing with every proposal of
  the reply to round 2.

### The three documentation lints are declared once, in the workspace's `[workspace.lints]` tables `##doc-lints-table`

- **Proposed by:** the agent. The reply to round 1 raised the placement; the reply to round 2 made
  it a thread, the workspace table against one table per published crate.
- **Final state:** argued.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-6`,
  `argument@doc-gate-and-lints@arg-20`, `argument@doc-gate-and-lints@arg-21`.
- **Shape:** the README's section "Decided design, doc lints table".
- **Harvest:** per the Harvest section of the README.
- **Closed by:** `argument@doc-gate-and-lints@arg-20`: cargo refuses `lints.workspace = true`
  beside a crate's own lint keys, as the agent measured. The agent closed it argued in its reply
  to round 3.

### Every warning of the three lints is answered by documentation, and this work narrows no item's visibility `##lint-warning-answer`

- **Proposed by:** the agent, in its reply to round 2, with a default of documenting every item
  and narrowing an item to `pub(crate)` by exception, where no crate of the workspace uses it.
- **Final state:** argued, without the narrowing exception.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-7`,
  `argument@doc-gate-and-lints@arg-22`, `argument@doc-gate-and-lints@arg-31`,
  `argument@doc-gate-and-lints@arg-33`.
- **Shape:** the README's section "Decided design, lint warning answer".
- **Harvest:** per the Harvest section of the README.
- **Closed by:** `argument@doc-gate-and-lints@arg-31`, brought by the record search:
  `design@check@the-facade-publishes-what-a-consumer-needs` and
  `design@git@an-operation-in-git-s-terms-is-the-library-s` make an item public because a consumer
  needs it, so the exception's test was the wrong one. The agent dropped the exception and closed
  the thread argued in its reply to round 3.

### This work splits no module, and each model file already past one read gets an issue `##doc-pass-split`

- **Proposed by:** the agent, in its reply to round 3, from the code-split scout's report, as
  "no split here, an issue per file".
- **Final state:** approved, shape (a) of Q1.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-7`,
  `argument@doc-gate-and-lints@arg-23`, `argument@doc-gate-and-lints@arg-24`,
  `argument@doc-gate-and-lints@arg-25`, `argument@doc-gate-and-lints@arg-26`,
  `argument@doc-gate-and-lints@arg-27`, `argument@doc-gate-and-lints@arg-34`.
- **Shape:** the README's section "Decided design, doc pass split".
- **Harvest:** the issues of step 5, per the Harvest section of the README; their kind is D2,
  which the owner approved, round 5.
- **Ruling:** in round 4 the owner chose shape (a) of Q1, the agent's default, over (b), no split
  and no issue, and (c), a split as steps before the doc pass, and gave an argument for it,
  `argument@doc-gate-and-lints@arg-34`. The agent's replies named the kind of each issue `todo`.
  After the search of `thread@doc-gate-and-lints@split-not-needed-is-an-issue`, the agent made it
  `design`, the one kind whose re-entry point the record search reads at the next design or
  planning, which is where the owner's argument of round 4 has the split reconsidered; the change
  keeps the ruling's direction.

### This branch decides only the doc gate's status, and the issue on what becomes an ask stays open `##doc-gate-ask-status`

- **Proposed by:** the agent, in its reply to round 3, from the record search's `rule-before-work`
  verdict on `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`, as Q2 with shape (a)
  as its default against (b), holding that discussion now.
- **Final state:** approved, shape (a) of Q2.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-28`,
  `argument@doc-gate-and-lints@arg-29`, `argument@doc-gate-and-lints@arg-30`,
  `argument@doc-gate-and-lints@arg-35`.
- **Shape:** the README's section "Decided design, doc gate ask status".
- **Harvest:** the issue gains the doc gate among its instances, per the Harvest section of the
  README.
- **Ruling:** in round 4 the owner chose shape (a), and accepted putting off the discussion whose
  re-entry point the work meets, on the ground that the issue's scope is too large for this
  discussion (`argument@doc-gate-and-lints@arg-35`).

### A split signal the discussion decides the work does not need becomes an issue, reconsidered at the next design or planning of that module's code `##split-not-needed-is-an-issue`

- **Proposed by:** the owner, in round 4, who suggested, with a hedge, that
  `design@agent-skills@splits-planned-before-work` might be clarified with the argument given for
  Q1. The agent's shape, a rewording of that head and of one sentence each of
  `skill@knowledge-architect-planning` and `skill@knowledge-architect-design`, followed it in the
  reply to round 4.
- **Final state:** approved, as default D1, round 5.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-34`,
  `argument@doc-gate-and-lints@arg-36`, `argument@doc-gate-and-lints@arg-37`,
  `argument@doc-gate-and-lints@arg-38`, `argument@doc-gate-and-lints@arg-39`,
  `argument@doc-gate-and-lints@arg-40`.
- **Shape:** the README's section "Decided design, split not needed is an issue", and D1 in its
  section "Defaults awaiting the owner".
- **Harvest:** `design@agent-skills@splits-planned-before-work` edited and the head of
  `thread@doc-gate-and-lints@split-not-needed-is-an-issue` written at step 4, with the installed text, per the Harvest section
  of the README. The shape was revised after the reviews of the spec into two heads, on
  `argument@doc-gate-and-lints@arg-42`, `argument@doc-gate-and-lints@arg-43` and
  `argument@doc-gate-and-lints@arg-44`.
- **Ruling:** the agent put it in scope as D1 on `argument@doc-gate-and-lints@arg-39`, and the
  issue kind as D2 on `argument@doc-gate-and-lints@arg-45`. In round 5 the owner approved both
  defaults: for each, the session's choice over the rivals of the README's "Losing alternatives",
  which no goal ranks.

### The library offers the recommended gate list in a head of its own, argued from the goal on generic tools `##recommended-gate-list`

- **Proposed by:** the agent, at planning, on the report of the design-record auditor dispatched
  over the two gates heads the harvest touches.
- **Final state:** argued.
- **Arguments that moved it:** `argument@doc-gate-and-lints@arg-41`.
- **Shape:** the README's section "Decided design, recommended gate list".
- **Harvest:** the new head of `thread@doc-gate-and-lints@recommended-gate-list`, at step 3, per the Harvest section of the
  README.
- **Closed by:** `argument@doc-gate-and-lints@arg-41`: the incumbent head joins two decisions with
  two rivals, and nothing argues the recommended list; `goal@generic-tools-and-procedures-are-shipped`
  defeats its nearest rival, no recommended list.

## Arguments

### The issue's site for the gate, xtask, conflicts with xtask running the recommended list unchanged, which leaves two shapes `##arg-1`

- **Round:** 1, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** the issue names `path@knowledge-architect@tools/xtask/`, "but two things there
  assume the gate list is fixed". `path@xtask@src/gates.rs` "runs `rust_project(...)` unchanged", and its test
  `this_repository_runs_the_recommended_list_over_its_own_checker` "asserts exactly that". Shape A
  changes `design@gates@a-project-holds-its-gate-list`, the setup skill's list and the xtask test,
  with a changelog entry; shape B "breaks the 'unchanged' claim in
  `design@xtask@gates-list-primary-home` and in the xtask test".

### A broken-link check fits the setup skill's existing ask that every adopting project documents its public items `##arg-2`

- **Round:** 1, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** "The setup skill already asks every adopting project to document its public items
  (`design@agent-skills@documentation-presence-is-linted`), so a broken-link check fits the same
  ask."

### A generic tool reaches each target project once the owner takes it in, by the goal on generic tools `##arg-3`

- **Round:** 1, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** "`goal@generic-tools-and-procedures-are-shipped` says a generic tool is shipped to
  every project." The goal ships it once knowledge-architect's owner takes it in, and to each target
  project that carries out its activity; the agent's words overstated its reach.

### Shape A costs each adopting project one more `cargo doc` in its gate run `##arg-4`

- **Round:** 1, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** "The cost of A is that each adopting project's gate run gets one `cargo doc`
  longer."

### The tree builds its documentation without a warning today, so the gate needs no repair first `##arg-5`

- **Round:** 1, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`, `thread@doc-gate-and-lints@doc-gate-denial`.
- **Measurement:** `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, on this tree.
- **Key words:** "The current state is clean:
  `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` exits 0 today, so the gate goes in
  with nothing to repair first."

### In the workspace table the lints also reach xtask, which is not published; per-crate tables would stop the crates sharing one lint table `##arg-6`

- **Round:** 1, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-lints-table`.
- **Key words:** "in the workspace table, the lints also reach `xtask`, which is not published. The
  setup text asks for them only on published crates. Per-crate tables would keep xtask out but stop
  the crates sharing one lint table."

### The three lints give 279 distinct warnings today, mostly fields and variants in `model`, and the text must be right as well as present `##arg-7`

- **Round:** 1, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@lint-warning-answer`, `thread@doc-gate-and-lints@doc-pass-split`.
- **Measurement:** the three lints run over `--all-targets` of each published crate on this
  machine, duplicates removed by source location.
- **Key words:** "279 distinct warnings. The issue's own figures predate the split of `check` into
  `model`, `query` and the other crates, and are stale." "The work is mostly fields and variants in
  `model`, plus about 70 `# Errors` sections. The text must be right as well as present, because
  the code-claims review axis reads it."

### Shape A keeps xtask calling `rust_project` unchanged and CI unchanged `##arg-8`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** "`xtask` keeps calling `rust_project` unchanged, so
  `design@xtask@gates-list-primary-home` stays true as written." "CI runs the same tool, so `ci.yml`
  does not change."

### The `doc` gate goes after `clippy`: both are lint passes, and clippy's extract is the fuller report on a missing doc `##arg-9`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Measurement:** one cold run each of the doc build and of clippy on this tree.
- **Key words:** "Cold, the doc build takes 3.72 s and clippy takes 4.40 s. One run each cannot
  separate figures that close." "I put `doc` after `clippy`. Both are lint passes, and clippy's
  extract is the fuller report on a missing doc".

### The owner's word for A is the taking in that the goal on generic tools requires `##arg-10`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** "Rival B, the gate in xtask only: you ruled for A. That ruling is also the 'taken
  in' the goal requires." The criterion it answers: "A tool reaches every target project only once
  you take it in (`goal@generic-tools-and-procedures-are-shipped`)".

### Only the gate's own variable denies a broken link in every configuration measured `##arg-11`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-denial`.
- **Measurement:** on a scratch crate with one broken intra-doc link: `cargo doc --no-deps` gives
  a warning and exits 0; `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` exits 101;
  `[lints.rustdoc] broken_intra_doc_links = "deny"` with a plain `cargo doc` exits 101;
  `build.rustdocflags = ["-D","warnings"]` in the scratch crate's cargo configuration file exits 101; the same
  configuration with `RUSTDOCFLAGS="--cfg foo"` exported gives 0 errors and exits 0.

### The gate's variable catches every rustdoc warning with nothing the project must declare, as the clippy gate's `-D warnings` does `##arg-12`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-denial`.
- **Key words:** "It catches every rustdoc warning, with nothing the project must declare. This is
  the pattern of the clippy gate's `-- -D warnings`."

### The gate's variable replaces the caller's rustdoc flags, and switching to it rebuilds the docs `##arg-13`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-denial`.
- **Measurement:** the rebuild of every workspace crate's docs on switching between a plain
  `cargo doc` and the gate, on this tree.
- **Key words:** "in the gate, the variable replaces any `RUSTDOCFLAGS` the caller exported and any
  `build.rustdocflags` the project configured. A project whose docs need a flag adds it to the
  gate's `envs` in its own list." "switching between a plain `cargo doc` and the gate rebuilds the
  docs of every workspace crate. That took 0.77 s here."

### A plain `cargo doc` with a `[lints.rustdoc]` ask passes silently in a project that has not adopted the ask `##arg-14`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-denial`.
- **Key words:** "Defeated: a plain `cargo doc` exits 0 on a broken link. A project that has not yet
  adopted the ask would get a gate that passes silently, which fails the first criterion." The
  first criterion: "A link that names nothing, in text docs.rs renders, fails a gate
  (`goal@adoption-is-easy`)".

### `--config build.rustdocflags` loses its deny to an exported `RUSTDOCFLAGS` `##arg-15`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-denial`.
- **Key words:** "Defeated by the last row of the table: an exported `RUSTDOCFLAGS` silently drops
  the deny."

### One missing doc fails both `clippy` and `doc`, and `-A missing_docs` would add gate logic only to avoid a duplicate report `##arg-16`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-denial`.
- **Key words:** "rustdoc also emits `missing_docs`, as I measured. One missing doc therefore fails
  two gates, `clippy` and `doc`. Adding `-A missing_docs` to the gate would avoid that, but it adds
  gate logic only to avoid a duplicate report."

### The issue's reason is the text docs.rs renders, which shows public items alone `##arg-17`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-item-scope`.
- **Key words:** "the issue's reason is the text docs.rs renders, which shows public items alone."
  "My default is public only, because it is the scope the criterion names."

### Private doc comments mislead a developer the same way, since rust-analyzer resolves their links, at a cost of 2 warnings here `##arg-18`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-item-scope`.
- **Measurement:** the documentation build with `--document-private-items` added, on this tree;
  the transcript gives no command line for it.
- **Key words:** "rust-analyzer resolves links in private doc comments on hover, so a broken one
  misleads a developer the same way." "it adds 2 warnings, both in `mutate` ('is both a function and
  a module: ambiguous link')." "For an adopting project, the cost at its pin move is unknown."

### No goal ranks the two item scopes, so the choice is a fork for the owner `##arg-19`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-item-scope`.
- **Key words:** "No goal ranks these two, so it is your fork."

### Cargo refuses `lints.workspace = true` beside a crate's own lint keys, so one table per crate would repeat every lint `##arg-20`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-lints-table`.
- **Measurement:** a crate manifest with `[lints] workspace = true` and a key of its own, run
  through cargo.
- **Key words:** "Defeated: cargo refuses `lints.workspace = true` combined with a crate's own lint
  keys, as I measured. Each crate would then repeat `unsafe_code` too, in 11 copies."

### xtask, never published, takes the lints too, with no warning under them today `##arg-21`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-lints-table`.
- **Measurement:** the three lints over `xtask`, on this tree.
- **Key words:** "`xtask`, which is never published, gets the lints too. It has 0 warnings under
  them today, as I measured."

### An item no other crate of the workspace uses could be narrowed to `pub(crate)` instead of documented `##arg-22`

- **Round:** 2, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@lint-warning-answer`.
- **Key words:** "an item that no other crate of the workspace uses could become `pub(crate)`
  instead of being documented. That shrinks the public API. Each such narrowing is a changelog
  entry, which 0.x allows. I have not measured how many items qualify." "The issue's closing
  condition ('answered by the documentation it asks for') assumed documentation everywhere, and
  narrowing is the alternative it did not weigh."

### The code-split scout finds no split needed, and three `model` files already past one read `##arg-23`

- **Round:** 3, the agent's reply, from the code-split scout's report.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** "no split is needed before the work. Every part of the work belongs to an existing
  Component." "`path@model@src/resolver.rs` (2511 lines), `entity.rs` (3904) and `manifest.rs`
  (3486) are already longer than one read (2000 lines). No issue records that yet." The correction
  to the draft: "the 114 warnings in `resolver.rs` are not struct fields. They are 75 fields and 39
  variants of `enum Cause`, plus 8 `# Errors` sections."

### The documentation added to `resolver.rs` falls inside one enum, which moving code does not divide, and no count of parts that change apart exists `##arg-24`

- **Round:** 3, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** "The roughly 114 added lines fall inside the single enum `Cause`, which moving code
  does not divide." "No count of changes that land apart in these files has been taken, so no cut
  can be argued." Against a split, the scout cites `design@model@resolver-outcome` and
  `design@model@cause-words-in-the-resolver`: a new refusal adds a variant, its arm and its words at
  once.

### The head on splits forbids parking a split as an issue, and the files lie inside the work's area `##arg-25`

- **Round:** 3, the agent's reply, from the record search.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** the search found that the default conflicts with
  `design@agent-skills@splits-planned-before-work`: "Neither is left to the implementer, nor parked
  as an issue." Of shape (a): "these files are inside the work's area, so treating their size as
  met outside the task is a stretch. The issues may sit untouched, as you reported." "The head also
  cites your own report as its premise: an issue on a split sits untouched for a long time."

### No issue loses the signal with the spec, and a split before the doc pass needs two or three cut designs with no count to argue them `##arg-26`

- **Round:** 3, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** of (b): "The signal leaves with the spec and nothing schedules the split." Of (c):
  "Moving the tests out is enough for `resolver.rs` alone. `entity.rs` (2322 before its tests) and
  `manifest.rs` (2020) need a real cut." "Two or three cut designs, with their own grounding, added
  to this branch. No count of the parts that change apart exists today." "My default is (a): the
  work does not need the split, and an issue keeps the signal where (b) loses it." Unverified: that
  moving the tests out brings `resolver.rs` within one read; its 1582 lines before the tests hold
  67985 bytes, near the Read's cap in tokens, as the code-claims review of the spec measured.

### The issue on `worklist.rs` keeps its closing condition, which this work does not meet `##arg-27`

- **Round:** 3, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** "`worklist.rs` (3118 lines) gets one `# Errors` section. Its existing issue's
  trigger is 'the work that next changes the writer', which this work does not meet, so it stays as
  it is." The issue is `issue@query@worklist-rs-holds-a-writer-and-a-judge-past-one-read`, a `todo`,
  which has no trigger: the quoted words are from its "What would close it", which reads in full "the
  work that next changes the writer of `worklist.rs` or a format both halves read".

### A doc gate in the recommended list meets the re-entry point of the issue on asks `##arg-28`

- **Round:** 3, the agent's reply, from the record search's `rule-before-work` verdict.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-ask-status`.
- **Key words:** the re-entry point of `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`
  is "the design discussion of what becomes an ask of every target project, 'at the latest when a
  branch next takes in a tool that judges a project's quality'. A doc gate added to `rust_project`
  is such a tool. The issue holds about ten statements".

### The doc gate takes the status every gate of the recommended list holds, and one gate does not change the general question `##arg-29`

- **Round:** 3, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-ask-status`.
- **Key words:** "The gate takes the status every other gate of the recommended list holds: a
  project edits its own list. The branch adds no kind of ask that is not already there." "the
  issue's question is general, and adding one gate does not change its shape." "Putting off a
  re-entry point that has been met is yours to accept, so the default is built and the branch waits
  for your word before it merges."

### The two gates heads that list the gates by name gain the `doc` gate at the harvest, named "the documentation build" `##arg-30`

- **Round:** 3, the agent's reply, from the record search's `check-reversal` verdicts.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`, `thread@doc-gate-and-lints@doc-gate-ask-status`.
- **Key words:** "`design@gates@a-project-holds-its-gate-list` and
  `design@gates@distill-over-full-logs`. Both list the gates by name, and the doc gate adds one to
  each list. Your ruling on #doc-gate-home covers that change. The harvest rewrites both heads in
  place". "The new gate is named `doc`, and the head calls it 'the documentation build', to keep it
  apart from the head's 'the document check', which is `check`."

### An item is public because a consumer needs it, so "no crate of the workspace uses it" is the wrong test for narrowing it `##arg-31`

- **Round:** 3, the agent's reply, from the record search's `check-reversal` verdicts.
- **Threads:** `thread@doc-gate-and-lints@lint-warning-answer`.
- **Key words:** "`design@git@an-operation-in-git-s-terms-is-the-library-s` and
  `design@check@the-facade-publishes-what-a-consumer-needs`. These defeat the narrowing exception".
  "An item is public because a consumer needs it, extensions outside the workspace included. 'No
  crate of the workspace uses it' is therefore the wrong test, and
  `design@knowledge-architect@capability-cost-before-migration` ranks first what such a consumer
  would lose. I dropped the exception". "Whether an item should stay public is each crate's facade
  decision, not something a lint pass decides."

### The binary tests that list the gates by name owe `doc`, and the mutation that breaks a link is a claim of the gate's step `##arg-32`

- **Round:** 3, the agent's reply, from the searchers' reports, verified by the agent.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** "`path@xtask@tests/gates_bin.rs` lists the gates by name in
  `locked_reaches_every_gate_that_resolves_dependencies` and `ci_reaches_every_gate`, and it now
  owes `doc`." "The `cargo x mutate` run that breaks one link is a claim of the step that builds the
  gate, not an acceptance criterion, per `design@agent-skills@completeness-check-is-a-step-claim`."

### No premortem cause becomes a tripwire or an acceptance criterion: each failure is loud, or is a cost its decision accepted `##arg-33`

- **Round:** 3, the agent's reply, its premortem.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-home`, `thread@doc-gate-and-lints@doc-gate-denial`,
  `thread@doc-gate-and-lints@doc-gate-item-scope`, `thread@doc-gate-and-lints@lint-warning-answer`.
- **Key words:** a project whose docs need a flag "such as `--cfg docsrs`, which the gate's variable
  replaces": "The gate then fails, which is loud, so no tripwire. It becomes a Migration line in the
  changelog." A toolchain update that makes a new rustdoc lint warn: "Loud, and
  `design@knowledge-architect@toolchain-is-pinned` makes the update an explicit edit here.
  Dropped." Broken links in private docs: "This is what that decision accepts, so it fails
  condition 3. Dropped." Empty boilerplate docs: "It concerns the quality of this work, not the
  decision. The code-claims review judges it." The double report: "Loud, and accepted in round 1.
  Dropped." "No cause becomes a tripwire and no acceptance criterion is needed". The quoted "round 1"
  is the agent's title for its reply to the owner's round 2. The argument predates
  `thread@doc-gate-and-lints@split-not-needed-is-an-issue`, whose silent cause the README's premortem
  adds.

### What matters in the head on splits is when a split is considered, at planning and not at implementation or review, so an issue on a split the work does not need is acceptable `##arg-34`

- **Round:** 4, the owner.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`,
  `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words, attributed:** the owner judged it acceptable to leave the split to an issue, in
  particular when the work does not touch the code itself. The owner stated that the important
  aspect of `design@agent-skills@splits-planned-before-work` is that a code split is considered at
  the planning steps and not at the implementation steps, and that such split issues are
  reconsidered at the planning steps too. The owner reported that before that head, split issues
  were often found and considered at review time by the standing-state reviewer, which would have
  added much unplanned work at a bad time.

### The issue on asks has too large a scope for this discussion `##arg-35`

- **Round:** 4, the owner.
- **Threads:** `thread@doc-gate-and-lints@doc-gate-ask-status`.
- **Key words, attributed:** the owner chose shape (a) of Q2, on the ground that the scope of
  `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` is too large for this discussion.

### The installed text says two different things about a split the work does not need `##arg-36`

- **Round:** 4, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words:** "`skill@knowledge-architect-planning` says a split the work needs is 'never left to
  the implementer, and never an issue'"; "`skill@knowledge-architect-design` lets the discussion
  decide 'that none is made', and says nothing of what becomes of the signal then." The rewording
  proposed: "A split is considered where a work is designed or planned, never where it is
  implemented or reviewed. A split the work needs is a step before the work. A split signal the
  discussion decides the work does not need, such as a work that changes only a module's
  documentation, becomes an issue. Its re-entry point is the design discussion or the planning of
  the next work that changes that module's code."

### Reconsidering a split issue at planning needs no new mechanism, since the record search grades a met re-entry point `##arg-37`

- **Round:** 4, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words:** "no new mechanism is needed. The record search at a design grounding already grades
  issue entries. A re-entry point that is met is graded `trigger-met`, with the action
  `rule-before-work`, as happened to the asks issue this session."

### The rewording changes the head, one sentence of each of two skills, and the changelog `##arg-38`

- **Round:** 4, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words:** "the head's title and body; one sentence of the planning skill and one of the
  design skill; a `CHANGELOG.md` entry, because every adopting project receives the installed
  text."

### The rewording is in this branch's scope, because the ruling on Q1 acts against the head's current wording `##arg-39`

- **Round:** 4, the agent's reply.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`,
  `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** "Your Q1 ruling acts against the head's current wording. Without the rewording,
  this branch would leave the head and the work in divergence." "My default is to build it in this
  branch, listed in the spec as D1, a default awaiting your ruling. You said 'maybe', so the branch
  merges only once you rule on it."

### The record search on the new thread returned two `check-reversal` verdicts on the rewording `##arg-40`

- **Round:** 4, the agent's reply, from the second record search.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words:** "Two `check-reversal` verdicts came back, both on the wording of the rewrite." The
  transcript records no more of them; their outcome is the README's.

### The gates head joins two decisions, and the recommended list stands on the goal on generic tools `##arg-41`

- **Round:** 4, at planning, from the design-record auditor's report on the heads the harvest touches.
- **Threads:** `thread@doc-gate-and-lints@recommended-gate-list`, `thread@doc-gate-and-lints@doc-gate-home`.
- **Key words:** "a project holds its gate list" loses to a list the library fixes, and "the library
  offers a recommended one" loses to no recommended list; five citing texts use the second alone.
  The per-gate distillers of `design@gates@distill-over-full-logs` answer the recommended list's
  question, and that list omits the rebase check's `Distiller::Ancestry`. The fact against that
  head's nearest rival: `grep -c ' \.\.\. ok$' target/gates/test.log` counts 1254 of 2226 lines.

### A split the work needs and a split it does not need are two decisions, so the issue route takes a head of its own `##arg-42`

- **Round:** 4, the decision-record review of the spec.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words:** the first rule loses to the split left to whoever meets it while writing the code,
  the second to "no split and no issue" and to a `todo` or `deferred` entry; two statements that lose
  to different nearest rivals are two decisions. The incumbent keeps its title.

### A split signal met at implementation or review keeps the routing of the table of things met outside the task `##arg-43`

- **Round:** 4, the design-conformance review of the spec.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words:** sending every such signal to an issue bypasses rows 1 and 4 of
  `primer@met-outside-the-task`, against `design@agent-skills@met-outside-the-task-outcomes` and the
  incumbent's second paragraph; and a signal over code the range writes is a departure the review
  weighs, per `design@agent-skills@code-shape-axis`. So the new head covers code the work does not
  write, a doc comment not counting as writing it, and gives the issue's shape where the routing
  opens one.

### A signal outweighed by the signals against a split makes no split stand and owes no issue `##arg-44`

- **Round:** 4, the transcript review of the spec, on the code-split scout's signal on
  `path@gates@src/gates.rs`.
- **Threads:** `thread@doc-gate-and-lints@split-not-needed-is-an-issue`, `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** the scout found that two parts change apart "holds weakly", against three signals
  against a split it named; an issue is owed only where a split stands and the work does not need it.

### A split no discussion has weighed for a work that changes the code is an open design question, so its issue is of kind `design` `##arg-45`

- **Round:** 4, the session, on the second search's verdicts and the design-conformance review of the
  spec.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`, `thread@doc-gate-and-lints@split-not-needed-is-an-issue`.
- **Key words:** a `todo` has no re-entry point for the search to grade `trigger-met`, and a
  `deferred` trigger is read at every merge, which brings the split back to review time; against
  `design@agent-skills@undesigned-work-is-an-issue`, the split is not known undesigned work, since
  whether and where to split is open, as the project-audit skill already records a split too large
  for its branch as a `design` issue. The owner's ruling of round 4 was on a `todo`; the kind is D2.

### The size signal is a Read that does not return the file whole, never a count of lines `##arg-46`

- **Round:** 4, the self-consistency and code-claims reviews of the spec.
- **Threads:** `thread@doc-gate-and-lints@doc-pass-split`.
- **Key words:** `design@agent-skills@module-split-guidelines` says "no number of lines is a signal";
  `path@git@src/git.rs`, 1949 lines, is past one read already: a Read with no range returned lines
  1 to 1293 and named a cap of 25000 tokens.
