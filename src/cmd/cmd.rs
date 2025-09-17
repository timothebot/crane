use std::path::PathBuf;

use clap::{Parser, ValueHint, command};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub enum CraneCli {
    Add(Add),
    List(List),
}

/// Add a brick to your directory
#[derive(Debug, Parser)]
pub struct Add {
    #[clap(num_args = 1.., required = true)]
    pub bricks: Vec<String>,

    #[arg(short, long, value_hint=ValueHint::DirPath, value_terminator=",", default_value="")]
    pub brick_dirs: Vec<PathBuf>,

    #[arg(short, long, value_hint=ValueHint::DirPath)]
    pub target_dir: Option<PathBuf>,
}

/// List all available bricks
#[derive(Debug, Parser, Clone)]
pub struct List {
    #[arg(short, long, value_hint=ValueHint::DirPath)]
    pub brick_dirs: Option<PathBuf>,
}
