use colored::Colorize;

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

    pub fn dev(&self, msg: &str) {
        if self.is_verbose {
            println!("{}{}{msg}", "D_ERR".dimmed(), SEPARATOR.magenta());
        }
    }
}

