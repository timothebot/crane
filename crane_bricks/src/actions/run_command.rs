use std::{path::Path, process::Command};

use serde::Deserialize;

use crate::{
    actions::{ExecuteAction, common::Common},
    brick::Brick,
    context::ActionContext,
};

/// Run a command
///
/// ## Example
///
/// ### Config
///
/// ```toml
/// [[actions]]
/// command = "echo hi > test.txt"
/// ```
///
/// ### Result
///
/// Will run echo hi and write the stdout into test.txt
#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct RunCommandAction {
    #[serde(flatten)]
    pub common: Common,

    pub command: String,
}

impl ExecuteAction for RunCommandAction {
    fn execute(
        &self,
        context: &ActionContext,
        _brick: &Brick,
        cwd: &Path,
    ) -> anyhow::Result<()> {
        if context.dry_run {
            return Ok(());
        }
        Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .current_dir(cwd)
            .output()?;

        Ok(())
    }
}
