# Rejected designs

The designs the command line did not keep, and why.

### The binary named knowledge-architect, after its package `##binary-named-after-package`

`live`. The binary's name is the default of the declared command, which a project types at every
run, and the owner wanted it short to type. Renaming the binary is a change to the command-line
surface every installing project scripts against, so the alternative will be raised again whenever
the name is questioned.

### Each domain crate parsing its own commands, flattened by this crate `##each-crate-parses-its-commands`

`live`. Each crate would derive clap's types for what it provides, so clap's major version would be part of each one's
interface, the decisions about the whole interface would be restated in each, and a frontend that is no command line
would meet a clap struct. It would change an interface others consume. It stays the shape of `gates` and `mutate`, whose
commands no shared crate sits above.
