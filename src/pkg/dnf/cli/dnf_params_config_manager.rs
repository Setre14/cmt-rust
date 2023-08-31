use clap::Args;

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct DnfParamsConfigManager {
    /// Add repo
    #[arg(short, long)]
    pub add_repo: Option<String>,

    /// Remove repo
    #[arg(short, long)]
    pub remove_repo: Option<String>,
}
