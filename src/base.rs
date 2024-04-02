use std::io::{stdout, Write};

use crossterm::{
    cursor::{MoveTo}, queue,
    style::{Color, Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal,
};

use crate::Config;

pub fn draw_base(config: &Config) {
    let base_type = config.base_type;
    let mut stdout = stdout();
    let (cols, rows) = terminal::size().unwrap(); // Get terminal size for centering

    match base_type {
        1 => {
            let lines = [
                ":___________./~~~\\.___________:",
                " \\                           / ",
                "  \\_________________________/ ",
                "  (_)                     (_)",
            ];
            let base_width = 31; // The maximum width of the base art for base type 1
            let start_pos = (cols / 2) - (base_width / 2);
            let y = rows - lines.len() as u16;
            queue!(
                stdout,
                SetAttribute(crossterm::style::Attribute::Bold),
                SetForegroundColor(Color::AnsiValue(8)),
                SetBackgroundColor(Color::Reset),
                MoveTo(start_pos, y),
                Print(":"),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("___________"),
                SetForegroundColor(Color::AnsiValue(11)),
                Print("./~~~\\."),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("___________"),
                SetAttribute(crossterm::style::Attribute::Bold),
                SetForegroundColor(Color::AnsiValue(8)),
                Print(":"),
                MoveTo(start_pos, y + 1),
                Print(" \\                           / "),
                MoveTo(start_pos, y + 2),
                Print("  \\_________________________/ "),
                MoveTo(start_pos, y + 3),
                Print("  (_)                     (_)"),
            )
            .unwrap();

            // flush
            stdout.flush().unwrap();
        }
        2 => {
            let base_width = 15; // The maximum width of the base art for base type 2
            let start_pos = (cols / 2) - (base_width / 2);
            let y = rows - 3; // Assuming there are always 3 lines for base type 2

            // Adjust color and attribute settings before printing each part of the base art
            queue!(
                stdout,
                SetAttribute(crossterm::style::Attribute::NormalIntensity), // Assuming you want to reset boldness here
                SetForegroundColor(Color::AnsiValue(8)),
                SetBackgroundColor(Color::Reset),
                MoveTo(start_pos, y),
                Print("("),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("---"),
                SetForegroundColor(Color::AnsiValue(11)),
                Print("./~~~\\."),
                SetForegroundColor(Color::AnsiValue(2)),
                Print("---"),
                SetForegroundColor(Color::AnsiValue(8)),
                Print(")"),
                MoveTo(start_pos, y + 1),
                Print(" (           ) "),
                MoveTo(start_pos, y + 2),
                Print("  (_________)  "),
            )
            .unwrap();

            // flush stdout to apply all queued actions
            stdout.flush().unwrap();
        }
        _ => {}
    }
}
