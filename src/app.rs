use crate::data::{Project, ProjectData, Task};
use anyhow::{bail, Context, Result};
use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::Path;
use crate::nav;

pub const PROJECT_FILE_NAME: &str = ".tutel.toml";

pub fn run() -> Result<()> {
    #[rustfmt::skip]
    let matches = App::new("tutel")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("init")
                .about("prints the shell functions necessary for tutelnav")
                .arg(Arg::with_name("shell")
                    .help("the shell to use")
                    .index(1)
                    .required(true)
                    .possible_values(&["fish", "bash"])
                ),
        )
        .subcommand(
            SubCommand::with_name("query")
                .about("query a project location from its name")
                .arg(Arg::with_name("name")
                    .help("the project name")
                    .index(1)
                    .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("create a new project")
                .arg(Arg::with_name("name")
                    .help("the project name")
                    .index(1)
                    .required(true)
                )
                .arg(
                    Arg::with_name("nonav")
                        .help("dont make this project reachable via tutelnav")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add a new task to the project")
                .arg(Arg::with_name("task")
                    .help("the task name to add")
                    .index(1)
                    .required(true)
                )
                .arg(
                    Arg::with_name("completed")
                        .help("set the task to be completed")
                        .takes_value(false)
                        .short("c"),
                ),
        )
        .subcommand(
            SubCommand::with_name("done")
                .about("mark a task as being done")
                .arg(Arg::with_name("index")
                    .help("the task index")
                    .index(1)
                    .required(true)
                )
                .arg(
                    Arg::with_name("not")
                        //Who would've thought?
                        .help("mark the task as not being done")
                        .takes_value(false)
                        .short("!"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove a task")
                .arg(Arg::with_name("index")
                    .help("the task index")
                    .index(1)
                    .required(true)
                )
        )
        .get_matches();

    //Run Commands
    match matches.subcommand() {
        ("init", Some(m)) => {
            let shell = m.value_of("shell").unwrap();
            nav::init(shell)?;
        }
        ("new", Some(m)) => {
            let name = m.value_of("name").unwrap();
            new_project(name, &*std::env::current_dir()?, !m.is_present("nonav"))?;
        }
        ("add", Some(m)) => {
            let mut p = load_project_rec(&*std::env::current_dir()?)?;
            let task = Task::new(m.value_of("task").unwrap(), m.is_present("completed"));
            p.add(task)?;
            p.save()?;
        }
        ("done", Some(m)) => {
            let mut p = load_project_rec(&*std::env::current_dir()?)?;
            let index = m
                .value_of("index")
                .unwrap()
                .parse::<usize>()
                .context("not a valid index")?;
            p.mark_completion(index, !m.is_present("not"))?;
            p.save()?;
        }
        ("rm", Some(m)) => {
            let mut p = load_project_rec(&*std::env::current_dir()?)?;
            let index = m
                .value_of("index")
                .unwrap()
                .parse::<usize>()
                .context("not a valid index")?;
            p.remove(index)?;
            p.save()?;
        }
        ("query", Some(m)) => {
            println!("{}", nav::query_nav(m.value_of("name").unwrap())?.display());
        }
        (_, _) => {
            let p = load_project_rec(&*std::env::current_dir()?)?;
            println!("{}", p);
        }
    }
    Ok(())
}


/// Determines whether a project exists in the given path by checking
/// for the existence of .tutel.project
fn has_project(path: &Path) -> bool {
    let project = path.join(PROJECT_FILE_NAME);
    return project.exists() && project.is_file();
}

/// Attempts to load a project from the given path.
/// Assumes the path is a directory and a file called .tutel.toml exists
fn load_project(path: &Path) -> Result<Project> {
    let project_path = path.join(PROJECT_FILE_NAME);

    let meta = project_path.metadata()?;

    let file_content =
        fs::read_to_string(project_path.as_path()).context("unable to read project file")?;

    let project_data: ProjectData =
        toml::from_str(file_content.as_str()).context("invalid project file syntax")?;

    Ok(Project::new(
        project_path,
        meta.permissions().readonly(),
        project_data,
    ))
}

/// Walks the path upwards until .tutel.toml is found and loads it
fn load_project_rec(path: &Path) -> Result<Project> {
    for p in path.ancestors() {
        if has_project(p) {
            return Ok(load_project(p)?);
        }
    }

    bail!("no project found");
}

//Creates a new project and adds it to the project list
fn new_project(name: &str, path: &Path, nav: bool) -> Result<()> {
    if name.contains(" ") {
        bail!("name cannot contain whitespaces");
    }

    let new = path.join(PROJECT_FILE_NAME);
    if new.exists() {
        bail!("project already exists at {}", path.to_string_lossy());
    }

    let project = ProjectData::empty(name);
    fs::write(new, toml::to_string_pretty(&project)?)?;

    if nav {
        nav::add_to_nav(name, path)?;
    }

    Ok(())
}
