---
kind: defect
---
# An ignored target cited by a commit message alone is reported as missing

## Summary

`commits` takes the ignore batch from the spans of the commit's tree, so a message citing an ignored path that no document cites is told the path does not exist, where `check` accepts the same span.

## Details

### What

In `path@core@src/cli/history.rs`, `ignore_queries` runs over the tree's model, not over the message. With `build/` ignored, a message citing a `path` span to `build/out.txt` gets `does not exist, at build/out.txt`, exit 1. Once a document of the commit's tree cites the same span, the same message passes. Reproduced on the main branch before the crate split.

### Why it matters

It stands against `design@model@ignored-targets-are-not-asserted` and `design@core@a-commit-message-is-a-document`: a message is a document, and the verdict on a span differs from the verdict on the same span in a file.

### What would close it

The batch `commits` takes covers the spans of the message, with a test through the binary.
