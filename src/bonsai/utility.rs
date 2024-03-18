use std::io::stdout;

use crossterm::{
    execute,
    style::{Attribute, Color, SetAttribute, SetBackgroundColor, SetForegroundColor},
};
use rand::Rng;

use crate::Config;

use super::BranchType;

pub(crate) fn set_deltas(
    branch_type: &BranchType,
    life: i32,
    age: i32,
    multiplier: i32,
) -> (i32, i32) {
    let mut rng = rand::thread_rng();
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

pub(crate) fn choose_color(branch_type: &BranchType) -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();

    // Default background color
    let bg = Color::Reset; // Using Reset to use terminal's default

    match branch_type {
        BranchType::Trunk | BranchType::ShootLeft | BranchType::ShootRight => {
            if rng.gen_range(0..2) == 0 {
                execute!(
                    stdout,
                    SetAttribute(Attribute::Bold),
                    SetForegroundColor(Color::AnsiValue(11)),
                    SetBackgroundColor(bg),
                )?;
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::AnsiValue(3)),
                    SetBackgroundColor(bg),
                )?;
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
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::AnsiValue(2)),
                    SetBackgroundColor(bg),
                )?;
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
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(Color::AnsiValue(10)),
                    SetBackgroundColor(bg),
                )?;
            }
        }
    }

    Ok(())
}
