use crate::{
    cmd::{Completion, Run},
};

impl Run for Completion {
    fn run(&self) {
        println!("{}", self.shell.to_string());
    }
}
