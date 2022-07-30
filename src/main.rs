#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::nursery)]
use std::path::Path;
use std::{fs, io::Write};
use tempfile::NamedTempFile;

use ansi_term::Color;
use anyhow::{bail, Context, Result};
use clap::App;
use data::{Project, ProjectData, Task};

mod app;
mod data;

const PROJECT_FILE_NAME: &str = ".tutel.toml";
const BASH_COMPLETIONS: &str = include_str!(concat!(env!("OUT_DIR"), "/tutel.bash"));
const ZSH_COMPLETIONS: &str = include_str!(concat!(env!("OUT_DIR"), "/_tutel"));
const FISH_COMPLETIONS: &str = include_str!(concat!(env!("OUT_DIR"), "/tutel.fish"));
const ELVISH_COMPLETIONS: &str = include_str!(concat!(env!("OUT_DIR"), "/tutel.elv"));

fn main() {
    let app = app::new();

    match run_app(app) {
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

fn run_app(app: App) -> Result<()> {
    let matches = app.get_matches();

    //Run Commands
    match matches.subcommand() {
        Some(("new", m)) => {
            let name = m.value_of("name");
            let force = m.is_present("force");
            new_project(name, &*std::env::current_dir()?, force)?;
        }
        Some(("add", m)) => {
            let mut p = load_project_rec(&*std::env::current_dir()?)?;
            let mut taskname = String::new();
            let values = m.values_of("taskname").unwrap();
            let vlen = values.len();

            for (i, s) in values.enumerate() {
                taskname.push_str(s);
                if i < vlen - 1 {
                    taskname.push(' ');
                }
            }

            let task = Task::new(taskname, m.is_present("completed"), p.next_index());
            p.add(task);
            p.save()?;
        }
        Some(("done", m)) => {
            let mut p = load_project_rec(&*std::env::current_dir()?)?;
            let completed = !m.is_present("not");

            if m.is_present("all") {
                p.mark_completion_all(completed)
            } else {
                let indices = m
                    .values_of("indices")
                    .unwrap()
                    .map(|index| index.parse::<u8>().context("invalid index: {index}"))
                    .collect::<Vec<_>>();

                for index in indices {
                    p.mark_completion(index?, !m.is_present("not"))?;
                }
            }

            p.save()?;
        }
        Some(("rm", m)) => {
            let mut p = load_project_rec(&*std::env::current_dir()?)?;

            if m.is_present("all") {
                p.remove_all();
            } else if m.is_present("cleanup") {
                p.remove_completed()
            } else {
                let indices = m
                    .values_of("indices")
                    .unwrap()
                    .map(|index| index.parse::<u8>().context("invalid index: {index}"))
                    .collect::<Vec<_>>();

                for index in indices {
                    p.remove(index?);
                }
            }
            p.save()?;
        }
        Some(("edit", m)) => {
            edit_task(m.value_of("index").unwrap().parse()?, m.value_of("editor"))?;
        }
        Some(("completions", m)) => {
            print_completions(m.value_of("shell").unwrap());
        }
        _ => {
            let p = load_project_rec(&*std::env::current_dir()?)?;
            println!("{}", p);
        }
    }

    Ok(())
}

/// Prints shell completions for the given shell by name
fn print_completions(shell_name: &str) {
    match shell_name {
        "bash" => println!("{}", BASH_COMPLETIONS),
        "zsh" => println!("{}", ZSH_COMPLETIONS),
        "fish" => println!("{}", FISH_COMPLETIONS),
        "elvish" => println!("{}", ELVISH_COMPLETIONS),
        _ => {}
    }
}

fn edit_task(index: u8, editor_cmd: Option<&str>) -> Result<()> {
    let mut project = load_project_rec(&*std::env::current_dir()?)?;
    let task = project.get_task_mut(index)?;
    let mut tmpfile = NamedTempFile::new()?;
    tmpfile.write_all(task.name.as_bytes())?;
    tmpfile.flush()?;

    let path = tmpfile.into_temp_path();

    let editor = if let Some(editor) = editor_cmd {
        editor.to_string()
    } else {
        std::env::var_os("EDITOR")
            .context("no editor specified and EDITOR isnt set")?
            .to_string_lossy()
            .to_string()
    };

    // Spawn editor process
    let mut cmd = std::process::Command::new(editor)
        .arg(&path)
        .spawn()
        .context("editor {editor} not found")?;

    cmd.wait()?;

    // Write changes
    let new_task = fs::read_to_string(path)?;
    task.name = new_task;

    project.save()?;

    Ok(())
}

/// Determines whether a project exists in the given path by checking
/// for the existence of .tutel.project
fn has_project(path: &Path) -> bool {
    let project = path.join(PROJECT_FILE_NAME);
    project.exists() && project.is_file()
}

/// Attempts to load a project from the given path.
/// Assumes the path is a directory and a file called .tutel.toml exists
fn load_project(path: &Path) -> Result<Project> {
    let project_path = path.join(PROJECT_FILE_NAME);

    let file_content =
        fs::read_to_string(project_path.as_path()).context("unable to read project file")?;

    let project_data: ProjectData =
        toml::from_str(file_content.as_str()).context("invalid project file syntax")?;

    Ok(Project::new(path.to_path_buf(), project_data))
}

/// Walks the path upwards until .tutel.toml is found and loads it
fn load_project_rec(path: &Path) -> Result<Project> {
    for p in path.ancestors() {
        if has_project(p) {
            return load_project(p);
        }
    }

    bail!("no project found");
}

/// Creates a new project and adds it to the project list
///
/// If no project name is given, the name of the current directory is chosen
fn new_project(name: Option<&str>, path: &Path, force: bool) -> Result<()> {
    // TODO: un-hack me
    let name = if let Some(name) = name {
        name.to_string()
    } else if let Some(name) = path.file_name() {
        name.to_string_lossy().to_string()
    } else {
        bail!("no project name given and cannot be inferred")
    };

    let new = path.join(PROJECT_FILE_NAME);
    if new.exists() && !force {
        bail!(
            "project already exists at {}. try using --force",
            path.to_string_lossy()
        );
    }

    let project = ProjectData::empty(name);
    fs::write(new, toml::to_string_pretty(&project)?)?;

    Ok(())
}
