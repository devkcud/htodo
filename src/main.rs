#[allow(dead_code)]
mod todomanager;

fn main() {
    let todo = todomanager::TodoFile::new(String::from("General"));

    todo.clone().remove_todo(0);
}
