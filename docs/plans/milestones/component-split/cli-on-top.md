# The binary, the parsing and the interface become the top-level `cli` crate

The fourth slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `cli`, crates/cli, taking the package name `knowledge-architect` from `check`,
  which becomes `knowledge-architect-check`; its goals drafted and put to the owner as the
  milestone document says. That each command keeps one spelling in
  every binary stays `design@cli@the-commands-are-a-library-module`, a means to
  `goal@agent-skills@installed-text-works-in-every-target-project`, and no goal of its own. `cli` holds the binary `klarch`, the command enum and the clap
  declarations of every klarch command, those of `worklist` and `transcript` included, the
  dispatch, the location of the project, and the calls of the refusals of a foreign build and of an
  unsatisfied pin.
- **`cli`'s library keeps every path a consumer names today**: the crate root, and the modules
  `knowledge_architect::cli`, `extension`, `document` and `testing`, the last until #integrated-tests
  moves the harness into its own crate, per D2 of the milestone document. The setup snippet,
  `path@agent-skills@snippets/xtask-main.rs`, names `cli::Command`, `Library`, `this_library`, `init`,
  `locate`, `refuse_a_foreign_build`, `refuse_another_version` and `run`, and stays unchanged. Three kinds
  of item stand behind those paths:
  - **a re-export of `check`**: `extension`, `document`, `testing`, `Document`, `Finding`, `Manifest`,
    `Model`, `MANIFEST_NAME`, `CHECKER_VERSION`, `cli::Gathered`, `cli::Library`,
    `cli::complete_working_tree`, `cli::output`, `cli::refuse_a_foreign_build` and
    `cli::refuse_another_version`;
  - **`cli`'s own, since it evaluates at `cli`'s build or names a clap type**: `component_dir` and
    `cli::this_library`, which name crates/cli and the package `knowledge-architect`, and `cli::init`,
    a function that takes the clap type `InitArgs` and calls `check`'s typed `init`, with its own
    `cli::Vcs`, since a clap `ValueEnum` cannot be implemented on another crate's type;
  - **`cli`'s code**: `cli::Command`, each `*Args`, `cli::run` and `cli::locate`.
- The refusal of a foreign build, `path@check@src/build_origin.rs`, and of an unsatisfied pin,
  `refuse_another_version` with its helpers and their tests, in `check`'s public API. The pin is the
  version of the `check` library and `"self"` and `"fixture"` read `check`'s directory, per
  `design@check@installed-binary-version-check`, so its body stays with that library. `check` publishes
  `CRATE_DIR` and `PACKAGE` as `git` and `model` do. `klarch`'s `main` lists the five libraries it links,
  `cli`, `check`, `git`, `model` and `agent-skills`, and hands the walk the directories of `cli`, `check`,
  `git` and `model`. `cli` calls the refusal before every command but `init`, which reaches it over
  PATH inside `check`'s `init`, and `create` will call it for `init`'s PATH.
- Each command's logic behind a typed API of the crate that holds it, `check` here, with no clap type
  in it, under the names of "New names". A combination of arguments the domain refuses is a type that
  cannot be built, and `cli`'s declaration refuses it while parsing, per
  `thread@component-split@command-parsing-home`. Each `*Args` struct stays public with its fields, so a
  value `cli::run` is given can hold a combination the type refuses: `cli::run` converts it, and
  returns `Err`, exit 2, before any work. Each typed command takes a manifest already configured, and
  its doc comment says so, since `cli::run` does that today for every command.
- **The typed API prints and returns `Result<ExitCode, String>`, as the commands do today**: D5 of the
  milestone document, ruled, the refactoring recorded in
  `issue@check@a-typed-command-prints-and-exits-rather-than-returning-data`. `path@check@src/commands/output.rs`, `print_report`, `counts`, `verdict` and `print_rows`
  stay in `check`, in `commands`, and `cli` prints nothing but a refusal of its own.
