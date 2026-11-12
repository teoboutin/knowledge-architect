//! What a run fetches before any check runs, gathered once for every caller.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::check::Inputs;
use crate::model::table::Anchors;
use crate::model::{foundation, Manifest, Model};

/// Everything a check needs that it may not fetch for itself, read from the working tree.
///
/// A check is a pure function over the model, so the files and the git answers it reads are
/// fetched here by the caller, per `design@check@model-then-checks`. `check` builds its
/// [`Inputs`] from this, a writer asks [`complete_working_tree`](super::complete_working_tree)
/// for one, and an extension's tests build theirs the same way, so no caller assembles
/// [`Inputs`] field by field.
pub struct Gathered {
    committed: HashMap<PathBuf, String>,
    configs: HashMap<PathBuf, String>,
    configs_not_text: HashSet<PathBuf>,
    generated_unreadable: HashSet<PathBuf>,
    shipped: Vec<(PathBuf, String)>,
    tree: foundation::Gathered,
}

impl Gathered {
    /// Read the working tree under the manifest's root: the committed generated files, every
    /// `register.toml`, the agent files this version ships, and what `model` gathers for phases 1
    /// to 3 and the resolver.
    ///
    /// `Err` is could-not-run: git or the filesystem refused a read the run cannot do without.
    /// A generated file or a `register.toml` that is absent is not an error: the checks that
    /// read them report the absence.
    pub fn over(manifest: &Manifest, model: &Model) -> Result<Gathered, String> {
        let tree = foundation::Gathered::over(manifest, model)?;
        Ok(Gathered::beside(tree, manifest, model))
    }

    /// What phase 4 alone reads, beside what `model` gathered. These reads cannot fail.
    pub(super) fn beside(
        tree: foundation::Gathered,
        manifest: &Manifest,
        model: &Model,
    ) -> Gathered {
        // The generated files are outside the walk — an extension's by a declared row, every
        // file-register index by construction — because a generated file is not a source of
        // citations. The set includes the extensions' files once `extension::configure` has
        // run over the manifest.
        let mut committed = HashMap::new();
        let mut generated_unreadable = HashSet::new();
        for rel in crate::model::generated::generated_paths(manifest, model.listing()) {
            // Git lists a file the working tree no longer holds until its deletion is staged, so
            // the listing cannot tell an absent file from one that would not read: the read
            // tells them apart, and an absent one is left out of both.
            match std::fs::read_to_string(manifest.root().join(&rel)) {
                Ok(text) => {
                    committed.insert(rel, text);
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => {
                    generated_unreadable.insert(rel);
                }
            }
        }
        // A register instance's options sit beside it and are not markdown, so the walk never
        // reads them.
        let anchors = Anchors::of(manifest, model.listing());
        let mut configs = HashMap::new();
        let mut configs_not_text = HashSet::new();
        for (_, _, home) in anchors.instances() {
            // A file that cannot be read at all is the tree's finding, in phase 2; one whose
            // bytes are not UTF-8 reads there as text, so it is kept here as such.
            if let Ok(bytes) = std::fs::read(manifest.root().join(&home.config)) {
                match String::from_utf8(bytes) {
                    Ok(text) => {
                        configs.insert(home.config.clone(), text);
                    }
                    Err(_) => {
                        configs_not_text.insert(home.config.clone());
                    }
                }
            }
        }
        Gathered {
            committed,
            configs,
            configs_not_text,
            generated_unreadable,
            shipped: crate::agents::shipped(manifest),
            tree,
        }
    }

    /// The inputs of one run, borrowed from what was gathered.
    pub fn inputs(&self) -> Inputs<'_> {
        let tree = self.tree.inputs();
        Inputs {
            installed: self.tree.installed(),
            shipped: &self.shipped,
            committed: &self.committed,
            configs: &self.configs,
            configs_not_text: &self.configs_not_text,
            generated_unreadable: &self.generated_unreadable,
            present: tree.present,
            directories: tree.directories,
            outside: tree.outside,
            ignored: tree.ignored,
            tracked_and_ignored: tree.tracked_and_ignored,
            refused: tree.refused,
            links: tree.links,
            judged: crate::check::Judged::WorkingTree,
        }
    }
}
