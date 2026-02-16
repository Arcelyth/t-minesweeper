use crossterm::{
    style::Stylize,
};
use std::time::Instant;


use crate::app::*;
use crate::error::*;
use crate::screen::*;

pub fn render(app: &App) -> Result<(), RenderError> {
    let box_x = 0;
    let mut box_y = 2;
    let box_w = 40;
    let box_h = 3;
    match app.status {
        Status::Welcome => {
            app.print(&get_banner());
            app.print("\n\n\n\n");
            app.print(&get_manual());
            app.print(&format!("{}\n", app.input.error_msg).dark_red().to_string());
            box_y += 18;
        }
        Status::Game => {
            let game = app.game.as_ref().ok_or(RenderError::NoGame)?;
            game.draw(game.draw_mine, &app.screen);
            app.print("\n\n\n");
            app.print(&"Input position: <X> <Y>\n".green().to_string());
            app.print(&"Enter q back to the menu\n".green().to_string());
            app.print(&format!("{}\n", app.input.error_msg).dark_red().to_string());
            box_y += game.config.row + 8;
        }
        Status::Success => {
            let now = Instant::now();
            let game = app.game.as_ref().ok_or(RenderError::NoGame)?;
            let dura = now - game.start; 
            game.draw(game.draw_mine, &app.screen);
            app.print("\n\n\n\n");
            app.print(&"You Win!\n".green().to_string());
            app.print(&format!("Use time: {}\n", format_duration(dura)).green().to_string());
            app.print(&"Enter q back to the menu\n".green().to_string());
            box_y += game.config.row + 9;
        }
        Status::Failed => {
            let game = app.game.as_ref().ok_or(RenderError::NoGame)?;
            game.draw(game.draw_mine, &app.screen);
            app.print("\n\n\n\n");
            app.print(&"You Lose!\n".red().to_string());
            app.print(&"Enter q back to the menu\n".red().to_string());
            box_y += game.config.row + 8;
        }
    }
    draw_box(&app.screen, box_x, box_y as u16, box_w, box_h);

    draw_text_in_box(&app.screen, box_x, box_y as u16, box_w, &app.input.content);
    Ok(())
}

pub fn draw_box(screen: &Screen, x: u16, y: u16, width: u16, height: u16) {
    // top border
    screen.set_pos(x, y).unwrap();
    screen.print("┌".into()).unwrap();
    screen.print("─".repeat((width - 2) as usize)).unwrap();
    screen.print("┐".into()).unwrap();

    // sides
    for i in 1..height - 1 {
        screen.set_pos(x, y + i).unwrap();
        screen.print("│".into()).unwrap();

        screen.set_pos(x + width - 1, y + i).unwrap();
        screen.print("│".into()).unwrap();
    }

    // bottom border
    screen.set_pos(x, y + height - 1).unwrap();
    screen.print("└".into()).unwrap();
    screen.print("─".repeat((width - 2) as usize)).unwrap();
    screen.print("┘".into()).unwrap();
}

pub fn draw_text_in_box(screen: &Screen, x: u16, y: u16, width: u16, text: &str) {
    screen.set_pos(x + 1, y + 1).unwrap();

    let max_len = (width - 2) as usize;
    let clipped: String = text.chars().take(max_len).collect();

    screen.print(clipped).unwrap();
}

fn get_manual() -> String {
    format!("{}\n{}\n{}\n{}\n{}\n", 
        "Enter e to select EASY mode (8 x 8 x 10)".green(),
        "Enter n to select NORMAL mode (16 x 16 x 40)".blue(),
        "Enter h to select HARD mode (16 x 30 x 99)".red(),
        "Enter c:<width> <height> <mines> to custom size and mine's number mode".yellow(),
        "Enter q to QUIT game".magenta(),
    )
}

fn get_banner() -> String {
    r#"
████████╗   ███╗   ███╗██╗███╗   ██╗███████╗███████╗██╗    ██╗███████╗███████╗██████╗ ███████╗██████╗ 
╚══██╔══╝   ████╗ ████║██║████╗  ██║██╔════╝██╔════╝██║    ██║██╔════╝██╔════╝██╔══██╗██╔════╝██╔══██╗
   ██║█████╗██╔████╔██║██║██╔██╗ ██║█████╗  ███████╗██║ █╗ ██║█████╗  █████╗  ██████╔╝█████╗  ██████╔╝
   ██║╚════╝██║╚██╔╝██║██║██║╚██╗██║██╔══╝  ╚════██║██║███╗██║██╔══╝  ██╔══╝  ██╔═══╝ ██╔══╝  ██╔══██╗
   ██║      ██║ ╚═╝ ██║██║██║ ╚████║███████╗███████║╚███╔███╔╝███████╗███████╗██║     ███████╗██║  ██║
   ╚═╝      ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚══╝╚══╝ ╚══════╝╚══════╝╚═╝     ╚══════╝╚═╝  ╚═╝
                                                                                                      
    "#.to_string()
}

fn format_duration(d: std::time::Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    let millis = d.subsec_millis();
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}.{:03}s", seconds, millis)
    } else {
        format!("{}ms", millis)
    }
}
