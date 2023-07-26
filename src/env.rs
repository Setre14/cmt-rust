use crate::config::env;
use std::fs;
use std::path::{PathBuf};
use config::base::ConfigReader;
use std;
use clap::{Subcommand};
use fs_extra::dir;
use fs_extra::copy_items;
use dirs;

#[derive(Subcommand)]
pub enum Command {
    Add {
        path: String,
    },
    Apply {
    },
    Remove {
        path: String,
    },
    Sync {
    }
}

pub struct Env {
    
}

impl Env {
    pub fn handle_command(command: &Command) {
        match command {
            Command::Add {path} => {
                Self::add(path);
            },
            Command::Apply {} => {
                Self::apply();
            },
            Command::Remove {path} => {
                Self::remove(path);
            },
            Command::Sync {} => {
                Self::sync();
            }
        }
    }

    pub fn add(path: &String) {
        let mut conf = env::get_conf();

        let abs_path = Self::get_abs_path(&path);
        println!("Abs path: {:#?}", abs_path.clone());
        let env_path = Self::get_env_path(&abs_path);
        println!("Env path: {}", env_path.clone());
        let abs_env_path = Self::get_abs_env_path(&env_path);
        println!("Abs env path: {:#?}", abs_env_path.clone());
        
        let mut abs_env_dir = abs_env_path.clone();
        abs_env_dir.pop();

        println!("Abs env dir: {:#?}", abs_env_dir.clone());


        let _ = fs::create_dir_all(abs_env_dir.clone());

        if abs_path.is_dir() {
            let mut options = dir::CopyOptions::new();
            options.overwrite = true;

            let mut from_paths = Vec::new();
            from_paths.push(abs_path);
            let result = copy_items(&from_paths, &abs_env_dir, &options);
            println!("Result: {:#?}", result);
        } else {
            let result = fs::copy(abs_path, abs_env_path);
            println!("Result: {:#?}", result);
        }
        
        conf.add_path(&env_path);
    }

    pub fn apply() {
    }

    pub fn remove(path: &String) {
        let mut conf = env::get_conf();

        let abs_path = Self::get_abs_path(&path);
        println!("Abs path: {:#?}", abs_path.clone());
        let env_path = Self::get_env_path(&abs_path);
        println!("Env path: {}", env_path.clone());
        let abs_env_path = Self::get_abs_env_path(&env_path);
        println!("Abs env path: {:#?}", abs_env_path.clone());
        
        if abs_env_path.exists() {
            if abs_path.is_dir() {
                let result = fs::remove_dir_all(abs_env_path);
                println!("Result: {:#?}", result);
            } else {
                let result = fs::remove_file(abs_env_path);
                println!("Result: {:#?}", result);
            }
        }

        conf.remove_path(&env_path);
    }

    pub fn sync() {
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
        let conf = env::get_conf();
        let mut env_path = path.clone().into_os_string().into_string().unwrap();
        if env_path.starts_with("/") {
            env_path = (&env_path[1..]).to_string();
        }
        if env_path.ends_with("/") {
            env_path.pop();
        }

        let mut user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        user_home = (&user_home[1..]).to_string();
        println!("user home: {:#?}", user_home.clone());

        env_path = env_path.replace(&user_home, &conf.user_home);
        env_path = env_path.replace("~", &conf.user_home);

        return env_path;
    }

    fn get_abs_env_path(env_path: &String) -> PathBuf {
        let conf = env::get_conf();

        let mut abs_env_path = conf.get_conf_dir();
        abs_env_path.push(env_path);

        return abs_env_path;
    }
}