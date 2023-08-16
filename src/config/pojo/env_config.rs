use std::{path::PathBuf, collections::BTreeSet};

use serde::{Serialize, Deserialize};

use crate::{config::pojo::base_config::BaseConfig, util::{path_util::PathUtil, confy_util::ConfyUtil}, env::{env_path::EnvPath}};



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EnvConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub paths: BTreeSet<EnvPath>,
}

impl BaseConfig for EnvConfig {
    fn get_default_config_file_name() -> String {
        let mut path = PathBuf::from(ConfyUtil::get_git_config_dir());
        path.push("env");

        log::debug!("Env config file name: {:?}", path.clone());

        PathUtil::to_string(&path)
    }

    fn get_config_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn set_config_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }
}

impl EnvConfig {
    pub fn get_env_config(config: &str) -> EnvConfig {
        EnvConfig::get_config(Some(format!("env/{}", config)))
    }

}