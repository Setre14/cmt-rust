use clap::Subcommand;

use crate::pkg::dnf::cli::dnf_params_install_remove::DnfParamsInstallRemove;

#[derive(Subcommand)]
pub enum PacmanCliCommand {
    /// Sync package
    Sync {
        #[command(flatten)]
        params: DnfParamsInstallRemove,
    },

    /// Remove package
    Remove {
        #[command(flatten)]
        params: DnfParamsInstallRemove,
    },
}
