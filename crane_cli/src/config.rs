use std::{env, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

const ENV_KEY_CONFIG_DIR: &'static str = "CRANE_CONFIG_DIR";

fn config_path_from_env() -> anyhow::Result<PathBuf> {
    Ok(PathBuf::try_from(env::var(ENV_KEY_CONFIG_DIR)?)?)
}

pub fn config_dir() -> PathBuf {
    match config_path_from_env() {
        Ok(path) => path,
        Err(_) => PathBuf::from("~/.config/crane"),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alias {
    name: String,
    bricks: Vec<String>,
}

impl Alias {
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn bricks(&self) -> &[String] {
        &self.bricks
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CraneConfig {
    brick_dirs: Vec<PathBuf>,
    alias: Vec<Alias>,
}

impl CraneConfig {
    pub fn new() -> Self {
        let config_file = config_dir().join("config.toml");
        if config_file.exists() {
            if let Ok(parsed_config) = toml::from_str::<CraneConfig>(
                fs::read_to_string(config_file).unwrap_or_default().as_str(),
            ) {
                return parsed_config;
            }
        }
        Self {
            brick_dirs: vec![config_dir().join("bricks")],
            alias: vec![]
        }
    }

    pub fn brick_dirs(&self) -> &[PathBuf] {
        &self.brick_dirs
    }

    pub fn alias(&self) -> &Vec<Alias> {
        &self.alias
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir_from_env() {
        // should be safe as long as it is only one test (of this kind)
        unsafe {
            env::set_var(ENV_KEY_CONFIG_DIR, "~/.crane");
        };
        assert_eq!(
            format!("{}", config_dir().display()),
            String::from("~/.crane")
        )
    }
}
