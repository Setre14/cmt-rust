use clap::Args;

use crate::pkg::cli::pkg_params_install_remove::PkgParamsInstallRemove;

#[derive(Args)]
pub struct DnfParamsInstallRemove {
    #[command(flatten)]
    pub pkg_params: PkgParamsInstallRemove,

    /// automatically answer yes for all questions
    #[arg(short = 'y', long)]
    pub assumeyes: bool,
}
