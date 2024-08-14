pub mod utility;
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{Color, Colors, Print, SetColors},
    terminal::{self},
    QueueableCommand,
};
use rand::Rng;
use std::io::{stdout, Write};
use utility::{choose_color, choose_string, set_deltas};

use crate::Config;

/// The different types of each symbol
pub(crate) enum BranchType {
    /// The main trunk of the tree
    Trunk,
    /// A branch that grows to the left
    ShootLeft,
    /// A branch that grows to the right
    ShootRight,
    /// A branch that is dying
    Dead,
    /// A branch that is dead
    Dying,
}

struct Counters {
    shoots: i32,
    branches: i32,
    shoot_counter: i32,
    tree_bottom: u16,
}

pub fn grow_tree(config: &Config) {
    let mut stdout = stdout();
    let (max_x, mut max_y) = terminal::size().unwrap();
    max_y = match config.base {
        1 => max_y - 5,
        2 => max_y - 4,
        _ => max_y,
    };

    // Reset counters
    let mut counters = Counters {
        shoots: 0,
        branches: 0,
        // Initialize shoot counter to a random value
        shoot_counter: (rand::random::<i32>() % 3) + 1,
        tree_bottom: max_y,
    };

    if config.verbose {
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
        max_y as i32,
        (max_x / 2) as i32,
        BranchType::Trunk,
        config.life,
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
        if config.verbose {
            stdout
                .queue(MoveTo(5, 3))
                .unwrap()
                .queue(Print(format!("starting life: {}", life)))
                .unwrap();
        }
        // Decrement life
        life -= 1;
        let age = config.life - life;

        let (dx, mut dy) = set_deltas(&branch_type, life, age, config.multiplier);
        let (max_x, max_y) = terminal::size().unwrap();

        if dy > 0 && y > (counters.tree_bottom as i32 - 1) {
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
                        if config.verbose {
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

        if config.verbose {
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

        if x < 0 || x as u16 >= max_x || y < 0 || y as u16 >= max_y {
            continue;
        }

        // Drawing the branch part
        let branch_str = choose_string(config, &branch_type, life, dx, dy);
        // Example to set color, adjust as needed
        choose_color(&branch_type).unwrap();
        execute!(stdout, MoveTo(x as u16, y as u16), Print(branch_str),).unwrap();
        // reset color
        execute!(stdout, SetColors(Colors::new(Color::Reset, Color::Reset)),).unwrap();
        if config.live {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
