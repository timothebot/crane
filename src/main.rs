use clap::Parser;

use crate::cmd::{CraneCli, Run};

mod config;
mod bricks;
mod cmd;

fn main() {
    CraneCli::parse().run();
}
