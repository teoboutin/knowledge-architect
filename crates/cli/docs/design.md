# Design

How the command line is built, and why. The designs this Component did not keep are
`register@cli@rejected`.

## The commands

### The commands are a library module, so a binary that registers extensions offers them unchanged `##the-commands-are-a-library-module`

The commands, `check`, `show`, `issues`, `tripwires`, `index`, `model`, `anchors`, `commits`,
`install-agent-skills`, `init`, `new-component`, `worklist`, `transcript` and `id`, and the code that parses and
dispatches them, are a module of this crate's library rather than of its binary. A binary that registers extensions
flattens the command enum into its own and adds the commands of its extensions beside them, so each command keeps one
spelling, one set of arguments and one exit-code contract in every binary. `klarch` is that module with no extension
registered. `init` creates the project the other commands locate, so every binary hands it to `cli::init` before it
locates one, as `klarch`'s main, the setup skill's maintenance crate and the crate documentation's example of an
extension binary do; `cli::run` refuses it inside a project, naming `cli::init`, so a binary that skips the step says
why instead of acting.

A binary crate cannot be depended on, so a command set held in `klarch`'s binary would be copied into every binary that
registers an extension, and the copies would drift. The installed text names a command with its arguments, substituting
only `{{command}}`, per `design@check@declared-command`, so an argument spelled otherwise in a project's binary would
make an installed instruction false there, where
`design@agent-skills@installed-text-works-with-what-an-installing-project-holds` has each installed instruction followed
with what the installing project holds, or need a substitution per argument. The helpers an extension's own commands need from a run, such as
assembling a complete working tree, are public in the same module for the same reason.

### A command of klarch is parsed in this crate, and its logic takes a domain's types `##command-parsing-home`

This crate is the nearest the binaries among those every binary offering these commands depends on, per
`design@agent-skills@command-parsed-nearest-the-binaries`, as `klarch` and an extension's own binary both depend on it,
per `design@agent-skills@top-level-binary-crate`, so each command keeps one spelling. Each command's logic is a typed function of the crate that holds it, with no clap
type in it, or is assembled here from those functions, so clap's major version stays out of those crates' interfaces and the decisions about the whole interface,
`design@cli@exit-code-ladder` among them, have one home. A combination of arguments the domain refuses is a type of the
domain that cannot hold it, as `design@check@fix-with-staged` is, and this crate's declaration refuses it while parsing.
`cli::run` refuses a value of an `*Args` struct that holds one, since those structs are public and a value of one can
reach it without clap, with exit 2 before any work, as `design@cli@a-refusal-by-arguments-writes-nothing` has an invocation refused, and a test holds
the declaration and the conversion each, so a drift between them fails one.

**A check that needs the value alone is made by the declaration, while parsing**, as
`design@agent-skills@top-level-binary-crate` has this crate apply the checks that can be made while parsing: a date an
argument takes, a regular expression an argument takes, the form of the name of a project a command creates. Whether a
value is valid in the project is the tool's to answer, since finding out may take the tool's own logic; so is a value
that names something in the project, or that the command is asked about, whatever its form. A rule the tool's own type
holds stays in the tool's crate: the declaration spells it, as a value parser that calls the rule does.

The
nearest rival is `rejected@cli@each-crate-parses-its-commands`. The cost accepted: each new command touches two crates. `gates` and `mutate` keep that shape, since
the binary that offers them is each project's own maintenance binary, which no shared crate sits above.

### Every command's arguments are declared through clap, and every binary tests its declaration `##arguments-parse-through-clap`

The commands are a clap `Subcommand` enum, `knowledge_architect::cli::Command`, and a binary that registers extensions
flattens it into its own derive, per `design@cli@the-commands-are-a-library-module`. So clap is part of this crate's
public interface, and a binary built on it parses through clap too.

