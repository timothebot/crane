use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Common {
    /// Relative path from where you run crane to where the files should go
    /// 
    /// ```toml
    /// [[actions]]
    /// working_dir = "./src/"
    /// ```
    #[serde(default)]
    pub working_dir: Option<String>,

    /// List of paths including for which files the action should run
    /// Empty means all files (except config file) will be included
    /// 
    /// ```toml
    /// [[actions]]
    /// sources = [ "README.md", "LICENSE" ]
    /// 
    /// # Or regex
    /// sources = [ "re:.+\.md", "LICENSE"]
    /// ```
    #[serde(default)]
    pub sources: Vec<String>
}
