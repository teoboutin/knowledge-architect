---
kind: todo
---
# No option of `klarch transcript show` removes the machine's home paths and the branch's commit SHAs from what it prints

## Summary

`milestone@plans@split-guidelines-and-plan-directories` has a plan document commit the transcript of
its discussion, extracted with `klarch transcript show`, after a step that reads it and removes
private information by hand. The owner proposed an option of the command that anonymizes part of
the output, such as the paths under the machine's home directory, and left it for later.

## Details

### What

`show`, per `design@transcript@transcript-query-verbs`, prints a session's kept lines as the harness
recorded them. Those lines name paths under the home directory of the machine the session ran on,
in tool results, in agents' reports and in the session's own replies: of the 4,319 lines that
`klarch transcript show --full` printed for the session of the discussion that designed that
milestone, 148 hold the string `/home/`, by `grep -c` over the extraction. They also quote the SHAs of
commits the conversation names, among them commits of the branch under way, which
`design@knowledge-architect@no-branch-sha-is-cited` refuses as citations in a document; the
milestone rewrites that head so a SHA a transcript quotes is no citation.

The work: an option of `show`, its name and its exact replacements decided by a discussion, that
prints the output with each path under the home directory written relative to it, and each SHA of a
commit of the branch replaced by a placeholder or by the commit's subject. The same pass could carry
the check the milestone's premortem routed here instead of a tripwire, as its cause P4: a scripted
search of a transcript for home-directory paths and e-mail addresses before its commit. Not
established: whether a SHA of a branch's own commit can be told apart from any other hexadecimal
string of the output without the repository at hand.

### Why it matters

A transcript committed to a public repository with private data in it cannot be taken back. The
manual removal step is the only guard until the option exists, and a step a session follows by
reading is the one the milestone's premortem names as the cause of that failure.

### What would close it

The option built in `transcript`, with a test over a transcript holding a home path and a branch
SHA, and the planning skill's removal step pointing to it for what it removes.
