# Goals

Every goal here serves one purpose: an extension of a target project, and the crates of knowledge-architect's program, can be tested over a whole project through what a consumer can use.

An extension is a set of checks a target project writes against the public library interfaces of knowledge-architect. A consumer is a person or a program that uses a published package of knowledge-architect from outside the knowledge-architect repository. A finding is what a check reports about a project. The crates of knowledge-architect's program are the crates its command line links.

## A target project tests its extension over a project of its own with a package it adds to its tests and to nothing else `##an-extension-is-tested-over-a-project`

A target project that writes an extension runs the checks over a project of its own in its tests, the extension included, as its own program would run them, and reads each finding. It does so with a package of this Component, which it adds to its tests and to nothing else. No crate of knowledge-architect depends on this Component, a dev-dependency included.

It is met while the published library of this Component runs the checks over a project with an extension included; a test of this Component does that through published interfaces alone and gets the findings a program including the same extension gets; and no crate of knowledge-architect names this Component among its dependencies of any kind.

It refines `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.

- Requires, for instance: an extension's test reads each finding the checks made over its project.
- Admits, for instance: a helper of the library that spares an extension's tests a step they could write themselves; an extension's test that writes the steps itself; an extension's test that also uses another published package of knowledge-architect.
- Rules out, for instance: a target project having to depend on this Component to run the checks, outside its tests; the crate of the command line dev-depending on this Component.

## A test of the crates of knowledge-architect's program that needs the shared means of running over a project sits in this Component, with the projects it runs over `##program-tests-over-whole-projects-have-one-home`

A test of the crates of knowledge-architect's program that needs what such tests share to run over a project, such as building the program or copying a project into a repository of its own, sits in this Component with every project it runs over, whatever that project's storage. So no crate of the program holds a copy of those projects, reaches into another crate's, or keeps its own copy of the means to run over one. A test of one crate's code that needs none of that stays in that crate, with whatever project it writes or keeps for itself.

It is met while the tests of this kind sit under this Component's tests, with the projects they run over and the means they share, no test of another Component reads one of those projects, and no other crate holds a copy of them or of those means.

- Requires, for instance: a test of the command line over a project sits here; a test of a tool over the record that reads a project in isolation with that shared means sits here; a test that builds what the program writes for a new project sits here.
- Admits, for instance: a unit test of the crate that reads a project, over a small project kept in that crate's own tree and read with nothing of that shared means; a test of one crate that writes its own project into a scratch directory and needs nothing shared.
- Rules out, for instance: a crate's test loading a project kept in another crate.

The gates library and the hand mutation are outside it: their crates need nothing of knowledge-architect's program, and their tests stay with them.

## A test of this Component uses only what a consumer can, and what it needs and cannot observe there, where a consumer could see its effect, is published `##a-consumer-test-uses-what-a-consumer-can`

A test of this Component of an effect a consumer could see reaches knowledge-architect through what a consumer can use. What it needs and cannot observe there is hidden from consumers too, and what is published is changed so that the test observes it. A claim whose effect no consumer could see is outside this goal.

It is met while each such test is compiled apart from the code it tests, so that the build refuses a private item, and no item is published only so that such a test can reach it.

- Requires, for instance: a finding a consumer sees that no public item returns is made public, or its test is rewritten to reach it as a consumer does.
- Admits, for instance: the published interface growing because such a test showed a need; a public item that only such a test uses at first, when the effect it shows is one a consumer could see.
- Rules out, for instance: a hidden item, a feature or a module published only so that a test here can reach it.
