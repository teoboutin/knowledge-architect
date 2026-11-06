---
kind: question
---
# An extension reads its manifest table as a `toml::Value`, so `check`'s toml version is part of the public API

## Summary

`Manifest::table` returns `Option<&toml::Value>`, and the crate documentation tells an extension
to read its tables with it. An extension then depends on the toml crate at the major version
`model` uses, and a toml major bump in `model` is a library break. Nothing records whether that
coupling is intended.

## Details

### What

Decide how an extension receives its tables. Three shapes are on the table: keep `toml::Value`
and state the coupling, as `design@cli@arguments-parse-through-clap` does for clap; re-export the
toml crate from the `extension` module, so an extension uses `check`'s copy; or hand the table
over in a form `check` owns, such as a deserialized type the extension names.

### Why it matters

`design@knowledge-architect@api-facade` makes the public surface the facade, but this signature exposes a type
of a dependency the facade does not re-export. Under `design@knowledge-architect@versioning-policy`
a toml major bump in `check` would then be a library break that no change of `check`'s own code
announces. thaum's rules extension calls `table("rules")` and converts the value with
`try_into`.

### What would close it

A recorded decision on the shape, in `check`'s design home, and the signature and the crate
documentation matching it.
