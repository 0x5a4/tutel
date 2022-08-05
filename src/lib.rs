#![allow(dead_code)]
#![warn(clippy::perf)]
#![warn(clippy::nursery)]
#![warn(clippy::style)]

mod data;
mod de;
mod ser;

use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

pub use data::{Project, Task};

pub const PROJECT_FILE_NAME: &str = ".tutel.toml";

/// Creates a new empty Project in the given directory
pub fn new_project(name: String) -> Result<Project> {
    let dir = std::env::current_dir()?;
    let path = dir.join(PROJECT_FILE_NAME);
    let mut project = Project::new(path, 0, name);

    project.save()?;

    Ok(project)
}

/// Walks the path upwards until a project file(.tutel.toml) is found and loads it
pub fn load_project_rec(path: &Path) -> Result<Project> {
    for (steps, p) in path.ancestors().enumerate() {
        if let Some(project_file) = has_project(p) {
            return Project::load(project_file, steps);
        }
    }

    bail!("no project found");
}

/// Determines whether a project exists in the given path by checking
/// for the existence of .tutel.project. Returns Some(project_path)
/// if it does exist, None otherwise
pub fn has_project(path: &Path) -> Option<PathBuf> {
    let project = path.join(PROJECT_FILE_NAME);

    if project.exists() && project.is_file() {
        Some(project)
    } else {
        None
    }
}
