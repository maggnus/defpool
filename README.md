# DefPool - Profit-Switching Mining Pool

DefPool is a next-generation cryptocurrency mining pool that automatically switches between the most profitable coins to maximize miner revenue. Built in Rust for performance and reliability.

## Features

### ✨ Core Features
- **Automatic Profit Switching** - Continuously monitors profitability and switches to the most profitable coin
- **Multi-Algorithm Support** - RandomX (Monero), Scrypt (Litecoin, Dogecoin)
- **Stratum V1 & V2** - Support for both protocol versions
- **Share Accounting** - Accurate tracking with PPLNS (Pay Per Last N Shares)
- **Automated Payouts** - Configurable thresholds and auto-exchange to BTC
- **Real-time Statistics** - Worker monitoring, hashrate tracking, and earnings

### 🔧 Technical Features
- **High Performance** - Built in Rust with async I/O
- **PostgreSQL Backend** - Reliable data storage with automatic backups
- **RESTful API** - Complete API for integration and monitoring
- **Docker Support** - Easy deployment with Docker Compose
- **Daemon Support** - Solo mining directly to coin daemons

## Quick Start

### Prerequisites
- Docker & Docker Compose
- Rust 1.70+ (for building from source)
- PostgreSQL 16+ (or use Docker)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/defpool.git
cd defpool

# Run quick start script
./scripts/quick-start.sh
```

This will:
1. Start PostgreSQL database
2. Run database migrations
3. Build and start DefPool server
4. Build and start DefPool proxy

### Connect Your Miner

```bash
# For xmrig (Monero)
xmrig -o localhost:3333 \
  -u YOUR_WALLET_ADDRESS \
  -p x

# For other miners
# Server: localhost:3333
# User: YOUR_WALLET_ADDRESS
# Password: x (or worker name)
```

### Check Status

```bash
# View server logs
tail -f defpool-server.log

# View proxy logs
tail -f defpool-proxy.log

# Check API
curl http://localhost:3000/api/v1/target
```

## Architecture

DefPool consists of four main components:

### 1. DefPool Server
The core API server that handles:
- Profitability calculations
- Target selection
- Share accounting
- Payout processing
- Statistics aggregation

**Port**: 3000 (HTTP API)

### 2. DefPool Proxy
Stratum proxy that:
- Accepts miner connections
- Forwards to upstream pools
- Records shares
- Translates protocols (V1 ↔ V2)

**Port**: 3333 (Stratum)

### 3. DefPool Portal
Web dashboard for:
- Real-time statistics
- Worker monitoring
- Balance tracking
- Payout management

**Port**: 8080 (HTTP)

### 4. PostgreSQL Database
Stores:
- Miner accounts
- Share history
- Balances
- Payout records

**Port**: 5432

## Configuration

### Server Configuration
Edit `defpool-server/defpool-server.toml`:

```toml
listen_address = "0.0.0.0:3000"
database_url = "postgres://defpool:password@localhost:5432/defpool"
profitability_check_interval_secs = 30
switch_threshold_percent = 5.0

[[targets]]
name = "supportxmr"
type = "pool"
address = "pool-sg.supportxmr.com:3333"
coin = "XMR"
algorithm = "RandomX"
```

### Proxy Configuration
Edit `defpool-proxy/defpool-proxy.toml`:

```toml
server_endpoint = "http://localhost:3000"
listen_address = "0.0.0.0:3333"
default_wallet = "YOUR_WALLET_ADDRESS"
```

## API Documentation

### Get Current Target
```bash
GET /api/v1/target
```

Response:
```json
{
  "address": "pool-sg.supportxmr.com:3333",
  "protocol": "sv1"
}
```

### List All Targets
```bash
GET /api/v1/targets
```

Response:
```json
[
  {
    "target_name": "supportxmr",
    "coin": "XMR",
    "score": 0.000123
  }
]
```

### Get Miner Statistics
```bash
GET /api/v1/miners/{wallet}/stats
```

Response:
```json
{
  "wallet_address": "44AFFq...",
  "total_shares": 150,
  "valid_shares": 148,
  "hashrate": 1234.56,
  "workers_count": 1
}
```

### Get Miner Balances
```bash
GET /api/v1/miners/{wallet}/balances
```

Response:
```json
[
  {
    "coin": "XMR",
    "balance": 0.05,
    "pending_balance": 0.0,
    "total_paid": 0.0
  }
]
```

### Request Payout
```bash
POST /api/v1/miners/{wallet}/payout
Content-Type: application/json

{
  "coin": "XMR",
  "amount": 0.01
}
```

See [API Documentation](doc/api.md) for complete API reference.

## Development

### Building from Source

```bash
# Build server
cd defpool-server
cargo build --release

# Build proxy
cd defpool-proxy
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test --all

# Run specific component tests
cd defpool-server
cargo test
```

### Development Mode

```bash
# Terminal 1: Server with auto-reload
cd defpool-server
cargo watch -x run

# Terminal 2: Proxy with auto-reload
cd defpool-proxy
cargo watch -x run
```

## Deployment

### Docker Compose (Recommended)

```bash
cd defpool-deploy
docker-compose up -d
```

This starts all components:
- PostgreSQL database
- DefPool server
- DefPool proxy
- DefPool portal

### Manual Deployment

See [Deployment Guide](doc/deployment.md) for detailed instructions.

## Monitoring

### Logs

```bash
# Server logs
tail -f defpool-server.log

# Proxy logs
tail -f defpool-proxy.log

# Database logs
docker-compose logs -f postgres
```

### Metrics

```bash
# Check profitability updates
curl http://localhost:3000/api/v1/targets

# Check miner count
psql $DATABASE_URL -c "SELECT COUNT(*) FROM miners;"

# Check share rate
psql $DATABASE_URL -c "SELECT COUNT(*) FROM shares WHERE created_at > NOW() - INTERVAL '1 hour';"
```

## Troubleshooting

### Miner Can't Connect
- Check proxy is running: `ps aux | grep defpool-proxy`
- Verify port 3333 is open: `netstat -an | grep 3333`
- Check firewall rules

### Shares Not Recording
- Verify server API is accessible: `curl http://localhost:3000/api/v1/target`
- Check proxy logs for errors: `tail -f defpool-proxy.log`
- Verify database connection

### Profitability Not Updating
- Check API rate limits (CoinGecko)
- Verify internet connectivity
- Check server logs: `tail -f defpool-server.log | grep profitability`

See [Troubleshooting Guide](doc/troubleshooting.md) for more help.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --all`
5. Submit a pull request

## Roadmap

### v0.5.0 (Current)
- [x] Payout system foundation
- [x] Daemon support infrastructure
- [x] Protocol translation framework
- [x] Share recording
- [ ] Complete SV2 ↔ SV1 translation
- [ ] Blockchain transaction integration

### v0.6.0 (Next)
- [ ] Exchange API integration
- [ ] Auto-exchange to BTC
- [ ] Web dashboard improvements
- [ ] WebSocket real-time updates

### v1.0.0 (Future)
- [ ] Production-ready security
- [ ] Load balancing
- [ ] Multi-region support
- [ ] Advanced analytics

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [docs/](doc/)
- **Issues**: [GitHub Issues](https://github.com/yourusername/defpool/issues)
- **Discord**: [Join our community](https://discord.gg/defpool)

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [Stratum V2 Reference Implementation](https://github.com/stratum-mining/stratum)
- Inspired by profit-switching pools like NiceHash and MiningPoolHub

---

**⚠️ Disclaimer**: This software is provided as-is. Always test thoroughly before using in production. Mining cryptocurrency involves financial risk.
