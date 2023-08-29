use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::config::pojo::base_config::BaseConfig;
use crate::util::confy_util::ConfyUtil;
use crate::util::exec::Exec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default = "LocalConfig::get_default_debug_level")]
    pub debug_level: u8,
    #[serde(default = "LocalConfig::get_default_git_auto_sync")]
    pub git_auto_sync: bool,
    #[serde(default = "LocalConfig::get_default_system_config")]
    pub system_config: String,
    #[serde(default = "LocalConfig::get_default_editor")]
    pub editor: String,
    #[serde(default = "LocalConfig::get_default_env_template_strict")]
    pub env_template_strict: bool,
}

impl Default for LocalConfig {
    fn default() -> Self { 
        LocalConfig {
            file_name: "local".to_string(),
            debug_level: LocalConfig::get_default_debug_level(),
            git_auto_sync: LocalConfig::get_default_git_auto_sync(),
            system_config: LocalConfig::get_default_system_config(),
            editor: LocalConfig::get_default_editor(),
            env_template_strict: LocalConfig::get_default_env_template_strict()
        }
    }
}

impl BaseConfig for LocalConfig {
    fn get_default_config_file_name() -> String {
        "local".to_string()
    }

    fn get_config_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn set_config_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    fn get_dir() -> PathBuf {
        ConfyUtil::get_root_configuration_dir()
    }
}

impl LocalConfig {
    pub fn get_default_debug_level() -> u8 {
        1
    }

    pub fn get_default_git_auto_sync() -> bool {
        true
    }

    pub fn get_default_system_config() -> String {
        format!("system-{}", Exec::get_hostname())
    }

    pub fn get_default_editor() -> String {
        "code".to_string()
    }

    pub fn get_default_env_template_strict() -> bool {
        true
    }
}