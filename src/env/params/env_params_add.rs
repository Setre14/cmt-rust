use clap::{Args};

#[derive(Args)]
pub struct EnvParamsAdd {
    /// Path to add to sync
    pub path: String,
}