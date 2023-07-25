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
    // pub fn add_package(&mut self, package: &String) {
    //     if !self.packages.contains(package) {
    //         self.packages.push(package.clone().to_string());
    //     }
    //     self.packages.sort();

    //     base::save_conf(self);
    // }

    // pub fn remove_package(&mut self, package: &String) {
    //     if self.packages.contains(package) {
    //         let index = self.packages.iter().position(|x| *x == package.to_string()).unwrap();
    //         self.packages.remove(index);
    //     }
    //     self.packages.sort();
    
    //     base::save_conf(self);
    // }
}

pub fn get_conf() -> EnvConfig {
    return base::get_conf(EnvConfig { ..Default::default() });
}
