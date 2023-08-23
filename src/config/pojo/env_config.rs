use std::{path::PathBuf, collections::BTreeSet};

use serde::{Serialize, Deserialize};

use crate::{config::pojo::base_config::BaseConfig, util::confy_util::ConfyUtil, env::env_path::EnvPath};



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EnvConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub paths: BTreeSet<String>,
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

    pub fn config_exists(config: &str) -> bool {
        let configs = Self::get_configs();

        configs.contains(&config.to_string())
    }

    pub fn add_path(&mut self, env_path: &EnvPath) {
        self.paths.insert(env_path.path.clone());
    }

    pub fn remove_path(&mut self, env_path: &EnvPath) {
        self.paths.remove(&env_path.path);
    }

    pub fn contains_path(&self, env_path: &EnvPath) -> bool {
        self.paths.contains(&env_path.path)
    }

    pub fn get_paths(&self) -> Vec<EnvPath> {
        self.paths.iter().map(|p| EnvPath{path: p.to_string()}).collect()
    }
}