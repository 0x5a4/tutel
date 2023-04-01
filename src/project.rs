use anyhow::Context;
use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// A Project holds multiple tasks. It also holds the location of
/// the file these tasks were loaded from and how many
/// recursive steps have been taken to reach that file.
#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    pub data: ProjectData,
    path: PathBuf,
    steps: isize,
    children: Vec<Project>,
}

impl Project {
    /// Creates a new project with no tasks and no children
    #[must_use]
    pub const fn new(project_file: PathBuf, steps: isize, name: String, is_child: bool) -> Self {
        Self {
            path: project_file,
            steps,
            children: Vec::new(),
            data: ProjectData {
                name,
                tasks: Vec::new(),
                is_child,
            },
        }
    }

    /// Tries to load a project from the specified file.
    ///
    /// # Errors
    /// This function will return an Error when the file doesn't exists, or
    /// a Project couldn't be loaded from it.
    pub fn load(project_file: PathBuf, steps: isize) -> Result<Self> {
        if !project_file.is_file() {
            bail!("not a project file: {}", project_file.to_string_lossy())
        }

        let file_content =
            fs::read_to_string(project_file.as_path()).context("unable to read project file")?;

        let data: ProjectData =
            toml::from_str(file_content.as_str()).context("invalid project file syntax")?;

        Ok(Self {
            path: project_file,
            children: Vec::new(),
            data,
            steps,
        })
    }

    /// Saves the project to where it was loaded from.
    ///
    /// # Errors
    /// This function will return an Error when the file this project was
    /// loaded from can't be written(doesnt exist, permission denied) or the
    /// project could not be serialized. Both of these are not very likely to occur
    pub fn save(&mut self) -> Result<()> {
        let serialized = toml::to_string_pretty(&self.data)?;
        fs::write(self.path.as_path(), serialized).context("unable to write project file")?;

        for child in &mut self.children {
            child.save()?;
        }

        Ok(())
    }

    #[must_use]
    pub const fn is_child(&self) -> bool {
        self.data.is_child
    }

    /// Attaches a child to this project
    ///
    /// # Panics
    /// This functions panics in test and debug builds when a child is attached that isnt
    /// actually a child(`is_child = false`).
    pub fn attach_child(&mut self, child: Self) {
        assert!(
            child.data.is_child,
            "attached a child to a project that isnt actually a child: {}",
            child.path.to_string_lossy()
        );

        self.children.push(child);
    }

    /// Returns a mutable reference to a contained Task.
    ///
    /// # Errors
    /// This function will return an error if no Task with the given index
    /// could be found.
    pub fn get_task_mut(&mut self, index: usize) -> Result<&mut Task> {
        for t in &mut self.data.tasks {
            if t.index == index {
                return Ok(t);
            }
        }

        bail!("no task {}:{} found", self.data.name, index)
    }

    pub fn find_task(&mut self, selector: &str, index: usize) -> Result<&mut Task> {
        if self.data.name.starts_with(selector) {
            return self.get_task_mut(index);
        }

        let mut result = None;
        if !self.children.is_empty() {
            for child in &mut self.children {
                let child_task = child.find_task(selector, index);

                if child_task.is_ok() {
                    result = Some(child_task);
                }
            }
        }

        if let Some(result) = result {
            return result;
        }

        bail!("no task {}:{} found", self.data.name, index)
    }

    pub fn add(&mut self, name: String, completed: bool) {
        self.data
            .tasks
            .push(Task::new(name, completed, self.next_index()));
    }

    pub fn remove(&mut self, index: usize) {
        self.data.tasks.retain(|t| t.index != index);
    }

    pub fn remove_all(&mut self) {
        self.data.tasks.clear();
    }

    pub fn remove_completed(&mut self) {
        self.data.tasks.retain(|t| !t.completed);
    }

    pub fn mark_completion_all(&mut self, completed: bool) {
        for mut t in &mut self.data.tasks {
            t.completed = completed;
        }
    }

    /// Marks the Task with the given Index as completed/not completed.
    ///
    /// # Errors
    /// This function will return an error if a Task with the given index
    /// could not be found.
    pub fn mark_completion(&mut self, index: usize, completed: bool) -> Result<()> {
        let task = self.get_task_mut(index)?;
        task.completed = completed;
        Ok(())
    }

