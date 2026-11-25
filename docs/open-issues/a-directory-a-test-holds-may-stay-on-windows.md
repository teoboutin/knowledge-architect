---
kind: observation
---
# Whether a test's scratch directory is removed on Windows is not established

## Summary

A test's scratch directory is removed when its guard is dropped. Windows can refuse to remove a directory
while a file in it is open, and the guard says nothing of a removal that fails, so a run that passes there
may leave directories behind.

## Details

### What

A lead, not established for any test, carried from an issue that the work on the gates on Windows closed.
A run of the suite on Windows that passes says nothing of it, as run 45637540683 does, since the drop of a
`tempfile::TempDir` ignores a removal that fails. What the run leaves is found by its prefix, `klarch-`, in
the temporary directory. A neighbour lead is ruled out by reading: git makes the files of its objects
read-only, and the standard library's removal on Windows ignores that attribute.

### Why it matters

`design@knowledge-architect@scratch-removed-by-a-guard` says that a run that ends leaves nothing in the
temporary directory, and an entry left by any test reopens it.

### What would close it

A listing of `klarch-*` in the temporary directory after a run of the suite on Windows, empty or not, with
the cause of what stays.
