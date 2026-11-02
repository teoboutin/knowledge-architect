**Generated — do not edit.** `cargo klarch index`

6 entries

| kind | title |
| --- | --- |
| defect | [A requested path that the snapshot does not hold and whose name holds a newline shifts every blob after it to the wrong path](a-missing-path-holding-a-newline-shifts-the-blobs-after-it.md) |
| defect | [Three public functions of the git library decode a path, so a path that is not plain UTF-8 or ASCII loses its answer](paths-are-decoded-outside-the-listings.md) |
| defect | [`rev_parse` answers `None` for every failure of git, though its contract says only a missing revision does](rev-parse-answers-none-for-every-git-failure.md) |
| observation | [A path or a revision a caller hands the git library can reach git as an option, and a directory reads as a blob](arguments-reach-git-unchecked.md) |
| observation | [`index --staged` clears the assume-unchanged bit of the entry it stages, and maybe its skip-worktree bit](index-staged-clears-an-entrys-flags.md) |
| todo | [Tools of knowledge-architect run git past the wrapper, whose public interface lacks what they need](tools-run-git-past-the-wrapper.md) |
