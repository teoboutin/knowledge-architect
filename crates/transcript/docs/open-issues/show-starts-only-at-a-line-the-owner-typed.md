---
kind: todo
---
# `transcript show --from` starts only at a line the owner typed

## Summary

The transcript reviewer reads a session from the message where the branch's work begins, found by
`show --from "<opening words>"`. `--from` matches only a line the owner typed. In a session that
carries several branches, a branch's work can begin at an agent's line after a merge, and the
reviewer then reads from an earlier owner's line, every line between them a cost in its context.

## Details

### What

`transcript show --help`: "--from <TEXT> Start at the first line the owner typed whose text contains
TEXT". The installed text assumes such a line: the review skill,
`path@agent-skills@content/skills/review/SKILL.md`, finds with `--containing "<opening words>"` the
session holding the message where the work begins "as one the owner typed", and
`path@agent-skills@content/agents/transcript-reviewer.md` runs
`transcript show <session> --from "<opening words>" --full`.

The instance: the session of 2026-11-02 that implemented the four slices of the
development-and-components milestone in a row, each on its own branch. The brief of a transcript
reviewer named the message where a branch's work began; that message was the agent's, after a
merge. The reviewer extracted the session from its opening, 12,209 lines, and searched for the
place.

`design@transcript@transcript-query-verbs` gives `show` "`--from` a text the owner typed";
`design@agent-skills@transcript-found-by-listing` names a session "by the opening words of the
message where its work begins". Neither argues against a start at another line, and a start at a
line number `show` prints, or at an agent's line, is within the argument of the verbs head: the
verbs compute what the review needs.

Ruled out as the cause of a lost item: the reviewer found its place by search. The cost is in
reading, one instance.

### Why it matters

The transcript reviewer reads the whole of what it extracts, so every line before the work is a
cost in its context, where `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` has a
tool do a task of the workflow at a lower cost to the agent than the agent's own means.

### What would close it

`show` starting at a line number it prints, or at an agent's line, with a test over a fixture, and
the review skill's sentence on the brief naming it; or a ruling that the last owner's message before
the work suffices, recorded in the commit that deletes this entry. A second instance in a
retrospective file is the occasion to raise it again.
