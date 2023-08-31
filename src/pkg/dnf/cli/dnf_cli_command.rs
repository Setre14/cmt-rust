use clap::Subcommand;

use crate::pkg::dnf::cli::dnf_params_install_remove::DnfParamsInstallRemove;

use super::dnf_params_config_manager::DnfParamsConfigManager;

#[derive(Subcommand)]
pub enum DnfCliCommand {
    /// Install package
    Install {
        #[command(flatten)]
        params: DnfParamsInstallRemove,
    },

    /// Remove package
    Remove {
        #[command(flatten)]
        params: DnfParamsInstallRemove,
    },

    /// Update all packages
    Update {},

    // Use config manager
    ConfigManager {
        #[command(flatten)]
        params: DnfParamsConfigManager,
    }
}
