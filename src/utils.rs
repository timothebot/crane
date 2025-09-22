use std::path::{Path, PathBuf};

use anyhow::anyhow;

pub fn sub_dirs(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    Ok(
        sub_paths(dir)?
            .into_iter()
            .filter(|path| path.is_dir())
            .collect::<Vec<PathBuf>>()
    )
}

/// Get a vec of all files and folders in the given dir if valid
pub fn sub_paths(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() || !dir.is_dir() {
        return Err(anyhow!("Target does not exist or not a directory"));
    }
    let dirs = dir.read_dir()?;
    Ok(
        dirs.filter_map(|entry_res| Some(entry_res.ok()?.path()))
            .collect(),
    )
}
