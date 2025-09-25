use std::path::Path;

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
#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
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
                .filter(|file| *&self.common.sources.contains(&file.name().to_string()))
                .collect();
        }
        debug!("{} executing for {} files", brick.name(), files.len());
        if files.len() > 1 {
            info!(
                "Inserting files: '{}'",
                files
                    .iter()
                    .map(|file| file.name().to_string())
                    .collect::<Vec<String>>()
                    .join("', '")
            )
        } else if files.len() == 1 {
            info!("Inserting file '{}'", files.first().unwrap().name());
        } else {
            warn!("No files found to insert!");
        }
        for file in files {
            let target_path = cwd.join(file.name());
            let content = file.content().to_string();
            if !target_path.exists() {
                info!("Created file '{}'", file.name());
                file_create_new(context, &target_path, Some(content))?;
                continue;
            }
            warn!("File '{}' already exists", file.name());
            match &self.if_file_exists {
                FileExistsAction::Append => {
                    info!("Appending content to file");
                    file_append_content(context, &target_path, &content)?
                }
                FileExistsAction::Replace => {
                    info!("Replacing all content of file");
                    file_replace_content(context, &target_path, &content)?
                }
                FileExistsAction::Pass => {
                    info!("Continuing");
                    continue
                },
            }
        }
        Ok(())
    }
}
