use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;

use handlebars::Handlebars;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

use crate::config::pojo::env_config::EnvConfig;
use crate::config::pojo::base_config::BaseConfig;
use crate::config::pojo::system_config::SystemConfig;
use crate::util::path_util::PathUtil;

use super::env_path::EnvPath;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, PartialOrd, Eq, Ord)]
pub struct EnvCopy {}

impl EnvCopy {
    pub fn copy_to_remote(env_path: &EnvPath) {
        env_path.delte_from_remote();

        let mut files = Vec::new();
        let local_path = env_path.get_local_path();
        if local_path.is_file() {
            files.push(env_path.clone());
        } else {
            for file in WalkDir::new(&local_path).into_iter().filter_map(|file| file.ok()) {
                let path = file.path();
                if path.is_file() {
                    let env_path = EnvPath::from_local(&PathUtil::to_string(path));
                    files.push(env_path);
                }
            }
        }
   
        for env_path in files {
            let source = env_path.get_local_path();
            let destination = env_path.get_remote_path();
            log::info!("Copy file from '{:#?}' to '{:#?}'", &source, &destination);

            let mut dest_dir = destination.clone();
            dest_dir.pop();
            let _ = fs::create_dir_all(dest_dir.clone());

            let _ = fs::copy(&source, &destination);


            let file = File::open(source).unwrap();
            let lines = BufReader::new(file).lines();
            let mut templated_lines = Vec::new();


            let mut first = true;
            let comment_header = "cmt-comment";
            let mut comment = comment_header.to_string();

            let mut next_line = "".to_string();
            let mut use_template = false;
            for res_line in lines {
                let mut line = res_line.unwrap();

                if first && line.contains(&comment) {
                    comment = line.rsplit_once('=').unwrap().1.to_string();
                }



                if line.starts_with(&format!("{} template=", comment)) {
                    next_line = line.split_once('=').unwrap().1.to_string();
                    use_template = true;
                } else {
                    if use_template {
                        line = next_line.clone();
                        use_template = false;
                    }
                    templated_lines.push(line);
                }

                first = false;
            }

            templated_lines.push("".to_string());


            fs::write(destination, templated_lines.join("\n")).expect("");
        }
    }

    pub fn copy_to_local(env_path: &EnvPath) {
        env_path.delte_from_local();

        let system_config = SystemConfig::get_system_config();
        let mut values_file = EnvConfig::get_dir();
        values_file.push(&system_config.env_config.template_values);
        let file = File::open(values_file).unwrap();
        let reader = BufReader::new(file);
    
        let data: serde_json::Value = serde_json::from_reader(reader).unwrap();
        log::debug!("data: {:?}", data);

        let mut files = Vec::new();
        let remote_path = env_path.get_remote_path();
        if remote_path.is_file() {
            files.push(env_path.clone());
        } else {
            for file in WalkDir::new(&remote_path).into_iter().filter_map(|file| file.ok()) {
                let path = file.path();
                if path.is_file() {
                    let env_path = EnvPath::from_remote(&PathUtil::to_string(path));
                    files.push(env_path);
                }
            }
        }

        for env_path in files {
            let source = env_path.get_remote_path();
            let destination = env_path.get_local_path();
            log::info!("Copy file from '{:#?}' to '{:#?}'", &source, &destination);

            let mut dest_dir = destination.clone();
            dest_dir.pop();
            let _ = fs::create_dir_all(dest_dir.clone());
            let _ = fs::copy(&source, &destination);


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
                    comment = line.rsplit_once('=').unwrap().1.to_string();
                }

                let templated_line = handlebars.render_template(&line, &data).unwrap();

                if !line.eq(&templated_line) && !comment.eq(comment_header) {
                    templated_lines.push(format!("{} template={}", comment, &line));
                }

                templated_lines.push(templated_line);
                first = false;
            }

            templated_lines.push("".to_string());

            fs::write(destination, templated_lines.join("\n")).expect("");
        }
    }
}
