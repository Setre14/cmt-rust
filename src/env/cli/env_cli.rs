use crate::env::cli::env_cli_command::EnvCliCommand;
use crate::env::env::Env;
use crate::system_config::cli::system_config_cli_command::SystemConfigCliCommand;

pub struct EnvCli {}

impl EnvCli {
    pub fn handle_command(command: &EnvCliCommand) {
        match command {
            EnvCliCommand::Add { params } => {
                Env::add(params);
            },
            EnvCliCommand::Remove { params } => {
                Env::remove(params);
            },
            EnvCliCommand::List { params } => {
                Env::list(params);
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
                        Env::config_list(params);
                    },
                    SystemConfigCliCommand::Add { params } => {
                        Env::config_add(params);
                    },
                    SystemConfigCliCommand::Remove { params } => {
                        Env::config_remove(params);
                    }
                }
            }
        }
    }
}