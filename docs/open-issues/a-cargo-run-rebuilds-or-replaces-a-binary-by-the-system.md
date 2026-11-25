---
kind: observation
---
# On Windows a working directory spelled another way may rebuild the crates tied to their checkout, and on macOS `cargo run` copies a binary while another runs

## Summary

Two behaviours of cargo were read and not run: the value that ties a build to its checkout follows how the
working directory is spelled, so on Windows two spellings of one directory may rebuild the crates at each
change of spelling; and on macOS cargo copies a built binary into place rather than linking it, so a parallel
`cargo klarch` can meet a moment with no binary.

## Details

### What

- `path@knowledge-architect@.cargo/config.toml` sets `KNOWLEDGE_ARCHITECT_CHECKOUT` to the checkout's root,
  which cargo computes from the path of the configuration file it found, per
  `design@knowledge-architect@a-build-is-tied-to-its-checkout`. On Windows one directory can be reached as
  `C:\x` and `c:\x`, or through a short file name, and each target that reads the variable then rebuilds.
  Not run; whether cargo normalises the spelling is not established.
- On macOS cargo is taken to copy the executable at each uplift, where Linux hard-links it, so that a second
  `cargo run` started meanwhile can find the file absent or partly written. Read in a discussion of cargo's
  behaviour, not in its source, and not run.

What would make each reproduce: on Windows, `cargo klarch check` run alternately from the two spellings, with
the rebuilds counted; on macOS, a loop of parallel `cargo klarch` calls after a change.

### Why it matters

The first costs a release build at each change of spelling; the second can fail a command that a session runs
in parallel with another, which the installed workflow's dispatches do.

### What would close it

Each run as above, and then either a repair of the configuration or of the aliases, or the observation ruled
out with its evidence.
