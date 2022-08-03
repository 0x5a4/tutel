#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::nursery)]

use app::{Command, TaskSelector};
use std::{fs, io::Write};
use tempfile::NamedTempFile;

use ansi_term::Color;
use anyhow::{bail, Context, Result};

mod app;

const BASH_COMPLETIONS: &str = "";
const ZSH_COMPLETIONS: &str = "";
const FISH_COMPLETIONS: &str = "";
const ELVISH_COMPLETIONS: &str = "";

fn main() {
    match run_app(app::parse_cli()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {}", Color::Red.paint("[tutel]"), e,);

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
        Command::MarkCompletion(selector, completed) => done(selector, completed),
        Command::RemoveTask(selector) => remove(selector),
        Command::EditTask(index, editor) => edit_task(index, editor),
        Command::PrintCompletion(shell) => print_completions(shell.as_str()),
        Command::RemoveProject => remove_project(),
    }
}

fn print_list() -> Result<()> {
    let p = tutel::load_project_rec(&*std::env::current_dir()?)?;
    println!("{p}");

    Ok(())
}

fn add(desc: String, completed: bool) -> Result<()> {
    let mut p = tutel::load_project_rec(&*std::env::current_dir()?)?;
    p.add(desc, completed);
    p.save()?;
    Ok(())
}

fn done(selector: TaskSelector, completed: bool) -> Result<()> {
    let mut p = tutel::load_project_rec(&*std::env::current_dir()?)?;

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
    let mut p = tutel::load_project_rec(&*std::env::current_dir()?)?;

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
    let p = tutel::load_project_rec(&*std::env::current_dir()?)?;

    fs::remove_file(p.path).context("could not delete project file")
}

fn print_completions(shell: &str) -> Result<()> {
    match shell {
        "bash" => println!("{}", BASH_COMPLETIONS),
        "zsh" => println!("{}", ZSH_COMPLETIONS),
        "fish" => println!("{}", FISH_COMPLETIONS),
        "elvish" => println!("{}", ELVISH_COMPLETIONS),
        _ => bail!("no such shell {shell}"),
    };

    Ok(())
}

fn edit_task(index: u8, editor: String) -> Result<()> {
    let mut project = tutel::load_project_rec(&*std::env::current_dir()?)?;
    let task = project.get_task_mut(index)?;

    let mut tmpfile = NamedTempFile::new()?;
    tmpfile.write_all(task.name.as_bytes())?;

    // Spawn editor process
    let mut cmd = std::process::Command::new(editor.as_str())
        .arg(tmpfile.path())
        .spawn()
        .context("editor {editor} not found")?;

    cmd.wait()?;

    // Write changes
    let new = fs::read_to_string(tmpfile.path())?;
    task.name = new.replace('\n', " ");

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

    tutel::new_project(&path, name)?;

    Ok(())
}
