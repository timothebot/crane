mod add;
mod cmd;
mod list;

pub use crate::cmd::cmd::*;

pub trait Run {
    fn run(&self);
}

impl Run for CraneCli {
    fn run(&self) {
        match &self.command {
            CraneCommand::Add(cmd) => cmd.run(),
            CraneCommand::List(cmd) => cmd.run()
        }
    }
}
