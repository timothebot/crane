use std::env;

use crate::{
    bricks::bricks,
    cmd::{List, Run},
};

impl Run for List {
    fn run(&self) {
        let Some(path) = self.brick_dirs.clone() else {
            return;
        };
        for brick in bricks(&path) {
            println!("{:?}", brick);
            println!("{:#?}", brick.files());
            
        }
    }
}
