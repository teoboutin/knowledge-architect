# Goals

Every goal here serves one purpose: a project meets the tools of knowledge-architect it runs as one program, whose command line says plainly what it accepts and whether a command ran.

A tool here is something knowledge-architect publishes for a person or an agent working in a target project to run, other than the text it installs. The program is what the package a project names provides; a project may carry it under a command of its own maintenance tooling.

## Every tool of knowledge-architect is a command of one program, unless its result does not exist without running the project's build or its tests `##one-program-for-its-tools`

A project gets those tools by naming one package and pinning its version, whichever tools those are, a later one included. A tool whose result does not exist without running the project's build or its tests stays out of that program, and the project's maintenance tooling may offer it, whether it chooses what to run or its caller does. A command of that program picks neither itself; running a command its caller names on the invocation is no breach, where the command's result exists without it.

It is met while each other tool is a command of that program, and no command of it picks and runs the project's build or its tests, which a test of the program's commands shows of those it names.

- Requires, for instance: a new tool that searches the record is a command of that program; so is one that reads the agent harness's session transcripts.
- Admits, for instance: a tool also offered as a library function; a library with no command of its own that the program links; a project's own command, in a program of its own; a task only this repository's own tooling performs, which is published to nobody; a command rewriting history that runs a command its caller names at each commit.
- Rules out, for instance: a second program a project must install to consult its record; a command of that program that picks and runs the project's tests; a command that runs the project's tests because a setting names them, with no name on the invocation.

## That program's command line is declared in one place, apart from what its tools do `##one-home-for-the-command-line`

`cli` holds the declaration of every command of that program, and refuses an invalid invocation: arguments that are wrong whatever the project holds, which a check of the value alone can tell. Whether a value is valid in the project is an invalid request, which the tool answers, since finding out may take the tool's own logic; so is a value that names something in the project, or that the command is asked about, whatever its form. What a command does is mostly pulled from the crates of the tools, or assembled in `cli` from what they provide. So a tool can change what it does with no change to how it is invoked.

It is met while no crate that program links, `cli` apart, declares a command's arguments, and a check of any other value that needs only the value is made while parsing, which a look at the declarations shows.

- Requires, for instance: a new command's arguments are declared in `cli`; a date an argument takes is checked while parsing; so is a regular expression an argument takes, and the form of the name of a project a command creates.
- Admits, for instance: a tool outside that program keeping its own parsing; a rule the tool's own type holds, which the declaration spells; a command in `cli` that composes two crates' functions; a crate that program links using a parser privately.
- Rules out, for instance: a crate that program links deriving its own commands' arguments for `cli` to flatten.

## An invocation of a command `cli` offers that its arguments refuse on the command line writes nothing, and its exit code says the command did not run `##a-refused-invocation-does-no-work`

This holds for every command `cli` offers, whichever commands those are. A command a project adds beside them is the project's, and so is what a project's own program does with a refusal. An invocation is refused by its arguments when it gives an argument the command does not declare, omits one the command requires, gives a value the argument's declaration does not accept, or gives a combination the command does not accept.

The refusal comes before the command writes anything, with an exit code that says the command could not run, which differs from each code that says it ran. A run that reads the code as a verdict, as a gate does, then never takes such an invocation for a check that passed, and the caller knows this refusal wrote nothing.

It is met while each of those four kinds of refused invocation behaves so, which tests show on the commands they give it to, each seeing that exit code and nothing written.

- Requires, for instance: a new command `cli` offers, given an argument it does not declare, exits with that code and writes nothing.
- Admits, for instance: a read of the project before the refusal; a value the argument's declaration accepts and the command then refuses or answers negatively, by the state of the tree or by what the value names, whatever its code; a code of could-not-run after a write that failed, which is no refusal by arguments.
- Rules out, for instance: an unknown argument ignored, so that a gate passes on a check it did not run.
