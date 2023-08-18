use clap::Args;

#[derive(Args)]
pub struct PkgParamsInstallRemove {
    /// Package to install
    pub package: String,

    /// Pkg config to use, default first in system config list
    #[arg(short = 'c', long)]
    pub pkg_config: Option<String>,
}
