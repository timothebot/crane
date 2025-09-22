use serde::Deserialize;

use crate::actions::common::Common;

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
/// location = "after"
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

    /// If the modification should append something next to the
    /// selector or if it should replace it.
    pub r#type: ModifyType,

    pub content: Option<String>,

    /// The content selector for the modification, must be unique.
    /// Can be regex if prefix with "re:".
    pub selector: Option<String>,

    /// If the modification should happen "before" or "after" (default)
    /// the given selector.
    pub location: ModifyLocation,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ModifyType {
    #[default]
    Append,
    Replace,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ModifyLocation {
    #[default]
    After,
    Before,
}
