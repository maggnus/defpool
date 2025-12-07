# Changelog

All notable changes to DefPool will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-12-07

### Added
- **Share Accounting System**
  - PostgreSQL integration with SQLx
  - Database models: Miner, Worker, Share, MinerStats
  - Repository pattern for clean data access
  - Accounting service for share tracking
  - Automatic miner and worker creation
  - Hashrate calculation (10-minute rolling window)
  
- **New API Endpoints**
  - `POST /api/v1/shares` - Record share submissions (internal)
  - `GET /api/v1/miners/{wallet}/stats` - Get miner statistics
  - `GET /api/v1/miners/{wallet}/workers` - Get miner's workers
  
- **Database Infrastructure**
  - PostgreSQL deployment via Docker Compose
  - Database initialization script with schema
  - Automatic triggers for stats updates
  - Backup and restore scripts
  
- **Deployment Tools**
  - `defpool-deploy/` directory for all deployment files
  - `make db` - Start database
  - `make backup` - Backup database
  - `make restore` - Restore database

### Changed
- AppState now includes AccountingService
- Configuration supports database_url with env override

### Technical Details
- **Database**: PostgreSQL 16 with connection pooling
- **ORM**: SQLx with compile-time query checking
- **Architecture**: Repository pattern with dependency injection
- **Performance**: Indexed queries, automatic stats updates


## [0.3.0] - 2025-12-07

### Changed
- **BREAKING**: Refactored "pool" terminology to "target" for abstraction
  - `Pool` struct renamed to `MiningTarget`
  - Added `TargetType` enum (`pool` or `daemon`)
  - Configuration now uses `[[targets]]` instead of `[[pools]]`
  - API endpoints restructured with versioning
  
- **API Restructuring**
  - New versioned endpoints: `/api/v1/target`, `/api/v1/targets`, `/api/v1/targets/current`
  - Legacy endpoint `/target` maintained for backward compatibility
  - All endpoints now follow RESTful best practices
  
### Added
- `target_type` field to distinguish between external pools and self-hosted daemons
- `daemon_rpc_url` field for future daemon integration
- API standards workflow documentation
- Comprehensive API documentation with examples

### Technical Details
- **Architecture**: More abstract and flexible for future daemon support
- **Configuration**: `type = "pool"` or `type = "daemon"`
- **API Versioning**: `/api/v1/` prefix for all new endpoints


## [0.2.0] - 2025-12-07

### Added
- **Real API Integration**
  - CoinGecko price provider for real-time cryptocurrency prices
  - MoneroBlocks difficulty provider for network difficulty data
  - Automatic profitability calculation every 60 seconds
  
- **Multi-Pool Support**
  - Configuration for multiple mining pools
  - Dynamic pool selection based on profitability
  - Hysteresis threshold (5%) to prevent pool thrashing
  
- **Profitability System**
  - Trait-based provider architecture (PriceProvider, DifficultyProvider)
  - Profitability calculator with dependency injection
  - Background monitoring task
  - Real-time profitability scoring
  
- **API Endpoints**
  - `GET /target` - Get current pool target
  - `GET /pools` - List all pools with profitability scores
  - `GET /current-pool` - Get current active pool name
  
- **Code Quality**
  - Build verification workflow
  - Zero compiler warnings
  - Coding standards documentation
  - Clean code practices enforced

### Changed
- Server configuration now supports multiple pools via TOML array
- State management refactored to track current pool and profitability scores
- Mock providers replaced with real API implementations

### Technical Details
- **Architecture**: Modular design with trait-based abstractions
- **Dependencies**: Added `reqwest`, `serde_json`, `async-trait`, `thiserror`
- **Configuration**: `profitability_check_interval_secs`, `switch_threshold_percent`

## [0.1.0] - 2025-12-07

### Added
- **Initial Release**
  - DefPool proxy with Stratum V1 passthrough
  - DefPool server with target management
  - End-to-end mining support (xmrig → proxy → pool)
  
- **Protocol Support**
  - Stratum V1 downstream (from miners)
  - Stratum V1 upstream (to pools)
  - Automatic protocol detection
  
- **Infrastructure**
  - Docker Compose setup
  - TOML configuration files
  - Environment variable overrides
  - Local development scripts
  
- **Proxy Features**
  - V1 protocol detection (peeks at first byte)
  - Bidirectional V1→V1 passthrough
  - Connection to defpool-server for target fetching
  
- **Server Features**
  - REST API for target retrieval
  - Configurable initial target
  - Mock profitability calculation

### Technical Details
- **Languages**: Rust
- **Frameworks**: Axum (server), Tokio (async runtime)
- **Protocols**: Stratum V1
- **Tested With**: xmrig mining to SupportXMR

---

## Release Notes Format

Each release should include:
- **Version number** (semantic versioning)
- **Date** (YYYY-MM-DD)
- **Categories**: Added, Changed, Deprecated, Removed, Fixed, Security
- **Technical details** for developers
- **User-facing changes** for miners/operators

## Links
- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
