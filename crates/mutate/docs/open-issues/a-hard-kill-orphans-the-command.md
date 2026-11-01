---
kind: observation
---
# A hard kill of a run leaves the command it started running

## Summary

`run` starts the caller's command with `Command::status()`, which inherits stdio and blocks until
the command returns. A `SIGKILL` to the process that runs the mode does not reach the command, so a
long-running command keeps going with nothing waiting on it.

## Details

### What

Observed in thaum, where the tool was built, by an adversarial reviewer of the branch that added the
hand mutation: `-- sleep 600`, `kill -9` the parent, and the `sleep` was still running afterwards.
It reproduces here: the test `a_run_killed_mid_mutation_is_restored_byte_for_byte` in
`path@mutate@tests/modes.rs` kills a run and must then stop the command itself, through a file the
command polls.

### Why it matters

The recovery is unaffected: the record is on disk before the target is touched, so `restore` puts
the file back whether or not the command still runs. What is left is a stray process, and, if the
command writes to the target, one writing into a file a later run may have claimed. Not
established: whether a command a session would plausibly pass writes to its own target.

### What would close it

Either the command put in a process group of its own, which the run kills on its way out, which
needs a signal handler and takes the tool beyond `std::process`; or a decision that a stray command
is acceptable because the record already protects the tree.
