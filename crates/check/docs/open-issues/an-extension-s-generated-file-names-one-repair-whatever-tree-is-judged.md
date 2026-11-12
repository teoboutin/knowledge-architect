---
kind: defect
---
# An extension's generated file names one repair whatever tree the run judges, so under `check --staged` it cannot name `index --staged`

## Summary

The `generated` check takes the action of an extension's generated file from the extension, as a
fixed `&'static str`. The register indexes name `index` or `index --staged` by the tree the run
judges; an extension's file cannot, so under `check --staged` its finding names a repair that
leaves the staged tree as it was, or no command at all.

## Details

### What

- `Generated` in `path@check@src/extension.rs` is a public struct an extension returns from
  `Prepared::generated`, with three public fields: `rel`, `text`, and `action: &'static str`, "What
  the `generated` check tells a reader to do when the committed file differs".
- `check` in `path@check@src/check/generated.rs` passes `file.action` to `compare` for each
  extension file, whatever `inputs.judged` holds. The file-register indexes get
  `index_action(manifest, inputs.judged)`, which names `index --staged` for `Judged::Index` and
  `index` for `Judged::WorkingTree` and `Judged::Commit`, since the commit "[check] A missing or
  out-of-date generated index names `index --staged` under `check --staged`". The same
  `file.action` reaches the missing, unreadable, directory and out-of-date findings.
- `index --staged`, `index_staged` in `path@check@src/commands/history.rs`, writes every generated
  file into git's index through `generated_list` in `path@check@src/commands/mod.rs`, which takes
  each prepared extension's `generated` files beside the register indexes. So `index --staged` is
  the repair of an extension's stale file under `check --staged` too, and `index` is the repair
  under `check` and, as a history edit's step, under `commits`.
- An extension cannot name that repair: `Generated` is built by `Prepared::generated(model,
  manifest)`, which is given no judged tree, and a `&'static str` cannot hold the manifest's
  declared command. The extensions in `path@testing@tests/extension_api.rs`, `ListingPrepared` and
  `ListingThenFails`, write "regenerate it", which names no command; an extension that writes
  "run `<command> index`" names a command that, under `check --staged`, writes the working tree and
  leaves the finding standing.

Established by reading the code above; not reproduced by a run.

### Why it matters

`design@check@finding-names-the-repair` asks a finding's action to name the repair, and under
`check --staged` the action an extension can give names a repair that does not clear the finding,
or none. The two findings a reader meets on one run, an index's and an extension file's, then name
different repairs for files that one command, `index --staged`, writes.

### What would close it

The `generated` check names, for an extension's file, the repair `index_action` names for the
tree the run judges, with a binary test over an extension's stale file under `check`,
`check --staged` and `commits`. Two shapes, each a change of the public `Generated` struct that a
design discussion weighs:

- the checker names the index command for every generated file, and `Generated.action` leaves,
  since every extension file is written by the same `index` and `index --staged`;
- `Generated.action` stays, as a part the checker adds to its own command, for an extension whose
  file needs a step before the index is written.
