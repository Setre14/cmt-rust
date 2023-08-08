use clap::{Subcommand};

use crate::env::params::env_params_add::EnvParamsAdd;
use crate::env::params::env_params_remove::EnvParamsRemove;

#[derive(Subcommand)]
pub enum EnvCliCommand {
    /// Add file/folder to sync
    Add {
        #[command(flatten)]
        params: EnvParamsAdd,
    },

    /// Copies all env defined in the user config to the system
    Apply {
    },

    /// Remove file/folder from sync
    Remove {
        #[command(flatten)]
        params: EnvParamsRemove,
    },

    /// Updates all files defined in the user config from the system to the config dir
    Sync {
    }
}