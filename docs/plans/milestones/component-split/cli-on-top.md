# The binary, the parsing and the interface become the top-level `cli` crate

The fourth slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `cli`, crates/cli, taking the package name `knowledge-architect` from `check`,
  which becomes `knowledge-architect-check`. `cli` holds the binary `klarch`, the command enum and
  its clap declaration, the dispatch, the location of the project, the refusal of a foreign build
  and of an unsatisfied pin, and re-exports the extension API and the test harness under the paths
  an extension uses today.
- Each command's logic behind a typed API of the crate that holds it, with no clap type in it; a
  combination of arguments the domain refuses is a type that cannot be built, and `cli`'s
  declaration refuses it while parsing, per #command-parsing-home.
- `path@core@tests/binary.rs` moved to `cli`.

## Claims

- **An extension written against the published `knowledge-architect` compiles with no change to its
  imports.** Test: `path@core@tests/extension_api.rs`, moved to `cli` or run against it, compiles
  unchanged; shown to discriminate by removing one re-export, which fails it.
- **Every refusal by a domain's options is exit 2 before any work**: premortem C3. Test: one binary
  test per refused combination asserts exit 2 and an untouched tree; a recorded mutation that drops
  one `conflicts_with` fails one.
- **`check`'s public API holds no clap type.** Test: `cargo tree -p knowledge-architect-check`
  lists no `clap`; refuted by any.
- **The binary's declaration is tested**, per `design@core@arguments-parse-through-clap`: the
  `debug_assert` test and the refusal of an unknown argument, moved to `cli`.

## Audit subjects

`design@core@single-crate`, reversed; `design@core@the-core-cli-is-a-library-module`,
`design@core@arguments-parse-through-clap`, `design@core@fix-with-staged`,
`design@core@exit-code-ladder`, `design@core@binary-name`, `design@core@a-foreign-build-is-refused`,
`design@core@installed-binary-version-check`, `design@core@checker-source-literals-are-data`,
`design@core@nothing-of-a-project-is-compiled-in`, `design@core@declared-command`;
`design@gates@the-library-owns-the-flags`, which stays; `rejected@gates@gates-command-of-the-checker`;
`design@agent-skills@xtask-pins-checker` and the setup snippets, whose dependency line stays;
`issue@knowledge-architect@command-output-is-not-declared-a-contract`. Each reference naming the anchor
`core` here names the head as this document is written; #check-renamed re-points them.

## Fails alone on

An extension that no longer compiles against `knowledge-architect`, or a refused combination that
starts work before it is refused.

## Premises that expire

`query`, `transcript` and `create` are still modules of `check` until #consumers-out; their parsing
moves to `cli` here, and their logic there.

## Decided design

**The top-level crate.** The commands module must sit above every crate that contributes a command,
or `query` and `create` would form a cycle with it (#arg-15). `design@core@single-crate` argued that
a separate binary crate saves its users no dependency; that premise fails once the commands span
crates (#arg-20). The owner: the binary as a top-level crate that "pulls the logic from the others
and implements the interface" (#arg-32), and the old decision can be reversed (#arg-33). `cli` keeps
the package name, so an extension's imports and the setup snippets' dependency line stay (#arg-21,
#arg-96). The successor head is a layout decision, recorded in the root design home (#arg-98). `cli`
flattens no command of `gates` (#arg-96).

**Where parsing lives.** P1 for klarch's commands: both kinds of binary that offer them, `klarch`
and an extension's own binary, depend on `cli`, so each command keeps one spelling (#arg-72); a
frontend that is no command line gets a typed API (#arg-73); the decisions about the whole interface
get one home (#arg-74); clap leaves the domain crates' interface (#arg-75). A rule that encodes the
domain's meaning, such as `design@core@fix-with-staged`, is a type of the domain, and the declaration
spells it (#arg-76). `gates` and `mutate` keep P2: the binary that offers them is each project's own
maintenance binary, which no shared crate sits above (#arg-71). P2 for klarch's commands is a losing
alternative of the milestone document.

## Implementation sequence

1. The typed APIs of `check`'s commands, inside the crate, the clap declarations still beside them:
   the mechanism alone.
2. Create `cli`, move the binary, the enum and the declarations; rename the packages.
3. Move the binary's tests.
4. The harvest.

## Harvest

| what | where |
| --- | --- |
| #cli-crate | `cli`'s design home: the moved heads; `design@core@single-crate` reversed, its successor recorded at the milestone's layout head and the reversed decision judged for the `rejected` home by `skill@knowledge-architect-decision-recording@reversal-check`; `design@core@the-core-cli-is-a-library-module` rewritten |
| #command-parsing-home | `cli`'s design home, with `design@core@arguments-parse-through-clap` rewritten; P2 judged as a losing alternative |
| premortem C3 | its claim above |
