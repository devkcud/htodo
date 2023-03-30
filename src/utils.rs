use crate::terminal::Terminal;
use colored::Colorize;
use prettytable::{Table, format, row};
use std::ops::Sub;

pub fn show_todo_list(todos: &str, term: &Terminal, category: &str, show_only_done: bool, show_only_todo: bool, simple: bool) {
    if todos.lines().count() == 0 {
        term.err("Nothing to show. File is empty");

        term.warn("Exited 0");
        std::process::exit(0);
    }

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    if !simple {
        println!("{} {}\n", "SELECTED CATEGORY:".yellow().bold(), category.to_string());
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["", "TASK".yellow().bold(), "STATUS".yellow().bold()]);
    }

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

    if simple { return; }

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
}
