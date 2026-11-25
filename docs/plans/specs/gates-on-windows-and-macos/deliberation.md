# The threads and arguments of the spec on the gates on Windows and macOS

## Threads

### CI runs the project's whole gates list on Linux, Windows and macOS, with the one command `##gates-on-three-systems`

Proposed by the owner in round 1, as build and test on Windows and macOS with other gates left out,
and reshaped by the session in the reply to round 4 on the owner's purpose of round 2. Final state:
approved. Arguments: `argument@gates-on-windows-and-macos@arg-1`,
`argument@gates-on-windows-and-macos@arg-2`, `argument@gates-on-windows-and-macos@arg-3`,
`argument@gates-on-windows-and-macos@arg-4`, `argument@gates-on-windows-and-macos@arg-8`,
`argument@gates-on-windows-and-macos@arg-9`, `argument@gates-on-windows-and-macos@arg-10`,
`argument@gates-on-windows-and-macos@arg-13`, `argument@gates-on-windows-and-macos@arg-14`. The
session's first shape, `cargo test` run directly, was withdrawn by its proposer. The owner's ruling:
round 5, agreed, on the full gates; round 8, the checkpoint table. Shape: README.md, Decided design,
#gates-on-three-systems. Harvest: the rewritten `design@xtask@gates-list-primary-home`.

### A job that gathers the matrix keeps the one status context the ruleset requires `##one-required-context`

Proposed by the session in the first reply to round 7. Final state: argued. Arguments:
`argument@gates-on-windows-and-macos@arg-22`. Closed on its argument: one shape meets the blocking
criterion with no setting changed; the owner confirmed it with the checkpoint table in round 8.
Shape: README.md, Decided design, #one-required-context. Harvest: a head of the root.

### The jobs of the three systems block a ready pull request `##blocking-jobs`

Proposed by the session in the reply to round 1, as one of two shapes, and the owner in round 2.
Final state: approved. Arguments: `argument@gates-on-windows-and-macos@arg-1`,
`argument@gates-on-windows-and-macos@arg-2`, `argument@gates-on-windows-and-macos@arg-7`. The
owner's rulings: round 2, the lean to blocking; round 5, agreed; round 7, the jobs land first,
blocking. Shape: README.md, Decided design, #blocking-jobs. Harvest: the rewritten
`design@xtask@gates-list-primary-home`.

### The maintenance crate runs in the release profile `##release-maintenance-build`

Proposed by the session in the reply to round 4. Final state: approved. Arguments:
`argument@gates-on-windows-and-macos@arg-3`, `argument@gates-on-windows-and-macos@arg-13`,
`argument@gates-on-windows-and-macos@arg-15`, `argument@gates-on-windows-and-macos@arg-21`. The
owner's ruling: round 5, the release build everywhere. Shape: README.md, Decided design,
#release-maintenance-build. Harvest: `design@agent-skills@xtask-pins-checker`.

### The gates library offers clippy for another target as a gate a project opts in to `##cross-clippy-gate`

Proposed by the owner in round 4, shaped by the session in the replies to rounds 4 and 5. Final
state: approved. Arguments: `argument@gates-on-windows-and-macos@arg-10`,
`argument@gates-on-windows-and-macos@arg-11`, `argument@gates-on-windows-and-macos@arg-12`,
`argument@gates-on-windows-and-macos@arg-17`, `argument@gates-on-windows-and-macos@arg-18`,
`argument@gates-on-windows-and-macos@arg-19`, `argument@gates-on-windows-and-macos@arg-30`. The
owner's rulings: round 5, inclined, asking for its costs; round 6, taken in and offered opt-in. The
optional return came after the search and was confirmed with the checkpoint table in round 8. Shape:
README.md, Decided design, #cross-clippy-gate. Harvest: a head of `register@gates@design`.

### This repository checks out every file as committed `##line-endings-of-this-repository`

Proposed by the session in the reply to round 1. Final state: argued. Arguments:
`argument@gates-on-windows-and-macos@arg-5`. Closed on its argument; the owner confirmed it with the
checkpoint table in round 8. Shape: README.md, Decided design, #line-endings-of-this-repository.
Harvest: a comment, or a head of the root.

### A generated file that differs by line endings alone is current, and the build gives one text whatever the checkout's `##generated-text-compared-without-line-endings`

