use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

use crate::base;
use crate::config::app;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvConfig {
    #[serde(default = "get_user_home")]
    pub user_home: String,
    #[serde(default)]
    pub paths: Vec<String>,
}

impl Default for EnvConfig {
    fn default() -> Self { 
        EnvConfig {
            user_home: get_user_home(),
            paths: Vec::new(),
        }
    }
}

fn get_user_home() -> String {
    return "user-home".to_string();
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
    pub fn add_path(&mut self, path: &String) {
        for p in self.paths.clone() {
            if p.starts_with(path) {
                base::remove_from_list(&mut self.paths, &p);
            } else if path.starts_with(&p) {
                return;
            }
        }
        base::add_to_list(&mut self.paths, path);
        base::save_conf(self);
    }

    pub fn remove_path(&mut self, path: &String) {
        base::remove_from_list(&mut self.paths, path);
        base::save_conf(self);
    }
}

pub fn get_conf() -> EnvConfig {
    return base::get_conf(EnvConfig { ..Default::default() });
}
