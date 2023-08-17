use clap::Subcommand;

use crate::pkg::dnf::params::dnf_params_install_remove::DnfParamsInstallRemove;

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

}
