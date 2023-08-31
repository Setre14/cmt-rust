use crate::config::pojo::system_config::SystemConfig;
use crate::env::cli::env_cli_command::EnvCliCommand;
use crate::env::env::Env;
use crate::system_config::cli::system_config_cli_command::SystemConfigCliCommand;

pub struct EnvCli {}

impl EnvCli {
    pub fn handle_command(command: &EnvCliCommand, config: &Option<String>) {
        match command {
            EnvCliCommand::Add { params } => {
                Env::add(params, config);
            },
            EnvCliCommand::Remove { params } => {
                Env::remove(params, config);
            },
            EnvCliCommand::List {} => {
                Env::list(config);
            },
            EnvCliCommand::Apply {} => {
                Env::apply();
            },
            EnvCliCommand::Sync {} => {
                Env::sync();
            },
            EnvCliCommand::Config { config_command } => {
                match config_command {
                    SystemConfigCliCommand::List { params } => {
                        SystemConfig::list(params);
                    },
                    SystemConfigCliCommand::Add { params } => {
                        SystemConfig::add(params);
                    },
                    SystemConfigCliCommand::Remove { params } => {
                        SystemConfig::remove(params);
                    }
                }
            }
        }
    }
}