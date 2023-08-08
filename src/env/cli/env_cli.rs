use crate::env::cli::env_cli_command::EnvCliCommand;
use crate::env::env::{Env};

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
            EnvCliCommand::Apply {} => {
                Env::apply();
            },
            EnvCliCommand::Sync {} => {
                Env::sync();
            }
        }
    }
}