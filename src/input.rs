pub struct Input {
    pub content: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.content = String::new();
    }
}
