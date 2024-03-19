use std::io::stdout;

use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use ronsai::{base::draw_base, bonsai::grow_tree, Config};
fn main() {
    let (_, rows) = crossterm::terminal::size().unwrap();
    for _ in 0..rows {
        println!();
    }
    let mut stdout = stdout();
    // execute!(stdout, Clear(ClearType::All),).unwrap();
    execute!(stdout, MoveTo(0, 0), Print("0,0")).unwrap();

    let base_type: u8 = 2;
    draw_base(base_type); // Example usage
    let config = Config {
        verbosity: 1,
        life_start: 32,
        multiplier: 3,
        base_type,
    };
    grow_tree(&config);
    // move cursor to bottom of terminal
    execute!(stdout, cursor::MoveTo(0, rows - 1),).unwrap();
    println!();
}
