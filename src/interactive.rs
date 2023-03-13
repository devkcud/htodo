use std::io::{stdin, self, Write};

pub struct Shell {
    category: String
}

impl Shell {
    pub fn new(category: &str) -> Shell {
        if !cfg!(target_os = "windows") {
            // Clear terminal screen
            // keep in mind that i tested it only on my main machine
            print!("{}c", 27 as char);
        }

        Shell { category: category.to_string() }
    }

    pub fn launch(&self) {
        println!("Launched as {}", self.category);

        let mut command: String = Default::default();

        'mainloop: loop {
            print!("> ");

            io::stdout().flush().unwrap();
            stdin().read_line(&mut command).unwrap();

            match command.trim() {
                "e" | "exit" => break 'mainloop,

                _ => (),
            }
        }
    }

    pub fn change_category(&mut self, new_category: &str) {
        self.category = new_category.to_string();
    }
}
