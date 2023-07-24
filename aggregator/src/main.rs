use std::env;

use clap::Parser;

use crate::capture::start_capture;
use crate::config::Config;

pub mod capture;
pub mod config;

/// The aggregator to collect from the exfiltrator
#[derive(Parser)]
pub struct Cli {
    /// The path to the config file
    #[clap(long)]
    #[clap(default_value_t = String::from("/etc/aggregator/config.toml"))]
    config_path: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO");
    }

    env_logger::init();

    let cli = Cli::parse();

    let config = Config::try_from(cli.config_path.as_str())?;

    start_capture(&config).await?;

    Ok(())
}
