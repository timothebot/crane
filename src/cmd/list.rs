use log::info;

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
            info!("{:?}", brick);
            info!("{:#?}", brick.files());
        }
    }
}
