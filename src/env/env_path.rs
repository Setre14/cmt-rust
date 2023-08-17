use std::io::{BufReader, BufRead};
use std::{path::PathBuf, fs::File};
use std::fs;

use fs_extra::{dir, copy_items};
use handlebars::Handlebars;
use serde::{Serialize, Deserialize};

use crate::config::pojo::env_config::EnvConfig;
use crate::config::pojo::base_config::BaseConfig;
use crate::config::pojo::system_config::SystemConfig;

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
        let mut confg_dir = EnvConfig::get_dir();
        confg_dir.push(&self.path);
        confg_dir
    }    
    
    pub fn get_local_path(&self) -> PathBuf {
        let user_home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        let local_path = self.path.replace("user-home", &user_home);
        PathBuf::from(&local_path)
    }

    pub fn copy_to_remote(&self) {
        let destination = self.get_remote_path();
        log::info!("remote_path: {:#?}", destination.clone());
        let source = self.get_local_path();
        log::info!("local_path: {:#?}", source.clone());
   
        Self::copy(&source, &destination);
    }

    pub fn copy_to_local(&self) {
        let source = self.get_remote_path();
        log::info!("remote_path: {:#?}", source.clone());
        let destination = self.get_local_path();
        log::info!("local_path: {:#?}", destination.clone());

        let system_config = SystemConfig::get_system_config();
        let mut values_file = EnvConfig::get_dir();
        values_file.push(&system_config.env_config.template_values);
        let file = File::open(values_file).unwrap();
        let reader = BufReader::new(file);
    
        let data: serde_json::Value = serde_json::from_reader(reader).unwrap();
        log::debug!("data: {:?}", data);


        let mut dest_dir = destination.clone();
        dest_dir.pop();
        let _ = fs::create_dir_all(dest_dir.clone());

        if source.is_dir() {
            let mut options = dir::CopyOptions::new();
            options.overwrite = true;

            let from_paths = vec![&source];

            log::info!("Copy directoy from '{:#?}' to '{:#?}'", &source, destination);

            let result = copy_items(&from_paths, &dest_dir, &options);
            log::debug!("Result: {:#?}", result);
        } else {
            log::info!("Copy file from '{:#?}' to '{:#?}'", source, destination);

            let handlebars = Handlebars::new();

            let file = File::open(source).unwrap();
            let lines = BufReader::new(file).lines();
            let mut templated_lines = Vec::new();


            let mut first = true;
            let comment_header = "cmt-comment";
            let mut comment = comment_header.to_string();
            for res_line in lines {
                let line = res_line.unwrap();

                if first && line.contains(&comment) {
                    comment = line.rsplit_once("=").unwrap().1.to_string();
                }

                let templated_line = handlebars.render_template(&line, &data).unwrap();

                if !line.eq(&templated_line) && !comment.eq(comment_header) {
                    templated_lines.push(format!("{} template={}", comment, &line));
                }

                templated_lines.push(templated_line);
                first = false;
            }


            fs::write(destination, templated_lines.join("\n")).expect("");
        }

        // Self::copy(&local_path, &remote_path);
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
            log::debug!("Copy Result: {:#?}", result);
        } else {
            log::info!("Copy file from '{:#?}' to '{:#?}'", source, destination);
            let result = fs::copy(source, destination);
            log::debug!("Copy Result: {:#?}", result);
        }
    }
}
