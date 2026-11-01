# Rejected designs — mutate

The designs the hand mutation did not keep, and why. Each entry is cited by the design heads whose
argument covers it. `skill@knowledge-architect-decision-recording` owns the entry shape, its `live`
and `void` markers, and the tests an entry passes.

### A forward `git apply --check` as proof that a mutation is gone `##forward-apply-check-proof`

`live`. Proposed as a convenience, in thaum where the tool was built: when a restore refuses and the
caller unwinds the mutation with the `git apply -R` the tool offers, the record can never be
discharged, so the tool would notice that the mutation had gone by applying its own patch forward.
Refuted by reproduction: **`git apply` places a hunk at any offset**, so a command that duplicated
the pre-mutation region satisfied the forward check while the mutant sat untouched at its site. The
tool reported the record cleared, deleted the pre-image, its only copy, and exited 0 with the
mutation live. The question is whether the mutant is at this position; a patch check answers
whether this text can be placed somewhere in this file.

### A worktree to hold the mutation, instead of mutating the working tree `##worktree-holds-the-mutation`

`live`. Proposed so that a mutation could never touch uncommitted work. Refuted by measurement, in
thaum: a worktree created for a run carries no `target/` directory, so every mutation check would
pay a full cold build of the workspace, on the mid-work case the tool exists for. Measured by
dispatching an agent into an isolated worktree and reading `git status`, `git rev-parse HEAD` and
`ls -d target` inside it. Build cost is the whole of the reason: that a worktree cannot see
uncommitted work is not, since a tool can copy the modified files in. A shape that removed the cost
reopens the entry: `tripwire@mutate@restore-by-comparison`.

### cargo-mutants instead of a tool that applies the caller's mutation `##cargo-mutants-instrument`

`live`. cargo-mutants generates its own mutants, by default replacing a function's body, so it
answers whether some mutant of a function survives, not whether the test written for a claim
catches the wrong implementation that claim names: a function with one tested branch and five
untested ones can read as covered, and a mutation the author wrote cannot be expressed at all. It is
also an external install that a project's build does not pin. Running it over a branch's changed
code, as a source of leads beside the hand mutation, is another question, which this entry does not
cover.

### Shipping thaum's sweep of invariant-check sites with the run and the restore `##sweep-shipped`

`live`. thaum's third mode deletes each invariant check of one directory of its engine in turn, and
its enumeration is tied to that directory and to the shape `return violation(` of a check there. A
generic sweep would need each project to declare its sites and its mutation, a recorded list that
no second project has asked for. The run and the restore are the generic part, which the crate
ships.
