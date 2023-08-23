use clap::Args;

#[derive(Args)]
pub struct EnvParamsConfigList {
    /// List all available configs
    #[arg(short, long)]
    pub all: bool,
}
