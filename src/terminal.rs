use colored::Colorize;

#[derive(Debug)]
struct Command {
    name:        &'static str,
    description: &'static str,
    usage:       &'static str,
    alias:       &'static str,
}

const COMMAND_LIST: [Command; 4] = [
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
];

pub struct Terminal {
    is_verbose: bool,
}

const SEPARATOR: &str = " * ";

impl Terminal {
    pub fn new(is_verbose: bool) -> Terminal {
        Terminal { is_verbose }
    }

    pub fn log(&self, msg: &str) {
        if self.is_verbose {
            println!("{}  {}{msg}", "LOG".bold().green(), SEPARATOR.magenta());
        }
    }

    pub fn warn(&self, msg: &str) {
        if self.is_verbose {
            println!("{} {}{msg}", "WARN".bold().yellow(), SEPARATOR.magenta());
        }
    }

    pub fn err(&self, msg: &str) {
        println!("{}{}{msg}", "ERROR".bold().red(), SEPARATOR.magenta());
    }

    pub fn dev(&self, msg: &str) {
        if self.is_verbose {
            println!("{}{}{msg}", "D_ERR".dimmed(), SEPARATOR.magenta());
        }
    }

    pub fn help_menu(&self, command_name: &str) {
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
}

