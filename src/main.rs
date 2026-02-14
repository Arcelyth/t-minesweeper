mod game;
use game::*;

mod config;
mod terminal;
use terminal::{screen::*, input::*};

use std::io::{Write, stdout};

fn main() {
    let screen = Screen::new();
    screen.init();
    loop {
        let cfg = screen.choose();

        let mut game = Game::new(cfg);

        game.generate();
        game.draw(false);
        game.run();
        let str = input();
        screen.clear_screen().unwrap();
        screen.set_pos(0, 0).unwrap();
        stdout().flush().unwrap();

        if str == "c".to_string() {
            continue;
        } else if str == "q".to_string() {
            break;
        }
    }
}
