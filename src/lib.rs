use clap::Parser;

pub mod base;
pub mod bonsai;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Whether the tree generation should pause after each step
    /// to allow the user to watch it grow
    #[arg(short, long, default_value_t = false)]
    pub live: bool,
    /// In live mode, wait time in seconds between each step of growth
    #[arg(short, long, default_value_t = 0.03)]
    pub time: f64,
    /// Infinite mode: keep growing trees
    #[arg(short, long, default_value_t = false)]
    pub infinite: bool,
    /// In infinite mode, the wait time in seconds between each tree
    #[arg(short, long, default_value_t = 4.)]
    pub wait: f64,
    /// Screensaver mode: equivalent to -li and quit on any keypress
    #[arg(short = 'S', long, default_value_t = false)]
    pub screensaver: bool,
    /// Attach message next to tree
    #[arg(short, long)]
    pub message: Option<String>,
    /// Ascii art plant base to use.
    #[arg(short, long, default_value_t = 1)]
    pub base: u8,
    /// The branch multiplier; higher -> less branches
    #[arg(short = 'M', long, default_value_t = 3)]
    pub multiplier: i32,
    /// The starting life of the tree
    /// higher -> bigger tree
    #[arg(short = 'L', long, default_value_t = 32)]
    pub life: i32,
    /// Print tree to terminal when finished
    #[arg(short, long, default_value_t = false)]
    pub print: bool,
    /// Random number seed for reproducable trees
    #[arg(short, long)]
    pub seed: Option<u64>,
    /// Whether there should be debug prints
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
    // TODO: Add support for saving to file, loading from file
}
