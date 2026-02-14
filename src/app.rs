use std::io::{stdout, Write};
use crate::game::*;
use crate::terminal::screen::*;
use crate::input::*;

#[derive(Clone, Copy)]
pub enum Status {
    Welcome,
    Game,
    Success,
    Failed,
}

pub struct App {
    pub screen: Screen,
    pub game: Option<Game>,
    pub should_exit: bool,
    pub status: Status,
    pub input: Input,
}

impl App {
    pub fn new() -> Self {
        let screen = Screen::new();
        Self {
            screen,
            game: None,
            should_exit: false,
            status: Status::Welcome,
            input: Input::new(),
        }
    }
    
    pub fn print(&self, s: &str) {
        self.screen.print(s.to_string()).unwrap();
        stdout().flush().unwrap();
    }
}
