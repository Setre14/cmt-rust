use crate::env::params::env_params_add::EnvParamsAdd;
use crate::env::params::env_params_remove::EnvParamsRemove;

pub struct Env {}

impl Env {
    pub fn add(_params: &EnvParamsAdd) {
        log::info!("Add commnand");
    }

    pub fn remove(_params: &EnvParamsRemove) {
        log::info!("Remove commnand");
    }

    pub fn apply() {
        log::info!("apply commnand");
    }

    pub fn sync() {
        log::info!("sync commnand");
    }
}