use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod game;
use game::*;

mod config;
use config::*;

mod terminal;
use terminal::{input::*, screen::*};

mod app;
use app::*;

mod render;
use render::*;

mod error;
use error::*;

mod input;

use std::io::{Write, stdout};

fn main() {
    let screen = Screen::new();
    screen.init();
    let mut app = App::new();

    loop {
        screen.clear_screen().unwrap();
        screen.set_pos(0, 0).unwrap();

        render(&app).unwrap();

        stdout().flush().unwrap();
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match app.status {
                Status::Welcome => match key.code {
                    KeyCode::Char('q') => {
                        app.should_exit = true;
                    }
                    KeyCode::Char('e') => {
                        let mut g = Game::new(Config::easy());
                        g.generate();
                        app.game = Some(g);
                        app.status = Status::Game;
                    }
                    KeyCode::Char('n') => {
                        let mut g = Game::new(Config::normal());
                        g.generate();
                        app.game = Some(g);
                        app.status = Status::Game;
                    }
                    KeyCode::Char('h') => {
                        let mut g = Game::new(Config::hard());
                        g.generate();
                        app.game = Some(g);
                        app.status = Status::Game;
                    }
                    _ => {}
                },
                Status::Game => match key.code {
                    KeyCode::Char('q') => {
                        app.status = Status::Welcome;
                    }
                    KeyCode::Enter => {
                        let game = app.game.as_mut().ok_or(RenderError::NoGame).unwrap();
                        let res =
                            game.handle_enter(&app.input.content, &mut app.screen, &mut app.status);
                        if let Err(e) = res {
                            app.print(format!("{}\n", e).as_str());
                        }
                        app.input.clear();
                    }
                    KeyCode::Char(c) if c.is_numeric() || c == ' ' => {
                        app.input.content.push(c);
                    }
                    _ => {}
                },
                _ => match key.code {
                    KeyCode::Char('q') => {
                        app.should_exit = true;
                    }
                    _ => {}
                },
            }
        }

        if app.should_exit {
            break;
        }
    }

}
