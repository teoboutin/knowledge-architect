**Generated — do not edit.** `cargo klarch index`

13 entries

| kind | title |
| --- | --- |
| defect | [A mutation run whose test filter matches no test reads as `SURVIVED`](a-filter-that-matches-no-test-reads-as-survived.md) |
| defect | [A `--patch` run is refused where its patch removes a line opening with `-- ` or adds one opening with `++ `](a-patch-line-that-removes-or-adds-a-line-opening-with-two-signs-reads-as-a-file-header.md) |
| defect | [A `--patch` run on a file at the project's root named `m.patch`, or under a directory so named, is refused](a-patch-of-a-root-file-named-m-patch-is-refused.md) |
| defect | [A `--patch` run whose patch replaces the target by a symlink mutates the target to the content of the file the link points at](a-patch-that-makes-the-target-a-symlink-mutates-from-the-file-it-points-at.md) |
| defect | [The liveness of a run is read wrongly outside Linux](a-run-s-liveness-is-read-wrongly-outside-linux.md) |
| defect | [A test command that fails for its environment reads as `caught`](an-environment-failure-reads-as-caught.md) |
| defect | [The hand mutation refuses, misreads or mutates wrongly in several ways on Windows](the-hand-mutation-fails-in-several-ways-on-windows.md) |
| design | [The hand mutation runs one mutation per invocation, so a list of mutations is a loop each session writes](a-list-of-mutations-runs-as-a-loop-of-the-session.md) |
| observation | [A reused process id reads as a live run, and blocks `restore` and `--discard`](a-reused-pid-reads-as-a-live-run.md) |
| observation | [After a run, the target directory holds the last mutant's build until the next build](the-target-directory-holds-the-last-mutant-s-build-after-a-run.md) |
| todo | [A mutation spanning several files cannot be applied in one run](a-mutation-of-several-files.md) |
| todo | [`cargo x mutate run --replace` takes only a text that occurs once in the file](a-repeated-text-cannot-be-replaced-at-one-of-its-occurrences.md) |
| todo | [The hand mutation's run reads two states as what they are not, fixed on one branch](the-mutation-run-s-false-verdicts-on-one-branch.md) |
