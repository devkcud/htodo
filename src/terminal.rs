use colored::Colorize;
use rustyline::DefaultEditor;

pub struct Terminal {
    is_verbose: bool,
}

pub const SEPARATOR: &str = " * ";

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

    pub fn question(&self, msg: &str) -> String {
        let mut rl = DefaultEditor::new().unwrap();
        let answer = rl.readline(&format!("{}    {}{msg}", "?".bold().blue(), SEPARATOR.magenta()));
        match answer {
            Ok(o) => o,

            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("{}{}CTRL-C", "ERROR".bold().red(), SEPARATOR.magenta());
                return String::new();
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("{}{}CTRL-D", "ERROR".bold().red(), SEPARATOR.magenta());
                return String::new();
            }
            Err(err) => {
                println!("{}{}{err}", "ERROR".bold().red(), SEPARATOR.magenta());
                std::process::exit(1);
            }
        }

    }

    pub fn dev(&self, msg: &str) {
        if self.is_verbose {
            println!("{}{}{msg}", "D_ERR".dimmed(), SEPARATOR.magenta());
        }
    }
}

