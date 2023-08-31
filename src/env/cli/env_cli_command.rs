use clap::Subcommand;

use crate::env::cli::env_params_add_remove::EnvParamsAddRemove;
use crate::system_config::cli::system_config_cli_command::SystemConfigCliCommand;

#[derive(Subcommand)]
pub enum EnvCliCommand {
    /// Add file/folder to sync
    Add {
        #[command(flatten)]
        params: EnvParamsAddRemove,
    },

    /// List paths set in env configs
    List { },

    /// Copies all env defined in the user config to the system
    Apply {
    },

    /// Remove file/folder from sync
    Remove {
        #[command(flatten)]
        params: EnvParamsAddRemove,
    },

    /// Updates all files defined in the user config from the system to the config dir
    Sync { },


    /// List, add and remove env config files
    Config {
        #[command(subcommand)]
        config_command: SystemConfigCliCommand
    }

}