- `path@check@src/commands/`, the commands' logic with their declarations out, is `check`'s role module. `extension`, `document`,
  `testing` and `commands` are `check`'s facade, under `#![warn(unreachable_pub)]`. The extension
  writer's crate documentation and its doctests, and the restatement comment under them, move to
  `cli`'s lib.rs, and `check`'s lib.rs gets its own documentation.
- The tests that need the binary or name `knowledge_architect` move to `cli`:
  `path@cli@tests/binary.rs` and `path@cli@tests/init.rs`, since `cli` is the one package where
  `CARGO_BIN_EXE_klarch` is set; `path@cli@tests/extension_api.rs` and `path@cli@tests/public_paths.rs`,
  since they drive `cli`'s library and its whole list of public paths. So do `cli_declaration_is_consistent`,
  `an_unknown_argument_is_refused` and `fix_and_staged_are_refused_together` of `path@cli@src/main.rs`,
  and `run_refuses_init` of `path@check@src/commands/create.rs`, which calls `run`. Each file reads `check`'s
  fixtures through `check`'s directory: the mock projects, the transcripts and the worklist's reference
  copy, which `cli`'s binary links, so `"fixture"` stays accepted for them. `check` dev-depends on
  nothing above it.
- `path@check@README.md` is split. `cli`'s takes installing, the usage of each command and the exit
  codes, with the outcome each command maps to each code. `check`'s keeps the rules a project's
  documents are checked by, so `skill@knowledge-architect-project-audit`'s lens, which reads the README
  that ships in the checker's package, still finds them, the lens naming that package. Both packages
  state the four conditions of a target project, the asks and the limit of the guarantee.
  `path@check@CRATES-IO.md` moves to `cli`, and `check` gets a short page of its own, which installs no
  binary.
- `knowledge-architect-check` is a name new on crates.io, so its first version is published by hand, per
  D1. `knowledge-architect` is not. It joins both loops of `path@knowledge-architect@.github/workflows/release.yml`
  and the lists of `skill@klarch-release`, before `knowledge-architect`.
- What follows the move, beside the milestone's list: the patch line of `path@xtask@tests/scaffold.rs`
  that points `knowledge-architect` at crates/check; `check`'s row of the root `CLAUDE.md`'s Component
  table; the probe `-p knowledge-architect` in `issue@check@the-tests-read-the-machine-s-git-configuration`
  and in `issue@model@the-resolver-s-arms-have-mutation-survivors`, which then names one package of two and
  no check reports, so each names both; the intra-doc links of
  `path@check@src/extension.rs`, `path@check@src/testing.rs` and `path@check@src/check/mod.rs` to
  `crate::cli`; the sections of `path@check@CLAUDE.md` on `Sandbox`, `History` and the test files, which
  go to `cli`'s `CLAUDE.md`. A pointer by function name is re-pointed by a search, since a pointer to a
  file that stays still resolves: `issue@check@the-missing-directory-refusal-does-not-hold-the-no-loss-claim`
  and `issue@check@tests-around-the-installed-files-let-mutations-survive`, at `check_destinations` and
  `install_agent_skills`.
- `cli`'s register homes are filled by hand where a head moves into them, per
  `issue@check@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`: the sentence
  `new-component` writes says the home holds no entry, and no check reads it.
- The successor of `rejected@knowledge-architect@single-crate`, in the root's design home, in this slice's change: a
  decision about which Components exist and how they depend on each other, per point 1 of
  `skill@knowledge-architect-decision-recording@owning-component`, where the reversal rule sends a
  successor; `rejected@knowledge-architect@single-crate` leaves the design home of `check` in the same change. It states
  that `cli` sits above every crate that contributes a command, and leaves which crates `check` may depend
  on to `issue@knowledge-architect@the-checker-may-depend-on-every-crate-below-it`. The sentence of
  `design@check@transcript-is-a-command-of-every-binary` that argues from `rejected@knowledge-architect@single-crate` is rewritten in the same
  change, so no citation dangles, and #consumers-out moves the rest of that head.

