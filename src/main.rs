#[allow(dead_code)]
mod filemanager;

fn main() {
    filemanager::TodoFile::new(String::from("General"));
}
