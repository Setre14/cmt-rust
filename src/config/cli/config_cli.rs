use crate::config::cli::config_cli_command::ConfigCliCommand;
use crate::config::config::{Config};

pub struct ConfigCli {}

impl ConfigCli {
    pub fn handle_command(command: &ConfigCliCommand) {
        match command {
            ConfigCliCommand::Init { params } => {
                Config::init(params);
            },
            ConfigCliCommand::Update { params } => {
                Config::update(params);
            }
        }
    }
}