## Claims

- **An extension written against the published `knowledge-architect` compiles with no change to its
  imports.** Test: `path@cli@tests/extension_api.rs` and `path@cli@tests/public_paths.rs`, moved to
  `cli`'s tests, with no line changed but the paths of their fixtures and the comment on whose test crate
  they are, and `cli::output::write` added to the whole list; shown to discriminate by removing one
  re-export, which fails their build. The mutation names `--build "cargo build -p knowledge-architect"`,
  a probe the mutant passes, since a probe that compiles the test target reads `unmutatable here`, and it
  removes a re-export `klarch`'s `main` does not name.
- **The binary links five libraries, and each is refused as a foreign copy.** Test: the binary test of
  the refusal, one case for `cli` and one for `check`, and the rest as
  `issue@check@the-self-pin-refusal-has-no-discriminating-test` holds them; shown to
  discriminate by removing each new entry from `klarch`'s list, which fails its case.
- **Every refusal by a domain's options is exit 2 before any work**: premortem C3. Five combinations are
  declared: `check --fix --staged`, `worklist --out --verify`, `worklist` with neither, `worklist --verify
  --bound` and `worklist --verify --seed`. Test: one binary test per combination asserts exit 2 and an
  untouched tree, and one unit test per `*Args` value that holds a refused combination asserts `cli::run`
  returns `Err`, which clap never sees; a recorded mutation that drops one `conflicts_with` fails one. A
  state refusal, such as `design@check@fix-refusal-mixed-state`, which can exit 1, is no refusal by
  options, and is outside this claim.
- **`check`'s public API holds no clap type.** Test: `cargo tree -e normal -p knowledge-architect-check`
  lists no `clap`; shown to fail before the declarations move, when it lists it.
- **The binary's declaration is tested**, per `design@cli@arguments-parse-through-clap`: the
  `debug_assert` test and the refusal of an unknown argument, moved to `cli`; shown to fail by a
  recorded malformed declaration, which `debug_assert` refuses.

## Audit subjects

`rejected@knowledge-architect@single-crate`, reversed; `design@cli@the-commands-are-a-library-module`,
`design@cli@arguments-parse-through-clap`, `design@check@fix-with-staged`,
`design@cli@exit-code-ladder`, `design@cli@binary-name`, `design@check@a-foreign-build-is-refused`,
`design@check@installed-binary-version-check`, `design@check@checker-source-literals-are-data`,
`design@check@nothing-of-a-project-is-compiled-in`, `design@check@declared-command`,
`design@knowledge-architect@api-facade`, `design@check@inputs-are-gathered`,
`design@check@an-extension-plugs-in-through-phased-hooks`,
`design@check@an-extension-claims-its-manifest-tables`, `design@check@transcript-is-a-command-of-every-binary`;
`design@gates@the-library-owns-the-flags`, whose rule stays and whose clause "as it is of `check`'s" is
rewritten; `design@mutate@report-not-gate`, whose clause on `check`'s commands is rewritten;
`design@knowledge-architect@no-component-directory-named-after-the-project`;
`rejected@gates@gates-command-of-the-checker`; `design@agent-skills@xtask-pins-checker`, whose dependency
line stays and whose words "through `check`'s library" turn false, and the setup snippets;
`design@xtask@one-module-per-subcommand`; `issue@knowledge-architect@command-output-is-not-declared-a-contract`.
The issue `the-reversal-rule-fixes-the-component-when-the-successor-decides-another-s-interface`,
closed by the revision of the rules on Components, gave this slice its successor's Component.
`goal@check@checks-a-commit-from-its-hook` is met by a test that moves to `cli`: the goal stays in
`check` as it stands, since "the checker" is the root's term for a command and names no crate.

