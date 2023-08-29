use std::{path::PathBuf, collections::BTreeSet};

use serde::{Serialize, Deserialize};

use crate::{config::{pojo::base_config::BaseConfig, config::Config}, util::{path_util::PathUtil, confy_util::ConfyUtil}, system_config::cli::{system_config_params_list::SystemConfigParamList, system_config_params_add_remove::SystemConfigParamAddRemove}};

use super::{local_config::LocalConfig, system_env_config::SystemEnvConfig, system_package_config::SystemPackageConfig, env_config::EnvConfig, base_config};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SystemConfig {
    #[serde(skip_serializing, default)]
    pub file_name: String,
    #[serde(default)]
    pub package_config: SystemPackageConfig,
    #[serde(default)]
    pub env_config: SystemEnvConfig,
}

impl BaseConfig for SystemConfig {
    fn get_default_config_file_name() -> String {
        let mut path = PathBuf::from(ConfyUtil::get_git_config_dir());
        path.push("system");

        log::debug!("System config file name: {:?}", path.clone());

        PathUtil::to_string(&path)
    }

    fn get_config_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn set_config_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }

    fn get_dir() -> PathBuf {
        ConfyUtil::get_git_configuration_dir()
    }
}

impl SystemConfig {
    pub fn get_system_config() -> SystemConfig {
        let settings = LocalConfig::get_config(None);
        SystemConfig::get_config(Some(settings.system_config))
    }

    pub fn list(params: &SystemConfigParamList) {
        Config::auto_pull();
        let configs = match params.all {
            true => EnvConfig::get_configs(),
            false => {
                let system_config = SystemConfig::get_system_config();
                let mut configs = system_config.env_config.link_config.configs.clone();
                configs.insert(system_config.env_config.link_config.main_config.clone());
                configs            
            }
            
        };

        for env_config in configs  {
            println!("{}", env_config)
        }
    }

    pub fn add(params: &SystemConfigParamAddRemove) {
        Config::auto_pull();
        if !EnvConfig::config_exists(&params.config) {
            println!("Config {} not found", &params.config);

            println!("List of available configs:");
            SystemConfig::list(&SystemConfigParamList { all: true });
            std::process::exit(1);
        }

        let mut system_config = SystemConfig::get_system_config();
        let mut configs = system_config.env_config.link_config.configs.clone();
        configs.insert(params.config.clone());
        system_config.env_config.link_config.configs = configs;
        base_config::save_config(&system_config);
        Config::auto_commit_push(Some(format!("Add env config: '{}'", &params.config)));
    }

    pub fn remove(params: &SystemConfigParamAddRemove) {
        Config::auto_pull();
        let mut system_config = SystemConfig::get_system_config();
        system_config.env_config.link_config.configs.remove(&params.config);
        base_config::save_config(&system_config);
        Config::auto_commit_push(Some(format!("Remove env config: '{}'", &params.config)));
    }
}