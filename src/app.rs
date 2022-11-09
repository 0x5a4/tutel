use bpaf::{construct, env, long, positional, short, OptionParser, Parser};

/// Indicates what Task(s) to select
#[derive(Debug, Clone)]
pub enum TaskSelector {
    Indexed(Vec<usize>),
    All,
    Completed,
}

/// The command to execute
#[derive(Debug, Clone)]
pub enum Command {
    Show,
    NewProject { name: Option<String>, force: bool },
    AddTask { desc: String, completed: bool },
    MarkCompletion(bool, TaskSelector),
    RemoveTask(TaskSelector),
    EditTask(String, usize),
    RemoveProject,
}

#[derive(Clone)]
pub struct App {
    pub cmd: Command,
}

fn parser() -> bpaf::OptionParser<App> {
    // Subcommands
    let new_cmd = new_project_command()
        .command("new")
        .help("create a new project");

    let add_cmd = add_task_command()
        .command("add")
        .short('a')
        .help("add a new task");

    let done_cmd = task_completed_command()
        .command("done")
        .short('d')
        .help("mark a task as being completed");

    let rm_cmd = remove_task_command().command("rm").help("remove a task");

    let edit_cmd = edit_task_command()
        .command("edit")
        .short('e')
        .help("edit an existing task");

    // TODO: completions cmd compat

    let cmd = construct!([new_cmd, add_cmd, done_cmd, rm_cmd, edit_cmd]).fallback(Command::Show);

    construct!(App { cmd })
        .to_options()
        .version(concat!("tutel v", env!("CARGO_PKG_VERSION")))
        .descr("tutel\na minimalistic todo app for terminal enthusiasts")
        .footer("run without a subcommand to show the todo list")
}

pub fn parse_cli() -> App {
    parser().run()
}

fn new_project_command() -> OptionParser<Command> {
    let name = positional("name").optional();
    let force = short('f')
        .long("force")
        .help("force project creation")
        .switch();

    construct!(Command::NewProject { force, name })
        .to_options()
        .descr("create a new project in the current directory")
}

fn add_task_command() -> OptionParser<Command> {
    let desc = positional::<String>("description")
        .many()
        .guard(|v| !v.is_empty(), "the task description is required")
        .map(|v| {
            let mut desc = String::new();
            let vlen = v.len();

            for (i, s) in v.iter().enumerate() {
                desc.push_str(s);
                if i < vlen - 1 {
                    desc.push(' ');
                }
            }
            "what".to_string()
        });

    let completed = short('c')
        .long("completed")
        .help("mark the task as already completed")
        .switch();

    construct!(Command::AddTask { completed, desc })
        .to_options()
        .descr("add a new task. aliases: a")
}

fn task_completed_command() -> OptionParser<Command> {
    let completed = short('!')
        .long("not")
        .help("mark the task as not being done")
        .flag(false, true);

    let all = short('a')
        .long("all")
        .help("select all tasks")
        .req_flag(TaskSelector::All);

    let selector = construct!([parse_indices(), all]);
    construct!(Command::MarkCompletion(completed, selector))
        .to_options()
        .descr("mark a task as being done. aliases: d")
}

fn remove_task_command() -> OptionParser<Command> {
    let all = short('a')
        .long("all")
        .help("remove all tasks")
        .req_flag(TaskSelector::All);

    let cleanup = short('c')
        .long("cleanup")
        .help("remove all completed tasks")
        .req_flag(TaskSelector::Completed);

    let project = long("project")
        .help("remove the whole project file")
        .req_flag(Command::RemoveProject);

    let remove_task = construct!([parse_indices(), all, cleanup]).map(Command::RemoveTask);

    construct!([remove_task, project])
        .to_options()
        .descr("remove a task from a project")
}

fn complete_indices(input: &Vec<String>) -> Vec<(String, Option<String>)> {
    let workdir = match std::env::current_dir() {
        Ok(x) => x,
        Err(_) => return Vec::new(),
    };

    let p = match tutel::load_project_rec(&workdir) {
        Ok(x) => x,
        Err(_) => return Vec::new(),
    };

    let mut res = Vec::new();

    let full = &input[..input.len() - 1];
    let active = input.last().unwrap();

    for task in p.data.tasks {
        let tid = task.index.to_string();
        if full.contains(&tid) {
            continue;
        }
        if tid.starts_with(active) {
            res.push((format!("{}", task.index), Some(task.desc.clone())));
        }
    }

    res
}

fn parse_indices() -> impl Parser<TaskSelector> {
    positional("indices")
        .some("one or more task indices are required")
        .complete(complete_indices)
        .parse::<_, _, String>(|v| {
            let mut indices = Vec::with_capacity(v.len());

            for x in v {
                indices.push(
                    x.parse::<usize>()
                        .map_err(|_| format!("not a valid index: {x}"))?,
                )
            }

            Ok(TaskSelector::Indexed(indices))
        })
}

fn edit_task_command() -> OptionParser<Command> {
    let index = positional::<usize>("index");

    let editor = env("EDITOR")
        .short('e')
        .long("editor")
        .help("the editor to use (default: $EDITOR)")
        .argument("editor");

    construct!(Command::EditTask(editor, index))
        .to_options()
        .descr("edit an existing task. aliases: e")
}
