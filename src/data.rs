use ansi_term::Color;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{
    fmt::{Display, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    data: ProjectData,
}

impl Project {
    pub const fn new(path: PathBuf, data: ProjectData) -> Self {
        Self { path, data }
    }

    /// Save the project to where it was loaded from
    pub fn save(&mut self) -> Result<()> {
        let serialized = toml::to_string_pretty(&self.data)?;
        let target = self.path.join(crate::PROJECT_FILE_NAME);
        fs::write(target, serialized)?;
        Ok(())
    }

    pub fn get_task_mut(&mut self, index: u8) -> Result<&mut Task> {
        for t in &mut self.data.tasks {
            if t.index == index {
                return Ok(t)
            }
        }
        bail!("no such task with index: {index}")
    }

    pub fn add(&mut self, t: Task) {
        self.data.tasks.push(t);
    }

    pub fn remove(&mut self, index: u8) {
        self.data.tasks.retain(|t| t.index != index);
    }

    pub fn remove_all(&mut self) {
        self.data.tasks.clear();
    }

    pub fn remove_completed(&mut self) {
        self.data.tasks.retain(|t| !t.completed)
    }

    pub fn mark_completion_all(&mut self, completed: bool) {
        for mut t in &mut self.data.tasks {
            t.completed = completed;
        }
    }

    pub fn mark_completion(&mut self, index: u8, completed: bool) -> Result<()> {
        for mut t in &mut self.data.tasks {
            if t.index == index {
                t.completed = completed;
                return Ok(());
            }
        }

        bail!("no such task with such index: {index}")
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
        let mut tasks = String::new();
        let mut completed = true;

        for t in &self.data.tasks {
            write!(&mut tasks, "\n{}", t)?;
            if !t.completed {
                completed = false;
            }
        }

        let mut headline_marker = String::new();
        if completed {
            if self.data.tasks.is_empty() {
                headline_marker.push_str("[empty]");
            } else {
                headline_marker.push_str("[✓]");
            }
        } else {
            headline_marker.push_str("[X]");
        }

        let headline = format!("{}{}", headline_marker, self.data.name);
        write!(f, "{}", Color::Yellow.bold().paint(headline))?;

        if !self.data.tasks.is_empty() {
            write!(f, "{}", tasks)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    name: String,
    tasks: Vec<Task>,
}

impl ProjectData {
    pub fn empty(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tasks: Vec::new(),
        }
    }
}

/// A completable Task within a Project
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub completed: bool,
    pub index: u8,
}

impl Task {
    pub fn new(name: impl Into<String>, completed: bool, index: u8) -> Self {
        Self {
            name: name.into(),
            completed,
            index,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let marker = if self.completed {
            Color::Green.paint("[✓]")
        } else {
            Color::Red.paint("[X]")
        };
        f.write_fmt(format_args!("{:03}) {marker}{}", self.index, self.name))
    }
}
