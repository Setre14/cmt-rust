use clap::Subcommand;

use crate::pkg::{dnf::cli::dnf_cli_command::DnfCliCommand, pacman::cli::pacman_params::PacmanParams, yay::cli::yay_params::YayParams};

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
    },
    
    /// Interact with yay package manager
    Yay {
        #[command(flatten)]
        params: YayParams,
    }
}
