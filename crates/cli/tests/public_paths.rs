//! Every public path of the library a consumer names, per `design@knowledge-architect@api-facade`.
//!
//! A path that leaves the facade fails this file's build. `path@testing@tests/extension_api.rs` drives
//! what an extension uses, and this holds the whole list, the items it never touches included.

#![allow(unused_imports)]

use knowledge_architect::cli::{
    complete_working_tree, init, locate, output::write, refuse_a_foreign_build,
    refuse_another_version, run, this_library, CheckArgs, Command, CommitsArgs, Gathered, IdArgs,
    IndexArgs, InitArgs, IssuesArgs, Library, NewComponentArgs, ShowArgs, TranscriptArgs,
    TranscriptListArgs, TranscriptSearchArgs, TranscriptShowArgs, TranscriptVerb, TripwiresArgs,
    Vcs, WorklistArgs,
};
use knowledge_architect::document::{
    md, rs, Frontmatter, Literals, Located, Observation, Parsed, Prose, Scope, ScopeKind, SlugSite,
};
use knowledge_architect::extension::{
    configure, label, normalise_list, normalise_one, rel_from, DumpRow, Entry, EntryKind,
    Extension, ExtensionReport, Generated, Inputs, Outside, Prepared, Purpose, Resolution,
    Revision, Snapshot, Tree,
};
use knowledge_architect::{
    component_dir, Document, Finding, Manifest, Model, CHECKER_VERSION, MANIFEST_NAME,
};

/// The claim: the types `model` holds are one type across the two crates, so a consumer of either
/// passes them to the other.
#[test]
fn a_type_the_model_crate_holds_is_one_type_across_the_crates() {
    fn manifest(m: Manifest) -> knowledge_architect_model::Manifest {
        m
    }
    fn outside(o: Outside) -> knowledge_architect_model::listing::Outside {
        o
    }
    let _ = (manifest, outside);
}
