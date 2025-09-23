use anyhow::{Ok, anyhow};
use log::debug;
use serde::Deserialize;

use crate::{
    actions::{ExecuteAction, common::Common},
    file_utils::{file_read_content, file_replace_content},
};

/// Modify a file by inserting content at a specific location.
///
/// ## Example
///
/// ### Config
///
/// ```toml
/// [[actions]]
/// # For this action, the name of the files that the modification
/// # will apply
/// sources = [
///     "Cargo.toml"
/// ]
/// type = "append"
/// content = "\nserde = \"1\""
/// selector = "[dependencies]"
/// ```
///
/// ### Result
///
/// ```toml
/// # Before
/// [dependencies]
/// crane = "9.9.9"
///
/// # After
/// [dependencies]
/// serde = "1"
/// crane = "9.9.9"
/// ```
#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct ModifyFileAction {
    #[serde(flatten)]
    pub common: Common,

    /// If the modification should append or prepend text next to the
    /// selector or if it should replace it.
    pub(self) r#type: ModifyType,

    pub content: Option<String>,

    /// The content selector for the modification, must be unique.
    /// Can be regex if prefix with "re:".
    pub selector: String,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ModifyType {
    #[default]
    Append,
    Prepend,
    Replace,
}

impl ModifyFileAction {
    pub fn content(&self) -> String {
        // TODO: Get content from somewhere else if not set
        self.content.clone().unwrap_or_default()
    }

    pub fn modify_content(&self, source_text: String) -> anyhow::Result<String> {
        // TODO: Handle regex
        // TODO: insert for all or just one?

        let locations: Vec<(usize, &str)> =
            source_text.match_indices(&self.selector).collect();

        debug!("Found {} matches", locations.len());
        if locations.len() == 0 {
            return Err(anyhow!("No selector matches in target file!"));
        }

        let mut output = source_text.clone();
        let start_length = source_text.len();

        for (index, selected) in locations {
            // This is to account for new inserted text, which
            // means the index has shifted.
            let modified_index = index + output.len().abs_diff(start_length);
            match &self.r#type {
                ModifyType::Append => {
                    output.insert_str(
                        (modified_index + selected.len()).max(0),
                        &self.content(),
                    );
                },
                ModifyType::Prepend => {
                    output.insert_str(modified_index, &self.content());
                },
                ModifyType::Replace => {
                    output.replace_range(modified_index..(modified_index + selected.len()).max(0), &self.content());
                }
            }
        }
        Ok(output)
    }
}

impl ExecuteAction for ModifyFileAction {
    fn execute(
        &self,
        context: &crate::context::ActionContext,
        brick: &crate::brick::Brick,
        cwd: &std::path::Path,
    ) -> anyhow::Result<()> {
        let mut files: Vec<String> = brick
            .files()
            .iter()
            .map(|brick_file| brick_file.name().to_string())
            .collect();
        files.extend(self.common.sources.clone().into_iter());
        for file in files {
            let target_path = cwd.join(file);
            if !target_path.exists() {
                return Err(anyhow!("Target file does not exist!"));
            }
            let content = file_read_content(context, &target_path)?;
            file_replace_content(context, &target_path, &self.modify_content(content)?)?;
        }
        Ok(())
    }
}