**No change of this slice sets what every commit's tree must pass**, as the milestone document's "How
the work is done" asks each audit to state. The tip's list of the tool's own directories only grows by
crates/cli, which no earlier tree holds, and crates/check stays on it. `commits` compares no installed file
on a commit's tree, per `design@check@only-check-compares-installed-files`, so a shipped byte that changes fails none.
`cargo klarch commits` on the branch confirms it. Steps 2 and 3 land as one commit, since the binary's
tests compile nowhere between them.

## Fails alone on

An extension that no longer compiles against `knowledge-architect`, or a refused combination that
starts work before it is refused.

## Premises that expire

- `cli`'s re-export of the harness is an interim one, which #integrated-tests removes; that slice's
  claim that an extension's tests reach the harness through `knowledge-architect-testing` alone
  guards it.
- `cli`'s tests run over the fixtures of `check`'s directory, a crate's tests reaching another's
  fixtures through a dependency the binary has; #consumers-out moves the transcripts and the worklist's
  reference copy, and #integrated-tests decides the mock projects' final place, at its audit.
- `output`, in `check`, serves commands that #consumers-out moves to `query`, `transcript` and `create`,
  none of which may need `check`: that slice's spec holds the question.

`query`, `transcript` and `create` are still modules of `check` until #consumers-out: their
parsing moves to `cli` here, and their logic there; #consumers-out's claim that every moved command
gives the same output guards it.

## New names, in one place

```text
crates/cli                                  package knowledge-architect, lib knowledge_architect, bin klarch
crates/check                                package knowledge-architect-check, lib knowledge_architect_check
knowledge_architect_check::commands         the module named in Builds: each command's typed entry
commands::CheckMode                         Working | Staged | Fix
commands::IndexMode                         Working | Staged
commands::WorklistMode                      Write { dir, bound, seeds } | Verify { dir }
commands::InitOptions, commands::Vcs        what init takes, clap-free
commands::IssuesFilter, TripwiresFilter     what the two listings take
commands::TranscriptVerb                    List(ListOptions) | Show(ShowOptions) | Search(SearchOptions)
knowledge_architect_check::CRATE_DIR, PACKAGE    as git and model publish them
```

`cli::CheckArgs`, `IndexArgs`, `WorklistArgs`, `InitArgs`, `TranscriptArgs` and the rest keep their paths.

## Decided design

**The top-level crate.** The commands module must sit above every crate that contributes a command,
or `query` and `create` would form a cycle with it (`argument@component-split@arg-15`).
`rejected@knowledge-architect@single-crate` argued that a separate binary crate saves its users no dependency; that
premise fails once the commands span crates (`argument@component-split@arg-20`). The owner: the binary as a
top-level crate that "pulls the logic from the others and implements the interface"
(`argument@component-split@arg-32`), and the old decision can be reversed
(`argument@component-split@arg-33`). `cli` keeps the package name, so an extension's imports and the setup
snippets' dependency line stay (`argument@component-split@arg-21`, `argument@component-split@arg-96`). `cli`
flattens no command of `gates` (`argument@component-split@arg-96`).

**Where parsing lives.** P1 for klarch's commands: both kinds of binary that offer them, `klarch`
and an extension's own binary, depend on `cli`, so each command keeps one spelling
(`argument@component-split@arg-72`); a frontend that is no command line gets a typed API
(`argument@component-split@arg-73`); the decisions about the whole interface get one home
(`argument@component-split@arg-74`); clap leaves the domain crates' interface
(`argument@component-split@arg-75`). A rule that encodes the domain's meaning, such as
`design@check@fix-with-staged`, is a type of the domain, and the declaration spells it
(`argument@component-split@arg-76`). `gates` and `mutate` keep P2: the binary that offers them is each
project's own maintenance binary, which no shared crate sits above (`argument@component-split@arg-71`). P2
for klarch's commands is a losing alternative of the milestone document.

