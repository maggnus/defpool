mod api;
mod config;
mod state;
mod profitability;
mod tasks;

use axum::{
    routing::get,
    Router,
};
use clap::Parser;
use tracing::info;
use std::sync::Arc;

use config::Config;
use state::AppState;
use profitability::ProfitabilityCalculator;
use profitability::providers::CoinGeckoProvider;
use profitability::providers::PoolApiProvider;
use tasks::start_profitability_monitor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "defpool-server.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("Loading config from: {}", args.config);
    let config = Config::load(&args.config)?;
    info!("Config loaded: {:?}", config);

    let listen_address = config.listen_address;
    let state = AppState::new(config.clone());

    // Initialize profitability calculator with real providers
    info!("Initializing CoinGecko price provider");
    let price_provider = Arc::new(CoinGeckoProvider::new());
    
    info!("Initializing pool API difficulty provider");
    let difficulty_provider = Arc::new(PoolApiProvider::new());
    
    let calculator = Arc::new(ProfitabilityCalculator::new(
        price_provider,
        difficulty_provider,
        config.pools.clone(),
    ));

    // Start background profitability monitor
    start_profitability_monitor(state.clone(), calculator, config);

    // Build API routes
    let app = Router::new()
        .route("/target", get(api::get_target))
        .route("/pools", get(api::get_pools))
        .route("/current-pool", get(api::get_current_pool))
        .with_state(state);

    info!("DefPool Server listening on {}", listen_address);
    let listener = tokio::net::TcpListener::bind(listen_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
