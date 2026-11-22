---
kind: observation
---
# The lints on ids may cost as much as their bound of 10%

## Summary

`design@check@id-lints-cost-bound` reopens the scan of the lints on ids when `klarch check` over
this repository takes more than 10% longer than the same commit built with the lints' arms and the
scanner's reading for them removed. Three takes of that measurement on one machine gave 8.8% to
10.6%, so the bound sits inside the instrument's spread. Whether the bound is crossed is not
established.

## Details

### What

The measurement, as the head states it: `klarch check` over this repository, release build, median
of five runs on one machine, against a baseline built with these removed:

- in `judge_part`, `path@check@src/check/references.rs`, the arm on `Observation::BareName`,
  `Observation::PlainWord` and `Observation::UnquotedReference`;
- in `scan`, `path@model@src/scan.rs`, the call to `outside_backticks`, its result replaced by
  `Vec::new()`.

The patch, against the tree of the commit "[review] Two touched
heads split of the decisions they bundled: the prose form of a thread's slug goes beside
its instruction, and the cost bound of the lints on ids takes a head of its own":

```diff
--- a/crates/check/src/check/references.rs
+++ b/crates/check/src/check/references.rs
@@ judge_part
-                Observation::BareName(_)
-                | Observation::PlainWord(_)
-                | Observation::UnquotedReference { .. }
-                    if id_lints =>
-                {
-                    out.extend(id_lint(&doc.rel, l.line, &l.what, &tables));
-                }
--- a/crates/model/src/scan.rs
+++ b/crates/model/src/scan.rs
@@ scan
-            let quotes = outside_backticks(
-                region,
-                at,
-                line,
-                fenced.contains(&n),
-                &mut paragraph,
-                |what| out.push(Located { line: n, what }),
-                |token| (n, token),
-                name_line == Some(n),
-            );
+            let quotes: Vec<(usize, usize)> = Vec::new();
```

Each binary was built with `cargo build --release -p knowledge-architect` in its own checkout and
run there, since a build is refused outside its checkout. The command, in each checkout:
`hyperfine -N -i -w 2 -r 5 'target/release/klarch check'`, the baseline's run also passing with no
findings. Conditions: an AMD Ryzen 9 9950X, Linux, 2026-11-22. Medians, with the lints against the
baseline:

| take | runs | with the lints | baseline | ratio |
| --- | --- | --- | --- | --- |
| 1 | 9 | 264.1 ms | 241.5 ms | 1.094 |
| 2 | 5 | 266.1 ms | 240.6 ms | 1.106 |
| 3 | 5 | 263.7 ms | 242.4 ms | 1.088 |

Within a take the standard deviation was 1.5 ms to 3.0 ms, at most about 1.2% of a run.

The commit that built the lints, "[core] A defined id written outside its reference form is a
finding, every id has two segments, and the spec's design audit", gives in its message 474 ms
against 431 ms for the same baseline, a ratio of 1.10, then says that the guards which pass over a
line holding no quotation mark, no `@` or no hyphen brought the run to 445 ms, a ratio of 1.03.
That commit's code holds those guards, so its 474 ms is read here as taken before them, and the
ratio as grown since that commit, through later changes to the lints that are not traced
(assumption). The same message says: "The baseline of 431 ms also lacks the scan of
backticked bare names, which the bare-name lint already ran." If that scan predates these lints,
the baseline is faster than the checker was before them, and the ratio overstates their cost by
that scan's share (assumption; the share is not measured).

It does not reproduce one way: take 2 crosses 10% and takes 1 and 3 do not. What would make it so:
more takes, alternating the two binaries, until the median's spread clears 10% one way.

### Why it matters

It bears on `design@check@id-lints-cost-bound`: a ratio above 10% reopens the scan of the lints on
ids, `design@check@reference-shape-outside-span`, `design@check@backticked-bare-id` and
`design@check@plain-word-lint`. A bound that the instrument's spread straddles cannot tell whether
the head fires.

### What would close it

The head's measurement re-taken until its result is clear of 10%: below it, the entry is deleted;
above it, the scan is reopened as the head says. A baseline that keeps the scan of backticked bare
names may move the ratio below the bound; it would change what the head measures against, so it
goes through the head.
