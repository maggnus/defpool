# DefPool Testing Guide

## Overview
This guide covers testing DefPool components in development and production environments.

## Prerequisites

### Required Software
- Rust 1.70+ (for building)
- PostgreSQL 16+ (for database)
- Docker & Docker Compose (for containerized testing)
- xmrig or similar miner (for end-to-end testing)

### Optional
- Monero daemon (for solo mining tests)
- Litecoin daemon (for Scrypt testing)

## Quick Start

### 1. Start Database
```bash
cd defpool-deploy
docker-compose up -d postgres
```

### 2. Run Database Migrations
```bash
cd defpool-server
# Set database URL
export DATABASE_URL="postgres://defpool:defpool_dev_password@localhost:5432/defpool"

# Run migrations
psql $DATABASE_URL < src/db/migrations/001_initial_schema.sql
psql $DATABASE_URL < src/db/migrations/002_payout_system.sql
```

### 3. Start DefPool Server
```bash
cd defpool-server
cargo run --release
```

Expected output:
```
INFO defpool_server: Loading config from: defpool-server.toml
INFO defpool_server: Connecting to database...
INFO defpool_server: Database connected successfully
INFO defpool_server: Accounting service initialized
INFO defpool_server: Payout service initialized
INFO defpool_server: Starting profitability monitor
INFO defpool_server: DefPool Server listening on 0.0.0.0:3000
```

### 4. Start DefPool Proxy
```bash
cd defpool-proxy
cargo run --release
```

Expected output:
```
INFO defpool_proxy: Starting DefPool Proxy...
INFO defpool_proxy: Loading config from: defpool-proxy.toml
INFO defpool_proxy: Verifying connection to server...
INFO defpool_proxy: Successfully connected to server
INFO defpool_proxy: Proxy listening for miners on: 0.0.0.0:3333
```

## Testing Components

### API Endpoints

#### Get Current Target
```bash
curl http://localhost:3000/api/v1/target
```

Expected response:
```json
{
  "address": "pool-sg.supportxmr.com:3333",
  "pubkey": null,
  "protocol": "sv1"
}
```

#### List All Targets with Profitability
```bash
curl http://localhost:3000/api/v1/targets
```

Expected response:
```json
[
  {
    "target_name": "supportxmr",
    "coin": "XMR",
    "score": 0.000123,
    "timestamp": "2025-12-08T..."
  },
  {
    "target_name": "moneroocean",
    "coin": "XMR",
    "score": 0.000118,
    "timestamp": "2025-12-08T..."
  }
]
```

#### Get Miner Stats
```bash
WALLET="44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBDDws8keQf66JxvVXuquhE3mAyUAL4f8cpAGzBVCTLG0P5sqDK17I3wcBiRT"
curl http://localhost:3000/api/v1/miners/$WALLET/stats
```

Expected response (after mining):
```json
{
  "wallet_address": "44AFFq...",
  "total_shares": 150,
  "valid_shares": 148,
  "invalid_shares": 2,
  "hashrate": 1234.56,
  "workers_count": 1,
  "last_seen": "2025-12-08T..."
}
```

#### Get Miner Balances
```bash
curl http://localhost:3000/api/v1/miners/$WALLET/balances
```

Expected response:
```json
[
  {
    "id": 1,
    "miner_id": 1,
    "coin": "XMR",
    "balance": 0.05,
    "pending_balance": 0.0,
    "total_paid": 0.0,
    "updated_at": "2025-12-08T..."
  }
]
```

### Mining Tests

#### Test with xmrig (Monero)

1. Download xmrig:
```bash
wget https://github.com/xmrig/xmrig/releases/download/v6.21.0/xmrig-6.21.0-linux-x64.tar.gz
tar xzf xmrig-6.21.0-linux-x64.tar.gz
cd xmrig-6.21.0
```

2. Create config:
```json
{
  "autosave": true,
  "cpu": true,
  "opencl": false,
  "cuda": false,
  "pools": [
    {
      "url": "localhost:3333",
      "user": "44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBDDws8keQf66JxvVXuquhE3mAyUAL4f8cpAGzBVCTLG0P5sqDK17I3wcBiRT",
      "pass": "x",
      "keepalive": true,
      "tls": false
    }
  ]
}
```

3. Run miner:
```bash
./xmrig -c config.json
```

4. Check logs:
- Proxy should show: "V1 Miner → Pool: login"
- Proxy should show: "V1 Miner → Pool: submit" (when shares found)
- Server should record shares in database

#### Verify Share Recording

```bash
# Connect to database
psql postgres://defpool:defpool_dev_password@localhost:5432/defpool

# Check shares
SELECT COUNT(*) FROM shares;
SELECT * FROM shares ORDER BY created_at DESC LIMIT 10;

# Check miner stats
SELECT * FROM miners;
SELECT * FROM workers;
```

### Payout System Tests

#### Request Payout
```bash
WALLET="44AFFq5kSiGBoZ4NMDwYtN18obc8AemS33DBDDws8keQf66JxvVXuquhE3mAyUAL4f8cpAGzBVCTLG0P5sqDK17I3wcBiRT"

curl -X POST http://localhost:3000/api/v1/miners/$WALLET/payout \
  -H "Content-Type: application/json" \
  -d '{
    "wallet_address": "'$WALLET'",
    "coin": "XMR",
    "amount": 0.01
  }'
```

