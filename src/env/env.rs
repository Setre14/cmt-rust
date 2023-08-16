use std::collections::BTreeSet;

use crate::config::pojo::base_config;
use crate::config::pojo::env_config::EnvConfig;
use crate::config::pojo::system_config::SystemConfig;
use crate::env::env_path::EnvPath;
use crate::env::params::env_params_add_remove::EnvParamsAddRemove;

use super::params::env_params_config_add_remove::EnvParamsConfigAddRemove;
use super::params::env_params_config_list::EnvParamsConfigList;

pub struct Env {}

impl Env {
    pub fn add(params: &EnvParamsAddRemove) {
        let env_path = EnvPath::from_local(&params.path);

        let mut env_config = Self::get_env_config(&params.env_config);

        env_path.copy_to_remote();

        env_config.paths.insert(env_path.clone());
    
        base_config::save_config(&env_config);
    }

    pub fn remove(params: &EnvParamsAddRemove) {
        let env_path = EnvPath::from_local(&params.path);

        let mut env_config = Self::get_env_config(&params.env_config);
        env_config.paths.remove(&env_path);
        base_config::save_config(&env_config);

        let env_configs = EnvConfig::get_configs();

        let mut remove = true;
        for env_conf in env_configs {
            let config = EnvConfig::get_env_config(&env_conf);

            if config.paths.contains(&env_path) {
                log::info!("'{}' is still references in env config '{}' - will not be delted from remote", &params.path, &env_conf);
                remove = false;
                break;
            }
        }

        if remove {
            env_path.delte_from_remote();
        }
    }

    fn get_env_config(config: &Option<String>) -> EnvConfig {
        let config_name = match config {
            Some(conf) => conf.clone(),
            None => {
                let system_config = SystemConfig::get_system_config();

                if system_config.env_config.len() > 1 {
                    log::error!("More then one env config is set, specify where it should be added with --env-config");
                    std::process::exit(1);
                }

                system_config.env_config.first().unwrap().clone()
            },
        };

        EnvConfig::get_env_config(&config_name)
    }

    pub fn apply() {
        let system_config = SystemConfig::get_system_config();

        let mut env_paths = BTreeSet::new();
        for env_config in system_config.env_config {
            let config = EnvConfig::get_env_config(&env_config);
            env_paths.extend(config.paths);
        }

        for env_path in env_paths {
            env_path.copy_to_local();
        }
    }

    pub fn sync() {
        let system_config = SystemConfig::get_system_config();

        let mut env_paths = BTreeSet::new();
        for env_config in system_config.env_config {
            let config = EnvConfig::get_env_config(&env_config);
            env_paths.extend(config.paths);
        }

        for env_path in env_paths {
            env_path.copy_to_remote();
        }
    }

    pub fn config_list(params: &EnvParamsConfigList) {
        let system_config = SystemConfig::get_system_config();

        let configs = match params.all {
            true => EnvConfig::get_configs(),
            false => system_config.env_config
            
        };

        for env_config in configs  {
            println!("{}", env_config)
        }
    }

    pub fn config_add(params: &EnvParamsConfigAddRemove) {
        if !EnvConfig::config_exists(&params.config) {
            println!("Config {} not found", &params.config);

            println!("List of available configs:");
            Self::config_list(&EnvParamsConfigList { all: true });
            std::process::exit(1);
        }

        let mut system_config = SystemConfig::get_system_config();
        system_config.env_config.insert(params.config.clone());
        base_config::save_config(&system_config);
    }

    pub fn config_remove(params: &EnvParamsConfigAddRemove) {
        let mut system_config = SystemConfig::get_system_config();
        system_config.env_config.remove(&params.config);
        base_config::save_config(&system_config);
    }
}