use crate::pkg::yay::yay::Yay;

use super::yay_params::YayParams;

pub struct YayCli {}

impl YayCli {
    pub fn handle_command(params: &YayParams) {

        if params.sync && params.remove {
            log::error!("Sync and remove can't be used together");
            std::process::exit(1);
        }

        if params.sync {
            Yay::sync(&params.pkg_params, &params.shared_params, &params.sync_params);
        } else if params.remove {
            Yay::remove(&params.pkg_params, &params.shared_params);
        }
    }
}