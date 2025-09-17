use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};


use anyhow::Error;

use crate::bricks::FileAction;

/// The object that contains all information except the target location
/// to execute a brick
#[derive(Debug, Clone)]
pub struct BrickFile {
    name: String,
    content: String,
    action: FileAction,
}

impl BrickFile {
    pub fn new(name: String, content: String, action: FileAction) -> Self {
        Self {
            name,
            content,
            action,
        }
    }

    /// Check if this file can be executed without the target
    /// file already existing
    fn needs_existing_file(&self) -> bool {
        match &self.action {
            FileAction::Replace | FileAction::Append => false,
        }
    }

    fn parse_content(&self) -> &String {
        &self.content
    }

    // TODO: Create with some kind of context?
    pub fn create(&self, path: PathBuf) -> anyhow::Result<()> {
        let target_file = path.join(&self.name);
        if !target_file.exists() {
            if self.needs_existing_file() {
                return Err(Error::msg("File does not exist"));
            }
            println!("create file: {:?}", target_file);
            File::create_new(&target_file).unwrap();

        }
        let file_options = File::options().write(true).read(true).open(&target_file);
        let Ok(mut file) = file_options else {
            return Err(Error::msg("Can't open file"));
        };

        let contents = fs::read_to_string(path).unwrap_or_default();

        let _ = match self.action {
            FileAction::Replace => file.write(self.parse_content().as_bytes()),
            FileAction::Append => {
                file.write(format!("{}{}", contents, &self.parse_content()).as_bytes())
            }
        };

        Ok(())
    }
}
