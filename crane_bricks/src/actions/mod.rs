pub mod common;
pub mod insert_file;
pub mod modify_file;

use std::path::Path;

use log::debug;
use serde::Deserialize;

use crate::{
    actions::{insert_file::InsertFileAction, modify_file::ModifyFileAction},
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
        }
    }
}
