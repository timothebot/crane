use std::{
    fs::{self, File, create_dir_all},
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use crate::context::ActionContext;

pub fn sub_dirs(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    Ok(sub_paths(dir)?
        .into_iter()
        .filter(|path| path.is_dir())
        .collect::<Vec<PathBuf>>())
}

/// Returns the path to every file located in the given dir, including
/// paths in subdirectories
pub fn all_file_paths(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let dir = PathBuf::from(shellexpand::tilde(&dir.display().to_string()).to_string());
    if !dir.exists() || !dir.is_dir() {
        return Err(anyhow!("Target does not exist or not a directory"));
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    if let Ok(children) = sub_paths(dir.as_path()) {
        for path in children {
            if path.is_dir() {
                paths.append(&mut all_file_paths(path.as_path()).unwrap());
            } else if path.is_file() {
                paths.push(path);
            }
        }
    }
    Ok(paths)
}

/// Returns a vec of all files and folders in the given dir (without subdirs) if valid
pub fn sub_paths(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let dir = PathBuf::from(shellexpand::tilde(&dir.display().to_string()).to_string());
    if !dir.exists() || !dir.is_dir() {
        return Err(anyhow!("Target does not exist or not a directory"));
    }
    let dirs = dir.read_dir()?;
    Ok(dirs
        .filter_map(|entry_res| Some(entry_res.ok()?.path()))
        .collect())
}

pub fn file_create_new(
    ctx: &ActionContext,
    path: &Path,
    content: Option<String>,
) -> anyhow::Result<()> {
    if ctx.dry_run {
        return Ok(());
    }
    debug!("Creating new file '{:?}'", path);
    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            debug!("Adding all missing parent dir(s).");
            match create_dir_all(parent_dir) {
                Ok(_) => {},
                Err(_) => return Err(anyhow!("Couldn't create parent dir(s)!")),
            }
        }
    } else {
        debug!("Parent dirs exists");
    }
    let mut file = File::create_new(path)?;
    file.write_all(content.unwrap_or_default().as_bytes())?;
    Ok(())
}

pub fn file_read_content(ctx: &ActionContext, path: &Path) -> anyhow::Result<String> {
    if ctx.dry_run && !path.exists() {
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

pub fn file_replace_content(
    ctx: &ActionContext,
    path: &Path,
    content: &String,
) -> anyhow::Result<()> {
    debug!("Replacing contents of '{:?}'", path.display());
    if ctx.dry_run {
        return Ok(());
    }
    let mut file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn file_append_content(
    ctx: &ActionContext,
    path: &Path,
    content: &String,
) -> anyhow::Result<()> {
    if ctx.dry_run {
        return Ok(());
    }
    let mut file = File::options().append(true).create(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
