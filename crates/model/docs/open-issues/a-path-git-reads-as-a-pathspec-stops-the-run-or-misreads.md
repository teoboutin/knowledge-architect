---
kind: defect
---
# A path reference that git reads as a pathspec it refuses stops the whole run, and one it reads as the repository's top is misjudged

## Summary

`ignore_queries` asks `git check-ignore` about the path of every anchored span and every relative markdown link that
`refused` accepts, and git reads each as a pathspec. A path through a symlink, into a submodule, or opening with pathspec
magic makes git fail the whole batch, and the run exits 2 naming no file and no line. A path opening with `:/` or `:(top)` is
answered against the repository's top, so a missing target can be read as ignored and pass.

## Details

### What

`ignore_queries` in `path@model@src/resolver.rs` pushes, for each span whose path `refused` accepts, the anchor's path joined to
the span's path, and for each relative link in a README or an index, the link resolved beside its file. `refused` refuses a
leading slash, an upward segment and a `.` segment, and nothing else.

Reproduced with git 2.53.0, each over a copy of the `core` mock of `path@testing@tests/projects/` outside any repository, pinned
to the binary's version, with one line appended to its README, then `git init` and `git add -A`, and `klarch check`:

- a path through a symlink: a directory `real` and a symlink `lnk` to it, and a path reference at the anchor `core-mock` with the
  path `lnk/x.md`, or a relative link to `lnk/x.md`: ``fatal: pathspec 'lnk/x.md' is beyond a symbolic link``, exit 2;
- a path into a submodule: a gitlink `sub`, declared under `[walk] exclude`, and a path reference at `core-mock` with the path
  `sub/x.md`: ``fatal: Pathspec 'sub/x.md' is in submodule 'sub'``, exit 2. Without the `exclude` row, the same span hides the
  phase-2 finding on the submodule, which the run without the span reports, exit 1;
- a path opening with pathspec magic, at the root anchor or at `*`, such as `:!x.md`, `:^x.md`, `:(glob)*.md`, `:(icase)x.md`,
  `:(literal)x.md`, `:(attr:foo)x.md` or `:(exclude)x.md`: ``fatal: :!x.md: pathspec magic not supported by this command``,
  exit 2. `git --literal-pathspecs check-ignore` refuses it too, ``pathspec magic not supported by this command: 'literal'``;
- a path `:/gen.md` or `:(top)gen.md` at `core-mock`, with a `.gitignore` holding `/gen.md` and no such file: `PASSED: no
  findings`, exit 0, where `zz/gen.md` under the same rule, and `:/gen.md` with no rule, each give the finding that the target does
  not exist, exit 1. git echoes the spelling back as ignored, judged against the repository's top.

Each failure stops every command that reads the ignore answers: `index`, `index --staged`, `check --fix` and `check --staged`
exit 2, and `commits` reports that the commit's tree does not load under the tip checker. A
commit message's span asks git in a batch of its own, which fails its commit alone, the message
then judged with no ignore answer.

**git answers about the working tree, so a span correct in its own commit is refused too**, and
`commits` fails the gate on it: the refusal of a message's batch is a finding, which counts in the
verdict and exits 1. Reproduced through the binary by
`a_span_correct_in_its_commit_and_refused_through_the_tip_s_link_names_no_reword` in
`path@testing@tests/binary.rs`, over the test's `tiny` project: a first commit adds
`link/data.txt` and its message holds a path reference at the anchor `tiny` with the path
`link/data.txt`, which its tree holds; a second
replaces the directory `link` by a symbolic link to `docs` and adds `skip-files = ["link"]`. `check`
at the tip passes, exit 0; `commits` over the two commits fails the first with ``fatal: pathspec
'link/data.txt' is beyond a symbolic link``, exit 1. An uncommitted symbolic link in the working
tree does the same, as `a_refused_message_ignore_batch_fails_a_commit_that_has_nothing_else_wrong`
in the same file shows. A commit's own tree is asked against the working tree the same way, so a
document span that the commit's tree holds as a file is refused when the working tree holds a link
there: the same two commits over a copy of the `dirhome` mock of `path@testing@tests/projects/`,
the span written in its README.md by the first and removed by the second, fail the first with
`this commit's tree does not load under the tip checker`, exit 1, by hand with git 2.53.0. The
finding's repair, `IGNORE_REPAIR` in `path@check@src/commands/history.rs`, names that
case apart: the span is correct, so it names running again from a working tree that holds the path
as the judged tree does and reporting the case where the tip must hold the link, and no reword. No
repair inside the project makes such a branch pass but a reword of a correct message.

Ruled out: the three shapes `refused` refuses. The span arm and the link arm of `ignore_queries` ask git nothing about them, and
`a_refused_path_shape_is_one_finding_and_the_run_goes_on` in `path@testing@tests/binary.rs` shows a leading slash and an upward
segment, at a named anchor and at `*`, as one finding each.

Not established: which finding each of these spans should get, since the walk keeps a symlink and a submodule out, and whether a
path the walk lists can take one of these shapes.

### Why it matters

It strains `design@check@finding-names-the-repair` and `design@check@phases-gate-the-report`: one prose span takes every check
from a project, with a message that names no file, and the `:/` case passes a reference to nothing.
`issue@check@a-submodule-is-a-project-of-its-own` names the `exclude` row as the declared silence, which such a span breaks.
`model`'s `Gathered::over` fails the same way for any tool over the documents. Under `commits`, a
branch whose tip replaces a directory by a symbolic link fails the gate on every earlier commit
whose message names a path under that directory, though each message is correct in its commit.

### What would close it

`ignore_queries` asks git about no path git reads as other than the literal path, through a symlink or into a submodule, and a
test through the binary shows the finding each such span and link gets.
