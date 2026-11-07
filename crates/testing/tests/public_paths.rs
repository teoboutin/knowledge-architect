//! Every public path of this crate's harness, per `design@knowledge-architect@api-facade`. A name that
//! leaves it fails this file's build, and the changelog's Migration entry promises each of them.

#![allow(unused_imports)]

use knowledge_architect_testing::{foundation, run_with, Phase, Report, Stop, Structure, CHECKS};

/// The claim: `Stop` and `Phase` are `model`'s types, so a consumer of either crate passes them to the other.
#[test]
fn a_type_the_model_crate_holds_is_one_type_across_the_crates() {
    fn stop(s: Stop) -> knowledge_architect_model::foundation::Stop {
        s
    }
    fn phase(p: Phase) -> knowledge_architect_model::foundation::Phase {
        p
    }
    let _ = (stop, phase);
}
