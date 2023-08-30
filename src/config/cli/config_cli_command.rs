use clap::Subcommand;

use super::config_params_init::ConfigParamsInit;
use super::config_params_open::ConfigParamsOpen;
use super::config_params_update::ConfigParamsUpdate;

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
    },

    /// Open config in editor
    Open {
        #[command(flatten)]
        params: ConfigParamsOpen,
    },

}
