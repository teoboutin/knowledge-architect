---
kind: defect
---
# `install-agent-skills` can exit 2 after it has written files, and lists none of them

## Summary

A symbolic link at an owned path the version does not ship, or a file it cannot rewrite, stops the install
after it has rewritten other files. It exits 2 and prints no `wrote` line, though exit 2 promises an untouched
tree elsewhere, and the doc comment of `install` says a link on an owned path is refused before anything is touched.

## Details

### What

Reproduced by an adversarial reviewer over a project `init` made, committed:

- An edited installed skill, and `ln -s /etc/hostname .claude/agents/knowledge-architect-link.md`: exit 2 naming
  the link, and the edited skill rewritten. `install` checks the links of shipped paths before writing, and
  finds a link at an unshipped path only through `owned_on_disk`, after the writes.
- A stale primer and agent, and `chmod 000` on a stale skill: exit 2, the others rewritten. `install(...)?` in
  `install_agent_skills` of `path@check@src/commands/mod.rs` drops the list of what was written.

### Why it matters

A session reading exit 2 takes the tree as it left it, against `design@cli@exit-code-ladder`, and the rewritten
files reach its next commit unannounced, against `goal@knowledge-architect@agents-work-without-drift`.

### What would close it

A link at any owned path refused before the first write, and the command listing what it wrote before a failure
and exiting 1 after any write, as `check --fix` does, each with a binary test.
