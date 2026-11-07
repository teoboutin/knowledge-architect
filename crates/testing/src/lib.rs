//! What an extension's tests need to run the checker over a mock project, as the binary runs it.
//!
//! A project that writes an extension adds this crate to its dev-dependencies, beside its dependency
//! on `knowledge-architect` and at the same exact version: another version brings a second `check`,
//! whose types are not those of `knowledge-architect`. A project with no extension never names it, and no crate of
//! knowledge-architect depends on it.
//!
//! # Testing an extension
//!
//! A test configures its extensions, gathers the inputs, runs phases 1 to 3 with [`foundation`], and,
//! once that returns `Ok` and each extension is prepared, the last phase with [`run_with`]:
//!
//! ```no_run
//! # use knowledge_architect::extension::Extension;
//! # fn the_extension() -> Box<dyn Extension> { unimplemented!() }
//! use knowledge_architect::extension::{configure, Purpose, Tree};
//! use knowledge_architect_testing::{foundation, run_with};
//! use knowledge_architect::{cli, Manifest, Model};
//!
//! let mut manifest = Manifest::load("tests/projects/a-mock".as_ref()).unwrap();
//! let mut extensions = vec![the_extension()];
//! configure(&mut manifest, &mut extensions);
//! let model = Model::build(&manifest, &[]).unwrap();
//! let gathered = cli::Gathered::over(&manifest, &model).unwrap();
//! let inputs = gathered.inputs();
//! foundation(&model, &manifest, &inputs).expect("phases 1 to 3 find nothing");
//! let prepared = extensions[0]
//!     .prepare(&manifest, &model, Tree::Checkout(manifest.root()), Purpose::Check)
//!     .unwrap();
//! let report = run_with(&model, &manifest, &inputs, &[(extensions[0].checks(), prepared.as_ref())]);
//! assert!(report.findings.is_empty());
//! ```
//!
//! A mock project is a directory inside the repository with its own `knowledge-architect.toml`.
//! git's listing is what the walk reads, so a mock's files are walked as soon as they exist, and
//! an ignore rule must not cover them. The project's own manifest excludes the mock's directory
//! with a `[walk] exclude` row, or the mock's planted defects are reported as the project's own.
//!
//! **A mock plants only the defects whose findings the extension emits.** A defect `check`
//! reports pins `check`'s behaviour in the extension's suite, and a release of `check` that
//! narrows or widens that check fails the suite with nothing to repair in the extension.
//!
//! **A test pins the set of findings it expects, not their count.** A count that moves says that
//! something changed and not what. Compare the set of `(file, line)` pairs of
//! `report.findings` with an expected set written in the test, or in a reference file the test
//! reads: a failure then names each finding gained or lost. The wording of a finding is left out
//! of the set, since a rewording is no change to what the check reports.
//!
//! A test reads `report.checks`, `report.not_run` and `report.summaries` rather than the printed
//! lines they produce. `report.checks` lists every check the last phase performed or was asked
//! to, `report.not_run` those the tree gave no input to, so a check ran when it is in the first and
//! not in the second, and `report.summaries` holds each extension's block of the summary.
//!
//! # Compatibility
//!
//! Under the crate's versioning policy, a breaking change to this API is major. While the
//! version is 0.x, a major change bumps 0.MINOR, so a break never ships in a patch.

// A `pub` item no consumer can reach is refused, per `design@knowledge-architect@api-facade`.
#![warn(unreachable_pub)]

// Read so that cargo rebuilds this crate when the checkout building it changes, per
// `design@knowledge-architect@a-build-is-tied-to-its-checkout`. A build outside this repository's
// cargo configuration, such as one from crates.io, sees it unset, and nothing changes.
const _: Option<&str> = option_env!("KNOWLEDGE_ARCHITECT_CHECKOUT");

pub use knowledge_architect_check::run::{
    foundation, run_with, Phase, Report, Stop, Structure, CHECKS,
};
