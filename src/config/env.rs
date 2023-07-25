use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

use crate::base;
use crate::config::app;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EnvConfig {
    pub files: Vec<String>,
    pub folders: Vec<String>,
}


impl base::ConfigReader for EnvConfig {
    fn get_conf_name(&self) -> String {
        return "env.json".to_string();
    }

    fn get_conf_dir(&self) -> PathBuf {
        let app_conf = app::get_conf();

        let mut conf_dir = PathBuf::new();
        conf_dir.push(app_conf.git_config_dir);
        conf_dir.push("env");
        let _ = fs::create_dir_all(conf_dir.clone());
    
        return conf_dir;
    }
}

impl EnvConfig {
    pub fn add_file(&mut self, file: &String) {
        base::add_to_list(&mut self.files, file);
        base::save_conf(self);
    }

    pub fn remove_file(&mut self, file: &String) {
        base::remove_from_list(&mut self.files, file);
        base::save_conf(self);
    }

    pub fn add_folder(&mut self, folder: &String) {
        base::add_to_list(&mut self.folders, folder);
        base::save_conf(self);
    }

    pub fn remove_folder(&mut self, folder: &String) {
        base::remove_from_list(&mut self.folders, folder);
        base::save_conf(self);
    }
}

pub fn get_conf() -> EnvConfig {
    return base::get_conf(EnvConfig { ..Default::default() });
}
