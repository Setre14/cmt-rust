use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::str::FromStr;

use crate::config::app_config;
use crate::config::config_track::ConfigTrack;
use crate::config::config_reader::ConfigReader;
use crate::config::config_util::ConfigUtil;
use crate::config::string_accessable::StringAccessable;
use crate::config::config_reader;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvConfig {
    #[serde(default)]
    pub track: String,
    #[serde(default = "get_user_home")]
    pub user_home: String,
    #[serde(default)]
    pub paths: Vec<String>,
}

impl Default for EnvConfig {
    fn default() -> Self { 
        EnvConfig {
            track: "".to_string(),
            user_home: get_user_home(),
            paths: Vec::new(),
        }
    }
}

fn get_user_home() -> String {
    return "user-home".to_string();
}

impl StringAccessable for EnvConfig {
    fn get_string(&self, field_string: &str) -> Result<&String, String> {
        match field_string {
            "user_home" => Ok(&self.user_home),
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }

    fn get_vec(&self, field_string: &str) -> Result<&Vec<String>, String> {
        match field_string {
            "paths" => Ok(&self.paths),
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }

    fn get_u8(&self, field_string: &str) -> Result<&u8, String> {
        match field_string {
            _ => Err(format!("invalid field name to get '{}'", field_string))
        }
    }
}

impl ConfigReader for EnvConfig {
    fn get_conf_dir(&self) -> PathBuf {
        let app_conf = app_config::get_conf();

        let mut conf_dir = PathBuf::new();
        conf_dir.push(app_conf.git_config_dir);
        conf_dir.push("env");
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
        self.user_home = other.get_string("user_home").unwrap().to_string();

        let paths = other.get_vec("paths").unwrap();
        ConfigUtil::merge_vec(&mut self.paths, paths);
    }
}

impl EnvConfig {
    pub fn add_path(&mut self, path: &String) {
        for p in self.paths.clone() {
            if p.starts_with(path) {
                ConfigUtil::remove_from_list(&mut self.paths, &p);
            } else if path.starts_with(&p) {
                return;
            }
        }
        ConfigUtil::add_to_list(&mut self.paths, path);
        config_reader::save_conf(self);
    }

    pub fn remove_path(&mut self, path: &String) -> bool {
        let removed = ConfigUtil::remove_from_list(&mut self.paths, path);
        config_reader::save_conf(self);
        return removed;
    }
}

pub fn get_conf(track: &ConfigTrack) -> EnvConfig {
    return config_reader::get_conf(track, &mut EnvConfig { ..Default::default() });
}

pub fn get_combined_conf() -> EnvConfig {
    return config_reader::get_combined_conf(&mut EnvConfig { ..Default::default() });
}

pub fn cleanup() {
    let global_conf = get_conf(&ConfigTrack::GLOBAL);
    let mut system_conf = get_conf(&ConfigTrack::SYSTEM);

    for path in global_conf.paths {
        system_conf.remove_path(&path);
    }
}
