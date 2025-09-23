use log::info;

use crate::cmd::{List, Run};
use crane_bricks::brick::bricks;

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
