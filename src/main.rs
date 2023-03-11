use std::{fs, ops::Sub};
use colored::Colorize;
use prettytable::{self, Table, format, row};

#[allow(dead_code)]
mod todomanager;

#[allow(dead_code)]
mod help;

#[allow(dead_code)]
mod terminal;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let commands: Vec<&String> = args.iter().filter(|x| !x.starts_with('-')).collect();
    let flags = args.iter().filter(|x| x.starts_with('-')).collect::<Vec<&String>>();

    let term = terminal::Terminal::new(flags.iter().find(|a| a.trim() == "-V" || a.trim() == "--verbose").is_some());
    term.warn("Verbose mode active (-V || --verbose)");

    let check_arg_len = |index: usize| {
        (commands.len() == index).then(|| {
            term.err(&format!("Too few arguments (<{index}), use help command"));
            term.dev("Error has been found; exit 1");
            std::process::exit(1);
        });
    };


    term.log("Initializing htodo configuration files");
    let todo = match todomanager::TodoFile::new(String::from("General")) {
        Ok(o) => o,
        _ => {
            term.err("Couldn't init required config dir/files");
            term.dev("Error has been found; exit 1");
            std::process::exit(1);
        },
    };

    term.log(&format!("Configuration file created/found at {}", todo.get_file_path().underline()));

    if commands.len() == 1 {
        let show_only_done = flags.iter().find(|a| a.trim() == "-y" || a.trim() == "--o-done").is_some();
        let show_only_todo = flags.iter().find(|a| a.trim() == "-n" || a.trim() == "--o-todo").is_some();

        let todos = fs::read_to_string(todo.get_file_path()).unwrap();

        if todos.lines().count() == 0 {
            term.err("Nothing to show. File is empty");

            term.warn("Exited 0");
            std::process::exit(0);
        }

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(row!["", "TASK".yellow().bold(), "STATUS".yellow().bold()]);

        let mut quantity_done = 0;

        'readloop: for (mut i, t) in todos.lines().enumerate() {
            i += 1;

            let prefix = &t[..3];
            let content = &t[3..];

            match prefix {
                "ye;" => {
                    if show_only_todo { continue 'readloop; }

                    table.add_row(row![i.to_string().yellow().bold(), content.magenta().dimmed().italic().strikethrough(), "Done".green()]);
                    quantity_done += 1;
                },
                "no;" => {
                    if show_only_done { continue 'readloop; }

                    table.add_row(row![i.to_string().yellow().bold(), content.magenta(), "Todo".blue()]);
                },
                _ => (),
            }
        };

        table.printstd();
        let total_shown = table.to_string().lines().count().sub(2);

        if !show_only_todo && !show_only_done {
            println!("\n{} {}/{}",
                "SIZE:".yellow().bold(),
                if quantity_done == total_shown { quantity_done.to_string().green() } else { quantity_done.to_string().red() },
                total_shown.to_string().green()
            );
        } else {
            println!("\n{} {}", "SIZE:".yellow().bold(), total_shown);
        }

        term.warn("Exited 0");
        std::process::exit(0);
    }

    term.log("Verifying command");

    let command = commands.get(1).unwrap().trim();
    let arg1 = commands.get(2);

    match command {
        "h" | "help" => help::help_menu(arg1.unwrap_or(&&String::new())),

        "a" | "add" => {
            check_arg_len(2);
            match todo.add_todo(arg1.unwrap()) {
                Ok(_) => {
                    term.log(&format!("Added {} to todo file", arg1.unwrap().green()));
                },
                Err(_) => {
                    term.err(&format!("Couldn't add {}", arg1.unwrap().red()));
                    term.dev("Error has been found; exit 1");
                    std::process::exit(1);
                },
            }
        }

        "r" | "remove" => {
            check_arg_len(2);

            let mut index = match arg1.unwrap().parse::<usize>() {
                Ok(o) => o,
                Err(_) => {
                    term.err(&format!("Couldn't parse {} to integer.", arg1.unwrap().red()));
                    term.dev("Error has been found; exit 1");
                    std::process::exit(1);
                },
            };

            let file_content = fs::read_to_string(todo.get_file_path()).unwrap();

            if file_content.lines().count() == 0 {
                term.err("File is empty");
                term.dev("Error has been found; exit 1");
                std::process::exit(1);
            }

            term.warn("Clamping index value to match the correct file size");
            index = index.clamp(1, file_content.lines().count());

            match todo.remove_todo(index.wrapping_sub(1)) {
                Ok(_) => {
                    term.log(&format!("Removed index {} from todo file", index));
                },
                Err(_) => {
                    term.err(&format!("Couldn't remove index {}", index));
                    term.dev("Error has been found; exit 1");
                    std::process::exit(1);
                },
            }
        }

        "t" | "toggle" => {
            check_arg_len(2);

            let mut index = match arg1.unwrap().parse::<usize>() {
                Ok(o) => o,
                Err(_) => {
                    term.err(&format!("Couldn't parse {} to integer.", arg1.unwrap().red()));
                    term.dev("Error has been found; exit 1");
                    std::process::exit(1);
                },
            };

            let file_content = fs::read_to_string(todo.get_file_path()).unwrap();

            if file_content.lines().count() == 0 {
                term.err("File is empty");
                term.dev("Error has been found; exit 1");
                std::process::exit(1);
            }

            term.warn("Clamping index value to match the correct file size");
            index = index.clamp(1, file_content.lines().count());

            match todo.toggle_todo(index.wrapping_sub(1)) {
                Ok(_) => {
                    term.log(&format!("Toggled index {} from todo file", index));
                },
                Err(_) => {
                    term.err(&format!("Couldn't toggle index {}", index));
                    term.dev("Error has been found; exit 1");
                    std::process::exit(1);
                },
            }
        }

        "g" | "get" => {
            check_arg_len(2);

            let mut index = match arg1.unwrap().parse::<usize>() {
                Ok(o) => o,
                Err(_) => {
                    term.err(&format!("Couldn't parse {} to integer.", arg1.unwrap().red()));
                    term.dev("Error has been found; exit 1");
                    std::process::exit(1);
                },
            };

            let file_content = fs::read_to_string(todo.get_file_path()).unwrap();

            if file_content.lines().count() == 0 {
                term.err("File is empty");
                term.dev("Error has been found; exit 1");
                std::process::exit(1);
            }

            term.warn("Clamping index value to match the correct file size");
            index = index.clamp(1, file_content.lines().count());

            match todo.get_todo(index.wrapping_sub(1)) {
                Some(o) => {
                    term.log(&format!("Got index {} from todo file", index));
                    println!("{}", o);
                },
                None => {
                    term.err(&format!("Couldn't get index {}", index));
                    term.dev("Error has been found; exit 1");
                    std::process::exit(1);
                },
            }
        }

        _ => help::help_menu(command),
    }

    term.warn("Exited 0");
}
