---
kind: todo
---
# No option of `klarch transcript show` removes the machine's home paths and the branch's commit SHAs from what it prints

## Summary

A plan document commits the transcript of its discussion, per
`design@agent-skills@committed-transcript`, extracted with `klarch transcript show`, after a step that reads it and removes
private information by hand, per `design@agent-skills@transcript-removal-step`. The owner proposed an option of the command that anonymizes part of
the output, such as the paths under the machine's home directory, and left it for later.

## Details

### What

`show`, per `design@transcript@transcript-query-verbs`, prints a session's kept lines as the harness
recorded them. Those lines name paths under the home directory of the machine the session ran on,
in tool results, in agents' reports and in the session's own replies: of the 4,319 lines that
`klarch transcript show --full` printed for the session of the discussion that designed that
milestone, 148 hold the string `/home/`, by `grep -c` over the extraction. They also quote the SHAs of
commits the conversation names, among them commits of the branch under way, which
`design@knowledge-architect@no-branch-sha-is-cited` refuses as citations in a document; per that
head, a SHA a transcript quotes is a quotation, not a citation.

The work: an option of `show`, its name and its exact replacements decided by a discussion, that
prints the output with each path under the home directory written relative to it, and each SHA of a
commit of the branch replaced by a placeholder or by the commit's subject. The same pass could carry
the check the milestone's premortem routed here instead of a tripwire, as its cause P4: a scripted
search of a transcript for home-directory paths and e-mail addresses before its commit. Not
established: whether a SHA of a branch's own commit can be told apart from any other hexadecimal
string of the output without the repository at hand.

A plan document's assembly in the first slice of the milestone on split guidelines and plan
directories carried the whole extraction by hand: the cut line found in `transcript show` output,
a regex replacing 100 home paths, and a grep for e-mail addresses and tokens. The same discussion
weighs a cut at a named message of the owner, where the planning skill's extraction ends,
per C1 of the retrospective file `2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow`.

### Why it matters

A transcript pushed to a public repository with private data in it is readable there until a later
commit removes it, and from the history after that. The removal step is the only guard before the
push: the commit that holds a transcript is pushed with its branch, with no word of the owner for
it, per `design@agent-skills@transcript-pushed-with-the-branch`. That step is a session reading
the file whole and removing by hand what a script could find, and nothing checks that it was done;
the milestone's premortem named the failure, its cause P4. An option that removes the home paths,
and a scripted search for e-mail addresses, would take that part of the step out of a session's
reading; the option's replacement of the branch's SHAs is work it adds, which the step does not
do.

### What would close it

The option built in `transcript`, with a test over a transcript holding a home path and a branch
SHA, and the planning skill's removal step pointing to it for what it removes.
