use std::fs;

use colored::Colorize;
use log::info;

use crate::{
    cmd::{List, Run},
    config::{CraneConfig, map_aliases},
};
use crane_bricks::brick::bricks_in_dir;

impl Run for List {
    fn run(&self) {
        let config = CraneConfig::new();
        let brick_dirs = if let Some(brick_dirs) = &self.brick_dirs
            && brick_dirs.is_empty()
        {
            brick_dirs
        } else {
            &config.brick_dirs().to_vec()
        };

        let alias_mapped = map_aliases(config.alias());

        for brick_dir in brick_dirs {
            println!(
                "{} Found brick directory at {}",
                "→".green(),
                fs::canonicalize(brick_dir)
                    .unwrap_or(brick_dir.to_path_buf())
                    .display()
            );
            for brick in bricks_in_dir(brick_dir) {
                let mut affix = String::new();
                if let Some(aliases) = alias_mapped.get(brick.name()) {
                    affix = format!(" (aliased in '{}')", aliases.join("', '"));
                }
                info!("{}{}", brick.name(), affix.dimmed());
            }
            println!()
        }
    }
}
