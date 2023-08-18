use clap::Subcommand;

use crate::pkg::{dnf::cli::dnf_cli_command::DnfCliCommand, pacman::cli::pacman_params::PacmanParams};

#[derive(Subcommand)]
pub enum PkgCliCommand {
    /// Interact with dnf package manager
    Dnf {
        #[command(subcommand)]
        command: DnfCliCommand,
    },    
    
    /// Interact with pacman package manager
    Pacman {
        #[command(flatten)]
        params: PacmanParams,
    }
}
