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

    println!("{}", todo.clone().get_all_todos());
    println!("{}", todo.clone().get_single_todo(1)); // Expect 'Hello, world 1!'
    println!("{}", todo.clone().get_single_todo(99)); // Expect 'Hello, world 4!' -> wrapping up
}
