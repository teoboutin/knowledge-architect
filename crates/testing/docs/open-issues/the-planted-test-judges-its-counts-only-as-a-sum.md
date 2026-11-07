---
kind: todo
---
# The test over `planted` judges each check's count only as part of a sum

## Summary

`every_check_has_a_row_and_the_run_is_their_sum` holds the sum of the counts in `PLANTED` and, per row, only that a fragment hits between 1 and the row's count. Swapping two rows' counts passes.

## Details

### What

In `path@testing@tests/checks_over_mocks.rs`, `PLANTED` holds 31 for `references` and 11 for `registers`. An adversarial reviewer changed them to 11 and 31 with `cargo x mutate`, and `cargo test -p knowledge-architect-testing` passed: the fragments `is referenced` and `issue register` hit 2 of 31 and 3 of 11, and the test accepts anything in `1..=n`. The structure was the same before the tests moved, read from the diff and not measured. The findings carry no name of the check that raised them, so the test cannot count by check from a finding alone.

### Why it matters

The doc comment of `PLANTED` says "how many findings it produces". A check that gains a finding while another loses one keeps the sum, and passes. It does not strain a recorded decision.

### What would close it

`PLANTED` holds a fragment per finding, or the run's report names each finding's check, so that each row's count is compared exactly, and the swap above fails.
