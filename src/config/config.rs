use std::path::Path;
use std::fs;

use crate::config::params::config_params_init::ConfigParamsInit;
use crate::config::params::config_params_update::ConfigParamsUpdate;
use crate::config::pojo::local_config::LocalConfig;
use crate::config::pojo::base_config::{BaseConfig, self};
use crate::util::command_line::CommandLine;
use crate::util::confy_util::ConfyUtil;
use crate::util::exec::Exec;
use crate::util::path_util::PathUtil;

pub struct Config {}

impl Config {
    pub fn init(params: &ConfigParamsInit) {
        log::info!("Init commnand");

        let settings: LocalConfig = LocalConfig::get_config(None);

        // let git_config_dir = settings.git_config_dir.clone();
        let git_config_dir = ConfyUtil::get_git_configuration_dir();
            
        if Path::new(&git_config_dir).is_dir() {
            if params.force {
                let _ = fs::remove_dir_all(Path::new(&git_config_dir));
            } else {
                log::error!("Dir {:?} already exists", git_config_dir.clone());
                std::process::exit(1);
            }
        }

        let git_config_dir_str = PathUtil::to_string(&git_config_dir);
        let git_clone = CommandLine::create("git", ["clone", &params.url, &git_config_dir_str].to_vec());
        Exec::status(&git_clone, None);

        base_config::save_config(&settings);

        Self::update(&params.udpate_params);
    }

    pub fn update(params: &ConfigParamsUpdate) {
        log::info!("Update commnand");

        let mut settings = LocalConfig::get_config(None);

        if params.debug_level.is_some() {
            settings.debug_level = params.debug_level.unwrap();
        }
        
        if params.git_auto_sync.is_some() {
            settings.git_auto_sync = params.git_auto_sync.unwrap();
        }

        if params.system_config.is_some() {
            settings.system_config = params.system_config.clone().unwrap();
        }        
        
        if params.editor.is_some() {
            settings.editor = params.editor.clone().unwrap();
        }

        base_config::save_config(&settings);
    }

    pub fn commit_push(message: Option<String>) {    
        let commit_message = message.unwrap_or("Automatic update".to_string());
        let cmt_commit_message = format!("Cmt: {}", &commit_message);
    
        log::debug!("Commit message for update: {}", cmt_commit_message);
    
        let git_add = CommandLine::create("git", ["add", "."].to_vec());
        Exec::status(&git_add, Some(ConfyUtil::get_git_configuration_dir()));
    
        let git_commit = CommandLine::create("git", ["commit", "-m", &cmt_commit_message].to_vec());
        let result = Exec::status(&git_commit, Some(ConfyUtil::get_git_configuration_dir()));
        if result {
            let git_push = CommandLine::create("git", ["push"].to_vec());
            Exec::status(&git_push, Some(ConfyUtil::get_git_configuration_dir()));
        }
    }
    
    pub fn auto_commit_push(message: Option<String>) {
        let local_config = LocalConfig::get_config(None);
    
        if local_config.git_auto_sync {
            Self::commit_push(message);
        }
    }
    
    pub fn pull() {
        let git_add = CommandLine::create("git", ["pull"].to_vec());
        let result = Exec::status(&git_add, Some(ConfyUtil::get_git_configuration_dir()));
    
        if !result {
            log::error!("Could not pull {:?}", &ConfyUtil::get_git_configuration_dir());
            panic!();
        }
    }
    
    pub fn auto_pull() {
        let local_config = LocalConfig::get_config(None);
    
        if local_config.git_auto_sync {
            Self::pull();
        }
    }

    pub fn open_in_editor(editor: &str, open_git_config: &bool) {
            let path = match open_git_config {
                true => ConfyUtil::get_git_configuration_dir(),
                false => ConfyUtil::get_root_configuration_dir(),
            };
            let path_string = PathUtil::to_string(&path);
            Exec::status(&CommandLine{command: editor.to_string(), args: [path_string].to_vec()}, None);
    }
}
