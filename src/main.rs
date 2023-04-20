use std::fs;
use colored::{Colorize, control};

#[allow(dead_code)]
mod todomanager;

#[allow(dead_code)]
mod help;

#[allow(dead_code)]
mod utils;

#[allow(dead_code)]
mod terminal;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let commands: Vec<&String> = args.iter().filter(|x| !x.starts_with('-')).collect();
    let flags = args.iter().filter(|x| x.starts_with('-')).collect::<Vec<&String>>();

    let show_only_done = flags.iter().find(|a| a.trim() == "-y" || a.trim() == "--o-done").is_some();
    let show_only_todo = flags.iter().find(|a| a.trim() == "-n" || a.trim() == "--o-todo").is_some();

    if flags.iter().find(|a| a.trim() == "-nc" || a.trim() == "--no-color").is_some() {
        control::set_override(false);
    }

    let term = terminal::Terminal::new(flags.iter().find(|a| a.trim() == "-V" || a.trim() == "--verbose").is_some());
    term.warn("Verbose mode active (-V || --verbose)");

    let check_arg_len = |index: usize| {
        (commands.len() == index).then(|| {
            term.err(&format!("Too few arguments (<{index}), use help command"));
            term.dev("Error has been found; exit 1");
            std::process::exit(1);
        });
    };

    let mut category = String::from("General");

    match args.iter().find(|x| x.trim().starts_with("-c") || x.trim().starts_with("--category")) {
        Some(s) => {
            if s.find("=").is_none() {
                term.warn("No category found after -c or --category; Skipping");
            } else {
                let text = s.split("=").collect::<Vec<&str>>()[1];

                if text.trim() != "" {
                    let normalized = regex::Regex::new(r"[^0-9a-zA-Z]").unwrap().replace_all(text, "");
                    category = normalized.to_string();
                    term.log(&format!("Using category {}", category));
                } else {
                    term.warn("No category found after -c= or --category=; Skipping");
                }
            }
        }

        None => {
            term.log("No category found; Using default General");
        },
    }

    term.log("Initializing htodo configuration files");
    let todo = match todomanager::TodoFile::new(String::from(&category)) {
        Ok(o) => o,
        _ => {
            term.err("Couldn't init required config dir/files");
            term.dev("Error has been found; exit 1");
            std::process::exit(1);
        },
    };

    term.log(&format!("Configuration file created/found at {}", todo.get_file_path().underline()));
    term.log("Verifying command");

    if commands.len() == 1 {
        let todos = fs::read_to_string(todo.get_file_path()).unwrap();

        utils::show_todo_list(&todos, &term, &category, show_only_done, show_only_todo);

        term.warn("Exited 0");
        std::process::exit(0);
    }

    let command = commands.get(1).unwrap().trim();
    let arg1 = commands.get(2);

    match command {
        "h" | "help" => {
            help::help_menu(arg1.unwrap_or(&&String::new()));

            term.warn("Exited 0");
            std::process::exit(0);
        }

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

            term.warn("Exited 0");
            std::process::exit(0);
        }

        "clear" => {
            term.warn(&format!("Removing {}", todo.get_file_path().green()));

            let mut answer = term.question(&format!("Deleting '{}'. Are you sure? {} ", todo.get_file_path().green(), "y/N".yellow())).to_lowercase();
            if answer.trim() == "" { answer = String::from("n"); }

            let answer = answer.trim();

            term.log(&format!("Got answer: {}", answer));

            if answer == "y" {
                fs::remove_file(todo.get_file_path()).unwrap();
                term.warn(&format!("Removed {}", todo.get_file_path().green()));
            } else {
                term.err("Aborting...");
            }

            term.warn("Exited 0");
            std::process::exit(0);
        }

        "e" | "edit" => {
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

            check_arg_len(3);

            term.log("Editing todo to {}");
            todo.edit_todo(index, commands.get(3).unwrap().to_string()).unwrap();
        }

        "c" | "categories" => {
            let mut files: Vec<String> = vec![];

            for file in fs::read_dir(&todo.todos_folder).unwrap() {
                let file = file.unwrap().path().display().to_string();
                files.push(file.split("/").last().unwrap().replace(".list", "").blue().to_string());
            }

            files.sort();

            println!("{} (case-sensitive)\n> {}", "CATEGORIES:".yellow().bold(), files.join("\n> "));

            term.warn("Exited 0");
            std::process::exit(0);
        }

        _ => {
            help::help_menu(command);
            std::process::exit(1);
        }
    }

    utils::show_todo_list(&fs::read_to_string(todo.get_file_path()).unwrap(), &term, &category, show_only_done, show_only_todo);

    term.warn("Exited 0");
}
