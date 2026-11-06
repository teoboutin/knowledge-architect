//! `anchors`: every anchor of the project, with its kind, its path and each register's home.

use std::path::Path;
use std::process::ExitCode;

use crate::model::table::Anchors;
use crate::output::outln;
use crate::Manifest;

/// Every anchor of the project, as a run builds them, with its kind, its path, and for a
/// declared anchor or `plans` the home of each register it carries, as the tree holds it, per
/// `design@query@anchors-listing-command`.
///
/// The output is for a reader, not a contract, per
/// `issue@knowledge-architect@command-output-is-not-declared-a-contract`. Exit 1 when a printed
/// home is missing, a negative answer about the project, per `design@cli@exit-code-ladder`;
/// `check` names the repair.
pub fn anchors(manifest: &Manifest, checker: &[&Path]) -> Result<ExitCode, String> {
    use crate::model::declared::Shape;
    use crate::model::table::Constructed;
    let model = crate::Model::build(manifest, checker).map_err(|e| e.to_string())?;
    let anchors = Anchors::of(manifest, model.listing());
    // The listing names files, so a directory is held where a listed file sits under it, as
    // `show` reads a register's home.
    let listed = |p: &Path| model.listing().iter().any(|l| l == p);
    let holds_dir = |p: &Path| model.listing().iter().any(|l| l.starts_with(p) && l != p);
    let mut missing = false;
    for anchor in anchors.all() {
        // An entry is named by the Directory register it is an entry of: `milestone`, or the
        // register a project declares.
        let kind = match (&anchor.constructed, &anchor.entry_of) {
            (Some(Constructed::Plans), _) => "plans",
            (Some(Constructed::Entry), Some(of)) => of.register.as_str(),
            (Some(Constructed::Entry), None) => "entry",
            (Some(Constructed::Spec), _) => "spec",
            (None, _) if anchor.is_component => "component",
            (None, _) => "location",
        };
        let path = if anchor.path.as_os_str().is_empty() {
            ".".to_string()
        } else {
            anchor.path.display().to_string()
        };
        outln!("{}  {kind}  {path}", anchor.name);
        // A plan's items have their homes in the plan's own documents: nothing to locate. Any
        // other entry lists what it carries, each home at the entry's own directory.
        if anchor.is_plan() {
            continue;
        }
        for name in &anchor.registers {
            let Some(register) = anchors.registers().by_name(name) else {
                continue;
            };
            let home = anchor.home_of(register);
            let mut held = Vec::new();
            if register.shape == Shape::Heading && listed(&home.file) {
                held.push(home.file.display().to_string());
            }
            // A home is held where `check` finds it, `heading_home` of phase 2 and its kin: a
            // register's directory, of any shape, is its home with or without its README, whose
            // absence `check` reports apart.
            let dir_held = match register.shape {
                Shape::Heading | Shape::File | Shape::Directory => holds_dir(&home.dir),
                Shape::Section => continue,
            };
            if dir_held {
                held.push(format!("{}/", home.dir.display()));
            }
            if held.is_empty() {
                missing = true;
                held.push("missing".to_string());
            }
            outln!("  {}  {}", register.name, held.join("  "));
        }
    }
    Ok(if missing {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
