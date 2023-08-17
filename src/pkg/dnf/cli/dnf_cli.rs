use crate::pkg::dnf::dnf::Dnf;

use super::dnf_cli_command::DnfCliCommand;

pub struct DnfCli {}

impl DnfCli {
    pub fn handle_command(command: &DnfCliCommand) {
        match command {
            DnfCliCommand::Install { params } => {
                Dnf::install(params);
            },
            DnfCliCommand::Remove { params } => {
                Dnf::remove(params);
            },
            DnfCliCommand::Update {} => {
                Dnf::update();
            },
        }
    }
}