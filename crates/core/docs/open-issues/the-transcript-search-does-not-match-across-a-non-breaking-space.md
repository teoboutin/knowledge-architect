---
kind: defect
---
# `transcript list --containing` and `show --from` do not match across a non-breaking space

## Summary

A message the owner typed can hold a non-breaking space, U+00A0. `cargo klarch transcript list
--containing "<words>"` and `transcript show --from "<words>"` match the text character for
character, so the message's words typed with ordinary spaces find no session, and the command
exits 1.

## Details

### What

Reproduction, met by the transcript reviewer of the intake of seventeen retrospective files of
2026-10-25 to 2026-10-31: the owner's first message of the session, "I want you to run the intake
on the pending retrospective files, ...", holds U+00A0 between "to" and "run", as the raw line of
the session's file shows. `cargo klarch transcript list --containing "I want you to run the intake
on the pending retrospective files"` lists no session and exits 1;
`--containing "run the intake on the pending"`, which leaves out that space, lists it.

The review skill finds a session with `--containing "<opening words>"`, so a dispatcher copying the
owner's words as it reads them meets an empty result. Ruled out: a wrong session or a `/clear`; the
same session is listed by `--branch`.

### Why it matters

A transcript reviewer or a dispatcher that gets no session takes the transcript as missing, and
the review's last axis runs on a guessed session or not at all, against
`goal@knowledge-architect@the-owner-s-word-is-verified`.

### What would close it

The matching of `--containing` and `--from` treats every Unicode space as a space, or normalises
both sides, with a test over a fixture line holding U+00A0.
