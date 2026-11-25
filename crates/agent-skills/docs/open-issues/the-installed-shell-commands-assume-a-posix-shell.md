---
kind: observation
---
# The installed text's shell commands assume a POSIX shell, where Claude Code on Windows can run PowerShell

## Summary

Commands that the installed skills and agents show use heredocs, `SHELLOPTS` and an unquoted `=<version>`.
Claude Code's documentation says that PowerShell is the primary shell on Windows where its tool is on, and
zsh is macOS's default shell. Neither was run, so whether a command of the installed text fails there is not
established.

## Details

### What

Read in the installed text, at this entry's opening:

- heredocs, `cat > <path> <<'EOF'`, in `skill@knowledge-architect-review`,
  `agent@knowledge-architect-goal-reviewer` and `agent@knowledge-architect-code-split-scout`: PowerShell has
  no heredoc of that form;
- `skill@knowledge-architect-setup@harness-shell` writes `"env": {"SHELLOPTS": "pipefail"}` into the harness
  settings, which bash alone reads;
- `skill@knowledge-architect-setup@setup-gates` shows `cargo install … --version =<version>`: in zsh a word
  that opens with `=` is expanded as a command's path, so a version written there fails unless quoted.

Read in Claude Code's documentation, the page "Tools reference", its section on the PowerShell tool, on
2026-11-25: "When the tool is enabled, Claude treats PowerShell as the primary shell. The Bash tool remains
available for POSIX scripts when Git Bash is installed", and, with Git Bash installed, "the tool is on by
default for claude.ai and Console accounts".

Whether projects on Windows are asked to use Git Bash, with the PowerShell tool off, is not decided: it is
the owner's to rule when this entry is taken up.

What would make it reproduce: a session of Claude Code on Windows with the PowerShell tool on, and one under
zsh, each following a section above.

### Why it matters

An instruction a session cannot run as written stops the procedure it belongs to, against
`goal@agents-get-a-complete-workflow`. Development on Windows is left for later by `spec@plans@gates-on-windows-and-macos`.

### What would close it

Each command above run under PowerShell and under zsh, and either rewritten in a form the three shells take,
or the shell asked of a project stated, with the owner's ruling on Git Bash.
