#![allow(dead_code)]

use std::time::Instant;

use crate::app::*;
use crate::config::*;
use crate::error::GameError;
use crate::terminal::screen::*;
use crossterm::style::Stylize;
use rand::Rng;

#[derive(Clone, Copy)]
enum Item {
    Space,
    Mine,
    Number(i8),
}

pub struct Game {
    pub start: Instant,
    first: bool,
    config: Config,
    world: Vec<Vec<Item>>,
    board: Vec<Vec<bool>>,
    pub draw_mine: bool,
}

impl Game {
    pub fn new(cfg: Config) -> Self {
        // init world
        let world: Vec<Vec<Item>> = vec![vec![Item::Space; cfg.col]; cfg.row];
        // init screen
        // init board
        let board: Vec<Vec<bool>> = vec![vec![false; cfg.col]; cfg.row];

        Self {
            start: Instant::now(),
            first: true,
            config: cfg,
            world: world,
            board: board,
            draw_mine: false,
        }
    }

    pub fn generate(&mut self) {
        self.generate_mine();
        self.generate_number();
    }

    pub fn draw(&self, all: bool, screen: &Screen) {
        let cfg = &self.config;
        let world = &self.world;
        let board = &self.board;

        let mut h: String = if cfg.col < 10 {
            "   ".to_string()
                + (1..=cfg.col)
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("  ")
                    .as_str()
        } else {
            "   ".to_string()
                + (1..=10)
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("  ")
                    .as_str()
                + " "
                + (11..=cfg.col)
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .as_str()
        };
        h.push(' ');
        h.push('Y');
        h.push('\n');
        let color_h = h.dark_red().to_string();
        screen.print(color_h).unwrap();

        for i in 0..cfg.row {
            let mut line = (i + 1).to_string().cyan().to_string();
            if i + 1 < 10 {
                line.push(' ');
            }
            line.push(' ');

            for j in 0..cfg.col {
                let c;
                if !board[i][j] && !all {
                    c = '·';
                } else {
                    c = world[i][j].render();
                }
                let color_c = render_color(c);
                line += color_c.as_str();
                line.push(' ');
                line.push(' ');
            }
            line.push('\n');
            screen.print(line).unwrap();
        }

        screen.print("X\n\n".cyan().to_string()).unwrap();
    }

    fn generate_mine(&mut self) {
        let Config { col, row, mine } = self.config;
        let mut rng = rand::rng();
        let mut i = 0;
        while i < mine {
            let rd_col = rng.random_range(0..col);
            let rd_row = rng.random_range(0..row);

            if let Item::Space = self.world[rd_row][rd_col] {
                self.world[rd_row][rd_col] = Item::Mine;
                i += 1;
            } else {
                continue;
            }
        }
    }

    fn generate_mine_by_pos(&mut self, p_col: usize, p_row: usize) {
        let Config { col, row, mine } = self.config;
        let mut rng = rand::rng();
        let mut rd_col;
        let mut rd_row;

        let mut i = 0;
        while i < mine {
            rd_col = rng.random_range(0..col);
            rd_row = rng.random_range(0..row);
            if rd_col != p_col || rd_row != p_row {
                if let Item::Space = self.world[rd_row][rd_col] {
                    self.world[rd_row][rd_col] = Item::Mine;
                    i += 1;
                } else {
                    continue;
                }
            }
        }
    }

    fn generate_number(&mut self) {
        const DIRS: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let Config { col, row, .. } = self.config;

        for i in 0..row {
            for j in 0..col {
                let count = DIRS
                    .iter()
                    .map(|&(dx, dy)| (i as i32 + dx as i32, j as i32 + dy as i32))
                    .filter(|&(x, y)| (0..row as i32).contains(&x) && (0..col as i32).contains(&y))
                    .filter(|&(x, y)| matches!(self.world[x as usize][y as usize], Item::Mine))
                    .count();
                if let Item::Space = self.world[i][j] {
                    self.world[i][j] = Item::Number(count as i8);
                }
            }
        }
    }

    pub fn handle_enter(
        &mut self,
        input: &String,
        status: &mut Status,
    ) -> Result<(), GameError> {
        let cfg = self.config;
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            return Err(GameError::InvalidInput);
        }

        let x = parts[0].parse::<i32>()? - 1;

        let y = parts[1].parse::<i32>()? - 1;

        if self.first {
            self.world = vec![vec![Item::Space; cfg.col]; cfg.row];
            self.generate_mine_by_pos(y as usize, x as usize);
            self.generate_number();
        }

        self.first = false;

        if x >= 0 && x < cfg.row as i32 && y >= 0 && y < cfg.col as i32 {
            match self.world[x as usize][y as usize] {
                Item::Mine => {
                    self.draw_mine = true;
                    *status = Status::Failed;
                }
                Item::Number(_) => {
                    if self.board[x as usize][y as usize] {
                        return Err(GameError::AlreadyExploded);
                    }
                    self.board[x as usize][y as usize] = true;
                    self.spread(x, y);
                }
                _ => (),
            }
        } else {
            return Err(GameError::InvalidInput);
        }

        if self.judge() {
            self.draw_mine = true;
            *status = Status::Success;
        }
        Ok(())
    }

    pub fn spread(&mut self, i: i32, j: i32) {
        let direction: [i32; 3] = [-1, 0, 1];
        let Config { col, row, .. } = self.config;
        for x in 0..3 {
            for y in 0..3 {
                let px = i as i32 + direction[x];
                let py = j as i32 + direction[y];

                if px >= 0 && px < row as i32 && py >= 0 && py < col as i32 {
                    match self.world[px as usize][py as usize] {
                        Item::Number(num) => {
                            if num == 0 {
                                if !self.board[px as usize][py as usize] {
                                    self.board[px as usize][py as usize] = true;
                                    self.spread(px, py);
                                }
                            } else {
                                if !self.board[px as usize][py as usize] {
                                    self.board[px as usize][py as usize] = true;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    pub fn judge(&self) -> bool {
        let mut count = 0;
        let cfg = &self.config;
        let board = &self.board;
        for i in 0..cfg.row {
            for j in 0..cfg.col {
                if board[i][j] {
                    count += 1;
                }
            }
        }
        if (cfg.col * cfg.row) - count == cfg.mine as usize {
            return true;
        }
        false
    }
}

impl Item {
    pub fn render(&self) -> char {
        match self {
            Item::Space => '·',
            Item::Number(0) => ' ',
            Item::Number(num) => num.to_string().chars().next().unwrap(),
            Item::Mine => 'X',
        }
    }
}

fn render_color(c: char) -> String {
    match c {
        '1' => '1'.to_string().blue().to_string(),
        '2' => '2'.to_string().dark_green().to_string(),
        '3' => '3'.to_string().dark_red().to_string(),
        '4' => '4'.to_string().dark_blue().to_string(),
        '5' => '5'.to_string().to_string().dark_yellow().to_string(),
        '6' => '6'.to_string().dark_cyan().to_string(),
        '7' => '7'.to_string().black().to_string(),
        '8' => '8'.to_string().grey().to_string(),
        '·' => '·'.to_string().white().to_string(),
        'X' => 'X'.to_string().grey().to_string(),
        ' ' => ' '.to_string(),
        _ => "".to_string(),
    }
}
