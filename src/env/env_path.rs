use std::path::PathBuf;
use std::fs;

use crate::config::pojo::env_config::EnvConfig;
use crate::config::pojo::base_config::BaseConfig;
use crate::util::path_util::PathUtil;

#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct EnvPath {
    pub path: String
}

impl EnvPath {
    pub fn from_local(path: &str) -> EnvPath {
        let mut env_path = fs::canonicalize(path).unwrap().into_os_string().into_string().unwrap();

        let user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();

        env_path = env_path.replace(&user_home, "user-home");

        if env_path.starts_with('/') {
            env_path = env_path[1..].to_string();
        }

        EnvPath {
            path: env_path.to_string()
        }
    }

    pub fn from_remote(path: &str) -> EnvPath {
        let remote_root = format!("{}/", &PathUtil::to_string(EnvConfig::get_dir().as_path()));
        let env_path = path.replace(&remote_root, "");

        EnvPath { path: env_path }
    }

    pub fn get_remote_path(&self) -> PathBuf {
        let mut confg_dir = EnvConfig::get_dir();
        confg_dir.push(&self.path);
        confg_dir
    }    
    
    pub fn get_local_path(&self) -> PathBuf {
        let user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        let local_path = self.path.replace("user-home", &user_home);
        PathBuf::from(&local_path)
    }

    pub fn delte_from_remote(&self) {
        Self::delte(&self.get_remote_path());
    }

    pub fn delte_from_local(&self) {
        Self::delte(&self.get_local_path());
    }

    fn delte(path: &PathBuf) {
        if path.exists() {
            if path.is_dir() {
                let _ = fs::remove_dir_all(path);
            } else {
                let _ = fs::remove_file(path);
            }
        }
    }
}
