use std::{collections::HashMap, env, fs, path::PathBuf};

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
    #[allow(dead_code)]
    pub fn new(name: String, bricks: Vec<String>) -> Self {
        Self { name, bricks }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bricks(&self) -> &[String] {
        &self.bricks
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CraneConfig {
    #[serde(default)]
    brick_dirs: Vec<PathBuf>,

    #[serde(default)]
    alias: Vec<Alias>,
}

impl CraneConfig {
    pub fn new() -> Self {
        let cnf_dir = config_dir();
        let config_file = cnf_dir.join("config.toml");
        let mut config = {
            if config_file.exists()
                && let Ok(parsed_config) = toml::from_str::<CraneConfig>(
                    fs::read_to_string(config_file).unwrap_or_default().as_str(),
                )
            {
                parsed_config
            } else {
                CraneConfig::default()
            }
        };

        if config.brick_dirs.is_empty() {
            config.brick_dirs.push(PathBuf::from("./bricks"));
        }

        config.brick_dirs = config
            .brick_dirs()
            .into_iter()
            .map(|brick_dir| {
                if brick_dir.is_absolute() {
                    brick_dir.clone()
                } else {
                    cnf_dir.join(brick_dir)
                }
            })
            .collect();
        config
    }

    pub fn brick_dirs(&self) -> &[PathBuf] {
        &self.brick_dirs
    }

    pub fn alias(&self) -> &[Alias] {
        &self.alias
    }
}

/// Converts a list of aliases to a map where the brick
/// name is the key and value all aliases
pub fn map_aliases(aliases: &[Alias]) -> HashMap<String, Vec<String>> {
    let mut brick_map: HashMap<String, Vec<String>> = HashMap::new();
    for alias in aliases {
        for brick in alias.bricks() {
            if brick_map.contains_key(brick) {
                brick_map
                    .get_mut(brick)
                    .unwrap()
                    .push(alias.name().to_string());
            } else {
                brick_map.insert(brick.to_string(), vec![alias.name().to_string()]);
            }
        }
    }
    brick_map
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
    
    #[test]
    fn test_alias_map() {
        let aliases = vec![
            Alias::new(
                String::from("hello"),
                vec![String::from("brick_a"), String::from("brick_b")],
            ),
            Alias::new(
                String::from("world"),
                vec![String::from("brick_c"), String::from("brick_b")],
            ),
        ];
        let mut brick_map: HashMap<String, Vec<String>> = HashMap::new();
        brick_map.insert(String::from("brick_a"), vec![String::from("hello")]);
        brick_map.insert(
            String::from("brick_b"),
            vec![String::from("hello"), String::from("world")],
        );
        brick_map.insert(String::from("brick_c"), vec![String::from("world")]);
        assert_eq!(map_aliases(&aliases), brick_map);
    }
}
