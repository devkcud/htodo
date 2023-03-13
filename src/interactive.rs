pub struct Shell {
    category: String
}

impl Shell {
    pub fn new(override_category: &str) -> Shell {
        Shell { category: override_category.to_string() }
    }

    pub fn launch(&self) {
        println!("Launched as {}", self.category);
    }

    fn change_category(mut self, new_category: &str) {
        self.category = new_category.to_string();
    }
}
