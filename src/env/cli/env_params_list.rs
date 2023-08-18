use clap::Args;

#[derive(Args)]
pub struct EnvParamsList {
    /// Set config
    #[arg(short, long)]
    pub config: Option<String>,
}
