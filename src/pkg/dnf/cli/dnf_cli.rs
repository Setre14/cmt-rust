use crate::pkg::dnf::{dnf::Dnf, dnf_config_manager::DnfConfigManager};

use super::dnf_cli_command::DnfCliCommand;

pub struct DnfCli {}

impl DnfCli {
    pub fn handle_command(command: &DnfCliCommand, config: &Option<String>) {
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
            DnfCliCommand::ConfigManager { params } => {
                DnfConfigManager::handle_command(params, config);
            },
        }
    }
}