    /// Calculates the next highest unused index.
    ///
    /// Wraps around to 0 after 999 is reached.
    #[must_use]
    pub fn next_index(&self) -> usize {
        if self.data.tasks.is_empty() {
            return 0;
        }

        let mut highest = 0;
        for t in &self.data.tasks {
            if t.index > highest {
                highest = t.index;
            }
        }

        // Wrap around
        if highest >= 999 {
            0
        } else {
            highest + 1
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    #[must_use]
    pub const fn steps(&self) -> isize {
        self.steps
    }

    #[must_use]
    pub fn get_children(&self) -> &[Self] {
        self.children.as_slice()
    }
}

/// The part of a Project that needs to be saved/loaded
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct ProjectData {
    pub name: String,
    pub tasks: Vec<Task>,
    pub is_child: bool,
}

/// A completable Task within a Project
#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub desc: String,
    pub index: usize,
    pub completed: bool,
}

impl Task {
    pub fn new(name: impl Into<String>, completed: bool, index: usize) -> Self {
        Self {
            desc: name.into(),
            completed,
            index,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::{io::Write, path::PathBuf};
    use tempfile::NamedTempFile;

    use super::{Project, Task};

    #[test]
    fn load() {
        let mut tmpfile = NamedTempFile::new().expect("unable to create tmpfile");
        write!(
            tmpfile,
            r#"
            name = 'testproject'

            [[tasks]]
            desc = 'testtask'
            completed = true
            index = 67

            [[tasks]]
            desc = 'moretest'
            completed = false
            index = 99
               "#
        )
        .expect("unable to write tmpfile");

        let project =
            Project::load(tmpfile.path().to_path_buf(), 0).expect("unable to load project");

        assert_eq!(project.data.name, "testproject");
        assert_eq!(
            project.data.tasks,
            vec![
                Task::new("testtask", true, 67),
                Task::new("moretest", false, 99),
            ]
        );
    }

    #[test]
    fn save() {
        let tmpfile = NamedTempFile::new().expect("unable to create tmpfile");

        let mut project = Project::new(
            tmpfile.path().to_path_buf(),
            0,
            String::from("testproject"),
            false,
        );

        project.add("hypa hypa".to_string(), false);
        project.add("HYPA HYPA".to_string(), true);

        project.save().expect("unable to save project");
    }

    #[test]
    fn remove_task() {
        let mut project = Project::new(
            PathBuf::from("/invalid/path"),
            0,
            String::from("testproject"),
            false,
        );

        project.add("iam".to_string(), false);
        project.add("root".to_string(), true);

        project.remove(0);

        assert!(project.get_task_mut(0).is_err());
        assert_eq!(project.next_index(), 2);
    }

    #[test]
    fn remove_all_completed() {
        let mut project = Project::new(
            PathBuf::from("/invalid/path"),
            0,
            String::from("testproject"),
            false,
        );

        project.add("never".to_string(), true);
        project.add("gonna".to_string(), false);
        project.add("give".to_string(), false);
        project.add("you".to_string(), true);
        project.add("up".to_string(), false);

        project.remove_completed();

        assert!(project.get_task_mut(0).is_err());
        assert!(project.get_task_mut(3).is_err());
    }

    #[test]
    fn next_index() {
        let mut project = Project::new(PathBuf::new(), 0, String::from("dummy"), false);
        project.data.tasks.push(Task::new("a", false, 5));
        project.data.tasks.push(Task::new("a", false, 16));

        assert_eq!(project.next_index(), 17);
    }

    #[test]
    fn wrap_index() {
        let mut project = Project::new(PathBuf::new(), 0, String::from("dummy"), false);
        project.data.tasks.push(Task::new("a", false, 999));
        project.data.tasks.push(Task::new("a", false, 3));

        assert_eq!(project.next_index(), 0);
    }

    #[test]
    fn get_task_from_selector() {
        let mut root = Project::new(PathBuf::new(), 0, String::from("root"), false);
        let mut child1 = Project::new(PathBuf::new(), 1, String::from("child1"), true);
        let mut child2 = Project::new(PathBuf::new(), 1, String::from("child2"), true);
        let mut child2_1 = Project::new(PathBuf::new(), 2, String::from("child2_1"), true);

        // create tasks
        child1.add("wegot".to_string(), false);
        child2_1.add("themoves".to_string(), false);

        // attach children
        child2.attach_child(child2_1);
        root.attach_child(child1);
        root.attach_child(child2);

        assert_eq!(
            *root.find_task("child1", 0).unwrap(),
            Task::new("wegot", false, 0)
        );
        assert_eq!(
            *root.find_task("child2_1", 0).unwrap(),
            Task::new("themoves", false, 0)
        );
    }
}
