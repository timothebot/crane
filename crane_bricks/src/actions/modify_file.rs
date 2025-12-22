use anyhow::{Ok, anyhow};
use regex::Regex;
use serde::Deserialize;

use crate::{
    actions::{ExecuteAction, common::Common},
    brick::BrickFile,
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

fn modify_content_using_regex(
    source_text: String,
    selector: &str,
    content: String,
    modify_type: &ModifyType,
) -> anyhow::Result<String> {
    let re = Regex::new(selector)?;
    return match modify_type {
        ModifyType::Append => Ok(re
            .replace_all(
                &source_text,
                format!("$0{}", content)
            )
            .to_string()),
        ModifyType::Prepend => Ok(re
            .replace_all(
                &source_text,
                format!("{}$0", content)
            )
            .to_string()),
        ModifyType::Replace => Ok(re
            .replace_all(
                &source_text,
                content
            )
            .to_string()),
    };
}

impl ModifyFileAction {
    pub fn content(&self, brick: &crate::brick::Brick) -> anyhow::Result<String> {
        let content = self.content.clone().unwrap_or_default();
        if let Some(file) = content.strip_prefix("file:") {
            let content_file: Vec<BrickFile> = brick
                .files()
                .into_iter()
                .filter(|f| f.name() == file)
                .collect();
            if content_file.len() != 1 {
                return Err(anyhow!(
                    "The file '{}' that was defined for taking the content from does not exis, or multiple results found. (Found {})",
                    file,
                    content_file.len()
                ));
            }
            return Ok(content_file.first().unwrap().content().to_string());
        };
        Ok(content)
    }

    pub fn modify_content(
        &self,
        brick: &crate::brick::Brick,
        source_text: String,
    ) -> anyhow::Result<String> {
        // TODO: Handle regex
        // TODO: insert for all or just one?

        let content = &self.content(brick)?;

        if self.selector.starts_with("regex:") {
            info!("Modifying content with regex selector.");
            return modify_content_using_regex(
                source_text,
                self.selector.strip_prefix("regex:").unwrap_or_default(),
                content.to_string(),
                &self.r#type,
            );
        }

        let locations: Vec<(usize, &str)> =
            source_text.match_indices(&self.selector).collect();

        if locations.is_empty() {
            return Err(anyhow!("No selector matches in target file!"));
        }
        if locations.len() > 1 {
            info!("Found {} matches", locations.len());
        } else {
            info!("Found {} match", locations.len());
        }

        let mut output = source_text.clone();
        let start_length = source_text.len();

        for (index, selected) in locations {
            // This is to account for new inserted text, which
            // means the index has shifted.
            let modified_index = index + output.len().abs_diff(start_length);
            match &self.r#type {
                ModifyType::Append => {
                    output.insert_str((modified_index + selected.len()).max(0), &content);
                }
                ModifyType::Prepend => {
                    output.insert_str(modified_index, &content);
                }
                ModifyType::Replace => {
                    debug!(
                        "Replacing from {} to {}",
                        modified_index,
                        (modified_index + selected.len()).max(0)
                    );
                    if modified_index > output.len() {
                        output.insert_str(output.len(), &content);
                    } else {
                        output.replace_range(
                            modified_index..(modified_index + selected.len()),
                            &content,
                        );
                    }
                }
            }
        }
        match &self.r#type {
            ModifyType::Append => {
                info!("Appended to all matches");
            }
            ModifyType::Prepend => {
                info!("Prepended to all matches");
            }
            ModifyType::Replace => {
                info!("Replaced all matches");
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
        let files: Vec<String> = self.common.sources.clone();
        for file in files {
            let target_path = cwd.join(file);
            if !target_path.exists() {
                return Err(anyhow!("Target file does not exist!"));
            }
            info!("Modifying file '{}'", target_path.display());
            let content = file_read_content(context, &target_path)?;
            file_replace_content(
                context,
                &target_path,
                &self.modify_content(brick, content)?,
            )?;
        }
        Ok(())
    }
}
