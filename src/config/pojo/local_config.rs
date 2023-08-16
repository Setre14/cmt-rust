use serde::{Serialize, Deserialize};

use crate::config::pojo::base_config::BaseConfig;
use crate::util::exec::Exec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default = "LocalConfig::get_default_debug_level")]
    pub debug_level: u8,
    #[serde(default = "LocalConfig::get_default_git_auto_sync")]
    pub git_auto_sync: bool,
    #[serde(default)]
    pub git_clone_url: String,
    #[serde(default = "LocalConfig::get_default_system_config")]
    pub system_config: String,
    #[serde(default = "LocalConfig::get_default_editor")]
    pub editor: String,
}

impl Default for LocalConfig {
    fn default() -> Self { 
        LocalConfig {
            file_name: "local".to_string(),
            debug_level: LocalConfig::get_default_debug_level(),
            git_auto_sync: LocalConfig::get_default_git_auto_sync(),
            git_clone_url: "".to_string(),
            system_config: "system".to_string(),
            editor: "code".to_string()
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
}

impl LocalConfig {
    pub fn get_default_debug_level() -> u8 {
        2
    }

    pub fn get_default_git_auto_sync() -> bool {
        true
    }

    pub fn get_default_system_config() -> String {
        Exec::get_hostname()
    }

    pub fn get_default_editor() -> String {
        "code".to_string()
    }
}