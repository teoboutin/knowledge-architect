---
kind: todo
---
# Some behaviour the commands moved with survives a mutation over the whole suite

## Summary

The adversarial review before the merge of the slice that moved the binary to the top found mutations of moved or thin code that
no test catches, each over `cargo test -p knowledge-architect -p knowledge-architect-check`. None is a wrong result today, each
read or run by hand. They are recorded together, since each is one small test.

## Details

### What

In `main` of `path@cli@src/main.rs`: `cli::init` given `&[]` for `checker`. `refuse_another_version` given `&[]` for its
directories, so that `"fixture"` is held only for a mock under `check`, since every mock of the suite lies there.

In `path@cli@src/cli/mod.rs`: `DEFAULT_WORKLIST_BOUND` changed to `26000`, against the README's 25,000 words; `pub rust` of
`InitArgs` made private, since no test outside the crate builds one; `#[non_exhaustive]` removed from `Command`; the message
of `locate` emptied.

In `path@check@src/commands/mod.rs`: `install_agent_skills`'s test of `manifest.complaints()` made `if false`; `show`'s
`trim_matches` of a backtick removed; `canonical` removed from `is_a_library_fixture` and from `is_its_own_build`, since no
test reaches them through a symlink for the library's directory. In `path@check@src/build_origin.rs`: the full-length trailing
run left out of `foreign_copy`.

The `--role` list: removing a role from `ROLES` of `path@check@src/commands/transcript.rs` is caught by no test relating it to
`label.role()`.

### Why it matters

Each claim is stated in a doc comment or a head, and a later change could break it with every gate green. The session chose to
list these rather than build them at that slice, since each predates it or is thin code that holds today by hand. None bears on
`goal@cli@a-refused-invocation-does-no-work`.

### What would close it

A test per item a later change touches, or the item judged to need none and removed from this entry.
