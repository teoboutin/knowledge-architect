---
kind: question
---
# A tool that holds no extension reads a project with one differently from the checker

## Summary

Phase 1 needs each extension's contribution, which a tool over the documents with no extension does not hold, so its gate and its walk differ from the binary that registers the extension.

## Details

### What

`complete_working_tree` judges what the manifest declares against what `contribute` recorded. A tool that never calls it reports no extension table, so it passes a table the binary refuses, as `issue@check@a-typed-command-trusts-its-caller-to-have-configured-the-manifest` measures; and it walks the files the extension generates, so an entry defined in one is read where the binary does not read it. `contribute` with the default value reports every extension table of the manifest, as `[rules] is a table no extension of this binary reads`, where the binary that registers the extension passes. The crate's documentation states the call and its order.

The pin is the same kind of gap: `refuse_another_version` is `check`'s, so such a tool reads a tree under another version's grammar with no refusal.

The question: what does a tool such as `show` or a renderer, once it links no check, do over a project with an extension? It may read the table names alone and leave the rest, read the binary's list, or be run only by a binary that registers the extensions.

### Why it matters

It bears on `design@model@contribution-is-data` and `design@knowledge-architect@model-crate`, and `spec@component-split@consumers-out` moves `show`, `issues`, `tripwires`, `anchors`, `id` and `worklist` to a crate below the extensions. Today `cli::run` hands them a configured manifest.

### What would close it

The audit of that slice answers it, the owner ruling whether a pin or an extension's table is what
`goal@model@a-tool-knows-the-read-is-right` asks the library to name, with a test of `query`'s commands over a project with an extension's table.
