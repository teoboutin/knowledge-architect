---
kind: todo
---
# Some behaviour the commands moved with survives a mutation over the whole suite

## Summary

The adversarial review before the merge of the slice that moved the binary to the top found mutations of moved or thin code that
no test catches, each over `cargo test -p knowledge-architect -p knowledge-architect-check`, as the code stood before the commands moved. None is a wrong result today, each
read or run by hand. They are recorded together, since each is one small test.

## Details

### What

In `main` of `path@cli@src/main.rs`: `cli::init` given `&[]` for `checker`.

In `path@cli@src/cli/mod.rs`: `pub rust` of
`InitArgs` made private, since no test outside the crate builds one; `#[non_exhaustive]` removed from `Command`; the message
of `locate` emptied.

In `path@query@src/show.rs`: `show`'s `trim_matches` of a backtick removed. In `path@check@src/commands/mod.rs`: `install_agent_skills`'s test of `manifest.complaints()` made `if false`; `canonical` removed from `is_a_library_fixture` and from `is_its_own_build`, since no
test reaches them through a symlink for the library's directory. In `path@check@src/build_origin.rs`: the full-length trailing
run left out of `foreign_copy`.

The `--role` list: removing a role from `ROLES` of `path@transcript@src/command.rs` is caught by no test relating it to
`label.role()`.

The review of the slice that moved the commands out of the checker found more, each a mutation it ran that survived:

- `path@cli@src/cli/mod.rs`, `run`: `show`, `issues`, `tripwires`, `anchors` and `worklist` can each hand `&[]` for `checker`; only `id`'s is caught.
- `path@query@src/rows.rs`: `tripwires`' anchor and text filters, its order and its trim of a backtick, and the filter of `issues` to the issue register.
- `path@query@src/worklist.rs`: the `..` arm of `absolute`, the trim of a seed's backticks, and either half of its refusal of a directory inside the project.
- `path@query@src/show.rs`: the order of `cited_at`, and the exit of the `no anchor is named` arm.
- `path@query@src/anchors.rs`: two tests of what a home holds.
- `path@transcript@src/command.rs`: the bounds of a search's line, and subagent tool lines under `--tools`.
- `path@transcript@src/reader.rs`: three labels in `never_cut`, canonicalisation of a root, the directory walk's tolerance, an empty `CLAUDE_CONFIG_DIR` or `HOME`, a subagent no session of its file holds, and known values no fixture holds.
- `path@create@src/command.rs`: `with_component`'s `\r\n` inside the array, `append`'s line for an element on the closing bracket's line, shipped destinations in the overlap list, and the output lines `created  a git repository`, `kept     .gitignore` and the enclosing project's note. `append`'s comment that "a reason never reads as the new element's" is false of `components = ["a", "b", # why b\n]`.
- `path@create@src/command.rs`, `lexical`: `init link/../x` folds `..` without the filesystem, where `cargo init` follows the link, against `design@create@init-follows-cargo-init`.

### Why it matters

Each claim is stated in a doc comment or a head, and a later change could break it with every gate green. The session chose to
list these rather than build them at that slice, since each predates it or is thin code that holds today by hand. None bears on
`design@cli@a-refusal-by-arguments-writes-nothing`.

### What would close it

A test per item a later change touches, or the item judged to need none and removed from this entry.
