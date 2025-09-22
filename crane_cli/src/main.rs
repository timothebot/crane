use clap::Parser;

use crate::cmd::{CraneCli, Run};

mod cmd;
mod config;
mod utils;

fn main() {
    let cli = CraneCli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    cli.run();
}
