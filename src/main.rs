use clap::Parser;

use crate::cmd::{CraneCli, Run};

mod bricks;
mod cmd;
mod config;
mod files;
mod utils;

fn main() {
    let cli = CraneCli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    cli.run();
}
