pub const HELP_MAIN: &'static str = "\
due 0.0.1
The task manager that embraces procrastination.


USAGE:
    due                                          View tasks
    due <description...> @ [time...] [date...]   Add a task
    due <SUBCOMMAND>                             Run a subcommand

ARGS:
    [description]...    Description of the task being added
    time... [date]...   When the task is due (see the `when` subcommand)

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    when    Information about timestamp arguments

EXAMPLES:

    # schedule some events
    due math homework @ 10 pm
    due schedule meeting @ noon tomorrow

    # edit events
    due edit 1 @ midnight
    due edit 1 s/homework/presentation
    due edit 2 schedule meeting with boss

    # learn about timestamps
    due when
    due when 1 pm tues
";

pub const HELP_EDIT: &'static str = "\
USAGE:
    due edit                        View this help
    due edit <id> @ <timestamp...>  Edit when a task is due 
    due edit <id> <description>     Update the task's description        
    due edit <id> s/old/new         Replace $old with $new in description        
    due edit <id> s/old/new/g       Replace all $old with $new in description
";
