use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use serde::Deserialize;

use crate::{
    actions::{insert_file::InsertFileAction, Action, ExecuteAction},
    context::ActionContext,
    file_utils::{sub_dirs, sub_paths},
};

const BRICK_CONFIG_FILE: &'static str = "brick.toml";

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct BrickConfig {
    name: String,

    #[serde(default)]
    actions: Vec<Action>,
}

impl BrickConfig {
    pub fn new(name: String, actions: Vec<Action>) -> Self {
        Self { name, actions }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
}

#[derive(Debug, Clone)]
pub struct BrickFile {
    name: String,
    content: String,
}

impl BrickFile {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, Clone)]
pub struct Brick {
    config: BrickConfig,
    source_path: PathBuf,
}

impl Brick {
    pub fn new(name: String, source_path: PathBuf) -> Self {
        Brick {
            config: BrickConfig {
                name,
                // If no action is configured, InsertFileAction is default
                actions: vec![
                    Action::InsertFile(InsertFileAction::default())
                ],
                ..BrickConfig::default()
            },
            source_path,
        }
    }

    pub fn new_with_config(config: BrickConfig, source_path: PathBuf) -> Self {
        Brick {
            config,
            source_path,
        }
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.source_path
    }

    pub fn config(&self) -> &BrickConfig {
        &self.config
    }

    pub fn execute(&self, context: &ActionContext, cwd: &Path) -> anyhow::Result<()> {
        for action in &self.config.actions {
            action.execute(context, &self, cwd)?;
        }
        Ok(())
    }

    /// Returns a list of all files that
    pub fn files(&self) -> Vec<BrickFile> {
        let Ok(paths) = sub_paths(&self.path()) else {
            return vec![];
        };
        paths
            .iter()
            .filter_map(|path| {
                let name = path.file_name()?.display().to_string();
                if !path.is_file() || name == BRICK_CONFIG_FILE {
                    return None;
                }
                let content = fs::read_to_string(path).unwrap_or_default();

                Some(BrickFile::new(name, content))
            })
            .collect()
    }
}

impl TryFrom<PathBuf> for Brick {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let config_file = value.join(BRICK_CONFIG_FILE);
        if !config_file.exists() {
            debug!(
                "Brick config file not found at '{:?}'",
                config_file.display()
            );
            let name = value
                .as_path()
                .file_name()
                .ok_or_else(|| anyhow!("Could not read brick dir name!"))?;
            return Ok(Brick::new(name.display().to_string(), value));
        }
        debug!("Creating Brick from config file");
        let config: BrickConfig =
            toml::from_str(fs::read_to_string(config_file)?.as_str())?;
        Ok(Brick::new_with_config(config, value))
    }
}

/// Get all bricks in a directory
pub fn bricks(dir: &PathBuf) -> Vec<Brick> {
    let Ok(dirs) = sub_dirs(dir) else {
        return vec![];
    };
    dirs.iter()
        .filter_map(|dir| {
            match Brick::try_from(dir.clone()) {
                Ok(brick) => Some(brick),
                Err(error) => {
                    warn!("Failed to create brick at '{}'. Error: {}", dir.display(), error);
                    None
                },
            }
        })
        .collect::<Vec<Brick>>()
}
