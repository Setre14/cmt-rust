use clap::Subcommand;

use crate::pkg::dnf::cli::dnf_cli_command::DnfCliCommand;

#[derive(Subcommand)]
pub enum PkgCliCommand {
    /// Interact with dnf package manager
    Dnf {
        #[command(subcommand)]
        command: DnfCliCommand,
    }
}