**The typed API at this slice.** It takes the domain's types and prints, as D5 of the milestone document
states, on the default the component-creation skill gives, "both sit by default in the Component that
returns or defines the data", and on the step's "the mechanism alone". What
`argument@component-split@arg-73` asks of it holds in part until that issue closes: a frontend names no clap
type, and it still receives stdout and a process that exits on a closed pipe.

## Implementation sequence

1. The typed APIs of the commands, inside the crate, the clap declarations still beside them: the
   mechanism alone.
2. Create `cli`, move the binary, the enum and the declarations; rename the packages; move the binary's
   tests and their fixture paths, in one commit with this step.
3. The foreign-build list's new entries, the refused combinations' tests, and the READMEs, pages and
   crate documentation split.
4. The reversal's successor in the root's design home, and the heads rewritten.
5. The harvest.

## Harvest

Each head's row states whether it moves, because its subject is `cli`'s, or stays, because its code or its
subject stays, with only its false sentences rewritten. Slugs are written beside each new head.

| what | where |
| --- | --- |
| `thread@component-split@cli-crate` | the successor of `rejected@knowledge-architect@single-crate` in the root's design home, under the slug of this thread, and the reversed decision judged for the `rejected` home by `skill@knowledge-architect-decision-recording@reversal-check`; a head of its own for `cli` keeping the package name, whose rival is a new name with changed imports |
| moved to `cli`'s design home | `design@cli@the-commands-are-a-library-module` under a new slug, `design@cli@arguments-parse-through-clap`, `design@cli@exit-code-ladder`, `design@cli@binary-name` with `rejected@cli@binary-named-after-package` |
| staying in `check`, each false sentence rewritten | `design@check@a-foreign-build-is-refused`, for "`check`'s binary does this in" a test file that moves; `design@check@installed-binary-version-check`, whose rule stays, read as a move of nothing; `design@check@checker-source-literals-are-data`, for the directories a binary hands the walk; `design@check@nothing-of-a-project-is-compiled-in`, whose items are stated by rule, so only its pointers change; `design@check@declared-command`, `design@check@an-extension-plugs-in-through-phased-hooks` and `design@check@an-extension-claims-its-manifest-tables`, each for "`check`'s binary"; `design@check@inputs-are-gathered`, for "`check`'s command module"; `design@check@the-facade-publishes-what-a-consumer-needs`, for its `cli` row, split from `design@knowledge-architect@api-facade`, which moves to the root since it holds of the two crates an extension is written against; `design@check@transcript-is-a-command-of-every-binary`, as Builds says |
| clauses of other Components | `design@gates@the-library-owns-the-flags`, `design@mutate@report-not-gate`, `design@knowledge-architect@no-component-directory-named-after-the-project`, `design@agent-skills@xtask-pins-checker`, `design@xtask@one-module-per-subcommand` with `path@xtask@README.md` and the comment of `path@xtask@Cargo.toml`, each rewritten where it names `check`'s library for what is now `cli`'s |
| `thread@component-split@command-parsing-home` | `cli`'s head on its parsing, under the slug of this thread, and the two heads of `register@agent-skills@design` on the top-level binary crate and on the crate nearest the binaries that parses a command, which the component-creation skill states alone today in two comment lines, `cli`'s head citing them; both comment lines then cut to a citation of their head, with no installed byte changed; `design@cli@arguments-parse-through-clap` rewritten; `design@check@fix-with-staged` rewritten, the rule being `check`'s type that `cli` spells, and its quotation of the owner converted to an attribution that keeps its hedge, per `design@agent-skills@attribution-replaces-quotation`; P2 judged as a losing alternative |
| issues | `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`, whose "the install lives in `check`, and it is not public API" turns false for the command's entry; the issues Builds names; those `cargo klarch check` reports, re-pointed |
| D5 | `design@check@a-typed-command-prints-and-returns-its-code`, in the weak form |
| premortem C3 | its claim above |
