---
kind: defect
---
# The escape anchor silences a path that resolves relative to its own document, which the unanchored-path lint reports

## Summary

A backticked span shaped like a path, with no anchor, is reported when it names something the tree
holds, read from the root, from an anchor's directory, or from the directory of the document that
writes it. The escape anchor's honesty check, which refuses `path@elsewhere@<path>` for a target the
tree holds, reads the target from the anchors' directories only. So a file named relative to its
document is reported bare and accepted under the escape, which the check exists to stop.

## Details

### What

Established by reading the code and by a run, in the adversarial review of the branch of the intake
of thirteen retrospective files, over a copy of the mock project `path@core@tests/projects/minimal/`:

- `names_this_tree` in `path@core@src/check/references.rs` resolves a span against the root, against
  each anchor's directory, and against the document's own directory, `rel.parent()`.
- `path` in the same file, for the escape anchor, looks for an anchor whose directory joined to the
  target is present: `anchors.all().iter().find(|a| inputs.present.contains(&a.path.join(trimmed)))`.
  It never joins the document's own directory.
- The reproduction: in a design home at docs/design.md of a project that holds docs/plans/README.md,
  the backticked span `plans/README.md` is reported as shaped like a path and naming no anchor;
  rewritten as `path@elsewhere@plans/README.md`, as the finding's repair offers for a path this tree
  does not hold, the line gets no finding.

Not established: whether a session has silenced a real pointer that way. The repair line's wording,
"for a path this tree does not hold", invites it for a writer who reads the relative path as one the
root does not hold.

Ruled out: `issue@core@the-escape-finding-names-the-first-anchor-not-the-one-that-holds-the-target`
is about which anchor the escape's finding names, not about a target it misses.

### Why it matters

`design@core@reserved-anchors` makes the escape honest: a target that resolves here is a real
reference wearing the escape. A pointer silenced under it dangles unseen when its target moves,
against `goal@knowledge-architect@documentation-stays-consistent`.

### What would close it

The escape's honesty check also resolves the target against the document's own directory, as
`names_this_tree` does, with a test over a mock project in which a document names a file relative to
its own directory under the escape anchor and gets the finding.
