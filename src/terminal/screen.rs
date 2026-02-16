#![allow(dead_code)]
use std::{
    io::{Error, Write, stdout},
    time::Duration,
};

use crossterm::{
    Command,
    cursor::MoveTo,
    queue,
    style::{Print, Stylize},
    terminal::{self, Clear, ClearType},
};

use super::input::input;
use crate::config::*;

pub struct Screen {
    width: u16,
    height: u16,
}

impl Screen {
    pub fn new() -> Self {
        let (w, h) = terminal::size().unwrap();
        Self {
            width: w,
            height: h,
        }
    }

    pub fn init(&self) {
        self.clear_screen().unwrap();
        self.set_pos(0, 0).unwrap();
        stdout().flush().unwrap();
    }

    pub fn clear_screen(&self) -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All)).unwrap();
        self.set_pos(0, 0).unwrap();
        stdout().flush().unwrap();

        Ok(())
    }

    pub fn print(&self, str: String) -> Result<(), Error> {
        Self::queue_command(Print(str))?;
        Ok(())
    }

    pub fn set_pos(&self, x: u16, y: u16) -> Result<(), Error> {
        Self::queue_command(MoveTo(x, y))?;
        Ok(())
    }

    fn flash() {}

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn success(&self, dura: Duration) -> Result<(), Error> {
        self.print("Success\n".dark_green().to_string())?;
        let p = format!("use time:{:?}", dura);
        self.print(p.dark_cyan().to_string())?;
        self.print("\n\n\n\n".to_string())?;
        stdout().flush()?;
        self.after()?;
        Ok(())
    }

    pub fn after(&self) -> Result<(), Error> {
        self.print("press c to continue\n".to_string())?;
        self.print("press q to quit".to_string())?;

        stdout().flush()?;
        Ok(())
    }

    pub fn die(&self) -> Result<(), Error> {
        self.print("You Die !!!!!".dark_red().to_string())?;
        self.print("\n\n\n\n".to_string())?;
        stdout().flush()?;
        self.after()?;
        Ok(())
    }
}
