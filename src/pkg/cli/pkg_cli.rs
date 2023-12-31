use crate::pkg::{dnf::cli::dnf_cli::DnfCli, pacman::cli::pacman_cli::PacmanCli, yay::cli::yay_cli::YayCli};

use super::pkg_cli_command::PkgCliCommand;

pub struct PkgCli {}

impl PkgCli {
    pub fn handle_command(command: &PkgCliCommand, config: &Option<String>) {
        match command {
            PkgCliCommand::Dnf { command } => {
                DnfCli::handle_command(command, config);
            },
            PkgCliCommand::Pacman { params } => {
                PacmanCli::handle_command(params);
            },
            PkgCliCommand::Yay { params } => {
                YayCli::handle_command(params);
            },
        }
    }
}