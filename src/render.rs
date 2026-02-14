use crate::app::*;
use crate::error::*;
use crate::terminal::screen::*;

pub fn render(app: &App) -> Result<(), RenderError> {
    let box_x = 0;
    let box_y = 20;
    let box_w = 40;
    let box_h = 3;
    match app.status {
        Status::Welcome => {
            app.print(&get_banner());
            app.print("\n\n\n\n");
            app.print(&get_manual());
        }
        Status::Game => {
            let game = app.game.as_ref().ok_or(RenderError::NoGame)?;
            game.draw(false, &app.screen);
            app.print("\n\n\n\n");
            app.print("Input position (X Y)");
            app.print(&app.input.content);
        }
        Status::Success => {
            app.print(&get_win());
        }
        Status::Failed => {
            app.print(&get_die());
        }
    }
    draw_box(&app.screen, box_x, box_y, box_w, box_h);

    draw_text_in_box(&app.screen, box_x, box_y, box_w, &app.input.content);
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
    r#"
        Enter e to select EASY mode (8 x 8 x 10)
        Enter n to select NORMAL mode (16 x 16 x 40)
        Enter h to select HARD mode (16 x 30 x 99)
        Enter q to QUIT game
    "#
    .to_string()
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

fn get_die() -> String {
    r#"
 █████ █████    ███████    █████  █████    ██████████   █████ ██████████
░░███ ░░███   ███░░░░░███ ░░███  ░░███    ░░███░░░░███ ░░███ ░░███░░░░░█
 ░░███ ███   ███     ░░███ ░███   ░███     ░███   ░░███ ░███  ░███  █ ░ 
  ░░█████   ░███      ░███ ░███   ░███     ░███    ░███ ░███  ░██████   
   ░░███    ░███      ░███ ░███   ░███     ░███    ░███ ░███  ░███░░█   
    ░███    ░░███     ███  ░███   ░███     ░███    ███  ░███  ░███ ░   █
    █████    ░░░███████░   ░░████████      ██████████   █████ ██████████
   ░░░░░       ░░░░░░░      ░░░░░░░░      ░░░░░░░░░░   ░░░░░ ░░░░░░░░░░ 
                                                                        
                                                                        
                                                                        
    "#
    .to_string()
}

fn get_win() -> String {
    r#"
 █████ █████    ███████    █████  █████    █████   ███   █████ █████ ██████   █████
░░███ ░░███   ███░░░░░███ ░░███  ░░███    ░░███   ░███  ░░███ ░░███ ░░██████ ░░███ 
 ░░███ ███   ███     ░░███ ░███   ░███     ░███   ░███   ░███  ░███  ░███░███ ░███ 
  ░░█████   ░███      ░███ ░███   ░███     ░███   ░███   ░███  ░███  ░███░░███░███ 
   ░░███    ░███      ░███ ░███   ░███     ░░███  █████  ███   ░███  ░███ ░░██████ 
    ░███    ░░███     ███  ░███   ░███      ░░░█████░█████░    ░███  ░███  ░░█████ 
    █████    ░░░███████░   ░░████████         ░░███ ░░███      █████ █████  ░░█████
   ░░░░░       ░░░░░░░      ░░░░░░░░           ░░░   ░░░      ░░░░░ ░░░░░    ░░░░░ 
                                                                                   
                                                                                   
                                                                                   
    "#
    .to_string()
}
