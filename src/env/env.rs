use crate::env::env_path::EnvPath;
use crate::env::params::env_params_add::EnvParamsAdd;
use crate::env::params::env_params_remove::EnvParamsRemove;

pub struct Env {}

impl Env {
    pub fn add(params: &EnvParamsAdd) {
        let env_path = EnvPath::from_local(&params.path);

        log::info!("env_path {:?}", env_path);
        log::info!("Add commnand");
        log::error!("Not implemented yet");
        std::process::exit(1);
    }

    pub fn remove(_params: &EnvParamsRemove) {
        log::info!("Remove commnand");
        log::error!("Not implemented yet");
        std::process::exit(1);
    }

    pub fn apply() {
        log::info!("apply commnand");
        log::error!("Not implemented yet");
        std::process::exit(1);
    }

    pub fn sync() {
        log::info!("sync commnand");
        log::error!("Not implemented yet");
        std::process::exit(1);
    }
}