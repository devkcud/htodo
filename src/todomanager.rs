use dirs;
use std::{fs, io::{prelude::*, self}};

pub struct TodoFile {
    file_path:    String,
    category:     String,

    pub todos_folder: String,
}

impl TodoFile {
    pub fn new(category: String) -> Result<TodoFile, io::Error> {
        let folder = dirs::config_dir().unwrap().to_string_lossy().to_string();

        let todos_folder = format!("{}/htodo/todolists", folder);
        let file_path = format!("{}/{}.list", todos_folder, category);

        fs::create_dir_all(&todos_folder)?;
        fs::OpenOptions::new().write(true).create(true).open(&file_path)?;

        Ok(TodoFile { todos_folder, file_path, category })
    }

    pub fn add_todo(&self, todo: &str) -> Result<(), io::Error> {
        let todo = format!("no;{todo}");
        let mut file = fs::OpenOptions::new().append(true).open(&self.file_path)?;

        writeln!(file, "{todo}")
    }

    pub fn remove_todo(&self, index: usize) -> Result<(), io::Error> {
        let output = fs::read_to_string(&self.file_path)?;
        let mut lines = output.lines().collect::<Vec<&str>>();
        lines.remove(index);

        fs::write(&self.file_path, lines.join("\n"))
    }

    pub fn toggle_todo(&self, index: usize) -> Result<(), io::Error> {
        let file_content = fs::read_to_string(&self.file_path)?;

        let new_content = if let Some(line) = file_content.lines().nth(index) {
            let new_line = format!("{}{}", if line.starts_with("no;") { "ye;" } else { "no;" }, &line[3..]);
            file_content.replacen(line, &new_line, 1)
        } else {
            file_content
        };

        fs::write(&self.file_path, new_content)?;
        Ok(())
    }

    pub fn get_todo(&self, index: usize) -> Option<String> {
        let content = fs::read_to_string(&self.file_path).ok()?;
        let mut lines = content.lines();

        lines.nth(index).map(|s| s[3..].to_owned())
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }
}

