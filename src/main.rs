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
    if args.verbose {
        execute!(stdout, MoveTo(0, 0), Print("0,0")).unwrap();
    }

    let _base_type: u8 = 1;
    draw_base(&args);
    grow_tree(&args);
    if let Some(message) = &args.message {
        create_message_window(message).unwrap();
    }
    // move cursor to bottom of terminal
    execute!(stdout, cursor::MoveTo(0, rows - 1),).unwrap();
    println!();
    execute!(stdout, cursor::Show).unwrap();
}
