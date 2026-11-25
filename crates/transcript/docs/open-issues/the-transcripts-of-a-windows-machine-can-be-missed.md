---
kind: defect
---
# On Windows the transcripts' directory is looked for through `HOME` alone, and a session of a removed directory is left out

## Summary

`transcript` finds the harness's directory through `CLAUDE_CONFIG_DIR` or `HOME`. PowerShell sets neither
by itself, so the command refuses there. And a session whose directory no longer exists is compared as
written with a root that Windows spells with a verbatim prefix, so it is left out. The first was read in the
code, and the second observed on a Windows runner.

## Details

### What

- `transcript_dir` of `path@transcript@src/reader.rs` reads `CLAUDE_CONFIG_DIR`, then `HOME`, and
  otherwise answers "neither CLAUDE_CONFIG_DIR nor HOME is set". Windows gives a process `USERPROFILE`;
  Git Bash sets `HOME` besides, and PowerShell does not set it as an environment variable (assumption,
  not run).
- `project_files` of the same file makes each root canonical, which on Windows gives `\\?\C:\…`, and the
  `cwd` of a session only where it exists. The `cwd` of a removed worktree stays `C:\…`, which starts with
  no such root, so the session is left out. Observed on a GitHub Windows runner: a case of
  `path@testing@tests/transcripts/`, places, whose session has the `cwd`
  `<root>/.claude/worktrees/w1`, a directory that does not exist, the root spelled with no verbatim prefix as
  a harness writes it: `transcript list` printed "# files read: 1", without that session, where Linux reads
  two. On Linux and macOS the two spellings are one, where the root holds no symbolic link.

Ruled out: a session whose directory exists is found on Windows, its `cwd` made canonical like the root, as
the tests of the binary over the transcripts show there. `transcript_case` of `path@testing@tests/binary.rs`
makes the worktree's directory on Windows, for this entry.

### Why it matters

A retrospective or a review of transcripts on Windows reads nothing, or misses the sessions of removed
worktrees, the ordinary place of a dispatched agent, with no line saying so, against
`design@transcript@transcript-directory-resolution`.
The owner left development on Windows for later, while a project developed there is one that
`design@knowledge-architect@platforms-served` serves.

### What would close it

`USERPROFILE` read where `HOME` is not set, and a comparison that holds one spelling on both sides, with a
test over a `cwd` that no longer exists, run on Windows, where the test helper makes no directory.