Proposed by the session in the reply to round 1. Final state: argued. Arguments:
`argument@gates-on-windows-and-macos@arg-5`, `argument@gates-on-windows-and-macos@arg-28`. The shape
under which the writing commands also leave such a file was ruled out after the search. The owner
confirmed it with the checkpoint table in round 8. Shape: README.md, Decided design,
#generated-text-compared-without-line-endings. Harvest: `design@check@generated-files-are-pure` and
`design@model@a-file-register-index-is-rows`.

### Outside Unix a path is named to git with slashes `##paths-to-git-with-slashes`

Proposed by the session in the reply to round 1. Final state: argued. Arguments:
`argument@gates-on-windows-and-macos@arg-6`. The owner's ruling: round 5, agreed with the product
repairs; confirmed with the checkpoint table in round 8. Shape: README.md, Decided design,
#paths-to-git-with-slashes. Harvest: `design@git@a-path-is-bytes`.

### A project-relative path is printed with slashes on every system `##printed-paths-with-slashes`

Proposed by the session in the reply to round 1. Final state: argued. Arguments:
`argument@gates-on-windows-and-macos@arg-6`. Agreed with the product repairs in round 5; confirmed
with the checkpoint table in round 8. Shape: README.md, Decided design, #printed-paths-with-slashes.
Harvest: a head of `register@model@design`.

### The printing function is in `model`, and `git` keeps a private spelling of its own `##path-spelling-home`

Proposed by the session, after the code-split scout, before the search. Final state: argued.
Arguments: `argument@gates-on-windows-and-macos@arg-26`. The draft's public function in `git` was
ruled out after the search. Confirmed with the checkpoint table in round 8. Shape: README.md,
Decided design, #path-spelling-home. Harvest: a head of `register@model@design`.

### The symbolic-link helper is a copy in each package's test code, and fails where links are refused `##symlink-helper-home`

Proposed by the session, after the code-split scout. Final state: argued. Arguments:
`argument@gates-on-windows-and-macos@arg-23`, `argument@gates-on-windows-and-macos@arg-32`.
Confirmed with the checkpoint table in round 8. Shape: README.md, Decided design,
#symlink-helper-home. Harvest: a comment at each copy.

### No module is split inside this work `##no-module-split-in-the-port`

Proposed by the session, after the code-split scout. Final state: argued. Arguments:
`argument@gates-on-windows-and-macos@arg-24`, `argument@gates-on-windows-and-macos@arg-25`.
Confirmed with the checkpoint table in round 8. Shape: README.md, Decided design,
#no-module-split-in-the-port. Harvest: no head; instances in the standing issues.

### The hand mutation's tests pass on macOS, and on Windows are repaired where cheap and otherwise skipped naming an issue `##mutate-tests-on-windows`

Proposed by the session in the reply to round 1, within the scope the owner set in rounds 2 and 5.
Final state: argued. Arguments: `argument@gates-on-windows-and-macos@arg-16`,
`argument@gates-on-windows-and-macos@arg-35`. The owner's ruling: round 5, that a skip on Windows is
for development workflow; confirmed with the checkpoint table in round 8. Shape: README.md, Decided
design, #mutate-tests-on-windows. Harvest: comments.

### A test is skipped on Windows for development workflow, naming its issue, or for a behaviour lacked by nature, with its reason `##platform-skips`

Proposed by the owner in round 5, and the session in the reply to round 7 for the second case. Final
state: approved. Arguments: `argument@gates-on-windows-and-macos@arg-8`,
`argument@gates-on-windows-and-macos@arg-16`, `argument@gates-on-windows-and-macos@arg-32`,
`argument@gates-on-windows-and-macos@arg-33`, `argument@gates-on-windows-and-macos@arg-36`. The
owner's rulings: round 5, skips for development workflow only; round 8, the three kinds lacked by
nature admitted, with a reason and no issue. Shape: README.md, Decided design, #platform-skips.
Harvest: a head of the root.

### The build and the gates run on three systems, the goals exclude no platform, and development on Windows is outstanding work `##platforms-served`

Proposed by the session in the reply to round 7, on the search's finding. Final state: argued.
Arguments: `argument@gates-on-windows-and-macos@arg-8`,
`argument@gates-on-windows-and-macos@arg-16`, `argument@gates-on-windows-and-macos@arg-31`. The
owner's statements: round 2, development on Windows left out of this work; round 5, wanted later,
its failures opened as issues. Confirmed with the checkpoint table in round 8. Shape: README.md,
Decided design, #platforms-served. Harvest: a head of the root.

