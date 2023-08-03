use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use dirs::config_dir;
use std::fs;
use std::str::FromStr;

use crate::config::config_reader;
use crate::config::config_reader::ConfigReader;
use crate::config::config_track::ConfigTrack;
use crate::config::config_util::ConfigUtil;
use crate::config::string_accessable::StringAccessable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    #[serde(default = "ConfigUtil::get_hostname")]
    pub track: String,
    #[serde(default = "get_default_debug_level")]
    pub debug_level: u8,
    #[serde(default)]
    pub git_config_dir: String,
    #[serde(default)]
    pub git_clone_url: String,
    #[serde(default = "ConfigUtil::get_hostname")]
    pub git_branch: String,
}

impl Default for AppConfig {
    fn default() -> Self { 
        AppConfig {
            git_config_dir: get_default_git_config_dir(),
            git_clone_url: "".to_string(),
            git_branch: ConfigUtil::get_hostname(),
            debug_level: get_default_debug_level(),
            track: ConfigUtil::get_hostname()
        }
    }
}

impl StringAccessable for AppConfig {
    fn get_string(&self, field_string: &str) -> Result<&String, String> {
        match field_string {
            "git_config_dir" => Ok(&self.git_config_dir),
            "git_clone_url" => Ok(&self.git_clone_url),
            "track" => Ok(&self.track),
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }

    fn get_vec(&self, field_string: &str) -> Result<&Vec<String>, String> {
        match field_string {
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }

    fn get_u8(&self, field_string: &str) -> Result<&u8, String> {
        match field_string {
            "debug_level" => Ok(&self.debug_level),
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }
}

impl ConfigReader for AppConfig {
    fn get_conf_dir(&self) -> PathBuf {
        let mut conf_dir = config_dir().expect("Could not get config dir");
        conf_dir.push("cmt-rust");
        conf_dir.push("app");
        let _ = fs::create_dir_all(conf_dir.clone());
    
        return conf_dir;
    }

    fn get_track(&self) -> ConfigTrack {
        return ConfigTrack::from_str(&self.track).unwrap();
    }

    fn set_track(&mut self, track: &ConfigTrack) {
        self.track = track.to_string();
    }

    fn merge<T: StringAccessable + Clone + std::fmt::Debug>(&mut self, other: T) {
        self.git_config_dir = other.get_string("git_config_dir").unwrap().to_string();
        self.git_clone_url = other.get_string("git_clone_url").unwrap().to_string();
        self.debug_level = other.get_u8("debug_level").unwrap().clone();
        self.track = other.get_string("track").unwrap().to_string();
        
    }
}

pub fn get_conf() -> AppConfig {
    return config_reader::get_conf(&ConfigTrack::GLOBAL, &mut AppConfig { ..Default::default() });
}

fn get_default_git_config_dir() -> String {
    let mut conf_dir = config_dir().expect("Could not get config dir");
    conf_dir.push("cmt-rust");
    let _ = fs::create_dir_all(conf_dir.clone());

    let mut default_git_config_dir =  conf_dir;
    default_git_config_dir.push("git-config".to_string());

    return default_git_config_dir.into_os_string().into_string().unwrap();
}

fn get_default_debug_level() -> u8 {
    return 2;
}
