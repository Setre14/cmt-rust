use crate::pkg::pacman::pacman::Pacman;

use super::pacman_params::PacmanParams;

pub struct PacmanCli {}

impl PacmanCli {
    pub fn handle_command(params: &PacmanParams) {

        if params.sync && params.remove {
            log::error!("Sync and remove can't be used together");
            std::process::exit(1);
        }

        if params.sync {
            Pacman::sync(&params.pkg_params, &params.shared_params, &params.sync_params);
        } else if params.remove {
            Pacman::remove(&params.pkg_params, &params.shared_params);
        }
    }
}