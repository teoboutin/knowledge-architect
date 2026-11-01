---
kind: todo
---
# Installed instructions assume pull requests and continuous integration

## Summary

`goal@knowledge-architect@agents-get-a-complete-workflow` requires that no installed instruction
outside an offered shape require or assume continuous integration or a pull request, and
`goal@knowledge-architect@setup-brings-quality-tools` takes CI out of the recommended tools. The
installed text still assumes both in several places.

## Details

### What

- The installed primer's coined terms define a spec as "the plan document of work done in one branch
  and one pull request", and the planning skill's terms as work done "in one branch and one PR", a
  slice the same way. The decision is `design@agent-skills@document-vocabulary`, and
  `design@agent-skills@plan-landing-is-not-tied-to-its-work` argues in pull requests too: both are
  read again with the installed text.
- The design skill speaks of "several pull requests" for a milestone.
- The setup skill recommends continuous integration "on every pull request that is ready", and its
  gates section says the continuous integration runs the same command.

A search of the installed text for "pull request", "PR", "CI" and "continuous integration", across
line breaks, since wrapped prose splits "pull request" over two lines, lists every site; the review
skill's paragraph on a project that squashes a branch holds one more. These are the sites found
while the goals were reviewed, not a full census (assumption).

### Why it matters

A target project that runs no pull request and no CI, as thaum and rootpoint merge locally, reads
instructions that do not apply to it; the goal is unmet while they stand.

### What would close it

Each such instruction is rewritten so that it assumes neither, or moved into an offered shape that a
project takes or leaves as a whole; the search above returns no site outside an offered shape.
