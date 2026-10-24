---
kind: design
---
# A handled retrospective file is marked where it lies, and the owner keeps handled files in an archive apart from pending ones

## Summary

`skill@knowledge-architect-retrospective-intake@after-the-ruling` marks each received project file
it carried out with a line at its head, and leaves it in the directory the owner keeps it in. The
owner keeps pending and handled files in two directories, and judges moving a handled file to an
archive better than deleting it, with the exact shape left to a design discussion.

## Details

### What

- The installed intake: "Then mark each received project file the analysis carried out: a line at
  its head saying that it was analysed and every outcome carried out … A file the owner keeps is
  then not handed to an intake again."
- This repository's own intake, `skill@klarch-retrospective-intake@received-files-after`, deleted
  the received files of this repository's sessions until the owner ruled, at the intake of
  2026-10-24, that the deletion instruction leaves, the archiving method being better and its exact
  shape needing a design discussion.
- The owner's own practice on this machine: received files wait in a pending directory, and a
  handled one moves to an archive directory, never deleted, so a later intake can read the other
  side of a file it receives.

The suspected mechanism, in one sentence: a mark keeps a handled file in the directory an intake
reads, so the owner's directory is the only thing that tells a session which files are handed to it.

Re-entry point: the design discussion this entry asks for, on the owner's word, or the next change
to the intake's section on the life of an analysis.

### Why it matters

A handled file left among pending ones can be handed to an intake again, and a deleted one loses
the other side of an interaction finding, against `goal@knowledge-architect@agents-get-a-complete-workflow`.
It strains `design@agent-skills@received-file-marked`, under which a marked file stays where the
owner keeps it.

### What would close it

A discussion under `skill@knowledge-architect-design` that settles what becomes of a handled file,
a mark, a move to an archive the retrospective names beside its pending directory, or both, in the
installed intake and in this repository's own.
