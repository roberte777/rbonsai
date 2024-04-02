use clap::Parser;

pub mod base;
pub mod bonsai;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Whether the tree generation should pause after each step
    /// to allow the user to watch it grow
    #[arg(short, long, default_value_t = false)]
    live: bool,
    /// Whether there should be debug prints
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    /// The base type to use
    #[arg(short, long, default_value_t = 1)]
    base_type: u8,
    /// The starting life of the tree
    /// higher -> bigger tree
    #[arg(short = 'L', long, default_value_t = 32)]
    life_start: i32,

    /// The branch multiplier; higher -> less branches
    #[arg(short, long, default_value_t = 3)]
    multiplier: i32,

    /// The message to display
    #[arg(short = 'M', long)]
    pub message: Option<String>,
}
