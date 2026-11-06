---
kind: question
---
# A stdout closed before the process starts reads as a clean run

## Summary

`klarch transcript list >&-` exits 0, while the exit table of `path@cli@README.md` names a closed stdout as exit 2 "as `| head` does".

## Details

### What

Assumption, not established: the Rust runtime reopens a closed descriptor on the null device, so the write succeeds. It would then apply to every command. `| true`, where the pipe closes during the run, exits 2.

### Why it matters

`design@cli@exit-code-ladder` keeps exit 0 for a run that delivered its output, and no output was delivered.

### What would close it

Either the table says which closed stdout it means, or a check at the start of `main` refuses it, with a test.
