use std::io::stdout;

use clap::Parser;
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::Print,
};
use ronsai::{
    base::draw_base,
    bonsai::{grow_tree, utility::create_message_window},
    Config,
};

fn main() {
    let args = Config::parse();

    execute!(stdout(), cursor::Hide).unwrap();
    let (_, rows) = crossterm::terminal::size().unwrap();
    for _ in 0..rows {
        println!();
    }
    let mut stdout = stdout();
    // execute!(stdout, Clear(ClearType::All),).unwrap();
    execute!(stdout, MoveTo(0, 0), Print("0,0")).unwrap();

    let base_type: u8 = 1;
    draw_base(&args);
    grow_tree(&args);
    // create_message_window("testing a really long message to see what happens").unwrap();
    // move cursor to bottom of terminal
    execute!(stdout, cursor::MoveTo(0, rows - 1),).unwrap();
    println!();
    execute!(stdout, cursor::Show).unwrap();
}
