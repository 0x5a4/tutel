#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::nursery)]

use app::{Command, TaskSelector};
use owo_colors::OwoColorize;
use std::{fs, io::Write};
use tempfile::NamedTempFile;
use tutel::{Project, Task};

use anyhow::{bail, Context, Result};

mod app;

fn main() {
    match run_app(app::parse_cli()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {}", "[tutel]".red(), e,);

            if e.chain().len() > 1 {
                eprintln!("\t{}", e.root_cause());
            }
            std::process::exit(1);
        }
    }
}

fn run_app(command: Command) -> Result<()> {
    //Run Commands
    match command {
        Command::Show => print_list(),
        Command::NewProject { name, force } => new_project(name, force),
        Command::AddTask { desc, completed } => add(desc, completed),
        Command::MarkCompletion(completed, selector) => done(selector, completed),
        Command::RemoveTask(selector) => remove(selector),
        Command::EditTask(editor, index) => edit_task(index, editor),
        Command::RemoveProject => remove_project(),
    }
}

fn print_list() -> Result<()> {
    let p = tutel::load_project_rec(&std::env::current_dir()?)?;
    println!("{}", stringify_project(&p));

    Ok(())
}

fn add(desc: String, completed: bool) -> Result<()> {
    let mut p = tutel::load_project_rec(&std::env::current_dir()?)?;
    p.add(desc, completed);
    p.save()?;
    Ok(())
}

fn done(selector: TaskSelector, completed: bool) -> Result<()> {
    let mut p = tutel::load_project_rec(&std::env::current_dir()?)?;

    match selector {
        TaskSelector::Indexed(indices) => {
            for index in indices {
                p.mark_completion(index, completed)?;
            }
        }
        TaskSelector::All => p.mark_completion_all(completed),
        TaskSelector::Completed => unreachable!(),
    }

    p.save()?;

    Ok(())
}

fn remove(selector: TaskSelector) -> Result<()> {
    let mut p = tutel::load_project_rec(&std::env::current_dir()?)?;

    match selector {
        TaskSelector::Indexed(indices) => {
            for index in indices {
                p.remove(index);
            }
        }
        TaskSelector::All => p.remove_all(),
        TaskSelector::Completed => p.remove_completed(),
    }

    p.save()?;

    Ok(())
}

fn remove_project() -> Result<()> {
    let p = tutel::load_project_rec(&std::env::current_dir()?)?;

    fs::remove_file(p.path).context("could not delete project file")
}

fn edit_task(index: usize, editor: String) -> Result<()> {
    let mut project = tutel::load_project_rec(&std::env::current_dir()?)?;
    let task = project.get_task_mut(index)?;

    let mut tmpfile = NamedTempFile::new()?;
    tmpfile.write_all(task.desc.as_bytes())?;

    // Spawn editor process
    let mut cmd = std::process::Command::new(editor.as_str())
        .arg(tmpfile.path())
        .spawn()
        .context("editor {editor} not found")?;

    cmd.wait()?;

    // Write changes
    let new = fs::read_to_string(tmpfile.path())?;
    task.desc = new.replace('\n', " ");

    project.save()?;

    Ok(())
}

/// Creates a new project
///
/// If no project name is given, the name of the current directory is chosen
fn new_project(name: Option<String>, force: bool) -> Result<()> {
    let path = std::env::current_dir()?;

    // TODO: un-hack me
    let name = if let Some(name) = name {
        name
    } else if let Some(name) = path.file_name() {
        name.to_string_lossy().to_string()
    } else {
        bail!("no project name given and cannot be inferred")
    };

    let new = path.join(tutel::PROJECT_FILE_NAME);
    if new.exists() && !force {
        bail!(
            "project already exists at {}. try using --force",
            path.to_string_lossy()
        );
    }

    tutel::new_project(name)?;

    Ok(())
}

fn stringify_project(project: &Project) -> String {
    let mut result = String::new();
    let mut tasks = String::new();
    let mut completed = true;

    for t in &project.data.tasks {
        tasks.push('\n');
        tasks.push_str(stringify_task(t).as_str());
        if !t.completed {
            completed = false;
        }
    }

    let steps = if project.steps == 0 {
        String::new()
    } else {
        format!(" [-{}]", project.steps).blue().bold().to_string()
    };

    let marker = if completed {
        "✓".green().to_string()
    } else {
        "X".red().to_string()
    };

    let headline = format!(
        "{}{}{}{} {}",
        '['.yellow().bold(), 
        marker,
        ']'.yellow().bold(),
        steps,
        project.data.name.bold()
    );
    result.push_str(headline.as_str());

    if tasks.is_empty() {
        result.push_str("\n[empty]");
    } else {
        result.push_str(tasks.as_str());
    }

    result
}

fn stringify_task(task: &Task) -> String {
    let marker = if task.completed {
        "[✓]".green().to_string()
    } else {
        "[X]".red().to_string()
    };

    format!("{:03} {} {}{}", task.index, "│".bold(), marker, task.desc)
}
