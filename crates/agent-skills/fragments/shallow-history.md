%% The decision: `design@agent-skills@tripwire-horizon`, whose occasions a command counts over the history. Seen
%% in a shallow cloud clone, where a standing-state reviewer's count missed a horizon that had passed.
%% Measured: in a clone of depth 3, `git log -S` on a `Watches until` line names the clone's boundary
%% commit, so the count starts there and comes out too low, with no error. The agent reports rather than
%% fetches: a fetch writes to the repository, which the tripwire auditor's read-only rule forbids, and needs
%% a reachable remote. A release tag missing locally is the same case: a standing-state reviewer ran
%% `git fetch --tags` in its worktree to count one. Per `git fetch --help` (git 2.53.0), `--tags` fetches
%% tags "in addition to whatever else would otherwise be fetched", so it also moves the remote-tracking
%% branches; per `git worktree --help`, every ref under refs/ but refs/bisect, refs/worktree and
%% refs/rewritten is shared between worktrees, so the write reaches the dispatcher's ranges.
%% `git ls-remote` lists the remote's tags and writes nothing, but cannot supply the tagged commits a
%% count needs in the local history, so the dispatcher fetches. Rivals: a sentence in the review
%% invariants misses the auditors the saved audit workflows dispatch; a sentence in the primer loads in
%% every session for an occasion few meet; a sentence in the standing-state reviewer alone misses the
%% tripwire auditor, which counts the same horizons.
In a shallow clone, where `git rev-parse --is-shallow-repository` prints `true`, a count over the
history starts from the clone's boundary commit and comes out too low, with no error. A count of
release tags sees only the tags the repository holds; `git ls-remote --tags origin`, which writes
nothing, lists the remote's. Report each horizon you could not count for either reason, and the
command that makes it countable in the dispatcher's checkout, `git fetch --unshallow` or
`git fetch --tags`. Run no fetch yourself: it writes refs that every worktree of the repository
shares, the remote-tracking branch of the main branch among them.
