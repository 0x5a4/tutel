use anyhow::Context;
use anyhow::{bail, Result};
use colored::Colorize;
use std::fs;
use std::{
    fmt::{Display, Write},
    path::PathBuf,
};

/// A Project hold multiple tasks. It also holds the location of
/// the file these tasks were loaded from and how many
/// recursive steps have been taken to reach that file.
#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub steps: usize,
    data: ProjectData,
}

impl Project {
    /// Creates a new project with not tasks
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
    /// This function will return an Error when the file doesnt exist, or
    /// a Project could not be loaded from it.
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

    /// Save the project to where it was loaded from.
    ///
    /// # Errors
    /// This function will return an Error when the file this project was
    /// loaded from cant be written(doesnt exist, permission denied) or the
    /// project could not be serialized. Both of these are not very likely to occur
    pub fn save(&mut self) -> Result<()> {
        let serialized = toml::to_string_pretty(&self.data)?;
        fs::write(self.path.as_path(), serialized).context("unable to write project file")?;
        Ok(())
    }

    /// Returns a mutable reference to a contained Task.
    ///
    /// # Errors
    /// This function will return an error if a Task with the given index
    /// could not be found.
    pub fn get_task_mut(&mut self, index: u8) -> Result<&mut Task> {
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

    pub fn remove(&mut self, index: u8) {
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
    pub fn mark_completion(&mut self, index: u8, completed: bool) -> Result<()> {
        let task = self.get_task_mut(index)?;
        task.completed = completed;
        Ok(())
    }

    /// Calculates the next highest unused index.
    ///
    /// Wraps around to 0 after 255 is reached.
    pub fn next_index(&self) -> u8 {
        if self.data.tasks.is_empty() {
            return 0;
        }

        let mut highest = 0;
        for t in &self.data.tasks {
            if t.index > highest {
                highest = t.index;
            }
        }
        highest + 1
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Gather all tasks and their completion state
        let mut tasks = String::new();
        let mut completed = true;

        for t in &self.data.tasks {
            write!(&mut tasks, "\n{}", t)?;
            if !t.completed {
                completed = false;
            }
        }

        let steps_counter = if self.steps == 0 {
            String::new()
        } else {
            format!(" [-{}]", self.steps).blue().to_string()
        };

        let marker = if completed { "✓".green() } else { "X".red() };

        let headline_marker = format!("[{marker}]{steps_counter}");

        let headline = format!("{} {}", headline_marker.yellow().bold(), self.data.name.bold());
        write!(f, "{}", headline)?;

        if !self.data.tasks.is_empty() {
            write!(f, "{}", tasks)?;
        } else {
            write!(f, "\n[empty]")?;
        }

        Ok(())
    }
}

/// The part of a Project that needs to be saved/loaded
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ProjectData {
    pub(crate) name: String,
    pub(crate) tasks: Vec<Task>,
}

/// A completable Task within a Project
#[derive(Debug)]
pub struct Task {
    pub desc: String,
    pub index: u8,
    pub completed: bool,
}

impl Task {
    pub fn new(name: impl Into<String>, completed: bool, index: u8) -> Self {
        Self {
            desc: name.into(),
            completed,
            index,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let marker = if self.completed {
            "[✓]".green()
        } else {
            "[X]".red()
        };
        f.write_fmt(format_args!(
            "{:03} {} {marker}{}",
            self.index,
            "│".bold(),
            self.desc
        ))
    }
}
