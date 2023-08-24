use clap::Args;

#[derive(Args)]
pub struct SystemConfigParamAddRemove {
    /// Config to add/remove
    pub config: String,
}
