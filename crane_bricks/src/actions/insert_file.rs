use std::path::Path;

use log::debug;
use serde::Deserialize;

use crate::{
    actions::{ExecuteAction, common::Common},
    brick::Brick,
    context::ActionContext,
    file_utils::{file_append_content, file_create_new, file_replace_content},
};

/// Creates a new file.
///
/// ## Example
///
/// ### Config
///
/// ```toml
/// [[actions]]
/// sources = [
///     "LICENSE"
/// ]
/// if_file_exists = "replace"
/// ```
///
/// ### Result
///
/// Will create the LICENSE file. If it already exists, it replaces it.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct InsertFileAction {
    #[serde(flatten)]
    pub common: Common,

    /// Define what happens if the file already exists
    #[serde(default)]
    pub if_file_exists: FileExistsAction,
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileExistsAction {
    #[default]
    Append,
    Replace,
    Pass,
}

impl ExecuteAction for InsertFileAction {
    fn execute(
        &self,
        context: &ActionContext,
        brick: &Brick,
        cwd: &Path,
    ) -> anyhow::Result<()> {
        let mut files = brick.files();
        if !&self.common.sources.is_empty() {
            files = files
                .into_iter()
                .filter(|file| {
                    *&self.common.sources.contains(&file.name().to_string())
                })
                .collect();
        }
        debug!("{} executing for {} files", brick.name(), files.len());
        for file in files {
            let target_path = cwd.join(file.name());
            let content = file.content().to_string();
            if !target_path.exists() {
                file_create_new(context, &target_path, Some(content))?;
                continue;
            }
            match &self.if_file_exists {
                FileExistsAction::Append => {
                    file_append_content(context, &target_path, &content)?
                }
                FileExistsAction::Replace => {
                    file_replace_content(context, &target_path, &content)?
                }
                FileExistsAction::Pass => continue,
            }
        }
        Ok(())
    }
}
