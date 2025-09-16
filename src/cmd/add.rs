use std::env;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use crate::{bricks::{bricks, Brick}, cmd::{Add, Run}};

impl Run for Add {
    fn run(&self) {
        println!("{:?}", env::current_dir().unwrap());
        let Some(path) = self.dir.clone() else {
            return;
        };

        let matcher = SkimMatcherV2::default();

        let bricks = bricks(&path);
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
                add_brick(matches.first().unwrap().0.clone())
            } else if matches.len() > 1 {
                multiple_matches_found(brick_query.to_string(), matches);
            } else {
                no_matches_found(brick_query.to_string());
            }


        }
    }
}

fn add_brick(brick: Brick) {
    println!("+ | Adding brick '{}'", brick.name());
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
