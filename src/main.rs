#[allow(dead_code)]
mod todomanager;

fn main() {
    let todo = todomanager::TodoFile::new(String::from("General"));

    /*
     * Current TodoFile:
     * no;Hello, world 1!
     * no;Hello, world 2!
     * no;Hello, world 3!
     * no;Hello, world 4!
     */

    //todo.clone().remove_todo(2);
    todo.clone().toggle_todo(2);
}
