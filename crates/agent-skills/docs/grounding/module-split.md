# Module splits: one project's threshold of lines, and its cut by scope of review

Findings read in thaum, a project of the owner's that installs the workflow, which
`design@agent-skills@module-split-guidelines` and
`rejected@agent-skills@module-split-at-a-size-threshold` lean on. Each was read at thaum's commit
66df2201, and is read again at the source it names.

## The threshold has no recorded basis

- **thaum splits a module at a threshold of lines.** Its development skill,
  `path@elsewhere@thaum/.claude/skills/thaum-development/SKILL.md`, section "The review axes for
  code": "A module is split before review is dispatched when the unit of work would leave it above
  800 lines of non-test code, or gives it a second frame kind's or rules section's advance logic."
- **The number came with no measurement and no argument.** thaum's commit 3997f0d1 added the rule,
  then to `path@elsewhere@thaum/.claude/skills/developing/SKILL.md`. Its message restates the rule.
  The lines it added give a reason for the cut axis, that it "lets a reviewer check a module against
  the printed section that scopes it", and none for the number 800. Read with
  `git show 3997f0d1` in thaum's repository.
- **The cut axis is a decision of its own there.** thaum's root design home,
  `path@elsewhere@thaum/docs/design.md`, holds the head "A module holds the code one scope of review
  bounds", whose scopes are a rules section, a frame kind, a concern of the invariant checks, a
  vocabulary type and the mechanism one design head states. The criterion of the module guidelines
  is that head's axis stated with no term of thaum's domain.

## A size that no move fixes

- **One item holds a module above the threshold.** thaum's issue
  `path@elsewhere@thaum/crates/thaum-engine/docs/open-issues/event-vocabulary-items-grow-per-variant.md`
  gives `Event`, one enum, as 999 lines at the commit that cut the modules around it, and its file,
  `path@elsewhere@thaum/crates/thaum-engine/src/runtime/step/event.rs`, as the one module the split
  left above the threshold, under the skill's exception for a module one item holds above it.
- **The item grew past that figure.** At thaum's commit 66df2201, `Event` runs 1018 lines from
  `pub enum Event` to its closing brace, counted by brace depth with
  `awk '/^pub enum Event[ {<]/{s=NR;d=0;on=1} on{n=gsub(/{/,"{");m=gsub(/}/,"}");d+=n-m;if(d==0&&NR>s&&(n+m)>0){print NR-s+1;exit}}' crates/thaum-engine/src/runtime/step/event.rs`
  in thaum's repository. A threshold of lines keeps reporting such a module, and only a change of the
  item's design, not a move, would answer it.
