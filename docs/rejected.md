# Rejected designs — knowledge-architect

The designs this project did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@knowledge-architect@design` whose argument covers it. An entry stays `live` while its
reason holds, and is marked `void` when it no longer does.

### The checker's crate in crates/knowledge-architect `##crate-directory-named-after-project`

`live`. Refuted by a run, not by argument: in a scratch project named knowledge-architect with a
component at that path, the check stopped in phase 1, because the directory's basename and the
project's name are the same anchor. A crate directory is named by its role.

### One published package holding both the checker and the installed text `##single-published-package`

`live`. The owner asked for a single package. cargo packages only the files under a package
directory, so a single package holding both components must be rooted at crates/. That makes crates/
a package holding two components, which mixes a crate with the directories around it, and a third
crate could not live under it. Users name one crate either way, because cargo fetches a dependency
without being asked.

### The agent workflow as a plugin published on a marketplace `##workflow-as-marketplace-plugin`

`live`. A Claude Code plugin is how the harness distributes skills, and it was designed in full: a
marketplace in this repository, the plugin pinned to each release's tag and sha, a hook printing the
primer and warning on a version mismatch. It loses on three facts. Two artifacts, the crate and the
plugin, had to be released in step, and a release had to edit the marketplace after tagging. The
harness's plugin cache is per user, so two projects on one machine needing two versions was never
shown to work. And a version mismatch could only be warned about, where an installed file can be
checked. A custom marketplace gave no discoverability in exchange.

### Tying the build to `CARGO_MANIFEST_DIR`, which every crate already reads `##build-tied-to-cargo-manifest-dir`

`live`. Refuted by a run on cargo 1.98.0: in two copies of a one-crate workspace sharing one
`CARGO_TARGET_DIR`, a binary printing `env!("CARGO_MANIFEST_DIR")` was not rebuilt in the second
copy and printed the first copy's directory. Cargo does not track a variable it sets itself.

### The crates' changelog copies written at packaging time and kept out of the tree `##changelog-copies-written-at-packaging`

`live`. Refuted by a run on cargo 1.98.0: a git-ignored copy that `include` lists makes
`cargo package --list` exit 101 as an uncommitted change, and an absent one is left out of the
package with no error.

### The gates library as the host of the hand mutation `##gates-library-hosts-the-mutation`

`live`. The gates library would have carried the mutation's run and restore, which a maintenance
crate already depends on. Every project that runs its gates through the library would then build
the mutation code, which no gate uses, and the library would hold a tool that judges nothing about
a branch's merge, against the purpose `design@gates@gates-crate` gives it.
