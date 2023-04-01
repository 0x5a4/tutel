// #![allow(dead_code)]
#![warn(clippy::perf)]
#![warn(clippy::nursery)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]

mod project;
mod ser;
mod de;

use std::{fs, path::Path};

use anyhow::{anyhow, Result};

pub use project::{Project, Task};

pub const PROJECT_FILE_NAME: &str = ".tutel.toml";
pub const CHILDREN_DEPTH_LIMIT: usize = 5;

/// Creates a new empty Project in the current directory
///
/// # Errors
/// Throws an error if the current directory could not be determined
pub fn new_project(name: String, is_child: bool) -> Result<Project> {
    let dir = std::env::current_dir()?;
    let path = dir.join(PROJECT_FILE_NAME);

    Ok(Project::new(path, 0, name, is_child))
}

pub fn load_project_rec(path: &Path) -> Result<Project> {
    let mut root = None;
    let mut dir = None;
    for (steps, p) in path.ancestors().enumerate() {
        if has_project(p) {
            let Ok(project) = Project::load(p.join(PROJECT_FILE_NAME), -(steps as isize)) else {
                continue;
            };

            if !project.is_child() {
                root = Some(project);
                dir = Some(p);
                break;
            }
        }
    }

    let mut root = root.ok_or_else(|| anyhow!("no project found"))?;
    let dir = dir.ok_or_else(|| anyhow!("no project found"))?.to_owned();
    let steps = root.steps();

    load_project_rec_impl(&dir, &mut root, CHILDREN_DEPTH_LIMIT, steps);

    Ok(root)
}

fn load_project_rec_impl(path: &Path, parent: &mut Project, limit: usize, steps: isize) {
    if limit == 0 {
        return;
    }

    let Ok(iter) = fs::read_dir(path) else {
        return;
    };

    for child_path in iter {
        let Ok(child_path) = child_path else {
            continue;
        };

        let project_file = child_path.path().join(PROJECT_FILE_NAME);

        if let Ok(mut child) = Project::load(project_file, steps + 1) {
            if !child.is_child() {
                continue;
            }

            load_project_rec_impl(
                child_path.path().as_path(),
                &mut child,
                limit - 1,
                steps + 1,
            );

            parent.attach_child(child);
        } else {
            load_project_rec_impl(child_path.path().as_path(), parent, limit - 1, steps + 1);
        }
    }
}

/// Determines whether a project exists in the given path by checking
/// for the existence of .tutel.project. Returns `Some(project_path)`
/// if it does exist, None otherwise
#[must_use]
pub fn has_project(path: &Path) -> bool {
    path.join(PROJECT_FILE_NAME).is_file()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use std::fs;

    use crate::{load_project_rec, Project, PROJECT_FILE_NAME};

    const ROOT_CONTENT: &str = r#"
        name = 'root'
        tasks = []
        "#;

    const LEVEL1_CONTENT: &str = r#"
        name = 'l1'
        tasks = []
        is_child = true
        "#;

    const LEVEL2_2_CONTENT: &str = r#"
        name = 'l2_2'
        tasks = []
        is_child = true
        "#;

    const LEVEL3_CONTENT: &str = r#"
        name = 'l3'
        tasks = []
        is_child = true
        "#;

    fn setup_tmpdir() -> TempDir {
        let tmpdir = tempfile::tempdir().unwrap();

        let level1 = tmpdir.path().join("level1");
        let level2 = level1.join("level2");
        let level2_2 = level1.join("level2_2");
        let level3 = level2.join("level3");

        // create directories
        fs::create_dir_all(level2_2.clone()).unwrap();
        fs::create_dir_all(level3.clone()).unwrap();

        // create project files
        fs::write(tmpdir.path().join(PROJECT_FILE_NAME), ROOT_CONTENT).unwrap();
        fs::write(level1.join(PROJECT_FILE_NAME), LEVEL1_CONTENT).unwrap();
        fs::write(level2_2.join(PROJECT_FILE_NAME), LEVEL2_2_CONTENT).unwrap();
        fs::write(level3.join(PROJECT_FILE_NAME), LEVEL3_CONTENT).unwrap();

        tmpdir
    }

    #[test]
    fn find_project_with_children() {
        let tmpdir = setup_tmpdir();
        let tmppath = tmpdir.path();

        let mut expected = Project::new(
            tmppath.join(PROJECT_FILE_NAME),
            -1,
            "root".to_string(),
            false,
        );

        let mut level1_child = Project::new(
            tmppath.join("level1/.tutel.toml"),
            0,
            "l1".to_string(),
            true,
        );

        level1_child.attach_child(Project::new(
            tmppath.join("level1/level2_2/.tutel.toml"),
            1,
            "l2_2".to_string(),
            true,
        ));

        level1_child.attach_child(Project::new(
            tmppath.join("level1/level2/level3/.tutel.toml"),
            2,
            "l3".to_string(),
            true,
        ));

        expected.attach_child(level1_child);

        assert_eq!(
            load_project_rec(tmpdir.path().join("level1").as_path()).unwrap(),
            expected
        );
    }
}
