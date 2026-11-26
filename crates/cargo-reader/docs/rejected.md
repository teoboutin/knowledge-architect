# Rejected designs

The designs cargo-reader did not keep, and why.

### Cargo's defaults applied by the reader of a manifest `##cargo-defaults-applied`

`live`. An absent `version` read as `0.0.0`, and an absent `publish` read as published or not by
whether `version` is set, take the defaults of one Cargo: `version` became optional at Cargo 1.75,
so before it the same manifest was refused, and a caller could no longer tell a manifest that
writes `version = "0.0.0"` or `publish = true` from one that writes nothing, which a finding that
quotes the manifest needs.
