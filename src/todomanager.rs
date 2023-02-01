use std::{ fs::{OpenOptions, create_dir_all, read_to_string}, io::{ Error, prelude::* } };

use dirs;
use regex::Regex;

#[derive(Clone)]
pub struct TodoFile {
    file_path: String,
    category:  String,
}

// TODO: Create better ways to handle errors

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

    pub fn get_all_todos(self) -> String {
        let mut output = String::new();

        for line in self.read_to_string().lines() {
            output.push_str(&format!("{}\n", &line[3..]));
        }

        return output.trim().to_string();
    } 

    pub fn get_single_todo(self, index: usize) -> String {
        let index = index.wrapping_sub(1).clamp(0, self.clone().read_to_string().lines().count() - 1);

        let s = self.read_to_string();
        let strict = &s.lines().nth(index).expect("Index out of bounds")[3..];

        strict.to_string()
        
    }

    pub fn add_todo(self, todo: &'static str) {
        // Template: <done?>;<todo>
        // 'ye;Do something' -> Done = true
        // 'no;Do something' -> Done = false
        
        let todo = format!("no;{}", todo);
        let mut file = self.open_as_obj(false).expect("Couldn't open todo file");

        writeln!(file, "{todo}").expect("Couldn't write todo file");
    }

    // TODO: remove_todo function

    fn open_as_obj(self, readonly: bool) -> Result<std::fs::File, Error> {
        if readonly {
            return OpenOptions::new().read(true).open(self.file_path);
        }

        OpenOptions::new().append(true).read(true).open(self.file_path)
    }

    fn read_to_string(self) -> String {
        read_to_string(self.file_path).expect("Couldn't read todo file").trim().to_string()
    }
}

