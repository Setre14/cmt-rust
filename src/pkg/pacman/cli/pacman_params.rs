use clap::Args;

use crate::pkg::cli::pkg_params_install_remove::PkgParamsInstallRemove;

#[derive(Args)]
pub struct PacmanParams {
    #[command(flatten)]
    pub pkg_params: PkgParamsInstallRemove,

    #[arg(short = 'S', long)]
    pub sync: bool,

    #[arg(short = 'R', long)]
    pub remove: bool,

    #[command(flatten)]
    pub shared_params: PacmanParamsShared,

    #[command(flatten)]
    pub sync_params: PacmanParamsSync,
}

#[derive(Args)]
pub struct PacmanParamsShared {
    /// do not ask for any confirmation
    #[arg(long)]
    pub noconfirm: bool,
}

#[derive(Args)]
pub struct PacmanParamsSync {
    /// upgrade installed packages
    #[arg(short = 'u', long)]
    pub sysupgrade: bool,

    /// download fresh package databases from the server
    #[arg(short = 'y', long)]
    pub refresh: bool,
}
