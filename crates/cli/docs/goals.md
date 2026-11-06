# Goals

Every goal here serves one purpose: whoever runs a command `cli` offers can tell a command that did not run from one that ran.

## An invocation of a command `cli` offers that its arguments refuse on the command line writes nothing, and its exit code says the command did not run `##a-refused-invocation-does-no-work`

This holds for every command `cli` offers, whichever commands those are. A command a project adds beside them is the project's, and so is what a project's own program does with a refusal. An invocation is refused by its arguments when it gives an argument the command does not declare, omits one the command requires, gives a value the argument's declaration does not accept, or gives a combination the command does not accept.

The refusal comes before the command writes anything, with an exit code that says the command could not run, which differs from each code that says it ran. A run that reads the code as a verdict, as a gate does, then never takes such an invocation for a check that passed, and the caller knows this refusal wrote nothing.

It is met while each of those four kinds of refused invocation behaves so, which tests show on the commands they give it to, each seeing that exit code and nothing written.

- Requires, for instance: a new command `cli` offers, given an argument it does not declare, exits with that code and writes nothing.
- Admits, for instance: a read of the project before the refusal; a value the argument's declaration accepts and the command then refuses or answers negatively, by the state of the tree or by what the value names, whatever its code; a code of could-not-run after a write that failed, which is no refusal by arguments.
- Rules out, for instance: an unknown argument ignored, so that a gate passes on a check it did not run.
