use serde::{Serialize, Deserialize};
use std::process::{Command, Stdio};

use crate::base;


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub git_config_dir: String,
    pub git_clone_url: String,
    pub git_branch: String,
}

impl Default for AppConfig {
    fn default() -> Self { 
        AppConfig {
            git_config_dir: get_default_git_config_dir(),
            git_clone_url: "".to_string(),
            git_branch: get_default_git_branch(),
        }
    }
}

impl base::ConfigReader for AppConfig {
    fn get_conf_name(&self) -> String {
        return "app.json".to_string();
    }
}

pub fn get_conf() -> AppConfig {
    return base::get_conf(AppConfig { ..Default::default() });
}

fn get_default_git_config_dir() -> String {
    let mut default_git_config_dir = base::get_conf_dir();
    default_git_config_dir.push("git-config".to_string());

    return default_git_config_dir.into_os_string().into_string().unwrap();
}

fn get_default_git_branch() -> String {
    let output = Command::new("hostname")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap().replace("\n", "");

    println!("{}", stdout);

    return stdout;
}