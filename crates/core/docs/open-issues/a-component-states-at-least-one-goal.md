---
kind: deferred
---
# Nothing checks that a component states a goal

## Summary

The owner's rule is that every component states at least one goal. The checker accepts a goals
home with no entry, so a component with no goal passes.

## Details

### What

A check that reports a component whose goals home holds no entry.

### Why it matters

Goals are what drive and constrain design in the long term, so a component with none has nothing
its design can be judged against, per `design@agent-skills@goals-required`. Every component of
this repository states one, and 8 of the 12 components of thaum, the checker's first user, state
none (counted on thaum's main at e98e296). A project adopting the check may have to write goals first,
or need a way to adopt it gradually. A tree `klarch init` or `klarch new-component` writes holds
an empty goals home, per `design@core@init-command`, so the check would fail it until the goals are
stated with the owner: the setup skill's goal-setting step would then be part of reaching a passing
check.

### Trigger

The first release after thaum's 8 components without a goal have one: thaum is the first project
the check would bind, and a release is the occasion that ships checks. The migration of thaum onto
the published checker is where that state is next looked at.
