---
kind: design
---
# The goal standard does not say what a goal is worth or what it is for, and its rule on what shows a goal met pulls the other way

## Summary

The owner holds that a goal covers what shapes the project's future and what argument alone might not defend, and that a
goal is probably worth most where no mechanical test can show it met. `skill@knowledge-architect-goal-setting@goal-or-not`
states no such purpose, and its rule 6 asks for a check or a test wherever an outcome depends on nothing the project does
not control. The owner also holds that goals should focus on the outcomes and functions of what the project
delivers, not on its shape, and judges that the current standard produces goals somewhat too
focused on shape. This entry holds the owner's words for a design discussion on the goal standard. They are quoted, on the
owner's request, where an entry otherwise attributes.

## Details

### What

The owner's words, given at the goal of `cli`, on a proposal to extend a test so that it reached every command for each kind of
refused invocation:

> tbh, the goal itself is probably enough. I don't think it is worth it to try everything to have a mechanical test that detects any breach of that goal. Keep the effort on that reasonable. Goals are supposed to cover aspects that shape the future of the project, and that argumentation alone would not necessarily be able to defend (for example the cost/effort being one day considered to high might end up outweighing the justification or value).
> Most of the time, a goal being met won't be testable by a mechanical test, and that is probably when goals have the most value.
> Here, I think the goals stand because even with a mechanical test, someone might judge the effort to have a clean cli api too high and just disable the test.

And, at the same goals, on whether an outcome should be a goal or a design head:

> Goals should not be added just for the sake of adding goals.

And, at the intake of forty retrospective files, ruling to keep the intent row of the table of what
the owner brings, by which an outcome a recorded decision already serves against every rival it is
cited to defeat is drafted as no goal:

> Since goals bind very strongly, I want to avoid having too much of them, especially when they are things that can rest on argument. Because that means a reversal would be a better shape. If a goal state it, it would never be questioned, thus never improve. I think goals should be overall focused more on outcomes of the project and not on its shape. And I think the current standard is creating goals that are a bit too focused on shapes than on outcomes and functions of the product the project delivers.

The instance: the draft measure of `goal@cli@a-refused-invocation-does-no-work` asked for a test that finds every command, for one
of its four kinds of refused invocation. The goal reviewer graded the partial coverage `blocks`, applying the standard as it
reads, so this is no lapse of the reviewer. The session proposed extending the walk to two more kinds, and the owner refused.
A second candidate goal, that a command keeps one spelling in every binary, went to a design head: the owner left its status open, and the session decided by the standard, since the outcome is a means to another goal.

The suspected mechanism, in one sentence: the standard says how a goal is shaped and shown met, and nothing of why one is
worth writing, so a drafter and a reviewer who follow it push each goal toward a test of every breach.

Not established: how many of the project's goals name a mechanical measure only because rule 6 asks for one, and whether the
owner's view reaches every rule 6 case or only those where a full test costs more than it shows.

### Why it matters

It strains `design@agent-skills@goal-met-in-tree-or-in-use`, which sorts what shows a goal met. A session spends effort the owner
does not want on tests of every breach, a goal can narrow to what a test can see, and a goal is written where a design head would
serve. The standard's rules 1 and 3 guard the narrowing for other reasons, and no rule guards the adding.

### What would close it

A design discussion on the goal standard under `skill@knowledge-architect-design`, which is this entry's re-entry point, with the
owner's words above as its first statement. It weighs at least: what the standard states a goal is for and when one is worth
writing; whether a goal states an outcome or a function of what the project delivers rather than a
shape, a shape that can rest on argument being left to a design head, which a reversal can improve; rule 6 and `design@agent-skills@goal-met-in-tree-or-in-use`; and what `agent@knowledge-architect-goal-reviewer` grades as
blocking on a measure. Its outcome is the installed text changed or kept, at
`path@agent-skills@content/skills/goal-setting/SKILL.md`, edited as `path@agent-skills@CLAUDE.md` says, with the review of the
whole goal set judged for what the change owes. The verbatim words leave with this entry.