### The discussion on what becomes an ask is put off again `##asks-discussion-put-off`

Proposed by the session in the reply to round 7. Final state: approved. Arguments:
`argument@gates-on-windows-and-macos@arg-34`. The owner's ruling: round 8, put off. Shape:
README.md, Decided design, #asks-discussion-put-off. Harvest: the issue's instance; no head.

### One branch and one spec, the CI jobs first `##single-branch-spec`

Proposed by the owner in round 7. Final state: approved. Arguments:
`argument@gates-on-windows-and-macos@arg-20`. The owner's ruling of round 7, against the session's
milestone of the reply to round 1. Shape: README.md, Decided design, #single-branch-spec. Harvest:
leaves with the spec.

### The refusal of a backslash in a reference or a manifest row leaves the work and becomes an issue `##backslash-in-a-pointer`

Proposed by the session in the reply to round 4, agreed by the owner in round 5 with the product
repairs; its withdrawal by the session in the reply to round 7, as a finding against that agreement.
Final state: withdrawn. Arguments: `argument@gates-on-windows-and-macos@arg-27`. The withdrawal's
default was put to the owner at the top of the reply to round 7; the owner contested nothing in
round 8. Shape: README.md, "Losing alternatives", #backslash-in-a-pointer. Harvest: an issue of
`model`.

### A push trigger on the branch lets CI run while the pull request is a draft `##iteration-before-ready`

Proposed by the session in the reply to round 1. Final state: superseded. Arguments:
`argument@gates-on-windows-and-macos@arg-20`, `argument@gates-on-windows-and-macos@arg-29`. Absorbed
by the probe branch, which the owner allowed to be pushed in round 2; the owner confirmed the
checkpoint table in round 8. Shape: README.md, "Losing alternatives", #iteration-before-ready.
Harvest: `design@knowledge-architect@ci-skips-a-draft`.

## Arguments

### The Windows runners may be weaker, and the jobs must not slow a merge, so gates whose verdict does not depend on the system may be left out there `##arg-1`

Given by the owner, in round 1. Bears on `thread@gates-on-windows-and-macos@gates-on-three-systems`,
`thread@gates-on-windows-and-macos@blocking-jobs`. The owner took the runners to be less powerful,
and named the checks over documents and past commits as ones that may not be needed on the other
systems; building and testing is the point.

### GitHub gives a public repository the same machine for Linux and Windows `##arg-2`

Given by the session, in the reply to round 1. Bears on
`thread@gates-on-windows-and-macos@blocking-jobs`,
`thread@gates-on-windows-and-macos@gates-on-three-systems`. Read in GitHub's documentation: 4 CPUs
and 16 GB for Linux and Windows, 3 CPUs and 7 GB for macOS, free. The last Linux job took 5:04, of
which the tests 171 s. Starting a process costs more on Windows; only a run tells.

### `cargo test` replaces the running binary of `cargo x` `##arg-3`

Given by the session, in the reply to round 1. Bears on
`thread@gates-on-windows-and-macos@release-maintenance-build`,
`thread@gates-on-windows-and-macos@gates-on-three-systems`. "confirmed on Linux: the file is swapped
each time", with two inodes that alternate; Windows refuses to replace a running program. So the
session first proposed `cargo test` run directly on the two systems.

### A gate's command repeated in the workflow drifts from the list `##arg-4`

Given by the session, in the reply to round 1. Bears on
`thread@gates-on-windows-and-macos@gates-on-three-systems`. `design@xtask@gates-list-primary-home`
states it. The session proposed a test that compares the two.

### A checkout under Git for Windows' default fails the check `##arg-5`

Given by the session, in the reply to round 1. Bears on
`thread@gates-on-windows-and-macos@generated-text-compared-without-line-endings`,
`thread@gates-on-windows-and-macos@line-endings-of-this-repository`. Reproduced on Linux: 16
generated indexes out of date and 22 installed files that differ here, 3 indexes in a new project.
"`.gitattributes` would hide problem 2 from our own CI, so that fix also needs a test that clones
with `autocrlf=true`".

### A printed path mixes separators on Windows, in suggested repairs too `##arg-6`

Given by the session, in the reply to round 1. Bears on
`thread@gates-on-windows-and-macos@printed-paths-with-slashes`,
`thread@gates-on-windows-and-macos@paths-to-git-with-slashes`. Simulated over a patched standard
library: "printing `/` everywhere took the separator failures from 98 to 7".

