use clap::Args;

#[derive(Args)]
pub struct SystemConfigParamList {
    /// List all available configs
    #[arg(short, long)]
    pub all: bool,
}
