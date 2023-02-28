#[allow(dead_code)]
mod todomanager;

#[allow(dead_code)]
mod terminal;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut commands: Vec<&String> = args.iter().filter(|x| !x.starts_with('-')).collect();
    let flags: Vec<&String> = args.iter().filter(|x| x.starts_with('-')).collect();

    let term = terminal::Terminal::new(flags.iter().find(|a| a.trim() == "-v" || a.trim() == "--verbose").is_some());

    commands.remove(0);

    (commands.len() == 0).then(|| std::process::exit(0));

    match commands.remove(0).trim() {
        "h" | "help" => term.help_menu(commands.get(0).unwrap_or(&&String::new())),
        _ => (),
    }
}
