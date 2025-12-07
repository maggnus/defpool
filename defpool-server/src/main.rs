mod api;
mod config;
mod state;
mod profitability;
mod tasks;
mod db;
mod accounting;

use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use config::Config;
use state::AppState;
use profitability::{ProfitabilityCalculator, providers::{CoinGeckoProvider, PoolApiProvider}};
use tasks::profitability_monitor::start_profitability_monitor;
use db::{create_pool, repository::ShareRepository};
use accounting::AccountingService;
use std::sync::Arc;
use tracing::info;

#[derive(Parser)]
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

    // Initialize database
    info!("Connecting to database: {}", config.database_url);
    let db_pool = create_pool(&config.database_url).await?;
    info!("Database connected successfully");

    // Initialize accounting service
    let repository = Arc::new(ShareRepository::new(db_pool));
    let accounting_service = Arc::new(AccountingService::new(repository));
    info!("Accounting service initialized");

    // Initialize state with accounting service
    let state = AppState::new(config.clone(), accounting_service);

    // Initialize profitability providers
    info!("Initializing CoinGecko price provider");
    let price_provider = Arc::new(CoinGeckoProvider::new());
    
    info!("Initializing pool API difficulty provider");
    let difficulty_provider = Arc::new(PoolApiProvider::new());

    let calculator = Arc::new(ProfitabilityCalculator::new(
        price_provider,
        difficulty_provider,
        config.targets.clone(),
    ));

    // Start background profitability monitor
    start_profitability_monitor(state.clone(), calculator, config);

    // Build API routes with versioning
    let app = Router::new()
        // V1 API routes
        .route("/api/v1/target", get(api::get_current_target))
        .route("/api/v1/targets", get(api::list_targets))
        .route("/api/v1/targets/current", get(api::get_current_target_name))
        // Miner endpoints
        .route("/api/v1/miners/:wallet/stats", get(api::get_miner_stats))
        .route("/api/v1/miners/:wallet/workers", get(api::get_miner_workers))
        // Share recording (internal)
        .route("/api/v1/shares", post(api::record_share))
        // Legacy routes (deprecated, for backward compatibility)
        .route("/target", get(api::get_current_target))
        .with_state(state);

    info!("DefPool Server listening on {}", listen_address);
    let listener = tokio::net::TcpListener::bind(listen_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
