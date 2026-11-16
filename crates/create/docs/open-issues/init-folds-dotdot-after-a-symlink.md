---
kind: design
---
# `init` folds a `..` that follows a symlink without the filesystem, and `cargo init` gives no single behaviour to follow

## Summary

`init link/../x`, where `link` is a symlink to a directory elsewhere, creates `x` beside `link`, since
`lexical` in `path@create@src/command.rs` folds `..` as text. `cargo init` follows the link when it
creates the directory and folds the path as text when it writes `Cargo.toml`, and fails. So
`design@create@init-follows-cargo-init` names no behaviour to follow here, and which one `init`
takes is a decision nobody has made.

## Details

### What

The instance: `lexical` folds every `.` and `..` of PATH "without asking the filesystem, which may
not hold it yet", as its doc comment says. For `link/../x`, with `link` a symlink to
`../elsewhere/dir`, `init` creates `work/x`, which is self-consistent: every later step reads the
same folded path.

What `cargo init` does, measured with cargo 1.99.1, the toolchain this machine selects outside the
repository, on the owner's machine, in a scratch directory
holding `work/link -> ../elsewhere/dir` and an empty `elsewhere/dir`, from `work`:

- `cargo init --vcs none link/../x` creates `elsewhere/x`, the directory the kernel reaches by
  following the link, then fails writing `work/x/Cargo.toml`, a lexically folded path, with
  "No such file or directory (os error 2)", exit 101. It leaves `elsewhere/x` empty.
- `cargo init --vcs none newdir/../y`, with no `newdir`, exits 0, creates `work/y`, and leaves an
  empty `work/newdir` beside it.

Inside the repository the selected cargo is 1.98.0, which was not measured.

The commands to take it again:

```sh
mkdir -p work elsewhere/dir && cd work && ln -s ../elsewhere/dir link
cargo init --vcs none link/../x; echo "exit $?"; find .. | sort
cargo init --vcs none newdir/../y; echo "exit $?"; ls
```

The shapes seen, none argued yet: follow the link as the kernel does, so `init` creates `elsewhere/x`
and writes there; keep the lexical fold, as today; refuse a PATH whose `..` follows a symlink. The
suspected mechanism: `cargo init` resolves PATH in two ways in one run, so no single choice of
`init` is "the same way".

Found by the adversarial review of the slice that moved the commands out of the checker, which
reported `lexical` as departing from `design@create@init-follows-cargo-init`; the measurement above
shows that `cargo init` itself takes both readings.

### Why it matters

`design@create@init-follows-cargo-init` makes a choice `cargo init` already makes the same way in
`init`; here `cargo init` makes two, and one of them fails, so the head cannot be applied, and
`init`'s current behaviour is a departure no head argues. A user who types `init link/../x`
expecting the kernel's reading gets a project in another directory than the one the link reaches,
with no message saying so.

### What would close it

A design discussion, under `skill@knowledge-architect-design`, that picks among the shapes above
and records the departure from `cargo init` in the head that makes it, with a test of `init` over a
PATH whose `..` follows a symlink. Its re-entry point is the next change to how `init` reads PATH.
