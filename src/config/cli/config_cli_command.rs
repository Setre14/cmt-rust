use clap::{Subcommand};

use crate::config::params::config_params_init::ConfigParamsInit;
use crate::config::params::config_params_update::ConfigParamsUpdate;

#[derive(Subcommand)]
pub enum ConfigCliCommand {
    /// Init local config and clone git config
    Init {
        #[command(flatten)]
        params: ConfigParamsInit,
    },

    /// Update local config
    Update {
        #[command(flatten)]
        params: ConfigParamsUpdate,
    }
}