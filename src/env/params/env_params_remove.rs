use clap::{Args};

#[derive(Args)]
pub struct EnvParamsRemove {
    /// Path to add to sync
    path: String,
}
