use clap::Args;

#[derive(Args)]
pub struct EnvParamsAddRemove {
    /// Path to add to sync
    pub path: String,

    /// Env config to use, default first in system config list
    #[arg(short = 'c', long)]
    pub env_config: Option<String>,
}
