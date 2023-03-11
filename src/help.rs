#[derive(Debug)]
struct Command {
    name:        &'static str,
    description: &'static str,
    usage:       &'static str,
    alias:       &'static str,
}

const COMMAND_LIST: [Command; 5] = [
    Command {
        name:        "help",
        description: "Show the help menu or help for a specific command",
        usage:       "[command name]",
        alias:       "h",
    },
    Command {
        name:        "add",
        description: "Add a todo to the list",
        usage:       "<command name>",
        alias:       "a",
    },
    Command {
        name:        "remove",
        description: "Remove a todo to the list",
        usage:       "<todo index>",
        alias:       "r",
    },
    Command {
        name:        "toggle",
        description: "Toggle a todo on the list",
        usage:       "<todo index>",
        alias:       "t",
    },
    Command {
        name:        "get",
        description: "Get a todo on the list",
        usage:       "<todo index>",
        alias:       "g",
    },
];

// Finish help command
pub fn help_menu(command_name: &str) {
    if !command_name.is_empty() {
        let command = COMMAND_LIST.into_iter().find(|c| c.name == command_name);

        return match command {
            Some(s) => {
                println!("Usage: {} {}", s.name, s.usage);
            },

            None => {
                println!("Couldn't find any command with the name: {}", command_name);
            },
        }
    }
}
