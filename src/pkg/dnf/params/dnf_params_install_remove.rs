use clap::Args;

#[derive(Args)]
pub struct DnfParamsInstallRemove {
    /// Package to install
    pub package: String,

    /// Pkg config to use, default first in system config list
    #[arg(short = 'c', long)]
    pub pkg_config: Option<String>,

    /// automatically answer yes for all questions
    #[arg(short = 'y', long)]
    pub assumeyes: bool,
}
