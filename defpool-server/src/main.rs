mod api;
mod config;
mod state;
mod profitability;
mod tasks;
mod db;
mod accounting;
mod payout;
mod daemon;

use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use config::Config;
use state::AppState;
use profitability::{ProfitabilityCalculator, providers::{CoinGeckoProvider, PoolApiProvider}};
use tasks::profitability_monitor::start_profitability_monitor;
use tasks::payout_processor::start_payout_processor;
use tasks::balance_updater::start_balance_updater;
use db::{create_pool, repository::ShareRepository};
use accounting::AccountingService;
use payout::BalanceCalculator;
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
    let repository = Arc::new(ShareRepository::new(db_pool.clone()));
    let accounting_service = Arc::new(AccountingService::new(repository));
    info!("Accounting service initialized");

    // Initialize payout service
    let payout_service = Arc::new(payout::PayoutService::new(db_pool.clone()));
    info!("Payout service initialized");

    // Initialize state with services
    let state = AppState::new(config.clone(), accounting_service, payout_service.clone());

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
    start_profitability_monitor(state.clone(), calculator, config.clone());

    // Start background payout processor
    start_payout_processor(payout_service.clone());

    // Start background balance updater
    let balance_calculator = Arc::new(BalanceCalculator::new(db_pool.clone()));
    let coins: Vec<String> = config.targets.iter()
        .map(|t| t.coin.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    start_balance_updater(balance_calculator, coins);

    // Build API routes with versioning
    let app = Router::new()
        // V1 API routes
        .route("/api/v1/target", get(api::get_current_target))
        .route("/api/v1/targets", get(api::list_targets))
        .route("/api/v1/targets/current", get(api::get_current_target_name))
        .route("/api/v1/stats", get(api::get_pool_stats))
        // Miner endpoints
        .route("/api/v1/miners/:wallet/stats", get(api::get_miner_stats))
        .route("/api/v1/miners/:wallet/workers", get(api::get_miner_workers))
        .route("/api/v1/miners/:wallet/balances", get(api::get_miner_balances))
        .route("/api/v1/miners/:wallet/balance/:coin", get(api::get_miner_balance))
        .route("/api/v1/miners/:wallet/payout", post(api::request_payout))
        .route("/api/v1/miners/:wallet/payouts", get(api::get_payout_history))
        .route("/api/v1/miners/:wallet/payout-settings", axum::routing::put(api::update_payout_settings))
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
