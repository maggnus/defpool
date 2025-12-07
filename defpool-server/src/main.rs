mod api;
mod state;
mod config;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use crate::state::AppState;
use crate::config::Config;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "defpool-server.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("Loading config from: {}", args.config);
    let config = Config::load(&args.config)?;
    info!("Config loaded: {:?}", config);

    let listen_address = config.listen_address;
    let state = AppState::new(config);
    let app = Router::new()
        .route("/target", get(api::get_target))
        .with_state(state.clone());

    // Spawn background profitability task (mocked)
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            info!("Calculating profitability... (Mock: Keeping current target)");
            // Logic to update state.current_target would go here
        }
    });

    info!("DefPool Server listening on {}", listen_address);
    let listener = TcpListener::bind(listen_address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
