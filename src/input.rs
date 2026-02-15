use std::sync::Arc;

pub struct Input {
    pub content: String,
    pub error_msg: Arc<str>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            error_msg: "".into(),
        }
    }

    pub fn clear(&mut self) {
        self.content = String::new();
    }
}
