use std::path::PathBuf;
use std::fs;

use fs_extra::{dir, copy_items};
use serde::{Serialize, Deserialize};



use super::env_dir::EnvDir;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, PartialOrd, Eq, Ord)]
pub struct EnvPath {
    path: String
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

    pub fn get_remote_path(&self) -> PathBuf {
        let mut confg_dir = EnvDir::get_env_dir();
        confg_dir.push(&self.path);
        confg_dir
    }    
    
    pub fn get_local_path(&self) -> PathBuf {
        let user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        let local_path = self.path.replace("user-home", &user_home);
        PathBuf::from(&local_path)
    }

    pub fn copy_to_remote(&self) {
        // todo!()
        let remote_path = self.get_remote_path();
        log::info!("remote_path: {:#?}", remote_path.clone());
        let local_path = self.get_local_path();
        log::info!("local_path: {:#?}", local_path.clone());
        // let abs_env_path = Self::get_abs_env_path(&env_path);
        // log::info!("Abs env path: {:#?}", abs_env_path.clone());

        Self::copy(&local_path, &remote_path);
    }

    pub fn delte_from_remote(&self) {
        let remote_path = self.get_remote_path();
        log::info!("remote_path: {:#?}", remote_path.clone());

        if remote_path.exists() {
            if remote_path.is_dir() {
                let result = fs::remove_dir_all(remote_path);
                log::info!("Result: {:#?}", result);
            } else {
                let result = fs::remove_file(remote_path);
                log::info!("Result: {:#?}", result);
            }
        }
    }


    fn copy(source: &PathBuf, destination: &PathBuf) {
        let mut dest_dir = destination.clone();
        dest_dir.pop();
        let _ = fs::create_dir_all(dest_dir.clone());
        if source.is_dir() {
            let mut options = dir::CopyOptions::new();
            options.overwrite = true;

            let from_paths = vec![source];

            log::info!("Copy directoy from '{:#?}' to '{:#?}'", source, destination);

            let result = copy_items(&from_paths, &dest_dir, &options);
            log::debug!("Result: {:#?}", result);
        } else {
            log::info!("Copy file from '{:#?}' to '{:#?}'", source, destination);
            let result = fs::copy(source, destination);
            log::debug!("Result: {:#?}", result);
        }
    }
}
