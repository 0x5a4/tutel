use anyhow::Context;
use anyhow::{bail, Result};
use std::fs;
use std::path::PathBuf;

/// A Project holds multiple tasks. It also holds the location of
/// the file these tasks were loaded from and how many
/// recursive steps have been taken to reach that file.
#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub steps: usize,
    pub data: ProjectData,
}

impl Project {
    /// Creates a new project with no tasks
    pub const fn new(project_file: PathBuf, steps: usize, name: String) -> Self {
        Self {
            path: project_file,
            data: ProjectData {
                name,
                tasks: Vec::new(),
            },
            steps,
        }
    }

    /// Tries to load a project from the specified file.
    ///
    /// # Errors
    /// This function will return an Error when the file doesn't exists, or
    /// a Project couldn't be loaded from it.
    pub fn load(project_file: PathBuf, steps: usize) -> Result<Self> {
        let file_content =
            fs::read_to_string(project_file.as_path()).context("unable to read project file")?;

        let data: ProjectData =
            toml::from_str(file_content.as_str()).context("invalid project file syntax")?;

        Ok(Self {
            path: project_file,
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
        Ok(())
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
        bail!("no task with index {}", &index)
    }

    pub fn add(&mut self, name: String, completed: bool) {
        self.data
            .tasks
            .push(Task::new(name, completed, self.next_index()))
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
}

/// The part of a Project that needs to be saved/loaded
#[derive(Debug)]
pub struct ProjectData {
    pub name: String,
    pub tasks: Vec<Task>,
}

/// A completable Task within a Project
#[derive(Debug)]
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
