pub mod utility;
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{
        Color, Colors, Print, SetAttribute, SetBackgroundColor, SetColors, SetForegroundColor,
    },
    terminal::{self, Clear},
    QueueableCommand,
};
use rand::{rngs::StdRng, Rng};
use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};
use utility::{check_key_press, choose_color, choose_string, set_deltas};

use crate::{base::draw_base, Config};

use self::utility::Style;

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

pub struct Val {
    pub style: Style,
    pub char: String,
    pub pos: Position,
    pub dx: i32,
    pub dy: i32,
    pub life: i32,
    pub branch_type: String,
    pub shoots: i32,
    pub shoot_cooldown: i32,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn grow_tree(config: &Config, rng: &mut StdRng) -> Vec<Val> {
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
        shoot_counter: (rng.gen::<i32>() % 3) + 1,
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

    let mut tree = Vec::new();

    // Recursively grow tree trunk and branches
    branch(
        config,
        &mut counters,
        &mut tree,
        rng,
        Position {
            x: (max_x / 2) as i32,
            y: max_y as i32,
        },
        BranchType::Trunk,
        config.life,
    );

    tree
}

fn branch(
    config: &Config,
    counters: &mut Counters,
    tree: &mut Vec<Val>,
    rng: &mut StdRng,
    mut pos: Position,
    branch_type: BranchType,
    mut life: i32,
) {
    counters.branches += 1;
    let mut shoot_cooldown = config.multiplier;

    // This is a highly simplified loop to mimic the growth logic
    while life > 0 {
        // Decrement life
        life -= 1;
        let age = config.life - life;

        let (dx, mut dy) = set_deltas(&branch_type, life, age, config.multiplier, rng);
        let (max_x, max_y) = terminal::size().unwrap();

        if dy > 0 && pos.y > (counters.tree_bottom as i32 - 1) {
            dy -= 1;
        } // reduce dy if too close to the ground
          // Ensure x and y are within terminal bounds
        if pos.x < 0 || pos.x as u16 >= max_x || pos.y < 0 || pos.y as u16 >= max_y {
            break;
        }

        if life < 3 {
            branch(config, counters, tree, rng, pos, BranchType::Dead, life);
        } else {
            match branch_type {
                BranchType::Trunk | BranchType::ShootLeft | BranchType::ShootRight
                    if life < (config.multiplier + 2) =>
                {
                    branch(config, counters, tree, rng, pos, BranchType::Dying, life);
                }
                BranchType::Trunk
                    if (rng.gen_range(0..3) == 0 || life % config.multiplier == 0) =>
                {
                    if rng.gen_range(0..8) == 0 && life > 7 {
                        shoot_cooldown = config.multiplier * 2;
                        let random_life = life + rng.gen_range(-2..3);
                        branch(
                            config,
                            counters,
                            tree,
                            rng,
                            pos,
                            BranchType::Trunk,
                            random_life,
                        );
                    } else if shoot_cooldown <= 0 {
                        shoot_cooldown = config.multiplier * 2;
                        let shoot_life = life + config.multiplier;
                        counters.shoots += 1;
                        counters.shoot_counter += 1;
                        let shoot_direction = if counters.shoot_counter % 2 == 0 {
                            BranchType::ShootLeft
                        } else {
                            BranchType::ShootRight
                        };
                        branch(
                            config,
                            counters,
                            tree,
                            rng,
                            pos,
                            shoot_direction,
                            shoot_life,
                        );
                    }
                }
                _ => {}
            }
        }

        shoot_cooldown -= 1;

        // Update x and y for the next iteration
        pos.x += dx;
        pos.y += dy;

        if pos.x < 0 || pos.x as u16 >= max_x || pos.y < 0 || pos.y as u16 >= max_y {
            continue;
        }

        // Drawing the branch part
        let branch_str = choose_string(config, &branch_type, life, dx, dy);
        // Example to set color, adjust as needed
        let style = choose_color(&branch_type, rng).unwrap();
        let type_str = match branch_type {
            BranchType::Trunk => "Trunk",
            BranchType::ShootLeft => "ShootLeft",
            BranchType::ShootRight => "ShootRight",
            BranchType::Dying => "Dying",
            BranchType::Dead => "Dead",
        };
        let val = Val {
            pos,
            style,
            char: branch_str,
            branch_type: type_str.into(),
            dx,
            dy,
            life,
            shoots: counters.shoots,
            shoot_cooldown,
        };
        tree.push(val);
    }
}
pub fn draw_tree(config: &Config, tree: &Vec<Val>) {
    let mut stdout = stdout();
    for val in tree {
        if config.verbose {
            // Queueing the commands instead of executing them immediately
            // This allows for batching the writes, which can be more efficient
            stdout
                .queue(MoveTo(5, 3))
                .unwrap()
                .queue(Print(format!("life: {}", val.life)))
                .unwrap()
                .queue(MoveTo(5, 4))
                .unwrap()
                .queue(Print(format!("shoots: {:02}", val.shoots)))
                .unwrap()
                .queue(MoveTo(5, 5))
                .unwrap()
                .queue(Print(format!("dx: {:02}", val.dx)))
                .unwrap()
                .queue(MoveTo(5, 6))
                .unwrap()
                .queue(Print(format!("dy: {:02}", val.dy)))
                .unwrap()
                .queue(MoveTo(5, 7))
                .unwrap()
                .queue(Print(format!("type: {}", val.branch_type)))
                .unwrap()
                .queue(MoveTo(5, 8))
                .unwrap()
                .queue(Print(format!("shootCooldown: {:3}", val.shoot_cooldown)))
                .unwrap();

            // Flush the stdout to apply the queued operations
            stdout.flush().unwrap();
        }

        let _ = execute!(
            stdout,
            SetAttribute(val.style.attribute),
            SetForegroundColor(val.style.foreground_color),
            SetBackgroundColor(val.style.background_color),
        );
        let _ = execute!(
            stdout,
            MoveTo(val.pos.x as u16, val.pos.y as u16),
            Print(val.char.clone()),
        );
        // reset color
        let _ = execute!(stdout, SetColors(Colors::new(Color::Reset, Color::Reset)),);
        if config.live {
            let start = Instant::now();
            let mut finished = false;
            while start.elapsed() < Duration::from_secs_f64(config.time) {
                if check_key_press() {
                    finished = true;
                    break;
                }
                thread::sleep(Duration::from_millis(50)); // Sleep to avoid busy-waiting
            }

            if finished {
                return;
            }
        }
    }
}

pub fn init(args: &Config) {
    let mut stdout = stdout();
    execute!(stdout, Clear(terminal::ClearType::All)).unwrap();
    draw_base(args);
}
