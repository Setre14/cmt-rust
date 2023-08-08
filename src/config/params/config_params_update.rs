use clap::{Args};

#[derive(Args)]
pub struct ConfigParamsUpdate {
    /// Debug level 
    #[arg(short = 'l', long)]
    pub debug_level: Option<u8>,

    /// Automatically commit and push changes
    #[arg(short = 's', long)]
    pub git_auto_sync: Option<bool>,

    /// System config to use
    #[arg(short = 'c', long)]
    pub system_config: Option<String>,
} 