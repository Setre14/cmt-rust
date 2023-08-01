use serde::{Serialize, Deserialize};
use std::process::{Command, Stdio};
use std::path::PathBuf;
use dirs::config_dir;
use std::fs;

use crate::base;


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default)]
    pub git_config_dir: String,
    #[serde(default)]
    pub git_clone_url: String,
    #[serde(default)]
    pub git_branch: String,
    #[serde(default = "get_default_debug_level")]
    pub debug_level: u8,
}

impl Default for AppConfig {
    fn default() -> Self { 
        AppConfig {
            git_config_dir: get_default_git_config_dir(),
            git_clone_url: "".to_string(),
            git_branch: get_default_git_branch(),
            debug_level: get_default_debug_level(),
        }
    }
}

impl base::ConfigReader for AppConfig {
    fn get_conf_name(&self) -> String {
        return "app.json".to_string();
    }
    fn get_conf_dir(&self) -> PathBuf {
        let mut conf_dir = config_dir().expect("Could not get config dir");
        conf_dir.push("cmt-rust");
        let _ = fs::create_dir_all(conf_dir.clone());
    
        return conf_dir;
    }
}

pub fn get_conf() -> AppConfig {
    return base::get_conf(AppConfig { ..Default::default() });
}

fn get_default_git_config_dir() -> String {
    let mut conf_dir = config_dir().expect("Could not get config dir");
    conf_dir.push("cmt-rust");
    let _ = fs::create_dir_all(conf_dir.clone());

    let mut default_git_config_dir =  conf_dir;
    default_git_config_dir.push("git-config".to_string());

    return default_git_config_dir.into_os_string().into_string().unwrap();
}

fn get_default_git_branch() -> String {
    let output = Command::new("hostname")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap().replace("\n", "");

    return stdout;
}

fn get_default_debug_level() -> u8 {
    return 2;
}
