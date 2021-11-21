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
    pub readonly: bool,
    pub path: PathBuf,
    data: ProjectData,
}

impl Project {
    pub fn new(path: PathBuf, readonly: bool, data: ProjectData) -> Self {
        Self {
            path,
            readonly,
            data,
        }
    }

    pub fn save(&mut self) -> Result<()> {
        if self.readonly {
            bail!("project is readonly");
        }
        let serialized = toml::to_string_pretty(&self.data)?;
        fs::write(&self.path, serialized)?;
        Ok(())
    }

    pub fn add(&mut self, t: Task) -> Result<()> {
        if self.readonly {
            bail!("project is readonly");
        }

        self.data.tasks.push(t);
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<()> {
        if self.readonly {
            bail!("project is readonly");
        }

        self.data.tasks.remove(index);
        Ok(())
    }

    pub fn mark_completion(&mut self, index: usize, completed: bool) -> Result<()> {
        if self.readonly {
            bail!("project is readonly");
        }

        if self.data.tasks.len() <= index {
            bail!("index out of bounds");
        }

        self.data.tasks[index].completed = completed;
        Ok(())
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tasks = String::new();
        let mut completed = true;

        for (i, t) in self.data.tasks.iter().enumerate() {
            write!(&mut tasks, "\n{}) {}", i, t)?;
            if !t.completed {
                completed = false;
            }
        }

        let mut headline_marker = String::new();
        if completed {
            if self.data.tasks.is_empty() {
                headline_marker.push_str("[empty]");
            } else {
                headline_marker.push_str("[✓]")
            }
        } else {
            headline_marker.push_str("[⨯]");
        }

        let headline = format!("{}{}", headline_marker, self.data.name);
        write!(f, "{}", Color::Yellow.bold().paint(headline))?;

        if self.readonly {
            write!(f, "🔒")?;
        }

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

impl Task {
    pub fn new(name: impl Into<String>, completed: bool) -> Self {
        Self {
            name: name.into(),
            completed,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let marker = if self.completed {
            Color::Green.paint("[✓]")
        } else {
            Color::Red.paint("[⨯]")
        };
        f.write_fmt(format_args!("{}{}", marker, self.name))
    }
}
