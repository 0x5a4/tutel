#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::nursery)]

use std::{fs, io::Write};
use tempfile::NamedTempFile;

use ansi_term::Color;
use anyhow::{bail, Context, Result};
use clap::{App, ArgMatches};

mod app;

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
        Some(("new", m)) => new_project(m)?,
        Some(("add", m)) => add(m)?,
        Some(("done", m)) => done(m)?,
        Some(("rm", m)) => remove(m)?,
        Some(("edit", m)) => edit_task(m)?,
        Some(("completions", m)) => print_completions(m),
        _ => {
            let p = tutel::load_project_rec(&*std::env::current_dir()?)?;
            println!("{}", p);
        }
    }

    Ok(())
}

fn add(args: &ArgMatches) -> Result<()> {
    let mut p = tutel::load_project_rec(&*std::env::current_dir()?)?;
    let mut taskname = String::new();
    let values = args.values_of("taskname").unwrap();
    let vlen = values.len();

    for (i, s) in values.enumerate() {
        taskname.push_str(s);
        if i < vlen - 1 {
            taskname.push(' ');
        }
    }

    p.add(taskname, args.is_present("completed"));
    p.save()?;
    Ok(())
}

fn done(args: &ArgMatches) -> Result<()> {
    let mut p = tutel::load_project_rec(&*std::env::current_dir()?)?;
    let completed = !args.is_present("not");

    if args.is_present("all") {
        p.mark_completion_all(completed)
    } else {
        let indices = args
            .values_of("indices")
            .unwrap()
            .map(|index| index.parse::<u8>().context("invalid index: {index}"))
            .collect::<Vec<_>>();

        for index in indices {
            p.mark_completion(index?, !args.is_present("not"))?;
        }
    }

    p.save()?;

    Ok(())
}

fn remove(args: &ArgMatches) -> Result<()> {
    let mut p = tutel::load_project_rec(&*std::env::current_dir()?)?;

    if args.is_present("all") {
        p.remove_all();
    } else if args.is_present("cleanup") {
        p.remove_completed()
    } else {
        let indices = args
            .values_of("indices")
            .unwrap()
            .map(|index| index.parse::<u8>().context("invalid index: {index}"))
            .collect::<Vec<_>>();

        for index in indices {
            p.remove(index?);
        }
    }
    p.save()?;

    Ok(())
}

fn print_completions(args: &ArgMatches) {
    match args.value_of("shell").unwrap() {
        "bash" => println!("{}", BASH_COMPLETIONS),
        "zsh" => println!("{}", ZSH_COMPLETIONS),
        "fish" => println!("{}", FISH_COMPLETIONS),
        "elvish" => println!("{}", ELVISH_COMPLETIONS),
        _ => {}
    }
}

fn edit_task(args: &ArgMatches) -> Result<()> {
    let index = args.value_of("index").unwrap().parse()?;

    let mut project = tutel::load_project_rec(&*std::env::current_dir()?)?;
    let task = project.get_task_mut(index)?;
    let mut tmpfile = NamedTempFile::new()?;
    tmpfile.write_all(task.name.as_bytes())?;
    tmpfile.flush()?;

    let path = tmpfile.into_temp_path();

    let editor = if let Some(editor) = args.value_of("editor") {
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

/// Creates a new project
///
/// If no project name is given, the name of the current directory is chosen
fn new_project(args: &ArgMatches) -> Result<()> {
    let name = args.value_of("name");
    let force = args.is_present("force");

    let path = std::env::current_dir()?;

    // TODO: un-hack me
    let name = if let Some(name) = name {
        name.to_string()
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
