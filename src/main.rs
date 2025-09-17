use clap::Parser;

use crate::cmd::{CraneCli, Run};

mod bricks;
mod cmd;
mod config;
mod files;
mod utils;

fn main() {
    CraneCli::parse().run();
}
