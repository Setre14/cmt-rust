use std::{path::PathBuf, collections::BTreeSet};

use serde::{Serialize, Deserialize};

use crate::{config::pojo::base_config::BaseConfig, util::{path_util::PathUtil, confy_util::ConfyUtil}};

use super::local_config::LocalConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub template_values: String,
    #[serde(default = "SystemConfig::get_default_env_config")]
    pub env_config: BTreeSet<String>,
    #[serde(default)]
    pub package_config: BTreeSet<String>,
}

impl Default for SystemConfig {
    fn default() -> Self { 
        SystemConfig {
            file_name: "system".to_string(),
            template_values: "".to_string(),
            env_config: SystemConfig::get_default_env_config(),
            package_config: BTreeSet::new(),
        }
    }
}

impl BaseConfig for SystemConfig {
    fn get_default_config_file_name() -> String {
        let mut path = PathBuf::from(ConfyUtil::get_git_config_dir());
        path.push("system");

        log::debug!("System config file name: {:?}", path.clone());

        PathUtil::to_string(&path)
    }

    fn get_config_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn set_config_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    fn get_dir() -> PathBuf {
        ConfyUtil::get_git_configuration_dir()
    }
}

impl SystemConfig {
    pub fn get_system_config() -> SystemConfig {
        let settings = LocalConfig::get_config(None);
        SystemConfig::get_config(Some(settings.system_config))
    }

    pub fn get_default_env_config() -> BTreeSet<String> {
        let mut env_configs: BTreeSet<String> = BTreeSet::new();
        env_configs.insert("env".to_string());
        env_configs
    }
}