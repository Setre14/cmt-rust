use clap::{Subcommand};

use crate::env::params::env_params_add_remove::EnvParamsAddRemove;
use crate::env::params::env_params_config_add_remove::EnvParamsConfigAddRemove;
use crate::env::params::env_params_config_list::EnvParamsConfigList;

#[derive(Subcommand)]
pub enum EnvCliCommand {
    /// Add file/folder to sync
    Add {
        #[command(flatten)]
        params: EnvParamsAddRemove,
    },

    /// Copies all env defined in the user config to the system
    Apply {
    },

    /// Remove file/folder from sync
    Remove {
        #[command(flatten)]
        params: EnvParamsAddRemove,
    },

    /// Updates all files defined in the user config from the system to the config dir
    Sync {
    },


    /// List, add and remove env config files
    Config {
        #[command(subcommand)]
        config_command: EnvConfigCliCommand
    }

}

#[derive(Subcommand)]
pub enum EnvConfigCliCommand {

    List {
        #[command(flatten)]
        params: EnvParamsConfigList,
    },

    Add {
        #[command(flatten)]
        params: EnvParamsConfigAddRemove,
    },

    Remove {
        #[command(flatten)]
        params: EnvParamsConfigAddRemove,
    }
}