Expected response:
```json
{
  "id": 1,
  "miner_id": 1,
  "coin": "XMR",
  "amount": 0.01,
  "tx_hash": null,
  "status": "pending",
  "created_at": "2025-12-08T...",
  "completed_at": null,
  "error_message": null
}
```

#### Check Payout Status
```bash
curl http://localhost:3000/api/v1/miners/$WALLET/payouts
```

#### Update Payout Settings
```bash
curl -X PUT http://localhost:3000/api/v1/miners/$WALLET/payout-settings \
  -H "Content-Type: application/json" \
  -d '{
    "min_payout_threshold": 0.1,
    "payout_coin": "BTC",
    "auto_exchange": true
  }'
```

### Database Tests

#### Check Background Tasks

```bash
# Watch profitability updates
tail -f defpool-server.log | grep "profitability"

# Watch balance updates
tail -f defpool-server.log | grep "balance"

# Watch payout processing
tail -f defpool-server.log | grep "payout"
```

#### Manual Balance Calculation

```sql
-- Check shares for a miner
SELECT 
    m.wallet_address,
    COUNT(*) as total_shares,
    SUM(CASE WHEN s.valid THEN 1 ELSE 0 END) as valid_shares,
    SUM(s.difficulty) as total_difficulty
FROM shares s
JOIN miners m ON s.miner_id = m.id
GROUP BY m.wallet_address;

-- Check hashrate (last 10 minutes)
SELECT 
    m.wallet_address,
    SUM(s.difficulty) / 600.0 as hashrate
FROM shares s
JOIN miners m ON s.miner_id = m.id
WHERE s.created_at > NOW() - INTERVAL '10 minutes'
  AND s.valid = true
GROUP BY m.wallet_address;
```

## Performance Testing

### Load Testing with Multiple Miners

```bash
# Start multiple xmrig instances
for i in {1..10}; do
  ./xmrig -c config.json --user "wallet:worker$i" &
done
```

### Monitor Performance

```bash
# Check database connections
psql $DATABASE_URL -c "SELECT count(*) FROM pg_stat_activity;"

# Check server memory
ps aux | grep defpool-server

# Check proxy connections
netstat -an | grep 3333 | wc -l
```

## Troubleshooting

### Server Won't Start

**Issue**: Database connection failed
```
Error: Failed to connect to database
```

**Solution**:
1. Check PostgreSQL is running: `docker-compose ps`
2. Verify DATABASE_URL environment variable
3. Check database credentials in config

### Miner Can't Connect

**Issue**: Connection refused
```
[ERROR] connect error: "Connection refused"
```

**Solution**:
1. Check proxy is running: `ps aux | grep defpool-proxy`
2. Verify listen address: `netstat -an | grep 3333`
3. Check firewall rules

### Shares Not Recording

**Issue**: Shares submitted but not in database

**Solution**:
1. Check proxy logs for "Failed to record share"
2. Verify server API is accessible from proxy
3. Check database migrations are applied
4. Verify miner wallet address format

### Profitability Not Updating

**Issue**: All targets show score 0.0

**Solution**:
1. Check API rate limits (CoinGecko, etc.)
2. Verify internet connectivity
3. Check server logs for API errors
4. Increase profitability_check_interval_secs if rate limited

## Integration Tests

### End-to-End Test Script

```bash
#!/bin/bash
set -e

echo "Starting DefPool integration test..."

# 1. Start services
docker-compose up -d postgres
sleep 5

# 2. Run migrations
psql $DATABASE_URL < defpool-server/src/db/migrations/001_initial_schema.sql
psql $DATABASE_URL < defpool-server/src/db/migrations/002_payout_system.sql

# 3. Start server
cd defpool-server && cargo run --release &
SERVER_PID=$!
sleep 5

# 4. Start proxy
cd ../defpool-proxy && cargo run --release &
PROXY_PID=$!
sleep 5

# 5. Test API
curl -f http://localhost:3000/api/v1/target || exit 1
echo "✓ API responding"

# 6. Start miner for 60 seconds
timeout 60 ./xmrig -c config.json || true

# 7. Check shares recorded
SHARES=$(psql $DATABASE_URL -t -c "SELECT COUNT(*) FROM shares;")
if [ "$SHARES" -gt 0 ]; then
    echo "✓ Shares recorded: $SHARES"
else
    echo "✗ No shares recorded"
    exit 1
fi

# 8. Cleanup
kill $SERVER_PID $PROXY_PID
docker-compose down

echo "✓ Integration test passed!"
```

## Continuous Testing

### Watch Mode for Development

```bash
# Terminal 1: Server with auto-reload
cd defpool-server
cargo watch -x run

# Terminal 2: Proxy with auto-reload
cd defpool-proxy
cargo watch -x run

# Terminal 3: Test miner
./xmrig -c config.json
```

### Automated Testing

```bash
# Run all tests
cargo test --all

# Run specific test
cargo test --package defpool-server test_calculate_profitability

# Run with output
cargo test -- --nocapture
```

## Production Checklist

Before deploying to production:

- [ ] All tests pass
- [ ] Database migrations applied
- [ ] Environment variables configured
- [ ] SSL/TLS certificates installed
- [ ] Firewall rules configured
- [ ] Monitoring set up
- [ ] Backup system configured
- [ ] Load testing completed
- [ ] Security audit performed
- [ ] Documentation updated

---

Last Updated: 2025-12-08
