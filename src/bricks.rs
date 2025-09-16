use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum InsertAlgorithm {
    #[default]
    Replace,
    Append,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Brick {
    name: String,
    path: PathBuf,
    // group: Vec<String>, like licenses / oss / <brick>
    #[serde(default)]
    insert: InsertAlgorithm,
}

impl Brick {
    pub fn new(name: String, path: PathBuf) -> Self {
        Brick {
            name,
            path,
            insert: InsertAlgorithm::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn insert_algorithm(&self) -> InsertAlgorithm {
        self.insert
    }
}

impl TryFrom<PathBuf> for Brick {
    type Error = String;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        // TODO: check if there is a brick.toml file in this folder
        let name = value
            .as_path()
            .file_name()
            .ok_or_else(|| "Could not read brick dir name!")?;
        Ok(Brick::new(name.display().to_string(), value))
    }
}

pub fn bricks(dir: &PathBuf) -> Vec<Brick> {
    if !dir.exists() || !dir.is_dir() {
        return vec![];
    }
    let Ok(dirs) = dir.read_dir() else {
        return vec![];
    };
    let brick_dirs: Vec<Brick> = dirs
        .filter_map(|entry_res| {
            let entry = entry_res.ok()?;
            if !entry.path().is_dir() {
                return None;
            }
            Brick::try_from(entry.path()).ok()
        })
        .collect();
    brick_dirs
}
