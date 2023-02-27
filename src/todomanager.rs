use dirs;
use std::{fs, io::prelude::*};

// TODO: Create better ways to handle errors (also remove unwraps and expects)
// TODO: Add toggle_todo, get_all_todos functions
// TODO: Remove derive(clone)

#[derive(Clone)]
pub struct TodoFile {
    file_path: String,
    category:  String,
}

impl TodoFile {
    pub fn new(category: String) -> TodoFile {
        let folder = dirs::config_dir().unwrap().to_string_lossy().to_string();

        let todos_folder = format!("{}/htodo/todolists", folder);
        fs::create_dir_all(&todos_folder).expect("Couldn't create config dir.");

        let file_path = format!("{}/{}.list", todos_folder, category);
        fs::OpenOptions::new().write(true).create(true).open(&file_path).unwrap();

        TodoFile { file_path, category }
    }

    pub fn add_todo(self, todo: &str) {
        let todo = format!("no;{todo}");
        let mut file = fs::OpenOptions::new().append(true).read(true).open(self.file_path).expect("Couldn't open file");

        writeln!(file, "{todo}").expect("Couldn't write todo file");
    }

    pub fn remove_todo(self, index: usize) {
        let output = fs::read_to_string(&self.file_path).expect("Couldn't read file.");
        let mut lines = output.lines().collect::<Vec<&str>>();
        lines.remove(index);

        fs::write(self.file_path, lines.join("\n")).unwrap();
    }

    pub fn get_todo(self, index: usize) -> String {
        fs::read_to_string(&self.file_path).unwrap().lines().nth(index).expect("Index out of bounds")[3..].to_string()
    }
}

