# Rejected designs — gates

The designs the Component gates did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@gates@design` whose argument covers it. An entry stays `live` while its
reason holds, and is marked `void` when it no longer does.

### Capturing a child's two streams through one regular file instead of two pipes `##streams-through-one-file`

`live`. Its one reason was a child that refuses a pipe on stdout, which the checker's `check` did
for a while, per the entry on that refusal, `rejected@check@check-refusing-a-pipe`; no child
refuses one now. Its one gain is that both streams keep the order the child wrote them in. It was
built, with the file in the system temporary directory, opened in append mode for both streams and
read back by polling, and an adversarial review reproduced three losses the pipes do not have:

- a child that reopens its standard output by the device path, /dev/stdout, truncates the capture:
  `sh -c 'echo first-line-that-is-long; echo b > /dev/stdout; echo c'` captured `b` and `c` alone;
- output a descendant writes after the child exits is dropped:
  `sh -c '(sleep 0.3; echo late) & echo early'` captured `early` alone;
- a killed run leaves the file behind, and since the name was the process id and a counter,
  created with `create_new`, a later run that reuses the process id fails every gate.

No gate child did any of the three, so the losses are latent. They are what a reintroduction has to
answer.

### A plain `cargo doc` gate, with an ask that each project declares its `[lints.rustdoc]` levels `##plain-cargo-doc-with-a-lints-ask`

`live`. A plain `cargo doc --no-deps` reports a link that names nothing as a warning and exits 0,
observed on cargo 1.98.0, so in a project that has not yet declared the levels the gate passes on
the broken link it exists to fail. The run that shows it is row 1 of the script of
`design@gates@doc-gate-denial`.

### The deny passed through cargo's `build.rustdocflags`, in a configuration file or by `--config` `##deny-through-build-rustdocflags`

`live`. An exported `RUSTDOCFLAGS` replaces `build.rustdocflags`: with `RUSTDOCFLAGS="--cfg foo"`
exported, a crate whose configuration file, or a `--config` argument, sets `["-D", "warnings"]`
documents a broken link with a warning and exits 0, observed on cargo 1.98.0. The deny is lost with
no sign. The runs that show it are rows 5 and 6 of the script of `design@gates@doc-gate-denial`.

### The link check carried by the `test` gate, through its doc tests `##link-check-in-the-test-gate`

`live`. `cargo test --doc` does not report a link that names nothing, and exits 0, with
`RUSTDOCFLAGS="-D warnings"` exported, observed on cargo 1.98.0. The run that shows
it is row 7 of the script of `design@gates@doc-gate-denial`.
