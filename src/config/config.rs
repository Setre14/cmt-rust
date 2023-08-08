use std::path::Path;
use std::fs;

use crate::config::params::config_params_init::ConfigParamsInit;
use crate::config::params::config_params_update::ConfigParamsUpdate;
use crate::config::pojo::local_config::LocalConfig;
use crate::config::pojo::base_config::BaseConfig;
use crate::util::command_line::CommandLine;
use crate::util::exec::Exec;

pub struct Config {}

impl Config {
    pub fn init(params: &ConfigParamsInit) {
        log::info!("Init commnand");

        let mut settings = LocalConfig::get_config();

        // let git_config_dir = settings.git_config_dir.clone();
        let git_config_dir = params.dest.clone().unwrap_or(settings.git_config_dir.clone());
    
        if Path::new(&git_config_dir).is_dir() {
            if params.force {
                let _ = fs::remove_dir_all(Path::new(&git_config_dir));
            } else {
        let git_config_dir = params.dest.clone().unwrap_or(settings.git_config_dir.clone());
                log::error!("Dir {} already exists", git_config_dir.clone());
                std::process::exit(1);
            }
        }

        let git_clone = CommandLine::create("git", ["clone", &params.url, &git_config_dir].to_vec());
        Exec::status(&git_clone);

        settings.git_clone_url = params.url.clone();
        settings.git_config_dir = git_config_dir.clone();

        settings.save_config();

        Self::update(&params.udpate_params);
    }

    pub fn update(params: &ConfigParamsUpdate) {
        log::info!("Update commnand");

        let mut settings = LocalConfig::get_config();

        if params.debug_level.is_some() {
            settings.debug_level = params.debug_level.unwrap();
        }
        
        if params.git_auto_sync.is_some() {
            settings.git_auto_sync = params.git_auto_sync.unwrap();
        }

        if params.system_config.is_some() {
            settings.system_config = params.system_config.clone().unwrap();
        }

        settings.save_config();
    }
}