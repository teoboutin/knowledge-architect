---
kind: defect
---
# Outside Unix a gates run refuses no pipe, and on a Windows console a cut character can lose the live echo

## Summary

`design@gates@gates-refuse-a-pipe` says that a gates run whose stdout is a pipe is refused outside GitHub
Actions. Outside Unix the code asks nothing and refuses nothing. Under `--full`, on a Windows console, a
chunk of a child's output cut inside a character can lose the rest of the live echo. Two tests are skipped
on Windows for the first.

## Details

### What

- `stdout_is_a_pipe` of `path@gates@src/gates.rs` is compiled for Unix, where it asks whether stdout is a
  FIFO, and answers `false` on every other system. So on Windows `cargo x gates | Select-String x` runs
  every gate, and the pipeline's exit code is the reader's.
- `spawn_reader` of `path@gates@src/process.rs` writes each chunk of up to 8192 bytes to stdout as it
  arrives, and drops a write's error. On a Windows console the standard library refuses bytes that are not
  UTF-8, so a chunk that ends inside a character is written in part, and what is left of the chunk is lost
  to the echo. Read, not run. The log file holds every byte either way.

Skipped on Windows for this entry, in `path@xtask@tests/gates_bin.rs`:
`a_piped_stdout_is_refused_before_any_gate_runs` and
`a_piped_stdout_is_refused_when_the_actions_variable_is_not_true`.

Ruled out: under GitHub Actions a pipe is accepted on every system, so CI is not concerned.

### Why it matters

The refusal protects a person or an agent who filters the report from taking the filter's exit code for the
gates' verdict. On Windows that protection is missing, so `design@gates@gates-refuse-a-pipe` does not hold
there.
The owner left development on Windows for later, while a project developed there is one that
`design@knowledge-architect@platforms-served` serves.

### What would close it

The pipe told on Windows, with the two tests run there; or the head narrowed to Unix by an argument. For the
echo, a chunk cut at a character's end, or a console written as text.
