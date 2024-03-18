use std::io::stdout;

use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};

pub fn draw_base(base_type: i32) {
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
            for (i, line) in lines.iter().enumerate() {
                let start_pos = (cols / 2) - (base_width / 2);
                let y = rows - (lines.len() as u16 - i as u16);
                execute!(stdout, cursor::MoveTo(start_pos, y),).unwrap();
                // Print each line with appropriate color settings
                match i {
                    0 => {
                        execute!(stdout, SetForegroundColor(Color::DarkGrey), Print(line),)
                            .unwrap();
                    }
                    1 | 2 => {
                        // Just an example, adjust colors as needed
                        execute!(stdout, SetForegroundColor(Color::Green), Print(line),).unwrap();
                    }
                    _ => {
                        // Default color for any other lines
                        execute!(stdout, Print(line),).unwrap();
                    }
                }
            }
        }
        2 => {
            let lines = ["(---./~~~\\.--)", " (           ) ", "  (_________)  "];
            let base_width = 15; // The maximum width of the base art for base type 2
            for (i, line) in lines.iter().enumerate() {
                let line_length = line.chars().count() as u16;
                let start_pos = (cols / 2) - (base_width / 2) + ((base_width - line_length) / 2);
                execute!(stdout, cursor::MoveTo(start_pos, i as u16 + 1),).unwrap();
                // Similarly, adjust color settings and print each line
                execute!(stdout, Print(line),).unwrap();
            }
        }
        _ => {}
    }
}