These binaries run blind, as gates: the caller reads the exit code and not the output. An unknown
flag or an invalid combination that is ignored lets a gate pass on a check it did not run, so each
has to be a refusal rather than a no-op. A hand-written parser refuses only the combinations its author
thought of. With clap, an unknown argument is refused by default, and exclusivity is a
declaration (`conflicts_with`, `requires`, `ArgGroup`) that a test checks as a whole.

**Each binary tests its own declaration**, with clap's `Command::debug_assert`, and asserts that
an unknown argument is refused. A declaration is checked only by such a test: clap finds a
malformed one at the first invocation that reaches it, not at build time.

### Every command's exit code says whether it ran before it says what it found `##exit-code-ladder`

| code | meaning |
| ---- | ---------------------------------------------------------------------- |
| 0    | the command ran, and its subject is in order                           |
| 1    | the command ran, and reports a negative answer about its subject       |
| 2    | the command could not run: bad arguments, no project, unreadable input |

This binds every command of `cli::Command`, in every binary that offers it, per
`design@cli@the-commands-are-a-library-module`. A command that answers several candidates in one
run, as `id` does, answers one it cannot read as a candidate with 1, a negative answer about it,
since the run still answers the others. `path@cli@README.md` lists which
outcome of each command maps to which code.

**The third code is what makes the other two mean anything.** It is the code of an invocation that its arguments
refuse, per `design@cli@a-refusal-by-arguments-writes-nothing`. A project runs `check` and
`commits` as gates, in CI and in a session, and reads the exit code without reading the output.
Without code 2, a mistyped invocation or an unreadable input is indistinguishable from a tree
that fails its check, and a gate that treats both as a failure cannot say which one to repair.

clap supplies 2 for a parse failure, per `design@cli@arguments-parse-through-clap`, so
the boundary costs nothing at the parse. What the ladder costs is that a command routes its own
unreadable-input errors to 2 rather than to the code it uses for a negative answer.

### An invocation of a command of `cli::Command` that its arguments refuse writes nothing `##a-refusal-by-arguments-writes-nothing`

An invocation is refused by its arguments when it gives an argument the command does not declare, omits one the
command requires, gives a value the argument's declaration does not accept, or gives a combination the command does
not accept. The refusal comes before the command writes anything, with code 2 of `design@cli@exit-code-ladder`. This
binds every command of `cli::Command`, a later one included, in every binary that offers it, per
`design@cli@the-commands-are-a-library-module`. A command a project adds beside them is the project's, and so is what a
project's own program does with a refusal.

It serves `goal@documentation-stays-consistent`, by which conformance is established by running
the checker: a gate that reads an exit code as the verdict must be able to tell an invocation that did not run from a
check that passed. The caller of a refused invocation also knows that the tree is as it was, so it repairs the
invocation and runs it again over the same tree.

The nearest rival is a value or a combination checked where the command uses it, after the command began to write. It
leaves a caller that runs blind, per `design@cli@arguments-parse-through-clap`, with a changed tree behind a code that
says the command did not run. Refusing first costs nothing at the parse: clap resolves the whole command line before a
command is dispatched, and supplies code 2 for a parse failure. A combination the declaration cannot express is refused by `cli::run` before any work, per
`design@cli@command-parsing-home`.

Three cases are no refusal by arguments, and the rule says nothing of them:

- a read of the project before the refusal;
- a value the argument's declaration accepts and the command then refuses, or answers negatively, by the state of
  the tree or by what the value names, whatever its code;
- code 2 after a write that failed.

Tests in `path@testing@tests/binary.rs` show the rule on the commands they give it to, an argument a command does not
declare over every command the declaration holds. No test is built to detect every breach: the owner does not think a
mechanical test that detects any breach is worth trying everything for, and asked that the effort on it stay
reasonable.

### The binary is named `klarch` `##binary-name`

The package is `knowledge-architect`; the binary it installs is `klarch`, as the crate ripgrep
installs `rg`. The binary's name is typed at every run, so it is short; the package's name is typed
once, in a dependency or an install, so it is descriptive.

The design this decision did not keep: `rejected@cli@binary-named-after-package`.
