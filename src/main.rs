use std::io::stdout;

use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
use ronsai::{
    base::draw_base,
    bonsai::{grow_tree, Config},
};
fn main() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All),).unwrap();
    execute!(stdout, MoveTo(0, 0), Print("0,0")).unwrap();

    draw_base(1); // Example usage
    let config = Config {
        verbosity: 1,
        life_start: 32,
        multiplier: 3,
    };
    grow_tree(&config);
    // move cursor to bottom of terminal
    let (_, rows) = crossterm::terminal::size().unwrap();
    execute!(stdout, cursor::MoveTo(0, rows - 1),).unwrap();
    println!();
}
