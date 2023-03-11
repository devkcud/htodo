use std::env::current_exe;

use colored::Colorize;
use prettytable::{Table, format, row};
use rand::Rng;

struct Command {
    name:        &'static str,
    description: &'static str,
    usage:       &'static str,
    alias:       &'static str,
}

struct Flag {
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

const FLAG_LIST: [Flag; 3] = [
    Flag {
        name:        "o-done",
        description: "Only show done tasks",
        usage:       "",
        alias:       "y",
    },
    Flag {
        name:        "o-todo",
        description: "Only show todo tasks",
        usage:       "",
        alias:       "n",
    },
    Flag {
        name:        "verbose",
        description: "Show more logging",
        usage:       "",
        alias:       "V",
    },
];

const EXAMPLES: [&str; 11] = [
    "toggle 2",
    "toggle 3 -V",
    "toggle 5",
    "add \"Hello, world!\"",
    "remove 1",
    "remove 2",
    "",
    "-y",
    "-n",
    "--o-done",
    "--verbose",
];

pub fn help_menu(command_name: &str) {
    if !command_name.is_empty() {
        let command = COMMAND_LIST.into_iter().find(|c| c.name == command_name || c.alias == command_name);

        return match command {
            Some(s) => {
                let mut help_table = Table::new();
                help_table.set_format(*format::consts::FORMAT_CLEAN);
                help_table.set_titles(row!["NAME".yellow().bold(), "ALIAS".yellow().bold(), "DESCRIPTION".yellow().bold(), "USAGE".yellow().bold()]);

                help_table.add_row(row![s.name.blue(), s.alias.green(), s.description, format!("{} {}", s.name.blue(), s.usage.magenta())]);

                help_table.printstd();
            },

            None => println!("{}{}Not able to find any command with the name: {}", "ERROR".bold().red(), crate::terminal::SEPARATOR.magenta(), command_name.bold()),
        }
    }

    println!("{}\n{} {}\n", "- COMMANDS:".yellow().bold(), " HEY: ".red().bold().on_black(), "If there is no command, the todo list will be shown ;)".italic());
    let mut help_table = Table::new();
    help_table.set_format(*format::consts::FORMAT_CLEAN);
    help_table.set_titles(row!["NAME".yellow().bold(), "ALIAS".yellow().bold(), "DESCRIPTION".yellow().bold(), "USAGE".yellow().bold()]);

    for command in COMMAND_LIST {
        help_table.add_row(row![command.name.blue(), command.alias.green(), command.description, format!("{} {}", command.name.blue(), command.usage.magenta())]);
    }

    help_table.printstd();

    println!("\n{}\n", "- FLAGS:".yellow().bold());
    let mut flag_table = Table::new();
    flag_table.set_format(*format::consts::FORMAT_CLEAN);
    flag_table.set_titles(row!["NAME".yellow().bold(), "ALIAS".yellow().bold(), "DESCRIPTION".yellow().bold(), "USAGE".yellow().bold()]);

    for flag in FLAG_LIST {
        flag_table.add_row(row!["--".to_string() + flag.name.blue().to_string().as_str(), "-".to_string() + flag.alias.green().to_string().as_str(), flag.description, format!("--{}{}", flag.name.blue(), if flag.usage != "" { format!("={}", flag.usage.magenta()) } else { "".to_string() })]);
    }

    flag_table.printstd();

    let mut random_value = rand::thread_rng();
    println!("\n{} {} {}", "- EXAMPLE:".yellow().bold(), current_exe().unwrap().to_str().unwrap().split('/').last().unwrap(), EXAMPLES[random_value.gen_range(0..(EXAMPLES.len() - 1))]);
}
