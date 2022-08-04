use bpaf::{command, construct, env, long, positional, short, Info, OptionParser, Parser};

/// Indicates what Tasks(s) to select
#[derive(Debug, Clone)]
pub enum TaskSelector {
    Indexed(Vec<u8>),
    All,
    Completed,
}

/// The command to execute
#[derive(Debug, Clone)]
pub enum Command {
    Show,
    NewProject {
        name: Option<String>,
        force: bool,
    },
    AddTask {
        desc: String,
        completed: bool,
        due: Option<String>,
    },
    MarkCompletion(TaskSelector, bool),
    RemoveTask(TaskSelector),
    EditTask(u8, String),
    PrintCompletion(String),
    RemoveProject,
}

/// Parse the command line and return the command to be executed
pub fn parse_cli() -> Command {
    let new_cmd = command("new", Some("create a new project"), new_project_command());

    let add_cmd = command("add", Some("add a new task"), add_task_command());
    let add_short = command::<_, &str>("a", None, add_task_command()).hide();

    let done_cmd = command(
        "done",
        Some("mark a task as being completed"),
        task_completed_command(),
    );
    let done_short = command::<_, &str>("d", None, task_completed_command()).hide();

    let rm_cmd = command("rm", Some("remove a task"), remove_task_command());

    let edit_cmd = command("edit", Some("edit an existing task"), edit_task_command());
    let edit_short = command::<_, &str>("e", None, edit_task_command()).hide();

    let completion_cmd = command(
        "completions",
        Some("print shell completions"),
        print_completions_command(),
    );

    let show = Parser::pure(Command::Show);

    let parser = construct!([
        new_cmd,
        add_cmd,
        add_short,
        done_cmd,
        done_short,
        rm_cmd,
        edit_cmd,
        edit_short,
        completion_cmd,
        show
    ]);

    Info::default()
        .version(concat!("tutel v", env!("CARGO_PKG_VERSION")))
        .descr(concat!(
            "tutel\na minimalistic todo app for terminal enthusiasts"
        ))
        .footer("run without a subcommand to show the todo list")
        .for_parser(parser)
        .run()
}

fn new_project_command() -> OptionParser<Command> {
    let name = positional("name").optional();
    let force = short('f')
        .long("force")
        .help("force project creation")
        .switch();

    Info::default()
        .descr("create a new project in the current directory")
        .for_parser(construct!(Command::NewProject { name, force }))
}

fn add_task_command() -> OptionParser<Command> {
    let desc = positional("description")
        .many()
        .guard(|v| !v.is_empty(), "the task description is required")
        .map(|v| {
            let mut desc = String::new();
            let vlen = v.len();

            for (i, s) in v.iter().enumerate() {
                desc.push_str(&*s);
                if i < vlen - 1 {
                    desc.push(' ');
                }
            }
            desc
        });

    let completed = short('c')
        .long("completed")
        .help("mark the task as already completed")
        .switch();

    let due = short('d')
        .long("due")
        .help("set a timestamp when this task is due")
        .argument("timestamp")
        .optional();

    Info::default()
        .descr("add a new task. aliases: a")
        .for_parser(construct!(Command::AddTask {
            desc,
            completed,
            due
        }))
}

fn task_completed_command() -> OptionParser<Command> {
    let completed = short('!')
        .long("not")
        .help("mark the task as not being done")
        .switch()
        .map(|v| !v);
    // can unconditionally be mapped to TaskSelector::All since its value is only used if it is
    // present
    let all = short('a')
        .long("all")
        .help("select all tasks")
        .switch()
        .parse(|v| match v {
            true => Ok(TaskSelector::All),
            false => Err("all must be specified on its own"),
        });
    let selector = parse_indices().or_else(all);

    Info::default()
        .descr("mark a task as being done. aliases: d")
        .for_parser(construct!(Command::MarkCompletion(selector, completed)))
}

fn remove_task_command() -> OptionParser<Command> {
    let all = short('a')
        .long("all")
        .help("remove all tasks")
        .switch()
        .parse(|v| match v {
            true => Ok(TaskSelector::All),
            false => Err(""),
        });

    let cleanup = short('c')
        .long("cleanup")
        .help("remove all completed tasks")
        .switch()
        .parse(|v| match v {
            true => Ok(TaskSelector::Completed),
            false => Err(""),
        });

    let project = long("project")
        .help("remove the whole project file")
        .switch()
        .parse(|v| match v {
            true => Ok(Command::RemoveProject),
            false => Err(""),
        });

    let selector = parse_indices().or_else(all).or_else(cleanup);

    let parser = construct!(Command::RemoveTask(selector)).or_else(project);

    Info::default()
        .descr("remove a task from a project")
        .for_parser(parser)
}

fn parse_indices() -> Parser<TaskSelector> {
    positional("indices")
        .many()
        .guard(|v| !v.is_empty(), "one or more task indices are required")
        .parse::<_, _, String>(|v| {
            let mut indices = Vec::with_capacity(v.len());

            for x in v {
                indices.push(
                    x.parse::<u8>()
                        .map_err(|_| format!("not a valid index: {x}"))?,
                )
            }

            Ok(TaskSelector::Indexed(indices))
        })
}

fn edit_task_command() -> OptionParser<Command> {
    let index = positional("index").from_str::<u8>();

    let editor = env("EDITOR")
        .short('e')
        .long("editor")
        .help("the editor to use (default: $EDITOR)")
        .argument("editor");

    Info::default()
        .descr("edit an existing task. aliases: e")
        .for_parser(construct!(Command::EditTask(index, editor)))
}

fn print_completions_command() -> OptionParser<Command> {
    let shell = positional("shell");

    Info::default()
        .descr("print shell completions for the given shell")
        .for_parser(construct!(Command::PrintCompletion(shell)))
}
