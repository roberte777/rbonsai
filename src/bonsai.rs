use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{
        Attribute, Color, Colors, Print, SetAttribute, SetBackgroundColor, SetColors,
        SetForegroundColor,
    },
    terminal::{self}, QueueableCommand,
};
use rand::Rng;
use std::io::{stdout, Write};
pub struct Config {
    pub verbosity: i32,
    pub life_start: i32,
    // branch multiplier; higher -> more
    pub multiplier: i32,
}

#[derive(PartialEq)]
enum BranchType {
    Trunk,
    ShootLeft,
    ShootRight,
    Dead,
    Dying,
}

struct Counters {
    shoots: i32,
    branches: i32,
    shoot_counter: i32,
}
pub fn grow_tree(config: &Config) {
    let mut stdout = stdout();
    let (max_x, max_y) = terminal::size().unwrap();

    // Reset counters
    let mut counters = Counters {
        shoots: 0,
        branches: 0,
        // Initialize shoot counter to a random value
        shoot_counter: (rand::random::<i32>() % 3) + 1,
    };

    if config.verbosity > 0 {
        execute!(
            stdout,
            cursor::MoveTo(5, 2),
            Print(format!("maxX: {:03}, maxY: {:03}", max_x, max_y)),
        )
        .unwrap();
    }

    // Recursively grow tree trunk and branches
    branch(
        config,
        &mut counters,
        max_y as i32 - 5,
        (max_x / 2) as i32,
        BranchType::Trunk,
        config.life_start,
    );
}

fn branch(
    config: &Config,
    counters: &mut Counters,
    mut y: i32,
    mut x: i32,
    branch_type: BranchType,
    mut life: i32,
) {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();
    counters.branches += 1;
    let mut shoot_cooldown = config.multiplier;

    // This is a highly simplified loop to mimic the growth logic
    while life > 0 {
        stdout
            .queue(MoveTo(5, 3))
            .unwrap()
            .queue(Print(format!("starting life: {}", life)))
            .unwrap();
        // Decrement life
        life -= 1;
        let age = config.life_start - life;

        let (dx, mut dy) = set_deltas(&branch_type, life, age, config.multiplier);
        let (max_x, max_y) = terminal::size().unwrap();

        if dy > 0 && y > (max_y as i32 - 6) {
            dy -= 1;
        } // reduce dy if too close to the ground
          // Ensure x and y are within terminal bounds
        if x < 0 || x as u16 >= max_x || y < 0 || y as u16 >= max_y {
            break;
        }

        if life < 3 {
            branch(config, counters, y, x, BranchType::Dead, life);
        } else {
            match branch_type {
                BranchType::Trunk | BranchType::ShootLeft | BranchType::ShootRight
                    if life < (config.multiplier + 2) =>
                {
                    branch(config, counters, y, x, BranchType::Dying, life);
                }
                BranchType::Trunk
                    if (rng.gen_range(0..3) == 0 || life % config.multiplier == 0) =>
                {
                    if rng.gen_range(0..8) == 0 && life > 7 {
                        shoot_cooldown = config.multiplier * 2;
                        let random_life = life + rng.gen_range(-2..3);
                        branch(config, counters, y, x, BranchType::Trunk, random_life);
                    } else if shoot_cooldown <= 0 {
                        shoot_cooldown = config.multiplier * 2;
                        let shoot_life = life + config.multiplier;
                        counters.shoots += 1;
                        counters.shoot_counter += 1;
                        if config.verbosity > 0 {
                            let _ = execute!(
                                stdout,
                                cursor::MoveTo(5, 4),
                                Print(format!("shoots: {:02}", counters.shoots)),
                            );
                        }
                        let shoot_direction = if counters.shoot_counter % 2 == 0 {
                            BranchType::ShootLeft
                        } else {
                            BranchType::ShootRight
                        };
                        branch(config, counters, y, x, shoot_direction, shoot_life);
                    }
                }
                _ => {}
            }
        }

        shoot_cooldown -= 1;

        if config.verbosity > 0 {
            let type_str = match branch_type {
                BranchType::Trunk => "Trunk",
                BranchType::ShootLeft => "ShootLeft",
                BranchType::ShootRight => "ShootRight",
                BranchType::Dying => "Dying",
                BranchType::Dead => "Dead",
            };

            // Queueing the commands instead of executing them immediately
            // This allows for batching the writes, which can be more efficient
            stdout
                .queue(MoveTo(5, 5))
                .unwrap()
                .queue(Print(format!("dx: {:02}", dx)))
                .unwrap()
                .queue(MoveTo(5, 6))
                .unwrap()
                .queue(Print(format!("dy: {:02}", dy)))
                .unwrap()
                .queue(MoveTo(5, 7))
                .unwrap()
                .queue(Print(format!("type: {}", type_str)))
                .unwrap()
                .queue(MoveTo(5, 8))
                .unwrap()
                .queue(Print(format!("shootCooldown: {:3}", shoot_cooldown)))
                .unwrap();

            // Flush the stdout to apply the queued operations
            stdout.flush().unwrap();
        }

        // Update x and y for the next iteration
        x += dx;
        y += dy;

        // Drawing the branch part
        let branch_str = choose_string(config, &branch_type, life, dx, dy);
        // Example to set color, adjust as needed
        choose_color(&branch_type).unwrap();
        execute!(stdout, MoveTo(x as u16, y as u16), Print(branch_str),).unwrap();
        // reset color
        execute!(stdout, SetColors(Colors::new(Color::Reset, Color::Reset)),).unwrap();
    }
}

fn set_deltas(branch_type: &BranchType, life: i32, age: i32, multiplier: i32) -> (i32, i32) {
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
fn choose_string(_conf: &Config, branch_type: &BranchType, life: i32, dx: i32, dy: i32) -> String {
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

fn choose_color(branch_type: &BranchType) -> Result<(), std::io::Error> {
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
