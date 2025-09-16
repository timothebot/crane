use std::{env, path::PathBuf};

use serde::{Deserialize, Serialize};


const ENV_KEY_CONFIG_DIR: &'static str = "CRANE_CONFIG_DIR";

fn config_path_from_env() -> Option<PathBuf> {
    PathBuf::try_from(env::var(ENV_KEY_CONFIG_DIR).ok()?).ok()
}

pub fn config_dir() -> PathBuf {
    match config_path_from_env() {
        Some(path) => path,
        None => PathBuf::from("~/.config/crane")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CraneConfig {

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
        assert_eq!(format!("{}", config_dir().display()), String::from("~/.crane"))
    }
}
