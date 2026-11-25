---
kind: defect
---
# `init` keeps a document spelled in another case on a disk that ignores case, and `new-component` refuses an absolute DIR under a linked directory

## Summary

On a disk that ignores case, `init` over a directory that holds `Readme.md` takes it for the README.md it
would write, keeps it, and the check then reports README.md missing. On macOS, where the directories /tmp and /var are
symbolic links, `new-component` given an absolute DIR under one refuses it as outside the project. Both were
read in the code and not run.

## Details

### What

- `init` of `path@create@src/command.rs` asks `symlink_metadata` whether a document stands at its path,
  which answers yes for another case on such a disk, and the document is kept. The check compares the
  required path with git's listing byte for byte, which holds `Readme.md`.
- `new_component` of the same file compares `lexical(cwd.join(root))` with `lexical(cwd.join(dir))`. The
  working directory is the resolved one, such as /private/tmp/p on macOS, and an absolute `/tmp/p/svc`
  stays as given, so `project_relative` answers that it is outside the project.

Ruled out: a relative DIR, which is joined to the same working directory as the root, so the two agree.

### Why it matters

A first use of the tool on macOS or Windows meets a finding or a refusal whose words name no right cause,
against `goal@adoption-is-easy`.

### What would close it

`init` refuses, or names, a document that differs from the one it would write by case alone; `new-component`
compares a DIR by its resolved parent; each with a test that runs where the state can be built.
