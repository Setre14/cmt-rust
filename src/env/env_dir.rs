use std::{fs, path::PathBuf, collections::BTreeSet};

use crate::util::confy_util::ConfyUtil;

pub struct EnvDir {}

impl EnvDir {
    pub fn get_env_dir() -> PathBuf {
        let mut path = ConfyUtil::get_git_configuration_dir();
        path.push("env");

        path.clone()
    }

    pub fn get_configs() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();

        let paths = fs::read_dir(Self::get_env_dir()).unwrap();

        for p in paths {
            let  path = p.unwrap().path();
            if !path.is_file() {
                continue;
            }

            let file = path.file_name().unwrap().to_str().unwrap();

            if !file.ends_with(&ConfyUtil::get_config_file_ending()) {
                continue;
            }

            let file_name = file.replace(&ConfyUtil::get_config_file_ending(), "");

            configs.insert(file_name);
        }

        configs
    }

    pub fn config_exists(config: &str) -> bool {
        let configs = Self::get_configs();

        configs.contains(&config.to_string())
    }
}