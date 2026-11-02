# Rejected designs — git

The designs the Component git did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@git@design` whose argument covers it. An entry stays `live` while its
reason holds, and is marked `void` when it no longer does.

### Every git variable of the caller's environment removed, `GIT_INDEX_FILE` included `##every-git-variable-dropped`

`live`. Git would then always find the repository and its index from the project's directory. It
lost to a measurement with git 2.53.0, a pre-commit hook printing `env | grep '^GIT_'`: under
`commit -a` and `commit <paths>`, `GIT_INDEX_FILE` names a lock file holding the tree the commit
records, while `.git/index` still holds what was staged before, so `check --staged` run from the
hook would judge another tree than the commit's.
