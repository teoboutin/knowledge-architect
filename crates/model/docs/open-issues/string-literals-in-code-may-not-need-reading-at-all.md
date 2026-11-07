---
kind: design
---
# Reading string literals in code as prose may cost more than it finds

## Summary

The scan reads each unbound string literal of a Rust file as prose, and the checker's own source is
exempt by a list of directories compiled into the binary. The owner doubts that anyone would cite
an entry from a literal, and wants to discuss whether a literal in code should be linted or scanned
at all. This repository's tree holds no such citation outside the exempt directories, and the
record's case for reading them rests on a measurement of thaum, whose citations, on the owner's report,
name an external specification's rules rather than entries of a record.

## Details

### What

**The mechanism.** `path@model@src/source/rs.rs` treats a literal that a `let`, a `const` or a
`static` does not bind as prose, per `design@model@grammars-not-prefixes`. The lints then read it, as
`design@check@reference-shape-outside-span` says. So a test file outside the exempt directories
whose literal holds a rule-shaped text gets a finding.

**The instance that raised it.** Moving the test files of the checker to the shared test Component
`testing`, which `klarch` does not link, puts them outside the exempt directories. Copied to that
directory untracked and run with `cargo klarch check`:

| files copied | findings |
| --- | --- |
| `path@testing@tests/checks_over_mocks.rs` and `path@testing@tests/extension_api.rs`, reaching phase 4 | 17 |
| `path@testing@tests/model_over_mocks.rs` | 7 in phase 3, which leaves phase 4 unjudged |
| `path@testing@tests/binary.rs` | 26 in phase 3, which leaves phase 4 unjudged |

Each is a fixture the test expects, such as a span naming a mock project's anchor, and none is a
citation of this project's record. The mock projects themselves are not involved: `[walk] exclude`
keeps them out of the walk. On the owner's word to take the easiest shape to build for now, the
session excludes that Component's test files from the walk as well, which also leaves the references
in their comments unchecked.

**What the record holds for reading them.**

- `design@model@grammars-not-prefixes`: reading every literal as data lost 32 citations in thaum's
  tree, assertion messages citing a rule for a human to read on failure.
- `rejected@check@no-string-literal-read`: 17 assertion messages in thaum's `thaum-testing` crate
  cite a rule behind a marker. Both figures were measured in thaum, and neither was taken again for
  this entry.

**What those citations are, on the owner's report.** They cite the rules of the external specification thaum follows, which thaum implements through an extension, and a test that exercises a rule names it in a string literal. There the owner finds reading the literal sensible. For the references knowledge-architect itself allows, to entries of a project's record, the owner finds that it makes little sense. So the measurements above support a citation of an extension's own subject, and support nothing yet about a reference to the record. Whether the scan's reading can tell the two apart, or an extension's citation could be read by that extension alone, is for the discussion.

**What this repository holds.** Searching the Rust files of `gates`, `mutate`, `agent-skills` and
`xtask`, the published and maintenance code outside the exempt directories, for a backticked span of
a built-in kind on a line that is not a comment line finds none:

```sh
git grep -nE '`(design|rejected|issue|goal|tripwire|register|path|skill|agent|primer)@[^`]+`' -- 'crates/gates/*.rs' 'crates/mutate/*.rs' 'crates/agent-skills/src/*.rs' 'crates/agent-skills/build.rs' 'tools/xtask/*.rs' | grep -vE ':[0-9]+:\s*//'
```

It is a line search, so it misses a span split across a literal's lines. Whether thaum's citations
still stand is `not established`.

**What the reading costs**, each named by its record:

- the exemption and its compiled lists, `design@check@checker-source-literals-are-data`, which every
  new crate of the checker extends, with the designs it rejected;
- the binding rule's special cases in the extractor, and `rejected@model@every-literal-read-as-prose`;
- a finding on a fixture wherever an extension's or a tool's test expects rule-shaped text outside
  those lists.

**Suspected mechanism:** the rule was measured on an extension's citations of an external
specification, and has come to bind every reference the checker reads, at a cost to each project, this
one included, that writes no such literal.

### Why it matters

It strains `design@model@grammars-not-prefixes` and `design@check@checker-source-literals-are-data`,
and it would reverse `rejected@check@no-string-literal-read` if the discussion ends that way. A
citation written in a literal that nothing reads dangles silently when its entry is renamed, which
`goal@knowledge-architect@documentation-stays-consistent` counts against. Against that, each test
that expects a rule-shaped text outside the compiled lists owes a binding, an exclusion from the walk
or a further exempt directory, for a class of citation this repository does not hold.

### What would close it

A design discussion that keeps or changes the rule, with the measurement above taken again over this
repository and, where it can be had, over thaum: its heads rewritten or confirmed against that
measurement, and the entry deleted. The exclusion of the test files of `testing` from the walk is
reconsidered in the same discussion, since a change of the rule removes its reason.

### Re-entry

A design discussion the owner opens on it.
