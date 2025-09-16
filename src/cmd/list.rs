use std::path::PathBuf;

use crate::{bricks::bricks, cmd::{List, Run}};

impl Run for List {
    fn run(&self) {
        let Some(path) = self.dir.clone() else {
            return;
        };
        println!("{:?}", bricks(&path));
    }
}
