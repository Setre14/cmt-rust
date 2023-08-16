use std::{path::PathBuf, collections::BTreeSet, fs};

use serde::{Serialize, Deserialize};

use crate::{config::pojo::base_config::BaseConfig, util::{path_util::PathUtil, confy_util::ConfyUtil}, env::{env_path::EnvPath}};



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EnvConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub paths: BTreeSet<EnvPath>,
}

impl BaseConfig for EnvConfig {
    fn get_default_config_file_name() -> String {
        "env/env".to_string()

    }

    fn get_config_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn set_config_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    fn get_dir() -> PathBuf {
        let mut path = ConfyUtil::get_git_configuration_dir();
        path.push("env");

        path.clone()
    }
}

impl EnvConfig {
    pub fn get_env_config(config: &str) -> EnvConfig {
        EnvConfig::get_config(Some(format!("env/{}", config)))
    }

    pub fn get_configs() -> BTreeSet<String> {
        let mut configs: BTreeSet<String> = BTreeSet::new();

        let paths = fs::read_dir(Self::get_dir()).unwrap();

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