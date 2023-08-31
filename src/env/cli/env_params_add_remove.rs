use clap::Args;

#[derive(Args)]
pub struct EnvParamsAddRemove {
    /// Path to add to sync
    pub path: String,
}
