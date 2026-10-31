%% The decision: `design@agent-skills@tripwire-horizon`, whose occasions a command counts over the history. Seen
%% in a shallow cloud clone, where a standing-state reviewer's count missed a horizon that had passed.
%% Measured: in a clone of depth 3, `git log -S` on a `Watches until` line names the clone's boundary
%% commit, so the count starts there and comes out too low, with no error. The agent reports rather than
%% fetches: a fetch writes to the repository, which the tripwire auditor's read-only rule forbids, and needs
%% a reachable remote. Rivals: a sentence in the review invariants misses the auditors the saved audit
%% workflows dispatch; a sentence in the primer loads in every session for an occasion few meet.
In a shallow clone, where `git rev-parse --is-shallow-repository` prints `true`, a count over the
history starts from the clone's boundary commit and comes out too low, with no error: report each
horizon you could not count for that reason, and that `git fetch --unshallow` in the dispatcher's
checkout makes it countable.
