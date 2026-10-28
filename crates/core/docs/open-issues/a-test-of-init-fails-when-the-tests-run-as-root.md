---
kind: defect
---
# A test of `init` fails when the tests run as root, since root writes into a read-only directory

## Summary

`a_failure_after_the_first_write_exits_one` of `path@core@tests/init.rs` makes a project's `docs/`
read-only to make a write of `klarch init` fail after its first write. A process running as root
writes into a read-only directory, so under root the write succeeds, the run exits 0, and the test
fails on its assertion that the exit code is 1. `cargo x gates` is then red in any environment that
runs the tests as root, such as a container whose user has uid 0, on a tree with nothing wrong.

## Details

### What

- The test sets `b/docs` to mode `0o555` with `std::fs::set_permissions`, runs `klarch init b`,
  and asserts `code == 1`. Under root the assertion fails: the run writes every file, the
  installed agent files included, and exits 0.
- Reproduced in a cloud container whose user is uid 0 (`id -u` prints 0): `cargo test --workspace
  --no-fail-fast` gives 760 passed and 1 failed, the failure this test, on a branch that changes no
  Rust source. A shell check in the same container: `mkdir -p d/docs && chmod 555 d/docs && echo x >
  d/docs/f` succeeds.
- Ruled out: the branch's changes, which touch only Markdown, the installed copies and the
  changelogs. Not established: whether CI runs as a non-root user; the test is assumed to pass there,
  since the main branch merges through CI.
- The other assertions of the test, a file where `.claude` is due refusing the run with exit 2 and
  the `c` case of `check --fix`, do not depend on permissions.

### Why it matters

A gate that fails on a correct tree costs every session in such an environment a diagnosis, and
teaches it to read a red gate as noise, against `goal@knowledge-architect@agents-work-without-drift`.
The claim the test makes, a failure after the first write exits 1, is untested under root.

### What would close it

The test makes the write fail by a means root does not bypass, such as a file where the directory
`docs/` is due, or skips the read-only case when the effective uid is 0 and says so in its output;
then `cargo test --workspace` passes as root, and the test still fails against an `init` that exits
2 after its first write.
