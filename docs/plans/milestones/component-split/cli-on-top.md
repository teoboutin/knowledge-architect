# The binary, the parsing and the interface become the top-level `cli` crate

The fourth slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `cli`, crates/cli, taking the package name `knowledge-architect` from `check`,
  which becomes `knowledge-architect-check`; its goals drafted and put to the owner as the
  milestone document says. `cli` holds the binary `klarch`, the command enum and the clap
  declarations of every klarch command, those of `worklist` and `transcript` included, the
  dispatch, the location of the project, and the calls of the refusals of a foreign build and of an
  unsatisfied pin. It re-exports the extension API under the path an extension uses today, and,
  until #integrated-tests moves the harness into its own crate, the harness too, per D2 of the
  milestone document.
- The refusal of a foreign build, `path@core@src/build_origin.rs`, in `check`'s public API, taking
  the list of crates the binary links; `cli` calls it before every command, and `create` will call
  it for `init`'s PATH.
- Each command's logic behind a typed API of the crate that holds it, with no clap type in it; a
  combination of arguments the domain refuses is a type that cannot be built, and `cli`'s
  declaration refuses it while parsing, per #command-parsing-home.
- `path@core@tests/binary.rs` moved to `cli`, the one package where `CARGO_BIN_EXE_klarch` is set,
  and `path@core@tests/extension_api.rs` moved to `cli` too, since it drives `cli`'s entry; both run
  over the mock projects of the core's directory, which `cli`'s binary links, so `"fixture"` stays
  accepted for them. `check` dev-depends on nothing above it.
- The successor of `design@core@single-crate`, in the incumbent's Component, `check`, in this
  slice's change, by the installed rule of the reversal.

## Claims

- **An extension written against the published `knowledge-architect` compiles with no change to its
  imports.** Test: `path@core@tests/extension_api.rs`, moved to `cli`'s tests, compiles with no
  change but its file's place; shown to discriminate by removing one re-export, which fails it.
- **Every refusal by a domain's options is exit 2 before any work**: premortem C3. Test: one binary
  test per refused combination asserts exit 2 and an untouched tree; a recorded mutation that drops
  one `conflicts_with` fails one.
- **`check`'s public API holds no clap type.** Test: `cargo tree -e normal -p knowledge-architect-check`
  lists no `clap`; shown to fail before the declarations move, when it lists it.
- **The binary's declaration is tested**, per `design@core@arguments-parse-through-clap`: the
  `debug_assert` test and the refusal of an unknown argument, moved to `cli`; shown to fail by a
  recorded malformed declaration, which `debug_assert` refuses.

## Audit subjects

`design@core@single-crate`, reversed; `design@core@the-core-cli-is-a-library-module`,
`design@core@arguments-parse-through-clap`, `design@core@fix-with-staged`,
`design@core@exit-code-ladder`, `design@core@binary-name`, `design@core@a-foreign-build-is-refused`,
`design@core@installed-binary-version-check`, `design@core@checker-source-literals-are-data`,
`design@core@nothing-of-a-project-is-compiled-in`, `design@core@declared-command`;
`design@gates@the-library-owns-the-flags`, whose rule stays and whose clause "as it is of the
core's" is rewritten; `design@mutate@report-not-gate`, whose clause on the core's commands is
rewritten; `design@knowledge-architect@no-component-directory-named-after-the-project`;
`rejected@gates@gates-command-of-the-checker`; `design@agent-skills@xtask-pins-checker` and the
setup snippets, whose dependency line stays; `issue@knowledge-architect@command-output-is-not-declared-a-contract`;
`issue@agent-skills@the-reversal-rule-fixes-the-component-when-the-successor-decides-another-s-interface`,
which this slice leaves open. Each reference naming the anchor `core` here names the entry as this
document is written; #check-renamed re-points them.

## Fails alone on

An extension that no longer compiles against `knowledge-architect`, or a refused combination that
starts work before it is refused.

## Premises that expire

- `cli`'s re-export of the harness is an interim one, which #integrated-tests removes; that slice's
  claim that an extension's tests reach the harness through `knowledge-architect-testing` alone
  guards it.
- `cli`'s tests run over the mock projects of `check`'s directory, a crate's tests reaching another's
  fixtures through a dependency the binary has; #integrated-tests decides their final place, at its
  audit.

`query`, `transcript` and `create` are still modules of `check` until #consumers-out: their
parsing moves to `cli` here, and their logic there; #consumers-out's claim that every moved command
gives the same output guards it.

## Decided design

**The top-level crate.** The commands module must sit above every crate that contributes a command,
or `query` and `create` would form a cycle with it (#arg-15). `design@core@single-crate` argued that
a separate binary crate saves its users no dependency; that premise fails once the commands span
crates (#arg-20). The owner: the binary as a top-level crate that "pulls the logic from the others
and implements the interface" (#arg-32), and the old decision can be reversed (#arg-33). `cli` keeps
the package name, so an extension's imports and the setup snippets' dependency line stay (#arg-21,
#arg-96). `cli` flattens no command of `gates` (#arg-96).

**Where parsing lives.** P1 for klarch's commands: both kinds of binary that offer them, `klarch`
and an extension's own binary, depend on `cli`, so each command keeps one spelling (#arg-72); a
frontend that is no command line gets a typed API (#arg-73); the decisions about the whole interface
get one home (#arg-74); clap leaves the domain crates' interface (#arg-75). A rule that encodes the
domain's meaning, such as `design@core@fix-with-staged`, is a type of the domain, and the declaration
spells it (#arg-76). `gates` and `mutate` keep P2: the binary that offers them is each project's own
maintenance binary, which no shared crate sits above (#arg-71). P2 for klarch's commands is a losing
alternative of the milestone document.

## Implementation sequence

1. The typed APIs of the commands, inside the crate, the clap declarations still beside them: the
   mechanism alone.
2. Create `cli`, move the binary, the enum and the declarations; rename the packages.
3. Move the binary's tests.
4. The reversal's successor in `check`, and the heads rewritten.
5. The harvest.

## Harvest

| what | where |
| --- | --- |
| #cli-crate | the successor of `design@core@single-crate` in `check`'s design home, and the reversed decision judged for the `rejected` home by `skill@knowledge-architect-decision-recording@reversal-check`; `cli`'s design home: `design@core@the-core-cli-is-a-library-module` under a new slug, `design@core@exit-code-ladder`, `design@core@a-foreign-build-is-refused`'s list, `design@core@installed-binary-version-check`, `design@core@checker-source-literals-are-data`'s list of directories, `design@core@nothing-of-a-project-is-compiled-in`'s list, moved or rewritten for `cli`; `design@gates@the-library-owns-the-flags`, `design@mutate@report-not-gate` and `design@knowledge-architect@no-component-directory-named-after-the-project`, each clause rewritten |
| #command-parsing-home | `cli`'s head on its parsing, citing the agent-skills head that `spec@plans@component-rules-revision` harvests for the rule, and `design@core@arguments-parse-through-clap` rewritten; P2 judged as a losing alternative |
| premortem C3 | its claim above |
