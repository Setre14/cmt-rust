use serde::{Serialize, Deserialize};

use crate::config::pojo::base_config::BaseConfig;
use crate::util::exec::Exec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalConfig {
    #[serde(default = "LocalConfig::get_default_debug_level")]
    pub debug_level: u8,
    #[serde(default = "LocalConfig::get_default_git_auto_sync")]
    pub git_auto_sync: bool,
    #[serde(default = "LocalConfig::get_default_git_config_dir")]
    pub git_config_dir: String,
    #[serde(default)]
    pub git_clone_url: String,
    #[serde(default = "LocalConfig::get_default_system_config")]
    pub system_config: String,
}

impl Default for LocalConfig {
    fn default() -> Self { 
        LocalConfig {
            debug_level: LocalConfig::get_default_debug_level(),
            git_auto_sync: LocalConfig::get_default_git_auto_sync(),
            git_config_dir: LocalConfig::get_default_git_config_dir(),
            git_clone_url: "".to_string(),
            system_config: "".to_string()
        }
    }
}

impl BaseConfig for LocalConfig {
    fn get_config_file_name() -> String {
        return "config.json".to_string();
    }

    fn get_default() -> LocalConfig {
        return LocalConfig { ..Default::default()}
    }
}

impl LocalConfig {
    pub fn get_default_debug_level() -> u8 {
        return 2;
    }

    pub fn get_default_git_auto_sync() -> bool {
        return true;
    }

    pub fn get_default_git_config_dir() -> String {
        let mut conf_dir = Self::get_config_dir();
    
        conf_dir.push("git-config".to_string());
    
        return conf_dir.into_os_string().into_string().unwrap();
    }

    pub fn get_default_system_config() -> String {
        return Exec::get_hostname();
    }
}