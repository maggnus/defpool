mod config;
mod proxy;

use anyhow::Result;
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "defpool-proxy.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("Starting DefPool Proxy...");
    info!("Loading config from: {}", args.config);

    let config = config::Config::load(&args.config)?;
    info!("Config loaded: {:?}", config);

    proxy::start(config).await?;

    Ok(())
}
