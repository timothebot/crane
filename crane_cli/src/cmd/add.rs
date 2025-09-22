use std::{env, path::PathBuf};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use log::{debug, error, info, warn};

use crane_bricks::brick::{Brick, bricks};
use crate::{
    cmd::{Add, Run},
    config::CraneConfig,
};

impl Run for Add {
    fn run(&self) {
        let config = CraneConfig::new();
        let brick_dirs = if self.brick_dirs.len() > 0 {
            &self.brick_dirs
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

        let bricks: Vec<Brick> = brick_dirs.iter().map(|dir| bricks(dir)).flatten().collect();

        debug!(
            "Found bricks:\n* {}",
            bricks
                .iter()
                .map(|brick| brick.name().to_string())
                .collect::<Vec<String>>()
                .join("\n* ")
        );

        let matcher = SkimMatcherV2::default();

        for brick_query in &self.bricks {
            let mut matches: Vec<(Brick, i64)> = Vec::new();
            let mut highest_score: i64 = 0;
            for brick in &bricks {
                if let Some(score) = matcher.fuzzy_match(brick.name(), brick_query.as_str()) {
                    if score >= highest_score {
                        matches.push((brick.clone(), score));
                        highest_score = score;
                    }
                }
            }
            if matches.len() == 1 {
                add_brick(
                    matches.first().unwrap().0.clone(),
                    &target_dir,
                    self.dry_run,
                );
            } else if matches.len() > 1 {
                multiple_matches_found(brick_query.to_string(), matches);
            } else {
                no_matches_found(brick_query.to_string());
            }
        }
    }
}

fn add_brick(brick: Brick, target_dir: &PathBuf, dry_run: bool) {
    info!("Adding brick '{}', {}, {:?}", brick.name(), dry_run, target_dir);
    
}

fn no_matches_found(query: String) {
    error!("No possible bricks found for '{}'", query);
}

fn multiple_matches_found(query: String, matches: Vec<(Brick, i64)>) {
    warn!(
        "Multiple possible bricks found for '{}'\n* {}",
        query,
        matches
            .iter()
            .map(|(brick, score)| format!("{} ({})", brick.name(), score))
            .collect::<Vec<String>>()
            .join("\n* ")
    );
}
