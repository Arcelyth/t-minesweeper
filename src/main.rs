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
                Status::Game => match key.code {
                    KeyCode::Char('q') => {
                        app.status = Status::Welcome;
                    }
                    KeyCode::Enter => {
                        let game = app.game.as_mut().ok_or(RenderError::NoGame).unwrap();
                        let res = game.handle_enter(&app.input.content, &mut app.status);
                        if let Err(e) = res {
                            app.input.error_msg = format!("{:?}", e).into();
                        } else {
                            app.input.error_msg = "".into();
                        }
                        app.input.clear();
                    }
                    KeyCode::Char(c) if c.is_numeric() || c == ' ' => {
                        app.input.content.push(c);
                    }
                    _ => {}
                },
                _ => match key.code {
                    KeyCode::Char(c) => {
                        app.input.content.push(c);
                    }

                    KeyCode::Enter => {
                        handle_command(&mut app);
                        app.input.clear();
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

fn handle_command(app: &mut App) {
    match app.input.content.trim() {
        "q" => {
            match app.status {
                Status::Welcome => {
                    app.should_exit = true;
                }
                _ => {
                    app.status = Status::Welcome;
                }
            }
        }
        "e" => {
            let mut g = Game::new(Config::easy());
            g.generate();
            app.game = Some(g);
            app.status = Status::Game;
        }
        "n" => {
            let mut g = Game::new(Config::normal());
            g.generate();
            app.game = Some(g);
            app.status = Status::Game;
        }
        "h" => {
            let mut g = Game::new(Config::hard());
            g.generate();
            app.game = Some(g);
            app.status = Status::Game;
        }
        _ => {}
    }
}
