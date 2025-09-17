use std::{env, path::PathBuf};

use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

use crate::{
    bricks::{Brick, bricks},
    cmd::{Add, Run},
    config::CraneConfig,
};

impl Run for Add {
    fn run(&self) {
        println!("{:?}", env::current_dir().unwrap());
        let config = CraneConfig::new();
        let brick_dirs = if self.brick_dirs.len() > 0 {
            &self.brick_dirs
        } else {
            &config.brick_dirs().to_vec()
        };

        let target_dir = match &self.target_dir {
            Some(dir) => dir,
            None => &env::current_dir().unwrap()
        };

        let bricks: Vec<Brick> = brick_dirs.iter().map(|dir| bricks(dir)).flatten().collect();

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
                add_brick(matches.first().unwrap().0.clone(), &target_dir);
            } else if matches.len() > 1 {
                multiple_matches_found(brick_query.to_string(), matches);
            } else {
                no_matches_found(brick_query.to_string());
            }
        }
    }
}

fn add_brick(brick: Brick, target_dir: &PathBuf) {
    println!("+ | Adding brick '{}'", brick.name());
    for file in brick.files() {
        match file.create(target_dir.clone()) {
            Ok(_) => println!("worked"),
            Err(err) => println!("{}", err),
        }
    }
}

fn no_matches_found(query: String) {
    println!("! | Found no possible bricks for '{query}'");
}

fn multiple_matches_found(query: String, matches: Vec<(Brick, i64)>) {
    println!("! | Found multiple possible bricks for '{query}'");
    for (brick, score) in matches {
        println!("    - {} ({})", brick.name(), score)
    }
}
