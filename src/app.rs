use clap::{builder::PossibleValuesParser, Arg, Command};

#[rustfmt::skip]
pub fn new() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("new")
                .about("create a new project")
                .visible_alias("n")
                .arg(
                    Arg::new("name")
                        .help("the project name")
                        .index(1)
                )
                .arg(
                    Arg::new("force")
                        .help("force project creation")
                        .long("force")
                        .short('f')
                        .takes_value(false)
                ),
        )
        .subcommand(
            Command::new("add")
                .about("add a new task to the project")
                .visible_alias("a")
                .arg(
                    Arg::new("taskname")
                        .help("the name of the task to add")
                        .index(1)
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                )
                .arg(
                    Arg::new("completed")
                        .help("set the task to be completed")
                        .takes_value(false)
                        .short('c')
                ),
        )
        .subcommand(
            Command::new("done")
                .about("mark a task as being done")
                .visible_alias("d")
                .arg(
                    Arg::new("indices")
                        .help("the task(s) index/indices")
                        .index(1)
                        .multiple(true)
                        .required_unless_present("all")
                )
                .arg(
                    Arg::new("not")
                        //Who would've thought?
                        .help("mark the task as not being done")
                        .takes_value(false)
                        .short('!')
                        .long("not")
                )
                .arg(
                    Arg::new("all")
                        .help("select all tasks")
                        .takes_value(false)
                        .short('a')
                        .long("all")
                )
        )
        .subcommand(
            Command::new("rm")
                .about("remove a task")
                .arg(
                    Arg::new("indices")
                        .help("the task(s) index/indices")
                        .index(1)
                        .multiple(true)
                        .required_unless_present_any(&["all", "cleanup"])
                )
                .arg(
                    Arg::new("all")
                        .help("remove all tasks")
                        .takes_value(false)
                        .short('a')
                        .long("all")
                        .conflicts_with("cleanup")
                )
                .arg(
                    Arg::new("cleanup")
                        .help("remove all completed tasks")
                        .takes_value(false)
                        .short('c')
                        .long("cleanup")
                        .conflicts_with("all")
                )
        )
        .subcommand(
            Command::new("edit")
                .about("edit a task")
                .visible_alias("e")
                .arg(
                    Arg::new("index")
                        .help("the task index")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("editor")
                        .help("the editor to use")
                        .short('e')
                        .long("editor")
                        .takes_value(true)
                        .forbid_empty_values(true)
                )
        )
        .subcommand(
            Command::new("completions")
                .about("output completion scripts for a given shell")
                .arg(
                    Arg::new("shell")
                        .index(1)
                        .takes_value(true)
                        .value_parser(PossibleValuesParser::new(["bash", "zsh", "fish", "elvish"]))
                        .required(true)
                )
        )
}
