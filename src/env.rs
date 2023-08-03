use crate::config::env;
use crate::git_config;
use crate::config::config_track::ConfigTrack;
use crate::config::config_reader::ConfigReader;

use std::fs;
use std::path::{PathBuf};
use std;
use clap::{Subcommand};
use fs_extra::dir;
use fs_extra::copy_items;
use dirs;

#[derive(Subcommand)]
pub enum Command {
    /// Add file/folder to sync
    Add {
        /// Path to add to sync
        path: String,

        /// Add path globally
        #[arg(short, long)]
        global: bool,
    },

    /// Copies all env defined in the user config to the system
    Apply {
    },

    /// Remove file/folder from sync
    Remove {
        /// Path to remove from sync
        path: String,

        /// Remove path globally
        #[arg(short, long)]
        global: bool,
    },

    /// Updates all files defined in the user config from the system to the config dir
    Sync {
    }
}

pub struct Env {
    
}

impl Env {
    pub fn handle_command(command: &Command) {
        match command {
            Command::Add {path, global} => {
                Self::add(path, global);
            },
            Command::Apply {} => {
                Self::apply();
            },
            Command::Remove {path, global} => {
                Self::remove(path, global);
            },
            Command::Sync {} => {
                Self::sync();
            }
        }
    }

    pub fn add(path: &String, global: &bool) {
        let track = match global {
            true => ConfigTrack::GLOBAL,
            false => ConfigTrack::SYSTEM,
        };

        let mut conf = env::get_conf(&track);

        let abs_path = Self::get_abs_path(&path);
        log::info!("Abs path: {:#?}", abs_path.clone());
        let env_path = Self::get_env_path(&abs_path);
        log::info!("Env path: {}", env_path.clone());
        let abs_env_path = Self::get_abs_env_path(&env_path);
        log::info!("Abs env path: {:#?}", abs_env_path.clone());
        
        Self::copy(&abs_path, &PathBuf::from(abs_env_path));

        conf.add_path(&env_path);
        git_config::update(&Some(format!("Env: Add path '{}'", env_path)));
    }

    pub fn apply() {
        let conf = env::get_combined_conf();

        for path in conf.paths {
            let abs_env_path = Self::get_abs_env_path(&path);
            let system_path = Self::get_system_path(&path);

            Self::copy(&abs_env_path, &PathBuf::from(system_path))
        }
    }

    pub fn remove(path: &String, global: &bool) {
        let track = match global {
            true => ConfigTrack::GLOBAL,
            false => ConfigTrack::SYSTEM,
        };

        let mut conf = env::get_conf(&track);

        let abs_path = Self::get_abs_path(&path);
        log::info!("Abs path: {:#?}", abs_path.clone());
        let env_path = Self::get_env_path(&abs_path);
        log::info!("Env path: {}", env_path.clone());
        let abs_env_path = Self::get_abs_env_path(&env_path);
        log::info!("Abs env path: {:#?}", abs_env_path.clone());
        
        if abs_env_path.exists() {
            if abs_path.is_dir() {
                let result = fs::remove_dir_all(abs_env_path);
                log::info!("Result: {:#?}", result);
            } else {
                let result = fs::remove_file(abs_env_path);
                log::info!("Result: {:#?}", result);
            }
        }

        conf.remove_path(&env_path);
        git_config::update(&Some(format!("Env: Remove path '{}'", env_path)));
    }

    pub fn sync() {
        let conf = env::get_combined_conf();

        for path in conf.paths {
            let abs_env_path = Self::get_abs_env_path(&path);
            let system_path = Self::get_system_path(&path);

            Self::copy(&PathBuf::from(system_path), &abs_env_path)
        }
        git_config::update(&Some(format!("Env: sync")));
    }

    fn get_abs_path(path: &str) -> PathBuf {
        let mut abs_path = PathBuf::new();

        if !abs_path.starts_with("/") {
            abs_path = std::env::current_dir().unwrap()
        }

        abs_path.push(path);

        return abs_path;
    }

    fn get_env_path(path: &PathBuf) -> String {
        let conf = env::get_conf(&ConfigTrack::GLOBAL);
        let mut env_path = path.clone().into_os_string().into_string().unwrap();
        if env_path.starts_with("/") {
            env_path = (&env_path[1..]).to_string();
        }
        if env_path.ends_with("/") {
            env_path.pop();
        }

        let mut user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        user_home = (&user_home[1..]).to_string();
        log::info!("user home: {:#?}", user_home.clone());

        env_path = env_path.replace(&user_home, &conf.user_home);
        env_path = env_path.replace("~", &conf.user_home);

        return env_path;
    }

    fn get_abs_env_path(env_path: &String) -> PathBuf {
        let conf = env::get_conf(&ConfigTrack::GLOBAL);

        let mut abs_env_path = conf.get_conf_dir();
        abs_env_path.push(env_path);

        return abs_env_path;
    }

    fn get_system_path(path: &String) -> String {
        let conf = env::get_conf(&ConfigTrack::GLOBAL);
        let user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();

        return path.replace(&conf.user_home, &user_home);
    }

    fn copy(source: &PathBuf, destination: &PathBuf) {
        let mut dest_dir = destination.clone();
        dest_dir.pop();
        let _ = fs::create_dir_all(dest_dir.clone());
        if source.is_dir() {
            let mut options = dir::CopyOptions::new();
            options.overwrite = true;

            let mut from_paths = Vec::new();
            from_paths.push(source);

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