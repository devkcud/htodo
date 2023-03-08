use std::fs;
use colored::Colorize;

#[allow(dead_code)]
mod todomanager;

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

    // TODO: Show the todo list when no args are passed
    check_arg_len(1);

    term.log("Initializing htodo configuration files");
    let todo = match todomanager::TodoFile::new(String::from("General")) {
        Ok(o) => o,
        _ => {
            term.err("Couldn't init required config dir/files");
            term.dev("Error has been found; exit 1");
            std::process::exit(1);
        },
    };

    term.log(&format!("New configuration file created/found at {}", todo.get_file_path().underline()));

    term.log("Verifying command");

    let command = commands.get(1).unwrap().trim();
    let arg1 = commands.get(2);


    match command {
        "h" | "help" => term.help_menu(arg1.unwrap_or(&&String::new())),

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

        _ => term.help_menu(""),
    }

    term.warn("Exited 0");
}
