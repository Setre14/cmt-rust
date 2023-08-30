use crate::config::config::Config;

use super::config_cli_command::ConfigCliCommand;

pub struct ConfigCli {}

impl ConfigCli {
    pub fn handle_command(command: &ConfigCliCommand) {
        match command {
            ConfigCliCommand::Init { params } => {
                Config::init(params);
            },
            ConfigCliCommand::Update { params } => {
                Config::update(params);
            },
            ConfigCliCommand::Open { params } => {
                Config::open_in_editor(params)
            },
        }
    }
}