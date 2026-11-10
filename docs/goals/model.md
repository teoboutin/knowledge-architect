# knowledge-architect goals: The documentation model

The goals on the structure of a target project's documentation, on the model of it that tools read, and on what that documentation records and whom it reaches.

## A project's documentation has a defined structure, and a model of it that tools read `##documentation-has-a-structure-and-a-model`

knowledge-architect defines a structure for a target project's documentation, and a project
can declare kinds of entries of its own under it. Each recorded item
takes a name. A reference is written in a
readable form, the same in every project, in any text the checker reads, and it says what it
names, with one reading.

knowledge-architect provides a model of that documentation, which its tools read: what the project
declares, what its documents hold, the items they define and what each reference names. The model
reports no breach of a rule on what a document says. A tool of knowledge-architect can tell whether what it read is
complete and right,
and what is wrong where it is not.

## A project's documentation records its design, with the intent and the arguments behind it `##design-is-recorded-with-its-arguments`

In a target project, the documentation records what was decided and why. Each decision the project
records in an entry of its own carries its intent, its argument, and the alternatives that lost
with why they lost. The reason of any other decision, where its code or its agent-facing text does
not show it, is recorded beside that code or that text and in the message of its commit. What is
outstanding is recorded, and so is the evidence that would flip a recorded decision. A later
session can then tell what it may change, and what a change costs, without deriving the argument
again or reversing a decision blind.

## A recorded item reaches the work it bears on `##records-reach-their-reader`

In a target project, each recorded item keeps one home, and is cited at each
place where it bears on other work and where a project may write a reference the checker checks. A session working
at such a place meets the item, and a session changing the item can list what to read again.
