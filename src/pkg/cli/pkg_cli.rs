use crate::pkg::dnf::cli::dnf_cli::DnfCli;

use super::pkg_cli_command::PkgCliCommand;

pub struct PkgCli {}

impl PkgCli {
    pub fn handle_command(command: &PkgCliCommand) {
        match command {
            PkgCliCommand::Dnf { command } => {
                DnfCli::handle_command(command);
            },
        }
    }
}