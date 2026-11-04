//! The parse of a document, as an extension reads it, per
//! `design@core@an-extension-builds-its-own-model`.
//!
//! The core's model holds nothing for an extension. An extension reads each [`Document`]'s
//! parse through these types and scans it again for its own subject.
//!
//! [`Document`]: crate::Document

pub use crate::model::document::{
    md, rs, Frontmatter, Literals, Located, Observation, Parsed, Prose, RetiredForm, Scope,
    ScopeKind, SlugSite,
};
