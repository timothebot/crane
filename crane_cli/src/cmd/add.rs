use std::{env, path::PathBuf};

use colored::Colorize;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use log::{debug, error, info, warn};

use crate::{
    cmd::{Add, Run},
    config::CraneConfig,
};
use crane_bricks::{
    brick::{Brick, bricks},
    context::ActionContext,
};

impl Run for Add {
    fn run(&self) {
        let config = CraneConfig::new();
        let brick_dirs = if let Some(brick_dirs) = &self.brick_dirs
            && brick_dirs.len() > 0
        {
            brick_dirs
        } else {
            &config.brick_dirs().to_vec()
        };
        debug!(
            "Checking brick dirs:\n* {}",
            brick_dirs
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<String>>()
                .join("\n* ")
        );

        let target_dir = match &self.target_dir {
            Some(dir) => dir,
            None => &env::current_dir().unwrap(),
        };

        let bricks: Vec<Brick> =
            brick_dirs.iter().map(|dir| bricks(dir)).flatten().collect();

        debug!(
            "Found bricks:\n* {}",
            bricks
                .iter()
                .map(|brick| brick.name().to_string())
                .collect::<Vec<String>>()
                .join("\n* ")
        );

        let brick_queries: Vec<String> = self
            .bricks
            .iter()
            .map(|query| {
                for alias in config.alias() {
                    if alias.name().to_lowercase() == query.to_lowercase() {
                        return alias.bricks().to_vec();
                    }
                }
                return vec![query.clone()];
            })
            .flatten()
            .collect();

        let mut bricks_to_execute: Vec<&Brick> = Vec::new();
        for brick_query in brick_queries {
            let mut found = false;
            for brick in &bricks {
                if brick.name().to_lowercase() == brick_query.to_lowercase() {
                    bricks_to_execute.push(brick);
                    found = true;
                    break;
                }
            }
            if !found {
                eprintln!("{} Could not find brick '{}'", "⚠".red(), brick_query);
            }
        }
        /* TODO: render aliases like this:
        → Executing 4 bricks
          • MIT
          • rust (alias)
            ◦ author-rust
            ◦ serde
            • rustfmt
        */
        let plural = if bricks_to_execute.len() > 1 {
            "s"
        } else {
            ""
        };
        println!(
            "{} Executing {} brick{}",
            "→".green(),
            bricks_to_execute.len().to_string().purple(),
            plural
        );
        for brick in &bricks_to_execute {
            println!("  {} {}", "•".dimmed(), brick.name())
        }

        let context = ActionContext::new(self.dry_run);
        for brick in bricks_to_execute {
            execute_brick(brick, &context, &target_dir);
        }
    }
}

fn execute_brick(brick: &Brick, context: &ActionContext, cwd: &PathBuf) {
    println!(
        "\n{} Executing brick '{}'",
        "→".green(),
        brick.name().purple()
    );
    match brick.execute(context, &cwd) {
        Ok(_) => println!(
            "{}",
            format!("✔ Successfully executed '{}'! ◝(°ᗜ°)◜", brick.name().bold()).green()
        ),
        Err(_) => eprintln!(
            "{}",
            format!("✘ Failed to execute '{}'! ヽ(°〇°)ﾉ", brick.name().bold()).red()
        ),
    }
}
