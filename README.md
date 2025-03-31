# rbonsai

This is a port of `cbonsai` to rust. Check out the original
[here](https://gitlab.com/jallbrit/cbonsai)

`rbonsai` is a bonsai tree generator, written in Rust using crossterm. It creates
an ascii bonsai tree that is colored using your terminals color scheme. It is
also configurable via CLI arguments - see [usage](#usage). It can print a static
tree to your terminal, or you can watch it grow live. There is also a screensaver
mode to repeatedly grow trees!

## Installation

`rbonsai` is available on crates.io. To install with Cargo:

```bash
cargo install rbonsai --locked
```

## Usage

```
Usage: rbonsai [OPTIONS]

Options:
  -l, --live                     Whether the tree generation should pause after each step to allow the user to watch it grow
  -t, --time <TIME>              In live mode, wait time in seconds between each step of growth [default: 0.03]
  -i, --infinite                 Infinite mode: keep growing trees
  -w, --wait <WAIT>              In infinite mode, the wait time in seconds between each tree [default: 4]
  -S, --screensaver              Screensaver mode: equivalent to -li and quit on any keypress
  -m, --message <MESSAGE>        Attach message next to tree
  -b, --base <BASE>              Ascii art plant base to use [default: 1]
  -M, --multiplier <MULTIPLIER>  The branch multiplier; higher -> less branches [default: 3]
  -L, --life <LIFE>              The starting life of the tree higher -> bigger tree [default: 32]
  -p, --print                    Print tree to terminal when finished
  -s, --seed <SEED>              Random number seed for reproducable trees
  -v, --verbose                  Whether there should be debug prints
  -h, --help                     Print help
  -V, --version                  Print version
```

## Add to `.bashrc`

For a new bonsai tree every time you open a terminal, add the following to the
end of your ~/.bashrc:

```bash
rbonsai -p
```

## Missing Features

`rbonsai` does not have support for loading or saving to a file as of yet. It
also does not have support for providing your of leave characters.

## Why Write This?

I had some issues installing cbonsai on one of my machines. I thought it was a
cool algorithm and wrote this to learn more about it. Additionally, I wanted to
use Crossterm for something to learn more about how it works. I also have plans 
for some newer features that I will add as soon as I complete the port of the 
original.

## Credits

This project is a port of [`cbonsai`](https://gitlab.com/jallbrit/cbonsai) into
Rust. Check it out if you want some of the missing features or would rather use
the original.
