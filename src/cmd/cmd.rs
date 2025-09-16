use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub enum CraneCli {
    Add(Add),
    List(List)
}

/// Add a brick to your directory
#[derive(Debug, Parser)]
pub struct Add {
    #[clap(num_args = 1.., required = true)]
    pub bricks: Vec<String>,

    #[arg(short, long)]
    pub dir: Option<PathBuf>,
}

/// List all available bricks
#[derive(Debug, Parser, Clone)]
pub struct List {
    #[arg(short, long)]
    pub dir: Option<PathBuf>,
}
