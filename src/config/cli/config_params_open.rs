use clap::Args;

#[derive(Args)]
pub struct ConfigParamsOpen {
    /// Editor to use
    #[arg(short, long)]
    pub editor: Option<String>,

    /// Open git config
    #[arg(short, long)]
    pub git_config: bool,
}
