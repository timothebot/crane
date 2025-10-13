use clap::Parser;

use crate::cmd::{CraneCli, Run};

mod cmd;
mod config;
mod logging;

fn main() {
    let cli = CraneCli::parse();
    logging::setup(&cli.verbose);
    cli.run();
}
