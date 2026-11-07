---
kind: defect
---
# An ignored target cited by a commit message alone is reported as missing

## Summary

`commits` takes the ignore batch from the spans of the commit's tree, so a message citing an ignored path that no document cites is told the path does not exist, where `check` accepts the same span.

## Details

### What

In `path@check@src/commands/history.rs`, `ignore_queries` runs over the tree's model, not over the message. With `build/` ignored, a message citing a `path` span to `build/out.txt` gets `does not exist, at build/out.txt`, exit 1. Once a document of the commit's tree cites the same span, the same message passes. Reproduced on the main branch before the crate split.

A further instance, the first from a project that installs the workflow: C1 of a retrospective of a session of thaum, 2026-11-02. Its documents cite a git-ignored directory of a Component by a `path` reference, which `check` accepts; its commit message cited a path under that directory, which no document cites, and `commits` refused it as not existing. The session rewrote the sentence to name no path.

### Why it matters

It stands against `design@model@ignored-targets-are-not-asserted` and `design@check@a-commit-message-is-a-document`: a message is a document, and the verdict on a span differs from the verdict on the same span in a file.

### What would close it

The batch `commits` takes covers the spans of the message, with a test through the binary.