### An absolute time is no criterion, since the jobs run in parallel and the suite grows `##arg-7`

Given by the owner, in round 2. Bears on `thread@gates-on-windows-and-macos@blocking-jobs`. The
owner's answer to the session's proposal of an 8-minute limit, with the lean to jobs that block.

### The purpose is that a consumer builds on Windows and tests itself there with its own gates, with no logic duplicated `##arg-8`

Given by the owner, in round 2. Bears on `thread@gates-on-windows-and-macos@gates-on-three-systems`,
`thread@gates-on-windows-and-macos@platforms-served`,
`thread@gates-on-windows-and-macos@platform-skips`. The gates library unifies what blocks a merge,
so the systems and the environments deviate as little as possible; development on Windows is not the
purpose now, and the owner considered retracting the first ask of round 1.

### A choice of gates in a workflow file is the duplicated logic `##arg-9`

Given by the session, in the reply to round 2. Bears on
`thread@gates-on-windows-and-macos@gates-on-three-systems`. "Any choice of gates written in a
workflow file is the duplicated logic you want to avoid, and the jobs run in parallel anyway."

### The two warnings exist on Windows alone, where code under `cfg(unix)` vanishes `##arg-10`

Given by the session, in the reply to round 3. Bears on
`thread@gates-on-windows-and-macos@cross-clippy-gate`,
`thread@gates-on-windows-and-macos@gates-on-three-systems`. Given in answer to the owner's question
of round 3, whether Linux clippy passes: it does, and clippy judges code for the system it runs on.

### Clippy for Windows can run on Linux, opt-in for consumers, and report before CI runs on a ready pull request `##arg-11`

Given by the owner, in round 4. Bears on `thread@gates-on-windows-and-macos@cross-clippy-gate`. The
owner's proposal.

### With the stub, clippy for a Windows target runs on Linux in seconds and reports what Linux clippy cannot `##arg-12`

Given by the session, in the reply to round 4. Bears on
`thread@gates-on-windows-and-macos@cross-clippy-gate`. Measured: 4.1 s cold, 1.5 s after an edit; at
main the compile errors of `git`, on the probe the two warnings; `-msvc` fails; `sccache` refuses
the stub. It would have caught none of the 118 test failures.

### The probe: `check` and `commits` pass on Windows, the swap is refused there, the release profile passes that point, and a scaffold builds and checks `##arg-13`

Given by the session, in the reply to round 4. Bears on
`thread@gates-on-windows-and-macos@gates-on-three-systems`,
`thread@gates-on-windows-and-macos@release-maintenance-build`. Before the results the session stated
what each outcome would decide. 118 tests fail on Windows and 5 on macOS.

### The gates other than the tests cost about 70 s cold on Windows, in parallel `##arg-14`

Given by the session, in the reply to round 4. Bears on
`thread@gates-on-windows-and-macos@gates-on-three-systems`. 70 s on Windows, 71 s on Linux, 44 s on
macOS.

### The release build fixes the swap everywhere, and the swap was a nuisance on Linux too `##arg-15`

Given by the owner, in round 5. Bears on
`thread@gates-on-windows-and-macos@release-maintenance-build`. The owner agreed with the proposal,
naming this reason.

### A test is skipped on Windows only for development workflow, and what does not work there becomes issues, since the whole workflow is to be followable there later `##arg-16`

Given by the owner, in round 5. Bears on `thread@gates-on-windows-and-macos@platform-skips`,
`thread@gates-on-windows-and-macos@mutate-tests-on-windows`,
`thread@gates-on-windows-and-macos@platforms-served`. The owner keeps it out of this work since it
needs design, such as what the harness does on Windows, and probably needs developing from Claude
Code there.

### The costs of the gate, measured `##arg-17`

Given by the session, in the reply to round 5. Bears on
`thread@gates-on-windows-and-macos@cross-clippy-gate`. 27 MB downloaded, 133 MB per toolchain, 487
MB per target directory here; 13 of 16 crates that build C pass with the stub, 1 without; two
consumer projects do not build on Windows since this project's own library does not compile there.

### Kept in this repository alone, each consumer finds the stub and the `sccache` refusal again `##arg-18`

