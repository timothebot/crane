use std::{fs, path::PathBuf};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{
    files::BrickFile,
    utils::{sub_dirs, sub_paths},
};

const BRICK_CONFIG_FILE: &'static str = "brick.toml";

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum FileAction {
    #[default]
    Replace,
    Append,
    // Regex {
    //     regex: String,
    //     position: After | Replace | Before,
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BrickConfig {
    pub name: String,
    #[serde(default)]
    pub action: FileAction,
    // pub requires: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Brick {
    config: BrickConfig,
    path: PathBuf,
}

impl Brick {
    pub fn new(name: String, path: PathBuf) -> Self {
        Brick {
            config: BrickConfig {
                name,
                ..BrickConfig::default()
            },
            path,
        }
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn action(&self) -> &FileAction {
        &self.config.action
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

                Some(BrickFile::new(name, content, self.action().clone()))
            })
            .collect()
    }
}

impl TryFrom<PathBuf> for Brick {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let config_file = value.join(BRICK_CONFIG_FILE);
        if !config_file.exists() {
            let name = value
                .as_path()
                .file_name()
                .ok_or_else(|| anyhow!("Could not read brick dir name!"))?;
            return Ok(Brick::new(name.display().to_string(), value));
        }
        let config: BrickConfig = toml::from_str(fs::read_to_string(config_file)?.as_str())?;
        Ok(Brick::new(config.name, value))
    }
}

pub fn bricks(dir: &PathBuf) -> Vec<Brick> {
    let Ok(dirs) = sub_dirs(dir) else {
        return vec![];
    };
    dirs.iter()
        .filter_map(|dir| Brick::try_from(dir.clone()).ok())
        .collect::<Vec<Brick>>()
}
