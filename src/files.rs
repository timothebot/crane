use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Ok;
use log::{debug, info};

use crate::bricks::FileAction;

/// The object that contains all information except the target location
/// to execute a brick
#[derive(Debug, Clone)]
pub struct BrickFile {
    name: String,
    content: String,
    action: FileAction,
}

struct FileUtility {
    dry_run: bool,
}

impl FileUtility {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    pub fn create_new(&self, path: &Path) -> anyhow::Result<()> {
        if !self.dry_run {
            File::create_new(path)?;
        }
        Ok(())
    }

    pub fn read_content(&self, path: &Path) -> anyhow::Result<String> {
        if self.dry_run && !path.exists() {
            return Ok(String::new());
        }
        if !path.exists() {
            return Err(anyhow::Error::new(io::Error::new(
                io::ErrorKind::NotFound,
                "Target file not found",
            )));
        }
        debug!("Reading content of file");
        Ok(fs::read_to_string(path).unwrap_or_default())
    }

    pub fn replace_content(&self, path: &Path, content: &String) -> anyhow::Result<()> {
        debug!("Replacing contents of '{:?}'", path.display());
        if self.dry_run {
            return Ok(());
        }
        let mut file = File::options().write(true).create(true).open(&path)?;
        file.write(content.as_bytes())?;
        Ok(())
    }
    pub fn append_content(&self, path: &Path, content: &String) -> anyhow::Result<()> {
        if self.dry_run {
            return Ok(());
        }
        let mut file = File::options().append(true).create(true).open(&path)?;
        file.write(content.as_bytes())?;
        Ok(())
    }
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
    pub fn create(&self, path: PathBuf, dry_run: bool) -> anyhow::Result<()> {
        let file_util = FileUtility::new(dry_run);
        let target_file = path.join(&self.name);
        if !target_file.exists() {
            if self.needs_existing_file() {
                return Err(anyhow::Error::msg("File does not exist"));
            }
            info!(
                "Target file does not exist, creating it at '{:?}'",
                target_file
            );
            file_util.create_new(&target_file).unwrap();
        }

        // let content = file_util.read_content(&path).unwrap_or_default();

        match self.action {
            FileAction::Replace => {
                file_util.replace_content(&path, &self.parse_content())?;
            }
            FileAction::Append => {
                file_util.append_content(&path, &self.parse_content())?;
            }
        };

        Ok(())
    }
}