Given by the session, in the reply to round 5. Bears on
`thread@gates-on-windows-and-macos@cross-clippy-gate`. The rival to offering it, stated with the
maintenance it brings: the stub rests on behaviour of the `cc` crate that nothing documents.

### A consumer the gate starts failing for opts out and loses little `##arg-19`

Given by the owner, in round 6. Bears on `thread@gates-on-windows-and-macos@cross-clippy-gate`. It
only catches some failures of Windows slightly earlier. The owner took the gate in, opt-in.

### One branch: the CI jobs, blocking, then the port until they pass `##arg-20`

Given by the owner, in round 7. Bears on `thread@gates-on-windows-and-macos@single-branch-spec`,
`thread@gates-on-windows-and-macos@iteration-before-ready`. The owner's ruling, against the
session's milestone.

### A project of the scaffold's shape runs its whole gates on the three systems, under the alias in each profile `##arg-21`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@release-maintenance-build`. The second run of the probe, made
since in that shape `check` and `commits` run the running binary itself. The dev alias passed there
too, so the swap does not reach every consumer.

### A gathering job named `check` needs no change of the ruleset, and must run when a job fails `##arg-22`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@one-required-context`. "GitHub counts a skipped job as passing."
The session had told the owner that the ruleset must change.

### Six packages need the helper and no crate can hold it `##arg-23`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@symlink-helper-home`. From the code-split scout: no crate may
depend on `testing`, making a link is no operation in git's terms, and `mutate` depends on no crate
of the workspace.

### Nearly every edit replaces one call, so the work pushes no split signal it does not find `##arg-24`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@no-module-split-in-the-port`. A move of a module past one read
would sit beside edits across every part of it. The gate goes in a module of its own.

### An issue for an unmade split is for code a work does not write `##arg-25`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@no-module-split-in-the-port`. The record search:
`design@agent-skills@split-not-needed-is-an-issue`, so the issue the draft opened for three modules
is dropped.

### Who calls a function is no ground for a place in `git` `##arg-26`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@path-spelling-home`. The record search:
`design@git@an-operation-in-git-s-terms-is-the-library-s`, against the draft's one public function
in `git`.

### The refusal of a backslash is not needed, removes a capability and splits the branch `##arg-27`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@backslash-in-a-pointer`. A document that passes on Linux holds
none that resolves; a `skip-files` row for a file whose name holds `\` would be refused; a stricter
check of past commits has the spec merge first.

### The wider shape reverses three more heads to avoid a harmless rewrite `##arg-28`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@generated-text-compared-without-line-endings`. The record search,
on `design@check@index-takes-no-dry-run`, `design@check@index-staged-write` and
`design@agent-skills@overlay-by-separate-skills`.

### A push trigger contradicts the head on drafts and satisfies the merge predicate `##arg-29`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@iteration-before-ready`. The record search: the trigger's fields
read the pull request, which a push has none of, and a successful run of it is a successful run on
the head.

### A gate absent on one system is a difference that no flag carries, so the head states it `##arg-30`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@cross-clippy-gate`. The record search, on
`design@xtask@gates-list-primary-home`; the function returns a gate or nothing.

### Closing the issue deletes the only text that argues the reading of the goals on platforms `##arg-31`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@platforms-served`. Found by the record search;
`issue@check@a-linked-manifest-splits-the-readers` cites it.

### A skip where links are refused would be a rule copied six times `##arg-32`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@symlink-helper-home`,
`thread@gates-on-windows-and-macos@platform-skips`. So the helper fails there, on a machine the work
does not serve.

### A test of what Windows lacks by nature fits neither the criterion nor an issue `##arg-33`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@platform-skips`. Mode bits, a named pipe, a refused name: they
cannot run there, and no issue could close.

### The gate is opt-in and asks nothing, so the discussion on asks can wait `##arg-34`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@asks-discussion-put-off`. Its re-entry point is met, since the
gate judges a project's quality.

### A patch run under `core.autocrlf` gave a false `SURVIVED` `##arg-35`

Given by the session, in the reply to round 7. Bears on
`thread@gates-on-windows-and-macos@mutate-tests-on-windows`. Measured in the simulation on Linux:
the scratch repository's `git apply` rewrote the file's line endings.

### Running such a test on Windows brings nothing `##arg-36`

Given by the owner, in round 8. Bears on `thread@gates-on-windows-and-macos@platform-skips`. The
owner's reason for admitting the skip, with its reason written and no issue.
