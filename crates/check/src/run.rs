//! A run's phases and what the last one reports, for a consumer that drives a run itself.
//!
//! A caller configures its extensions with [`crate::extension::configure`] and gathers the inputs
//! with [`crate::commands::Gathered`]. It calls [`foundation`] for phases 1 to 3. Once that returns
//! `Ok` and each extension is prepared, it calls [`run_with`] for the last phase.
//! `knowledge-architect-testing` publishes these to an extension's tests.

pub use crate::check::{foundation, run_with, Phase, Report, Stop, Structure, CHECKS};
