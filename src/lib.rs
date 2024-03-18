pub mod base;
pub mod bonsai;

pub struct Config {
    // error logging
    pub verbosity: i32,
    // starting life of the tree
    // higher -> bigger tree
    pub life_start: i32,
    // branch multiplier; higher -> less branches
    pub multiplier: i32,
    /// base type
    pub base_type: u8,
}
