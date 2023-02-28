#[allow(dead_code)]
mod todomanager;

#[allow(dead_code)]
mod terminal;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut commands: Vec<&String> = args.iter().filter(|x| !x.starts_with('-')).collect();
    let mut flags = args.iter().filter(|x| x.starts_with('-')).collect::<Vec<&String>>().into_iter();

    let term = terminal::Terminal::new(flags.find(|a| a.trim() == "-V" || a.trim() == "--verbose").is_some());
    term.log("Verbose mode active (-V || --verbose)");

    commands.remove(0);

    (commands.len() == 0).then(|| {
        term.err("Too few arguments (<1)");
        term.warn("Exited 1");
        term.help_menu(&String::new());
        std::process::exit(1);
    });

    term.log("Verifying command");

    match commands.remove(0).trim() {
        "h" | "help" => term.help_menu(commands.get(0).unwrap_or(&&String::new())),
        _ => term.help_menu(&String::new()),
    }

    term.warn("Exited 0");
}
