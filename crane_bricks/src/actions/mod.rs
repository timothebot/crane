pub mod common;
pub mod insert_file;
pub mod modify_file;
pub mod run_command;

use std::path::Path;

use serde::Deserialize;

use crate::{
    actions::{insert_file::InsertFileAction, modify_file::ModifyFileAction, run_command::RunCommandAction},
    brick::Brick,
    context::ActionContext,
};

pub trait ExecuteAction {
    fn execute(
        &self,
        context: &ActionContext,
        brick: &Brick,
        cwd: &Path,
    ) -> anyhow::Result<()>;
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Action {
    InsertFile(InsertFileAction),
    ModifyFile(ModifyFileAction),
    RunCommand(RunCommandAction),
}

impl ExecuteAction for Action {
    fn execute(
        &self,
        context: &ActionContext,
        brick: &Brick,
        cwd: &Path,
    ) -> anyhow::Result<()> {
        debug!("Executing '{}' brick action '{:#?}'", brick.name(), &self);
        match &self {
            Action::InsertFile(action) => action.execute(context, brick, cwd),
            Action::ModifyFile(action) => action.execute(context, brick, cwd),
            Action::RunCommand(action) => action.execute(context, brick, cwd),
        }
    }
}
