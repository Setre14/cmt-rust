use clap::{Args};

#[derive(Args)]
pub struct EnvParamsConfigAddRemove {
    /// Config to add/remove
    pub config: String,
}
