# Goals — model

Every goal here serves one purpose: a tool of knowledge-architect reads what a project's documents hold and
what they name through one library, which judges none of what they say.

## Every tool of knowledge-architect that reads what a project's documents hold can read it through one library that judges none of what they say `##a-tool-reads-the-documents-with-no-check`

A tool here is any code of knowledge-architect that reads what a project declares and what its
documents hold, such as a command that shows or searches entries, a work list over them, a renderer
of the documents, or a checker of their content. Through the library such a tool reads what the
project declares, what its documents hold, the entries they define, and what each reference names or
why it names nothing. To judge what a document says is to report that it breaks a rule on its
content, such as a section an entry owes. It refines
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, and reaches past it to tools no session
task asks for.

It is met while the library's public interface gives each of those readings with no such report
attached and holds no function that makes one, and its dependencies hold no part of
knowledge-architect that makes one.

- Requires, for instance: a tool asks what a reference names, and gets the target or the cause of its
  failure.
- Admits, for instance: a tool that scans a document's parse again for a subject of its own, or that
  reads a file past the library.
- Rules out, for instance: the library depending on the checks of a document's content.

## A tool can tell whether what the library read of a project is right `##a-tool-knows-the-read-is-right`

What a project declares, or what its tree holds, can leave what the library read wrong or incomplete,
such as a declaration that cannot be honoured, a file that cannot be read, a home that is missing, or
one entry defined twice. An answer computed over it is unreliable. The library answers whether what it read is
right, and names what is wrong.

It is met while the library's public interface answers that question, and a test over a project
holding each of those four shows the answer naming it.

- Requires, for instance: the answer names a declared home that is missing.
- Admits, for instance: a tool that shows one entry without asking.
- Rules out, for instance: the answer naming a section an entry owes, or a reference to something
  missing, each of which leaves the other answers right.

## Each recorded item can be given a name, and referenced from any text the checker reads `##recorded-items-can-be-referenced`

Through the library, each recorded item, one of a kind a project declares included, takes a name, and a
reference written in any text the checker reads says what it names, with one reading, or why it names nothing. This is what lets an item be cited where it bears, with one home.

It is met while the library reads a name for each such item, and its public interface answers for each reference what it names, with one reading, or the cause of its failure.

It refines `goal@knowledge-architect@records-reach-their-reader`.

- Requires, for instance: a decision, a design not kept, an outstanding item and a goal can each be
  referenced from a code comment.
- Admits, for instance: any name a session chooses, within the grammar.
- Rules out, for instance: a reference with two readings, read as the first of them.
