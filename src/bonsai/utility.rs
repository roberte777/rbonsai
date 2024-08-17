use std::{
    error::Error,
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    event::{self},
    execute, queue,
    style::{Attribute, Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::size,
};
use rand::{rngs::StdRng, Rng};

use crate::Config;

use super::BranchType;

pub(crate) fn set_deltas(
    branch_type: &BranchType,
    life: i32,
    age: i32,
    multiplier: i32,
    rng: &mut StdRng,
) -> (i32, i32) {
    let (dx, dy): (i32, i32);

    match branch_type {
        BranchType::Trunk => {
            if age <= 2 || life < 4 {
                dy = 0;
                dx = rng.gen_range(-1..=1);
            } else if age < (multiplier * 3) {
                if age % (multiplier / 2) == 0 {
                    dy = -1;
                } else {
                    dy = 0;
                }

                dx = match rng.gen_range(0..10) {
                    0 => -2,
                    1..=3 => -1,
                    4..=5 => 0,
                    6..=8 => 1,
                    _ => 2,
                };
            } else {
                if rng.gen_range(0..10) > 2 {
                    dy = -1;
                } else {
                    dy = 0;
                }
                dx = rng.gen_range(-1..=1);
            }
        }
        BranchType::ShootLeft => {
            dy = match rng.gen_range(0..10) {
                0..=1 => -1,
                2..=7 => 0,
                _ => 1,
            };
            dx = match rng.gen_range(0..10) {
                0..=1 => -2,
                2..=5 => -1,
                6..=8 => 0,
                _ => 1,
            };
        }
        BranchType::ShootRight => {
            dy = match rng.gen_range(0..10) {
                0..=1 => -1,
                2..=7 => 0,
                _ => 1,
            };
            dx = match rng.gen_range(0..10) {
                0..=1 => 2,
                2..=5 => 1,
                6..=8 => 0,
                _ => -1,
            };
        }
        BranchType::Dying => {
            dy = match rng.gen_range(0..10) {
                0..=1 => -1,
                2..=8 => 0,
                _ => 1,
            };
            dx = match rng.gen_range(0..15) {
                0 => -3,
                1..=2 => -2,
                3..=5 => -1,
                6..=8 => 0,
                9..=11 => 1,
                12..=13 => 2,
                _ => 3,
            };
        }
        BranchType::Dead => {
            dy = match rng.gen_range(0..10) {
                0..=2 => -1,
                3..=6 => 0,
                _ => 1,
            };
            dx = rng.gen_range(-1..=1);
        }
    }

    (dx, dy)
}
pub(crate) fn choose_string(
    _conf: &Config,
    branch_type: &BranchType,
    life: i32,
    dx: i32,
    dy: i32,
) -> String {
    let mut branch_str = match branch_type {
        BranchType::Trunk => match (dx, dy) {
            (0, 0) => "/~".to_string(),
            _ if dx < 0 => "\\|".to_string(),
            (0, _) => "/|\\".to_string(),
            _ if dx > 0 => "|/".to_string(),
            _ => "?".to_string(), // Fallback
        },
        BranchType::ShootLeft => match (dx, dy) {
            _ if dy > 0 => "\\".to_string(),
            (0, 0) => "\\_".to_string(),
            _ if dx < 0 => "\\|".to_string(),
            (0, _) => "/|".to_string(),
            _ if dx > 0 => "/".to_string(),
            _ => "?".to_string(), // Fallback
        },
        BranchType::ShootRight => match (dx, dy) {
            _ if dy > 0 => "/".to_string(),
            (0, 0) => "_/".to_string(),
            _ if dx < 0 => "\\|".to_string(),
            (0, _) => "/|".to_string(),
            _ if dx > 0 => "/".to_string(),
            _ => "?".to_string(), // Fallback
        },
        BranchType::Dying | BranchType::Dead => {
            "&".to_string() // Fallback
                            // add the below if user input leaves become allowedc
                            // conf.leaves[rng.gen_range(0..conf.leaves.len())].clone()
        }
    };

    // If life < 4, override with dying or dead branch representation
    if life < 4 {
        branch_str = "&".to_string(); // Fallback
                                      // add the below if user input leaves become allowedc
                                      // return conf.leaves[rng.gen_range(0..conf.leaves.len())].clone();
    }

    branch_str
}

pub struct Style {
    pub attribute: Attribute,
    pub foreground_color: Color,
    pub background_color: Color,
}

pub(crate) fn choose_color(
    branch_type: &BranchType,
    rng: &mut StdRng,
) -> Result<Style, std::io::Error> {
    let mut stdout = stdout();

    // Default background color
    let bg = Color::Reset; // Using Reset to use terminal's default
    let mut style = Style {
        attribute: Attribute::Reset,
        foreground_color: Color::Reset,
        background_color: bg,
    };

    match branch_type {
        BranchType::Trunk | BranchType::ShootLeft | BranchType::ShootRight => {
            if rng.gen_range(0..2) == 0 {
                style.attribute = Attribute::Bold;
                style.foreground_color = Color::AnsiValue(11);
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::AnsiValue(3)),
                    SetBackgroundColor(bg),
                )?;
                style.foreground_color = Color::AnsiValue(3);
            }
        }
        BranchType::Dying => {
            if rng.gen_range(0..10) == 0 {
                execute!(
                    stdout,
                    SetAttribute(Attribute::Bold),
                    SetForegroundColor(Color::AnsiValue(2)),
                    SetBackgroundColor(bg),
                )?;
                style.attribute = Attribute::Bold;
                style.foreground_color = Color::AnsiValue(2);
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::AnsiValue(2)),
                    SetBackgroundColor(bg),
                )?;
                style.foreground_color = Color::AnsiValue(2);
            }
        }
        BranchType::Dead => {
            if rng.gen_range(0..3) == 0 {
                execute!(
                    stdout,
                    SetAttribute(Attribute::Bold),
                    SetForegroundColor(Color::AnsiValue(10)),
                    SetBackgroundColor(bg),
                )?;
                style.attribute = Attribute::Bold;
                style.foreground_color = Color::AnsiValue(10);
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::AnsiValue(10)),
                    SetBackgroundColor(bg),
                )?;
                style.foreground_color = Color::AnsiValue(10);
            }
        }
    }

    Ok(style)
}
pub fn create_message_window(message: &str) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    // Get terminal size
    let (max_x, max_y) = size()?;

    // Calculate box dimensions based on message length
    let message_length = message.chars().count() as u16;
    let (box_width, box_height) = if message_length + 3 <= (0.25 * max_x as f32) as u16 {
        (message_length + 1, 0)
    } else {
        let width = (0.25 * max_x as f32) as u16;
        let height = message_length / width;
        (width, height)
    };

    // Calculate position based on terminal size
    let num_lines = box_height + 2;
    let num_cols = box_width + 4;
    let border_x_start = (max_x as f32 * 0.7) as u16 - 2;
    let border_y_start = (max_y as f32 * 0.7) as u16 - 1;
    let message_x_start = (max_x as f32 * 0.7) as u16;
    let message_y_start = (max_y as f32 * 0.7) as u16;

    // Draw the box border
    queue!(
        stdout,
        MoveTo(border_x_start, border_y_start),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Reset),
        Print("+"),
        MoveTo(border_x_start + num_cols, border_y_start),
        Print("+"),
        MoveTo(border_x_start, border_y_start + num_lines),
        Print("+"),
        MoveTo(border_x_start + num_cols, border_y_start + num_lines),
        Print("+"),
    )?;
    for i in 1..num_cols {
        queue!(
            stdout,
            MoveTo(border_x_start + i, border_y_start),
            Print("-"),
            MoveTo(border_x_start + i, border_y_start + num_lines),
            Print("-"),
        )?;
    }
    for i in 1..num_lines {
        queue!(
            stdout,
            MoveTo(border_x_start, border_y_start + i),
            Print("|"),
            MoveTo(border_x_start + num_cols, border_y_start + i),
            Print("|"),
        )?;
    }

    // Print the message inside the box
    let lines: Vec<&str> = message.split_whitespace().collect();
    let mut current_line = String::new();

    let mut line_count = 0;
    for word in lines.iter() {
        if current_line.len() + word.len() > box_width as usize {
            queue!(
                stdout,
                MoveTo(message_x_start + 1, message_y_start + line_count as u16),
                Print(&current_line)
            )?;
            current_line.clear();
            line_count += 1;
        }
        current_line.push_str(word);
        current_line.push(' ');
    }
    queue!(
        stdout,
        MoveTo(message_x_start + 1, message_y_start + line_count as u16),
        Print(&current_line)
    )?;

    stdout.flush()?;

    Ok(())
}

pub fn check_key_press() -> bool {
    if event::poll(Duration::from_millis(0)).unwrap() {
        return true;
        // if let Event::Key(key_event) = event::read().unwrap() {
        //     if key_event.code == KeyCode::Char('q') {
        //         return true;
        //     }
        // }
    }
    false
}
