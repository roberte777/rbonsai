use std::{
    io::stdout,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{rngs::StdRng, SeedableRng};
use rbonsai::{
    bonsai::{
        draw_tree, grow_tree, init,
        utility::{check_key_press, create_message_window},
    },
    Config,
};

fn main() {
    let mut args = Config::parse();

    if args.screensaver {
        args.live = true;
        args.infinite = true;
    }

    let mut stdout = stdout();

    let seed = args.seed.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
    });

    let mut rng = StdRng::seed_from_u64(seed);

    enable_raw_mode().unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    // Flush any pending events
    while event::poll(Duration::from_millis(10)).unwrap() {
        let _ = event::read();
    }

    let last_tree = loop {
        init(&args);
        let tree = grow_tree(&args, &mut rng);
        draw_tree(&args, &tree);

        if let Some(message) = &args.message {
            create_message_window(message).unwrap();
        }
        if !args.infinite {
            break tree;
        }
        let start = Instant::now();
        let mut finished = false;
        while start.elapsed() < Duration::from_secs_f64(args.wait) {
            if check_key_press() {
                finished = true;
                break;
            }
            thread::sleep(Duration::from_millis(50)); // Sleep to avoid busy-waiting
        }

        if finished {
            break tree;
        }
    };

    let (_, rows) = crossterm::terminal::size().unwrap();
    if args.print {
        args.live = false;
        execute!(stdout, LeaveAlternateScreen).unwrap();
        for _ in 0..rows {
            println!();
        }

        init(&args);
        draw_tree(&args, &last_tree);
        if let Some(message) = &args.message {
            create_message_window(message).unwrap();
        }
        execute!(stdout, MoveTo(0, rows - 1),).unwrap();
        println!();
    } else {
        loop {
            match event::read() {
                Ok(event) => {
                    // break if key event
                    if let Event::Key(key_event) = event {
                        if let KeyEventKind::Press = key_event.kind {
                            break;
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        execute!(stdout, LeaveAlternateScreen).unwrap();
    }

    // move cursor to bottom of terminal
    execute!(stdout, MoveTo(0, rows - 1),).unwrap();
    let _ = disable_raw_mode();
    execute!(stdout, cursor::Show).unwrap();
}
