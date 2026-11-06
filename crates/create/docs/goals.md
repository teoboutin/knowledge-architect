# Goals

Every goal here serves one purpose: a project gets by one command the files the checker requires of it, ready to be completed, and loses nothing it wrote.

What the checker requires of a project is what the project declares to it, the documents and the homes of each Component, and the installed text. A file the project wrote is any file but one whose name the installed text of this version holds: such a file is installed text, even where it was edited by hand. A file the install would remove counts as one the project wrote, one an older version installed included, since nothing tells the two apart.

## A project, new or existing, adopts knowledge-architect by one command, which leaves every file the checker requires ready to be completed and loses nothing the project wrote `##a-project-adopts-by-one-command`

A person or an agent adopting knowledge-architect runs one command. Every file the checker requires then stands, ready to be completed, and the checker can read the project whole. A full pass of the checker is not asked of this command: what remains is work the checker names. A file the project wrote is neither overwritten nor removed, under whatever name it stands. Where the run would have to do either, it refuses before it writes. Where a document the project already holds is one the checker cannot read, keeping its bytes ranks first: the run keeps it and reports what the checker found. A run that fails partway for another reason may leave what it wrote, as long as it overwrote nothing.

It is met while tests show that a new project and an existing one each reach that state by one command, that a document already there keeps its bytes, and that a file the project wrote, which the install would remove, stops the run before it writes.

It refines `goal@knowledge-architect@every-target-project-can-adopt-it`.

- Requires, for instance: a project holding its own README keeps it; a project holding a file of its own that the install would remove is refused, before any write.
- Admits, for instance: a run that fails partway on a full disk, leaving what it wrote; a project with an extension declaring that extension's tables by hand; an installed file rewritten to the version that ships, even where it was edited by hand; a line added to the project's ignore rules; the refusal of a project that already declares itself.
- Rules out, for instance: removing a file the project wrote and listing that it did; overwriting a document so that the checker passes.

## A project gains a Component by one command, which leaves the files the checker requires of it ready to be completed and loses nothing the project wrote `##a-project-gains-a-component-by-one-command`

A person or an agent adding a Component to a project runs one command. Every file the checker requires of that Component then stands, ready to be completed. A file the project wrote is neither overwritten nor removed, under whatever name it stands. Where the run would have to do either, it refuses before it writes. A run that fails partway may leave what it wrote, as long as it overwrote nothing.

It is met while tests show that a project reaches that state by one command, that a document already there keeps its bytes, and that a file the project wrote, which the install would remove, stops the run before it writes.

- Requires, for instance: a README already in the Component's directory is kept; a project holding a file of its own that the install would remove is refused, before any write.
- Admits, for instance: the one entry the command adds to what the project declares, keeping every other byte.
- Rules out, for instance: overwriting a document so that the checker passes.
