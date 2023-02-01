use std::{ fs::{OpenOptions, create_dir_all}, io::Error };

use dirs;
use regex::Regex;

pub struct TodoFile {
    file_path: String,
    category:  String,
}

impl TodoFile {
    pub fn new(category: String) -> TodoFile {
        let file_path = dirs::config_dir().unwrap();
        let mut file_path = file_path.to_string_lossy().to_string();

        if !file_path.ends_with("/") && !file_path.ends_with("\\") {
            file_path.push('/');
        }

        file_path = format!("{}htodo/todolists/{}/", file_path, if !category.is_empty() { category.to_string() } else { ".".to_string() });

        // TODO: Reduce the amount of regex
        let mut re = Regex::new(r"(?i)/+").unwrap();
        let normalized = re.replace_all(&file_path, "/");

        re = Regex::new(r"/\./").unwrap();
        let normalized = re.replace_all(&normalized, "/");

        re = Regex::new(r"^\./").unwrap();
        let normalized = re.replace_all(&normalized, "");

        re = Regex::new(r"/[^/]+/\.\./").unwrap();
        let normalized = re.replace_all(&normalized, "/").to_string();

        create_dir_all(&normalized).expect("Couldn't create config dir.");

        file_path = format!("{}todo.list", normalized);

        OpenOptions::new().write(true).create(true).open(&file_path).unwrap();

        TodoFile { file_path, category }
    }

    pub fn open_as_obj(self) -> Result<std::fs::File, Error> {
        OpenOptions::new().write(true).read(true).open(self.file_path)
    }
}
