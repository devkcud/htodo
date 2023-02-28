use colored::Colorize;

#[derive(Debug)]
struct Command {
    name:        &'static str,
    description: &'static str,
    usage:       &'static str,
    alias:       &'static str,
}

const COMMAND_LIST: [Command; 1] = [
    Command {
        name:        "help",
        description: "Show the help menu or help for a specific command",
        usage:       "[command name]",
        alias:       "h",
    }
];

pub struct Terminal {
    is_verbose: bool,
}

impl Terminal {
    pub fn new(is_verbose: bool) -> Terminal {
        Terminal { is_verbose }
    }

    pub fn log(&self, msg: &str) {
        if self.is_verbose {
            println!("{}  {} {msg}", "LOG".bold().green(), "~>".magenta());
        }
    }

    pub fn warn(&self, msg: &str) {
        if self.is_verbose {
            println!("{} {} {msg}", "WARN".bold().yellow(), "~>".magenta());
        }
    }

    pub fn err(&self, msg: &str) {
        println!("{}  {} {msg}", "ERR".bold().red(), "~>".magenta());
    }

    pub fn throw_err(&self) {
        println!("{}  {} Error threw; exited 1", "ERR".bold().red(), "~>".magenta());
        std::process::exit(1);
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

